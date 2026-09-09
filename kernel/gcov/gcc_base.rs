// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/gcov implementation.

#[allow(non_camel_case_types)]
pub type gcov_type = u64;

#[repr(C)]
pub struct gcov_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    static mut gcov_lock: mutex;
    static mut gcov_events_enabled: bool;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn gcov_info_version(info: *mut gcov_info) -> u32;
    fn gcov_info_link(info: *mut gcov_info);
    fn gcov_event(event: u32, info: *mut gcov_info);
}

// Build-time kernel logging and export-symbol facilities are represented by
// their corresponding external interfaces in the surrounding implementation.
const GCOV_ADD: u32 = 0;

/*
 * __gcov_init is called by gcc-generated constructor code for each object
 * file compiled with -fprofile-arcs.
 */
#[no_mangle]
pub unsafe extern "C" fn __gcov_init(info: *mut gcov_info) {
    static mut gcov_version: u32 = 0;

    mutex_lock(&raw mut gcov_lock);
    if gcov_version == 0 {
        gcov_version = gcov_info_version(info);
        /*
         * Printing gcc's version magic may prove useful for debugging
         * incompatibility reports.
         */
        // pr_info("version magic: 0x%x\n", gcov_version);
    }
    /*
     * Add new profiling data structure to list and inform event
     * listener.
     */
    gcov_info_link(info);
    if gcov_events_enabled {
        gcov_event(GCOV_ADD, info);
    }
    mutex_unlock(&raw mut gcov_lock);
}
// EXPORT_SYMBOL(__gcov_init);

/*
 * These functions may be referenced by gcc-generated profiling code but serve
 * no function for kernel profiling.
 */
#[no_mangle]
pub unsafe extern "C" fn __gcov_flush() {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_flush);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_add(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_add);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_single(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_single);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_delta(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_delta);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_ior(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_ior);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_time_profile(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_time_profile);

#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_icall_topn(
    _counters: *mut gcov_type,
    _n_counters: u32,
) {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_merge_icall_topn);

#[no_mangle]
pub unsafe extern "C" fn __gcov_exit() {
    /* Unused. */
}
// EXPORT_SYMBOL(__gcov_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
