/* SPDX-License-Identifier: GPL-2.0 */

// C header guard __WIRELESS_SYSFS_H omitted; Rust modules provide inclusion control.

extern "C" {
    pub fn wiphy_sysfs_init() -> ::core::ffi::c_int;
    pub fn wiphy_sysfs_exit();

    pub static mut ieee80211_class: crate::class;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
