/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2xxx Power/Reset Management (PRM) register definitions
 *
 * Copyright (C) 2007-2009, 2011-2012 Texas Instruments, Inc.
 * Copyright (C) 2008-2010 Nokia Corporation
 * Paul Walmsley
 *
 * The PRM hardware modules on the OMAP2/3 are quite similar to each
 * other.  The PRM on OMAP4 has a new register layout, and is handled
 * in a separate file.
 */

// Dependencies supplied by the surrounding translation unit:
// prcm-common.h, prm.h, and prm2xxx_3xxx.h.

macro_rules! OMAP2420_PRM_REGADDR {
    ($module:expr, $reg:expr) => {
        OMAP2_L4_IO_ADDRESS(OMAP2420_PRM_BASE + ($module) + ($reg))
    };
}

macro_rules! OMAP2430_PRM_REGADDR {
    ($module:expr, $reg:expr) => {
        OMAP2_L4_IO_ADDRESS(OMAP2430_PRM_BASE + ($module) + ($reg))
    };
}

/* OMAP2-specific global PRM registers. */
pub const OMAP2_PRCM_REVISION_OFFSET: usize = 0x0000;
pub const OMAP2420_PRCM_REVISION: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0000);
pub const OMAP2_PRCM_SYSCONFIG_OFFSET: usize = 0x0010;
pub const OMAP2420_PRCM_SYSCONFIG: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0010);

pub const OMAP2_PRCM_IRQSTATUS_MPU_OFFSET: usize = 0x0018;
pub const OMAP2420_PRCM_IRQSTATUS_MPU: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0018);
pub const OMAP2_PRCM_IRQENABLE_MPU_OFFSET: usize = 0x001c;
pub const OMAP2420_PRCM_IRQENABLE_MPU: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x001c);

pub const OMAP2_PRCM_VOLTCTRL_OFFSET: usize = 0x0050;
pub const OMAP2420_PRCM_VOLTCTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0050);
pub const OMAP2_PRCM_VOLTST_OFFSET: usize = 0x0054;
pub const OMAP2420_PRCM_VOLTST: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0054);
pub const OMAP2_PRCM_CLKSRC_CTRL_OFFSET: usize = 0x0060;
pub const OMAP2420_PRCM_CLKSRC_CTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0060);
pub const OMAP2_PRCM_CLKOUT_CTRL_OFFSET: usize = 0x0070;
pub const OMAP2420_PRCM_CLKOUT_CTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0070);
pub const OMAP2_PRCM_CLKEMUL_CTRL_OFFSET: usize = 0x0078;
pub const OMAP2420_PRCM_CLKEMUL_CTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0078);
pub const OMAP2_PRCM_CLKCFG_CTRL_OFFSET: usize = 0x0080;
pub const OMAP2420_PRCM_CLKCFG_CTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0080);
pub const OMAP2_PRCM_CLKCFG_STATUS_OFFSET: usize = 0x0084;
pub const OMAP2420_PRCM_CLKCFG_STATUS: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0084);
pub const OMAP2_PRCM_VOLTSETUP_OFFSET: usize = 0x0090;
pub const OMAP2420_PRCM_VOLTSETUP: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0090);
pub const OMAP2_PRCM_CLKSSETUP_OFFSET: usize = 0x0094;
pub const OMAP2420_PRCM_CLKSSETUP: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0094);
pub const OMAP2_PRCM_POLCTRL_OFFSET: usize = 0x0098;
pub const OMAP2420_PRCM_POLCTRL: usize = OMAP2420_PRM_REGADDR!(OCP_MOD, 0x0098);

pub const OMAP2430_PRCM_REVISION: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0000);
pub const OMAP2430_PRCM_SYSCONFIG: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0010);
pub const OMAP2430_PRCM_IRQSTATUS_MPU: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0018);
pub const OMAP2430_PRCM_IRQENABLE_MPU: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x001c);
pub const OMAP2430_PRCM_VOLTCTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0050);
pub const OMAP2430_PRCM_VOLTST: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0054);
pub const OMAP2430_PRCM_CLKSRC_CTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0060);
pub const OMAP2430_PRCM_CLKOUT_CTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0070);
pub const OMAP2430_PRCM_CLKEMUL_CTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0078);
pub const OMAP2430_PRCM_CLKCFG_CTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0080);
pub const OMAP2430_PRCM_CLKCFG_STATUS: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0084);
pub const OMAP2430_PRCM_VOLTSETUP: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0090);
pub const OMAP2430_PRCM_CLKSSETUP: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0094);
pub const OMAP2430_PRCM_POLCTRL: usize = OMAP2430_PRM_REGADDR!(OCP_MOD, 0x0098);

/* Module-specific PRM register offsets. */
pub const OMAP2_RM_RSTCTRL: usize = 0x0050;
pub const OMAP2_RM_RSTTIME: usize = 0x0054;
pub const OMAP2_RM_RSTST: usize = 0x0058;
pub const OMAP2_PM_PWSTCTRL: usize = 0x00e0;
pub const OMAP2_PM_PWSTST: usize = 0x00e4;
pub const PM_WKEN: usize = 0x00a0;
pub const PM_WKEN1: usize = PM_WKEN;
pub const PM_WKST: usize = 0x00b0;
pub const PM_WKST1: usize = PM_WKST;
pub const PM_WKDEP: usize = 0x00c8;
pub const PM_EVGENCTRL: usize = 0x00d4;
pub const PM_EVGENONTIM: usize = 0x00d8;
pub const PM_EVGENOFFTIM: usize = 0x00dc;
pub const OMAP24XX_PM_WKEN2: usize = 0x00a4;
pub const OMAP24XX_PM_WKST2: usize = 0x00b4;
pub const OMAP24XX_PRCM_IRQSTATUS_DSP: usize = 0x00f0;
pub const OMAP24XX_PRCM_IRQENABLE_DSP: usize = 0x00f4;
pub const OMAP24XX_PRCM_IRQSTATUS_IVA: usize = 0x00f8;
pub const OMAP24XX_PRCM_IRQENABLE_IVA: usize = 0x00fc;

extern "C" {
    pub fn omap2xxx_clkdm_sleep(clkdm: *mut clockdomain) -> i32;
    pub fn omap2xxx_clkdm_wakeup(clkdm: *mut clockdomain) -> i32;
    pub fn omap2xxx_prm_init(data: *const omap_prcm_init_data) -> i32;
}

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_prcm_init_data {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
