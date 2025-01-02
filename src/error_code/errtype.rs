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
#[non_exhaustive]
#[getset2(get_copy(pub, const))]
pub struct ErrType {
    #[getset2(set(pub, const), set_with(pub, const))]
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

/// A possible error value when converting a `ErrType` from a digit
///
/// This error indicates that the supplied input was not a valid digit, was
/// less than 1000, or was greater than 4293.
#[derive(PartialEq)]
pub struct InvalidErrTypeFlag {
    _priv: (),
}
impl InvalidErrTypeFlag {
    const fn new() -> Self {
        Self { _priv: () }
    }
}
impl std::fmt::Debug for InvalidErrTypeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvalidErrTypeFlag")
            // skip _priv noise
            .finish()
    }
}

impl std::fmt::Display for InvalidErrTypeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid error type flag")
    }
}

impl std::convert::TryFrom<i16> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: i16) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

impl std::convert::TryFrom<u16> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: u16) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

impl std::convert::TryFrom<i32> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: i32) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

impl std::convert::TryFrom<u32> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: u32) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

impl std::convert::TryFrom<i64> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: i64) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

impl std::convert::TryFrom<u64> for ErrType {
    type Error = InvalidErrTypeFlag;
    fn try_from(digit: u64) -> Result<Self, Self::Error> {
        if (1000..=4293).contains(&digit) {
            Ok(ErrType {
                text: "",
                flag: digit as u16,
            })
        } else {
            Err(InvalidErrTypeFlag::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrType, InvalidErrTypeFlag};

    #[test]
    fn display() {
        const ET1: ErrType = ErrType::T4293("");
        assert_eq!("<no description> ErrType(4293)", ET1.to_string());
        const ET2: ErrType = ErrType::T1000("The operation was cancelled.");
        assert_eq!("The operation was cancelled. ErrType(1000)", ET2.to_string());
    }
    #[test]
    fn convert() {
        assert_eq!(Err(InvalidErrTypeFlag::new()), ErrType::try_from(0));
        assert_eq!(Ok(ErrType::T1000("")), ErrType::try_from(1000));
        assert_eq!(Ok(ErrType::T4293("")), ErrType::try_from(4293));
        assert_eq!(Err(InvalidErrTypeFlag::new()), ErrType::try_from(0));
        assert_eq!(Err(InvalidErrTypeFlag::new()), ErrType::try_from(999));
        assert_eq!(Err(InvalidErrTypeFlag::new()), ErrType::try_from(4294));
    }
}
