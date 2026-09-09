/* SPDX-License-Identifier: GPL-2.0 */

// Linux dependencies and btrfs_inode.h are supplied by other translation units.

use core::ffi::c_void;

#[repr(C)]
pub struct address_space;
#[repr(C)]
pub struct folio {
    pub mapping: *mut address_space,
}

#[repr(C)]
pub struct btrfs_fs_info {
    pub nodesize: usize,
    pub sectorsize: usize,
}

#[repr(C)]
pub struct extent_buffer;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

pub type gfp_t = u32;
pub type u64 = core::primitive::u64;
pub type u32 = core::primitive::u32;

pub const PAGE_SIZE: usize = 4096;

pub const btrfs_bitmap_nr_uptodate: i32 = 0;
pub const btrfs_bitmap_nr_dirty: i32 = 1;
pub const btrfs_bitmap_nr_writeback: i32 = 2;
pub const btrfs_bitmap_nr_fixup: i32 = 3;
pub const btrfs_bitmap_nr_max: i32 = 4;

#[repr(C)]
pub struct btrfs_folio_state {
    pub lock: spinlock_t,
    pub refs: btrfs_folio_state_refs,
    pub bitmaps: [c_void; 0],
}

#[repr(C)]
pub union btrfs_folio_state_refs {
    pub eb_refs: atomic_t,
    pub nr_locked: atomic_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum btrfs_folio_type {
    BTRFS_SUBPAGE_METADATA,
    BTRFS_SUBPAGE_DATA,
}

#[inline]
pub unsafe fn btrfs_meta_is_subpage(fs_info: *const btrfs_fs_info) -> bool {
    (*fs_info).nodesize < PAGE_SIZE
}

#[inline]
pub unsafe fn btrfs_is_subpage(
    fs_info: *const btrfs_fs_info,
    folio: *mut folio,
) -> bool {
    // C condition: if (folio->mapping && folio->mapping->host)
    // ASSERT(is_data_inode(BTRFS_I(folio->mapping->host)));
    (*fs_info).sectorsize < folio_size(folio)
}

extern "C" {
    pub fn folio_size(folio: *mut folio) -> usize;
    pub fn btrfs_attach_folio_state(
        fs_info: *const btrfs_fs_info,
        folio: *mut folio,
        type_: btrfs_folio_type,
    ) -> i32;
    pub fn btrfs_detach_folio_state(
        fs_info: *const btrfs_fs_info,
        folio: *mut folio,
        type_: btrfs_folio_type,
    );
    pub fn btrfs_alloc_folio_state(
        fs_info: *const btrfs_fs_info,
        fsize: usize,
        type_: btrfs_folio_type,
        gfp: gfp_t,
    ) -> *mut btrfs_folio_state;
    pub fn kfree(ptr: *mut btrfs_folio_state);
    pub fn btrfs_folio_inc_eb_refs(fs_info: *const btrfs_fs_info, folio: *mut folio);
    pub fn btrfs_folio_dec_eb_refs(fs_info: *const btrfs_fs_info, folio: *mut folio);
    pub fn btrfs_folio_end_lock(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_folio_set_lock(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_folio_end_lock_bitmap(fs_info: *const btrfs_fs_info, folio: *mut folio, bitmap: *mut usize);
}

#[inline]
pub unsafe fn btrfs_free_folio_state(bfs: *mut btrfs_folio_state) {
    kfree(bfs);
}

// DECLARE_BTRFS_SUBPAGE_OPS(name) expands to the following declarations for
// each of uptodate, dirty, and writeback.
macro_rules! declare_btrfs_subpage_ops {
    ($name:ident) => {
        extern "C" {
            pub fn btrfs_subpage_set_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_subpage_clear_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_subpage_test_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
            pub fn btrfs_folio_set_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_folio_clear_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_folio_test_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
            pub fn btrfs_folio_clamp_set_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_folio_clamp_clear_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
            pub fn btrfs_folio_clamp_test_$name(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
            pub fn btrfs_meta_folio_set_$name(folio: *mut folio, eb: *const extent_buffer);
            pub fn btrfs_meta_folio_clear_$name(folio: *mut folio, eb: *const extent_buffer);
            pub fn btrfs_meta_folio_test_$name(folio: *mut folio, eb: *const extent_buffer) -> bool;
        }
    };
}

declare_btrfs_subpage_ops!(uptodate);
declare_btrfs_subpage_ops!(dirty);
declare_btrfs_subpage_ops!(writeback);

extern "C" {
    pub fn btrfs_subpage_clear_fixup(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_subpage_test_fixup(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
    pub fn btrfs_folio_test_fixup(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
    pub fn btrfs_folio_set_fixup_dirty(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_folio_clear_fixup(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_folio_clear_fixup_dirty(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_subpage_clear_and_test_dirty(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32) -> bool;
    pub fn btrfs_folio_assert_not_dirty(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
    pub fn btrfs_meta_folio_clear_and_test_dirty(folio: *mut folio, eb: *const extent_buffer) -> bool;
    pub fn btrfs_copy_subpage_dirty_bitmap(fs_info: *mut btrfs_fs_info, folio: *mut folio, dst: *mut usize);
    pub fn btrfs_subpage_dump_bitmap(fs_info: *const btrfs_fs_info, folio: *mut folio, start: u64, len: u32);
}

#[inline]
pub unsafe fn btrfs_folio_clamp_finish_io(
    fs_info: *mut btrfs_fs_info,
    locked_folio: *mut folio,
    start: u64,
    len: u32,
) {
    btrfs_folio_clamp_clear_dirty(fs_info, locked_folio, start, len);
    btrfs_folio_clamp_set_writeback(fs_info, locked_folio, start, len);
    btrfs_folio_clamp_clear_writeback(fs_info, locked_folio, start, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
