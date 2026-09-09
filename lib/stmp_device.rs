// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 1999 ARM Limited
 * Copyright (C) 2000 Deep Blue Solutions Ltd
 * Copyright 2006-2007,2010 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 * Copyright 2009 Ilya Yanok, Emcraft Systems Ltd, yanok@emcraft.com
 * Copyright (C) 2011 Wolfram Sang, Pengutronix e.K.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *const c_void) -> u32;
    fn udelay(usecs: u32);
    fn pr_err(format: *const u8, ...);
}

// STMP_OFFSET_REG_CLR is supplied by the translated stmp_device definitions.
extern "C" {
    static STMP_OFFSET_REG_CLR: usize;
    static STMP_OFFSET_REG_SET: usize;
}

const STMP_MODULE_CLKGATE: u32 = 1u32 << 30;
const STMP_MODULE_SFTRST: u32 = 1u32 << 31;

/*
 * Clear the bit and poll it cleared.  This is usually called with
 * a reset address and mask being either SFTRST(bit 31) or CLKGATE
 * (bit 30).
 */
unsafe fn stmp_clear_poll_bit(addr: *mut c_void, mask: u32) -> i32 {
    let mut timeout: i32 = 0x400;

    writel(mask, (addr as *mut u8).add(STMP_OFFSET_REG_CLR) as *mut c_void);
    udelay(1);
    while (readl(addr as *const c_void) & mask != 0 && {
        timeout -= 1;
        timeout != 0
    }) {
        // nothing
    }

    if timeout == 0 { 1 } else { 0 }
}

pub unsafe fn stmp_reset_block(reset_addr: *mut c_void) -> i32 {
    let ret: i32;
    let mut timeout: i32 = 0x400;

    /* clear and poll SFTRST */
    ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_SFTRST);
    if ret != 0 {
        goto_error(reset_addr);
        return -110;
    }

    /* clear CLKGATE */
    writel(
        STMP_MODULE_CLKGATE,
        (reset_addr as *mut u8).add(STMP_OFFSET_REG_CLR) as *mut c_void,
    );

    /* set SFTRST to reset the block */
    writel(
        STMP_MODULE_SFTRST,
        (reset_addr as *mut u8).add(STMP_OFFSET_REG_SET) as *mut c_void,
    );
    udelay(1);

    /* poll CLKGATE becoming set */
    while (readl(reset_addr as *const c_void) & STMP_MODULE_CLKGATE == 0) && {
        timeout -= 1;
        timeout != 0
    } {
        // nothing
    }
    if timeout == 0 {
        goto_error(reset_addr);
        return -110;
    }

    /* clear and poll SFTRST */
    let ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_SFTRST);
    if ret != 0 {
        goto_error(reset_addr);
        return -110;
    }

    /* clear and poll CLKGATE */
    let ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_CLKGATE);
    if ret != 0 {
        goto_error(reset_addr);
        return -110;
    }

    return 0;
}

unsafe fn goto_error(reset_addr: *mut c_void) {
    static MESSAGE: &[u8] = b"%s(%p): module reset timeout\n\0";
    pr_err(MESSAGE.as_ptr(), b"stmp_reset_block\0".as_ptr(), reset_addr);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
