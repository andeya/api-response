mod errpath;
mod errtype;
pub mod ety_grpc;
pub mod tally;

use std::{
    fmt::Display,
    ops::{Add, BitOr},
    thread::LocalKey,
};

pub use errpath::*;
pub use errtype::*;
use getset2::Getset2;
use serde::{Deserialize, Serialize};

use crate::ApiError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize)]
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
    pub const fn err_flag(&self) -> u32 {
        self.err_type.flag() as u32
    }
    #[inline]
    pub const fn err_path_flag(&self) -> u32 {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Getset2, Serialize, Deserialize)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrBrief {
    message: &'static str,
    /// The value range of the code is from 1000000000 to 4293999999 inclusive.
    code: u32,
}
impl Display for ErrBrief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.message.is_empty() {
            write!(f, "<no message> ErrCode({})", self.code)
        } else {
            write!(f, "{} ErrCode({})", self.message, self.code)
        }
    }
}
impl ErrBrief {
    #[inline(always)]
    pub const fn new(err_type: ErrType, err_path: &ErrPath) -> Self {
        Self {
            message: err_type.text(),
            code: (err_type.flag() as u32 * 1000000) + err_path.path_flag(),
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

    #[inline(always)]
    fn bitor(self, rhs: &'static str) -> Self::Output {
        self.with_text(rhs)
    }
}

impl Add<ErrPath> for ErrType {
    type Output = ErrDecl;

    #[inline(always)]
    fn add(self, rhs: ErrPath) -> Self::Output {
        self.declare(rhs)
    }
}
impl Add<&ErrPath> for ErrType {
    type Output = ErrDecl;

    #[inline(always)]
    fn add(self, rhs: &ErrPath) -> Self::Output {
        self.declare(*rhs)
    }
}
impl Add<&'static LocalKey<ErrPath>> for ErrType {
    type Output = ErrDecl;

    #[inline(always)]
    fn add(self, rhs: &'static LocalKey<ErrPath>) -> Self::Output {
        rhs.with(|v| self.declare(*v))
    }
}

impl BitOr<&ErrPath> for ErrType {
    type Output = ApiError;

    #[inline(always)]
    fn bitor(self, rhs: &ErrPath) -> Self::Output {
        self.api_error(rhs)
    }
}
impl BitOr<&'static LocalKey<ErrPath>> for ErrType {
    type Output = ApiError;

    #[inline(always)]
    fn bitor(self, rhs: &'static LocalKey<ErrPath>) -> Self::Output {
        rhs.with(|v| self.api_error(v))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::{ErrDecl, ErrPath, ErrPathParent, ErrPathRoot, ErrType};
    use crate::ApiError;

    #[test]
    fn display() {
        const ET: ErrType = ErrType::T1100("The operation was cancelled.");
        const EP_LV1: ErrPathRoot = ErrPathRoot::X00("product");
        const EP_LV2: ErrPathParent = EP_LV1.Y01("system");
        const EP_LV3: ErrPath = EP_LV2.Z20("module");
        const EC: ErrDecl = ErrDecl::new(ET, EP_LV3);
        assert_eq!(
            "The operation was cancelled. ErrCode(1100000120), X00(product)/Y01(system)/Z20(module)",
            EC.to_string()
        );

        let api_error: ApiError = ET | &EP_LV3;
        assert_eq!(EC.api_error().code(), api_error.code());
        let mp: LazyCell<ErrPath> = LazyCell::new(|| EP_LV3);
        let api_error: ApiError = ET | &*mp;
        assert_eq!(EC.api_error().code(), api_error.code());
    }

    #[test]
    fn min_max_code() {
        let min_code: ErrDecl = ErrType::T1000("") + ErrPathRoot::X00("").Y00("").Z00("");
        assert_eq!(
            "<no message> ErrCode(1000000000), X00()/Y00()/Z00()",
            min_code.to_string()
        );

        let max_code: ErrDecl = ErrType::T4293("") + ErrPathRoot::X99("").Y99("").Z99("");
        assert_eq!(
            "<no message> ErrCode(4293999999), X99()/Y99()/Z99()",
            max_code.to_string()
        );
    }
}
