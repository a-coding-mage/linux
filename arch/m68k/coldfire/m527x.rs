// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m527x.c  -- platform support for ColdFire 527x based boards
 *
 *	Sub-architcture dependent initialization code for the Freescale
 *	5270/5271 and 5274/5275 CPUs.
 *
 *	Copyright (C) 1999-2004, Greg Ungerer (gerg@snapgear.com)
 *	Copyright (C) 2001-2004, SnapGear Inc. (www.snapgear.com)
 */

/***************************************************************************/

// The following symbols and types are supplied by the surrounding kernel.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ClkLookup {
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub clk: *mut c_void,
}

extern "C" {
    static mut clk_pll: c_void;
    static mut clk_sys: c_void;

    static mut mach_sched_init: Option<unsafe extern "C" fn()>;

    fn hw_timer_init();
    fn clkdev_add_table(table: *mut ClkLookup, size: usize);
    fn mcf_write8(value: u8, address: usize);
    fn mcf_read8(address: usize) -> u8;
    fn mcf_write16(value: u16, address: usize);
    fn mcf_read16(address: usize) -> u16;
}

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);

// CLKDEV_INIT/DEFINE_CLK expand to kernel-specific clock lookup initializers.
// Keep the same lookup table and ordering here; the referenced addresses are
// provided by the surrounding kernel integration.
static mut m527x_clk_lookup: [ClkLookup; 13] = [
    ClkLookup { dev_id: core::ptr::null(), con_id: b"pll.0\0".as_ptr() as *const c_char, clk: unsafe { &raw mut clk_pll } },
    ClkLookup { dev_id: core::ptr::null(), con_id: b"sys.0\0".as_ptr() as *const c_char, clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"mcfpit.0\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_pll } },
    ClkLookup { dev_id: b"mcfpit.1\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_pll } },
    ClkLookup { dev_id: b"mcfpit.2\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_pll } },
    ClkLookup { dev_id: b"mcfpit.3\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_pll } },
    ClkLookup { dev_id: b"mcfuart.0\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"mcfuart.1\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"mcfuart.2\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"mcfqspi.0\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"fec.0\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"fec.1\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
    ClkLookup { dev_id: b"imx1-i2c.0\0".as_ptr() as *const c_char, con_id: core::ptr::null(), clk: unsafe { &raw mut clk_sys } },
];

unsafe fn m527x_qspi_init() {
    // CONFIG_SPI_COLDFIRE_QSPI and CONFIG_M5271/CONFIG_M5275 are build-time
    // conditions from the C source and are intentionally preserved here.
    #[cfg(feature = "CONFIG_M5271")]
    {
        mcf_write8(0x1f, MCFGPIO_PAR_QSPI);
        let mut par = mcf_read16(MCFGPIO_PAR_TIMER);
        par &= 0x3f3f;
        mcf_write16(par, MCFGPIO_PAR_TIMER);
    }
    #[cfg(feature = "CONFIG_M5275")]
    mcf_write16(0x003e, MCFGPIO_PAR_QSPI);
}

unsafe fn m527x_i2c_init() {
    #[cfg(feature = "CONFIG_M5271")]
    {
        let mut par = mcf_read8(MCFGPIO_PAR_FECI2C);
        par |= 0x0f;
        mcf_write8(par, MCFGPIO_PAR_FECI2C);
    }
    #[cfg(feature = "CONFIG_M5275")]
    {
        let mut par = mcf_read16(MCFGPIO_PAR_FECI2C);
        par |= 0x0f;
        mcf_write16(par, MCFGPIO_PAR_FECI2C);
    }
}

unsafe fn m527x_uarts_init() {
    /* External Pin Mask Setting & Enable External Pin for Interface */
    let mut sepmask = mcf_read16(MCFGPIO_PAR_UART);
    sepmask |= UART0_ENABLE_MASK | UART1_ENABLE_MASK | UART2_ENABLE_MASK;
    mcf_write16(sepmask, MCFGPIO_PAR_UART);
}

unsafe fn m527x_fec_init() {
    /* Set multi-function pins to ethernet mode for fec0 */
    #[cfg(feature = "CONFIG_M5271")]
    {
        let v = mcf_read8(MCFGPIO_PAR_FECI2C);
        mcf_write8(v | 0xf0, MCFGPIO_PAR_FECI2C);
    }
    #[cfg(not(feature = "CONFIG_M5271"))]
    {
        let mut par = mcf_read16(MCFGPIO_PAR_FECI2C);
        mcf_write16(par | 0xf00, MCFGPIO_PAR_FECI2C);
        let mut v = mcf_read8(MCFGPIO_PAR_FEC0HL);
        mcf_write8(v | 0xc0, MCFGPIO_PAR_FEC0HL);

        /* Set multi-function pins to ethernet mode for fec1 */
        par = mcf_read16(MCFGPIO_PAR_FECI2C);
        mcf_write16(par | 0xa0, MCFGPIO_PAR_FECI2C);
        v = mcf_read8(MCFGPIO_PAR_FEC1HL);
        mcf_write8(v | 0xc0, MCFGPIO_PAR_FEC1HL);
    }
}

pub unsafe extern "C" fn config_BSP(_commandp: *mut c_char, _size: c_int) {
    mach_sched_init = Some(hw_timer_init);
    m527x_uarts_init();
    m527x_fec_init();
    m527x_qspi_init();
    m527x_i2c_init();
    clkdev_add_table(m527x_clk_lookup.as_mut_ptr(), m527x_clk_lookup.len());
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
