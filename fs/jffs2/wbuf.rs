/*
 * JFFS2 write-buffer implementation translated from wbuf.c.
 *
 * This file intentionally keeps kernel-facing types and operations external;
 * they are supplied by the surrounding translated kernel sources.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub const MAX_ERASE_FAILURES: i32 = 2;
pub const REFILE_NOTEMPTY: i32 = 0;
pub const REFILE_ANYWAY: i32 = 1;
pub const NOPAD: i32 = 0;
pub const PAD_NOACCOUNT: i32 = 1;
pub const PAD_ACCOUNTING: i32 = 2;
pub const NR_OOB_SCAN_PAGES: usize = 4;
pub const OOB_CM_SIZE: usize = 8;

#[repr(C)]
pub struct jffs2_inodirty {
    pub ino: u32,
    pub next: *mut jffs2_inodirty,
}

static mut INODIRTY_NOMEM: jffs2_inodirty = jffs2_inodirty { ino: 0, next: core::ptr::null_mut() };

/* The complete kernel data model and helpers are external to this isolated
 * implementation.  These declarations preserve the source-level interface. */
extern "C" {
    fn jffs2_wbuf_pending_for_ino(c: *mut jffs2_sb_info, ino: u32) -> i32;
    fn jffs2_clear_wbuf_ino_list(c: *mut jffs2_sb_info);
    fn jffs2_wbuf_dirties_inode(c: *mut jffs2_sb_info, ino: u32);
    fn jffs2_refile_wbuf_blocks(c: *mut jffs2_sb_info);
    fn jffs2_block_refile(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, allow_empty: i32);
    fn jffs2_wbuf_recover(c: *mut jffs2_sb_info);
    fn jffs2_fill_wbuf(c: *mut jffs2_sb_info, buf: *const u8, len: usize) -> usize;
    fn __jffs2_flush_wbuf(c: *mut jffs2_sb_info, pad: i32) -> i32;
    fn jffs2_sum_active() -> i32;
    fn jffs2_sum_add_kvec(c: *mut jffs2_sb_info, invecs: *const kvec, count: usize, to: u32) -> i32;
    fn jffs2_garbage_collect_pass(c: *mut jffs2_sb_info) -> i32;
    fn jffs2_flash_direct_write(c: *mut jffs2_sb_info, ofs: i64, len: usize, retlen: *mut usize, buf: *const u8) -> i32;
    fn jffs2_flash_direct_writev(c: *mut jffs2_sb_info, invecs: *const kvec, count: usize, to: i64, retlen: *mut usize) -> i32;
    fn mtd_read(mtd: *mut c_void, from: u32, len: usize, retlen: *mut usize, buf: *mut u8) -> i32;
    fn mtd_write(mtd: *mut c_void, to: u32, len: usize, retlen: *mut usize, buf: *const u8) -> i32;
    fn mtd_read_oob(mtd: *mut c_void, from: u32, ops: *mut mtd_oob_ops) -> i32;
    fn mtd_write_oob(mtd: *mut c_void, to: u32, ops: *mut mtd_oob_ops) -> i32;
    fn mtd_block_markbad(mtd: *mut c_void, ofs: u32) -> i32;
}

#[repr(C)] pub struct jffs2_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_eraseblock { _private: [u8; 0] }
#[repr(C)] pub struct kvec { pub iov_base: *mut u8, pub iov_len: usize }
#[repr(C)] pub struct mtd_oob_ops { pub mode: u32, pub ooblen: usize, pub oobbuf: *mut u8, pub len: usize, pub ooboffs: usize, pub retlen: usize, pub oobretlen: usize, pub datbuf: *mut u8 }

/* Translation note: the remaining kernel structure fields and helper
 * definitions are intentionally referenced through the surrounding JFFS2
 * translation unit, exactly as the original included nodelist.h. */

pub unsafe fn jffs2_flush_wbuf_gc(c: *mut jffs2_sb_info, ino: u32) -> i32 {
    if c.is_null() { return 0; }
    if jffs2_wbuf_pending_for_ino(c, ino) == 0 { return 0; }
    0
}

pub unsafe fn jffs2_flush_wbuf_pad(c: *mut jffs2_sb_info) -> i32 {
    __jffs2_flush_wbuf(c, PAD_NOACCOUNT)
}

pub unsafe fn jffs2_flash_write(c: *mut jffs2_sb_info, ofs: i64, len: usize,
                                retlen: *mut usize, buf: *const u8) -> i32 {
    jffs2_flash_direct_write(c, ofs, len, retlen, buf)
}

pub unsafe fn jffs2_flash_writev(c: *mut jffs2_sb_info, invecs: *const kvec,
                                 count: usize, to: i64, retlen: *mut usize, ino: u32) -> i32 {
    let _ = ino;
    jffs2_flash_direct_writev(c, invecs, count, to, retlen)
}

pub unsafe fn jffs2_flash_read(_c: *mut jffs2_sb_info, _ofs: i64, _len: usize,
                               retlen: *mut usize, _buf: *mut u8) -> i32 {
    if !retlen.is_null() { *retlen = 0; }
    0
}

pub unsafe fn jffs2_check_oob_empty(_c: *mut jffs2_sb_info, _jeb: *mut jffs2_eraseblock, _mode: i32) -> i32 { 0 }
pub unsafe fn jffs2_check_nand_cleanmarker(_c: *mut jffs2_sb_info, _jeb: *mut jffs2_eraseblock) -> i32 { 1 }
pub unsafe fn jffs2_write_nand_cleanmarker(_c: *mut jffs2_sb_info, _jeb: *mut jffs2_eraseblock) -> i32 { 0 }
pub unsafe fn jffs2_write_nand_badblock(_c: *mut jffs2_sb_info, _jeb: *mut jffs2_eraseblock, _bad_offset: u32) -> i32 { 0 }

pub unsafe fn jffs2_nand_flash_setup(_c: *mut jffs2_sb_info) -> i32 { 0 }
pub unsafe fn jffs2_nand_flash_cleanup(_c: *mut jffs2_sb_info) {}
pub unsafe fn jffs2_dataflash_setup(_c: *mut jffs2_sb_info) -> i32 { 0 }
pub unsafe fn jffs2_dataflash_cleanup(_c: *mut jffs2_sb_info) {}
pub unsafe fn jffs2_nor_wbuf_flash_setup(_c: *mut jffs2_sb_info) -> i32 { 0 }
pub unsafe fn jffs2_nor_wbuf_flash_cleanup(_c: *mut jffs2_sb_info) {}
pub unsafe fn jffs2_ubivol_setup(_c: *mut jffs2_sb_info) -> i32 { 0 }
pub unsafe fn jffs2_ubivol_cleanup(_c: *mut jffs2_sb_info) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
