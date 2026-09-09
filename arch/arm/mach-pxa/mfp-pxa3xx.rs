// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-pxa/mfp.c
 *
 * PXA3xx Multi-Function Pin Support
 *
 * Copyright (C) 2007 Marvell Internation Ltd.
 *
 * 2007-08-21: eric miao <eric.miao@marvell.com>
 *             initial version
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn mfp_config_lpm();
    fn mfp_config_run();

    static mut ASCR: u32;
}

// Register bit definitions supplied by pxa3xx-regs.h.
extern "C" {
    static ASCR_RDH: u32;
    static ASCR_D1S: u32;
    static ASCR_D2S: u32;
    static ASCR_D3S: u32;
}

// Declaration supplied by the kernel syscore interface.
#[repr(C)]
pub struct syscore_ops {
    pub suspend: Option<unsafe extern "C" fn(data: *mut c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(data: *mut c_void)>,
}

#[repr(C)]
pub struct syscore {
    pub ops: *const syscore_ops,
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn pxa3xx_mfp_suspend(_data: *mut c_void) -> i32 {
    mfp_config_lpm();
    0
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn pxa3xx_mfp_resume(_data: *mut c_void) {
    mfp_config_run();

    /* clear RDH bit when MFP settings are restored
     *
     * NOTE: the last 3 bits DxS are write-1-to-clear so carefully
     * preserve them here in case they will be referenced later
     */
    ASCR &= !(ASCR_RDH | ASCR_D1S | ASCR_D2S | ASCR_D3S);
}

#[cfg(not(feature = "CONFIG_PM"))]
const pxa3xx_mfp_suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32> = None;

#[cfg(not(feature = "CONFIG_PM"))]
const pxa3xx_mfp_resume: Option<unsafe extern "C" fn(*mut c_void)> = None;

static pxa3xx_mfp_syscore_ops: syscore_ops = syscore_ops {
    suspend: {
        #[cfg(feature = "CONFIG_PM")]
        {
            Some(pxa3xx_mfp_suspend)
        }
        #[cfg(not(feature = "CONFIG_PM"))]
        {
            pxa3xx_mfp_suspend
        }
    },
    resume: {
        #[cfg(feature = "CONFIG_PM")]
        {
            Some(pxa3xx_mfp_resume)
        }
        #[cfg(not(feature = "CONFIG_PM"))]
        {
            pxa3xx_mfp_resume
        }
    },
};

pub static mut pxa3xx_mfp_syscore: syscore = syscore {
    ops: &pxa3xx_mfp_syscore_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
