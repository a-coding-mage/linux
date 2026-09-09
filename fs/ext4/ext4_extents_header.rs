// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2003-2006, Cluster File Systems, Inc, info@clusterfs.com
 * Written by Alex Tomas <alex@clusterfs.com>
 */

// Translated from ext4_extents.h. The included ext4 definitions are supplied externally.

/* With AGGRESSIVE_TEST, EXTENTS_STATS, and CHECK_BINSEARCH defined in the C
 * header, their build-time intent is preserved here as marker constants. */
pub const AGGRESSIVE_TEST_: bool = true;
pub const EXTENTS_STATS__: bool = true;
pub const CHECK_BINSEARCH__: bool = true;

#[repr(C)]
pub struct ext4_extent_tail {
    pub et_checksum: __le32,
}

#[repr(C)]
pub struct ext4_extent {
    pub ee_block: __le32,
    pub ee_len: __le16,
    pub ee_start_hi: __le16,
    pub ee_start_lo: __le32,
}

#[repr(C)]
pub struct ext4_extent_idx {
    pub ei_block: __le32,
    pub ei_leaf_lo: __le32,
    pub ei_leaf_hi: __le16,
    pub ei_unused: __u16,
}

#[repr(C)]
pub struct ext4_extent_header {
    pub eh_magic: __le16,
    pub eh_entries: __le16,
    pub eh_max: __le16,
    pub eh_depth: __le16,
    pub eh_generation: __le32,
}

pub const EXT4_EXT_MAGIC: __le16 = cpu_to_le16(0xf30a);
pub const EXT4_MAX_EXTENT_DEPTH: i32 = 5;

#[inline]
pub unsafe fn EXT4_EXTENT_TAIL_OFFSET(hdr: *const ext4_extent_header) -> usize {
    core::mem::size_of::<ext4_extent_header>()
        + core::mem::size_of::<ext4_extent>() * le16_to_cpu((*hdr).eh_max) as usize
}

#[inline]
pub unsafe fn find_ext4_extent_tail(eh: *mut ext4_extent_header) -> *mut ext4_extent_tail {
    (eh as *mut u8).add(EXT4_EXTENT_TAIL_OFFSET(eh)) as *mut ext4_extent_tail
}

#[repr(C)]
pub struct ext4_ext_path {
    pub p_block: ext4_fsblk_t,
    pub p_depth: __u16,
    pub p_maxdepth: __u16,
    pub p_ext: *mut ext4_extent,
    pub p_idx: *mut ext4_extent_idx,
    pub p_hdr: *mut ext4_extent_header,
    pub p_bh: *mut buffer_head,
}

#[repr(i32)]
pub enum partial_cluster_state { initial, tofree, nofree }

#[repr(C)]
pub struct partial_cluster {
    pub pclu: ext4_fsblk_t,
    pub lblk: ext4_lblk_t,
    pub state: partial_cluster_state,
}

pub const EXT_INIT_MAX_LEN: usize = 1usize << 15;
pub const EXT_UNWRITTEN_MAX_LEN: usize = EXT_INIT_MAX_LEN - 1;

