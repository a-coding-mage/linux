// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/
/*
 *	525x.c  -- platform support for ColdFire 525x based boards
 *
 *	Copyright (C) 2012, Steven King <sfking@fdwdc.com>
 */
/***************************************************************************/

// The declarations used below are supplied by the surrounding kernel port.

extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;

    fn mcf_read32(address: u32) -> u32;
    fn mcf_write32(value: u32, address: u32);
    fn mcf_write8(value: u8, address: u32);
    fn mcf_mapirq2imr(irq: u32, imr: u32);
    fn clkdev_add_table(table: *mut clk_lookup, size: usize);

    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    fn hw_timer_init();
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

static mut m525x_clk_lookup: [clk_lookup; 9] = [
    clk_lookup { _private: [] }, // CLKDEV_INIT(NULL, "pll.0", &clk_pll)
    clk_lookup { _private: [] }, // CLKDEV_INIT(NULL, "sys.0", &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("mcftmr.0", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("mcftmr.1", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("mcfuart.0", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("mcfuart.1", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("mcfqspi.0", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("imx1-i2c.0", NULL, &clk_sys)
    clk_lookup { _private: [] }, // CLKDEV_INIT("imx1-i2c.1", NULL, &clk_sys)
];

unsafe extern "C" fn m525x_qspi_init() {
    // #if IS_ENABLED(CONFIG_SPI_COLDFIRE_QSPI)
    /* set the GPIO function for the qspi cs gpios */
    /* FIXME: replace with pinmux/pinctl support */
    let mut f: u32 = mcf_read32(MCFSIM2_GPIOFUNC);
    f |= (1u32 << MCFQSPI_CS2) | (1u32 << MCFQSPI_CS1) | (1u32 << MCFQSPI_CS0);
    mcf_write32(f, MCFSIM2_GPIOFUNC);

    /* QSPI irq setup */
    mcf_write8(
        MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL4 | MCFSIM_ICR_PRI0,
        MCFSIM_QSPIICR,
    );
    mcf_mapirq2imr(MCF_IRQ_QSPI, MCFINTC_QSPI);
    // #endif /* IS_ENABLED(CONFIG_SPI_COLDFIRE_QSPI) */
}

unsafe extern "C" fn m525x_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    let mut r: u32;

    /* first I2C controller uses regular irq setup */
    mcf_write8(
        MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL5 | MCFSIM_ICR_PRI0,
        MCFSIM_I2CICR,
    );
    mcf_mapirq2imr(MCF_IRQ_I2C0, MCFINTC_I2C);

    /* second I2C controller is completely different */
    r = mcf_read32(MCFINTC2_INTPRI_REG(MCF_IRQ_I2C1));
    r &= !MCFINTC2_INTPRI_BITS(0xf, MCF_IRQ_I2C1);
    r |= MCFINTC2_INTPRI_BITS(0x5, MCF_IRQ_I2C1);
    mcf_write32(r, MCFINTC2_INTPRI_REG(MCF_IRQ_I2C1));
    // #endif /* IS_ENABLED(CONFIG_I2C_IMX) */
}

pub unsafe extern "C" fn config_BSP(commandp: *mut i8, size: i32) {
    let _ = (commandp, size);
    mach_sched_init = Some(hw_timer_init);

    m525x_qspi_init();
    m525x_i2c_init();

    clkdev_add_table(m525x_clk_lookup.as_mut_ptr(), m525x_clk_lookup.len());
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
