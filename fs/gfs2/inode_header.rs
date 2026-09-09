/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Dependencies supplied by the kernel and other translated units are intentionally external.

extern "C" {
    pub fn gfs2_release_folio(folio: *mut folio, gfp_mask: gfp_t) -> bool;
    pub fn gfs2_internal_read(
        ip: *mut gfs2_inode,
        buf: *mut std::ffi::c_char,
        pos: *mut loff_t,
        size: usize,
    ) -> isize;
    pub fn gfs2_set_aops(inode: *mut inode);

    pub fn gfs2_setup_inode(inode: *mut inode);
    pub fn gfs2_inode_lookup(
        sb: *mut super_block,
        type_: std::ffi::c_uint,
        no_addr: u64,
        no_formal_ino: u64,
        blktype: std::ffi::c_uint,
    ) -> *mut inode;
    pub fn gfs2_lookup_by_inum(
        sdp: *mut gfs2_sbd,
        no_addr: u64,
        no_formal_ino: u64,
        blktype: std::ffi::c_uint,
    ) -> *mut inode;
    pub fn gfs2_dinode_dealloc(ip: *mut gfs2_inode) -> std::ffi::c_int;
    pub fn gfs2_lookupi(
        dir: *mut inode,
        name: *const qstr,
        is_root: std::ffi::c_int,
    ) -> *mut inode;
    pub fn gfs2_permission(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        mask: std::ffi::c_int,
    ) -> std::ffi::c_int;
    pub fn gfs2_lookup_meta(dip: *mut inode, name: *const std::ffi::c_char) -> *mut inode;
    pub fn gfs2_dinode_out(ip: *const gfs2_inode, buf: *mut std::ffi::c_void);
    pub fn gfs2_open_common(inode: *mut inode, file: *mut file) -> std::ffi::c_int;
    pub fn gfs2_seek_data(file: *mut file, offset: loff_t) -> loff_t;
    pub fn gfs2_seek_hole(file: *mut file, offset: loff_t) -> loff_t;
    pub fn gfs2_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> std::ffi::c_int;
    pub fn gfs2_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> std::ffi::c_int;
    pub fn gfs2_set_inode_flags(inode: *mut inode);
}

pub unsafe fn gfs2_is_stuffed(ip: *const gfs2_inode) -> std::ffi::c_int {
    ((*ip).i_height == 0) as std::ffi::c_int
}

pub unsafe fn gfs2_is_jdata(ip: *const gfs2_inode) -> std::ffi::c_int {
    ((*ip).i_diskflags & GFS2_DIF_JDATA) as std::ffi::c_int
}

pub unsafe fn gfs2_is_ordered(sdp: *const gfs2_sbd) -> bool {
    (*sdp).sd_args.ar_data == GFS2_DATA_ORDERED
}

pub unsafe fn gfs2_is_writeback(sdp: *const gfs2_sbd) -> bool {
    (*sdp).sd_args.ar_data == GFS2_DATA_WRITEBACK
}

pub unsafe fn gfs2_is_dir(ip: *const gfs2_inode) -> std::ffi::c_int {
    S_ISDIR((*ip).i_inode.i_mode)
}

pub unsafe fn gfs2_set_inode_blocks(inode: *mut inode, blocks: u64) {
    (*inode).i_blocks = blocks << ((*inode).i_blkbits - SECTOR_SHIFT);
}

pub unsafe fn gfs2_get_inode_blocks(inode: *const inode) -> u64 {
    (*inode).i_blocks >> ((*inode).i_blkbits - SECTOR_SHIFT)
}

pub unsafe fn gfs2_add_inode_blocks(inode: *mut inode, mut change: i64) {
    change <<= (*inode).i_blkbits - SECTOR_SHIFT;
    gfs2_assert(GFS2_SB(inode), change >= 0 || (*inode).i_blocks >= -change);
    (*inode).i_blocks += change;
}

pub unsafe fn gfs2_check_inum(
    ip: *const gfs2_inode,
    no_addr: u64,
    no_formal_ino: u64,
) -> std::ffi::c_int {
    ((*ip).i_no_addr == no_addr && (*ip).i_no_formal_ino == no_formal_ino) as std::ffi::c_int
}

pub unsafe fn gfs2_inum_out(ip: *const gfs2_inode, dent: *mut gfs2_dirent) {
    (*dent).de_inum.no_formal_ino = cpu_to_be64((*ip).i_no_formal_ino);
    (*dent).de_inum.no_addr = cpu_to_be64((*ip).i_no_addr);
}

pub unsafe fn gfs2_check_internal_file_size(
    inode: *mut inode,
    minsize: u64,
    maxsize: u64,
) -> std::ffi::c_int {
    let size = i_size_read(inode);
    if size < minsize || size > maxsize || size & (BIT((*inode).i_blkbits) - 1) != 0 {
        gfs2_consist_inode(GFS2_I(inode));
        return -EIO;
    }
    0
}

// CONFIG_GFS2_FS_LOCKING_DLM selects the locking-DLM file operations.
extern "C" {
    pub static gfs2_file_fops_nolock: file_operations;
    pub static gfs2_dir_fops_nolock: file_operations;
    #[cfg(feature = "CONFIG_GFS2_FS_LOCKING_DLM")]
    pub static gfs2_file_fops: file_operations;
    #[cfg(feature = "CONFIG_GFS2_FS_LOCKING_DLM")]
    pub static gfs2_dir_fops: file_operations;
}

#[cfg(feature = "CONFIG_GFS2_FS_LOCKING_DLM")]
pub unsafe fn gfs2_localflocks(sdp: *const gfs2_sbd) -> std::ffi::c_int {
    (*sdp).sd_args.ar_localflocks
}

#[cfg(not(feature = "CONFIG_GFS2_FS_LOCKING_DLM"))]
pub unsafe fn gfs2_localflocks(_sdp: *const gfs2_sbd) -> std::ffi::c_int {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
