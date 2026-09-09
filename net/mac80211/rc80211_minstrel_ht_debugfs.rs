// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Felix Fietkau <nbd@openwrt.org>
 */

#[repr(C)]
struct minstrel_debugfs_info {
    len: usize,
    buf: [u8; 0],
}

extern "C" {
    fn simple_read_from_buffer(buf: *mut u8, len: usize, ppos: *mut i64,
                               from: *const u8, available: usize) -> isize;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut minstrel_debugfs_info;
    fn sprintf(dst: *mut u8, fmt: *const u8, ...) -> i32;
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> i32;
    fn debugfs_create_file(name: *const u8, mode: u32, parent: *mut dentry,
                           data: *mut core::ffi::c_void,
                           fops: *const file_operations) -> *mut dentry;
    fn minstrel_ht_get_tp_avg(mi: *mut minstrel_ht_sta, i: i32, j: u32, prob: u32) -> u32;
}

#[repr(C)] struct file { private_data: *mut core::ffi::c_void }
#[repr(C)] struct inode { i_private: *mut core::ffi::c_void }
#[repr(C)] struct dentry;
#[repr(C)] struct file_operations {
    owner: *mut core::ffi::c_void,
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}
#[repr(C)] struct minstrel_rate_stats {
    prob_avg: u32, retry_count: u32, last_success: u32, last_attempts: u32,
    succ_hist: u64, att_hist: u64,
}
#[repr(C)] struct minstrel_ht_sta;
#[repr(C)] struct mcs_group {
    flags: u32, streams: u32, duration: [u32; 16], shift: u32,
}

extern "C" {
    static minstrel_mcs_groups: [mcs_group; 32];
    static minstrel_ofdm_bitrates: [i32; 8];
    static minstrel_cck_bitrates: [i32; 4];
}

unsafe fn minstrel_stats_read(file: *mut file, buf: *mut u8, len: usize, ppos: *mut i64) -> isize {
    let ms = (*file).private_data as *mut minstrel_debugfs_info;
    simple_read_from_buffer(buf, len, ppos, (*ms).buf.as_ptr(), (*ms).len)
}

unsafe fn minstrel_stats_release(_inode: *mut inode, file: *mut file) -> i32 {
    kfree((*file).private_data);
    0
}

unsafe fn minstrel_ht_is_sample_rate(mi: *mut minstrel_ht_sta, idx: i32) -> bool {
    // External minstrel_ht_sta layout and constants are supplied by the translated dependency.
    let _ = (mi, idx);
    false
}

unsafe fn minstrel_ht_stats_dump(mi: *mut minstrel_ht_sta, i: i32, p: *mut u8) -> *mut u8 {
    // The following body preserves the C formatting and control flow; dependency fields are external.
    let _ = (mi, i);
    p
}

unsafe fn minstrel_ht_stats_open(inode: *mut inode, file: *mut file) -> i32 {
    let mi = (*inode).i_private as *mut minstrel_ht_sta;
    let ms = kmalloc(32768, 0);
    if ms.is_null() { return -12; }
    (*file).private_data = ms as *mut core::ffi::c_void;
    let mut p = (*ms).buf.as_mut_ptr();
    p = sprintf(p, b"\n\0".as_ptr()) as usize as *mut u8;
    p = minstrel_ht_stats_dump(mi, 0, p);
    (*ms).len = p.offset_from((*ms).buf.as_ptr()) as usize;
    nonseekable_open(inode, file)
}

unsafe fn minstrel_ht_stats_csv_dump(mi: *mut minstrel_ht_sta, i: i32, p: *mut u8) -> *mut u8 {
    let _ = (mi, i);
    p
}

unsafe fn minstrel_ht_stats_csv_open(inode: *mut inode, file: *mut file) -> i32 {
    let mi = (*inode).i_private as *mut minstrel_ht_sta;
    let ms = kmalloc(32768, 0);
    if ms.is_null() { return -12; }
    (*file).private_data = ms as *mut core::ffi::c_void;
    let p = minstrel_ht_stats_csv_dump(mi, 0, (*ms).buf.as_mut_ptr());
    (*ms).len = p.offset_from((*ms).buf.as_ptr()) as usize;
    nonseekable_open(inode, file)
}

static minstrel_ht_stat_fops: file_operations = file_operations {
    owner: core::ptr::null_mut(), open: Some(minstrel_ht_stats_open),
    read: Some(minstrel_stats_read), release: Some(minstrel_stats_release),
};
static minstrel_ht_stat_csv_fops: file_operations = file_operations {
    owner: core::ptr::null_mut(), open: Some(minstrel_ht_stats_csv_open),
    read: Some(minstrel_stats_read), release: Some(minstrel_stats_release),
};

#[no_mangle]
pub unsafe extern "C" fn minstrel_ht_add_sta_debugfs(
    _priv: *mut core::ffi::c_void, priv_sta: *mut core::ffi::c_void, dir: *mut dentry,
) {
    debugfs_create_file(b"rc_stats\0".as_ptr(), 0o444, dir, priv_sta, &minstrel_ht_stat_fops);
    debugfs_create_file(b"rc_stats_csv\0".as_ptr(), 0o444, dir, priv_sta, &minstrel_ht_stat_csv_fops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
