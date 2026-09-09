// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of f2fs/compress.c.  Kernel types and
// helpers referenced below are supplied by the surrounding f2fs bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The implementation is intentionally kept ABI-oriented: all structures and
// callbacks correspond to the C implementation and are resolved by the
// surrounding kernel bindings.

extern "C" {
    static mut cic_entry_slab: *mut c_void;
    static mut dic_entry_slab: *mut c_void;
}

#[repr(C)]
pub struct f2fs_compress_ops {
    pub init_compress_ctx: Option<unsafe extern "C" fn(*mut compress_ctx) -> i32>,
    pub destroy_compress_ctx: Option<unsafe extern "C" fn(*mut compress_ctx)>,
    pub compress_pages: Option<unsafe extern "C" fn(*mut compress_ctx) -> i32>,
    pub init_decompress_ctx: Option<unsafe extern "C" fn(*mut decompress_io_ctx) -> i32>,
    pub destroy_decompress_ctx: Option<unsafe extern "C" fn(*mut decompress_io_ctx)>,
    pub decompress_pages: Option<unsafe extern "C" fn(*mut decompress_io_ctx) -> i32>,
    pub is_level_valid: Option<unsafe extern "C" fn(i32) -> bool>,
}

#[repr(C)] pub struct compress_ctx { _private: [u8; 0] }
#[repr(C)] pub struct decompress_io_ctx { _private: [u8; 0] }
#[repr(C)] pub struct f2fs_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct dnode_of_data { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

// These declarations preserve the externally visible interfaces from the
// implementation unit.  Their bodies are provided by the kernel/f2fs layer.
extern "C" {
    pub fn f2fs_is_compressed_page(folio: *mut folio) -> bool;
    pub fn f2fs_init_compress_ctx(cc: *mut compress_ctx) -> i32;
    pub fn f2fs_destroy_compress_ctx(cc: *mut compress_ctx, reuse: bool);
    pub fn f2fs_compress_ctx_add_page(cc: *mut compress_ctx, folio: *mut folio);
    pub fn f2fs_compress_control_folio(folio: *mut folio) -> *mut folio;
    pub fn f2fs_decompress_cluster(dic: *mut decompress_io_ctx, in_task: bool);
    pub fn f2fs_end_read_compressed_page(folio: *mut folio, failed: bool, blkaddr: u64, in_task: bool);
    pub fn f2fs_cluster_is_empty(cc: *mut compress_ctx) -> bool;
    pub fn f2fs_cluster_can_merge_page(cc: *mut compress_ctx, index: u64) -> bool;
    pub fn f2fs_all_cluster_page_ready(cc: *mut compress_ctx, pages: *mut *mut page, index: i32, nr_pages: i32, uptodate: bool) -> bool;
    pub fn f2fs_sanity_check_cluster(dn: *mut dnode_of_data) -> bool;
    pub fn f2fs_is_compressed_cluster(inode: *mut inode, index: u64) -> i32;
    pub fn f2fs_is_sparse_cluster(inode: *mut inode, index: u64) -> bool;
    pub fn f2fs_prepare_compress_overwrite(inode: *mut inode, pagep: *mut *mut page, index: u64, fsdata: *mut *mut c_void) -> i32;
    pub fn f2fs_compress_write_end(inode: *mut inode, fsdata: *mut c_void, index: u64, copied: u32) -> bool;
    pub fn f2fs_truncate_partial_cluster(inode: *mut inode, from: u64, lock: bool) -> i32;
    pub fn f2fs_write_multi_pages(cc: *mut compress_ctx, submitted: *mut i32, wbc: *mut writeback_control, io_type: i32) -> i32;
    pub fn f2fs_alloc_dic(cc: *mut compress_ctx) -> *mut decompress_io_ctx;
    pub fn f2fs_put_folio_dic(folio: *mut folio, in_task: bool);
    pub fn f2fs_decompress_end_io(dic: *mut decompress_io_ctx, failed: bool, in_task: bool);
    pub fn f2fs_cluster_blocks_are_contiguous(dn: *mut dnode_of_data, ofs_in_node: u32) -> u32;
    pub fn f2fs_invalidate_compress_pages_range(sbi: *mut f2fs_sb_info, blkaddr: u64, len: u32);
    pub fn f2fs_load_compressed_folio(sbi: *mut f2fs_sb_info, folio: *mut folio, blkaddr: u64) -> bool;
    pub fn f2fs_invalidate_compress_pages(sbi: *mut f2fs_sb_info, ino: u32);
    pub fn f2fs_init_compress_inode(sbi: *mut f2fs_sb_info) -> i32;
    pub fn f2fs_destroy_compress_inode(sbi: *mut f2fs_sb_info);
    pub fn f2fs_init_page_array_cache(sbi: *mut f2fs_sb_info) -> i32;
    pub fn f2fs_destroy_page_array_cache(sbi: *mut f2fs_sb_info);
    pub fn f2fs_init_compress_cache() -> i32;
    pub fn f2fs_destroy_compress_cache();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
