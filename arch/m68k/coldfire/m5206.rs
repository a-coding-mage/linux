// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m5206.c  -- platform support for ColdFire 5206 based boards
 *
 *	Copyright (C) 1999-2002, Greg Ungerer (gerg@snapgear.com)
 * 	Copyright (C) 2000-2001, Lineo Inc. (www.lineo.com) 
 */

/***************************************************************************/

use core::ffi::{c_char, c_int, c_void};

// The following declarations are supplied by the surrounding kernel code.
extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    unsafe fn hw_timer_init();
    unsafe fn mcf_write8(value: u8, address: u32);
    unsafe fn mcf_mapirq2imr(irq: c_int, imr: c_int);
    unsafe fn clkdev_add_table(table: *mut clk_lookup, size: usize);
    unsafe fn memcpy(dest: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_lookup {
    _private: [u8; 0],
}

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);

// CLKDEV_INIT entries are provided by the kernel's clock lookup definitions.
extern "C" {
    fn CLKDEV_INIT(
        con_id: *const c_char,
        dev_id: *const c_char,
        clk: *mut clk,
    ) -> clk_lookup;
}

static mut m5206_clk_lookup: [clk_lookup; 7] = [
    // CLKDEV_INIT(NULL, "pll.0", &clk_pll),
    // CLKDEV_INIT(NULL, "sys.0", &clk_sys),
    // CLKDEV_INIT("mcftmr.0", NULL, &clk_sys),
    // CLKDEV_INIT("mcftmr.1", NULL, &clk_sys),
    // CLKDEV_INIT("mcfuart.0", NULL, &clk_sys),
    // CLKDEV_INIT("mcfuart.1", NULL, &clk_sys),
    // CLKDEV_INIT("imx1-i2c.0", NULL, &clk_sys),
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
    unsafe { core::mem::MaybeUninit::<clk_lookup>::uninit().assume_init() },
];

/***************************************************************************/

unsafe extern "C" fn m5206_i2c_init() {
    // Build-time condition preserved from: IS_ENABLED(CONFIG_I2C_IMX)
    #[cfg(feature = "CONFIG_I2C_IMX")]
    {
        mcf_write8(
            MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL5 | MCFSIM_ICR_PRI0,
            MCFSIM_I2CICR,
        );
        mcf_mapirq2imr(MCF_IRQ_I2C0, MCFINTC_I2C);
    }
}

pub unsafe extern "C" fn config_BSP(commandp: *mut c_char, size: c_int) {
    // Build-time condition preserved from: defined(CONFIG_NETtel)
    #[cfg(feature = "CONFIG_NETtel")]
    {
        /* Copy command line from FLASH to local buffer... */
        memcpy(
            commandp as *mut c_void,
            0xf0004000usize as *const c_void,
            size as usize,
        );
        *commandp.add((size - 1) as usize) = 0;
    }

    mach_sched_init = Some(hw_timer_init);

    /* Only support the external interrupts on their primary level */
    mcf_mapirq2imr(25, MCFINTC_EINT1);
    mcf_mapirq2imr(28, MCFINTC_EINT4);
    mcf_mapirq2imr(31, MCFINTC_EINT7);
    m5206_i2c_init();

    clkdev_add_table(m5206_clk_lookup.as_mut_ptr(), ARRAY_SIZE_m5206_clk_lookup());
}

unsafe fn ARRAY_SIZE_m5206_clk_lookup() -> usize {
    core::mem::size_of_val(&m5206_clk_lookup) / core::mem::size_of::<clk_lookup>()
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
