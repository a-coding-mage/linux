// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m5407.c  -- platform support for ColdFire 5407 based boards
 *
 *	Copyright (C) 1999-2002, Greg Ungerer (gerg@snapgear.com)
 *	Copyright (C) 2000, Lineo (www.lineo.com)
 */

/***************************************************************************/

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn mcf_write8(value: u8, address: u32);
    fn mcf_mapirq2imr(irq: i32, interrupt: i32);
    fn clkdev_add_table(table: *mut clk_lookup, size: usize);
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    fn hw_timer_init();
}

#[repr(C)]
pub struct clk_lookup {
    _opaque: [u8; 0],
}

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);
extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;
}

#[repr(C)]
pub struct clk {
    _opaque: [u8; 0],
}

// CLKDEV_INIT entries are supplied by the kernel clock-device framework.
extern "C" {
    static mut m5407_clk_lookup: [clk_lookup; 7];
}

/***************************************************************************/

unsafe fn m5407_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    mcf_write8(
        MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL5 | MCFSIM_ICR_PRI0,
        MCFSIM_I2CICR,
    );
    mcf_mapirq2imr(MCF_IRQ_I2C0, MCFINTC_I2C);
    // #endif /* IS_ENABLED(CONFIG_I2C_IMX) */
}

/***************************************************************************/

pub unsafe extern "C" fn config_BSP(_commandp: *mut i8, _size: i32) {
    mach_sched_init = Some(hw_timer_init);

    /* Only support the external interrupts on their primary level */
    mcf_mapirq2imr(25, MCFINTC_EINT1);
    mcf_mapirq2imr(27, MCFINTC_EINT3);
    mcf_mapirq2imr(29, MCFINTC_EINT5);
    mcf_mapirq2imr(31, MCFINTC_EINT7);
    m5407_i2c_init();

    clkdev_add_table(m5407_clk_lookup.as_mut_ptr(), ARRAY_SIZE(m5407_clk_lookup));
}

/***************************************************************************/

// Constants and ARRAY_SIZE are provided by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
