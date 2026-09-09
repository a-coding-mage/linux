// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/* Dependencies supplied by the surrounding translation unit. */

const BLOCKSIZE_MIN: usize = 3500;
const THRESHOLD_PENALTY_RATE: u64 = 16;
const THRESHOLD_BASE: u64 = THRESHOLD_PENALTY_RATE - 2;
const THRESHOLD_PENALTY: i32 = 3;

const HASHLENGTH: usize = 2;
const HASHLOG_MAX: usize = 10;
const HASHTABLESIZE: usize = 1 << HASHLOG_MAX;
const HASHMASK: usize = HASHTABLESIZE - 1;
const KNUTH: u32 = 0x9e3779b9;

/* for hashLog > 8, hash 2 bytes.
 * for hashLog == 8, just take the byte, no hashing.
 * The speed of this method relies on compile-time constant propagation */
#[inline]
unsafe fn hash2(p: *const core::ffi::c_void, hash_log: u32) -> u32 {
    assert!(hash_log >= 8);
    if hash_log == 8 {
        return *(p as *const u8) as u32;
    }
    assert!(hash_log <= HASHLOG_MAX as u32);
    let value = u16::from_ne_bytes([*(p as *const u8), *((p as *const u8).add(1))]);
    ((value as u32).wrapping_mul(KNUTH)) >> (32 - hash_log)
}

#[repr(C)]
pub struct Fingerprint {
    pub events: [u32; HASHTABLESIZE],
    pub nbEvents: usize,
}

#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}

unsafe fn initStats(fpstats: *mut FPStats) {
    core::ptr::write_bytes(fpstats as *mut u8, 0, core::mem::size_of::<FPStats>());
}

#[inline]
unsafe fn addEvents_generic(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    src_size: usize,
    sampling_rate: usize,
    hash_log: u32,
) {
    let p = src as *const u8;
    let limit = src_size - HASHLENGTH + 1;
    assert!(src_size >= HASHLENGTH);
    let mut n = 0;
    while n < limit {
        let index = hash2(p.add(n) as *const core::ffi::c_void, hash_log) as usize;
        (*fp).events[index] = (*fp).events[index].wrapping_add(1);
        n += sampling_rate;
    }
    (*fp).nbEvents += limit / sampling_rate;
}

#[inline]
unsafe fn recordFingerprint_generic(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    src_size: usize,
    sampling_rate: usize,
    hash_log: u32,
) {
    core::ptr::write_bytes(fp as *mut u8, 0, core::mem::size_of::<u32>() * (1usize << hash_log));
    (*fp).nbEvents = 0;
    addEvents_generic(fp, src, src_size, sampling_rate, hash_log);
}

type RecordEventsF = unsafe fn(*mut Fingerprint, *const core::ffi::c_void, usize);

unsafe fn ZSTD_recordFingerprint_1(fp: *mut Fingerprint, src: *const core::ffi::c_void, src_size: usize) {
    recordFingerprint_generic(fp, src, src_size, 1, 10);
}
unsafe fn ZSTD_recordFingerprint_5(fp: *mut Fingerprint, src: *const core::ffi::c_void, src_size: usize) {
    recordFingerprint_generic(fp, src, src_size, 5, 10);
}
unsafe fn ZSTD_recordFingerprint_11(fp: *mut Fingerprint, src: *const core::ffi::c_void, src_size: usize) {
    recordFingerprint_generic(fp, src, src_size, 11, 9);
}
unsafe fn ZSTD_recordFingerprint_43(fp: *mut Fingerprint, src: *const core::ffi::c_void, src_size: usize) {
    recordFingerprint_generic(fp, src, src_size, 43, 8);
}

unsafe fn abs64(s64: i64) -> u64 { if s64 < 0 { (-s64) as u64 } else { s64 as u64 } }

unsafe fn fpDistance(fp1: *const Fingerprint, fp2: *const Fingerprint, hash_log: usize) -> u64 {
    let mut distance = 0;
    assert!(hash_log <= HASHLOG_MAX);
    for n in 0..(1usize << hash_log) {
        distance += abs64((*fp1).events[n] as i64 * (*fp2).nbEvents as i64
            - (*fp2).events[n] as i64 * (*fp1).nbEvents as i64);
    }
    distance
}

/* Compare newEvents with pastEvents
 * return 1 when considered "too different"
 */
unsafe fn compareFingerprints(ref_fp: *const Fingerprint, newfp: *const Fingerprint, penalty: i32, hash_log: usize) -> i32 {
    assert!((*ref_fp).nbEvents > 0);
    assert!((*newfp).nbEvents > 0);
    let p50 = (*ref_fp).nbEvents as u64 * (*newfp).nbEvents as u64;
    let deviation = fpDistance(ref_fp, newfp, hash_log);
    let threshold = p50 * (THRESHOLD_BASE + penalty as u64) / THRESHOLD_PENALTY_RATE;
    (deviation >= threshold) as i32
}

unsafe fn mergeEvents(acc: *mut Fingerprint, newfp: *const Fingerprint) {
    for n in 0..HASHTABLESIZE { (*acc).events[n] += (*newfp).events[n]; }
    (*acc).nbEvents += (*newfp).nbEvents;
}

