/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AM33XX PRM instance offset macros
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel translation:
// prcm-common.h and prm.h

pub const AM33XX_PRM_BASE: usize = 0x44E00000;

/* The C macro delegates address mapping to AM33XX_L4_WK_IO_ADDRESS. */
#[macro_export]
macro_rules! AM33XX_PRM_REGADDR {
    ($inst:expr, $reg:expr) => {
        AM33XX_L4_WK_IO_ADDRESS(AM33XX_PRM_BASE + ($inst) + ($reg))
    };
}

/* PRM instances */
pub const AM33XX_PRM_OCP_SOCKET_MOD: usize = 0x0B00;
pub const AM33XX_PRM_PER_MOD: usize = 0x0C00;
pub const AM33XX_PRM_WKUP_MOD: usize = 0x0D00;
pub const AM33XX_PRM_MPU_MOD: usize = 0x0E00;
pub const AM33XX_PRM_DEVICE_MOD: usize = 0x0F00;
pub const AM33XX_PRM_RTC_MOD: usize = 0x1000;
pub const AM33XX_PRM_GFX_MOD: usize = 0x1100;
pub const AM33XX_PRM_CEFUSE_MOD: usize = 0x1200;

/* PRM.PER_PRM register offsets */
pub const AM33XX_PM_PER_PWRSTST_OFFSET: usize = 0x0008;
pub const AM33XX_PM_PER_PWRSTCTRL_OFFSET: usize = 0x000c;
macro_rules! AM33XX_PM_PER_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_PER_MOD, 0x0008) }; }
macro_rules! AM33XX_PM_PER_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_PER_MOD, 0x000c) }; }

/* PRM.WKUP_PRM register offsets */
pub const AM33XX_PM_WKUP_PWRSTCTRL_OFFSET: usize = 0x0004;
pub const AM33XX_PM_WKUP_PWRSTST_OFFSET: usize = 0x0008;
macro_rules! AM33XX_PM_WKUP_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_WKUP_MOD, 0x0004) }; }
macro_rules! AM33XX_PM_WKUP_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_WKUP_MOD, 0x0008) }; }

/* PRM.MPU_PRM register offsets */
pub const AM33XX_PM_MPU_PWRSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_PM_MPU_PWRSTST_OFFSET: usize = 0x0004;
macro_rules! AM33XX_PM_MPU_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_MPU_MOD, 0x0000) }; }
macro_rules! AM33XX_PM_MPU_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_MPU_MOD, 0x0004) }; }

/* PRM.DEVICE_PRM register offsets */
pub const AM33XX_PRM_RSTCTRL_OFFSET: usize = 0x0000;
macro_rules! AM33XX_PRM_RSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_DEVICE_MOD, 0x0000) }; }

/* PRM.RTC_PRM register offsets */
pub const AM33XX_PM_RTC_PWRSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_PM_RTC_PWRSTST_OFFSET: usize = 0x0004;
macro_rules! AM33XX_PM_RTC_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_RTC_MOD, 0x0000) }; }
macro_rules! AM33XX_PM_RTC_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_RTC_MOD, 0x0004) }; }

/* PRM.GFX_PRM register offsets */
pub const AM33XX_PM_GFX_PWRSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_PM_GFX_PWRSTST_OFFSET: usize = 0x0010;
macro_rules! AM33XX_PM_GFX_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_GFX_MOD, 0x0000) }; }
macro_rules! AM33XX_PM_GFX_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_GFX_MOD, 0x0010) }; }

/* PRM.CEFUSE_PRM register offsets */
pub const AM33XX_PM_CEFUSE_PWRSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_PM_CEFUSE_PWRSTST_OFFSET: usize = 0x0004;
macro_rules! AM33XX_PM_CEFUSE_PWRSTCTRL { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_CEFUSE_MOD, 0x0000) }; }
macro_rules! AM33XX_PM_CEFUSE_PWRSTST { () => { AM33XX_PRM_REGADDR!(AM33XX_PRM_CEFUSE_MOD, 0x0004) }; }

#[repr(C)]
pub struct omap_prcm_init_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn am33xx_prm_init(data: *const omap_prcm_init_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
