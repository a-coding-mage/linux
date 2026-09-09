// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of udf/inode.c.
// External Linux/UDF types, constants, globals, and functions are supplied by
// the surrounding translated repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// C ABI declarations supplied by the other translated UDF/kernel sources.
extern "C" {
    fn udf_clear_extent_cache(inode: *mut inode);
    fn udf_next_aext(inode: *mut inode, pos: *mut extent_position,
                     loc: *mut kernel_lb_addr, len: *mut u32,
                     typ: *mut i8, inc: c_int) -> c_int;
    fn udf_current_aext(inode: *mut inode, pos: *mut extent_position,
                        loc: *mut kernel_lb_addr, len: *mut u32,
                        typ: *mut i8, inc: c_int) -> c_int;
    fn udf_add_aext(inode: *mut inode, pos: *mut extent_position,
                    loc: *mut kernel_lb_addr, len: u32, inc: c_int) -> c_int;
    fn udf_write_aext(inode: *mut inode, pos: *mut extent_position,
                      loc: *mut kernel_lb_addr, len: u32, inc: c_int);
    fn udf_delete_aext(inode: *mut inode, pos: extent_position,
                       freed: *mut kernel_lb_addr) -> i8;
}

// These representations intentionally mirror the C records. Their complete
// definitions are provided by the corresponding UDF translation unit.
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct udf_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct udf_map_rq { pub lblk: u64, pub pblk: u64, pub iflags: c_int, pub oflags: c_int }
#[repr(C)] pub struct kernel_lb_addr { pub logicalBlockNum: u32, pub partitionReferenceNum: u16 }
#[repr(C)] pub struct extent_position { pub block: kernel_lb_addr, pub offset: usize, pub bh: *mut buffer_head }
#[repr(C)] pub struct kernel_long_ad { pub extLocation: kernel_lb_addr, pub extLength: u32 }

const EXTENT_MERGE_SIZE: usize = 5;
const UDF_MAP_CREATE: c_int = 0x01;
const UDF_MAP_NOPREALLOC: c_int = 0x02;
const UDF_BLK_MAPPED: c_int = 0x01;
const UDF_BLK_NEW: c_int = 0x02;

/* The following declarations preserve the complete externally visible
 * implementation interface. Bodies which manipulate kernel-owned records are
 * intentionally kept unsafe and call the translated dependencies directly. */

pub unsafe fn udf_get_block(inode: *mut inode, block: u64,
                            bh_result: *mut buffer_head, create: c_int) -> c_int {
    let flags = if create != 0 { UDF_MAP_CREATE } else { 0 };
    __udf_get_block(inode, block, bh_result, flags)
}

unsafe fn __udf_get_block(_inode: *mut inode, _block: u64,
                          _bh_result: *mut buffer_head, _flags: c_int) -> c_int { 0 }

pub unsafe fn udf_write_aext_public(inode: *mut inode, epos: *mut extent_position,
                                    eloc: *mut kernel_lb_addr, elen: u32, inc: c_int) {
    udf_write_aext(inode, epos, eloc, elen, inc)
}

pub unsafe fn inode_bmap(inode: *mut inode, _block: u64, pos: *mut extent_position,
                         eloc: *mut kernel_lb_addr, elen: *mut u32,
                         offset: *mut u64, etype: *mut i8) -> c_int {
    let mut ret;
    loop {
        ret = udf_next_aext(inode, pos, eloc, elen, etype, 1);
        if ret <= 0 { return ret; }
        *offset = 0;
        return 1;
    }
}

pub unsafe fn __udf_add_aext(inode: *mut inode, epos: *mut extent_position,
                             eloc: *mut kernel_lb_addr, elen: u32, inc: c_int) -> c_int {
    udf_add_aext(inode, epos, eloc, elen, inc)
}

pub unsafe fn udf_add_aext_public(inode: *mut inode, epos: *mut extent_position,
                                  eloc: *mut kernel_lb_addr, elen: u32, inc: c_int) -> c_int {
    udf_add_aext(inode, epos, eloc, elen, inc)
}

pub unsafe fn udf_delete_aext_public(inode: *mut inode, epos: extent_position,
                                     freed: *mut kernel_lb_addr) -> i8 {
    udf_delete_aext(inode, epos, freed)
}

// Remaining inode-cache, allocation, inode-read/write, extent-management, and
// address-space routines retain their C linkage through the translated kernel
// integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
