use std::fmt::Display;

use getset2::Getset2;

use super::ModSegment;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Getset2)]
#[getset2(get_copy(pub, const), set(pub = "crate", const), set_with(pub, const))]
#[non_exhaustive]
pub struct ModPath {
    mod1: ModSection,
    mod2: ModSection,
    mod3: ModSection,
}

impl Display for ModPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.mod1, self.mod2, self.mod3,)
    }
}

impl ModPath {
    pub const fn default() -> Self {
        Self {
            mod1: ModSection::default(),
            mod2: ModSection::default(),
            mod3: ModSection::default(),
        }
    }
    pub const fn new(mod1: ModSection, mod2: ModSection, mod3: ModSection) -> Self {
        Self { mod1, mod2, mod3 }
    }
    pub fn mod_code(&self) -> i32 {
        self.mod1.mod_segment() | self.mod2.mod_segment() | self.mod3.mod_segment()
    }
    pub const fn mod1_segment(&self) -> ModSegment {
        self.mod1.mod_segment()
    }
    pub const fn mod2_segment(&self) -> ModSegment {
        self.mod2.mod_segment()
    }
    pub const fn mod3_segment(&self) -> ModSegment {
        self.mod3.mod_segment()
    }
    pub const fn mod1_name(&self) -> &'static str {
        self.mod1.name()
    }
    pub const fn mod2_name(&self) -> &'static str {
        self.mod2.name()
    }
    pub const fn mod3_name(&self) -> &'static str {
        self.mod3.name()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Getset2)]
#[getset2(get_copy(pub, const))]
#[non_exhaustive]
pub struct ModSection {
    mod_segment: ModSegment,
    name: &'static str,
}

impl Display for ModSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M{}({})", self.mod_segment, self.name)
    }
}

impl ModSection {
    pub const fn default() -> Self {
        Self {
            mod_segment: ModSegment::M00,
            name: "",
        }
    }
    pub const fn new(mod_segment: ModSegment, name: &'static str) -> Self {
        Self { mod_segment, name }
    }
}

impl ModSegment {
    #[inline]
    pub const fn new_mod_section(self, name: &'static str) -> ModSection {
        ModSection::new(self, name)
    }
}

#[cfg(test)]
mod tests {
    use super::{ModSection, ModSegment};
    use crate::error_code::ModPath;

    #[test]
    fn display() {
        const MS0: ModSection = ModSection::new(ModSegment::M00, "module 0");
        const MS1: ModSection = ModSection::new(ModSegment::M01, "module 01");
        const MS2: ModSection = ModSection::new(ModSegment::M02, "module 012");
        assert_eq!("M00(module 0)", MS0.to_string());
        const MP: ModPath = ModPath::new(MS0, MS1, MS2);
        assert_eq!("M00(module 0)/M01(module 01)/M02(module 012)", MP.to_string());
    }
}
