use api_response_macros::ErrTypeConstructor;

use super::{ErrBrief, ErrDecl, ErrPath};
use crate::ApiError;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    getset2::Getset2,
    serde::Serialize,
    serde::Deserialize,
    ErrTypeConstructor
)]
#[getset2(get_copy(pub, const), set(pub, const), set_with(pub, const))]
#[non_exhaustive]
pub struct ErrType {
    text: &'static str,
    /// The value range of the flag is from 1000 to 4293 inclusive.
    flag: u16,
}

impl ErrType {
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

impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.text.is_empty() {
            write!(f, "<no description> ErrType({:04})", self.flag)
        } else {
            write!(f, "{} ErrType({:04})", self.text, self.flag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ErrType;

    #[test]
    fn display() {
        assert_eq!("<no description> ErrType(4293)", ET1.to_string());
        const ET1: ErrType = ErrType::T4293("");
        assert_eq!("<no description> ErrType(4293)", ET1.to_string());
        const ET2: ErrType = ErrType::T1000("The operation was cancelled.");
        assert_eq!("The operation was cancelled. ErrType(1000)", ET2.to_string());
    }
}
