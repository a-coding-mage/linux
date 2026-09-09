// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	m5272.c  -- platform support for ColdFire 5272 based boards
 *
 *	Copyright (C) 1999-2002, Greg Ungerer (gerg@snapgear.com)
 *	Copyright (C) 2001-2002, SnapGear Inc. (www.snapgear.com)
 */

/***************************************************************************/

// Dependency declarations supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;

    fn local_irq_disable();
    fn mcf_read32(address: usize) -> u32;
    fn mcf_write32(value: u32, address: usize);
    fn mcf_write16(value: u16, address: usize);
    fn mcf_write8(value: u8, address: usize);
    fn hw_timer_init();
    fn clkdev_add_table(table: *mut clk_lookup, count: usize);
    fn memcpy(destination: *mut c_void, source: *const c_void, count: usize) -> *mut c_void;
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
extern "C" {
    static mut clk_pll: clk;
    static mut clk_sys: clk;
}

/***************************************************************************/

/*
 *	Some platforms need software versions of the GPIO data registers.
 */
#[no_mangle]
pub static mut ppdata: u16 = 0;
#[no_mangle]
pub static mut ledbank: u8 = 0xff;

/***************************************************************************/

// CLKDEV_INIT entries are provided by the kernel clock framework.
static mut m5272_clk_lookup: [clk_lookup; 10] = [
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
    clk_lookup { _private: [] },
];

/***************************************************************************/

unsafe extern "C" fn m5272_uarts_init() {
    let mut v: u32;

    /* Enable the output lines for the serial ports */
    v = mcf_read32(MCFSIM_PBCNT);
    v = (v & !0x000000ff) | 0x00000055;
    mcf_write32(v, MCFSIM_PBCNT);

    v = mcf_read32(MCFSIM_PDCNT);
    v = (v & !0x000003fc) | 0x000002a8;
    mcf_write32(v, MCFSIM_PDCNT);
}

/***************************************************************************/

unsafe extern "C" fn m5272_cpu_reset() {
    local_irq_disable();
    /* Set watchdog to reset, and enabled */
    mcf_write16(0, MCFSIM_WIRR);
    mcf_write16(1, MCFSIM_WRRR);
    mcf_write16(0, MCFSIM_WCR);
    loop {
        /* wait for watchdog to timeout */
    }
}

/***************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn config_BSP(commandp: *mut c_char, size: c_int) {
    #[cfg(CONFIG_MOD5272)]
    {
        /* Set base of device vectors to be 64 */
        mcf_write8(0x40, MCFSIM_PIVR);
    }

    #[cfg(any(CONFIG_NETtel, CONFIG_SCALES))]
    {
        /* Copy command line from FLASH to local buffer... */
        memcpy(commandp as *mut c_void, 0xf0004000usize as *const c_void, size as usize);
        *commandp.add((size - 1) as usize) = 0;
    }
    #[cfg(CONFIG_CANCam)]
    {
        /* Copy command line from FLASH to local buffer... */
        memcpy(commandp as *mut c_void, 0xf0010000usize as *const c_void, size as usize);
        *commandp.add((size - 1) as usize) = 0;
    }

    mach_reset = Some(m5272_cpu_reset);
    mach_sched_init = Some(hw_timer_init);
}

/***************************************************************************/

unsafe extern "C" fn init_BSP() -> c_int {
    m5272_uarts_init();
    clkdev_add_table(m5272_clk_lookup.as_mut_ptr(), 10);
    0
}

// arch_initcall(init_BSP);

/***************************************************************************/

// Symbols supplied by the ColdFire platform headers.
extern "C" {
    static MCFSIM_PBCNT: usize;
    static MCFSIM_PDCNT: usize;
    static MCFSIM_WIRR: usize;
    static MCFSIM_WRRR: usize;
    static MCFSIM_WCR: usize;
    static MCFSIM_PIVR: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
