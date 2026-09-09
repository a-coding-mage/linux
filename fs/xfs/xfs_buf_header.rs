// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Linux dependency declarations are supplied by other translated units.

extern "C" {
    pub static mut xfs_buf_cache: *mut kmem_cache;
}

#[repr(C)]
pub struct xfs_buf;

pub const XFS_BUF_DADDR_MAX: xfs_daddr_t = i64::MAX as xfs_daddr_t;
pub const XFS_BUF_DADDR_NULL: xfs_daddr_t = -1i64 as xfs_daddr_t;

pub const XBF_READ: u32 = 1u32 << 0;
pub const XBF_WRITE: u32 = 1u32 << 1;
pub const XBF_READ_AHEAD: u32 = 1u32 << 2;
pub const XBF_ASYNC: u32 = 1u32 << 4;
pub const XBF_DONE: u32 = 1u32 << 5;
pub const XBF_STALE: u32 = 1u32 << 6;
pub const XBF_WRITE_FAIL: u32 = 1u32 << 7;
pub const _XBF_KMEM: u32 = 1u32 << 21;
pub const _XBF_DELWRI_Q: u32 = 1u32 << 22;
pub const XBF_LIVESCAN: u32 = 1u32 << 28;
pub const XBF_INCORE: u32 = 1u32 << 29;
pub const XBF_TRYLOCK: u32 = 1u32 << 30;

pub type xfs_buf_flags_t = u32;

#[repr(C)]
pub struct xfs_buftarg {
    pub bt_dev: dev_t,
    pub bt_bdev: *mut block_device,
    pub bt_daxdev: *mut dax_device,
    pub bt_file: *mut file,
    pub bt_dax_part_off: u64,
    pub bt_mount: *mut xfs_mount,
    pub bt_meta_sectorsize: c_uint,
    pub bt_meta_sectormask: usize,
    pub bt_logical_sectorsize: usize,
    pub bt_logical_sectormask: usize,
    pub bt_nr_sectors: xfs_daddr_t,
    pub bt_shrinker: *mut shrinker,
    pub bt_lru: list_lru,
    pub bt_readahead_count: percpu_counter,
    pub bt_ioerror_rl: ratelimit_state,
    pub bt_awu_min: c_uint,
    pub bt_awu_max: c_uint,
    pub bt_hash: rhashtable,
}

#[repr(C)]
pub struct xfs_buf_map {
    pub bm_bn: xfs_daddr_t,
    pub bm_len: c_int,
    pub bm_flags: c_uint,
}

pub const XBM_LIVESCAN: u32 = 1u32 << 0;

#[repr(C)]
pub union xfs_buf_ops_magic {
    pub magic: [__be32; 2],
    pub magic16: [__be16; 2],
}

#[repr(C)]
pub struct xfs_buf_ops {
    pub name: *mut c_char,
    pub magic_u: xfs_buf_ops_magic,
    pub verify_read: Option<unsafe extern "C" fn(*mut xfs_buf)>,
    pub verify_write: Option<unsafe extern "C" fn(*mut xfs_buf)>,
    pub verify_struct: Option<unsafe extern "C" fn(*mut xfs_buf) -> xfs_failaddr_t>,
}

#[repr(C)]
pub struct xfs_buf {
    pub b_rhash_head: rhash_head,
    pub b_rhash_key: xfs_daddr_t,
    pub b_length: c_int,
    pub b_lockref: lockref,
    pub b_lru_ref: atomic_t,
    pub b_flags: xfs_buf_flags_t,
    pub b_sema: semaphore,
    pub b_lru: list_head,
    pub b_waiters: wait_queue_head_t,
    pub b_list: list_head,
    pub b_pag: *mut xfs_perag,
    pub b_mount: *mut xfs_mount,
    pub b_target: *mut xfs_buftarg,
    pub b_addr: *mut c_void,
    pub b_ioend_work: work_struct,
    pub b_iowait: completion,
    pub b_log_item: *mut xfs_buf_log_item,
    pub b_li_list: list_head,
    pub b_transp: *mut xfs_trans,
    pub b_maps: *mut xfs_buf_map,
    pub __b_map: xfs_buf_map,
    pub b_map_count: c_int,
    pub b_pin_count: atomic_t,
    pub b_error: c_int,
    pub b_iodone: Option<unsafe extern "C" fn(*mut xfs_buf)>,
    pub b_retries: c_int,
    pub b_first_retry_time: c_ulong,
    pub b_last_error: c_int,
    pub b_ops: *const xfs_buf_ops,
    pub b_rcu: rcu_head,
}

