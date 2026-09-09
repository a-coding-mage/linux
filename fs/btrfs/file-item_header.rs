/* SPDX-License-Identifier: GPL-2.0 */

// Translated from file-item.h. Dependencies are supplied by other translation units.

pub const BTRFS_FILE_EXTENT_INLINE_DATA_START: usize =
    std::mem::offset_of!(btrfs_file_extent_item, disk_bytenr);

#[inline]
pub unsafe fn BTRFS_MAX_INLINE_DATA_SIZE(info: *const btrfs_fs_info) -> u32 {
    BTRFS_MAX_ITEM_SIZE(info) - BTRFS_FILE_EXTENT_INLINE_DATA_START as u32
}

/*
 * Return the number of bytes used by the item on disk, minus the size of any
 * extent headers. If a file is compressed on disk, this is the compressed
 * size.
 */
#[inline]
pub unsafe fn btrfs_file_extent_inline_item_len(
    eb: *const extent_buffer,
    nr: i32,
) -> u32 {
    btrfs_item_size(eb, nr) - BTRFS_FILE_EXTENT_INLINE_DATA_START as u32
}

#[inline]
pub unsafe fn btrfs_file_extent_inline_start(
    e: *const btrfs_file_extent_item,
) -> usize {
    e as usize + BTRFS_FILE_EXTENT_INLINE_DATA_START
}

#[inline]
pub const fn btrfs_file_extent_calc_inline_size(datasize: u32) -> u32 {
    BTRFS_FILE_EXTENT_INLINE_DATA_START as u32 + datasize
}

extern "C" {
    pub fn btrfs_del_csums(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        bytenr: u64,
        len: u64,
    ) -> i32;
    pub fn btrfs_lookup_bio_sums(bbio: *mut btrfs_bio) -> i32;
    pub fn btrfs_insert_hole_extent(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        objectid: u64,
        pos: u64,
        num_bytes: u64,
    ) -> i32;
    pub fn btrfs_lookup_file_extent(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        objectid: u64,
        bytenr: u64,
        mod_: i32,
    ) -> i32;
    pub fn btrfs_insert_data_csums(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        sums: *mut btrfs_ordered_sum,
    ) -> i32;
    pub fn btrfs_csum_one_bio(bbio: *mut btrfs_bio, async_: bool) -> i32;
    pub fn btrfs_alloc_dummy_sum(bbio: *mut btrfs_bio) -> i32;
    pub fn btrfs_lookup_csums_range(
        root: *mut btrfs_root,
        start: u64,
        end: u64,
        list: *mut list_head,
        search_commit: i32,
        nowait: bool,
    ) -> i32;
    pub fn btrfs_lookup_csums_list(
        root: *mut btrfs_root,
        start: u64,
        end: u64,
        list: *mut list_head,
        nowait: bool,
    ) -> i32;
    pub fn btrfs_lookup_csums_bitmap(
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        start: u64,
        end: u64,
        csum_buf: *mut u8,
        csum_bitmap: *mut usize,
    ) -> i32;
    pub fn btrfs_extent_item_to_extent_map(
        inode: *mut btrfs_inode,
        path: *const btrfs_path,
        fi: *const btrfs_file_extent_item,
        em: *mut extent_map,
    );
    pub fn btrfs_inode_clear_file_extent_range(
        inode: *mut btrfs_inode,
        start: u64,
        len: u64,
    ) -> i32;
    pub fn btrfs_inode_set_file_extent_range(
        inode: *mut btrfs_inode,
        start: u64,
        len: u64,
    ) -> i32;
    pub fn btrfs_inode_safe_disk_i_size_write(
        inode: *mut btrfs_inode,
        new_i_size: u64,
    );
    pub fn btrfs_file_extent_end(path: *const btrfs_path) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
