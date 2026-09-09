// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/ext4/extents_status.h
 *
 * Written by Yongqiang Yang <xiaoqiangnk@gmail.com>
 * Modified by
 *	Allison Henderson <achender@linux.vnet.ibm.com>
 *	Zheng Liu <wenqing.lz@taobao.com>
 */

/*
 * Turn on ES_DEBUG__ to get lots of info about extent status operations.
 * The C es_debug(fmt, ...) macro selects printk/no_printk at build time.
 */

/* With ES_AGGRESSIVE_TEST__ defined, extent-status caching is checked against
 * the old map_block result. */
pub const ES_AGGRESSIVE_TEST__: bool = true;

pub const ES_WRITTEN_B: u32 = 0;
pub const ES_UNWRITTEN_B: u32 = 1;
pub const ES_DELAYED_B: u32 = 2;
pub const ES_HOLE_B: u32 = 3;
pub const ES_REFERENCED_B: u32 = 4;
pub const ES_FLAGS: u32 = 5;

pub const ES_SHIFT: usize = core::mem::size_of::<ext4_fsblk_t>() * 8 - ES_FLAGS as usize;
pub const ES_MASK: ext4_fsblk_t = !0 << ES_SHIFT;

pub const EXTENT_STATUS_WRITTEN: u32 = 1 << ES_WRITTEN_B;
pub const EXTENT_STATUS_UNWRITTEN: u32 = 1 << ES_UNWRITTEN_B;
pub const EXTENT_STATUS_DELAYED: u32 = 1 << ES_DELAYED_B;
pub const EXTENT_STATUS_HOLE: u32 = 1 << ES_HOLE_B;
pub const EXTENT_STATUS_REFERENCED: u32 = 1 << ES_REFERENCED_B;

pub const ES_TYPE_MASK: ext4_fsblk_t =
    (EXTENT_STATUS_WRITTEN | EXTENT_STATUS_UNWRITTEN | EXTENT_STATUS_DELAYED |
        EXTENT_STATUS_HOLE) as ext4_fsblk_t;

#[inline]
pub const fn es_type_valid(type_: u32) -> bool {
    type_ != 0 && (type_ & (type_ - 1)) == 0
}

#[repr(C)]
pub struct ext4_sb_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ext4_extent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct extent_status {
    pub rb_node: rb_node,
    pub es_lblk: ext4_lblk_t, /* first logical block extent covers */
    pub es_len: ext4_lblk_t,  /* length of extent in block */
    pub es_pblk: ext4_fsblk_t, /* first physical block */
}

#[repr(C)]
pub struct ext4_es_tree {
    pub root: rb_root,
    pub cache_es: *mut extent_status, /* recently accessed extent */
}

#[repr(C)]
pub struct ext4_es_stats {
    pub es_stats_shrunk: c_ulong,
    pub es_stats_cache_hits: percpu_counter,
    pub es_stats_cache_misses: percpu_counter,
    pub es_stats_scan_time: u64,
    pub es_stats_max_scan_time: u64,
    pub es_stats_all_cnt: percpu_counter,
    pub es_stats_shk_cnt: percpu_counter,
}

#[repr(C)]
pub struct pending_reservation {
    pub rb_node: rb_node,
    pub lclu: ext4_lblk_t,
}

#[repr(C)]
pub struct ext4_pending_tree {
    pub root: rb_root,
}

