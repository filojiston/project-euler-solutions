// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

fn problem3() -> u64 {
    let mut n: u64 = 600851475143;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            n /= i;
        } else {
            i += 1;
        }
    }

    n
}

fn problem3_sieve() -> usize {
    let mut sieve = vec![true; 10000];
    let mut primes = vec![];
    for i in 2..sieve.len() {
        if sieve[i] {
            primes.push(i);
            let mut j = i * i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i;
            }
        }
    }
    let mut n = 600851475143;
    let mut i = 0;
    while n > 1 {
        if n % primes[i] == 0 {
            n /= primes[i];
        } else {
            i += 1;
        }
    }

    primes[i]
}

fn main() {
    println!("{}", problem3());
    println!("{}", problem3_sieve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem3() {
        assert_eq!(problem3(), 6857);
    }

    #[test]
    fn test_problem3_sieve() {
        assert_eq!(problem3_sieve(), 6857);
    }
}
