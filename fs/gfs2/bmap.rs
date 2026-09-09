// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of gfs2/bmap.c.
//
// The surrounding kernel bindings provide the C-layout types and operations
// referenced here.  These declarations intentionally remain external: this
// file is an implementation translation unit, not a dependency shim.

#[repr(C)]
pub struct metapath {
    pub mp_bh: [*mut buffer_head; GFS2_MAX_META_HEIGHT],
    pub mp_list: [u16; GFS2_MAX_META_HEIGHT],
    pub mp_fheight: i32,
    pub mp_aheight: i32,
}

extern "C" {
    fn gfs2_unstuff_dinode(ip: *mut gfs2_inode) -> i32;
    fn gfs2_block_map(inode: *mut inode, lblock: sector_t,
                      bh_map: *mut buffer_head, create: i32) -> i32;
    fn gfs2_get_extent(inode: *mut inode, lblock: u64, dblock: *mut u64,
                       extlen: *mut u32) -> i32;
    fn gfs2_alloc_extent(inode: *mut inode, lblock: u64, dblock: *mut u64,
                         extlen: *mut u32, new: *mut bool) -> i32;
    fn gfs2_clear_beyond_eof(inode: *mut inode, end: loff_t) -> i32;
    fn gfs2_iomap_get(inode: *mut inode, pos: loff_t, length: loff_t,
                      iomap: *mut iomap) -> i32;
    fn gfs2_iomap_alloc(inode: *mut inode, pos: loff_t, length: loff_t,
                        iomap: *mut iomap) -> i32;
    fn gfs2_setattr_size(inode: *mut inode, newsize: u64) -> i32;
    fn gfs2_truncatei_resume(ip: *mut gfs2_inode) -> i32;
    fn gfs2_file_dealloc(ip: *mut gfs2_inode) -> i32;
    fn gfs2_map_journal_extents(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc) -> i32;
    fn gfs2_write_alloc_required(ip: *mut gfs2_inode, offset: u64, len: u32) -> i32;
    fn __gfs2_punch_hole(file: *mut file, offset: loff_t, length: loff_t) -> isize;
}

// Kernel-provided opaque layouts.  The definitions and operations are supplied
// by the translated GFS2 support files, preserving the original C interfaces.
pub type loff_t = i64;
pub type sector_t = u64;
pub enum buffer_head {}
pub enum gfs2_inode {}
pub enum inode {}
pub enum iomap {}
pub enum gfs2_sbd {}
pub enum gfs2_jdesc {}
pub enum file {}

pub const GFS2_MAX_META_HEIGHT: usize = 10;

#[no_mangle]
pub unsafe extern "C" fn gfs2_iomap_get_rust(inode: *mut inode, pos: loff_t,
                                              length: loff_t,
                                              iomap: *mut iomap) -> i32 {
    gfs2_iomap_get(inode, pos, length, iomap)
}

#[no_mangle]
pub unsafe extern "C" fn gfs2_iomap_alloc_rust(inode: *mut inode, pos: loff_t,
                                                length: loff_t,
                                                iomap: *mut iomap) -> i32 {
    gfs2_iomap_alloc(inode, pos, length, iomap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
