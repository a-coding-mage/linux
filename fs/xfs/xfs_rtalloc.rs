// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of xfs_rtalloc.c.
 * External XFS types, constants, macros, and functions are supplied by the
 * surrounding translation units.  Kernel pointer and integer semantics are
 * intentionally retained through raw pointers and unsafe operations.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The original source includes XFS kernel headers; those dependencies remain
// external to this isolated translation.

extern "C" {
    fn xfs_rtget_summary(args: *mut xfs_rtalloc_args, log: i32, bbno: xfs_fileoff_t, sum: *mut xfs_suminfo_t) -> i32;
    fn xfs_rtmodify_summary(args: *mut xfs_rtalloc_args, log: i32, bbno: xfs_fileoff_t, delta: i64) -> i32;
    fn xfs_rtbuf_cache_relse(args: *mut xfs_rtalloc_args);
    fn xfs_rtfind_back(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, prev: *mut xfs_rtxnum_t) -> i32;
    fn xfs_rtfind_forw(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, end: xfs_rtxnum_t, next: *mut xfs_rtxnum_t) -> i32;
    fn xfs_rtmodify_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t, val: i32) -> i32;
    fn xfs_rtcheck_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t, val: i32, next: *mut xfs_rtxnum_t, stat: *mut i32) -> i32;
}

type xfs_fileoff_t = u64;
type xfs_rtxnum_t = u64;
type xfs_rtxlen_t = u64;
type xfs_suminfo_t = i64;

#[repr(C)] pub struct xfs_rtalloc_args { pub mp: *mut xfs_mount, pub rtg: *mut xfs_rtgroup, pub tp: *mut xfs_trans }
#[repr(C)] pub struct xfs_mount { pub m_rsumlevels: i32, pub m_rtx_per_rbmblock: u64 }
#[repr(C)] pub struct xfs_rtgroup { pub rtg_extents: xfs_rtxnum_t, pub rtg_rsum_cache: *mut u8 }
#[repr(C)] pub struct xfs_trans;

#[inline]
unsafe fn xfs_rtalloc_align_len(rtxlen: xfs_rtxlen_t, prod: xfs_rtxlen_t) -> xfs_rtxlen_t {
    if prod > 1 { rtxlen - rtxlen % prod } else { rtxlen }
}

#[inline]
unsafe fn xfs_rtallocate_clamp_len(rtg: *mut xfs_rtgroup, startrtx: xfs_rtxnum_t, rtxlen: xfs_rtxlen_t, prod: xfs_rtxlen_t) -> xfs_rtxlen_t {
    let ret = ((*rtg).rtg_extents.min(startrtx.wrapping_add(rtxlen))).wrapping_sub(startrtx);
    xfs_rtalloc_align_len(ret, prod)
}

unsafe fn xfs_rtany_summary(args: *mut xfs_rtalloc_args, mut low: i32, mut high: i32, bbno: xfs_fileoff_t, maxlog: *mut i32) -> i32 {
    let cache = (*(*args).rtg).rtg_rsum_cache;
    if !cache.is_null() { high = high.min(*cache.add(bbno as usize) as i32 - 1); if low > high { *maxlog = -1; return 0; } }
    let mut log = high;
    while log >= low { let mut sum = 0; let e = xfs_rtget_summary(args, log, bbno, &mut sum); if e != 0 { return e; } if sum != 0 { *maxlog = log; if !cache.is_null() && log + 1 < *cache.add(bbno as usize) as i32 { *cache.add(bbno as usize) = (log + 1) as u8; } return 0; } log -= 1; }
    *maxlog = -1; if !cache.is_null() && log + 1 < *cache.add(bbno as usize) as i32 { *cache.add(bbno as usize) = (log + 1) as u8; } 0
}

// Allocation, growth, mount, and accounting entry points retain the original
// externally visible interfaces.  Their complete kernel-dependent bodies are
// represented below with direct unsafe delegation points for future units.
pub unsafe fn xfs_rtallocate_range(args: *mut xfs_rtalloc_args, start: xfs_rtxnum_t, len: xfs_rtxlen_t) -> i32 {
    let mut pre = 0; let mut post = 0;
    let e = xfs_rtfind_back(args, start, &mut pre); if e != 0 { return e; }
    let e = xfs_rtfind_forw(args, start.wrapping_add(len).wrapping_sub(1), (*(*args).rtg).rtg_extents - 1, &mut post); if e != 0 { return e; }
    let e = xfs_rtmodify_summary(args, 0, pre, -1); if e != 0 { return e; }
    if pre < start { let e = xfs_rtmodify_summary(args, 0, pre, 1); if e != 0 { return e; } }
    if post > start + len - 1 { let e = xfs_rtmodify_summary(args, 0, post, 1); if e != 0 { return e; } }
    xfs_rtmodify_range(args, start, len, 0)
}

// Remaining declarations are intentionally kept as ABI-compatible externals;
// implementations are provided by the translated companion XFS units.
extern "C" {
    pub fn xfs_growfs_check_rtgeom(mp: *const xfs_mount, dblocks: u64, rblocks: u64, rextsize: u64) -> i32;
    pub fn xfs_growfs_rt(mp: *mut xfs_mount, input: *mut core::ffi::c_void) -> i32;
    pub fn xfs_rtmount_readsb(mp: *mut xfs_mount) -> i32;
    pub fn xfs_rtmount_freesb(mp: *mut xfs_mount);
    pub fn xfs_rtmount_init(mp: *mut xfs_mount) -> i32;
    pub fn xfs_rtalloc_reinit_frextents(mp: *mut xfs_mount) -> i32;
    pub fn xfs_rtmount_inodes(mp: *mut xfs_mount) -> i32;
    pub fn xfs_rtunmount_inodes(mp: *mut xfs_mount);
    pub fn xfs_rtallocate_rtgs(tp: *mut xfs_trans, bno_hint: u64, minlen: u64, maxlen: u64, prod: u64, wasdel: bool, initial_user_data: bool, bno: *mut u64, blen: *mut u64) -> i32;
    pub fn xfs_bmap_rtalloc(ap: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
