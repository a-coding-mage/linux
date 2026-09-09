/* SPDX-License-Identifier: GPL-2.0 */

// C conditional: these declarations are omitted when CONFIG_KERNEL_UNCOMPRESSED
// is defined.
#[cfg(not(feature = "CONFIG_KERNEL_UNCOMPRESSED"))]
extern "C" {
    pub fn mem_safe_offset() -> core::ffi::c_ulong;
    pub fn deploy_kernel(output: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
