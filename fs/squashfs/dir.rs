// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * dir.c
 */

/* This file implements code to read directories from disk. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn squashfs_read_metadata(sb: *mut super_block, buffer: *mut c_void,
        block: *mut u64, offset: *mut c_int, length: usize) -> c_int;
    fn dir_emit(ctx: *mut dir_context, name: *const c_char, namelen: c_uint,
        ino: c_int, typ: u8) -> bool;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn generic_read_dir(file: *mut file, ctx: *mut dir_context) -> c_int;
    fn generic_file_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn generic_setlease(file: *mut file, arg: c_int, fl: *mut c_void) -> c_int;
    fn i_size_read(inode: *mut inode) -> i64;
    fn squashfs_i(inode: *mut inode) -> *mut squashfs_inode_info;
    fn file_inode(file: *mut file) -> *mut inode;
    fn le16_to_cpu(v: u16) -> u16;
    fn le32_to_cpu(v: u32) -> u32;
    fn trace_(fmt: *const c_char, ...);
    fn error_(fmt: *const c_char, ...);
}

#[repr(C)] pub struct super_block { pub s_fs_info: *mut c_void }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: c_ulong }
#[repr(C)] pub struct dir_context { pub pos: i64 }
#[repr(C)] pub struct squashfs_sb_info { pub directory_table: u64 }
#[repr(C)] pub struct squashfs_inode_info {
    pub start: u64, pub offset: c_int, pub parent: c_int,
    pub dir_idx_start: u64, pub dir_idx_offset: c_int, pub dir_idx_cnt: c_int,
}
#[repr(C)] pub struct squashfs_dir_index { pub index: u32, pub start_block: u32, pub size: u32 }
#[repr(C)] pub struct squashfs_dir_header { pub count: u32, pub start_block: u32, pub inode_number: u32 }
#[repr(C)] pub struct squashfs_dir_entry { pub offset: u16, pub inode_number: u16, pub type_: u16, pub size: u16, pub name: [c_char; 0] }
#[repr(C)] pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut dir_context) -> c_int>,
    pub iterate_shared: Option<unsafe extern "C" fn(*mut file, *mut dir_context) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
    pub setlease: Option<unsafe extern "C" fn(*mut file, c_int, *mut c_void) -> c_int>,
}

const SQUASHFS_NAME_LEN: usize = 256;
const SQUASHFS_METADATA_SIZE: c_int = 8192;
const SQUASHFS_DIR_COUNT: u32 = 256;
const SQUASHFS_MAX_DIR_TYPE: u16 = 7;
const DT_UNKNOWN: u8 = 0;
static SQUASHFS_FILETYPE_TABLE: [u8; 8] = [DT_UNKNOWN, 4, 8, 10, 6, 2, 1, 12];

unsafe fn get_dir_index_using_offset(sb: *mut super_block, next_block: *mut u64,
    next_offset: *mut c_int, mut index_start: u64, mut index_offset: c_int,
    i_count: c_int, mut f_pos: i64) -> c_int {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut length: c_int = 0;
    if f_pos <= 3 { return f_pos as c_int; }
    f_pos -= 3;
    for _ in 0..i_count {
        let mut dir_index = core::mem::MaybeUninit::<squashfs_dir_index>::uninit();
        let err = squashfs_read_metadata(sb, dir_index.as_mut_ptr() as *mut c_void,
            &mut index_start, &mut index_offset, core::mem::size_of::<squashfs_dir_index>());
        if err < 0 { break; }
        let dir_index = dir_index.assume_init();
        let index = le32_to_cpu(dir_index.index) as i64;
        if index > f_pos { break; }
        let size = le32_to_cpu(dir_index.size) as usize + 1;
        if size > SQUASHFS_NAME_LEN { break; }
        if squashfs_read_metadata(sb, core::ptr::null_mut(), &mut index_start,
            &mut index_offset, size) < 0 { break; }
        length = index as c_int;
        *next_block = le32_to_cpu(dir_index.start_block) as u64 + (*msblk).directory_table;
    }
    *next_offset = (length + *next_offset) % SQUASHFS_METADATA_SIZE;
    length + 3
}

unsafe extern "C" fn squashfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file);
    let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    let si = squashfs_i(inode);
    let mut block = (*si).start + (*msblk).directory_table;
    let mut offset = (*si).offset;
    let dire = kmalloc(core::mem::size_of::<squashfs_dir_entry>() + SQUASHFS_NAME_LEN + 1, 0)
        as *mut squashfs_dir_entry;
    if dire.is_null() { error_(b"Failed to allocate squashfs_dir_entry\0".as_ptr() as *const c_char); return 0; }
    let mut size: usize;
    while (*ctx).pos < 3 {
        let (name, ino, n) = if (*ctx).pos == 0 { (b".\0".as_ptr(), (*inode).i_ino as c_int, 1) }
            else { (b"..\0".as_ptr(), (*si).parent, 2) };
        if !dir_emit(ctx, name as *const c_char, n, ino, SQUASHFS_FILETYPE_TABLE[1]) { kfree(dire as *mut c_void); return 0; }
        (*ctx).pos += n as i64;
    }
    let mut length = get_dir_index_using_offset((*inode).i_sb, &mut block, &mut offset,
        (*si).dir_idx_start, (*si).dir_idx_offset, (*si).dir_idx_cnt, (*ctx).pos);
    while length < i_size_read(inode) {
        let mut dirh = core::mem::MaybeUninit::<squashfs_dir_header>::uninit();
        if squashfs_read_metadata((*inode).i_sb, dirh.as_mut_ptr() as *mut c_void, &mut block, &mut offset, core::mem::size_of::<squashfs_dir_header>()) < 0 { break; }
        let dirh = dirh.assume_init(); length += core::mem::size_of::<squashfs_dir_header>() as c_int;
        let mut dir_count = le32_to_cpu(dirh.count) + 1;
        if dir_count > SQUASHFS_DIR_COUNT { break; }
        while dir_count != 0 { dir_count -= 1;
            if squashfs_read_metadata((*inode).i_sb, dire as *mut c_void, &mut block, &mut offset, core::mem::size_of::<squashfs_dir_entry>()) < 0 { break; }
            size = le16_to_cpu((*dire).size) as usize + 1;
            if size > SQUASHFS_NAME_LEN || squashfs_read_metadata((*inode).i_sb, (*dire).name.as_mut_ptr() as *mut c_void, &mut block, &mut offset, size) < 0 { break; }
            length += (core::mem::size_of::<squashfs_dir_entry>() + size) as c_int;
            if (*ctx).pos >= length { continue; }
            (*dire).name[size] = 0;
            let ino = le32_to_cpu(dirh.inode_number) as c_int + (le16_to_cpu((*dire).inode_number) as i16 as c_int);
            let typ = le16_to_cpu((*dire).type_);
            if typ > SQUASHFS_MAX_DIR_TYPE || !dir_emit(ctx, (*dire).name.as_ptr(), size as c_uint, ino, SQUASHFS_FILETYPE_TABLE[typ as usize]) { kfree(dire as *mut c_void); return 0; }
            (*ctx).pos = length as i64;
        }
    }
    kfree(dire as *mut c_void); 0
}

#[no_mangle]
pub static squashfs_dir_ops: file_operations = file_operations {
    read: Some(generic_read_dir), iterate_shared: Some(squashfs_readdir),
    llseek: Some(generic_file_llseek), setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
