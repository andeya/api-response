use std::fmt::Display;

use getset2::Getset2;

use super::{ErrBrief, ErrDecl, ErrPath};
use crate::ApiError;

api_response_macros::enum_digits!(ErrFlag, u16, 100, 999, E, 3);

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    Getset2,
    serde::Serialize,
    serde::Deserialize
)]
#[getset2(get_copy(pub, const), set(pub, const), set_with(pub, const))]
#[non_exhaustive]
pub struct ErrType {
    text: &'static str,
    flag: ErrFlag,
}

impl ErrType {
    #[inline]
    pub const fn new(flag: ErrFlag, text: &'static str) -> Self {
        Self { flag, text }
    }
    #[inline(always)]
    pub const fn declare(self, err_path: ErrPath) -> ErrDecl {
        ErrDecl::new(self, err_path)
    }
    #[inline(always)]
    pub const fn extract(self, err_path: &ErrPath) -> ErrBrief {
        ErrBrief::new(self, err_path)
    }
    #[inline(always)]
    pub fn api_error(self, err_path: &ErrPath) -> ApiError {
        self.extract(err_path).api_error()
    }
}
impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ErrType({})", self.text, self.flag)
    }
}
impl ErrFlag {
    pub const fn define(self, text: &'static str) -> ErrType {
        ErrType::new(self, text)
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrFlag, ErrType};

    #[test]
    fn display() {
        const ET: ErrType = ErrType::new(ErrFlag::E100, "The operation was cancelled.");
        assert_eq!("The operation was cancelled. ErrType(100)", ET.to_string());
    }
}
