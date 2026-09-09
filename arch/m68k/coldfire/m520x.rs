// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/
/*
 *  m520x.c  -- platform support for ColdFire 520x based boards
 *
 *  Copyright (C) 2005,      Freescale (www.freescale.com)
 *  Copyright (C) 2005,      Intec Automation (mike@steroidmicros.com)
 *  Copyright (C) 1999-2007, Greg Ungerer (gerg@snapgear.com)
 *  Copyright (C) 2001-2003, SnapGear Inc. (www.snapgear.com)
 */
/***************************************************************************/

// Linux header dependencies are supplied by the surrounding Rust translation.

DEFINE_CLK!(0, "flexbus", 2, MCF_CLK);
DEFINE_CLK!(0, "fec.0", 12, MCF_CLK);
DEFINE_CLK!(0, "edma", 17, MCF_CLK);
DEFINE_CLK!(0, "intc.0", 18, MCF_CLK);
DEFINE_CLK!(0, "iack.0", 21, MCF_CLK);
DEFINE_CLK!(0, "imx1-i2c.0", 22, MCF_CLK);
DEFINE_CLK!(0, "mcfqspi.0", 23, MCF_CLK);
DEFINE_CLK!(0, "mcfuart.0", 24, MCF_BUSCLK);
DEFINE_CLK!(0, "mcfuart.1", 25, MCF_BUSCLK);
DEFINE_CLK!(0, "mcfuart.2", 26, MCF_BUSCLK);
DEFINE_CLK!(0, "mcftmr.0", 28, MCF_CLK);
DEFINE_CLK!(0, "mcftmr.1", 29, MCF_CLK);
DEFINE_CLK!(0, "mcftmr.2", 30, MCF_CLK);
DEFINE_CLK!(0, "mcftmr.3", 31, MCF_CLK);
DEFINE_CLK!(0, "mcfpit.0", 32, MCF_CLK);
DEFINE_CLK!(0, "mcfpit.1", 33, MCF_CLK);
DEFINE_CLK!(0, "mcfeport.0", 34, MCF_CLK);
DEFINE_CLK!(0, "mcfwdt.0", 35, MCF_CLK);
DEFINE_CLK!(0, "pll.0", 36, MCF_CLK);
DEFINE_CLK!(0, "sys.0", 40, MCF_BUSCLK);
DEFINE_CLK!(0, "gpio.0", 41, MCF_BUSCLK);
DEFINE_CLK!(0, "sdram.0", 42, MCF_CLK);

static mut M520X_CLK_LOOKUP: [ClkLookup; 22] = [
    CLKDEV_INIT!(None, Some("flexbus"), &__clk_0_2),
    CLKDEV_INIT!(Some("fec.0"), None, &__clk_0_12),
    CLKDEV_INIT!(Some("edma"), None, &__clk_0_17),
    CLKDEV_INIT!(Some("intc.0"), None, &__clk_0_18),
    CLKDEV_INIT!(Some("iack.0"), None, &__clk_0_21),
    CLKDEV_INIT!(Some("imx1-i2c.0"), None, &__clk_0_22),
    CLKDEV_INIT!(Some("mcfqspi.0"), None, &__clk_0_23),
    CLKDEV_INIT!(Some("mcfuart.0"), None, &__clk_0_24),
    CLKDEV_INIT!(Some("mcfuart.1"), None, &__clk_0_25),
    CLKDEV_INIT!(Some("mcfuart.2"), None, &__clk_0_26),
    CLKDEV_INIT!(Some("mcftmr.0"), None, &__clk_0_28),
    CLKDEV_INIT!(Some("mcftmr.1"), None, &__clk_0_29),
    CLKDEV_INIT!(Some("mcftmr.2"), None, &__clk_0_30),
    CLKDEV_INIT!(Some("mcftmr.3"), None, &__clk_0_31),
    CLKDEV_INIT!(Some("mcfpit.0"), None, &__clk_0_32),
    CLKDEV_INIT!(Some("mcfpit.1"), None, &__clk_0_33),
    CLKDEV_INIT!(Some("mcfeport.0"), None, &__clk_0_34),
    CLKDEV_INIT!(Some("mcfwdt.0"), None, &__clk_0_35),
    CLKDEV_INIT!(None, Some("pll.0"), &__clk_0_36),
    CLKDEV_INIT!(None, Some("sys.0"), &__clk_0_40),
    CLKDEV_INIT!(Some("gpio.0"), None, &__clk_0_41),
    CLKDEV_INIT!(Some("sdram.0"), None, &__clk_0_42),
];

