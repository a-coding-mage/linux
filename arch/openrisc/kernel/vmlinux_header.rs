/* SPDX-License-Identifier: GPL-2.0 */

// C conditional: CONFIG_BLK_DEV_INITRD
#[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
unsafe extern "C" {
    pub static mut __initrd_start: core::ffi::c_char;
    pub static mut __initrd_end: core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
