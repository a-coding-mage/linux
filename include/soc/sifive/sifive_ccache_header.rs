/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SiFive Composable Cache Controller header file
 *
 */

// Declaration supplied by an external dependency.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn register_sifive_ccache_error_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_sifive_ccache_error_notifier(nb: *mut notifier_block) -> i32;
}

pub const SIFIVE_CCACHE_ERR_TYPE_CE: i32 = 0;
pub const SIFIVE_CCACHE_ERR_TYPE_UE: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