unsafe fn flushEvents(fpstats: *mut FPStats) {
    for n in 0..HASHTABLESIZE { (*fpstats).pastEvents.events[n] = (*fpstats).newEvents.events[n]; }
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    core::ptr::write_bytes(&mut (*fpstats).newEvents as *mut Fingerprint as *mut u8, 0, core::mem::size_of::<Fingerprint>());
}

unsafe fn removeEvents(acc: *mut Fingerprint, slice: *const Fingerprint) {
    for n in 0..HASHTABLESIZE {
        assert!((*acc).events[n] >= (*slice).events[n]);
        (*acc).events[n] -= (*slice).events[n];
    }
    (*acc).nbEvents -= (*slice).nbEvents;
}

const CHUNKSIZE: usize = 8 << 10;

unsafe fn ZSTD_splitBlock_byChunks(block_start: *const core::ffi::c_void, block_size: usize, level: i32, workspace: *mut core::ffi::c_void, wksp_size: usize) -> usize {
    let records_fs: [RecordEventsF; 4] = [ZSTD_recordFingerprint_43, ZSTD_recordFingerprint_11, ZSTD_recordFingerprint_5, ZSTD_recordFingerprint_1];
    let hash_params: [usize; 4] = [8, 9, 10, 10];
    assert!(0 <= level && level <= 3);
    let record_f = records_fs[level as usize];
    let fpstats = workspace as *mut FPStats;
    let p = block_start as *const u8;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos = 0;
    assert!(block_size == (128 << 10));
    assert!(!workspace.is_null());
    initStats(fpstats);
    record_f(&mut (*fpstats).pastEvents, p as *const core::ffi::c_void, CHUNKSIZE);
    while pos <= block_size - CHUNKSIZE {
        record_f(&mut (*fpstats).newEvents, p.add(pos) as *const core::ffi::c_void, CHUNKSIZE);
        if compareFingerprints(&(*fpstats).pastEvents, &(*fpstats).newEvents, penalty, hash_params[level as usize]) != 0 { return pos; }
        mergeEvents(&mut (*fpstats).pastEvents, &(*fpstats).newEvents);
        if penalty > 0 { penalty -= 1; }
        pos += CHUNKSIZE;
    }
    assert!(pos == block_size);
    let _ = wksp_size;
    let _ = (flushEvents as unsafe fn(*mut FPStats), removeEvents as unsafe fn(*mut Fingerprint, *const Fingerprint));
    block_size
}

/* ZSTD_splitBlock_fromBorders(): very fast strategy :
 * compare fingerprint from beginning and end of the block,
 * derive from their difference if it's preferable to split in the middle,
 * repeat the process a second time, for finer grained decision.
 * 3 times did not brought improvements, so I stopped at 2.
 * Benefits are good enough for a cheap heuristic.
 * More accurate splitting saves more, but speed impact is also more perceptible.
 * For better accuracy, use more elaborate variant *_byChunks.
 */
unsafe fn ZSTD_splitBlock_fromBorders(block_start: *const core::ffi::c_void, block_size: usize, workspace: *mut core::ffi::c_void, wksp_size: usize) -> usize {
    const SEGMENT_SIZE: usize = 512;
    let fpstats = workspace as *mut FPStats;
    let middle_events = (workspace as *mut u8).add(512 * core::mem::size_of::<u32>()) as *mut Fingerprint;
    assert!(block_size == (128 << 10));
    assert!(!workspace.is_null());
    initStats(fpstats);
    HIST_add((*fpstats).pastEvents.events.as_mut_ptr(), block_start, SEGMENT_SIZE);
    HIST_add((*fpstats).newEvents.events.as_mut_ptr(), (block_start as *const u8).add(block_size - SEGMENT_SIZE) as *const core::ffi::c_void, SEGMENT_SIZE);
    (*fpstats).pastEvents.nbEvents = SEGMENT_SIZE;
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE;
    if compareFingerprints(&(*fpstats).pastEvents, &(*fpstats).newEvents, 0, 8) == 0 { return block_size; }
    HIST_add((*middle_events).events.as_mut_ptr(), (block_start as *const u8).add(block_size / 2 - SEGMENT_SIZE / 2) as *const core::ffi::c_void, SEGMENT_SIZE);
    (*middle_events).nbEvents = SEGMENT_SIZE;
    let dist_from_begin = fpDistance(&(*fpstats).pastEvents, middle_events, 8);
    let dist_from_end = fpDistance(&(*fpstats).newEvents, middle_events, 8);
    let min_distance = SEGMENT_SIZE as u64 * SEGMENT_SIZE as u64 / 3;
    let _ = wksp_size;
    if abs64(dist_from_begin as i64 - dist_from_end as i64) < min_distance { return 64 * 1024; }
    if dist_from_begin > dist_from_end { 32 * 1024 } else { 96 * 1024 }
}

pub unsafe fn ZSTD_splitBlock(block_start: *const core::ffi::c_void, block_size: usize, level: i32, workspace: *mut core::ffi::c_void, wksp_size: usize) -> usize {
    assert!(0 <= level && level <= 4);
    if level == 0 { return ZSTD_splitBlock_fromBorders(block_start, block_size, workspace, wksp_size); }
    ZSTD_splitBlock_byChunks(block_start, block_size, level - 1, workspace, wksp_size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
