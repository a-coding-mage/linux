// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// Dependencies supplied by the surrounding platform/kernel bindings.
use core::ffi::c_void;

extern "C" {
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn __raw_readl(addr: *const c_void) -> u32;
}

// Oscillators, PLL & clocks
const ICLK_MASK: u32 = 0x00000080;
const PLLDIV_MASK: u32 = 0x00000007;
const CUROSC_MASK: u32 = 0x00000007;
const PLLMUL_MASK: u32 = 0x0000007F;
const PB_MASK: u32 = 0x00000007;
const FRC1: i32 = 0;
const FRC2: i32 = 7;
const SPLL: i32 = 1;
const POSC: i32 = 2;
const FRC_CLK: u32 = 8000000;

const PIC32_POSC_FREQ: u32 = 24000000;

const OSCCON: usize = 0x0000;
const SPLLCON: usize = 0x0020;
const PB1DIV: u32 = 0x0140;

pub unsafe fn pic32_get_sysclk() -> u32 {
    let mut osc_freq: u32 = 0;
    let pllclk: u32;
    let frcdivn: u32;
    let osccon: u32;
    let spllcon: u32;
    let curr_osc: i32;

    let plliclk: u32;
    let pllidiv: u32;
    let mut pllodiv: u32;
    let pllmult: u32;
    let frcdiv: u32;

    let osc_base = ioremap(PIC32_BASE_OSC as usize, 0x200);

    osccon = __raw_readl(osc_base.add(OSCCON) as *const c_void);
    spllcon = __raw_readl(osc_base.add(SPLLCON) as *const c_void);

    plliclk = spllcon & ICLK_MASK;
    pllidiv = ((spllcon >> 8) & PLLDIV_MASK) + 1;
    pllodiv = (spllcon >> 24) & PLLDIV_MASK;
    pllmult = ((spllcon >> 16) & PLLMUL_MASK) + 1;
    frcdiv = (osccon >> 24) & PLLDIV_MASK;

    pllclk = if plliclk != 0 { FRC_CLK } else { PIC32_POSC_FREQ };
    frcdivn = ((1u32 << frcdiv) + 1) + (128 * (frcdiv == 7) as u32);

    if pllodiv < 2 {
        pllodiv = 2;
    } else if pllodiv < 5 {
        pllodiv = 1u32 << pllodiv;
    } else {
        pllodiv = 32;
    }

    curr_osc = ((osccon >> 12) & CUROSC_MASK) as i32;

    match curr_osc {
        FRC1 | FRC2 => {
            osc_freq = FRC_CLK / frcdivn;
        }
        SPLL => {
            osc_freq = ((pllclk / pllidiv) * pllmult) / pllodiv;
        }
        POSC => {
            osc_freq = PIC32_POSC_FREQ;
        }
        _ => {}
    }

    iounmap(osc_base);

    osc_freq
}

pub unsafe fn pic32_get_pbclk(bus: i32) -> u32 {
    let clk_freq: u32;
    let osc_base = ioremap(PIC32_BASE_OSC as usize, 0x200);
    let pbxdiv = PB1DIV.wrapping_add(((bus - 1) * 0x10) as u32);
    let pbdiv = (__raw_readl(osc_base.add(pbxdiv as usize) as *const c_void) & PB_MASK) + 1;

    iounmap(osc_base);

    clk_freq = pic32_get_sysclk();

    clk_freq / pbdiv
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
