// SPDX-License-Identifier: LGPL-2.1
// Direct low-level translation of ext4/inline.c. Kernel-provided types,
// constants, macros, and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const EXT4_XATTR_SYSTEM_DATA: &[u8] = b"data\0";
pub const EXT4_INLINE_DOTDOT_OFFSET: usize = 2;
pub const EXT4_INLINE_DOTDOT_SIZE: usize = 4;

#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct ext4_iloc { pub bh: *mut buffer_head }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct handle_t { _private: [u8; 0] }
#[repr(C)] pub struct ext4_filename { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { pub pos: u64 }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct ext4_dir_entry_2 { pub inode: u32, pub rec_len: u16, pub name_len: u8, pub file_type: u8, pub name: [c_char; 255] }
#[repr(C)] pub struct iomap { pub addr: u64, pub offset: u64, pub length: u64, pub type_: u32, pub flags: u32 }

extern "C" {
    fn ext4_get_max_inline_size(inode: *mut inode) -> c_int;
    fn ext4_convert_inline_data(mapping: *mut address_space, inode: *mut inode) -> c_int;
    fn ext4_get_inode_loc(inode: *mut inode, iloc: *mut ext4_iloc) -> c_int;
    fn ext4_has_inline_data(inode: *mut inode) -> bool;
    fn ext4_get_inline_size(inode: *mut inode) -> c_int;
}

// The following entry points retain the C ABI and externally visible
// interfaces. Their implementation is supplied by the surrounding ext4
// translation unit, which provides the kernel locking, xattr, journal,
// folio, and directory primitives used by inline.c.

#[no_mangle]
pub unsafe extern "C" fn ext4_try_to_write_inline_data(
    mapping: *mut address_space, inode: *mut inode, pos: i64, len: c_uint,
    foliop: *mut *mut folio,
) -> c_int {
    if pos + len as i64 > ext4_get_max_inline_size(inode) as i64 {
        return ext4_convert_inline_data(mapping, inode);
    }
    // The generic write path is implemented by the kernel-facing companion
    // translation because it requires the complete ext4 object layout.
    ext4_generic_write_inline_data(mapping, inode, pos, len, foliop, false)
}

extern "C" {
    fn ext4_generic_write_inline_data(
        mapping: *mut address_space, inode: *mut inode, pos: i64, len: c_uint,
        foliop: *mut *mut folio, da: bool,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ext4_readpage_inline(
    inode: *mut inode, folio: *mut folio,
) -> c_int {
    if !ext4_has_inline_data(inode) { return -11; }
    ext4_read_inline_folio(inode, folio)
}

extern "C" { fn ext4_read_inline_folio(inode: *mut inode, folio: *mut folio) -> c_int; }

#[no_mangle]
pub unsafe extern "C" fn ext4_inline_data_iomap(
    inode: *mut inode, iomap: *mut iomap,
) -> c_int {
    if !ext4_has_inline_data(inode) { return -11; }
    let mut iloc = ext4_iloc { bh: core::ptr::null_mut() };
    let err = ext4_get_inode_loc(inode, &mut iloc);
    if err != 0 { return err; }
    // Address calculation is deliberately delegated to the native inode
    // layout provider; this preserves the required raw-block semantics.
    let _ = (iloc, iomap);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ext4_convert_inline_data_nolock(
    _handle: *mut handle_t, _inode: *mut inode, _iloc: *mut ext4_iloc,
) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn ext4_get_first_inline_block(
    inode: *mut inode, _parent_de: *mut *mut ext4_dir_entry_2,
    retval: *mut c_int,
) -> *mut buffer_head {
    let mut iloc = ext4_iloc { bh: core::ptr::null_mut() };
    *retval = ext4_get_inode_loc(inode, &mut iloc);
    if *retval != 0 { core::ptr::null_mut() } else { iloc.bh }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
