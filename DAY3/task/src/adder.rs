use std::ops::Add;

/// 두 정수를 더하는 함수
/// # Examples
///
/// ```
/// use task::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add<T: Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 덧셈_가능() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn 음수_덧셈_가능() {
        let result = add(-2, -2);
        assert_eq!(result, -4);
    }

    #[test]
    fn zero를_더할_수_있음() {
        let result = add(5, 0);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add_u64() {
        assert_eq!(add(10u64, 20u64), 30u64);
    }
}
