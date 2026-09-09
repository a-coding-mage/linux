/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of xfs_btree.h. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::ManuallyDrop;

pub enum xfs_buf {}
pub enum xfs_inode {}
pub enum xfs_mount {}
pub enum xfs_trans {}
pub enum xfs_ifork {}
pub enum xfs_perag {}
pub enum xfs_btree_block {}
pub enum xfs_btree_cur {}
pub enum xfs_buf_ops {}
pub enum kmem_cache {}
pub enum xfs_group {}
pub enum xbtree_ifakeroot {}
pub enum xbtree_afakeroot {}
pub enum xfbtree {}
pub enum list_head {}

pub type __be32 = u32;
pub type __be64 = u64;
pub type __u16 = u16;
pub type __u64 = u64;
pub type xfs_lookup_t = c_int;
pub type xfs_extlen_t = u64;
pub type xfs_agblock_t = u64;
pub type xfs_fileoff_t = u64;
pub type xfs_filblks_t = u64;
pub type xfs_failaddr_t = *mut c_void;

extern "C" {
    pub fn be16_to_cpu(v: u16) -> u16;
    pub fn cpu_to_be16(v: u16) -> u16;
    pub fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: c_uint) -> *mut xfs_btree_cur;
}

#[repr(C)]
pub union xfs_btree_ptr { pub s: __be32, pub l: __be64 }

#[repr(C)]
pub union xfs_btree_key {
    pub bmbt: ManuallyDrop<xfs_bmbt_key>, pub bmbr: ManuallyDrop<xfs_bmdr_key_t>,
    pub alloc: ManuallyDrop<xfs_alloc_key_t>, pub inobt: ManuallyDrop<xfs_inobt_key>,
    pub rmap: ManuallyDrop<xfs_rmap_key>, pub __rmap_bigkey: ManuallyDrop<[xfs_rmap_key; 2]>,
    pub refc: ManuallyDrop<xfs_refcount_key>,
}
#[repr(C)]
pub union xfs_btree_rec {
    pub bmbt: ManuallyDrop<xfs_bmbt_rec>, pub bmbr: ManuallyDrop<xfs_bmdr_rec_t>,
    pub alloc: ManuallyDrop<xfs_alloc_rec>, pub inobt: ManuallyDrop<xfs_inobt_rec>,
    pub rmap: ManuallyDrop<xfs_rmap_rec>, pub refc: ManuallyDrop<xfs_refcount_rec>,
}

pub enum xfs_bmbt_key {} pub enum xfs_bmdr_key_t {} pub enum xfs_alloc_key_t {}
pub enum xfs_inobt_key {} pub enum xfs_rmap_key {} pub enum xfs_refcount_key {}
pub enum xfs_bmbt_rec {} pub enum xfs_bmdr_rec_t {} pub enum xfs_alloc_rec {}
pub enum xfs_inobt_rec {} pub enum xfs_rmap_rec {} pub enum xfs_refcount_rec {}
pub enum xfs_alloc_rec_incore {} pub enum xfs_bmbt_irec {}
pub enum xfs_inobt_rec_incore {} pub enum xfs_rmap_irec {} pub enum xfs_refcount_irec {}
pub enum xfs_btree_ops {}
pub enum xfs_btree_irec {}
pub enum enum_xbtree_recpacking {}
pub type xbtree_recpacking = enum_xbtree_recpacking;

pub const XFS_LOOKUP_EQ: xfs_lookup_t = 0; pub const XFS_LOOKUP_LE: xfs_lookup_t = 1; pub const XFS_LOOKUP_GE: xfs_lookup_t = 2;
pub const XFS_BB_MAGIC: u32 = 1 << 0; pub const XFS_BB_LEVEL: u32 = 1 << 1; pub const XFS_BB_NUMRECS: u32 = 1 << 2;
pub const XFS_BB_LEFTSIB: u32 = 1 << 3; pub const XFS_BB_RIGHTSIB: u32 = 1 << 4; pub const XFS_BB_BLKNO: u32 = 1 << 5;
pub const XFS_BB_LSN: u32 = 1 << 6; pub const XFS_BB_UUID: u32 = 1 << 7; pub const XFS_BB_OWNER: u32 = 1 << 8;
pub const XFS_BB_NUM_BITS: u32 = 5; pub const XFS_BB_ALL_BITS: u32 = (1 << XFS_BB_NUM_BITS) - 1;
pub const XFS_BB_NUM_BITS_CRC: u32 = 9; pub const XFS_BB_ALL_BITS_CRC: u32 = (1 << XFS_BB_NUM_BITS_CRC) - 1;

