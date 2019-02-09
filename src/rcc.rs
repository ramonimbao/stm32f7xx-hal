//! Reset and Clock Control

use crate::flash::ACR;
use crate::stm32::{rcc, RCC};
use crate::time::Hertz;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb1: AHB1 { _0: () },
            ahb2: AHB2 { _0: () },
            ahb3: AHB3 { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            bdcr: BDCR { _0: () },
            cfgr: CFGR { _0: () },
            cir: CIR { _0: () },
            cr: CR { _0: () },
            csr: CSR { _0: () },
            dkcfgr1: DKCFGR1 { _0: () },
            dkcfgr2: DKCFGR2 { _0: () },
            pllcfgr: PLLCFGR { _0: () },
            pllsaicfgr: PLLSAICFGR { _0: () },
            plli2scfgr: PLLI2SCFGR { _0: () },
            sscgr: SSCGR { _0: () },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// AMBA High-performance Bus 1 (AHB1) registers
    pub ahb1: AHB1,
    /// AMBA High-performance Bus 2 (AHB2) registers
    pub ahb2: AHB2,
    /// AMBA High-performance Bus 3 (AHB3) registers
    pub ahb3: AHB3,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    /// Backup domain control register
    pub bdcr: BDCR,
    /// Clock configuration register
    pub cfgr: CFGR,
    /// Clock interrupt register
    pub cir: CIR,
    /// Clock control register
    pub cr: CR,
    /// Clock control and status register
    pub csr: CSR,
    /// Dedicated clocks configuration register 1
    pub dkcfgr1: DKCFGR1,
    /// Dedicated clocks configuration register 2
    pub dkcfgr2: DKCFGR2,
    /// PLL configuration register
    pub pllcfgr: PLLCFGR,
    /// PLLI2S configuration register
    pub plli2scfgr: PLLI2SCFGR,
    /// PLLSAI configuration register
    pub pllsaicfgr: PLLSAICFGR,
    /// Spread spectrum clock generation register
    pub sscgr: SSCGR,
}

/// AMBA High-performance Bus 1 (AHB1) registers
pub struct AHB1 {
    _0: (),
}

impl AHB1 {
    #[allow(dead_code)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1enr }
    }

    #[allow(dead_code)]
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB1LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1lpenr }
    }

    #[allow(dead_code)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1rstr }
    }
}

/// AMBA High-performance Bus 2 (AHB2) registers
pub struct AHB2 {
    _0: (),
}

impl AHB2 {
    #[allow(dead_code)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2enr }
    }

    #[allow(dead_code)]
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB2LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2lpenr }
    }

    #[allow(dead_code)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2rstr }
    }
}

/// AMBA High-performance Bus 3 (AHB3) registers
pub struct AHB3 {
    _0: (),
}

impl AHB3 {
    #[allow(dead_code)]
    pub(crate) fn enr(&mut self) -> &rcc::AHB3ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3enr }
    }

    #[allow(dead_code)]
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB3LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3lpenr }
    }

    #[allow(dead_code)]
    pub(crate) fn rstr(&mut self) -> &rcc::AHB3RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3rstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    #[allow(dead_code)]
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1enr }
    }

    #[allow(dead_code)]
    pub(crate) fn lpenr(&mut self) -> &rcc::APB1LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1lpenr }
    }

    #[allow(dead_code)]
    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    #[allow(dead_code)]
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    #[allow(dead_code)]
    pub(crate) fn lpenr(&mut self) -> &rcc::APB2LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2lpenr }
    }

    #[allow(dead_code)]
    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}

/// Backup domain control register
pub struct BDCR {
    _0: (),
}

impl BDCR {
    #[allow(dead_code)]
    pub(crate) fn bdcr(&mut self) -> &rcc::BDCR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).bdcr }
    }
}

/// Clock configuration register
pub struct CFGR {
    _0: (),
}

impl CFGR {
    #[allow(dead_code)]
    pub(crate) fn cfgr(&mut self) -> &rcc::CFGR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).cfgr }
    }

    pub fn freeze(&mut self, acr: &mut ACR) {
        let rcc = unsafe { &*RCC::ptr() };

        // TODO: Implement lock configuration
        // Adjust flash wait states
        acr.acr().write(|w| w.latency().bits(0b000));

        // Use HSI as source
        rcc.cr.write(|w| w.hsion().set_bit());
        while rcc.cr.read().hsirdy().bit_is_clear() {}

        let hpre_bits = 0b0000; // I got these from the .unwrap_or() values
        let ppre1_bits = 0b0000;
        let ppre2_bits = 0b000;
        let sysclk_src_bits = 0b00;
        // SW: HSI selected as system clock
        rcc.cfgr.write(|w| unsafe {
            w.ppre2()
                .bits(ppre2_bits)
                .ppre1()
                .bits(ppre1_bits)
                .hpre()
                .bits(hpre_bits)
                .sw()
                .bits(sysclk_src_bits)
        });
        while rcc.cfgr.read().sws().bits() != sysclk_src_bits {}
    }
}

/// Clock interrupt register
pub struct CIR {
    _0: (),
}

impl CIR {
    #[allow(dead_code)]
    pub(crate) fn cir(&mut self) -> &rcc::CIR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).cir }
    }
}

/// Clock control register
pub struct CR {
    _0: (),
}

impl CR {
    #[allow(dead_code)]
    pub(crate) fn cr(&mut self) -> &rcc::CR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).cr }
    }
}

/// Clock control and status register
pub struct CSR {
    _0: (),
}

impl CSR {
    #[allow(dead_code)]
    pub(crate) fn csr(&mut self) -> &rcc::CSR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).csr }
    }
}

/// Dedicated clocks configuration register 1
pub struct DKCFGR1 {
    _0: (),
}

impl DKCFGR1 {
    #[allow(dead_code)]
    pub(crate) fn dkcfgr1(&mut self) -> &rcc::DKCFGR1 {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).dkcfgr1 }
    }
}

/// Dedicated clocks configuration register 2
pub struct DKCFGR2 {
    _0: (),
}

impl DKCFGR2 {
    #[allow(dead_code)]
    pub(crate) fn dkcfgr2(&mut self) -> &rcc::DKCFGR2 {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).dkcfgr2 }
    }
}

/// PLL configuration register
pub struct PLLCFGR {
    _0: (),
}

impl PLLCFGR {
    #[allow(dead_code)]
    pub(crate) fn pllcfgr(&mut self) -> &rcc::PLLCFGR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).pllcfgr }
    }
}

/// PLLI2S configuration register
pub struct PLLI2SCFGR {
    _0: (),
}

impl PLLI2SCFGR {
    #[allow(dead_code)]
    pub(crate) fn plli2scfgr(&mut self) -> &rcc::PLLI2SCFGR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).plli2scfgr }
    }
}

/// PLLSAI configuration register
pub struct PLLSAICFGR {
    _0: (),
}

impl PLLSAICFGR {
    #[allow(dead_code)]
    pub(crate) fn pllsaicfgr(&mut self) -> &rcc::PLLSAICFGR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).pllsaicfgr }
    }
}

/// Spread spectrum clock generation register
pub struct SSCGR {
    _0: (),
}

impl SSCGR {
    #[allow(dead_code)]
    pub(crate) fn sscgr(&mut self) -> &rcc::SSCGR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).sscgr }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    hclk: Hertz,
}
