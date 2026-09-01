/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

// C dependencies from:
//   #include <linux/perf_event.h>
//   #include <linux/types.h>
//   #include <stdbool.h>
pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;

#[repr(C)]
pub struct event_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    pub print_start: Option<unsafe extern "C" fn(print_state: *mut c_void)>,
    pub print_end: Option<unsafe extern "C" fn(print_state: *mut c_void)>,
    pub print_event: Option<
        unsafe extern "C" fn(
            print_state: *mut c_void,
            topic: *const c_char,
            pmu_name: *const c_char,
            pmu_type: u32,
            event_name: *const c_char,
            event_alias: *const c_char,
            scale_unit: *const c_char,
            deprecated: bool,
            event_type_desc: *const c_char,
            desc: *const c_char,
            long_desc: *const c_char,
            encoding_desc: *const c_char,
        ),
    >,
    pub print_metric: Option<
        unsafe extern "C" fn(
            print_state: *mut c_void,
            group: *const c_char,
            name: *const c_char,
            desc: *const c_char,
            long_desc: *const c_char,
            expr: *const c_char,
            threshold: *const c_char,
            unit: *const c_char,
            pmu_name: *const c_char,
        ),
    >,
    pub skip_duplicate_pmus: Option<unsafe extern "C" fn(print_state: *mut c_void) -> bool>,
}

unsafe extern "C" {
    /** Print all events, the default when no options are specified. */
    pub fn print_events(print_cb: *const print_callbacks, print_state: *mut c_void);
    pub fn print_sdt_events(print_cb: *const print_callbacks, print_state: *mut c_void);
    pub fn metricgroup__print(print_cb: *const print_callbacks, print_state: *mut c_void);
    pub fn is_event_supported(type_: u8, config: u64) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
