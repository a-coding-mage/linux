/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ep93xx_soc_model {
    EP93XX_9301_SOC,
    EP93XX_9307_SOC,
    EP93XX_9312_SOC,
}

pub const EP93XX_CHIP_REV_D0: u32 = 3;
pub const EP93XX_CHIP_REV_D1: u32 = 4;
pub const EP93XX_CHIP_REV_E0: u32 = 5;
pub const EP93XX_CHIP_REV_E1: u32 = 6;
pub const EP93XX_CHIP_REV_E2: u32 = 7;

#[repr(C)]
pub struct ep93xx_regmap_adev {
    // struct auxiliary_device adev;
    pub adev: crate::auxiliary_device,
    pub map: *mut regmap,
    // void __iomem *base;
    pub base: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub write: Option<unsafe extern "C" fn(
        map: *mut regmap,
        lock: *mut spinlock_t,
        reg: u32,
        val: u32,
    )>,
    pub update_bits: Option<unsafe extern "C" fn(
        map: *mut regmap,
        lock: *mut spinlock_t,
        reg: u32,
        mask: u32,
        val: u32,
    )>,
}

#[macro_export]
macro_rules! to_ep93xx_regmap_adev {
    ($adev:expr) => {
        crate::container_of!($adev, $crate::ep93xx_regmap_adev, adev)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
