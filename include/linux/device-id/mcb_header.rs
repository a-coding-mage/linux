/* SPDX-License-Identifier: GPL-2.0 */

// The C definition of `kernel_ulong_t` is available when building in-kernel.
pub type kernel_ulong_t = usize;

#[repr(C)]
pub struct mcb_device_id {
    pub device: u16,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