static ENABLE_CLKS: [&'static Clk; 13] = [
    &__clk_0_2, &__clk_0_18, &__clk_0_21, &__clk_0_24, &__clk_0_25,
    &__clk_0_26, &__clk_0_32, &__clk_0_33, &__clk_0_34, &__clk_0_36,
    &__clk_0_40, &__clk_0_41, &__clk_0_42,
];

static DISABLE_CLKS: [&'static Clk; 9] = [
    &__clk_0_12, &__clk_0_17, &__clk_0_22, &__clk_0_23, &__clk_0_28,
    &__clk_0_29, &__clk_0_30, &__clk_0_31, &__clk_0_35,
];

unsafe fn m520x_clk_init() {
    for clk in ENABLE_CLKS.iter() { __clk_init_enabled(*clk); }
    for clk in DISABLE_CLKS.iter() { __clk_init_disabled(*clk); }
    clkdev_add_table(M520X_CLK_LOOKUP.as_mut_ptr(), M520X_CLK_LOOKUP.len());
}

unsafe fn m520x_qspi_init() {
    // CONFIG_SPI_COLDFIRE_QSPI conditional preserved from the C source.
    #[cfg(feature = "CONFIG_SPI_COLDFIRE_QSPI")]
    {
        mcf_write8(0x3f, MCF_GPIO_PAR_QSPI);
        let mut par = mcf_read16(MCF_GPIO_PAR_UART);
        par &= 0x00ff;
        mcf_write16(par, MCF_GPIO_PAR_UART);
    }
}

unsafe fn m520x_i2c_init() {
    // CONFIG_I2C_IMX conditional preserved from the C source.
    #[cfg(feature = "CONFIG_I2C_IMX")]
    {
        let mut par = mcf_read8(MCF_GPIO_PAR_FECI2C);
        par |= 0x0f;
        mcf_write8(par, MCF_GPIO_PAR_FECI2C);
    }
}

unsafe fn m520x_uarts_init() {
    let mut par = mcf_read16(MCF_GPIO_PAR_UART);
    par |= MCF_GPIO_PAR_UART_PAR_UTXD0 | MCF_GPIO_PAR_UART_PAR_URXD0;
    par |= MCF_GPIO_PAR_UART_PAR_UTXD1 | MCF_GPIO_PAR_UART_PAR_URXD1;
    mcf_write16(par, MCF_GPIO_PAR_UART);

    let mut par2 = mcf_read8(MCF_GPIO_PAR_FECI2C);
    par2 &= !0x0F;
    par2 |= MCF_GPIO_PAR_FECI2C_PAR_SCL_UTXD2 | MCF_GPIO_PAR_FECI2C_PAR_SDA_URXD2;
    mcf_write8(par2, MCF_GPIO_PAR_FECI2C);
}

unsafe fn m520x_fec_init() {
    let mut v = mcf_read8(MCF_GPIO_PAR_FEC);
    mcf_write8(v | 0xf0, MCF_GPIO_PAR_FEC);
    v = mcf_read8(MCF_GPIO_PAR_FECI2C);
    mcf_write8(v | 0x0f, MCF_GPIO_PAR_FECI2C);
}

pub unsafe fn config_BSP(_commandp: *mut c_char, _size: c_int) {
    mach_sched_init = hw_timer_init;
    m520x_clk_init();
    m520x_uarts_init();
    m520x_fec_init();
    m520x_qspi_init();
    m520x_i2c_init();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
