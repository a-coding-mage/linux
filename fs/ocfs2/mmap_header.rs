/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard OCFS2_MMAP_H has no executable Rust equivalent.

#[repr(C)]
pub struct vm_area_desc {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ocfs2_mmap_prepare(desc: *mut vm_area_desc) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
