/*
 * super.h
 */

use core::ffi::c_int;

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct befs_super_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn befs_load_sb(sb: *mut super_block, disk_sb: *mut befs_super_block) -> c_int;
    pub fn befs_check_sb(sb: *mut super_block) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
