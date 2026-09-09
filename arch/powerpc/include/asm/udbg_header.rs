/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * (c) 2001, 2006 IBM Corporation.
 */

// This header's declarations are intended for the kernel build (__KERNEL__).

use core::ffi::{c_char, c_int, c_uchar, c_void};

extern "C" {
    pub static mut udbg_putc: Option<unsafe extern "C" fn(c: c_char)>;
    pub static mut udbg_flush: Option<unsafe extern "C" fn()>;
    pub static mut udbg_getc: Option<unsafe extern "C" fn() -> c_int>;
    pub static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> c_int>;

    pub fn udbg_puts(s: *const c_char);
    pub fn udbg_write(s: *const c_char, n: c_int) -> c_int;

    pub fn register_early_udbg_console();
    pub fn udbg_printf(fmt: *const c_char, ...);
    pub fn udbg_progress(s: *mut c_char, hex: u16);

    // __init
    pub fn udbg_uart_init_mmio(addr: *mut c_void, stride: u32);
    // __init
    pub fn udbg_uart_init_pio(port: usize, stride: u32);

    // __init
    pub fn udbg_uart_setup(speed: u32, clock: u32);
    // __init
    pub fn udbg_probe_uart_speed(clock: u32) -> u32;

    pub fn udbg_scc_init(force_scc: c_int);
    pub fn udbg_adb_init(force_btext: c_int) -> c_int;
    pub fn udbg_adb_init_early();

    // __init
    pub fn udbg_early_init();
    // __init
    pub fn udbg_init_debug_lpar();
    // __init
    pub fn udbg_init_debug_lpar_hvsi();
    // __init
    pub fn udbg_init_pmac_realmode();
    // __init
    pub fn udbg_init_pas_realmode();
    // __init
    pub fn udbg_init_rtas_panel();
    // __init
    pub fn udbg_init_rtas_console();
    // __init
    pub fn udbg_init_btext();
    // __init
    pub fn udbg_init_44x_as1();
    // __init
    pub fn udbg_init_cpm();
    // __init
    pub fn udbg_init_usbgecko();
    // __init
    pub fn udbg_init_memcons();
    // __init
    pub fn udbg_init_ehv_bc();
    // __init
    pub fn udbg_init_ps3gelic();
    // __init
    pub fn udbg_init_debug_opal_raw();
    // __init
    pub fn udbg_init_debug_opal_hvsi();
    // __init
    pub fn udbg_init_debug_16550();
}

// Forward declaration: struct device_node;
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
