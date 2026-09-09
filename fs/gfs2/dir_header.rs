/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct gfs2_diradd {
    pub nr_blocks: u32,
    pub dent: *mut gfs2_dirent,
    pub bh: *mut buffer_head,
    pub save_loc: i32,
}

extern "C" {
    pub fn gfs2_dir_search(
        dir: *mut inode,
        filename: *const qstr,
        fail_on_exist: bool,
    ) -> *mut inode;
    pub fn gfs2_dir_check(
        dir: *mut inode,
        filename: *const qstr,
        ip: *const gfs2_inode,
    ) -> i32;
    pub fn gfs2_dir_add(
        inode: *mut inode,
        filename: *const qstr,
        ip: *const gfs2_inode,
        da: *mut gfs2_diradd,
    ) -> i32;
    pub fn gfs2_dir_del(dip: *mut gfs2_inode, dentry: *const dentry) -> i32;
    pub fn gfs2_dir_read(
        inode: *mut inode,
        ctx: *mut dir_context,
        f_ra: *mut file_ra_state,
    ) -> i32;
    pub fn gfs2_dir_mvino(
        dip: *mut gfs2_inode,
        filename: *const qstr,
        nip: *const gfs2_inode,
        new_type: u32,
    ) -> i32;
    pub fn gfs2_dir_exhash_dealloc(dip: *mut gfs2_inode) -> i32;
    pub fn gfs2_diradd_alloc_required(
        dir: *mut inode,
        filename: *const qstr,
        da: *mut gfs2_diradd,
    ) -> i32;
    pub fn gfs2_dir_get_new_buffer(
        ip: *mut gfs2_inode,
        block: u64,
        bhp: *mut *mut buffer_head,
    ) -> i32;
    pub fn gfs2_dir_hash_inval(ip: *mut gfs2_inode);
}

#[inline]
pub unsafe fn gfs2_dir_no_add(da: *mut gfs2_diradd) {
    brelse((*da).bh);
    (*da).bh = core::ptr::null_mut();
}

#[inline]
pub unsafe fn gfs2_disk_hash(data: *const i8, len: i32) -> u32 {
    crc32_le(!0u32, data, len) ^ !0u32
}

#[inline]
pub unsafe fn gfs2_str2qstr(name: *mut qstr, fname: *const i8) {
    (*name).name = fname;
    (*name).len = strlen(fname);
    (*name).hash = gfs2_disk_hash((*name).name, (*name).len);
}

/* N.B. This probably ought to take inum & type as args as well */
#[inline]
pub unsafe fn gfs2_qstr2dirent(name: *const qstr, reclen: u16, dent: *mut gfs2_dirent) {
    (*dent).de_inum.no_addr = cpu_to_be64(0);
    (*dent).de_inum.no_formal_ino = cpu_to_be64(0);
    (*dent).de_hash = cpu_to_be32((*name).hash);
    (*dent).de_rec_len = cpu_to_be16(reclen);
    (*dent).de_name_len = cpu_to_be16((*name).len);
    (*dent).de_type = cpu_to_be16(0);
    memset((*dent).__pad.as_mut_ptr(), 0, core::mem::size_of_val(&(*dent).__pad));
    memcpy(
        dent.add(1) as *mut core::ffi::c_void,
        (*name).name as *const core::ffi::c_void,
        (*name).len as usize,
    );
}

extern "C" {
    pub static mut gfs2_qdot: qstr;
    pub static mut gfs2_qdotdot: qstr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