unsafe extern "C" {
    pub fn ext4_init_es() -> c_int;
    pub fn ext4_exit_es();
    pub fn ext4_es_init_tree(tree: *mut ext4_es_tree);
    pub fn ext4_es_insert_extent(inode: *mut inode, lblk: ext4_lblk_t, len: ext4_lblk_t,
        pblk: ext4_fsblk_t, status: c_uint, delalloc_reserve_used: bool);
    pub fn ext4_es_cache_extent(inode: *mut inode, lblk: ext4_lblk_t, len: ext4_lblk_t,
        pblk: ext4_fsblk_t, status: c_uint);
    pub fn ext4_es_remove_extent(inode: *mut inode, lblk: ext4_lblk_t, len: ext4_lblk_t);
    pub fn ext4_es_find_extent_range(inode: *mut inode,
        match_fn: Option<unsafe extern "C" fn(*mut extent_status) -> c_int>,
        lblk: ext4_lblk_t, end: ext4_lblk_t, es: *mut extent_status);
    pub fn ext4_es_lookup_extent(inode: *mut inode, lblk: ext4_lblk_t,
        next_lblk: *mut ext4_lblk_t, es: *mut extent_status, pseq: *mut u64) -> c_int;
    pub fn ext4_es_scan_range(inode: *mut inode,
        matching_fn: Option<unsafe extern "C" fn(*mut extent_status) -> c_int>,
        lblk: ext4_lblk_t, end: ext4_lblk_t) -> bool;
    pub fn ext4_es_scan_clu(inode: *mut inode,
        matching_fn: Option<unsafe extern "C" fn(*mut extent_status) -> c_int>,
        lblk: ext4_lblk_t) -> bool;
    pub fn ext4_es_register_shrinker(sbi: *mut ext4_sb_info) -> c_int;
    pub fn ext4_es_unregister_shrinker(sbi: *mut ext4_sb_info);
}

#[inline]
pub unsafe fn ext4_es_status(es: *mut extent_status) -> c_uint {
    (*es).es_pblk >> ES_SHIFT
}
#[inline]
pub unsafe fn ext4_es_type(es: *mut extent_status) -> c_uint {
    ((unsafe { (*es).es_pblk } >> ES_SHIFT) & ES_TYPE_MASK) as c_uint
}
#[inline]
pub unsafe fn ext4_es_is_written(es: *mut extent_status) -> c_int { (ext4_es_type(es) & EXTENT_STATUS_WRITTEN != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_is_unwritten(es: *mut extent_status) -> c_int { (ext4_es_type(es) & EXTENT_STATUS_UNWRITTEN != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_is_delayed(es: *mut extent_status) -> c_int { (ext4_es_type(es) & EXTENT_STATUS_DELAYED != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_is_hole(es: *mut extent_status) -> c_int { (ext4_es_type(es) & EXTENT_STATUS_HOLE != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_is_mapped(es: *mut extent_status) -> c_int { (ext4_es_is_written(es) != 0 || ext4_es_is_unwritten(es) != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_set_referenced(es: *mut extent_status) { (*es).es_pblk |= (EXTENT_STATUS_REFERENCED as ext4_fsblk_t) << ES_SHIFT; }
#[inline]
pub unsafe fn ext4_es_clear_referenced(es: *mut extent_status) { (*es).es_pblk &= !((EXTENT_STATUS_REFERENCED as ext4_fsblk_t) << ES_SHIFT); }
#[inline]
pub unsafe fn ext4_es_is_referenced(es: *mut extent_status) -> c_int { (ext4_es_status(es) & EXTENT_STATUS_REFERENCED != 0) as c_int }
#[inline]
pub unsafe fn ext4_es_pblock(es: *mut extent_status) -> ext4_fsblk_t { (*es).es_pblk & !ES_MASK }
#[inline]
pub unsafe fn ext4_es_show_pblock(es: *mut extent_status) -> ext4_fsblk_t { let pblock = ext4_es_pblock(es); if pblock == !ES_MASK { 0 } else { pblock } }
#[inline]
pub unsafe fn ext4_es_store_pblock(es: *mut extent_status, pb: ext4_fsblk_t) { (*es).es_pblk = (pb & !ES_MASK) | ((*es).es_pblk & ES_MASK); }
#[inline]
pub unsafe fn ext4_es_store_pblock_status(es: *mut extent_status, pb: ext4_fsblk_t, status: c_uint) {
    (*es).es_pblk = (((status as ext4_fsblk_t) << ES_SHIFT) & ES_MASK) | (pb & !ES_MASK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
