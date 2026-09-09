/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/rbtree.h, refcount.h, fiemap.h, btrfs_tree.h,
// spinlock.h, atomic.h, rwsem.h, list.h, slab.h, messages.h, ulist.h, misc.h.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod extent_io_header {
    pub const EXTENT_BUFFER_UPTODATE: u32 = 0;
    pub const EXTENT_BUFFER_DIRTY: u32 = 1;
    pub const EXTENT_BUFFER_TREE_REF: u32 = 2;
    pub const EXTENT_BUFFER_STALE: u32 = 3;
    pub const EXTENT_BUFFER_WRITEBACK: u32 = 4;
    pub const EXTENT_BUFFER_UNMAPPED: u32 = 5;
    pub const EXTENT_BUFFER_WRITE_ERR: u32 = 6;
    pub const EXTENT_BUFFER_ZONED_ZEROOUT: u32 = 7;
    pub const EXTENT_BUFFER_READING: u32 = 8;

    pub const PAGE_UNLOCK: u32 = 0;
    pub const PAGE_START_WRITEBACK: u32 = 1;
    pub const PAGE_END_WRITEBACK: u32 = 2;

    pub const EXTENT_FOLIO_PRIVATE: usize = 1;
    pub const INLINE_EXTENT_BUFFER_PAGES: usize = BTRFS_MAX_METADATA_BLOCKSIZE / PAGE_SIZE;

    pub type u64_ = u64;
    pub type u32_ = u32;
    pub type s8 = i8;
    pub type u8_ = u8;
    pub type gfp_t = usize;
    pub type pid_t = i32;

    #[repr(C)] pub struct page { _unused: [u8; 0] }
    #[repr(C)] pub struct file { _unused: [u8; 0] }
    #[repr(C)] pub struct folio { _unused: [u8; 0] }
    #[repr(C)] pub struct inode { _unused: [u8; 0] }
    #[repr(C)] pub struct fiemap_extent_info { _unused: [u8; 0] }
    #[repr(C)] pub struct readahead_control { _unused: [u8; 0] }
    #[repr(C)] pub struct address_space { _unused: [u8; 0] }
    #[repr(C)] pub struct writeback_control { _unused: [u8; 0] }
    #[repr(C)] pub struct extent_io_tree { _unused: [u8; 0] }
    #[repr(C)] pub struct extent_map_tree { _unused: [u8; 0] }
    #[repr(C)] pub struct extent_state { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_block_group { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_fs_info { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_inode { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_root { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_trans_handle { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_tree_parent_check { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_folio_state { _unused: [u8; 0] }
    #[repr(C)] pub struct btrfs_key { _unused: [u8; 0] }
    #[repr(C)] pub struct rcu_head { _unused: [u8; 0] }
    #[repr(C)] pub struct spinlock_t { _unused: [u8; 0] }
    #[repr(C)] pub struct refcount_t { _unused: [u8; 0] }
    #[repr(C)] pub struct atomic_t { _unused: [u8; 0] }
    #[repr(C)] pub struct rw_semaphore { _unused: [u8; 0] }
    #[repr(C)] pub struct list_head { _unused: [u8; 0] }
    #[repr(C)] pub struct ulist { pub prealloc: *mut ulist_node }
    #[repr(C)] pub struct ulist_node { _unused: [u8; 0] }

    pub const BITS_PER_BYTE: usize = 8;
    pub const BTRFS_FSID_SIZE: usize = 16;
    pub const PAGE_SHIFT: usize = 12;
    pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
    pub const BTRFS_MAX_METADATA_BLOCKSIZE: usize = 65536;
    pub const PAGECACHE_TAG_DIRTY: u32 = 0;
    pub const TASK_UNINTERRUPTIBLE: u32 = 2;

    #[inline] pub const fn bit_byte(nr: usize) -> usize { nr / BITS_PER_BYTE }
    #[inline] pub const fn byte_mask() -> u32 { (1u32 << BITS_PER_BYTE) - 1 }
    #[inline] pub const fn bitmap_first_byte_mask(start: usize) -> u32 { (byte_mask() << (start & (BITS_PER_BYTE - 1))) & byte_mask() }
    #[inline] pub const fn bitmap_last_byte_mask(nbits: usize) -> u32 { byte_mask() >> ((-(nbits as isize) as usize) & (BITS_PER_BYTE - 1)) }

    #[repr(C)]
    pub struct extent_buffer {
        pub start: u64, pub len: u32, pub folio_size: u32, pub bflags: usize,
        pub fs_info: *mut btrfs_fs_info, pub addr: *mut core::ffi::c_void,
        pub refs_lock: spinlock_t, pub refs: refcount_t, pub read_mirror: i32,
        pub writeback_inhibitors: atomic_t, pub log_index: s8, pub folio_shift: u8,
        pub rcu_head: rcu_head, pub lock: rw_semaphore,
        pub folios: [*mut folio; INLINE_EXTENT_BUFFER_PAGES],
    }

    #[repr(C)] pub struct btrfs_eb_prealloc { pub eb: *mut extent_buffer, pub bfs: *mut btrfs_folio_state, pub supports_nowait: bool, pub needs_prealloc: bool }
    #[repr(C)] pub struct btrfs_eb_write_context { pub wbc: *mut writeback_control, pub eb: *mut extent_buffer, pub zoned_bg: *mut btrfs_block_group }
    #[repr(C)] pub struct extent_changeset { pub bytes_changed: u64, pub range_changed: ulist }

    pub const EXTENT_CHANGESET_BYTES_ONLY: *mut ulist_node = 1usize as *mut ulist_node;

    extern "C" {
        pub fn extent_buffer_init_cachep() -> i32;
        pub fn extent_buffer_free_cachep();
        pub fn try_release_extent_mapping(folio: *mut folio, mask: gfp_t) -> bool;
        pub fn try_release_extent_buffer(folio: *mut folio) -> i32;
        pub fn btrfs_read_folio(file: *mut file, folio: *mut folio) -> i32;
        pub fn extent_write_locked_range(inode: *mut inode, locked_folio: *const folio, start: u64, end: u64, wbc: *mut writeback_control, pages_dirty: bool);
        pub fn btrfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32;
        pub fn btree_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32;
        pub fn btrfs_btree_wait_writeback_range(fs_info: *mut btrfs_fs_info, start: u64, end: u64);
        pub fn btrfs_readahead(rac: *mut readahead_control);
        pub fn set_folio_extent_mapped(folio: *mut folio) -> i32;
        pub fn clear_folio_extent_mapped(folio: *mut folio);
        pub fn alloc_extent_buffer(fs_info: *mut btrfs_fs_info, pa: *mut btrfs_eb_prealloc, start: u64, owner_root: u64, level: i32) -> *mut extent_buffer;
        pub fn btrfs_init_eb_prealloc(fs_info: *mut btrfs_fs_info, pa: *mut btrfs_eb_prealloc, nowait: bool) -> i32;
        pub fn btrfs_free_eb_prealloc(pa: *mut btrfs_eb_prealloc);
        pub fn alloc_dummy_extent_buffer(fs_info: *mut btrfs_fs_info, start: u64) -> *mut extent_buffer;
        pub fn btrfs_clone_extent_buffer(src: *const extent_buffer) -> *mut extent_buffer;
        pub fn find_extent_buffer(fs_info: *mut btrfs_fs_info, start: u64) -> *mut extent_buffer;
        pub fn free_extent_buffer(eb: *mut extent_buffer);
        pub fn free_extent_buffer_stale(eb: *mut extent_buffer);
        pub fn read_extent_buffer_pages(eb: *mut extent_buffer, mirror_num: i32, parent_check: *const btrfs_tree_parent_check) -> i32;
        pub fn read_extent_buffer_pages_nowait(eb: *mut extent_buffer, mirror_num: i32, parent_check: *const btrfs_tree_parent_check) -> i32;
        pub fn btrfs_readahead_tree_block(fs_info: *mut btrfs_fs_info, bytenr: u64, owner_root: u64, gen: u64, level: i32, first_key: *const btrfs_key);
        pub fn btrfs_readahead_node_child(node: *mut extent_buffer, slot: i32);
        pub fn memcmp_extent_buffer(eb: *const extent_buffer, ptrv: *const core::ffi::c_void, start: usize, len: usize) -> i32;
        pub fn read_extent_buffer(eb: *const extent_buffer, dst: *mut core::ffi::c_void, start: usize, len: usize);
        pub fn read_extent_buffer_to_user_nofault(eb: *const extent_buffer, dst: *mut core::ffi::c_void, start: usize, len: usize) -> i32;
        pub fn write_extent_buffer(eb: *const extent_buffer, src: *const core::ffi::c_void, start: usize, len: usize);
        pub fn copy_extent_buffer_full(dst: *const extent_buffer, src: *const extent_buffer);
        pub fn copy_extent_buffer(dst: *const extent_buffer, src: *const extent_buffer, dst_offset: usize, src_offset: usize, len: usize);
        pub fn memcpy_extent_buffer(dst: *const extent_buffer, dst_offset: usize, src_offset: usize, len: usize);
        pub fn memmove_extent_buffer(dst: *const extent_buffer, dst_offset: usize, src_offset: usize, len: usize);
        pub fn memzero_extent_buffer(eb: *const extent_buffer, start: usize, len: usize);
        pub fn extent_buffer_test_bit(eb: *const extent_buffer, start: usize, pos: usize) -> bool;
        pub fn extent_buffer_bitmap_set(eb: *const extent_buffer, start: usize, pos: usize, len: usize);
        pub fn extent_buffer_bitmap_clear(eb: *const extent_buffer, start: usize, pos: usize, len: usize);
        pub fn set_extent_buffer_dirty(eb: *mut extent_buffer);
        pub fn set_extent_buffer_uptodate(eb: *mut extent_buffer);
        pub fn clear_extent_buffer_uptodate(eb: *mut extent_buffer);
        pub fn btrfs_alloc_page_array(nr_pages: u32, page_array: *mut *mut page, gfp: gfp_t) -> i32;
        pub fn btrfs_alloc_folio_array(nr_folios: u32, order: u32, folio_array: *mut *mut folio, gfp: gfp_t) -> i32;
        pub fn alloc_test_extent_buffer(fs_info: *mut btrfs_fs_info, start: u64) -> *mut extent_buffer;
        pub fn btrfs_inhibit_eb_writeback(trans: *mut btrfs_trans_handle, eb: *mut extent_buffer);
        pub fn btrfs_uninhibit_all_eb_writeback(trans: *mut btrfs_trans_handle);
        pub fn extent_clear_unlock_delalloc(inode: *mut btrfs_inode, start: u64, end: u64, locked_folio: *const folio, cached: *mut *mut extent_state, bits_to_clear: u32, page_ops: usize);
        pub fn btrfs_clear_buffer_dirty(trans: *mut btrfs_trans_handle, buf: *mut extent_buffer);
        pub fn btrfs_zoned_release_dirty_metadata(fs_info: *mut btrfs_fs_info);
    }

    #[inline] pub unsafe fn offset_in_eb_folio(eb: *const extent_buffer, start: u64) -> usize { start as usize & ((*eb).folio_size as usize - 1) }
    #[inline] pub unsafe fn get_eb_offset_in_folio(eb: *const extent_buffer, offset: usize) -> usize { offset + (*eb).start as usize }
    #[inline] pub unsafe fn get_eb_folio_index(eb: *const extent_buffer, offset: usize) -> usize { offset >> (*eb).folio_shift }
    #[inline] pub unsafe fn extent_changeset_init(c: *mut extent_changeset) { (*c).bytes_changed = 0; (*c).range_changed.prealloc = core::ptr::null_mut(); }
    #[inline] pub unsafe fn extent_changeset_init_bytes_only(c: *mut extent_changeset) { (*c).bytes_changed = 0; (*c).range_changed.prealloc = EXTENT_CHANGESET_BYTES_ONLY; }
    #[inline] pub unsafe fn extent_changeset_tracks_ranges(c: *const extent_changeset) -> bool { (*c).range_changed.prealloc != EXTENT_CHANGESET_BYTES_ONLY }
    #[inline] pub unsafe fn extent_changeset_prealloc(c: *mut extent_changeset, _gfp_mask: gfp_t) { let _ = c; }
    #[inline] pub unsafe fn extent_changeset_release(c: *mut extent_changeset) { if !c.is_null() { (*c).bytes_changed = 0; } }
    #[inline] pub unsafe fn extent_changeset_free(c: *mut extent_changeset) { if !c.is_null() { extent_changeset_release(c); } }
    #[inline] pub unsafe fn wait_on_extent_buffer_writeback(_eb: *mut extent_buffer) {}
    #[inline] pub unsafe fn num_extent_pages(eb: *const extent_buffer) -> i32 { let n = ((*eb).len as usize >> PAGE_SHIFT) as i32; if n == 0 { 1 } else { n } }
    #[inline] pub unsafe fn num_extent_folios(eb: *const extent_buffer) -> i32 { if (*eb).folios[0].is_null() { 0 } else { num_extent_pages(eb) } }
    #[inline] pub unsafe fn extent_buffer_uptodate(_eb: *const extent_buffer) -> bool { true }
    #[inline] pub unsafe fn extent_buffer_under_io(_eb: *const extent_buffer) -> bool { true }
    #[inline] pub unsafe fn write_extent_buffer_chunk_tree_uuid(eb: *const extent_buffer, uuid: *const core::ffi::c_void) { write_extent_buffer(eb, uuid, 0, BTRFS_FSID_SIZE); }
    #[inline] pub unsafe fn write_extent_buffer_fsid(eb: *const extent_buffer, fsid: *const core::ffi::c_void) { write_extent_buffer(eb, fsid, 0, BTRFS_FSID_SIZE); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
