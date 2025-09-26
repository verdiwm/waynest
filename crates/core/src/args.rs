use std::{fmt, num::NonZeroU32};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct Fixed(i32);

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Fixed {
    /// # Safety
    /// The caller must ensure the passed i32 is actually a fixed
    pub const unsafe fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    pub const fn as_raw(&self) -> i32 {
        self.0
    }
}

impl From<f64> for Fixed {
    fn from(value: f64) -> Self {
        Fixed((value * 256.0).round() as i32)
    }
}

impl From<i32> for Fixed {
    fn from(value: i32) -> Self {
        Fixed(value * 256)
    }
}

impl From<Fixed> for f64 {
    fn from(value: Fixed) -> Self {
        f64::from(value.0) / 256.0
    }
}

impl From<Fixed> for i32 {
    fn from(value: Fixed) -> Self {
        value.0 / 256
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct ObjectId(NonZeroU32);

impl ObjectId {
    pub const DISPLAY: Self = unsafe { Self::from_raw(1) };

    pub const fn as_raw(&self) -> u32 {
        self.0.get()
    }

    /// # Safety
    /// The value must not be zero.
    pub const unsafe fn from_raw(id: u32) -> Self {
        // Safety is upheld by the caller
        unsafe { Self(NonZeroU32::new_unchecked(id)) }
    }

    pub const fn new(id: u32) -> Option<Self> {
        if id == 0 {
            return None;
        }

        Some(unsafe { Self::from_raw(id) })
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct NewId {
    pub interface: String,
    pub version: u32,
    pub object_id: ObjectId,
}

impl std::fmt::Display for NewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "new id {}#{}", self.interface, self.object_id)
    }
}
