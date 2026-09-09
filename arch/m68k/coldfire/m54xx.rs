// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m54xx.c  -- platform support for ColdFire 54xx based boards
 *
 *	Copyright (C) 2010, Philippe De Muyter <phdm@macqel.be>
 */

/***************************************************************************/

// Linux and architecture-specific declarations are supplied by other files.

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);
extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;
}

static mut m54xx_clk_lookup: [clk_lookup; 9] = [
    CLKDEV_INIT(core::ptr::null(), "pll.0", &raw mut clk_pll),
    CLKDEV_INIT(core::ptr::null(), "sys.0", &raw mut clk_sys),
    CLKDEV_INIT("mcfslt.0", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("mcfslt.1", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("mcfuart.0", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("mcfuart.1", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("mcfuart.2", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("mcfuart.3", core::ptr::null(), &raw mut clk_sys),
    CLKDEV_INIT("imx1-i2c.0", core::ptr::null(), &raw mut clk_sys),
];

/***************************************************************************/

unsafe fn m54xx_uarts_init() {
    /* enable io pins */
    mcf_write8(MCF_PAR_PSC_TXD | MCF_PAR_PSC_RXD, MCFGPIO_PAR_PSC0);
    mcf_write8(
        MCF_PAR_PSC_TXD | MCF_PAR_PSC_RXD | MCF_PAR_PSC_RTS_RTS,
        MCFGPIO_PAR_PSC1,
    );
    mcf_write8(
        MCF_PAR_PSC_TXD | MCF_PAR_PSC_RXD | MCF_PAR_PSC_RTS_RTS | MCF_PAR_PSC_CTS_CTS,
        MCFGPIO_PAR_PSC2,
    );
    mcf_write8(MCF_PAR_PSC_TXD | MCF_PAR_PSC_RXD, MCFGPIO_PAR_PSC3);
}

/***************************************************************************/

unsafe fn m54xx_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    #[cfg(feature = "CONFIG_I2C_IMX")]
    {
        let mut r: u32;

        /* set the fec/i2c/irq pin assignment register for i2c */
        r = mcf_read32(MCF_PAR_FECI2CIRQ);
        r |= MCF_PAR_FECI2CIRQ_SDA | MCF_PAR_FECI2CIRQ_SCL;
        mcf_write32(r, MCF_PAR_FECI2CIRQ);
    }
    // #endif /* IS_ENABLED(CONFIG_I2C_IMX) */
}

/***************************************************************************/

unsafe fn mcf54xx_reset() {
    /* disable interrupts and enable the watchdog */
    core::arch::asm!("movew #0x2700, %sr");
    mcf_write32(0, MCF_GPT_GMS0);
    mcf_write32(MCF_GPT_GCIR_CNT(1), MCF_GPT_GCIR0);
    mcf_write32(
        MCF_GPT_GMS_WDEN | MCF_GPT_GMS_CE | MCF_GPT_GMS_TMS(4),
        MCF_GPT_GMS0,
    );
}

/***************************************************************************/

pub unsafe extern "C" fn config_BSP(commandp: *mut core::ffi::c_char, size: i32) {
    mach_reset = Some(mcf54xx_reset);
    mach_sched_init = Some(hw_timer_init);
    m54xx_uarts_init();
    m54xx_i2c_init();

    clkdev_add_table(m54xx_clk_lookup.as_mut_ptr(), ARRAY_SIZE(m54xx_clk_lookup));
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
