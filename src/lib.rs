//! STM32F7 HAL implementation

#![no_std]

#[cfg(not(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
)))]
compile_error!("This crate requires one of the following features enabled: stm32f7x2, stm32f7x3, stm32f7x5, stm32f7x6, stm32f7x9");

pub use embedded_hal as hal;

pub use stm32f7;
#[cfg(feature = "stm32f7x2")]
pub use stm32f7::stm32f7x2 as stm32;
#[cfg(feature = "stm32f7x3")]
pub use stm32f7::stm32f7x3 as stm32;
#[cfg(feature = "stm32f7x5")]
pub use stm32f7::stm32f7x5 as stm32;
#[cfg(feature = "stm32f7x6")]
pub use stm32f7::stm32f7x6 as stm32;
#[cfg(feature = "stm32f7x7")]
pub use stm32f7::stm32f7x7 as stm32;
#[cfg(feature = "stm32f7x9")]
pub use stm32f7::stm32f7x9 as stm32;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod delay;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod flash;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod gpio;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod prelude;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod rcc;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9"
))]
pub mod time;
