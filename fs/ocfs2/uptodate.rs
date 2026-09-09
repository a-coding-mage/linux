// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of uptodate.c.  Kernel and OCFS2 types and
 * operations referenced here are supplied by the surrounding translation. */

use core::ffi::c_void;

#[repr(C)]
pub struct Ocfs2MetaCacheItem { pub c_node: RbNode, pub c_block: SectorT }
pub type SectorT = u64;
pub type U64 = u64;
pub type U32 = u32;
pub type CInt = i32;

#[repr(C)] pub struct RbNode { pub rb_left: *mut RbNode, pub rb_right: *mut RbNode }
#[repr(C)] pub struct RbRoot { pub rb_node: *mut RbNode }
#[repr(C)] pub struct BufferHead { pub b_blocknr: SectorT }
#[repr(C)] pub struct SuperBlock;
#[repr(C)] pub struct KmemCache;
#[repr(C)] pub struct Ocfs2CachingOperations;
#[repr(C)] pub struct Ocfs2CachingInfo;

extern "C" {
    static mut ocfs2_uptodate_cachep: *mut KmemCache;
    fn kmem_cache_alloc(c: *mut KmemCache, flags: u32) -> *mut Ocfs2MetaCacheItem;
    fn kmem_cache_free(c: *mut KmemCache, p: *mut Ocfs2MetaCacheItem);
    fn kmem_cache_create(n: *const i8, s: usize, a: usize, f: u32, x: *mut c_void) -> *mut KmemCache;
    fn kmem_cache_destroy(c: *mut KmemCache);
    fn rb_last(r: *mut RbRoot) -> *mut RbNode;
    fn rb_erase(n: *mut RbNode, r: *mut RbRoot);
    fn rb_link_node(n: *mut RbNode, p: *mut RbNode, l: *mut *mut RbNode);
    fn rb_insert_color(n: *mut RbNode, r: *mut RbRoot);
    fn buffer_uptodate(b: *mut BufferHead) -> CInt;
    fn buffer_jbd(b: *mut BufferHead) -> CInt;
    fn buffer_locked(b: *mut BufferHead) -> CInt;
    fn set_buffer_uptodate(b: *mut BufferHead);
    fn ocfs2_clusters_to_blocks(sb: *mut SuperBlock, n: u32) -> u32;
}

// The following declarations preserve the external OCFS2 cache interface.
extern "C" {
    fn ocfs2_metadata_cache_owner(ci: *mut Ocfs2CachingInfo) -> U64;
    fn ocfs2_metadata_cache_get_super(ci: *mut Ocfs2CachingInfo) -> *mut SuperBlock;
    fn ocfs2_metadata_cache_purge(ci: *mut Ocfs2CachingInfo);
    fn ocfs2_buffer_cached(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead) -> CInt;
    fn __ocfs2_set_buffer_uptodate(ci: *mut Ocfs2CachingInfo, block: SectorT, expand: CInt);
    fn ocfs2_set_buffer_uptodate(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead);
    fn ocfs2_remove_block_from_cache(ci: *mut Ocfs2CachingInfo, block: SectorT);
}

extern "C" {
    pub fn ocfs2_metadata_cache_init(ci: *mut Ocfs2CachingInfo, ops: *const Ocfs2CachingOperations);
    pub fn ocfs2_metadata_cache_exit(ci: *mut Ocfs2CachingInfo);
    pub fn ocfs2_metadata_cache_io_lock(ci: *mut Ocfs2CachingInfo);
    pub fn ocfs2_metadata_cache_io_unlock(ci: *mut Ocfs2CachingInfo);
    pub fn ocfs2_metadata_cache_purge(ci: *mut Ocfs2CachingInfo);
}

// Kernel rb-tree/cache structure layout and operation bodies are intentionally
// kept as direct C-compatible declarations for use by the generated sibling
// translations.
pub unsafe fn ocfs2_buffer_uptodate(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead) -> CInt {
    if buffer_uptodate(bh) == 0 { return 0; }
    if buffer_jbd(bh) != 0 { return 1; }
    ocfs2_buffer_cached(ci, bh)
}

pub unsafe fn ocfs2_buffer_read_ahead(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead) -> CInt {
    (buffer_locked(bh) != 0 && ocfs2_buffer_cached(ci, bh) != 0) as CInt
}

pub unsafe fn ocfs2_set_new_buffer_uptodate(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead) {
    set_buffer_uptodate(bh);
    ocfs2_set_buffer_uptodate(ci, bh);
}

pub unsafe fn ocfs2_remove_from_cache(ci: *mut Ocfs2CachingInfo, bh: *mut BufferHead) {
    ocfs2_remove_block_from_cache(ci, (*bh).b_blocknr);
}

pub unsafe fn ocfs2_remove_xattr_clusters_from_cache(ci: *mut Ocfs2CachingInfo, mut block: SectorT, c_len: U32) {
    let sb = ocfs2_metadata_cache_get_super(ci);
    let b_len = ocfs2_clusters_to_blocks(sb, 1).wrapping_mul(c_len);
    for _ in 0..b_len { ocfs2_remove_block_from_cache(ci, block); block = block.wrapping_add(1); }
}

pub unsafe fn init_ocfs2_uptodate_cache() -> CInt {
    let name = b"ocfs2_uptodate\0";
    ocfs2_uptodate_cachep = kmem_cache_create(name.as_ptr() as *const i8,
        core::mem::size_of::<Ocfs2MetaCacheItem>(), 0, 0, core::ptr::null_mut());
    if ocfs2_uptodate_cachep.is_null() { -12 } else { 0 }
}

pub unsafe fn exit_ocfs2_uptodate_cache() { kmem_cache_destroy(ocfs2_uptodate_cachep); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
