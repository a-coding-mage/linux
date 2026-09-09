/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/io.h>: these low-level I/O helpers are
// supplied externally.
extern "C" {
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);
}

/*
 * Copyright (C) 2003  Andriy Skulysh
 */

pub const DADR0: usize = 0xa40000a0;
pub const DADR1: usize = 0xa40000a2;
pub const DACR: usize = 0xa40000a4;
pub const DACR_DAOE1: u8 = 0x80;
pub const DACR_DAOE0: u8 = 0x40;
pub const DACR_DAE: u8 = 0x20;

#[inline]
pub unsafe fn sh_dac_enable(channel: i32) {
    let mut v: u8 = __raw_readb(DACR);
    if channel != 0 {
        v |= DACR_DAOE1;
    } else {
        v |= DACR_DAOE0;
    }
    __raw_writeb(v, DACR);
}

#[inline]
pub unsafe fn sh_dac_disable(channel: i32) {
    let mut v: u8 = __raw_readb(DACR);
    if channel != 0 {
        v &= !DACR_DAOE1;
    } else {
        v &= !DACR_DAOE0;
    }
    __raw_writeb(v, DACR);
}

#[inline]
pub unsafe fn sh_dac_output(value: u8, channel: i32) {
    if channel != 0 {
        __raw_writeb(value, DADR1);
    } else {
        __raw_writeb(value, DADR0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
