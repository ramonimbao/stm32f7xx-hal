//! Flash memory

use crate::stm32::{flash, FLASH};

/// Extension trait to constraint the FLASH peripheral
pub trait FlashExt {
    /// Constrains the FLASH peripheral to play nicely with the other abstractions
    fn constrain(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR { _0: () },
            cr: CR { _0: () },
            keyr: KEYR { _0: () },
            optcr: OPTCR { _0: () },
            optcr1: OPTCR1 { _0: () },
            optkeyr: OPTKEYR { _0: () },
            sr: SR { _0: () },
        }
    }
}

/// Constrained FLASH peripheral
pub struct Parts {
    /// Access control register
    pub acr: ACR,
    /// Control register
    pub cr: CR,
    /// Flash key register
    pub keyr: KEYR,
    /// Flash option control register
    pub optcr: OPTCR,
    /// Flash option control register 1
    pub optcr1: OPTCR1,
    /// Flash option key register
    pub optkeyr: OPTKEYR,
    /// Status register
    pub sr: SR,
}

pub struct ACR {
    _0: (),
}

impl ACR {
    #[allow(dead_code)]
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}

pub struct CR {
    _0: (),
}

impl CR {
    #[allow(dead_code)]
    pub(crate) fn cr(&mut self) -> &flash::CR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).cr }
    }
}

pub struct KEYR {
    _0: (),
}

impl KEYR {
    #[allow(dead_code)]
    pub(crate) fn keyr(&mut self) -> &flash::KEYR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).keyr }
    }
}

pub struct OPTCR {
    _0: (),
}

impl OPTCR {
    #[allow(dead_code)]
    pub(crate) fn optcr(&mut self) -> &flash::OPTCR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).optcr }
    }
}

pub struct OPTCR1 {
    _0: (),
}

impl OPTCR1 {
    #[allow(dead_code)]
    pub(crate) fn optcr1(&mut self) -> &flash::OPTCR1 {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).optcr1 }
    }
}

pub struct OPTKEYR {
    _0: (),
}

impl OPTKEYR {
    #[allow(dead_code)]
    pub(crate) fn optkeyr(&mut self) -> &flash::OPTKEYR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).optkeyr }
    }
}

pub struct SR {
    _0: (),
}

impl SR {
    #[allow(dead_code)]
    pub(crate) fn sr(&mut self) -> &flash::SR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).sr }
    }
}
