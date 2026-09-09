/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: LINUX_DEVICE_ID_SPMI_H

// Preserved from the C build condition: this alias is available only when
// compiling for the kernel (__KERNEL__).
#[cfg(__KERNEL__)]
pub type kernel_ulong_t = ::core::ffi::c_ulong;

pub const SPMI_NAME_SIZE: usize = 32;
pub const SPMI_MODULE_PREFIX: &str = "spmi:";

#[repr(C)]
pub struct spmi_device_id {
    pub name: [::core::ffi::c_char; SPMI_NAME_SIZE],
    pub driver_data: kernel_ulong_t, // Data private to the driver
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
