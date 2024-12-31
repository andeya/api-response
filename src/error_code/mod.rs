mod errpath;
mod errtype;
pub mod ety_grpc;

use std::{
    fmt::Display,
    ops::{Add, BitOr},
    thread::LocalKey,
};

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

impl BitOr<&ErrPath> for ErrType {
    type Output = ErrBrief;

    #[inline]
    fn bitor(self, rhs: &ErrPath) -> Self::Output {
        self.extract(rhs)
    }
}
impl BitOr<&'static LocalKey<ErrPath>> for ErrType {
    type Output = ErrBrief;

    #[inline]
    fn bitor(self, rhs: &'static LocalKey<ErrPath>) -> Self::Output {
        rhs.with(|v| self.extract(v))
    }
}
impl Add<&ErrPath> for ErrType {
    type Output = ApiError;

    #[inline]
    fn add(self, rhs: &ErrPath) -> Self::Output {
        self.api_error(rhs)
    }
}
impl Add<&'static LocalKey<ErrPath>> for ErrType {
    type Output = ApiError;

    #[inline]
    fn add(self, rhs: &'static LocalKey<ErrPath>) -> Self::Output {
        rhs.with(|v| self.api_error(v))
    }
}

#[cfg(test)]
mod tests {
    // use std::cell::LazyCell;

    // use super::{ErrDecl, ErrPath, ErrFlag, ErrType, ModSection,
    // ModSegment}; use crate::ApiError;

    // #[test]
    // fn display() {
    //     const ET: ErrType = ErrType::new(ErrFlag::E100, "The operation was
    // cancelled.");     const MS0: ModSection =
    // ModSection::new(ModSegment::M00, "module 0");     const MS1:
    // ModSection = ModSection::new(ModSegment::M01, "module 01");     const
    // MS2: ModSection = ModSection::new(ModSegment::M02, "module 012");
    //     const MP: ErrPath = ErrPath::new(MS0, MS1, MS2);
    //     const EC: ErrDecl = ErrDecl::new(ET, MP);
    //     assert_eq!(
    //         "The operation was cancelled. ErrDecl(100000102), M00(module
    // 0)/M01(module 01)/M02(module 012)",         EC.to_string()
    //     );

    //     let err_code: ErrDecl = ET | MP;
    //     assert_eq!(EC, err_code);
    //     let api_error: ApiError = ET + MP;
    //     assert_eq!(EC.to_api_error().code(), api_error.code());
    //     let mp: LazyCell<ErrPath> = LazyCell::new(|| MP);
    //     let err_code: ErrDecl = ET | *mp;
    //     assert_eq!(EC, err_code);
    //     let api_error: ApiError = ET + *mp;
    //     assert_eq!(EC.to_api_error().code(), api_error.code());
    // }
}
