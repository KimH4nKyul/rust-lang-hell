pub fn add(a: i32, b: i32) -> i32 {
    a + b
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
}