// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn problem4() -> u64 {
    let mut largest = 0;
    let mut x = 999;
    let mut y = 999;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if product > largest && is_palindrome(product) {
                largest = product;
                x = i;
                y = j;
            }
        }
    }
    println!("{} * {} = {}", x, y, largest);
    return largest;
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    problem4();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(1234), false);
    }

    #[test]
    fn test_problem4() {
        assert_eq!(problem4(), 906609);
    }
}
