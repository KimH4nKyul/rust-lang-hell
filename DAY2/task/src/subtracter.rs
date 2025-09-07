/// 두 정수를 빼는 함수
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
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
}
