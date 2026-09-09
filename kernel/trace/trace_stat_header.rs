// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the Linux seq_file interface:
// use linux::seq_file::{cmp_func_t, seq_file};

/*
 * If you want to provide a stat file (one-shot statistics), fill
 * an iterator with stat_start/stat_next and a stat_show callbacks.
 * The others callbacks are optional.
 */
#[repr(C)]
pub struct tracer_stat {
    /* The name of your stat file */
    pub name: *const ::std::os::raw::c_char,
    /* Iteration over statistic entries */
    pub stat_start: Option<unsafe extern "C" fn(trace: *mut tracer_stat) -> *mut ::std::ffi::c_void>,
    pub stat_next: Option<unsafe extern "C" fn(prev: *mut ::std::ffi::c_void, idx: ::std::os::raw::c_int) -> *mut ::std::ffi::c_void>,
    /* Compare two entries for stats sorting */
    pub stat_cmp: cmp_func_t,
    /* Print a stat entry */
    pub stat_show: Option<unsafe extern "C" fn(s: *mut seq_file, p: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int>,
    /* Release an entry */
    pub stat_release: Option<unsafe extern "C" fn(stat: *mut ::std::ffi::c_void)>,
    /* Print the headers of your stat entries */
    pub stat_headers: Option<unsafe extern "C" fn(s: *mut seq_file) -> ::std::os::raw::c_int>,
}

/*
 * Destroy or create a stat file
 */
unsafe extern "C" {
    pub fn register_stat_tracer(trace: *mut tracer_stat) -> ::std::os::raw::c_int;
    pub fn unregister_stat_tracer(trace: *mut tracer_stat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
