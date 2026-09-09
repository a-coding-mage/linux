/* SPDX-License-Identifier: GPL-2.0 */

//
// This is the "SEAMLDR_INFO" data structure defined in the
// "SEAM Loader (SEAMLDR) Interface Specification".
//
// Must be aligned to a 256-byte boundary.
//
#[repr(C, packed, align(256))]
pub struct seamldr_info {
    pub version: u32,
    pub attributes: u32,
    pub vendor_id: u32,
    pub build_date: u32,
    pub build_num: u16,
    pub minor_version: u16,
    pub major_version: u16,
    pub update_version: u16,
    pub acm_x2apicid: u32,
    pub num_remaining_updates: u32,
    pub seam_info: [u8; 128],
    pub seam_ready: u8,
    pub seam_debug: u8,
    pub p_seam_ready: u8,
    pub reserved: [u8; 93],
}

const _: [(); 256] = [(); core::mem::size_of::<seamldr_info>()];

unsafe extern "C" {
    pub fn seamldr_get_info(seamldr_info: *mut seamldr_info) -> i32;
    pub fn seamldr_install_module(data: *const u8, data_len: u32) -> i32;
    pub fn seamldr_lock_module_update();
    pub fn seamldr_unlock_module_update();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
