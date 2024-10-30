// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // Vec is a heap-allocated data structure.
    // v.leak() tells Rust to never free that heap allocation.
    // Thus it returns a mutable 'static reference to the contents.
    let v_leak: &'static mut [i32] = v.leak();

    let handle1 = thread::spawn(|| {
        let half = v_leak.len() / 2;
        let left = &v_leak[0..half];
        let sum: i32 = left.iter().sum();
        sum
    });

    let handle2 = thread::spawn(|| {
        let half = v_leak.len() / 2;
        let right = &v_leak[half..];
        let sum: i32 = right.iter().sum();
        sum
    });

    handle1.join().unwrap() + handle2.join().unwrap()
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
