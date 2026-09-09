// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m523x.c  -- platform support for ColdFire 523x based boards
 *
 *	Sub-architcture dependent initialization code for the Freescale
 *	523x CPUs.
 *
 *	Copyright (C) 1999-2005, Greg Ungerer (gerg@snapgear.com)
 *	Copyright (C) 2001-2003, SnapGear Inc. (www.snapgear.com)
 */

/***************************************************************************/

// Dependency declarations supplied by the surrounding kernel translation.
use crate::{ARRAY_SIZE, MCF_BUSCLK, MCF_CLK, MCFGPIO_PAR_FECI2C,
    MCFGPIO_PAR_QSPI, MCFGPIO_PAR_TIMER};

extern "C" {
    static mut clk_pll: crate::clk;
    static mut clk_sys: crate::clk;
    fn hw_timer_init();
    fn clkdev_add_table(table: *mut crate::clk_lookup, num: usize);
    fn mcf_write8(value: u8, address: usize);
    fn mcf_read8(address: usize) -> u8;
    fn mcf_write16(value: u16, address: usize);
    fn mcf_read16(address: usize) -> u16;
}

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);

static mut m523x_clk_lookup: [crate::clk_lookup; 12] = [
    crate::CLKDEV_INIT(None, Some("pll.0"), unsafe { &mut clk_pll }),
    crate::CLKDEV_INIT(None, Some("sys.0"), unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("mcfpit.0"), None, unsafe { &mut clk_pll }),
    crate::CLKDEV_INIT(Some("mcfpit.1"), None, unsafe { &mut clk_pll }),
    crate::CLKDEV_INIT(Some("mcfpit.2"), None, unsafe { &mut clk_pll }),
    crate::CLKDEV_INIT(Some("mcfpit.3"), None, unsafe { &mut clk_pll }),
    crate::CLKDEV_INIT(Some("mcfuart.0"), None, unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("mcfuart.1"), None, unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("mcfuart.2"), None, unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("mcfqspi.0"), None, unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("fec.0"), None, unsafe { &mut clk_sys }),
    crate::CLKDEV_INIT(Some("imx1-i2c.0"), None, unsafe { &mut clk_sys }),
];

/***************************************************************************/

unsafe fn m523x_qspi_init() {
    // IS_ENABLED(CONFIG_SPI_COLDFIRE_QSPI)
    #[cfg(feature = "CONFIG_SPI_COLDFIRE_QSPI")]
    {
        let mut par: u16;

        /* setup QSPS pins for QSPI with gpio CS control */
        mcf_write8(0x1f, MCFGPIO_PAR_QSPI);
        /* and CS2 & CS3 as gpio */
        par = mcf_read16(MCFGPIO_PAR_TIMER);
        par &= 0x3f3f;
        mcf_write16(par, MCFGPIO_PAR_TIMER);
    }
}

/***************************************************************************/

unsafe fn m523x_i2c_init() {
    // IS_ENABLED(CONFIG_I2C_IMX)
    #[cfg(feature = "CONFIG_I2C_IMX")]
    {
        let mut par: u8;

        /* setup Port AS Pin Assignment Register for I2C */
        /*  set PASPA0 to SCL and PASPA1 to SDA */
        par = mcf_read8(MCFGPIO_PAR_FECI2C);
        par |= 0x0f;
        mcf_write8(par, MCFGPIO_PAR_FECI2C);
    }
}

/***************************************************************************/

unsafe fn m523x_fec_init() {
    /* Set multi-function pins to ethernet use */
    mcf_write8(mcf_read8(MCFGPIO_PAR_FECI2C) | 0xf0, MCFGPIO_PAR_FECI2C);
}

/***************************************************************************/

pub unsafe fn config_BSP(commandp: *mut u8, size: i32) {
    let _ = (commandp, size);
    crate::mach_sched_init = Some(hw_timer_init);
    m523x_fec_init();
    m523x_qspi_init();
    m523x_i2c_init();

    clkdev_add_table(m523x_clk_lookup.as_mut_ptr(), ARRAY_SIZE(&m523x_clk_lookup));
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
