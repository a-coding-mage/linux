/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/mtd/mtd.h

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

extern "C" {
    pub fn mtdram_init_device(
        mtd: *mut mtd_info,
        mapped_address: *mut core::ffi::c_void,
        size: core::ffi::c_ulong,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
