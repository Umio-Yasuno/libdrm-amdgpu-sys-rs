#[derive(Debug, Clone)]
pub struct GfxTargetVersion {
    pub major: u32,
    pub minor: u32,
    pub stepping: u32,
}

impl GfxTargetVersion {
    pub fn to_single_value(&self) -> u32 {
        (self.major * 10000) + (self.minor * 100) + self.stepping
    }
}

impl From<u32> for GfxTargetVersion {
    /// e.g. 90012, 100302
    fn from(value: u32) -> Self {
        let [major, minor, stepping] = [
            value / 10000,
            (value / 100) % 100,
            value % 100,
        ];

        Self { major, minor, stepping }
    }
}

impl From<(u32, u32, u32)> for GfxTargetVersion {
    fn from((major, minor, stepping): (u32, u32, u32)) -> Self {
        Self { major, minor, stepping }
    }
}

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl fmt::Display for GfxTargetVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "gfx{}{}{:x}", self.major, self.minor, self.stepping)
    }
}
