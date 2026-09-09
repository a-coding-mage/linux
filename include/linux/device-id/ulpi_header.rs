/* SPDX-License-Identifier: GPL-2.0 */

// The C header provides these definitions under __KERNEL__.
pub type kernel_ulong_t = usize;

#[repr(C)]
pub struct ulpi_device_id {
    pub vendor: u16,
    pub product: u16,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
