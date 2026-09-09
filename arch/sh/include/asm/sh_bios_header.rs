/* SPDX-License-Identifier: GPL-2.0 */

// C source condition: CONFIG_SH_STANDARD_BIOS
#[cfg(feature = "CONFIG_SH_STANDARD_BIOS")]
mod standard_bios {
    /*
     * Copyright (C) 2000 Greg Banks, Mitch Davis
     * C API to interface to the standard LinuxSH BIOS
     * usually from within the early stages of kernel boot.
     */

    unsafe extern "C" {
        pub fn sh_bios_console_write(buf: *const core::ffi::c_char, len: core::ffi::c_uint);
        pub fn sh_bios_gdb_detach();

        pub fn sh_bios_get_node_addr(node_addr: *mut core::ffi::c_uchar);
        pub fn sh_bios_shutdown(how: core::ffi::c_uint);

        pub fn sh_bios_vbr_init();
        pub fn sh_bios_vbr_reload();
    }
}

// When CONFIG_SH_STANDARD_BIOS is not enabled, these C inline functions are
// empty no-op definitions.
#[cfg(not(feature = "CONFIG_SH_STANDARD_BIOS"))]
#[inline]
pub fn sh_bios_vbr_init() {}

#[cfg(not(feature = "CONFIG_SH_STANDARD_BIOS"))]
#[inline]
pub fn sh_bios_vbr_reload() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
