// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/cpufreq/freq_table.c
 *
 * Copyright (C) 2002 - 2003 Dominik Brodowski
 */

use core::ffi::{c_char, c_int, c_void};

const CPUFREQ_BOOST_FREQ: u32 = 1 << 0;
const CPUFREQ_RELATION_H: u32 = 0;
const CPUFREQ_RELATION_L: u32 = 1;
const CPUFREQ_RELATION_C: u32 = 2;
const CPUFREQ_TABLE_UNSORTED: u32 = 0;
const CPUFREQ_TABLE_SORTED_ASCENDING: u32 = 1;
const CPUFREQ_TABLE_SORTED_DESCENDING: u32 = 2;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct CpufreqFrequencyTable {
    pub flags: u32,
    pub driver_data: u32,
    pub frequency: u32,
}

#[repr(C)]
pub struct CpufreqCpuinfo {
    pub min_freq: u32,
    pub max_freq: u32,
}

#[repr(C)]
pub struct CpufreqPolicy {
    pub freq_table: *mut CpufreqFrequencyTable,
    pub cpuinfo: CpufreqCpuinfo,
    pub boost_enabled: bool,
    pub cpu: u32,
    pub freq_table_sorted: u32,
    pub boost_supported: bool,
}

#[repr(C)]
pub struct CpufreqPolicyData {
    pub freq_table: *mut CpufreqFrequencyTable,
    pub min: u32,
    pub max: u32,
    pub cpu: u32,
}

#[repr(C)]
pub struct FreqAttr {
    pub attr: *mut c_void,
    pub show: Option<unsafe extern "C" fn(*mut CpufreqPolicy, *mut c_char) -> isize>,
}

unsafe extern "C" {
    fn cpufreq_boost_enabled() -> bool;
    fn cpufreq_verify_within_cpu_limits(policy: *mut CpufreqPolicyData);
    fn has_target_index() -> bool;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn warn(condition: bool, fmt: *const c_char, ...);
}

unsafe fn policy_has_boost_freq(policy: *mut CpufreqPolicy) -> bool {
    let table = (*policy).freq_table;
    if table.is_null() {
        return false;
    }

    let mut pos = table;
    while (*pos).frequency != u32::MAX {
        if (*pos).flags & CPUFREQ_BOOST_FREQ != 0 {
            return true;
        }
        pos = pos.add(1);
    }
    false
}

pub unsafe extern "C" fn cpufreq_frequency_table_cpuinfo(policy: *mut CpufreqPolicy) -> c_int {
    let table = (*policy).freq_table;
    let mut min_freq = u32::MAX;
    let mut max_freq = 0;
    let mut i = 0;
    if !table.is_null() {
        let mut pos = table;
        while (*pos).frequency != u32::MAX {
            let freq = (*pos).frequency;
            if (!cpufreq_boost_enabled() || !(*policy).boost_enabled)
                && ((*pos).flags & CPUFREQ_BOOST_FREQ != 0)
            {
                pos = pos.add(1);
                i += 1;
                continue;
            }
            if freq < min_freq { min_freq = freq; }
            if freq > max_freq { max_freq = freq; }
            pos = pos.add(1);
            i += 1;
        }
    }
    (*policy).cpuinfo.min_freq = min_freq;
    if (*policy).cpuinfo.max_freq < max_freq { (*policy).cpuinfo.max_freq = max_freq; }
    if min_freq == u32::MAX { -EINVAL } else { 0 }
}

pub unsafe extern "C" fn cpufreq_frequency_table_verify(policy: *mut CpufreqPolicyData) -> c_int {
    let table = (*policy).freq_table;
    cpufreq_verify_within_cpu_limits(policy);
    let mut prev_smaller = 0;
    let mut found = false;
    if !table.is_null() {
        let mut pos = table;
        while (*pos).frequency != u32::MAX {
            let freq = (*pos).frequency;
            if freq >= (*policy).min && freq <= (*policy).max { found = true; break; }
            if prev_smaller < freq && freq <= (*policy).max { prev_smaller = freq; }
            pos = pos.add(1);
        }
    }
    if !found { (*policy).max = prev_smaller; cpufreq_verify_within_cpu_limits(policy); }
    0
}

pub unsafe extern "C" fn cpufreq_generic_frequency_table_verify(policy: *mut CpufreqPolicyData) -> c_int {
    if (*policy).freq_table.is_null() { return -ENODEV; }
    cpufreq_frequency_table_verify(policy)
}

