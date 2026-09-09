/*
 * ARM PrimeXsys System Controller SP810 header file
 *
 * Copyright (C) 2009 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use core::ffi::c_void;

/* Dependency supplied by the Linux I/O layer. */
unsafe extern "C" {
    fn writel(value: u32, address: *mut c_void);
}

/* sysctl registers offset */
pub const SCCTRL: usize = 0x000;
pub const SCSYSSTAT: usize = 0x004;
pub const SCIMCTRL: usize = 0x008;
pub const SCIMSTAT: usize = 0x00C;
pub const SCXTALCTRL: usize = 0x010;
pub const SCPLLCTRL: usize = 0x014;
pub const SCPLLFCTRL: usize = 0x018;
pub const SCPERCTRL0: usize = 0x01C;
pub const SCPERCTRL1: usize = 0x020;
pub const SCPEREN: usize = 0x024;
pub const SCPERDIS: usize = 0x028;
pub const SCPERCLKEN: usize = 0x02C;
pub const SCPERSTAT: usize = 0x030;
pub const SCSYSID0: usize = 0xEE0;
pub const SCSYSID1: usize = 0xEE4;
pub const SCSYSID2: usize = 0xEE8;
pub const SCSYSID3: usize = 0xEEC;
pub const SCITCR: usize = 0xF00;
pub const SCITIR0: usize = 0xF04;
pub const SCITIR1: usize = 0xF08;
pub const SCITOR: usize = 0xF0C;
pub const SCCNTCTRL: usize = 0xF10;
pub const SCCNTDATA: usize = 0xF14;
pub const SCCNTSTEP: usize = 0xF18;
pub const SCPERIPHID0: usize = 0xFE0;
pub const SCPERIPHID1: usize = 0xFE4;
pub const SCPERIPHID2: usize = 0xFE8;
pub const SCPERIPHID3: usize = 0xFEC;
pub const SCPCELLID0: usize = 0xFF0;
pub const SCPCELLID1: usize = 0xFF4;
pub const SCPCELLID2: usize = 0xFF8;
pub const SCPCELLID3: usize = 0xFFC;

#[inline]
pub const fn scctrl_timerenn_sel_shift(n: usize) -> usize {
    15 + n * 2
}

pub unsafe fn sysctl_soft_reset(base: *mut c_void) {
    /* switch to slow mode */
    writel(0x2, (base as *mut u8).add(SCCTRL) as *mut c_void);

    /* writing any value to SCSYSSTAT reg will reset system */
    writel(0, (base as *mut u8).add(SCSYSSTAT) as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
