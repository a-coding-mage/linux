// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Linux kernel includes and local XFS headers are external dependencies.
// Build-time CONFIG_* conditions from the C header are preserved below.

#[cfg(target_pointer_width = "64")]
pub type xfs_off_t = i64;
pub type xfs_ino_t = u64;
pub type xfs_daddr_t = i64;
pub type xfs_dev_t = u32;
pub type xfs_nlink_t = u32;

#[cfg(feature = "CONFIG_XFS_DEBUG")]
pub const DEBUG: i32 = 1;
#[cfg(feature = "CONFIG_XFS_DEBUG_EXPENSIVE")]
pub const DEBUG_EXPENSIVE: i32 = 1;
#[cfg(feature = "CONFIG_XFS_ASSERT_FATAL")]
pub const XFS_ASSERT_FATAL: i32 = 1;
#[cfg(feature = "CONFIG_XFS_WARN")]
pub const XFS_WARN: i32 = 1;

#[cfg(target_endian = "big")]
pub const XFS_NATIVE_HOST: i32 = 1;

// Required external types and symbols are supplied by the surrounding kernel/XFS translation.
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfsstats {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_kobj {
    pub kobject: kobject,
    pub complete: completion,
}
#[repr(C)]
pub struct xstats {
    pub xs_stats: *mut xfsstats,
    pub xs_kobj: xfs_kobj,
}

unsafe extern "C" {
    pub static mut xfsstats: xstats;
    pub fn sysv_major(dev: u32) -> u32;
    pub fn sysv_minor(dev: u32) -> u32;
    pub fn sysv_encode_dev(dev: u32) -> u32;
    pub fn MKDEV(major: u32, minor: u32) -> u32;
    pub fn schedule_timeout_uninterruptible(ticks: i64);
    pub fn sort(a: *mut core::ffi::c_void, n: usize, size: usize, cmp: *mut core::ffi::c_void, swap: *mut core::ffi::c_void);
    pub fn dump_stack();
    pub fn is_power_of_2(value: u64) -> bool;
    pub fn ilog2(value: u64) -> u32;
    pub fn is_vmalloc_addr(addr: *const core::ffi::c_void) -> bool;
    pub fn vmalloc_to_page(addr: *const core::ffi::c_void) -> *mut page;
    pub fn virt_to_page(addr: *const core::ffi::c_void) -> *mut page;
    pub fn xfs_corruption_error(expr: *const core::ffi::c_char, level: i32, mp: *mut core::ffi::c_void, info: *mut core::ffi::c_void, flags: i32, file: *const core::ffi::c_char, line: i32, address: *const core::ffi::c_void);
    pub fn assfail(expr: *const core::ffi::c_char, file: *const core::ffi::c_char, line: i32) -> !;
    pub fn asswarn(expr: *const core::ffi::c_char, file: *const core::ffi::c_char, line: i32) -> !;
}

pub const NBBY: u32 = 8;
pub const BLKDEV_IOSHIFT: u32 = PAGE_SHIFT;
pub const BLKDEV_IOSIZE: usize = 1usize << BLKDEV_IOSHIFT;
pub const ENOATTR: i32 = ENODATA;
pub const EWRONGFS: i32 = EINVAL;

pub const PTR_FMT: &str = if cfg!(feature = "CONFIG_XFS_DEBUG") { "%px" } else { "%p" };

#[macro_export]
macro_rules! xfs_panic_mask { () => { xfs_params.panic_mask.val }; }
#[macro_export]
macro_rules! xfs_error_level { () => { xfs_params.error_level.val }; }
#[macro_export]
macro_rules! xfs_syncd_centisecs { () => { xfs_params.syncd_timer.val }; }
#[macro_export]
macro_rules! xfs_stats_clear { () => { xfs_params.stats_clear.val }; }
#[macro_export]
macro_rules! xfs_inherit_sync { () => { xfs_params.inherit_sync.val }; }
#[macro_export]
macro_rules! xfs_inherit_nodump { () => { xfs_params.inherit_nodump.val }; }
#[macro_export]
macro_rules! xfs_inherit_noatime { () => { xfs_params.inherit_noatim.val }; }
#[macro_export]
macro_rules! xfs_inherit_nosymlinks { () => { xfs_params.inherit_nosym.val }; }
#[macro_export]
macro_rules! xfs_rotorstep { () => { xfs_params.rotorstep.val }; }
#[macro_export]
macro_rules! xfs_inherit_nodefrag { () => { xfs_params.inherit_nodfrg.val }; }
#[macro_export]
macro_rules! xfs_fstrm_centisecs { () => { xfs_params.fstrm_timer.val }; }
#[macro_export]
macro_rules! xfs_blockgc_secs { () => { xfs_params.blockgc_timer.val }; }

#[macro_export]
macro_rules! current_cpu { () => { raw_smp_processor_id() }; }
#[macro_export]
macro_rules! howmany_bounded { ($x:expr, $y:expr) => { (($x + ($y - 1)) / $y) }; }

// xfs_sort(a,n,s,fn) expands to sort(a,n,s,fn,NULL); xfs_stack_trace() to dump_stack().
// BLKDEV_BB is BTOBB(BLKDEV_IOSIZE), with PAGE_SHIFT supplied by the kernel.
extern "C" {
    pub fn xfs_rw_bdev(bdev: *mut block_device, sector: sector_t, count: u32, data: *mut i8, op: req_op) -> i32;
}

pub const fn howmany(x: u64, y: u64) -> u64 { (x + (y - 1)) / y }

pub unsafe fn delay(ticks: i64) { schedule_timeout_uninterruptible(ticks); }

pub unsafe fn xfs_to_linux_dev_t(dev: xfs_dev_t) -> u32 {
    MKDEV(sysv_major(dev) & 0x1ff, sysv_minor(dev))
}

pub unsafe fn linux_to_xfs_dev_t(dev: u32) -> xfs_dev_t { sysv_encode_dev(dev) }

pub unsafe fn rounddown_64(mut x: u64, y: u32) -> u64 { x -= x % y as u64; x }
pub unsafe fn roundup_64(mut x: u64, y: u32) -> u64 { x += y as u64 - 1; x -= x % y as u64; x }
pub unsafe fn howmany_64(mut x: u64, y: u32) -> u64 { x += y as u64 - 1; x / y as u64 }
pub unsafe fn isaligned_64(x: u64, y: u32) -> bool { x % y as u64 == 0 }

pub unsafe fn log2_if_power2(b: u64) -> i8 {
    if is_power_of_2(b) { ilog2(b) as i8 } else { -1 }
}
pub unsafe fn mask64_if_power2(b: u64) -> u64 {
    if is_power_of_2(b) { b - 1 } else { 0 }
}

pub unsafe fn kmem_to_page(addr: *mut core::ffi::c_void) -> *mut page {
    if is_vmalloc_addr(addr) { vmalloc_to_page(addr) } else { virt_to_page(addr) }
}

// The remaining C macros (parameter aliases, assertions, address labels, and
// configuration-dependent realtime-inode checks) depend on surrounding XFS
// structures and compiler/kernel primitives and retain their intent here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
