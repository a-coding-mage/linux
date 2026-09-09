// SPDX-License-Identifier: GPL-2.0-only
/*
 *  drivers/cpufreq/cpufreq_stats.c
 *
 *  Copyright (C) 2003-2004 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
 *  (C) 2004 Zou Nan hai <nanhai.zou@intel.com>.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

// External kernel types, functions, constants, and generated attribute macros
// are supplied by the surrounding kernel translation.
#[repr(C)]
pub struct cpufreq_policy {
    pub stats: *mut cpufreq_stats,
    pub kobj: c_void,
    pub freq_table: *mut cpufreq_frequency_table,
    pub freq_table_sorted: c_uint,
    pub cur: c_uint,
}

#[repr(C)]
pub struct cpufreq_frequency_table {
    pub frequency: c_uint,
}

#[repr(C)]
pub struct attribute { _private: [u8; 0] }

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
    pub name: *const c_char,
}

#[repr(C)]
pub struct cpufreq_stats {
    pub total_trans: c_uint,
    pub last_time: c_ulonglong,
    pub max_state: c_uint,
    pub state_num: c_uint,
    pub last_index: c_uint,
    pub time_in_state: *mut u64,
    pub freq_table: *mut c_uint,
    pub trans_table: *mut c_uint,
    /* Deferred reset */
    pub reset_pending: c_uint,
    pub reset_time: c_ulonglong,
}

