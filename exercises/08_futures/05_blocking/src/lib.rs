// TODO: the `echo` server uses non-async primitives.
//  When running the tests, you should observe that it hangs, due to a
//  deadlock between the caller and the server.
//  Use `spawn_blocking` inside `echo` to resolve the issue.
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::task;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (socket, _) = listener.accept().await?;
        // run blocking operation on a dedicated thread pool
        let handle = task::spawn_blocking(|| {
            let mut socket = socket.into_std()?;
            // ensures that the socket operates in blocking mode
            socket.set_nonblocking(false)?;
            let mut buffer = Vec::new();
            // read and write are blocking I/O operations
            socket.read_to_end(&mut buffer)?;
            socket.write_all(&buffer)?;
            Ok::<(), std::io::Error>(()) // Return a result from the blocking task
        });
        // double error propagation: first ? checks for JoinError, second ? checks for std::io::Error
        handle.await??
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (listener, addr) = bind_random().await;
        tokio::spawn(echo(listener));

        let requests = vec![
            "hello here we go with a long message",
            "world",
            "foo",
            "bar",
        ];
        // A collection of tasks spawned on a Tokio runtime.
        let mut join_set = JoinSet::new();

        for request in requests {
            join_set.spawn(async move {
                let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                let (mut reader, mut writer) = socket.split();

                // Send the request
                writer.write_all(request.as_bytes()).await.unwrap();
                // Close the write side of the socket
                writer.shutdown().await.unwrap();

                // Read the response
                let mut buf = Vec::with_capacity(request.len());
                reader.read_to_end(&mut buf).await.unwrap();
                assert_eq!(&buf, request.as_bytes());
            });
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
