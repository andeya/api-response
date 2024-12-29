mod errtype;
pub mod ety_grpc;
mod modpath;
mod segment;

use std::fmt::Display;

pub use errtype::*;
pub use modpath::*;
pub use segment::*;

use crate::ApiError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct ErrCode {
    pub err_type: ErrType,
    pub mod_path: ModPath,
}

impl From<ErrCode> for i32 {
    fn from(value: ErrCode) -> Self {
        value.code()
    }
}
impl ErrCode {
    pub const fn new(err_type: ErrType, mod_path: ModPath) -> Self {
        Self { err_type, mod_path }
    }
    pub fn code(&self) -> i32 {
        self.err_type.err_segment()
            | self.mod_path.mod1_segment()
            | self.mod_path.mod2_segment()
            | self.mod_path.mod3_segment()
    }
    pub const fn err_type(&self) -> &ErrType {
        &self.err_type
    }
    pub const fn err_segment(&self) -> ErrSegment {
        self.err_type.err_segment()
    }
    pub const fn text(&self) -> &'static str {
        self.err_type.text()
    }
    pub const fn mod_path(&self) -> ModPath {
        self.mod_path
    }
    pub const fn mod1(&self) -> ModSection {
        self.mod_path.mod1()
    }
    pub const fn mod2(&self) -> ModSection {
        self.mod_path.mod2()
    }
    pub const fn mod3(&self) -> ModSection {
        self.mod_path.mod3()
    }
    pub const fn with_mod1(mut self, mod1: ModSection) -> Self {
        self.mod_path.set_mod1(mod1);
        self
    }
    pub const fn with_mod2(mut self, mod2: ModSection) -> Self {
        self.mod_path.set_mod2(mod2);
        self
    }
    pub const fn with_mod3(mut self, mod3: ModSection) -> Self {
        self.mod_path.set_mod3(mod3);
        self
    }
    pub fn to_api_error(self) -> ApiError {
        ApiError {
            code: self.code(),
            message: self.text().to_owned(),
            details: None,
            source: None,
        }
    }
}

impl Display for ErrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ErrCode({}), {}", self.text(), self.code(), self.mod_path)
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrCode, ErrSegment, ErrType, ModPath, ModSection, ModSegment};

    #[test]
    fn display() {
        const ET: ErrType = ErrType::new(ErrSegment::E100, "The operation was cancelled.");
        const MS0: ModSection = ModSection::new(ModSegment::M00, "module 0");
        const MS1: ModSection = ModSection::new(ModSegment::M01, "module 01");
        const MS2: ModSection = ModSection::new(ModSegment::M02, "module 012");
        const MP: ModPath = ModPath::new(MS0, MS1, MS2);
        const EC: ErrCode = ErrCode::new(ET, MP);
        let err_code: ErrCode = ET + MP;
        assert_eq!(EC, err_code);
        assert_eq!(
            "The operation was cancelled. ErrCode(100000102), M00(module 0)/M01(module 01)/M02(module 012)",
            EC.to_string()
        );
    }
}
