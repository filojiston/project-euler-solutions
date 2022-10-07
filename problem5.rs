// 2520 is the smallest number that can be divided
// by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is
// evenly divisible by all of the numbers from 1 to 20?

fn problem5() -> u64 {
    // n equals the product of all the primes from 1 to 20
    // and the answer has to be a multiple of 9699690
    let mut n = 9_699_690;
    // we don't need to check all numbers from 1 to 20
    // for example a number that is divisible by 20 is also divisible by 10, 5, 4, 2
    let numbers_to_check = [11, 13, 14, 16, 17, 18, 19, 20];
    loop {
        if numbers_to_check.iter().all(|&x| n % x == 0) {
            return n;
        }
        n += 9_699_690;
    }
}

fn main() {
    println!("{}", problem5());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem5() {
        assert_eq!(problem5(), 232_792_560);
    }
}
