//! Prelude - Include traits for HAL

pub use crate::hal::prelude::*; // embedded-hal traits

pub use crate::flash::FlashExt as _stm32f7_hal_FlashExt;
pub use crate::gpio::GpioExt as _stm32f7_hal_GpioExt;
pub use crate::rcc::RccExt as _stm32f7_hal_RccExt;