#[inline]
pub unsafe fn EXT_FIRST_EXTENT(hdr: *mut ext4_extent_header) -> *mut ext4_extent {
    (hdr as *mut u8).add(core::mem::size_of::<ext4_extent_header>()) as *mut ext4_extent
}
#[inline]
pub unsafe fn EXT_FIRST_INDEX(hdr: *mut ext4_extent_header) -> *mut ext4_extent_idx {
    (hdr as *mut u8).add(core::mem::size_of::<ext4_extent_header>()) as *mut ext4_extent_idx
}
#[inline]
pub unsafe fn EXT_HAS_FREE_INDEX(path: *mut ext4_ext_path) -> bool {
    le16_to_cpu((*(*path).p_hdr).eh_entries) < le16_to_cpu((*(*path).p_hdr).eh_max)
}
#[inline]
pub unsafe fn EXT_LAST_EXTENT(hdr: *mut ext4_extent_header) -> *mut ext4_extent {
    EXT_FIRST_EXTENT(hdr).add(le16_to_cpu((*hdr).eh_entries) as usize - 1)
}
#[inline]
pub unsafe fn EXT_LAST_INDEX(hdr: *mut ext4_extent_header) -> *mut ext4_extent_idx {
    EXT_FIRST_INDEX(hdr).add(le16_to_cpu((*hdr).eh_entries) as usize - 1)
}
#[inline]
pub unsafe fn EXT_MAX_EXTENT(hdr: *mut ext4_extent_header) -> *mut ext4_extent {
    if le16_to_cpu((*hdr).eh_max) != 0 { EXT_FIRST_EXTENT(hdr).add(le16_to_cpu((*hdr).eh_max) as usize - 1) } else { core::ptr::null_mut() }
}
#[inline]
pub unsafe fn EXT_MAX_INDEX(hdr: *mut ext4_extent_header) -> *mut ext4_extent_idx {
    if le16_to_cpu((*hdr).eh_max) != 0 { EXT_FIRST_INDEX(hdr).add(le16_to_cpu((*hdr).eh_max) as usize - 1) } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn ext_inode_hdr(inode: *mut inode) -> *mut ext4_extent_header { EXT4_I(inode).i_data.as_mut_ptr() as *mut ext4_extent_header }
#[inline]
pub unsafe fn ext_block_hdr(bh: *mut buffer_head) -> *mut ext4_extent_header { (*bh).b_data as *mut ext4_extent_header }
#[inline]
pub unsafe fn ext_depth(inode: *mut inode) -> u16 { le16_to_cpu((*ext_inode_hdr(inode)).eh_depth) }

#[inline]
pub unsafe fn ext4_ext_mark_unwritten(ext: *mut ext4_extent) {
    BUG_ON((le16_to_cpu((*ext).ee_len) & !(EXT_INIT_MAX_LEN as u16)) == 0);
    (*ext).ee_len |= cpu_to_le16(EXT_INIT_MAX_LEN as u16);
}
#[inline]
pub unsafe fn ext4_ext_is_unwritten(ext: *mut ext4_extent) -> i32 { (le16_to_cpu((*ext).ee_len) > EXT_INIT_MAX_LEN as u16) as i32 }
#[inline]
pub unsafe fn ext4_ext_get_actual_len(ext: *mut ext4_extent) -> i32 {
    if le16_to_cpu((*ext).ee_len) <= EXT_INIT_MAX_LEN as u16 { le16_to_cpu((*ext).ee_len) as i32 } else { (le16_to_cpu((*ext).ee_len) - EXT_INIT_MAX_LEN as u16) as i32 }
}
#[inline]
pub unsafe fn ext4_ext_mark_initialized(ext: *mut ext4_extent) { (*ext).ee_len = cpu_to_le16(ext4_ext_get_actual_len(ext) as u16); }

#[inline]
pub unsafe fn ext4_ext_pblock(ex: *mut ext4_extent) -> ext4_fsblk_t {
    let mut block = le32_to_cpu((*ex).ee_start_lo) as ext4_fsblk_t;
    block |= ((le16_to_cpu((*ex).ee_start_hi) as ext4_fsblk_t) << 31) << 1;
    block
}
#[inline]
pub unsafe fn ext4_idx_pblock(ix: *mut ext4_extent_idx) -> ext4_fsblk_t {
    let mut block = le32_to_cpu((*ix).ei_leaf_lo) as ext4_fsblk_t;
    block |= ((le16_to_cpu((*ix).ei_leaf_hi) as ext4_fsblk_t) << 31) << 1;
    block
}
#[inline]
pub unsafe fn ext4_ext_store_pblock(ex: *mut ext4_extent, pb: ext4_fsblk_t) {
    (*ex).ee_start_lo = cpu_to_le32((pb & 0xffffffff) as u32);
    (*ex).ee_start_hi = cpu_to_le16((((pb >> 31) >> 1) & 0xffff) as u16);
}
#[inline]
pub unsafe fn ext4_idx_store_pblock(ix: *mut ext4_extent_idx, pb: ext4_fsblk_t) {
    (*ix).ei_leaf_lo = cpu_to_le32((pb & 0xffffffff) as u32);
    (*ix).ei_leaf_hi = cpu_to_le16((((pb >> 31) >> 1) & 0xffff) as u16);
}

extern "C" {
    pub fn __ext4_ext_dirty(where_: *const i8, line: u32, handle: *mut handle_t, inode: *mut inode, path: *mut ext4_ext_path) -> i32;
    pub fn ext4_ext_zeroout(inode: *mut inode, ex: *mut ext4_extent) -> i32;
}

// Preserves the C conditional: IS_ENABLED(CONFIG_EXT4_KUNIT_TESTS).
#[cfg(feature = "CONFIG_EXT4_KUNIT_TESTS")]
extern "C" {
    pub fn ext4_ext_space_root_idx_test(inode: *mut inode, check: i32) -> i32;
    pub fn ext4_split_convert_extents_test(
        handle: *mut handle_t,
        inode: *mut inode,
        map: *mut ext4_map_blocks,
        path: *mut ext4_ext_path,
        flags: i32,
        allocated: *mut u32,
    ) -> *mut ext4_ext_path;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
