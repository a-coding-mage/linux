/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * squashfs.h
 */

// C preprocessor logging macros TRACE, ERROR, and WARNING depend on the
// kernel logging macros and variadic macro syntax; preserve their intent.
// CONFIG_SQUASHFS_FILE_CACHE selects SQUASHFS_READ_PAGES as
// `msblk->max_thread_num`, otherwise it is 0.

extern "C" {

// External types supplied by the surrounding kernel code.
pub enum super_block {}
pub enum squashfs_page_actor {}
pub enum squashfs_cache {}
pub enum squashfs_cache_entry {}
pub enum squashfs_decompressor {}
pub enum squashfs_sb_info {}
pub enum bio {}
pub enum folio {}
pub enum inode {}
pub enum dentry {}
pub enum file_operations {}
pub enum export_operations {}
pub enum address_space_operations {}
pub enum inode_operations {}
pub enum xattr_handler {}

/* block.c */
pub fn squashfs_read_data(
    sb: *mut super_block,
    start: u64,
    length: core::ffi::c_int,
    next: *mut u64,
    actor: *mut squashfs_page_actor,
) -> core::ffi::c_int;

/* cache.c */
pub fn squashfs_cache_init(
    name: *mut core::ffi::c_char,
    entries: core::ffi::c_int,
    block_size: core::ffi::c_int,
) -> *mut squashfs_cache;
pub fn squashfs_cache_delete(cache: *mut squashfs_cache);
pub fn squashfs_cache_get(
    sb: *mut super_block,
    cache: *mut squashfs_cache,
    block: u64,
    length: core::ffi::c_int,
) -> *mut squashfs_cache_entry;
pub fn squashfs_cache_put(entry: *mut squashfs_cache_entry);
pub fn squashfs_copy_data(
    buffer: *mut core::ffi::c_void,
    entry: *mut squashfs_cache_entry,
    offset: core::ffi::c_int,
    length: core::ffi::c_int,
) -> core::ffi::c_int;
pub fn squashfs_read_metadata(
    sb: *mut super_block,
    buffer: *mut core::ffi::c_void,
    start: *mut u64,
    length: *mut core::ffi::c_int,
    block_size: core::ffi::c_int,
) -> core::ffi::c_int;
pub fn squashfs_get_fragment(
    sb: *mut super_block,
    block: u64,
    length: core::ffi::c_int,
) -> *mut squashfs_cache_entry;
pub fn squashfs_get_datablock(
    sb: *mut super_block,
    block: u64,
    length: core::ffi::c_int,
) -> *mut squashfs_cache_entry;
pub fn squashfs_read_table(
    sb: *mut super_block,
    start: u64,
    length: core::ffi::c_int,
) -> *mut core::ffi::c_void;

/* decompressor.c */
pub fn squashfs_lookup_decompressor(index: core::ffi::c_int) -> *const squashfs_decompressor;
pub fn squashfs_decompressor_setup(
    sb: *mut super_block,
    block_size: u16,
) -> *mut core::ffi::c_void;

/* decompressor_xxx.c */
#[repr(C)]
pub struct squashfs_decompressor_thread_ops {
    pub create: Option<unsafe extern "C" fn(
        msblk: *mut squashfs_sb_info,
        comp_opts: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void>,
    pub destroy: Option<unsafe extern "C" fn(msblk: *mut squashfs_sb_info)>,
    pub decompress: Option<unsafe extern "C" fn(
        msblk: *mut squashfs_sb_info,
        bio: *mut bio,
        offset: core::ffi::c_int,
        length: core::ffi::c_int,
        output: *mut squashfs_page_actor,
    ) -> core::ffi::c_int>,
    pub max_decompressors: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

/* export.c */
pub fn squashfs_read_inode_lookup_table(
    sb: *mut super_block,
    start: u64,
    length: u64,
    indexes: core::ffi::c_uint,
) -> *mut u64;

/* fragment.c */
pub fn squashfs_frag_lookup(
    sb: *mut super_block,
    fragment: core::ffi::c_uint,
    block: *mut u64,
) -> core::ffi::c_int;
pub fn squashfs_read_fragment_index_table(
    sb: *mut super_block,
    start: u64,
    length: u64,
    indexes: core::ffi::c_uint,
) -> *mut u64;

/* file.c */
pub fn squashfs_copy_cache(
    folio: *mut folio,
    entry: *mut squashfs_cache_entry,
    bytes: usize,
    offset: usize,
);

/* file_xxx.c */
pub fn squashfs_readpage_block(
    folio: *mut folio,
    block: u64,
    bsize: core::ffi::c_int,
    expected: core::ffi::c_int,
) -> core::ffi::c_int;

/* id.c */
pub fn squashfs_get_id(
    sb: *mut super_block,
    index: core::ffi::c_uint,
    id: *mut core::ffi::c_uint,
) -> core::ffi::c_int;
pub fn squashfs_read_id_index_table(
    sb: *mut super_block,
    start: u64,
    length: u64,
    indexes: u16,
) -> *mut u64;

/* inode.c */
pub fn squashfs_iget(
    sb: *mut super_block,
    inode_number: i64,
    inode_type: core::ffi::c_uint,
) -> *mut inode;
pub fn squashfs_read_inode(inode: *mut inode, inode_number: i64) -> core::ffi::c_int;

/* xattr.c */
pub fn squashfs_listxattr(
    dentry: *mut dentry,
    buffer: *mut core::ffi::c_char,
    size: usize,
) -> isize;

/* Inodes, files, decompressor and xattr operations. */

/* dir.c */
pub static squashfs_dir_ops: file_operations;
/* export.c */
pub static squashfs_export_ops: export_operations;
/* file.c */
pub static squashfs_aops: address_space_operations;
/* inode.c */
pub static squashfs_inode_ops: inode_operations;
pub static squashfs_file_operations: file_operations;
/* namei.c */
pub static squashfs_dir_inode_ops: inode_operations;
/* symlink.c */
pub static squashfs_symlink_aops: address_space_operations;
pub static squashfs_symlink_inode_ops: inode_operations;
/* xattr.c */
pub static squashfs_xattr_handlers: [*const xattr_handler; 0];

}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
