// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/
/*
 * m5307.c -- platform support for ColdFire 5307 based boards
 *
 * Copyright (C) 1999-2002, Greg Ungerer (gerg@snapgear.com)
 * Copyright (C) 2000, Lineo (www.lineo.com)
 */
/***************************************************************************/

// Linux and architecture headers from the original translation unit provide
// the constants, types, macros, and declarations referenced below.

extern "C" {
    pub static mut mach_sched_init: unsafe extern "C" fn();
    pub fn hw_timer_init();
    pub fn mcf_write8(value: u8, address: u32);
    pub fn mcf_mapirq2imr(irq: i32, source: i32);
    pub fn wdebug(address: u32, value: u32);
    pub fn clkdev_add_table(table: *mut clk_lookup, size: usize);
}

#[repr(C)]
pub struct clk_lookup {
    _opaque: [u8; 0],
}

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);
extern "C" {
    pub static mut clk_pll: clk;
    pub static mut clk_sys: clk;
}

#[repr(C)]
pub struct clk {
    _opaque: [u8; 0],
}

// CLKDEV_INIT entries are supplied by the kernel clock lookup API.
extern "C" {
    pub static mut m5307_clk_lookup: [clk_lookup; 7];
}

/***************************************************************************/

/* Some platforms need software versions of the GPIO data registers. */
pub static mut ppdata: u16 = 0;
pub static mut ledbank: u8 = 0xff;

/***************************************************************************/

unsafe extern "C" fn m5307_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    mcf_write8(
        (MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL5 | MCFSIM_ICR_PRI0) as u8,
        MCFSIM_I2CICR as u32,
    );
    mcf_mapirq2imr(MCF_IRQ_I2C0, MCFINTC_I2C);
    // #endif /* IS_ENABLED(CONFIG_I2C_IMX) */
}

/***************************************************************************/

pub unsafe extern "C" fn config_BSP(commandp: *mut u8, size: i32) {
    // #if defined(CONFIG_NETtel) || defined(CONFIG_SECUREEDGEMP3) || \
    //     defined(CONFIG_CLEOPATRA)
    /* Copy command line from FLASH to local buffer... */
    core::ptr::copy_nonoverlapping(0xf0004000usize as *const u8, commandp, size as usize);
    *commandp.add((size - 1) as usize) = 0;
    // #endif

    mach_sched_init = hw_timer_init;

    /* Only support the external interrupts on their primary level. */
    mcf_mapirq2imr(25, MCFINTC_EINT1);
    mcf_mapirq2imr(27, MCFINTC_EINT3);
    mcf_mapirq2imr(29, MCFINTC_EINT5);
    mcf_mapirq2imr(31, MCFINTC_EINT7);

    // #ifdef CONFIG_BDM_DISABLE
    /*
     * Disable the BDM clocking. This also turns off most of the rest of
     * the BDM device. This is good for EMC reasons. This option is not
     * incompatible with the memory protection option.
     */
    wdebug(MCFDEBUG_CSR, MCFDEBUG_CSR_PSTCLK);
    // #endif
    m5307_i2c_init();

    clkdev_add_table(m5307_clk_lookup.as_mut_ptr(), 7);
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
