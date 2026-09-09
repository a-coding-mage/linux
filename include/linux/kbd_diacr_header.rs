/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/kd.h>; `kbdiacruc` is supplied by that
// dependency and is intentionally not defined here.

unsafe extern "C" {
    pub static mut accent_table: [crate::kbdiacruc; 0];
    pub static mut accent_table_size: core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
