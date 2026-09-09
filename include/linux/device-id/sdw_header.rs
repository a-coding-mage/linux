/* SPDX-License-Identifier: GPL-2.0 */

// In the kernel build, this corresponds to `typedef unsigned long kernel_ulong_t`.
pub type kernel_ulong_t = usize;

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: u16,
    pub part_id: u16,
    pub sdw_version: u8,
    pub class_id: u8,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
