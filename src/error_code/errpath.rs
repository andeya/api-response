use api_response_macros::ErrPathConstructor;

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
    ErrPathConstructor
)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrPathRoot {
    name: &'static str,
    /// The value range of the flag is from 0 to 99 inclusive.
    flag: u8,
}
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
    ErrPathConstructor
)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrPathParent {
    root: ErrPathRoot,
    name: &'static str,
    /// The value range of the flag is from 0 to 99 inclusive.
    flag: u8,
}

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
    serde::Deserialize
)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ErrPath {
    parent: ErrPathParent,
    name: &'static str,
    /// The value range of the flag is from 0 to 99 inclusive.
    flag: u8,
}
impl ErrPathRoot {
    #[inline]
    pub const fn default() -> Self {
        Self { name: "", flag: 0 }
    }
    #[inline]
    pub fn path(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub const fn path_flag(&self) -> u32 {
        self.flag as u32
    }
    pub fn try_from<T: TryInto<u8>>(flag: T, name: &'static str) -> Result<Self, InvalidErrPathFlag> {
        let x: u8 = flag.try_into().map_err(|_| InvalidErrPathFlag::new())?;
        if x > 99 {
            Err(InvalidErrPathFlag::new())
        } else {
            Ok(Self { name, flag: x })
        }
    }
    pub fn try_to_child<T: TryInto<u8>>(
        self,
        child_flag: T,
        child_name: &'static str,
    ) -> Result<ErrPathParent, InvalidErrPathFlag> {
        let x: u8 = child_flag.try_into().map_err(|_| InvalidErrPathFlag::new())?;
        if x > 99 {
            Err(InvalidErrPathFlag::new())
        } else {
            Ok(ErrPathParent {
                root: self,
                name: child_name,
                flag: x,
            })
        }
    }
}
impl ErrPathParent {
    #[inline]
    pub const fn default() -> Self {
        Self {
            root: ErrPathRoot::default(),
            name: "",
            flag: 0,
        }
    }
    #[inline]
    pub fn path(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub const fn path_flag(&self) -> u32 {
        (self.root.path_flag() * 100) + self.flag as u32
    }
    pub fn try_to_child<T: TryInto<u8>>(
        self,
        child_flag: T,
        child_name: &'static str,
    ) -> Result<ErrPath, InvalidErrPathFlag> {
        let x: u8 = child_flag.try_into().map_err(|_| InvalidErrPathFlag::new())?;
        if x > 99 {
            Err(InvalidErrPathFlag::new())
        } else {
            Ok(ErrPath {
                parent: self,
                name: child_name,
                flag: x,
            })
        }
    }
}
impl ErrPath {
    #[inline]
    pub const fn default() -> Self {
        Self {
            parent: ErrPathParent::default(),
            name: "",
            flag: 0,
        }
    }
    #[inline]
    pub fn path(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub const fn path_flag(&self) -> u32 {
        (self.parent.path_flag() * 100) + self.flag as u32
    }
}

impl std::fmt::Display for ErrPathRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X{:02}({})", self.flag, self.name)
    }
}
impl std::fmt::Display for ErrPathParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/Y{:02}({})", self.root, self.flag, self.name)
    }
}
impl std::fmt::Display for ErrPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/Z{:02}({})", self.parent, self.flag, self.name)
    }
}

/// A possible error value when converting a `ErrType` from a digit
///
/// This error indicates that the supplied input was not a valid digit, was
/// less than 1000, or was greater than 4293.
#[derive(PartialEq)]
pub struct InvalidErrPathFlag {
    _priv: (),
}
impl InvalidErrPathFlag {
    const fn new() -> Self {
        Self { _priv: () }
    }
}
impl std::fmt::Debug for InvalidErrPathFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvalidErrPathFlag")
            // skip _priv noise
            .finish()
    }
}

impl std::fmt::Display for InvalidErrPathFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid error path flag")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn useage() {
        const ERR_PATH_ROOT: ErrPathRoot = ErrPathRoot::X99("name1");
        const ERR_PATH_PARENT: ErrPathParent = ERR_PATH_ROOT.Y01("name2");
        const ERR_PATH: ErrPath = ERR_PATH_PARENT.Z20("name3");

        assert_eq!("X99(name1)", ERR_PATH_ROOT.path());
        assert_eq!("X99(name1)/Y01(name2)", ERR_PATH_PARENT.path());
        assert_eq!("X99(name1)/Y01(name2)/Z20(name3)", ERR_PATH.path());
    }
    #[test]
    fn convert() {
        assert_eq!(Ok(ErrPathRoot::X00("")), ErrPathRoot::try_from(0, ""));
        assert_eq!(Ok(ErrPathRoot::X99("")), ErrPathRoot::try_from(99, ""));
        assert_eq!(Err(InvalidErrPathFlag::new()), ErrPathRoot::try_from(100, ""));
        assert_eq!(
            Ok(ErrPathRoot::X00("").Y00("")),
            ErrPathRoot::X00("").try_to_child(0, "")
        );
        assert_eq!(
            Ok(ErrPathRoot::X99("").Y99("")),
            ErrPathRoot::X99("").try_to_child(99, "")
        );
        assert_eq!(
            Err(InvalidErrPathFlag::new()),
            ErrPathRoot::X99("").try_to_child(100, "")
        );
        assert_eq!(
            Ok(ErrPathRoot::X00("").Y00("").Z00("")),
            ErrPathRoot::X00("").Y00("").try_to_child(0, "")
        );
        assert_eq!(
            Ok(ErrPathRoot::X99("").Y99("").Z99("")),
            ErrPathRoot::X99("").Y99("").try_to_child(99, "")
        );
    }
}
