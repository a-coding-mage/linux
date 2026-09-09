/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/wbt.h.
// The C TRACE_EVENT declarations below are represented as C-layout event
// entry types; tracepoint registration and formatting are supplied externally.

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct WbtStatEntry {
    pub name: [c_char; 32],
    pub rmean: i64,
    pub rmin: u64,
    pub rmax: u64,
    pub rnr_samples: i64,
    pub rtime: i64,
    pub wmean: i64,
    pub wmin: u64,
    pub wmax: u64,
    pub wnr_samples: i64,
    pub wtime: i64,
}

#[repr(C)]
pub struct WbtLatEntry {
    pub name: [c_char; 32],
    pub lat: c_ulong,
}

#[repr(C)]
pub struct WbtStepEntry {
    pub name: [c_char; 32],
    pub msg: *const c_char,
    pub step: i32,
    pub window: c_ulong,
    pub bg: u32,
    pub normal: u32,
    pub max: u32,
}

#[repr(C)]
pub struct WbtTimerEntry {
    pub name: [c_char; 32],
    pub status: u32,
    pub step: i32,
    pub inflight: u32,
}

// External kernel types and helpers referenced by the original header.
#[repr(C)]
pub struct BackingDevInfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BlkRqStat {
    pub mean: i64,
    pub min: u64,
    pub max: u64,
    pub nr_samples: i64,
}

extern "C" {
    pub fn bdi_dev_name(bdi: *mut BackingDevInfo) -> *const c_char;
    pub fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    pub fn div_u64(dividend: u64, divisor: u32) -> u64;
}

// wbt_stat: trace stats for blk_wb.
// TP_PROTO(struct backing_dev_info *bdi, struct blk_rq_stat *stat)
#[inline]
pub unsafe fn wbt_stat_assign(entry: *mut WbtStatEntry, bdi: *mut BackingDevInfo, stat: *const BlkRqStat) {
    strscpy((*entry).name.as_mut_ptr(), bdi_dev_name(bdi), (*entry).name.len());
    (*entry).rmean = (*stat.add(0)).mean;
    (*entry).rmin = (*stat.add(0)).min;
    (*entry).rmax = (*stat.add(0)).max;
    (*entry).rnr_samples = (*stat.add(0)).nr_samples;
    (*entry).wmean = (*stat.add(1)).mean;
    (*entry).wmin = (*stat.add(1)).min;
    (*entry).wmax = (*stat.add(1)).max;
    (*entry).wnr_samples = (*stat.add(1)).nr_samples;
}

// wbt_lat: trace latency event.
#[inline]
pub unsafe fn wbt_lat_assign(entry: *mut WbtLatEntry, bdi: *mut BackingDevInfo, lat: c_ulong) {
    strscpy((*entry).name.as_mut_ptr(), bdi_dev_name(bdi), (*entry).name.len());
    (*entry).lat = div_u64(lat as u64, 1000) as c_ulong;
}

// wbt_step: trace wb event step.
#[inline]
pub unsafe fn wbt_step_assign(
    entry: *mut WbtStepEntry,
    bdi: *mut BackingDevInfo,
    msg: *const c_char,
    step: i32,
    window: c_ulong,
    bg: u32,
    normal: u32,
    max: u32,
) {
    strscpy((*entry).name.as_mut_ptr(), bdi_dev_name(bdi), (*entry).name.len());
    (*entry).msg = msg;
    (*entry).step = step;
    (*entry).window = div_u64(window as u64, 1000) as c_ulong;
    (*entry).bg = bg;
    (*entry).normal = normal;
    (*entry).max = max;
}

// wbt_timer: trace wb timer event.
#[inline]
pub unsafe fn wbt_timer_assign(
    entry: *mut WbtTimerEntry,
    bdi: *mut BackingDevInfo,
    status: u32,
    step: i32,
    inflight: u32,
) {
    strscpy((*entry).name.as_mut_ptr(), bdi_dev_name(bdi), (*entry).name.len());
    (*entry).status = status;
    (*entry).step = step;
    (*entry).inflight = inflight;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
