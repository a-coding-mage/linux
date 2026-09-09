// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 * m528x.c -- platform support for ColdFire 528x based boards
 *
 * Sub-architecture dependent initialization code for the Freescale
 * 5280, 5281 and 5282 CPUs.
 *
 * Copyright (C) 1999-2003, Greg Ungerer (gerg@snapgear.com)
 * Copyright (C) 2001-2003, SnapGear Inc. (www.snapgear.com)
 */

/***************************************************************************/

// C dependencies supplied by the surrounding kernel translation unit.

// DEFINE_CLK(pll, "pll.0", MCF_CLK);
// DEFINE_CLK(sys, "sys.0", MCF_BUSCLK);
extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;
}

#[repr(C)]
struct clk;

#[repr(C)]
struct clk_lookup {
    dev_id: *const core::ffi::c_char,
    con_id: *const core::ffi::c_char,
    clk: *mut clk,
}

extern "C" {
    fn mcf_read8(addr: usize) -> u8;
    fn mcf_write8(value: u8, addr: usize);
    fn mcf_read16(addr: usize) -> u16;
    fn mcf_write16(value: u16, addr: usize);
    fn clkdev_add_table(table: *mut clk_lookup, size: usize);
    fn hw_timer_init();
    static mut mach_halt: Option<unsafe extern "C" fn()>;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    fn printk(fmt: *const core::ffi::c_char, ...);
}

// CLKDEV_INIT(NULL, "pll.0", &clk_pll), etc.
static mut m528x_clk_lookup: [clk_lookup; 12] = [
    clk_lookup { dev_id: core::ptr::null(), con_id: b"pll.0\0".as_ptr() as _, clk: unsafe { &mut clk_pll } },
    clk_lookup { dev_id: core::ptr::null(), con_id: b"sys.0\0".as_ptr() as _, clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"mcfpit.0\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_pll } },
    clk_lookup { dev_id: b"mcfpit.1\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_pll } },
    clk_lookup { dev_id: b"mcfpit.2\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_pll } },
    clk_lookup { dev_id: b"mcfpit.3\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_pll } },
    clk_lookup { dev_id: b"mcfuart.0\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"mcfuart.1\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"mcfuart.2\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"mcfqspi.0\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"fec.0\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
    clk_lookup { dev_id: b"imx1-i2c.0\0".as_ptr() as _, con_id: core::ptr::null(), clk: unsafe { &mut clk_sys } },
];

unsafe fn m528x_qspi_init() {
    // #if IS_ENABLED(CONFIG_SPI_COLDFIRE_QSPI)
    mcf_write8(0x07, MCFGPIO_PQSPAR);
    // #endif
}

unsafe fn m528x_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    let mut paspar: u16;
    paspar = mcf_read16(MCFGPIO_PASPAR);
    paspar |= 0xF;
    mcf_write16(paspar, MCFGPIO_PASPAR);
    // #endif
}

unsafe fn m528x_uarts_init() {
    let mut port: u8;
    port = mcf_read8(MCFGPIO_PUAPAR);
    port |= 0x03 | (0x03 << 2);
    mcf_write8(port, MCFGPIO_PUAPAR);
}

unsafe fn m528x_fec_init() {
    let v16: u16 = mcf_read16(MCFGPIO_PASPAR);
    mcf_write16(v16 | 0xf00, MCFGPIO_PASPAR);
    mcf_write8(0xc0, MCFGPIO_PEHLPAR);
}

// #ifdef CONFIG_WILDFIRE
unsafe extern "C" fn wildfire_halt() {
    mcf_write8(0, 0x30000007);
    mcf_write8(0x2, 0x30000007);
}
// #endif

// #ifdef CONFIG_WILDFIREMOD
unsafe extern "C" fn wildfiremod_halt() {
    printk(b"WildFireMod hibernating...\n\0".as_ptr() as _);
    mcf_write16(mcf_read16(MCFGPIO_PEPAR) & !(1 << (5 * 2)), MCFGPIO_PEPAR);
    mcf_write8(mcf_read8(MCFGPIO_PDDR_E) | (1 << 5), MCFGPIO_PDDR_E);
    mcf_write8(mcf_read8(MCFGPIO_PODR_E) & !(1 << 5), MCFGPIO_PODR_E);
    mcf_write8(mcf_read8(MCFGPIO_PODR_E) | (1 << 5), MCFGPIO_PODR_E);
    printk(b"Failed to hibernate. Halting!\n\0".as_ptr() as _);
}
// #endif

pub unsafe extern "C" fn config_BSP(_commandp: *mut core::ffi::c_char, _size: i32) {
    // #ifdef CONFIG_WILDFIRE
    mach_halt = Some(wildfire_halt);
    // #endif
    // #ifdef CONFIG_WILDFIREMOD
    mach_halt = Some(wildfiremod_halt);
    // #endif
    mach_sched_init = Some(hw_timer_init);
    m528x_uarts_init();
    m528x_fec_init();
    m528x_qspi_init();
    m528x_i2c_init();
    clkdev_add_table(m528x_clk_lookup.as_mut_ptr(), m528x_clk_lookup.len());
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
