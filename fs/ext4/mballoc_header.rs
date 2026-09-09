// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/ext4/mballoc.h
 *
 *  Written by: Alex Tomas <alex@clusterfs.com>
 */

// Kernel and ext4 dependencies are supplied by other translation units.

pub const EXT4_MB_HISTORY_ALLOC: u32 = 1;
pub const EXT4_MB_HISTORY_PREALLOC: u32 = 2;
pub const MB_DEFAULT_MAX_TO_SCAN: u32 = 200;
pub const MB_DEFAULT_MIN_TO_SCAN: u32 = 10;
pub const MB_DEFAULT_STATS: u32 = 0;
pub const MB_DEFAULT_STREAM_THRESHOLD: u32 = 16;
pub const MB_DEFAULT_ORDER2_REQS: u32 = 2;
pub const MB_DEFAULT_GROUP_PREALLOC: u32 = 512;
pub const MB_DEFAULT_LINEAR_LIMIT: u32 = 4;
pub const MB_DEFAULT_LINEAR_SCAN_THRESHOLD: u32 = 16;
pub const MB_DEFAULT_BEST_AVAIL_TRIM_ORDER: u32 = 3;

#[inline]
pub const unsafe fn mb_num_orders(sb: *const super_block) -> u32 {
    (*sb).s_blocksize_bits + 2
}

#[repr(C)]
pub struct ext4_free_data {
    pub efd_list: list_head,
    pub efd_node: rb_node,
    pub efd_group: ext4_group_t,
    pub efd_start_cluster: ext4_grpblk_t,
    pub efd_count: ext4_grpblk_t,
    pub efd_tid: tid_t,
}

#[repr(C)]
pub union ext4_prealloc_space_pa_node {
    pub inode_node: rb_node,
    pub lg_list: list_head,
}

#[repr(C)]
pub union ext4_prealloc_space_u {
    pub pa_tmp_list: list_head,
    pub pa_rcu: rcu_head,
}

#[repr(C)]
pub union ext4_prealloc_space_pa_node_lock {
    pub inode_lock: *mut rwlock_t,
    pub lg_lock: *mut spinlock_t,
}

#[repr(C)]
pub struct ext4_prealloc_space {
    pub pa_node: ext4_prealloc_space_pa_node,
    pub pa_group_list: list_head,
    pub u: ext4_prealloc_space_u,
    pub pa_lock: spinlock_t,
    pub pa_count: atomic_t,
    pub pa_deleted: c_uint,
    pub pa_pstart: ext4_fsblk_t,
    pub pa_lstart: ext4_lblk_t,
    pub pa_len: ext4_grpblk_t,
    pub pa_free: ext4_grpblk_t,
    pub pa_type: c_ushort,
    pub pa_node_lock: ext4_prealloc_space_pa_node_lock,
    pub pa_inode: *mut inode,
}

pub const MB_INODE_PA: u32 = 0;
pub const MB_GROUP_PA: u32 = 1;

#[repr(C)]
pub struct ext4_free_extent {
    pub fe_logical: ext4_lblk_t,
    pub fe_start: ext4_grpblk_t,
    pub fe_group: ext4_group_t,
    pub fe_len: ext4_grpblk_t,
}

pub const PREALLOC_TB_SIZE: usize = 10;

#[repr(C)]
pub struct ext4_locality_group {
    pub lg_mutex: mutex,
    pub lg_prealloc_list: [list_head; PREALLOC_TB_SIZE],
    pub lg_prealloc_lock: spinlock_t,
}

#[repr(C)]
pub struct ext4_allocation_context {
    pub ac_inode: *mut inode,
    pub ac_sb: *mut super_block,
    pub ac_o_ex: ext4_free_extent,
    pub ac_g_ex: ext4_free_extent,
    pub ac_b_ex: ext4_free_extent,
    pub ac_f_ex: ext4_free_extent,
    pub ac_orig_goal_len: ext4_grpblk_t,
    pub ac_prefetch_grp: ext4_group_t,
    pub ac_prefetch_ios: c_uint,
    pub ac_prefetch_nr: c_uint,
    pub ac_first_err: c_int,
    pub ac_flags: __u32,
    pub ac_groups_scanned: __u16,
    pub ac_found: __u16,
    pub ac_cX_found: [__u16; EXT4_MB_NUM_CRS],
    pub ac_tail: __u16,
    pub ac_buddy: __u16,
    pub ac_status: __u8,
    pub ac_criteria: __u8,
    pub ac_2order: __u8,
    pub ac_op: __u8,
    pub ac_e4b: *mut ext4_buddy,
    pub ac_bitmap_folio: *mut folio,
    pub ac_buddy_folio: *mut folio,
    pub ac_pa: *mut ext4_prealloc_space,
    pub ac_lg: *mut ext4_locality_group,
}

