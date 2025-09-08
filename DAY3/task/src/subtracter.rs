use std::ops::Sub;

/// 두 정수를 빼는 함수
pub fn subtract<T: Sub<Output = T>>(left: T, right: T) -> T {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 뺄셈_가능() {
        let result = subtract(5, 2);
        assert_eq!(result, 3)
    }

    #[test]
    fn 음수_뺄셈_가능() {
        let result = subtract(2, 5);
        assert_eq!(result, -3)
    }

    #[test]
    fn zero를_뺄수_있음() {
        let result = subtract(0, 5);
        assert_eq!(result, -5)
    }

    #[test]
    fn test_subtract_i8() {
        assert_eq!(subtract(10i8, 2i8), 8i8);
    }
}
