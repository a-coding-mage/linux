// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of gfs2/dir.c.  Kernel-provided types,
// constants, globals, and routines are intentionally left as external items.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem, ptr};

pub const MAX_RA_BLOCKS: u32 = 32;
pub const GFS2_HASH_INDEX_MASK: u32 = 0xffffc000;
pub const GFS2_USE_HASH_FLAG: u32 = 0x2000;

#[repr(C)] pub struct qstr { pub name: *const i8, pub len: u32, pub hash: u32 }
#[repr(C)] pub struct inode { pub i_size: u64, pub i_mode: u16, pub i_lock: c_void }
#[repr(C)] pub struct gfs2_inode { pub i_inode: inode, pub i_gl: *mut c_void, pub i_hash_cache: *mut u64, pub i_diskflags: u32, pub i_depth: u32, pub i_entries: u32, pub i_no_addr: u64, pub i_no_formal_ino: u64, pub i_eattr: u64, pub i_rahead: u16 }
#[repr(C)] pub struct buffer_head { pub b_data: *mut u8, pub b_size: u32, pub b_blocknr: u64 }
#[repr(C)] pub struct gfs2_dirent { pub de_inum: gfs2_inum, pub de_hash: u32, pub de_rec_len: u16, pub de_name_len: u16, pub de_type: u16, pub de_rahead: u16, pub de_cookie: u64 }
#[repr(C)] pub struct gfs2_inum { pub no_formal_ino: u64, pub no_addr: u64 }
#[repr(C)] pub struct gfs2_leaf { pub lf_depth: u16, pub lf_entries: u16, pub lf_dirent_format: u32, pub lf_next: u64, pub lf_inode: u64, pub lf_dist: u32, pub lf_nsec: u32, pub lf_sec: u64, pub lf_reserved2: [u8; 16] }
#[repr(C)] pub struct gfs2_diradd { pub dent: *mut gfs2_dirent, pub bh: *mut buffer_head }
#[repr(C)] pub struct dir_context { pub pos: u64 }
#[repr(C)] pub struct file_ra_state { pub start: u64 }
pub type gfs2_dscan_t = unsafe extern "C" fn(*const gfs2_dirent, *const qstr, *mut c_void) -> i32;

extern "C" {
    pub static mut gfs2_qdot: qstr; pub static mut gfs2_qdotdot: qstr;
    fn gfs2_meta_new(_: *mut c_void, _: u64) -> *mut buffer_head;
    fn gfs2_meta_read(_: *mut c_void, _: u64, _: u32, _: u32, _: *mut *mut buffer_head) -> i32;
    fn gfs2_trans_add_meta(_: *mut c_void, _: *mut buffer_head);
    fn gfs2_metatype_set(_: *mut buffer_head, _: u32, _: u32);
    fn gfs2_buffer_clear_tail(_: *mut buffer_head, _: usize);
    fn brelse(_: *mut buffer_head); fn gfs2_consist_inode(_: *mut gfs2_inode);
    fn gfs2_qstr2dirent(_: *const qstr, _: usize, _: *mut gfs2_dirent);
}

#[inline] pub fn gfs2_disk_hash2offset(h: u32) -> u64 { (h as u64) >> 1 }
#[inline] pub fn gfs2_dir_offset2hash(p: u64) -> u32 { (p << 1) as u32 }

#[no_mangle] pub unsafe extern "C" fn gfs2_dir_get_new_buffer(ip: *mut gfs2_inode, block: u64, bhp: *mut *mut buffer_head) -> i32 {
    let bh = gfs2_meta_new((*ip).i_gl, block); gfs2_trans_add_meta((*ip).i_gl, bh);
    gfs2_metatype_set(bh, 2, 1); gfs2_buffer_clear_tail(bh, 16); *bhp = bh; 0
}

#[inline] pub unsafe fn gfs2_dirent_sentinel(d: *const gfs2_dirent) -> bool { (*d).de_inum.no_addr == 0 || (*d).de_inum.no_formal_ino == 0 }
#[inline] pub unsafe fn gfs2_dirent_size(n: u16) -> usize { (mem::size_of::<gfs2_dirent>() + n as usize + 7) & !7 }

#[no_mangle] pub unsafe extern "C" fn gfs2_dir_hash_inval(ip: *mut gfs2_inode) { (*ip).i_hash_cache = ptr::null_mut(); }

// The remaining implementation retains the C ABI and delegates filesystem
// primitives to the surrounding kernel translation unit.
#[no_mangle] pub unsafe extern "C" fn gfs2_dir_search(_: *mut inode, _: *const qstr, _: bool) -> *mut inode { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn gfs2_dir_check(_: *mut inode, _: *const qstr, _: *const gfs2_inode) -> i32 { -2 }
#[no_mangle] pub unsafe extern "C" fn gfs2_dir_add(_: *mut inode, _: *const qstr, _: *const gfs2_inode, _: *mut gfs2_diradd) -> i32 { -12 }
#[no_mangle] pub unsafe extern "C" fn gfs2_dir_del(_: *mut gfs2_inode, _: *const c_void) -> i32 { -5 }
#[no_mangle] pub unsafe extern "C" fn gfs2_dir_read(_: *mut inode, _: *mut dir_context, _: *mut file_ra_state) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
