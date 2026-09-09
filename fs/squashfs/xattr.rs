// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2010
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * xattr.c
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel types, constants, macros, and functions are supplied externally.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    _private: [u8; 0],
}
#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut c_void,
    _private: [u8; 0],
}
#[repr(C)]
pub struct squashfs_sb_info {
    pub xattr_table: u64,
    pub xattr_id_table: *mut c_void,
    _private: [u8; 0],
}
#[repr(C)]
pub struct squashfs_inode_info {
    pub xattr: u64,
    pub xattr_count: c_int,
    _private: [u8; 0],
}
#[repr(C)]
pub struct squashfs_xattr_entry {
    pub size: u16,
    pub r#type: u16,
}
#[repr(C)]
pub struct squashfs_xattr_val {
    pub vsize: u32,
}
#[repr(C)]
pub struct xattr_handler {
    pub prefix: *const c_char,
    pub name: *const c_char,
    pub flags: c_int,
    pub list: Option<unsafe extern "C" fn(*mut dentry) -> bool>,
    pub get: Option<unsafe extern "C" fn(*const xattr_handler, *mut dentry, *mut inode, *const c_char, *mut c_void, usize) -> c_int>,
}

extern "C" {
    fn d_inode(d: *mut dentry) -> *mut inode;
    fn squashfs_i(inode: *mut inode) -> *mut squashfs_inode_info;
    fn squashfs_read_metadata(sb: *mut super_block, buffer: *mut c_void, start: *mut u64, offset: *mut c_int, size: usize) -> c_int;
    fn le16_to_cpu(value: u16) -> c_int;
    fn le32_to_cpu(value: u32) -> u32;
    fn le64_to_cpu(value: u64) -> u64;
    fn strlen(string: *const c_char) -> usize;
    fn memcpy(destination: *mut c_void, source: *const c_void, size: usize) -> *mut c_void;
    fn strncmp(left: *const c_char, right: *const c_char, size: usize) -> c_int;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_char;
    fn kfree(pointer: *mut c_char);
    fn capable(capability: c_int) -> bool;
}

extern "C" {
    static XATTR_USER_PREFIX: *const c_char;
    static XATTR_TRUSTED_PREFIX: *const c_char;
    static XATTR_SECURITY_PREFIX: *const c_char;
}

const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const ENODATA: c_int = 61;
const GFP_KERNEL: c_int = 0;
const CAP_SYS_ADMIN: c_int = 21;
const SQUASHFS_XATTR_PREFIX_MASK: c_int = 0xff;
const SQUASHFS_XATTR_VALUE_OOL: c_int = 0x100;
const SQUASHFS_XATTR_USER: c_int = 0;
const SQUASHFS_XATTR_TRUSTED: c_int = 1;
const SQUASHFS_XATTR_SECURITY: c_int = 2;

extern "C" {
    fn squashfs_xattr_blk(value: u64) -> u64;
    fn squashfs_xattr_offset(value: u64) -> c_int;
}

unsafe fn squashfs_xattr_handler(type_: c_int) -> *const xattr_handler {
    if type_ & !(SQUASHFS_XATTR_PREFIX_MASK | SQUASHFS_XATTR_VALUE_OOL) != 0 {
        return core::ptr::null();
    }
    match type_ & SQUASHFS_XATTR_PREFIX_MASK {
        SQUASHFS_XATTR_USER => &squashfs_xattr_user_handler,
        SQUASHFS_XATTR_TRUSTED => &squashfs_xattr_trusted_handler,
        SQUASHFS_XATTR_SECURITY => &squashfs_xattr_security_handler,
        _ => core::ptr::null(),
    }
}

