/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The Kernel Concurrency Sanitizer (KCSAN) infrastructure. For more info please
 * see Documentation/dev-tools/kcsan.rst.
 *
 * Copyright (C) 2019, Google LLC.
 */

/* C header guard: _KERNEL_KCSAN_KCSAN_H */
/* External Linux definitions supplied by the surrounding translation unit. */

pub const KCSAN_CHECK_ADJACENT: usize = 1;
pub const NUM_SLOTS: usize = 1 + 2 * KCSAN_CHECK_ADJACENT;

extern "C" {
    pub static mut kcsan_udelay_task: core::ffi::c_uint;
    pub static mut kcsan_udelay_interrupt: core::ffi::c_uint;
}

/* Globally enable and disable KCSAN. */
extern "C" {
    pub static mut kcsan_enabled: bool;
}

/* Save/restore IRQ flags state trace dirtied by KCSAN. */
extern "C" {
    pub fn kcsan_save_irqtrace(task: *mut task_struct);
    pub fn kcsan_restore_irqtrace(task: *mut task_struct);
}

/*
 * Statistics counters displayed via debugfs; should only be modified in
 * slow-paths.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kcsan_counter_id {
    /* Number of watchpoints currently in use. */
    KCSAN_COUNTER_USED_WATCHPOINTS,
    /* Total number of watchpoints set up. */
    KCSAN_COUNTER_SETUP_WATCHPOINTS,
    /* Total number of data races. */
    KCSAN_COUNTER_DATA_RACES,
    /*
     * Total number of ASSERT failures due to races. If the observed race is
     * due to two conflicting ASSERT type accesses, then both will be counted.
     */
    KCSAN_COUNTER_ASSERT_FAILURES,
    /* Number of times no watchpoints were available. */
    KCSAN_COUNTER_NO_CAPACITY,
    /*
     * A thread checking a watchpoint raced with another checking thread;
     * only one will be reported.
     */
    KCSAN_COUNTER_REPORT_RACES,
    /* Observed data value change, but writer thread unknown. */
    KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN,
    /* The access cannot be encoded to a valid watchpoint. */
    KCSAN_COUNTER_UNENCODABLE_ACCESSES,
    /*
     * Watchpoint encoding caused a watchpoint to fire on mismatching
     * accesses.
     */
    KCSAN_COUNTER_ENCODING_FALSE_POSITIVES,
    KCSAN_COUNTER_COUNT, /* number of counters */
}

extern "C" {
    pub static mut kcsan_counters: [atomic_long_t; KCSAN_COUNTER_COUNT as usize];
}

/*
 * Returns true if data races in the function symbol that maps to func_addr
 * (offsets are ignored) should *not* be reported.
 */
extern "C" {
    pub fn kcsan_skip_report_debugfs(func_addr: core::ffi::c_ulong) -> bool;
}

/* Value-change states. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kcsan_value_change {
    /*
     * Did not observe a value-change, however, it is valid to report the
     * race, depending on preferences.
     */
    KCSAN_VALUE_CHANGE_MAYBE,
    /* Did not observe a value-change, and it is invalid to report the race. */
    KCSAN_VALUE_CHANGE_FALSE,
    /* The value was observed to change, and the race should be reported. */
    KCSAN_VALUE_CHANGE_TRUE,
}

/*
 * The calling thread hit and consumed a watchpoint: set the access information
 * to be consumed by the reporting thread. No report is printed yet.
 */
extern "C" {
    pub fn kcsan_report_set_info(
        ptr: *const core::ffi::c_void,
        size: usize,
        access_type: core::ffi::c_int,
        ip: core::ffi::c_ulong,
        watchpoint_idx: core::ffi::c_int,
    );

    /*
     * The calling thread observed that the watchpoint it set up was hit and
     * consumed: print the full report based on information set by the racing
     * thread.
     */
    pub fn kcsan_report_known_origin(
        ptr: *const core::ffi::c_void,
        size: usize,
        access_type: core::ffi::c_int,
        ip: core::ffi::c_ulong,
        value_change: kcsan_value_change,
        watchpoint_idx: core::ffi::c_int,
        old: u64,
        new: u64,
        mask: u64,
    );

    /*
     * No other thread was observed to race with the access, but the data value
     * before and after the stall differs. Reports a race of "unknown origin".
     */
    pub fn kcsan_report_unknown_origin(
        ptr: *const core::ffi::c_void,
        size: usize,
        access_type: core::ffi::c_int,
        ip: core::ffi::c_ulong,
        old: u64,
        new: u64,
        mask: u64,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
