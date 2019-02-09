//! Time units

/// Hertz
#[derive(Clone, Copy, Debug)]
pub struct Hertz(pub u32);

// TODO: Implement kHz, and MHz

/// Extension trait that adds convenience to the `u32` type
pub trait U32Ext {
    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;
}

impl U32Ext for u32 {
    fn hz(self) -> Hertz {
        Hertz(self)
    }
}