pub unsafe extern "C" fn cpufreq_table_index_unsorted(policy: *mut CpufreqPolicy, target_freq: u32, min: u32, max: u32, relation: u32) -> c_int {
    let table = (*policy).freq_table;
    let mut optimal_freq = 0;
    let mut suboptimal_freq = 0;
    let mut optimal_data = u32::MAX;
    let mut suboptimal_data = u32::MAX;
    if relation == CPUFREQ_RELATION_H { suboptimal_freq = u32::MAX; }
    if relation == CPUFREQ_RELATION_L || relation == CPUFREQ_RELATION_C { optimal_freq = u32::MAX; }
    let mut i = 0;
    let mut pos = table;
    while !pos.is_null() && (*pos).frequency != u32::MAX {
        let freq = (*pos).frequency;
        if freq >= min && freq <= max {
            if freq == target_freq { optimal_data = i; break; }
            match relation {
                CPUFREQ_RELATION_H => if freq < target_freq { if freq >= optimal_freq { optimal_freq=freq; optimal_data=i; } } else if freq <= suboptimal_freq { suboptimal_freq=freq; suboptimal_data=i; },
                CPUFREQ_RELATION_L => if freq > target_freq { if freq <= optimal_freq { optimal_freq=freq; optimal_data=i; } } else if freq >= suboptimal_freq { suboptimal_freq=freq; suboptimal_data=i; },
                CPUFREQ_RELATION_C => { let diff = freq.abs_diff(target_freq); if diff < optimal_freq || (diff == optimal_freq && freq > (*table.add(optimal_data as usize)).frequency) { optimal_freq=diff; optimal_data=i; } },
                _ => {}
            }
        }
        i += 1; pos = pos.add(1);
    }
    let index = if optimal_data > i { if suboptimal_data > i { warn(true, c"Invalid frequency table: %u\n".as_ptr(), (*policy).cpu); return 0; } suboptimal_data } else { optimal_data };
    index as c_int
}

pub unsafe extern "C" fn cpufreq_frequency_table_get_index(policy: *mut CpufreqPolicy, freq: u32) -> c_int {
    let table = (*policy).freq_table;
    if table.is_null() { return -ENOENT; }
    let mut pos = table; let mut idx = 0;
    while (*pos).frequency != u32::MAX { if (*pos).frequency == freq { return idx; } pos=pos.add(1); idx+=1; }
    -EINVAL
}

unsafe fn show_available_freqs(policy: *mut CpufreqPolicy, buf: *mut c_char, show_boost: bool) -> isize {
    let table = (*policy).freq_table;
    if table.is_null() { return -ENODEV as isize; }
    let mut count = 0; let mut pos = table;
    while (*pos).frequency != u32::MAX {
        if show_boost != ((*pos).flags & CPUFREQ_BOOST_FREQ != 0) { pos=pos.add(1); continue; }
        count += sprintf(buf.add(count as usize), c"%u ".as_ptr(), (*pos).frequency);
        pos=pos.add(1);
    }
    count += sprintf(buf.add(count as usize), c"\n".as_ptr()); count as isize
}

pub unsafe extern "C" fn scaling_available_frequencies_show(policy: *mut CpufreqPolicy, buf: *mut c_char) -> isize { show_available_freqs(policy, buf, false) }
pub unsafe extern "C" fn scaling_boost_frequencies_show(policy: *mut CpufreqPolicy, buf: *mut c_char) -> isize { show_available_freqs(policy, buf, true) }

unsafe fn set_freq_table_sorted(policy: *mut CpufreqPolicy) -> c_int {
    (*policy).freq_table_sorted = CPUFREQ_TABLE_UNSORTED;
    let table = (*policy).freq_table; if table.is_null() { return 0; }
    let mut pos = table; let mut prev: *mut CpufreqFrequencyTable = core::ptr::null_mut(); let mut ascending = 0;
    while (*pos).frequency != u32::MAX { if !prev.is_null() { if (*pos).frequency == (*prev).frequency { return -EINVAL; } if (*pos).frequency > (*prev).frequency { if ascending < 0 { return 0; } ascending += 1; } else { if ascending > 0 { return 0; } ascending -= 1; } } prev=pos; pos=pos.add(1); }
    (*policy).freq_table_sorted = if ascending > 0 { CPUFREQ_TABLE_SORTED_ASCENDING } else { CPUFREQ_TABLE_SORTED_DESCENDING }; 0
}

pub unsafe extern "C" fn cpufreq_table_validate_and_sort(policy: *mut CpufreqPolicy) -> c_int {
    if (*policy).freq_table.is_null() { if has_target_index() { return -EINVAL; } return 0; }
    let ret = cpufreq_frequency_table_cpuinfo(policy); if ret != 0 { return ret; }
    if policy_has_boost_freq(policy) { (*policy).boost_supported = true; }
    if (*policy).freq_table_sorted == CPUFREQ_TABLE_SORTED_ASCENDING || (*policy).freq_table_sorted == CPUFREQ_TABLE_SORTED_DESCENDING { return 0; }
    set_freq_table_sorted(policy)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
