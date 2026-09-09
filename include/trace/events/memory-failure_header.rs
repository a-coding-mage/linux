/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TRACE_SYSTEM is ras.  For historical versions, memory_failure_event is in
 * the ras subsystem; some user programs depend on it.
 *
 * TRACE_INCLUDE_FILE is memory-failure.
 * The original include guard and TRACE_HEADER_MULTI_READ conditional are
 * represented by this Rust translation unit.
 */

use core::ffi::{c_int, c_ulong};

/*
 * memory-failure recovery action result event
 *
 * unsigned long pfn - Page Frame Number of the corrupted page
 * int type          - Page types of the corrupted page
 * int result        - Result of recovery action
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MfActionResult {
    MfIgnored = 0,
    MfFailed,
    MfDelayed,
    MfRecovered,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MfPageType {
    MfMsgKernel = 0,
    MfMsgKernelHighOrder,
    MfMsgHuge,
    MfMsgFreeHuge,
    MfMsgGetHwpoison,
    MfMsgUnmapFailed,
    MfMsgDirtySwapcache,
    MfMsgCleanSwapcache,
    MfMsgDirtyMlockedLru,
    MfMsgCleanMlockedLru,
    MfMsgDirtyUnevictableLru,
    MfMsgCleanUnevictableLru,
    MfMsgDirtyLru,
    MfMsgCleanLru,
    MfMsgTruncatedLru,
    MfMsgBuddy,
    MfMsgDax,
    MfMsgUnsplitThp,
    MfMsgAlreadyPoisoned,
    MfMsgPfnMap,
    MfMsgUnknown,
}

/* String mappings produced by the original EM/EMe macro lists. */
pub const MF_ACTION_RESULT_STRINGS: &[(c_int, &str)] = &[
    (MfActionResult::MfIgnored as c_int, "Ignored"),
    (MfActionResult::MfFailed as c_int, "Failed"),
    (MfActionResult::MfDelayed as c_int, "Delayed"),
    (MfActionResult::MfRecovered as c_int, "Recovered"),
];

pub const MF_PAGE_TYPE_STRINGS: &[(c_int, &str)] = &[
    (MfPageType::MfMsgKernel as c_int, "reserved kernel page"),
    (MfPageType::MfMsgKernelHighOrder as c_int, "high-order kernel page"),
    (MfPageType::MfMsgHuge as c_int, "huge page"),
    (MfPageType::MfMsgFreeHuge as c_int, "free huge page"),
    (MfPageType::MfMsgGetHwpoison as c_int, "get hwpoison page"),
    (MfPageType::MfMsgUnmapFailed as c_int, "unmapping failed page"),
    (MfPageType::MfMsgDirtySwapcache as c_int, "dirty swapcache page"),
    (MfPageType::MfMsgCleanSwapcache as c_int, "clean swapcache page"),
    (MfPageType::MfMsgDirtyMlockedLru as c_int, "dirty mlocked LRU page"),
    (MfPageType::MfMsgCleanMlockedLru as c_int, "clean mlocked LRU page"),
    (MfPageType::MfMsgDirtyUnevictableLru as c_int, "dirty unevictable LRU page"),
    (MfPageType::MfMsgCleanUnevictableLru as c_int, "clean unevictable LRU page"),
    (MfPageType::MfMsgDirtyLru as c_int, "dirty LRU page"),
    (MfPageType::MfMsgCleanLru as c_int, "clean LRU page"),
    (MfPageType::MfMsgTruncatedLru as c_int, "already truncated LRU page"),
    (MfPageType::MfMsgBuddy as c_int, "free buddy page"),
    (MfPageType::MfMsgDax as c_int, "dax page"),
    (MfPageType::MfMsgUnsplitThp as c_int, "unsplit thp"),
    (MfPageType::MfMsgAlreadyPoisoned as c_int, "already poisoned"),
    (MfPageType::MfMsgPfnMap as c_int, "non struct page pfn"),
    (MfPageType::MfMsgUnknown as c_int, "unknown page"),
];

#[repr(C)]
pub struct MemoryFailureEventEntry {
    pub pfn: c_ulong,
    pub type_: c_int,
    pub result: c_int,
}

/* TRACE_EVENT(memory_failure_event, ...): externally defined tracepoint. */
unsafe extern "C" {
    pub fn trace_memory_failure_event(pfn: c_ulong, type_: c_int, result: c_int);
}

/* The original trace/define_trace.h include supplies the tracepoint definition. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
