// By listing the first six prime numbers:
// 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn problem7() -> u64 {
    let mut sieve = vec![true; 1000000];
    let mut primes = Vec::new();

    for i in 2..sieve.len() {
        if sieve[i] {
            primes.push(i);
            for j in (i..sieve.len()).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    primes[10000] as u64
}

fn main() {
    println!("{}", problem7());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem7() {
        assert_eq!(problem7(), 104743);
    }
}