pub const AC_STATUS_CONTINUE: u32 = 1;
pub const AC_STATUS_FOUND: u32 = 2;
pub const AC_STATUS_BREAK: u32 = 3;

#[repr(C)]
pub struct ext4_buddy {
    pub bd_buddy_folio: *mut folio,
    pub bd_buddy: *mut c_void,
    pub bd_bitmap_folio: *mut folio,
    pub bd_bitmap: *mut c_void,
    pub bd_info: *mut ext4_group_info,
    pub bd_sb: *mut super_block,
    pub bd_blkbits: __u16,
    pub bd_group: ext4_group_t,
}

#[inline]
pub unsafe fn ext4_grp_offs_to_block(sb: *mut super_block, fex: *mut ext4_free_extent) -> ext4_fsblk_t {
    ext4_group_first_block_no(sb, (*fex).fe_group)
        + (((*fex).fe_start as ext4_fsblk_t) << EXT4_SB(sb).s_cluster_bits)
}

#[inline]
pub unsafe fn extent_logical_end(sbi: *mut ext4_sb_info, fex: *mut ext4_free_extent) -> loff_t {
    (*fex).fe_logical as loff_t + EXT4_C2B(sbi, (*fex).fe_len)
}

#[inline]
pub unsafe fn pa_logical_end(sbi: *mut ext4_sb_info, pa: *mut ext4_prealloc_space) -> loff_t {
    (*pa).pa_lstart as loff_t + EXT4_C2B(sbi, (*pa).pa_len)
}

pub type ext4_mballoc_query_range_fn = unsafe extern "C" fn(
    sb: *mut super_block, agno: ext4_group_t, start: ext4_grpblk_t,
    len: ext4_grpblk_t, priv_: *mut c_void,
) -> c_int;

unsafe extern "C" {
    pub fn ext4_mballoc_query_range(
        sb: *mut super_block, agno: ext4_group_t, start: ext4_grpblk_t,
        end: ext4_grpblk_t, meta_formatter: ext4_mballoc_query_range_fn,
        formatter: ext4_mballoc_query_range_fn, priv_: *mut c_void,
    ) -> c_int;
    pub fn ext4_mb_mark_context(
        handle: *mut handle_t, sb: *mut super_block, state: bool,
        group: ext4_group_t, blkoff: ext4_grpblk_t, len: ext4_grpblk_t,
        flags: c_int, ret_changed: *mut ext4_grpblk_t,
    ) -> c_int;
}

// Preserved from CONFIG_EXT4_KUNIT_TESTS conditional declarations.
#[cfg(CONFIG_EXT4_KUNIT_TESTS)]
unsafe extern "C" {
    pub fn mb_clear_bits_test(bm: *mut c_void, cur: c_int, len: c_int);
    pub fn ext4_mb_new_blocks_simple_test(ar: *mut ext4_allocation_request, errp: *mut c_int) -> ext4_fsblk_t;
    pub fn mb_find_next_zero_bit_test(addr: *mut c_void, max: c_int, start: c_int) -> c_int;
    pub fn mb_find_next_bit_test(addr: *mut c_void, max: c_int, start: c_int) -> c_int;
    pub fn mb_clear_bit_test(bit: c_int, addr: *mut c_void);
    pub fn mb_test_bit_test(bit: c_int, addr: *mut c_void) -> c_int;
    pub fn ext4_mb_mark_diskspace_used_test(ac: *mut ext4_allocation_context, handle: *mut handle_t) -> c_int;
    pub fn mb_mark_used_test(e4b: *mut ext4_buddy, ex: *mut ext4_free_extent) -> c_int;
    pub fn ext4_mb_generate_buddy_test(sb: *mut super_block, buddy: *mut c_void, bitmap: *mut c_void, group: ext4_group_t, grp: *mut ext4_group_info);
    pub fn ext4_mb_load_buddy_test(sb: *mut super_block, group: ext4_group_t, e4b: *mut ext4_buddy) -> c_int;
    pub fn ext4_mb_unload_buddy_test(e4b: *mut ext4_buddy);
    pub fn mb_free_blocks_test(inode: *mut inode, e4b: *mut ext4_buddy, first: c_int, count: c_int);
    pub fn ext4_free_blocks_simple_test(inode: *mut inode, block: ext4_fsblk_t, count: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
