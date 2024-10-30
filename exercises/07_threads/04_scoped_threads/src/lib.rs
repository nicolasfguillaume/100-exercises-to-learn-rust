// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let half = v.len() / 2;

    // One can use non-'static references and values inside the threads because 
    // the threads inside scope are guaranteed to finish before the scope ends.
    let res = std::thread::scope(|scope| {

        let handle1 = scope.spawn(|| {
            let left = &v[..half];
            let sum: i32 = left.iter().sum();
            sum
        });

        let handle2 = scope.spawn(|| {
            let right = &v[half..];
            let sum: i32 = right.iter().sum();
            sum
        });

        handle1.join().unwrap() + handle2.join().unwrap()
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
