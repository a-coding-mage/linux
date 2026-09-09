/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: __u32 and __u8 are supplied by the Linux types layer.
// The kernel build condition is preserved as dependency intent; this
// translation uses Rust's fixed-width integer equivalents directly.

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: u32,
    pub rev_id: u32,
    pub api_version: u8,
    pub name: *const std::os::raw::c_char,
    pub driver_data: std::os::raw::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
