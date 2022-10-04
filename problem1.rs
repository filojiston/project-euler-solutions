// If we list all the natural numbers below 10 that are
// multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn problem1(n: i32) -> i32 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    println!("{}", problem1(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_23_when_bound_is_10() {
        assert_eq!(23, problem1(10));
    }

    #[test]
    fn should_return_233168_when_bound_is_1000() {
        assert_eq!(233168, problem1(1000));
    }
}