#[repr(C)] pub enum xbtree_key_contig { XBTREE_KEY_GAP = 0, XBTREE_KEY_CONTIGUOUS, XBTREE_KEY_OVERLAP }
#[inline] pub fn xbtree_key_contig(x: u64, y: u64) -> xbtree_key_contig { let x = x.wrapping_add(1); if x < y { xbtree_key_contig::XBTREE_KEY_GAP } else if x == y { xbtree_key_contig::XBTREE_KEY_CONTIGUOUS } else { xbtree_key_contig::XBTREE_KEY_OVERLAP } }
pub const XFS_BTREE_LONG_PTR_LEN: usize = 8; pub const XFS_BTREE_SHORT_PTR_LEN: usize = 4;
#[repr(C)] pub enum xfs_btree_type { XFS_BTREE_TYPE_AG, XFS_BTREE_TYPE_INODE, XFS_BTREE_TYPE_MEM }

#[repr(C)] pub struct xfs_btree_level { pub bp: *mut xfs_buf, pub ptr: u16, pub ra: u16 }
#[repr(C)] pub union xfs_btree_irec { pub a: ManuallyDrop<xfs_alloc_rec_incore>, pub b: ManuallyDrop<xfs_bmbt_irec>, pub i: ManuallyDrop<xfs_inobt_rec_incore>, pub r: ManuallyDrop<xfs_rmap_irec>, pub rc: ManuallyDrop<xfs_refcount_irec> }

pub const XFS_BTREE_STAGING: u32 = 1 << 0; pub const XFS_BTREE_BMBT_WASDEL: u32 = 1 << 1;
pub const XFS_BTREE_BMBT_INVALID_OWNER: u32 = 1 << 2; pub const XFS_BTREE_ALLOCBT_ACTIVE: u32 = 1 << 3;
pub const XFS_BTREE_NOERROR: c_int = 0; pub const XFS_BTREE_ERROR: c_int = 1;
pub const XFS_BTREE_VISIT_RECORDS: u32 = 1 << 0; pub const XFS_BTREE_VISIT_LEAVES: u32 = 1 << 1; pub const XFS_BTREE_VISIT_ALL: u32 = 3;

