/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from <linux/soc/qcom/mdt_loader.h>. */

pub const QCOM_MDT_TYPE_MASK: u32 = 7u32 << 24;
pub const QCOM_MDT_TYPE_HASH: u32 = 2u32 << 24;
pub const QCOM_MDT_RELOCATABLE: u32 = 1u32 << 27;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qcom_pas_context {
    _private: [u8; 0],
}

/* IS_ENABLED(CONFIG_QCOM_MDT_LOADER) selects the declaration branch at build time. */
#[cfg(feature = "CONFIG_QCOM_MDT_LOADER")]
extern "C" {
    pub fn qcom_mdt_get_size(fw: *const firmware) -> isize;

    pub fn qcom_mdt_load(
        dev: *mut device,
        fw: *const firmware,
        fw_name: *const core::ffi::c_char,
        pas_id: i32,
        mem_region: *mut core::ffi::c_void,
        mem_phys: u64,
        mem_size: usize,
        reloc_base: *mut u64,
    ) -> i32;

    pub fn qcom_mdt_pas_load(
        ctx: *mut qcom_pas_context,
        fw: *const firmware,
        firmware: *const core::ffi::c_char,
        reloc_base: *mut u64,
    ) -> i32;

    pub fn qcom_mdt_load_no_init(
        dev: *mut device,
        fw: *const firmware,
        fw_name: *const core::ffi::c_char,
        mem_region: *mut core::ffi::c_void,
        mem_phys: u64,
        mem_size: usize,
        reloc_base: *mut u64,
    ) -> i32;

    pub fn qcom_mdt_read_metadata(
        fw: *const firmware,
        data_len: *mut usize,
        fw_name: *const core::ffi::c_char,
        dev: *mut device,
    ) -> *mut core::ffi::c_void;
}

/* !IS_ENABLED(CONFIG_QCOM_MDT_LOADER): the kernel's ENODEV error value. */
#[cfg(not(feature = "CONFIG_QCOM_MDT_LOADER"))]
pub unsafe fn qcom_mdt_get_size(_fw: *const firmware) -> isize {
    -19
}

#[cfg(not(feature = "CONFIG_QCOM_MDT_LOADER"))]
pub unsafe fn qcom_mdt_load(
    _dev: *mut device,
    _fw: *const firmware,
    _fw_name: *const core::ffi::c_char,
    _pas_id: i32,
    _mem_region: *mut core::ffi::c_void,
    _mem_phys: u64,
    _mem_size: usize,
    _reloc_base: *mut u64,
) -> i32 {
    -19
}

#[cfg(not(feature = "CONFIG_QCOM_MDT_LOADER"))]
pub unsafe fn qcom_mdt_pas_load(
    _ctx: *mut qcom_pas_context,
    _fw: *const firmware,
    _firmware: *const core::ffi::c_char,
    _reloc_base: *mut u64,
) -> i32 {
    -19
}

#[cfg(not(feature = "CONFIG_QCOM_MDT_LOADER"))]
pub unsafe fn qcom_mdt_load_no_init(
    _dev: *mut device,
    _fw: *const firmware,
    _fw_name: *const core::ffi::c_char,
    _mem_region: *mut core::ffi::c_void,
    _mem_phys: u64,
    _mem_size: usize,
    _reloc_base: *mut u64,
) -> i32 {
    -19
}

#[cfg(not(feature = "CONFIG_QCOM_MDT_LOADER"))]
pub unsafe fn qcom_mdt_read_metadata(
    _fw: *const firmware,
    _data_len: *mut usize,
    _fw_name: *const core::ffi::c_char,
    _dev: *mut device,
) -> *mut core::ffi::c_void {
    (-19isize) as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
