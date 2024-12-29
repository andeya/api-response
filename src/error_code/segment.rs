use std::ops::BitOr;

pub use ErrSegment::*;
pub use ModSegment::*;

api_code_macros::enum_segment!(ErrSegment, u16, 100, 999, E, 3);
api_code_macros::enum_segment!(ModSegment, u8, 0, 99, M, 2);

impl From<ErrSegment> for i32 {
    #[allow(clippy::as_conversions)]
    fn from(value: ErrSegment) -> Self {
        value as i32
    }
}
impl From<ModSegment> for i32 {
    #[allow(clippy::as_conversions)]
    fn from(value: ModSegment) -> Self {
        value as i32
    }
}

pub(crate) const OVERFLOW: &str = "A calculation overflow occurs when generating segmented-error-code.";

/// Append two digits at the end in the form of a decimal literal.
impl BitOr<ModSegment> for ErrSegment {
    type Output = i32;

    fn bitor(self, rhs: ModSegment) -> Self::Output {
        i32::from(self)
            .checked_mul(100)
            .expect(OVERFLOW)
            .checked_add(i32::from(rhs))
            .expect(OVERFLOW)
    }
}

/// Append two digits at the end in the form of a decimal literal.
impl BitOr<ModSegment> for i32 {
    type Output = i32;

    fn bitor(self, rhs: ModSegment) -> Self::Output {
        self.checked_mul(100)
            .expect(OVERFLOW)
            .checked_add(i32::from(rhs))
            .expect(OVERFLOW)
    }
}
/// Append two digits at the end in the form of a decimal literal.
impl BitOr<ModSegment> for ModSegment {
    type Output = i32;

    fn bitor(self, rhs: ModSegment) -> Self::Output {
        i32::from(self)
            .checked_mul(100)
            .expect(OVERFLOW)
            .checked_add(i32::from(rhs))
            .expect(OVERFLOW)
    }
}

#[allow(clippy::derivable_impls)]
impl Default for ModSegment {
    fn default() -> Self {
        ModSegment::M00
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrSegment, ModSegment};

    #[test]
    fn bit_or() {
        assert_eq!(10001, ErrSegment::E100 | ModSegment::M01)
    }

    #[test]
    fn display() {
        assert_eq!("100", ErrSegment::E100.to_string());
        assert_eq!("00", ModSegment::M00.to_string());
    }
}