extern "C" {
    pub fn xfs_btree_magic(mp: *mut xfs_mount, ops: *const xfs_btree_ops) -> u32;
    pub fn __xfs_btree_check_block(cur: *mut xfs_btree_cur, block: *mut xfs_btree_block, level: c_int, bp: *mut xfs_buf) -> xfs_failaddr_t;
    pub fn __xfs_btree_check_ptr(cur: *mut xfs_btree_cur, ptr: *const xfs_btree_ptr, index: c_int, level: c_int) -> c_int;
    pub fn xfs_btree_del_cursor(cur: *mut xfs_btree_cur, error: c_int);
    pub fn xfs_btree_get_block(cur: *mut xfs_btree_cur, level: c_int, bpp: *mut *mut xfs_buf) -> *mut xfs_btree_block;
    pub fn xfs_btree_masked_keycmp_lt(cur: *mut xfs_btree_cur, key1: *const xfs_btree_key, key2: *const xfs_btree_key, mask: *const xfs_btree_key) -> bool;
    pub fn xfs_btree_check_block(cur: *mut xfs_btree_cur, block: *mut xfs_btree_block, level: c_int, bp: *mut xfs_buf) -> c_int;
    pub fn xfs_btree_dup_cursor(cur: *mut xfs_btree_cur, ncur: *mut *mut xfs_btree_cur) -> c_int;
    pub fn xfs_btree_offsets(fields: u32, offsets: *const i16, nbits: c_int, first: *mut c_int, last: *mut c_int);
    pub fn xfs_btree_init_buf(mp: *mut xfs_mount, bp: *mut xfs_buf, ops: *const xfs_btree_ops, level: u16, numrecs: u16, owner: u64);
    pub fn xfs_btree_init_block(mp: *mut xfs_mount, buf: *mut xfs_btree_block, ops: *const xfs_btree_ops, level: u16, numrecs: u16, owner: u64);
    pub fn xfs_btree_increment(cur: *mut xfs_btree_cur, level: c_int, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_decrement(cur: *mut xfs_btree_cur, level: c_int, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_lookup(cur: *mut xfs_btree_cur, cmp: xfs_lookup_t, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_update(cur: *mut xfs_btree_cur, rec: *mut xfs_btree_rec) -> c_int;
    pub fn xfs_btree_new_iroot(cur: *mut xfs_btree_cur, stat: *mut c_int, error: *mut c_int) -> c_int;
    pub fn xfs_btree_insert(cur: *mut xfs_btree_cur, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_delete(cur: *mut xfs_btree_cur, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_get_rec(cur: *mut xfs_btree_cur, rec: *mut *mut xfs_btree_rec, stat: *mut c_int) -> c_int;
    pub fn xfs_btree_change_owner(cur: *mut xfs_btree_cur, owner: u64, list: *mut list_head) -> c_int;
    pub fn xfs_btree_fsblock_calc_crc(bp: *mut xfs_buf); pub fn xfs_btree_fsblock_verify_crc(bp: *mut xfs_buf) -> bool;
    pub fn xfs_btree_agblock_calc_crc(bp: *mut xfs_buf); pub fn xfs_btree_agblock_verify_crc(bp: *mut xfs_buf) -> bool;
    pub fn xfs_btree_log_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf, fields: u32);
    pub fn xfs_btree_log_recs(cur: *mut xfs_btree_cur, bp: *mut xfs_buf, first: c_int, last: c_int);
    pub fn xfs_btree_query_range(cur: *mut xfs_btree_cur, low: *const xfs_btree_irec, high: *const xfs_btree_irec, f: xfs_btree_query_range_fn, priv_: *mut c_void) -> c_int;
    pub fn xfs_btree_query_all(cur: *mut xfs_btree_cur, f: xfs_btree_query_range_fn, priv_: *mut c_void) -> c_int;
    pub fn xfs_btree_visit_blocks(cur: *mut xfs_btree_cur, f: xfs_btree_visit_blocks_fn, flags: u32, data: *mut c_void) -> c_int;
    pub fn xfs_btree_count_blocks(cur: *mut xfs_btree_cur, blocks: *mut xfs_filblks_t) -> c_int;
    pub fn xfs_btree_rec_addr(cur: *mut xfs_btree_cur, n: c_int, block: *mut xfs_btree_block) -> *mut xfs_btree_rec;
    pub fn xfs_btree_key_addr(cur: *mut xfs_btree_cur, n: c_int, block: *mut xfs_btree_block) -> *mut xfs_btree_key;
    pub fn xfs_btree_high_key_addr(cur: *mut xfs_btree_cur, n: c_int, block: *mut xfs_btree_block) -> *mut xfs_btree_key;
    pub fn xfs_btree_ptr_addr(cur: *mut xfs_btree_cur, n: c_int, block: *mut xfs_btree_block) -> *mut xfs_btree_ptr;
    pub fn xfs_btree_ptr_is_null(cur: *mut xfs_btree_cur, ptr: *const xfs_btree_ptr) -> bool;
    pub fn xfs_btree_set_ptr_null(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr);
    pub fn xfs_btree_goto_left_edge(cur: *mut xfs_btree_cur) -> c_int;
    pub fn xfs_btree_init_cur_caches() -> c_int; pub fn xfs_btree_destroy_cur_caches();
    pub fn xfs_btree_agblock_v5hdr_verify(bp: *mut xfs_buf) -> xfs_failaddr_t;
    pub fn xfs_btree_agblock_verify(bp: *mut xfs_buf, max_recs: c_uint) -> xfs_failaddr_t;
    pub fn xfs_btree_fsblock_v5hdr_verify(bp: *mut xfs_buf, owner: u64) -> xfs_failaddr_t;
    pub fn xfs_btree_fsblock_verify(bp: *mut xfs_buf, max_recs: c_uint) -> xfs_failaddr_t;
    pub fn xfs_btree_memblock_verify(bp: *mut xfs_buf, max_recs: c_uint) -> xfs_failaddr_t;
    pub fn xfs_btree_compute_maxlevels(limits: *const c_uint, records: c_ulonglong) -> c_uint;
    pub fn xfs_btree_calc_size(limits: *const c_uint, records: c_ulonglong) -> c_ulonglong;
    pub fn xfs_btree_space_to_height(limits: *const c_uint, blocks: c_ulonglong) -> c_uint;
}

pub type xfs_btree_query_range_fn = Option<unsafe extern "C" fn(*mut xfs_btree_cur, *const xfs_btree_rec, *mut c_void) -> c_int>;
pub type xfs_btree_visit_blocks_fn = Option<unsafe extern "C" fn(*mut xfs_btree_cur, c_int, *mut c_void) -> c_int>;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
