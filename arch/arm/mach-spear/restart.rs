// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/plat-spear/restart.c
 *
 * SPEAr platform specific restart functions
 *
 * Copyright (C) 2009 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

use core::ffi::{c_char, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn soft_restart(addr: usize);
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn sysctl_soft_reset(base: *mut c_void);
}

// VA_MISC_BASE is supplied by spear.h.
pub const SPEAR13XX_SYS_SW_RES: usize = VA_MISC_BASE + 0x204;

pub unsafe fn spear_restart(mode: reboot_mode, _cmd: *const c_char) {
    if mode == REBOOT_SOFT {
        /* software reset, Jump into ROM at address 0 */
        soft_restart(0);
    } else {
        /* hardware reset, Use on-chip reset capability */
        // Preserved from CONFIG_ARCH_SPEAR13XX.
        #[cfg(CONFIG_ARCH_SPEAR13XX)]
        {
            writel_relaxed(0x01, SPEAR13XX_SYS_SW_RES as *mut c_void);
        }

        // Preserved from CONFIG_ARCH_SPEAR3XX || CONFIG_ARCH_SPEAR6XX.
        #[cfg(any(CONFIG_ARCH_SPEAR3XX, CONFIG_ARCH_SPEAR6XX))]
        {
            sysctl_soft_reset(VA_SPEAR_SYS_CTRL_BASE as *mut c_void);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
