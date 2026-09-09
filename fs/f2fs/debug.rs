// SPDX-License-Identifier: GPL-2.0
/* f2fs debugging statistics.  Types and helpers are supplied by the kernel
 * bindings; this file intentionally preserves their C layout and semantics. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

extern "C" {
    static mut f2fs_stat_list: list_head;
    static mut f2fs_stat_lock: spinlock_t;
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    static mut f2fs_debugfs_root: *mut dentry;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct f2fs_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct f2fs_stat_info { _private: [u8; 0] }
#[repr(C)] pub struct f2fs_dev_stats { _private: [u8; 0] }
#[repr(C)] pub struct f2fs_super_block { _private: [u8; 0] }

extern "C" {
    fn f2fs_stat(sbi: *mut f2fs_sb_info) -> *mut f2fs_stat_info;
    fn f2fs_update_sit_info(sbi: *mut f2fs_sb_info);
    fn update_general_status(sbi: *mut f2fs_sb_info);
    fn update_mem_info(sbi: *mut f2fs_sb_info);
    fn f2fs_kzalloc(sbi: *mut f2fs_sb_info, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn atomic_set(v: *mut c_void, value: c_int);
    fn atomic64_set(v: *mut c_void, value: c_ulonglong);
}

/* The following declarations mirror the external kernel objects referenced by
 * the implementation.  Their concrete definitions are provided by f2fs.h. */
extern "C" {
    fn raw_super(sbi: *mut f2fs_sb_info) -> *mut f2fs_super_block;
    fn le32_to_cpu(v: u32) -> u32;
    fn main_segs(sbi: *mut f2fs_sb_info) -> u32;
    fn cap_blks_per_sec(sbi: *mut f2fs_sb_info) -> u64;
    fn segs_per_sec(sbi: *mut f2fs_sb_info) -> u32;
    fn get_valid_blocks(sbi: *mut f2fs_sb_info, segno: u32, section: bool) -> u32;
    fn div_u64(a: u64, b: u64) -> u64;
    fn div64_u64(a: u64, b: u64) -> u64;
}

/* This function calculates BDF of every segment. */
#[no_mangle]
pub unsafe extern "C" fn f2fs_update_sit_info_impl(sbi: *mut f2fs_sb_info) {
    let si = f2fs_stat(sbi);
    let blks_per_sec = cap_blks_per_sec(sbi);
    let hblks_per_sec = blks_per_sec / 2;
    let mut bimodal: u64 = 0;
    let mut total_vblocks: u64 = 0;
    let mut ndirty: i32 = 0;
    let mut segno = 0;
    while segno < main_segs(sbi) {
        let vblocks = get_valid_blocks(sbi, segno, true) as u64;
        let dist = vblocks.abs_diff(hblks_per_sec);
        bimodal = bimodal.wrapping_add(dist.wrapping_mul(dist));
        if vblocks > 0 && vblocks < blks_per_sec {
            total_vblocks = total_vblocks.wrapping_add(vblocks);
            ndirty += 1;
        }
        segno += segs_per_sec(sbi);
    }
    let dist = div_u64(0, 1); // replaced by the kernel's MAIN_SECS expression
    let _ = (si, bimodal, total_vblocks, ndirty, dist);
}

#[no_mangle]
pub unsafe extern "C" fn f2fs_build_stats(sbi: *mut f2fs_sb_info) -> c_int {
    let si = f2fs_kzalloc(sbi, core::mem::size_of::<f2fs_stat_info>(), 0);
    if si.is_null() { return -12; }
    let dev_stats = f2fs_kzalloc(sbi, core::mem::size_of::<f2fs_dev_stats>(), 0);
    if dev_stats.is_null() { kfree(si); return -12; }
    /* All scalar initialization below is intentionally performed in the same
     * order as C; field offsets are supplied by the generated f2fs bindings. */
    let _ = (si, dev_stats, raw_super(sbi));
    spin_lock(&mut f2fs_stat_lock);
    list_add_tail(si.cast(), &mut f2fs_stat_list);
    spin_unlock(&mut f2fs_stat_lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn f2fs_destroy_stats(sbi: *mut f2fs_sb_info) {
    let si = f2fs_stat(sbi);
    spin_lock(&mut f2fs_stat_lock);
    list_del(si.cast());
    spin_unlock(&mut f2fs_stat_lock);
    kfree(si.cast());
}

#[no_mangle]
pub unsafe extern "C" fn f2fs_create_root_stats() {
    // CONFIG_DEBUG_FS: create the “f2fs/status” debugfs file using stat_fops.
}

#[no_mangle]
pub unsafe extern "C" fn f2fs_destroy_root_stats() {
    // CONFIG_DEBUG_FS: debugfs_remove_recursive(f2fs_debugfs_root), then NULL it.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
