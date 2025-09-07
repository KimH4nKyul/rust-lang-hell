use std::ops::Div;
use thiserror::Error;

/**
PartialEq: 부분 동등성 비교 트레이트
- 이 트레이트를 구현하면 타입의 인스턴스들을 ==, != 연산를 통해 비교할 수 있게 됨
- 모든 값에 대한 완전한 동치 관계가 성립하지 않아도 됨
- 예, 부동소수점은 NaN 값 때문에 완전 동치 관계를 가질 수 없어 PartialEq가 필요함
- 반대의 경우, 완전 동치를 요구하는 Eq가 있음
*/
#[derive(PartialEq, Error, Debug)]
pub enum DivisionError {
    #[error("0으로 나눌 수 없음")]
    DivisionByZero,
}

/// 두 정수를 나누는 함수
/// # Examples
///
/// ```
/// use task::divide;
/// assert_eq!(divide(10, 2), Ok(5));
/// ```
///
/// # Errors
///
/// 만약 `b`가 0이라면 `DivisionError::DivisionByZero` 에러를 반환합니다.
///
/// ```
/// use task::{divide, DivisionError};
/// assert_eq!(divide(10, 0), Err(DivisionError::DivisionByZero));
/// ```
pub fn divide<T>(left: T, right: T) -> Result<T, DivisionError>
where
    T: Div<Output = T> + PartialEq + From<u8> + Copy,
{
    if right == T::from(0) {
        return Err(DivisionError::DivisionByZero);
    }
    Ok(left / right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        // 0으로 나누려 할 때 에러가 발생
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_divide_generic() {
        assert_eq!(divide(10u32, 2u32), Ok(5u32));
        assert!(divide(5u64, 0u64).is_err());
    }
}
