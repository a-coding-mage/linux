/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/types.h>.

use core::ffi::c_int;

#[repr(C)]
pub struct ulpi {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct ulpi_ops - ULPI register access
 * @read: read operation for ULPI register access
 * @write: write operation for ULPI register access
 */
#[repr(C)]
pub struct ulpi_ops {
    pub read: Option<unsafe extern "C" fn(dev: *mut device, addr: u8) -> c_int>,
    pub write:
        Option<unsafe extern "C" fn(dev: *mut device, addr: u8, val: u8) -> c_int>,
}

unsafe extern "C" {
    pub fn ulpi_register_interface(
        dev: *mut device,
        ops: *const ulpi_ops,
    ) -> *mut ulpi;
    pub fn ulpi_unregister_interface(ulpi: *mut ulpi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
