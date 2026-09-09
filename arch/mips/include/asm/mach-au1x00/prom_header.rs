/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the declarations from au1x00 prom.h.

unsafe extern "C" {
    pub static mut prom_argc: core::ffi::c_int;
    pub static mut prom_argv: *mut *mut core::ffi::c_char;
    pub static mut prom_envp: *mut *mut core::ffi::c_char;

    pub fn prom_init_cmdline();
    pub fn prom_getenv(envname: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn prom_get_ethernet_addr(ethernet_addr: *mut core::ffi::c_char) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
