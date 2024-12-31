mod errpath;
mod errtype;
pub mod ety_grpc;

use std::{fmt::Display, ops::BitOr, thread::LocalKey};

pub use errpath::*;
pub use errtype::*;
use getset2::Getset2;

use crate::ApiError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct ErrDecl {
    pub err_type: ErrType,
    pub err_path: ErrPath,
}

impl ErrDecl {
    #[inline]
    pub const fn new(err_type: ErrType, err_path: ErrPath) -> Self {
        Self { err_type, err_path }
    }
    #[inline]
    pub const fn err_flag(&self) -> i32 {
        self.err_type.flag() as i32
    }
    #[inline]
    pub const fn err_path_flag(&self) -> i32 {
        self.err_path.path_flag()
    }
    #[inline]
    pub const fn text(&self) -> &'static str {
        self.err_type.text()
    }
    pub const fn err_type(&self) -> &ErrType {
        &self.err_type
    }
    pub const fn err_path(&self) -> &ErrPath {
        &self.err_path
    }
    #[inline(always)]
    pub const fn extract(&self) -> ErrBrief {
        ErrBrief::new(self.err_type, &self.err_path)
    }
    #[inline(always)]
    pub fn api_error(&self) -> ApiError {
        self.extract().api_error()
    }
}

impl Display for ErrDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.extract(), self.err_path)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Getset2)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrBrief {
    message: &'static str,
    code: i32,
}
impl Display for ErrBrief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ErrCode({})", self.message, self.code)
    }
}
impl ErrBrief {
    #[inline(always)]
    pub const fn new(err_type: ErrType, err_path: &ErrPath) -> Self {
        Self {
            message: err_type.text(),
            code: (err_type.flag() as i32 * 1000000) + err_path.path_flag(),
        }
    }
    #[inline(always)]
    pub fn api_error(&self) -> ApiError {
        ApiError {
            code: self.code,
            message: self.message.to_owned(),
            details: None,
            source: None,
        }
    }
}

impl BitOr<&'static str> for ErrType {
    type Output = ErrType;

    #[inline]
    fn bitor(self, rhs: &'static str) -> Self::Output {
        self.with_text(rhs)
    }
}

impl BitOr<&ErrPath> for ErrType {
    type Output = ApiError;

    #[inline]
    fn bitor(self, rhs: &ErrPath) -> Self::Output {
        self.api_error(rhs)
    }
}
impl BitOr<&'static LocalKey<ErrPath>> for ErrType {
    type Output = ApiError;

    #[inline]
    fn bitor(self, rhs: &'static LocalKey<ErrPath>) -> Self::Output {
        rhs.with(|v| self.api_error(v))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::{ErrDecl, ErrFlag, ErrPath, ErrPathParent, ErrPathRoot, ErrType, X00};
    use crate::ApiError;

    #[test]
    fn display() {
        const ET: ErrType = ErrType::new(ErrFlag::E100, "The operation was cancelled.");
        const EP_LV1: ErrPathRoot = X00("product");
        const EP_LV2: ErrPathParent = EP_LV1.Y01("system");
        const EP_LV3: ErrPath = EP_LV2.Z20("module");
        const EC: ErrDecl = ErrDecl::new(ET, EP_LV3);
        assert_eq!(
            "The operation was cancelled. ErrCode(100000120), X00(product)/Y01(system)/Z20(module)",
            EC.to_string()
        );

        let api_error: ApiError = ET | &EP_LV3;
        assert_eq!(EC.api_error().code(), api_error.code());
        let mp: LazyCell<ErrPath> = LazyCell::new(|| EP_LV3);
        let api_error: ApiError = ET | &*mp;
        assert_eq!(EC.api_error().code(), api_error.code());
    }
}
