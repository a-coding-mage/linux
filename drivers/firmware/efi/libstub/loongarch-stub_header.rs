/* SPDX-License-Identifier: GPL-2.0-only */

extern "C" {
    pub fn kernel_entry_address(
        kernel_addr: core::ffi::c_ulong,
        image: *mut efi_loaded_image_t,
    ) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
