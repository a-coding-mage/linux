/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub union u64_swap {
    pub val64: u64,
    pub val32: [u32; 2],
}

unsafe extern "C" {
    pub fn mem_bswap_64(src: *mut core::ffi::c_void, byte_size: core::ffi::c_int);
    pub fn mem_bswap_32(src: *mut core::ffi::c_void, byte_size: core::ffi::c_int);
}