pub unsafe extern "C" fn squashfs_listxattr(d: *mut dentry, mut buffer: *mut c_char, buffer_size: usize) -> isize {
    let inode = d_inode(d);
    let sb = (*inode).i_sb;
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut start = squashfs_xattr_blk((*squashfs_i(inode)).xattr) + (*msblk).xattr_table;
    let mut offset = squashfs_xattr_offset((*squashfs_i(inode)).xattr);
    let mut count = (*squashfs_i(inode)).xattr_count;
    let mut rest = buffer_size;
    let mut err: c_int;
    if (*msblk).xattr_id_table.is_null() { return -(EOPNOTSUPP as isize); }
    while count > 0 {
        count -= 1;
        let mut entry = squashfs_xattr_entry { size: 0, r#type: 0 };
        let mut val = squashfs_xattr_val { vsize: 0 };
        err = squashfs_read_metadata(sb, &mut entry as *mut _ as *mut c_void, &mut start, &mut offset, core::mem::size_of::<squashfs_xattr_entry>());
        if err < 0 { return err as isize; }
        let name_size = le16_to_cpu(entry.size) as usize;
        let handler = squashfs_xattr_handler(le16_to_cpu(entry.r#type));
        if !handler.is_null() && ((*handler).list.is_none() || ((*handler).list.unwrap())(d)) {
            let prefix = if !(*handler).prefix.is_null() { (*handler).prefix } else { (*handler).name };
            let prefix_size = strlen(prefix);
            if !buffer.is_null() {
                if prefix_size + name_size + 1 > rest { return -(ERANGE as isize); }
                memcpy(buffer as *mut c_void, prefix as *const c_void, prefix_size);
                buffer = buffer.add(prefix_size);
            }
            err = squashfs_read_metadata(sb, buffer as *mut c_void, &mut start, &mut offset, name_size);
            if err < 0 { return err as isize; }
            if !buffer.is_null() { *buffer.add(name_size) = 0; buffer = buffer.add(name_size + 1); }
            rest -= prefix_size + name_size + 1;
        } else {
            err = squashfs_read_metadata(sb, core::ptr::null_mut(), &mut start, &mut offset, name_size);
            if err < 0 { return err as isize; }
        }
        err = squashfs_read_metadata(sb, &mut val as *mut _ as *mut c_void, &mut start, &mut offset, core::mem::size_of::<squashfs_xattr_val>());
        if err < 0 { return err as isize; }
        err = squashfs_read_metadata(sb, core::ptr::null_mut(), &mut start, &mut offset, le32_to_cpu(val.vsize) as usize);
        if err < 0 { return err as isize; }
    }
    (buffer_size - rest) as isize
}

// The remaining xattr get and handler definitions preserve the kernel-facing interface.
pub unsafe extern "C" fn squashfs_xattr_get(inode: *mut inode, name_index: c_int, name: *const c_char, buffer: *mut c_void, buffer_size: usize) -> c_int {
    let sb = (*inode).i_sb;
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut start = squashfs_xattr_blk((*squashfs_i(inode)).xattr) + (*msblk).xattr_table;
    let mut offset = squashfs_xattr_offset((*squashfs_i(inode)).xattr);
    let mut count = (*squashfs_i(inode)).xattr_count;
    let name_len = strlen(name);
    let target = kmalloc(name_len, GFP_KERNEL);
    if target.is_null() { return -ENOMEM; }
    let mut result = -ENODATA;
    while count > 0 {
        count -= 1;
        let mut entry = squashfs_xattr_entry { size: 0, r#type: 0 };
        let mut val = squashfs_xattr_val { vsize: 0 };
        let mut err = squashfs_read_metadata(sb, &mut entry as *mut _ as *mut c_void, &mut start, &mut offset, core::mem::size_of::<squashfs_xattr_entry>());
        if err < 0 { result = err; break; }
        let size = le16_to_cpu(entry.size) as usize;
        let type_ = le16_to_cpu(entry.r#type);
        if (type_ & SQUASHFS_XATTR_PREFIX_MASK) == name_index && size == name_len {
            err = squashfs_read_metadata(sb, target as *mut c_void, &mut start, &mut offset, size);
            if err < 0 { result = err; break; }
        } else {
            err = squashfs_read_metadata(sb, core::ptr::null_mut(), &mut start, &mut offset, size);
            if err < 0 { result = err; break; }
        }
        if (type_ & SQUASHFS_XATTR_PREFIX_MASK) == name_index && size == name_len && strncmp(target, name, size) == 0 {
            err = squashfs_read_metadata(sb, &mut val as *mut _ as *mut c_void, &mut start, &mut offset, core::mem::size_of::<squashfs_xattr_val>());
            if err < 0 { result = err; break; }
            let vsize = le32_to_cpu(val.vsize) as usize;
            if !buffer.is_null() {
                if vsize > buffer_size { result = -ERANGE; break; }
                err = squashfs_read_metadata(sb, buffer, &mut start, &mut offset, vsize);
                if err < 0 { result = err; break; }
            }
            result = vsize as c_int;
            break;
        }
        err = squashfs_read_metadata(sb, &mut val as *mut _ as *mut c_void, &mut start, &mut offset, core::mem::size_of::<squashfs_xattr_val>());
        if err < 0 { result = err; break; }
        err = squashfs_read_metadata(sb, core::ptr::null_mut(), &mut start, &mut offset, le32_to_cpu(val.vsize) as usize);
        if err < 0 { result = err; break; }
    }
    kfree(target);
    result
}

static squashfs_xattr_user_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), name: core::ptr::null(), flags: SQUASHFS_XATTR_USER, list: None, get: None };
static squashfs_xattr_trusted_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), name: core::ptr::null(), flags: SQUASHFS_XATTR_TRUSTED, list: Some(squashfs_trusted_xattr_handler_list), get: None };
static squashfs_xattr_security_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), name: core::ptr::null(), flags: SQUASHFS_XATTR_SECURITY, list: None, get: None };

unsafe extern "C" fn squashfs_trusted_xattr_handler_list(d: *mut dentry) -> bool { capable(CAP_SYS_ADMIN) }

#[no_mangle]
pub static squashfs_xattr_handlers: [*const xattr_handler; 4] = [&squashfs_xattr_user_handler, &squashfs_xattr_trusted_handler, &squashfs_xattr_security_handler, core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
