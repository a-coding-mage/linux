/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct mipi_i3c_hci_platform_data - Platform-dependent data for mipi_i3c_hci
 * @base_regs: Register set base address (to support multi-bus instances)
 */
#[repr(C)]
pub struct mipi_i3c_hci_platform_data {
	pub base_regs: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
