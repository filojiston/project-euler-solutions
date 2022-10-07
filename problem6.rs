// Find the difference between the sum of the squares of the
// first one hundred natural numbers and the square of the sum.

fn problem6() -> u64 {
    let sum_of_squares: u64 = (1..101).map(|x| x * x).sum();
    let square_of_sum: u64 = (1..101).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}

fn problem6_improved() -> u64 {
    // n * (n + 1) * (2n + 1) / 6
    let sum_of_squares: u64 = 100 * 101 * 201 / 6;
    // n * (n + 1) / 2
    let square_of_sum: u64 = (100 * 101 / 2 as u64).pow(2);

    square_of_sum - sum_of_squares
}

fn main() {
    println!("{}", problem6());
    println!("{}", problem6_improved());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem6() {
        assert_eq!(problem6(), 25_164_150);
    }

    #[test]
    fn test_problem6() {
        assert_eq!(problem6_improved(), 25_164_150);
    }
}
