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
}
