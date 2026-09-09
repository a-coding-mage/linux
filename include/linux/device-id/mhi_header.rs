/* SPDX-License-Identifier: GPL-2.0 */

// The C header defines this alias only when __KERNEL__ is enabled.
// Preserve the kernel_ulong_t spelling for the translated declaration.
pub type kernel_ulong_t = std::os::raw::c_ulong;

pub const MHI_DEVICE_MODALIAS_FMT: &str = "mhi:%s";
pub const MHI_NAME_SIZE: usize = 32;

pub const MHI_EP_DEVICE_MODALIAS_FMT: &str = "mhi_ep:%s";

/**
 * struct mhi_device_id - MHI device identification
 * @chan: MHI channel name
 * @driver_data: driver data;
 */
#[repr(C)]
pub struct mhi_device_id {
    pub chan: [std::os::raw::c_char; MHI_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
