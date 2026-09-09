/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Broadcom BCM63xx Processor Monitor Bus shared routines (SMP and reset)
 *
 * Copyright (C) 2015, Broadcom Corporation
 * Author: Florian Fainelli <f.fainelli@gmail.com>
 */

// Dependencies supplied by the surrounding Linux/Rust environment:
// linux/io.h, linux/types.h, linux/delay.h, and linux/err.h.

pub const PMB_CTRL: usize = 0x00;
pub const PMC_PMBM_START: u32 = 1u32 << 31;
pub const PMC_PMBM_TIMEOUT: u32 = 1u32 << 30;
pub const PMC_PMBM_SLAVE_ERR: u32 = 1u32 << 29;
pub const PMC_PMBM_BUSY: u32 = 1u32 << 28;
pub const PMC_PMBM_READ: u32 = 0u32 << 20;
pub const PMC_PMBM_WRITE: u32 = 1u32 << 20;
pub const PMB_WR_DATA: usize = 0x04;
pub const PMB_TIMEOUT: usize = 0x08;
pub const PMB_RD_DATA: usize = 0x0C;

pub const PMB_BUS_ID_SHIFT: u32 = 8;

extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn writel(value: u32, addr: *mut core::ffi::c_void);
    pub fn udelay(usecs: u32);
}

// Perform the low-level PMB master operation, shared between reads and writes.
#[inline]
pub unsafe fn __bpcm_do_op(
    master: *mut core::ffi::c_void,
    addr: u32,
    off: u32,
    op: u32,
) -> i32 {
    let mut timeout: u32 = 1000;
    let mut cmd: u32;

    cmd = PMC_PMBM_START | op | ((addr & 0xff) << 12) | off;
    writel(cmd, master.add(PMB_CTRL));
    loop {
        cmd = readl(master.add(PMB_CTRL));
        if (cmd & PMC_PMBM_START) == 0 {
            return 0;
        }

        if (cmd & PMC_PMBM_SLAVE_ERR) != 0 {
            return -5; // -EIO
        }

        if (cmd & PMC_PMBM_TIMEOUT) != 0 {
            return -110; // -ETIMEDOUT
        }

        udelay(1);
        let old_timeout = timeout;
        timeout = timeout.wrapping_sub(1);
        if old_timeout == 0 {
            break;
        }
    }

    -110 // -ETIMEDOUT
}

#[inline]
pub unsafe fn bpcm_rd(
    master: *mut core::ffi::c_void,
    addr: u32,
    off: u32,
    val: *mut u32,
) -> i32 {
    let ret = __bpcm_do_op(master, addr, off >> 2, PMC_PMBM_READ);
    *val = readl(master.add(PMB_RD_DATA));

    ret
}

#[inline]
pub unsafe fn bpcm_wr(
    master: *mut core::ffi::c_void,
    addr: u32,
    off: u32,
    val: u32,
) -> i32 {
    writel(val, master.add(PMB_WR_DATA));
    let ret = __bpcm_do_op(master, addr, off >> 2, PMC_PMBM_WRITE);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