extern "C" {
    pub fn xfs_buf_get_map(target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf) -> c_int;
    pub fn xfs_buf_read_map(target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops, fa: xfs_failaddr_t) -> c_int;
    pub fn xfs_buf_readahead_map(target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: c_int, ops: *const xfs_buf_ops);
    pub fn xfs_buf_get_uncached(target: *mut xfs_buftarg, numblks: usize, bpp: *mut *mut xfs_buf) -> c_int;
    pub fn xfs_buf_read_uncached(target: *mut xfs_buftarg, daddr: xfs_daddr_t, numblks: usize, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> c_int;
    pub fn _xfs_buf_read(bp: *mut xfs_buf) -> c_int;
    pub fn xfs_buf_hold(bp: *mut xfs_buf);
    pub fn xfs_buf_rele(bp: *mut xfs_buf);
    pub fn xfs_buf_trylock(bp: *mut xfs_buf) -> c_int;
    pub fn xfs_buf_lock(bp: *mut xfs_buf);
    pub fn xfs_buf_unlock(bp: *mut xfs_buf);
    pub fn xfs_bwrite(bp: *mut xfs_buf) -> c_int;
    pub fn __xfs_buf_ioerror(bp: *mut xfs_buf, error: c_int, failaddr: xfs_failaddr_t);
    pub fn xfs_buf_ioerror_alert(bp: *mut xfs_buf, fa: xfs_failaddr_t);
    pub fn xfs_buf_fail(bp: *mut xfs_buf);
    pub fn __xfs_buf_mark_corrupt(bp: *mut xfs_buf, fa: xfs_failaddr_t);
    pub fn xfs_buf_set_uptodate(bp: *mut xfs_buf);
    pub fn xfs_buf_stale(bp: *mut xfs_buf);
    pub fn xfs_buf_clear_stale(bp: *mut xfs_buf);
    pub fn xfs_buf_delwri_cancel(head: *mut list_head);
    pub fn xfs_buf_delwri_queue(bp: *mut xfs_buf, head: *mut list_head) -> bool;
    pub fn xfs_buf_delwri_queue_here(bp: *mut xfs_buf, bl: *mut list_head);
    pub fn xfs_buf_delwri_submit(head: *mut list_head) -> c_int;
    pub fn xfs_buf_delwri_submit_nowait(head: *mut list_head) -> c_int;
    pub fn xfs_buf_set_ref(bp: *mut xfs_buf, lru_ref: c_int);
    pub fn xfs_verify_cksum(addr: *mut c_void, len: usize, offset: c_ulong) -> c_int;
    pub fn xfs_update_cksum(addr: *mut c_void, len: usize, offset: c_ulong);
    pub fn xfs_alloc_buftarg(mp: *mut xfs_mount, bdev_file: *mut file) -> *mut xfs_buftarg;
    pub fn xfs_free_buftarg(btp: *mut xfs_buftarg);
    pub fn xfs_buftarg_wait(btp: *mut xfs_buftarg);
    pub fn xfs_buftarg_drain(btp: *mut xfs_buftarg);
    pub fn xfs_configure_buftarg(btp: *mut xfs_buftarg, sectorsize: c_uint, nr_blocks: xfs_fsblock_t) -> c_int;
    pub fn xfs_verify_magic(bp: *mut xfs_buf, dmagic: __be32) -> bool;
    pub fn xfs_verify_magic16(bp: *mut xfs_buf, dmagic: __be16) -> bool;
    pub fn xfs_init_buftarg(btp: *mut xfs_buftarg, logical_sectorsize: usize, descr: *const c_char) -> c_int;
    pub fn xfs_destroy_buftarg(btp: *mut xfs_buftarg);
}

pub const XBF_FLAGS: &[(u32, &str)] = &[
    (XBF_READ, "READ"), (XBF_WRITE, "WRITE"), (XBF_READ_AHEAD, "READ_AHEAD"),
    (XBF_ASYNC, "ASYNC"), (XBF_DONE, "DONE"), (XBF_STALE, "STALE"),
    (XBF_WRITE_FAIL, "WRITE_FAIL"), (_XBF_KMEM, "KMEM"), (_XBF_DELWRI_Q, "DELWRI_Q"),
    (XBF_LIVESCAN, "LIVESCAN"), (XBF_INCORE, "INCORE"), (XBF_TRYLOCK, "TRYLOCK"),
];

#[inline]
pub unsafe fn xfs_buf_incore(target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: usize, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf) -> c_int {
    let mut map = xfs_buf_map { bm_bn: blkno, bm_len: numblks as c_int, bm_flags: 0 };
    xfs_buf_get_map(target, &mut map, 1, XBF_INCORE | flags, bpp)
}

#[inline]
pub unsafe fn xfs_buf_get(target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: usize, bpp: *mut *mut xfs_buf) -> c_int {
    let mut map = xfs_buf_map { bm_bn: blkno, bm_len: numblks as c_int, bm_flags: 0 };
    xfs_buf_get_map(target, &mut map, 1, 0, bpp)
}

#[inline]
pub unsafe fn xfs_buf_read(target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: usize, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> c_int {
    let mut map = xfs_buf_map { bm_bn: blkno, bm_len: numblks as c_int, bm_flags: 0 };
    // __builtin_return_address(0) has no direct stable Rust equivalent.
    xfs_buf_read_map(target, &mut map, 1, flags, bpp, ops, core::ptr::null_mut())
}

#[inline]
pub unsafe fn xfs_buf_readahead(target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: usize, ops: *const xfs_buf_ops) {
    let mut map = xfs_buf_map { bm_bn: blkno, bm_len: numblks as c_int, bm_flags: 0 };
    xfs_buf_readahead_map(target, &mut map, 1, ops);
}

#[inline]
pub unsafe fn xfs_buf_relse(bp: *mut xfs_buf) {
    xfs_buf_unlock(bp);
    xfs_buf_rele(bp);
}

#[inline]
pub unsafe fn xfs_buf_offset(bp: *mut xfs_buf, offset: usize) -> *mut c_void {
    ((*bp).b_addr as *mut u8).add(offset) as *mut c_void
}

#[inline]
pub unsafe fn xfs_buf_zero(bp: *mut xfs_buf, boff: usize, bsize: usize) {
    core::ptr::write_bytes(((*bp).b_addr as *mut u8).add(boff), 0, bsize);
}

#[inline]
pub unsafe fn xfs_buf_daddr(bp: *mut xfs_buf) -> xfs_daddr_t { (*(*bp).b_maps).bm_bn }

#[inline]
pub unsafe fn xfs_buf_oneshot(bp: *mut xfs_buf) {
    if !list_empty(&mut (*bp).b_lru) || atomic_read(&mut (*bp).b_lru_ref) > 1 { return; }
    atomic_set(&mut (*bp).b_lru_ref, 0);
}

#[inline]
pub unsafe fn xfs_buf_ispinned(bp: *mut xfs_buf) -> c_int { atomic_read(&mut (*bp).b_pin_count) }

// xfs_buf_islocked, xfs_buf_ioerror, xfs_buf_mark_corrupt, and xfs_readonly_buftarg
// are macro interfaces in C and depend on declarations supplied by other units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
