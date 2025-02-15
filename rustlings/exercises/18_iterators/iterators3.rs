#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // 例如: 42 / 0
    DivideByZero,
    // `i64::MIN / -1` 是唯一的溢出情况，因为结果是 `i64::MAX + 1`
    IntegerOverflow,
    // 例如: 5 / 2 = 2.5 (不是整除)
    NotDivisible,
}

// 计算 `a` 除以 `b`，如果 `a` 可以被 `b` 整除，则返回 `Ok`，否则返回适当的错误。
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == i64::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible)
    } else {
        Ok(a / b)
    }
}

// 返回 `Ok` 包含计算结果的数组
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .collect() // `collect()` 会把 `Result` 解析成 `Result<Vec<_>, _>`
}

// 返回 `Result` 类型的数组
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    println!("{:?}", result_with_list()); // 应该返回 Ok([1, 11, 1426, 3])
    println!("{:?}", list_of_results());  // 应该返回 [Ok(1), Ok(11), Ok(1426), Ok(3)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
