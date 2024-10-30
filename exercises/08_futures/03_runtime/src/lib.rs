// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    // todo!()
    let formatted_reply = format!("{}", reply);
    loop {
        let (mut socket_1, _) = first.accept().await.unwrap();
        let formatted_reply_clone = formatted_reply.clone();
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start 
        // accepting new connections
        let _handle_1 = tokio::spawn(async move {
            let (mut reader_1, mut writer_1) = socket_1.split();
            // tokio::io::copy(&mut formatted_reply, &mut writer_1).await;
            // Send the request
            writer_1.write_all(formatted_reply_clone.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer_1.shutdown().await.unwrap();
        });

        let (mut socket_2, _) = second.accept().await.unwrap();
        let formatted_reply_clone = formatted_reply.clone();
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start 
        // accepting new connections
        let _handle_2 = tokio::spawn(async move {
            let (mut reader_2, mut writer_2) = socket_2.split();
            // tokio::io::copy(&mut formatted_reply, &mut writer_1).await;
            // Send the request
            writer_2.write_all(formatted_reply_clone.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer_2.shutdown().await.unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
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
