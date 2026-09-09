/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AM33XX CM offset macros
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 * Vaibhav Hiremath <hvaibhav@ti.com>
 */

// Dependencies supplied by the corresponding translated headers:
// "cm.h", "cm-regbits-33xx.h", and "prcm-common.h".

/* CM base address */
pub const AM33XX_CM_BASE: usize = 0x44e00000;

macro_rules! AM33XX_CM_REGADDR {
    ($inst:expr, $reg:expr) => {
        AM33XX_L4_WK_IO_ADDRESS!(AM33XX_CM_BASE + ($inst) + ($reg))
    };
}

/* CM instances */
pub const AM33XX_CM_PER_MOD: usize = 0x0000;
pub const AM33XX_CM_WKUP_MOD: usize = 0x0400;
pub const AM33XX_CM_DPLL_MOD: usize = 0x0500;
pub const AM33XX_CM_MPU_MOD: usize = 0x0600;
pub const AM33XX_CM_DEVICE_MOD: usize = 0x0700;
pub const AM33XX_CM_RTC_MOD: usize = 0x0800;
pub const AM33XX_CM_GFX_MOD: usize = 0x0900;
pub const AM33XX_CM_CEFUSE_MOD: usize = 0x0A00;

/* CM.PER_CM register offsets */
pub const AM33XX_CM_PER_L4LS_CLKSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_CM_PER_L4LS_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0000);
pub const AM33XX_CM_PER_L3S_CLKSTCTRL_OFFSET: usize = 0x0004;
pub const AM33XX_CM_PER_L3S_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0004);
pub const AM33XX_CM_PER_L4FW_CLKSTCTRL_OFFSET: usize = 0x0008;
pub const AM33XX_CM_PER_L4FW_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0008);
pub const AM33XX_CM_PER_L3_CLKSTCTRL_OFFSET: usize = 0x000c;
pub const AM33XX_CM_PER_L3_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x000c);
pub const AM33XX_CM_PER_EMIF_CLKCTRL_OFFSET: usize = 0x0028;
pub const AM33XX_CM_PER_EMIF_CLKCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0028);
pub const AM33XX_CM_PER_L4HS_CLKSTCTRL_OFFSET: usize = 0x011c;
pub const AM33XX_CM_PER_L4HS_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x011c);
pub const AM33XX_CM_PER_OCPWP_L3_CLKSTCTRL_OFFSET: usize = 0x012c;
pub const AM33XX_CM_PER_OCPWP_L3_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x012c);
pub const AM33XX_CM_PER_PRUSS_CLKSTCTRL_OFFSET: usize = 0x0140;
pub const AM33XX_CM_PER_PRUSS_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0140);
pub const AM33XX_CM_PER_CPSW_CLKSTCTRL_OFFSET: usize = 0x0144;
pub const AM33XX_CM_PER_CPSW_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0144);
pub const AM33XX_CM_PER_LCDC_CLKSTCTRL_OFFSET: usize = 0x0148;
pub const AM33XX_CM_PER_LCDC_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0148);
pub const AM33XX_CM_PER_CLK_24MHZ_CLKSTCTRL_OFFSET: usize = 0x0150;
pub const AM33XX_CM_PER_CLK_24MHZ_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_PER_MOD, 0x0150);

/* CM.WKUP_CM register offsets */
pub const AM33XX_CM_WKUP_CLKSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_CM_WKUP_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_WKUP_MOD, 0x0000);
pub const AM33XX_CM_L3_AON_CLKSTCTRL_OFFSET: usize = 0x0018;
pub const AM33XX_CM_L3_AON_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_WKUP_MOD, 0x0018);
pub const AM33XX_CM_L4_WKUP_AON_CLKSTCTRL_OFFSET: usize = 0x00cc;
pub const AM33XX_CM_L4_WKUP_AON_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_WKUP_MOD, 0x00cc);

/* CM.DPLL_CM register offsets */
pub const AM33XX_CLKSEL_GFX_FCLK: usize = AM33XX_CM_REGADDR!(AM33XX_CM_DPLL_MOD, 0x002c);

/* CM.MPU_CM register offsets */
pub const AM33XX_CM_MPU_CLKSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_CM_MPU_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_MPU_MOD, 0x0000);
pub const AM33XX_CM_MPU_MPU_CLKCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_MPU_MOD, 0x0004);

/* CM.DEVICE_CM register offsets */

/* CM.RTC_CM register offsets */
pub const AM33XX_CM_RTC_CLKSTCTRL_OFFSET: usize = 0x0004;
pub const AM33XX_CM_RTC_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_RTC_MOD, 0x0004);

/* CM.GFX_CM register offsets */
pub const AM33XX_CM_GFX_L3_CLKSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_CM_GFX_L3_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_GFX_MOD, 0x0000);
pub const AM33XX_CM_GFX_L4LS_GFX_CLKSTCTRL__1_OFFSET: usize = 0x000c;
pub const AM33XX_CM_GFX_L4LS_GFX_CLKSTCTRL__1: usize = AM33XX_CM_REGADDR!(AM33XX_CM_GFX_MOD, 0x000c);

/* CM.CEFUSE_CM register offsets */
pub const AM33XX_CM_CEFUSE_CLKSTCTRL_OFFSET: usize = 0x0000;
pub const AM33XX_CM_CEFUSE_CLKSTCTRL: usize = AM33XX_CM_REGADDR!(AM33XX_CM_CEFUSE_MOD, 0x0000);

#[repr(C)]
pub struct omap_prcm_init_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn am33xx_cm_init(data: *const omap_prcm_init_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
