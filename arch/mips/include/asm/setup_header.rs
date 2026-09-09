/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

unsafe extern "C" {
    pub fn prom_putchar(ch: c_char);
    pub fn setup_early_printk();
}

// CONFIG_EARLY_PRINTK_8250 controls whether the external implementation is available.
#[cfg(feature = "CONFIG_EARLY_PRINTK_8250")]
unsafe extern "C" {
    pub fn setup_8250_early_printk_port(
        base: c_ulong,
        reg_shift: c_uint,
        timeout: c_uint,
    );
}

#[cfg(not(feature = "CONFIG_EARLY_PRINTK_8250"))]
#[inline]
pub unsafe fn setup_8250_early_printk_port(
    _base: c_ulong,
    _reg_shift: c_uint,
    _timeout: c_uint,
) {
}

unsafe extern "C" {
    pub fn set_handler(offset: c_ulong, addr: *const c_void, len: c_ulong);
    pub fn set_uncached_handler(offset: c_ulong, addr: *mut c_void, len: c_ulong);
}

pub type vi_handler_t = unsafe extern "C" fn();

unsafe extern "C" {
    pub fn set_vi_handler(n: c_int, addr: vi_handler_t) -> *mut c_void;
    pub fn set_except_vector(n: c_int, addr: *mut c_void) -> *mut c_void;
    pub static mut ebase: c_ulong;
    pub static mut hwrena: c_uint;
    pub fn per_cpu_trap_init(flag: bool);
    pub fn cpu_cache_init();
    pub fn tlb_init();
}

// CONFIG_RELOCATABLE controls whether the relocation declarations are available.
#[cfg(feature = "CONFIG_RELOCATABLE")]
unsafe extern "C" {
    pub fn relocate_kernel() -> *mut c_void;
    pub fn plat_post_relocation(value: c_long) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
