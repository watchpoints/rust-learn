fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

fn main() {
    println!("{}", factorial(5)); // 输出: 120
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn factorial_of_5() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_of_10() {
        assert_eq!(factorial(10), 3_628_800);
    }
}
