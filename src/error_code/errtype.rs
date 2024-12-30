use std::{
    fmt::Display,
    ops::{Add, BitOr},
};

use getset2::Getset2;

use super::{ErrCode, ErrSegment, ModPath};
use crate::ApiError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Getset2)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrType {
    err_segment: ErrSegment,
    text: &'static str,
}

impl ErrType {
    pub const fn new(err_segment: ErrSegment, text: &'static str) -> Self {
        Self { err_segment, text }
    }
    pub const fn to_err_code(self) -> ErrCode {
        ErrCode::new(self, ModPath::default())
    }
    pub const fn new_err_code(self, mod_path: ModPath) -> ErrCode {
        ErrCode::new(self, mod_path)
    }
    #[inline]
    pub fn new_api_error(self, mod_path: ModPath) -> ApiError {
        self.new_err_code(mod_path).to_api_error()
    }
}
impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ErrType({})", self.text, self.err_segment)
    }
}
impl ErrSegment {
    pub const fn new_err_type(self, text: &'static str) -> ErrType {
        ErrType::new(self, text)
    }
}
impl BitOr<ModPath> for ErrType {
    type Output = ErrCode;

    #[inline]
    fn bitor(self, rhs: ModPath) -> Self::Output {
        self.new_err_code(rhs)
    }
}
impl Add<ModPath> for ErrType {
    type Output = ApiError;

    #[inline]
    fn add(self, rhs: ModPath) -> Self::Output {
        self.new_api_error(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrSegment, ErrType};

    #[test]
    fn display() {
        const ET: ErrType = ErrType::new(ErrSegment::E100, "The operation was cancelled.");
        assert_eq!("The operation was cancelled. ErrType(100)", ET.to_string());
    }
}
