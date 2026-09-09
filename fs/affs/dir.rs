// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/affs/dir.c
 *
 *  (c) 1996  Hans-Joachim Widmaier - Rewritten
 *
 *  (C) 1993  Ray Burr - Modified for Amiga FFS filesystem.
 *
 *  (C) 1992  Eric Youngdale Modified for ISO 9660 filesystem.
 *
 *  (C) 1991  Linus Torvalds - minix filesystem
 *
 *  affs directory handling functions
 *
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct affs_dir_data {
    pub ino: libc::c_ulong,
    pub cookie: u64,
}

extern "C" {
    fn generic_llseek_cookie(file: *mut file, offset: loff_t, whence: libc::c_int,
                             cookie: *mut u64) -> loff_t;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut libc::c_void);
    fn file_inode(file: *mut file) -> *mut inode;
    fn generic_read_dir(file: *mut file, ctx: *mut dir_context) -> libc::c_int;
    fn affs_file_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: bool) -> libc::c_int;
    fn generic_setlease(file: *mut file, arg: libc::c_int, fl: *mut file_lock, priv_: *mut libc::c_void) -> libc::c_int;
    fn affs_create(dir: *mut inode, dentry: *mut dentry, mode: umode_t, excl: bool) -> libc::c_int;
    fn affs_lookup(dir: *mut inode, dentry: *mut dentry, flags: libc::c_uint) -> *mut dentry;
    fn affs_link(old: *mut dentry, dir: *mut inode, new: *mut dentry) -> libc::c_int;
    fn affs_unlink(dir: *mut inode, dentry: *mut dentry) -> libc::c_int;
    fn affs_symlink(dir: *mut inode, dentry: *mut dentry, symname: *const libc::c_char) -> libc::c_int;
    fn affs_mkdir(dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> libc::c_int;
    fn affs_rmdir(dir: *mut inode, dentry: *mut dentry) -> libc::c_int;
    fn affs_rename2(old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode,
                    new_dentry: *mut dentry, flags: libc::c_uint) -> libc::c_int;
    fn affs_setattr(id: *mut dentry, ia: *mut iattr) -> libc::c_int;
    fn affs_lock_dir(inode: *mut inode);
    fn affs_unlock_dir(inode: *mut inode);
    fn affs_bread(sb: *mut super_block, block: u32) -> *mut buffer_head;
    fn affs_brelse(bh: *mut buffer_head);
    fn affs_warning(sb: *mut super_block, where_: *const libc::c_char, msg: *const libc::c_char);
    fn affs_error(sb: *mut super_block, where_: *const libc::c_char, msg: *const libc::c_char, arg: u32);
    fn inode_eq_iversion(inode: *mut inode, version: u64) -> bool;
    fn inode_query_iversion(inode: *mut inode) -> u64;
    fn dir_emit_dots(file: *mut file, ctx: *mut dir_context) -> bool;
    fn dir_emit(ctx: *mut dir_context, name: *const u8, namelen: usize, ino: u32, dtype: u8) -> bool;
    fn affs_hash_size(sb: *mut super_block) -> u32;
    fn affs_dir_hash_entry(bh: *mut buffer_head, pos: u32) -> u32;
    fn affs_chain_entry(sb: *mut super_block, bh: *mut buffer_head) -> u32;
    fn affs_name_entry(sb: *mut super_block, bh: *mut buffer_head) -> *const u8;
    fn affs_name_len(sb: *mut super_block, bh: *mut buffer_head) -> u8;
}

#[repr(C)] pub struct file { pub private_data: *mut libc::c_void }
#[repr(C)] pub struct inode { pub i_ino: u64, pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block;
#[repr(C)] pub struct buffer_head;
#[repr(C)] pub struct dir_context { pub pos: loff_t }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct file_lock;
#[repr(C)] pub struct iattr;
pub type loff_t = i64;
pub type umode_t = u16;

extern "C" {
    static affs_dir_operations: file_operations;
    static affs_dir_inode_operations: inode_operations;
}
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct inode_operations;

pub unsafe fn affs_dir_llseek(file: *mut file, offset: loff_t, whence: libc::c_int) -> loff_t {
    let data = (*file).private_data as *mut affs_dir_data;
    generic_llseek_cookie(file, offset, whence, &mut (*data).cookie)
}

pub unsafe fn affs_dir_open(_inode: *mut inode, file: *mut file) -> libc::c_int {
    let data = kzalloc_obj::<affs_dir_data>();
    if data.is_null() { return -12; }
    (*file).private_data = data as *mut libc::c_void;
    0
}

pub unsafe fn affs_dir_release(_inode: *mut inode, file: *mut file) -> libc::c_int {
    kfree((*file).private_data);
    0
}

pub unsafe fn affs_readdir(file: *mut file, ctx: *mut dir_context) -> libc::c_int {
    let inode = file_inode(file);
    let data = (*file).private_data as *mut affs_dir_data;
    let sb = (*inode).i_sb;
    let mut dir_bh: *mut buffer_head = core::ptr::null_mut();
    let mut fh_bh: *mut buffer_head = core::ptr::null_mut();
    let mut error = 0;
    if (*ctx).pos < 2 {
        (*data).ino = 0;
        if !dir_emit_dots(file, ctx) { return 0; }
    }
    affs_lock_dir(inode);
    let mut chain_pos = ((*ctx).pos - 2) & 0xffff;
    let mut hash_pos = ((*ctx).pos - 2) >> 16;
    if chain_pos == 0xffff { chain_pos = 0; hash_pos += 1; (*ctx).pos = ((hash_pos << 16) | chain_pos) + 2; }
    dir_bh = affs_bread(sb, (*inode).i_ino as u32);
    if dir_bh.is_null() { affs_unlock_dir(inode); return 0; }
    let mut ino = (*data).ino as u32;
    if ino == 0 || !inode_eq_iversion(inode, (*data).cookie) {
        if hash_pos >= affs_hash_size(sb) { affs_brelse(dir_bh); affs_unlock_dir(inode); return 0; }
        ino = affs_dir_hash_entry(dir_bh, hash_pos);
        let mut i = 0;
        while ino != 0 && i < chain_pos {
            fh_bh = affs_bread(sb, ino);
            if fh_bh.is_null() { error = -5; affs_brelse(dir_bh); affs_unlock_dir(inode); return error; }
            ino = affs_chain_entry(sb, fh_bh);
            affs_brelse(fh_bh); fh_bh = core::ptr::null_mut(); i += 1;
        }
        if ino == 0 {
            hash_pos += 1;
            while hash_pos < affs_hash_size(sb) {
                ino = affs_dir_hash_entry(dir_bh, hash_pos);
                if ino != 0 { (*ctx).pos = (hash_pos << 16) + 2; break; }
                hash_pos += 1;
            }
        }
    }
    while ino != 0 {
        fh_bh = affs_bread(sb, ino);
        if fh_bh.is_null() { break; }
        let namelen = core::cmp::min(affs_name_len(sb, fh_bh), 30) as usize;
        let name = affs_name_entry(sb, fh_bh);
        if !dir_emit(ctx, name, namelen, ino, 0) { break; }
        (*ctx).pos += 1;
        ino = affs_chain_entry(sb, fh_bh);
        affs_brelse(fh_bh); fh_bh = core::ptr::null_mut();
    }
    (*data).cookie = inode_query_iversion(inode);
    (*data).ino = ino as libc::c_ulong;
    affs_brelse(fh_bh); affs_brelse(dir_bh); affs_unlock_dir(inode);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
