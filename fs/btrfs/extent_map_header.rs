/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other translation units:
// linux/compiler_types.h, linux/spinlock_types.h, linux/rbtree.h,
// linux/list.h, linux/refcount.h, and fs.h.

use core::ffi::c_long;

pub const EXTENT_MAP_LAST_BYTE: u64 = u64::MAX - 3;
pub const EXTENT_MAP_HOLE: u64 = u64::MAX - 2;
pub const EXTENT_MAP_INLINE: u64 = u64::MAX - 1;

/* bits for the extent_map::flags field */
pub const EXTENT_FLAG_PINNED: u32 = 1 << 0;
pub const EXTENT_FLAG_COMPRESS_ZLIB: u32 = 1 << 1;
pub const EXTENT_FLAG_COMPRESS_LZO: u32 = 1 << 2;
pub const EXTENT_FLAG_COMPRESS_ZSTD: u32 = 1 << 3;
pub const EXTENT_FLAG_PREALLOC: u32 = 1 << 4;
pub const EXTENT_FLAG_LOGGING: u32 = 1 << 5;
pub const EXTENT_FLAG_MERGED: u32 = 1 << 6;

/* Opaque types supplied by other translation units. */
#[repr(C)]
pub struct btrfs_inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

/*
 * This structure represents file extents and holes.
 *
 * Unlike on-disk file extent items, extent maps can be merged to save memory.
 * This means members only match file extent items before any merging.
 *
 * Keep this structure as compact as possible, as we can have really large
 * amounts of allocated extent maps at any time.
 */
#[repr(C)]
pub struct extent_map {
    pub rb_node: rb_node,
    /* All of these are in bytes. */
    pub start: u64,
    pub len: u64,
    pub disk_bytenr: u64,
    pub disk_num_bytes: u64,
    pub offset: u64,
    pub ram_bytes: u64,
    pub generation: u64,
    pub flags: u32,
    pub refs: refcount_t,
    pub list: list_head,
}

#[repr(C)]
pub struct extent_map_tree {
    pub root: rb_root,
    pub modified_extents: list_head,
    pub lock: rwlock_t,
}

pub unsafe fn btrfs_extent_map_set_compression(
    em: *mut extent_map,
    type_: btrfs_compression_type,
) {
    if type_ == BTRFS_COMPRESS_ZLIB {
        (*em).flags |= EXTENT_FLAG_COMPRESS_ZLIB;
    } else if type_ == BTRFS_COMPRESS_LZO {
        (*em).flags |= EXTENT_FLAG_COMPRESS_LZO;
    } else if type_ == BTRFS_COMPRESS_ZSTD {
        (*em).flags |= EXTENT_FLAG_COMPRESS_ZSTD;
    }
}

pub unsafe fn btrfs_extent_map_compression(
    em: *const extent_map,
) -> btrfs_compression_type {
    if (*em).flags & EXTENT_FLAG_COMPRESS_ZLIB != 0 {
        return BTRFS_COMPRESS_ZLIB;
    }
    if (*em).flags & EXTENT_FLAG_COMPRESS_LZO != 0 {
        return BTRFS_COMPRESS_LZO;
    }
    if (*em).flags & EXTENT_FLAG_COMPRESS_ZSTD != 0 {
        return BTRFS_COMPRESS_ZSTD;
    }
    BTRFS_COMPRESS_NONE
}

/* More efficient way to determine if extent is compressed. */
pub unsafe fn btrfs_extent_map_is_compressed(em: *const extent_map) -> bool {
    (*em).flags
        & (EXTENT_FLAG_COMPRESS_ZLIB | EXTENT_FLAG_COMPRESS_LZO | EXTENT_FLAG_COMPRESS_ZSTD)
        != 0
}

pub unsafe fn btrfs_extent_map_in_tree(em: *const extent_map) -> i32 {
    (!RB_EMPTY_NODE(&(*em).rb_node)) as i32
}

pub unsafe fn btrfs_extent_map_block_start(em: *const extent_map) -> u64 {
    if (*em).disk_bytenr < EXTENT_MAP_LAST_BYTE {
        if btrfs_extent_map_is_compressed(em) {
            return (*em).disk_bytenr;
        }
        return (*em).disk_bytenr.wrapping_add((*em).offset);
    }
    (*em).disk_bytenr
}

pub unsafe fn btrfs_extent_map_end(em: *const extent_map) -> u64 {
    let end = (*em).start.wrapping_add((*em).len);
    if end < (*em).start {
        return u64::MAX;
    }
    end
}

extern "C" {
    pub fn btrfs_extent_map_tree_init(tree: *mut extent_map_tree);
    pub fn btrfs_lookup_extent_mapping(tree: *mut extent_map_tree, start: u64, len: u64)
        -> *mut extent_map;
    pub fn btrfs_remove_extent_mapping(inode: *mut btrfs_inode, em: *mut extent_map);
    pub fn btrfs_split_extent_map(
        inode: *mut btrfs_inode, start: u64, len: u64, pre: u64, new_logical: u64,
    ) -> i32;
    pub fn btrfs_alloc_extent_map() -> *mut extent_map;
    pub fn btrfs_free_extent_map(em: *mut extent_map);
    pub fn btrfs_extent_map_init() -> i32;
    pub fn btrfs_extent_map_exit();
    pub fn btrfs_unpin_extent_cache(inode: *mut btrfs_inode, start: u64, len: u64, gen: u64) -> i32;
    pub fn btrfs_clear_em_logging(inode: *mut btrfs_inode, em: *mut extent_map);
    pub fn btrfs_search_extent_mapping(tree: *mut extent_map_tree, start: u64, len: u64)
        -> *mut extent_map;
    pub fn btrfs_add_extent_mapping(
        inode: *mut btrfs_inode, em_in: *mut *mut extent_map, start: u64, len: u64,
    ) -> i32;
    pub fn btrfs_drop_extent_map_range(
        inode: *mut btrfs_inode, start: u64, end: u64, skip_pinned: bool,
    );
    pub fn btrfs_replace_extent_map_range(
        inode: *mut btrfs_inode, new_em: *mut extent_map, modified: bool,
    ) -> i32;
    pub fn btrfs_free_extent_maps(fs_info: *mut btrfs_fs_info, nr_to_scan: c_long);
    pub fn btrfs_init_extent_map_shrinker_work(fs_info: *mut btrfs_fs_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