extern "C" {
    fn local_clock() -> c_ulonglong;
    fn cpufreq_table_count_valid_entries(policy: *mut cpufreq_policy) -> c_uint;
    fn sysfs_remove_group(kobj: *mut c_void, group: *const attribute_group);
    fn sysfs_create_group(kobj: *mut c_void, group: *const attribute_group) -> c_int;
    fn nsec_to_clock_t(nsec: c_ulonglong) -> c_ulonglong;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sysfs_emit_at(buf: *mut c_char, at: isize, fmt: *const c_char, ...) -> c_int;
    fn pr_warn_once(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn smp_rmb();
    fn smp_wmb();
}

const EFBIG: c_int = 27;
const PAGE_SIZE: isize = 4096;
const GFP_KERNEL: c_uint = 0;
const CPUFREQ_TABLE_UNSORTED: c_uint = 1;

unsafe fn cpufreq_stats_update(stats: *mut cpufreq_stats, time: c_ulonglong) {
    let cur_time = local_clock();
    (*stats).time_in_state.add((*stats).last_index as usize).write(
        (*stats).time_in_state.add((*stats).last_index as usize).read()
            .wrapping_add(cur_time.wrapping_sub(time)),
    );
    (*stats).last_time = cur_time;
}

unsafe fn cpufreq_stats_reset_table(stats: *mut cpufreq_stats) {
    let count = (*stats).max_state as usize;
    core::ptr::write_bytes((*stats).time_in_state, 0, count);
    core::ptr::write_bytes((*stats).trans_table, 0, count * count);
    (*stats).last_time = local_clock();
    (*stats).total_trans = 0;
    (*stats).reset_pending = 0;
    smp_rmb();
    cpufreq_stats_update(stats, (*stats).reset_time);
}

unsafe fn show_total_trans(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    let stats = (*policy).stats;
    if (*stats).reset_pending != 0 {
        sprintf(buf, b"%d\0".as_ptr() as *const c_char, 0) as isize
    } else {
        sprintf(buf, b"%u\n\0".as_ptr() as *const c_char, (*stats).total_trans) as isize
    }
}

unsafe fn show_time_in_state(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    let stats = (*policy).stats;
    let pending = (*stats).reset_pending != 0;
    let mut len: isize = 0;
    let mut i: c_uint = 0;
    while i < (*stats).state_num {
        let time = if pending {
            if i == (*stats).last_index { local_clock().wrapping_sub((*stats).reset_time) } else { 0 }
        } else {
            let mut value = (*stats).time_in_state.add(i as usize).read();
            if i == (*stats).last_index { value = value.wrapping_add(local_clock().wrapping_sub((*stats).last_time)); }
            value
        };
        len += sprintf(buf.offset(len), b"%u %llu\n\0".as_ptr() as *const c_char,
                       (*stats).freq_table.add(i as usize).read(), nsec_to_clock_t(time)) as isize;
        i += 1;
    }
    len
}

unsafe fn store_reset(policy: *mut cpufreq_policy, _buf: *const c_char, count: usize) -> isize {
    let stats = (*policy).stats;
    (*stats).reset_time = local_clock();
    smp_wmb();
    (*stats).reset_pending = 1;
    count as isize
}

unsafe fn show_trans_table(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    let stats = (*policy).stats;
    let pending = (*stats).reset_pending != 0;
    let mut len: isize = 0;
    len += sysfs_emit_at(buf, len, b"   From  :    To\n\0".as_ptr() as *const c_char) as isize;
    len += sysfs_emit_at(buf, len, b"         : \0".as_ptr() as *const c_char) as isize;
    let mut i = 0;
    while i < (*stats).state_num && len < PAGE_SIZE - 1 {
        len += sysfs_emit_at(buf, len, b"%9u \0".as_ptr() as *const c_char,
                             (*stats).freq_table.add(i as usize).read()) as isize;
        i += 1;
    }
    if len >= PAGE_SIZE - 1 { return PAGE_SIZE - 1; }
    len += sysfs_emit_at(buf, len, b"\n\0".as_ptr() as *const c_char) as isize;
    i = 0;
    while i < (*stats).state_num && len < PAGE_SIZE - 1 {
        len += sysfs_emit_at(buf, len, b"%9u: \0".as_ptr() as *const c_char,
                             (*stats).freq_table.add(i as usize).read()) as isize;
        let mut j = 0;
        while j < (*stats).state_num && len < PAGE_SIZE - 1 {
            let count = if pending { 0 } else { (*stats).trans_table.add(i as usize * (*stats).max_state as usize + j as usize).read() };
            len += sysfs_emit_at(buf, len, b"%9u \0".as_ptr() as *const c_char, count) as isize;
            j += 1;
        }
        if len >= PAGE_SIZE - 1 { break; }
        len += sysfs_emit_at(buf, len, b"\n\0".as_ptr() as *const c_char) as isize;
        i += 1;
    }
    if len >= PAGE_SIZE - 1 { pr_warn_once(b"cpufreq transition table exceeds PAGE_SIZE. Disabling\n\0".as_ptr() as *const c_char); return -EFBIG as isize; }
    len
}

unsafe fn freq_table_get_index(stats: *mut cpufreq_stats, freq: c_uint) -> c_int {
    let mut index: c_uint = 0;
    while index < (*stats).max_state {
        if (*stats).freq_table.add(index as usize).read() == freq {
            return index as c_int;
        }
        index += 1;
    }
    -1
}

pub unsafe fn cpufreq_stats_free_table(policy: *mut cpufreq_policy) {
    let stats = (*policy).stats;
    if stats.is_null() { return; }
    sysfs_remove_group(&mut (*policy).kobj as *mut c_void, &stats_attr_group);
    kfree((*stats).time_in_state as *mut c_void);
    kfree(stats as *mut c_void);
    (*policy).stats = core::ptr::null_mut();
}

pub unsafe fn cpufreq_stats_create_table(policy: *mut cpufreq_policy) {
    let count = cpufreq_table_count_valid_entries(policy);
    if count == 0 || !(*policy).stats.is_null() { return; }
    let stats = kzalloc(core::mem::size_of::<cpufreq_stats>(), GFP_KERNEL) as *mut cpufreq_stats;
    if stats.is_null() { return; }
    let alloc_size = count as usize * core::mem::size_of::<c_int>()
        + count as usize * core::mem::size_of::<u64>()
        + count as usize * count as usize * core::mem::size_of::<c_int>();
    (*stats).time_in_state = kzalloc(alloc_size, GFP_KERNEL) as *mut u64;
    if (*stats).time_in_state.is_null() { kfree(stats as *mut c_void); return; }
    (*stats).freq_table = (*stats).time_in_state.add(count as usize) as *mut c_uint;
    (*stats).trans_table = (*stats).freq_table.add(count as usize);
    (*stats).max_state = count;
    // cpufreq_for_each_valid_entry(pos, policy->freq_table): supplied by the kernel.
    // Preserve the source's valid/unique frequency-table population operation.
    let mut i = 0;
    let mut pos = (*policy).freq_table;
    while i < count && !pos.is_null() {
        let frequency = (*pos).frequency;
        if (*policy).freq_table_sorted != CPUFREQ_TABLE_UNSORTED
            || freq_table_get_index(stats, frequency) == -1 {
            (*stats).freq_table.add(i as usize).write(frequency);
            i += 1;
        }
        pos = pos.add(1);
    }
    (*stats).state_num = i;
    (*stats).last_time = local_clock();
    (*stats).last_index = freq_table_get_index(stats, (*policy).cur) as c_uint;
    (*policy).stats = stats;
    if sysfs_create_group(&mut (*policy).kobj as *mut c_void, &stats_attr_group) == 0 { return; }
    (*policy).stats = core::ptr::null_mut();
    kfree((*stats).time_in_state as *mut c_void);
    kfree(stats as *mut c_void);
}

pub unsafe fn cpufreq_stats_record_transition(policy: *mut cpufreq_policy, new_freq: c_uint) {
    let stats = (*policy).stats;
    if stats.is_null() { return; }
    if (*stats).reset_pending != 0 { cpufreq_stats_reset_table(stats); }
    let old_index = (*stats).last_index as c_int;
    let new_index = freq_table_get_index(stats, new_freq);
    if old_index == -1 || new_index == -1 || old_index == new_index { return; }
    cpufreq_stats_update(stats, (*stats).last_time);
    (*stats).last_index = new_index as c_uint;
    let slot = old_index as usize * (*stats).max_state as usize + new_index as usize;
    (*stats).trans_table.add(slot).write((*stats).trans_table.add(slot).read().wrapping_add(1));
    (*stats).total_trans = (*stats).total_trans.wrapping_add(1);
}

// Attribute declarations and show/store handlers are generated through the
// cpufreq_freq_attr_* kernel macros in the original source.
extern "C" {
    static stats_attr_group: attribute_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
