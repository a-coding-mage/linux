// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/plat-spear/pl080.c
 *
 * DMAC pl080 definitions for SPEAr platform
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

use core::ffi::c_int;

// External Linux/kernel and platform declarations supplied by the surrounding tree.
#[repr(C)]
pub struct pl08x_channel_data {
    pub min_signal: u32,
    pub muxval: u32,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    static mut DMA_CHN_CFG: *mut u32;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn BUG() -> !;
}

const EBUSY: c_int = 16;

static mut lock: spinlock_t = spinlock_t { _private: [] };

#[repr(C)]
#[derive(Copy, Clone)]
struct Signal {
    busy: u8,
    val: u8,
}

static mut signals: [Signal; 16] = [Signal { busy: 0, val: 0 }; 16];

pub unsafe fn pl080_get_signal(cd: *const pl08x_channel_data) -> c_int {
    let signal: u32 = (*cd).min_signal;
    let mut val: u32;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut lock, &mut flags);

    /* Return if signal is already acquired by somebody else */
    if signals[signal as usize].busy != 0
        && (signals[signal as usize].val as u32 != (*cd).muxval)
    {
        spin_unlock_irqrestore(&mut lock, flags);
        return -EBUSY;
    }

    /* If acquiring for the first time, configure it */
    if signals[signal as usize].busy == 0 {
        val = readl(DMA_CHN_CFG as *const u32);

        /*
         * Each request line has two bits in DMA_CHN_CFG register. To
         * goto the bits of current request line, do left shift of
         * value by 2 * signal number.
         */
        val &= !(0x3u32 << (signal * 2));
        val |= (*cd).muxval << (signal * 2);
        writel(val, DMA_CHN_CFG);
    }

    signals[signal as usize].busy = signals[signal as usize].busy.wrapping_add(1);
    signals[signal as usize].val = (*cd).muxval as u8;
    spin_unlock_irqrestore(&mut lock, flags);

    signal as c_int
}

pub unsafe fn pl080_put_signal(cd: *const pl08x_channel_data, signal: c_int) {
    let mut flags: usize = 0;
    let _ = cd;

    spin_lock_irqsave(&mut lock, &mut flags);

    /* if signal is not used */
    if signals[signal as usize].busy == 0 {
        BUG();
    }

    signals[signal as usize].busy = signals[signal as usize].busy.wrapping_sub(1);

    spin_unlock_irqrestore(&mut lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
