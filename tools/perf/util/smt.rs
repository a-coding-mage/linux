// SPDX-License-Identifier: GPL-2.0-only

use std::os::raw::{c_char, c_int};

extern "C" {
    fn sysfs__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn online_topology() -> *mut c_void;
    fn cpu_topology__smt_on(topology: *mut c_void) -> bool;
    fn cpu_topology__core_wide(
        topology: *mut c_void,
        user_requested_cpu_list: *const c_char,
    ) -> bool;
}

use std::ffi::c_void;

static mut CACHED: bool = false;
static mut CACHED_RESULT: bool = false;

#[no_mangle]
pub unsafe extern "C" fn smt_on() -> bool {
    let mut fs_value: c_int = 0;

    if CACHED {
        return CACHED_RESULT;
    }

    if sysfs__read_int(
        b"devices/system/cpu/smt/active\0".as_ptr() as *const c_char,
        &mut fs_value,
    ) >= 0
    {
        CACHED_RESULT = fs_value == 1;
    } else {
        CACHED_RESULT = cpu_topology__smt_on(online_topology());
    }

    CACHED = true;
    CACHED_RESULT
}

#[no_mangle]
pub unsafe extern "C" fn core_wide(
    system_wide: bool,
    user_requested_cpu_list: *const c_char,
) -> bool {
    /* If not everything running on a core is being recorded then we can't use core_wide. */
    if !system_wide {
        return false;
    }

    /* Cheap case that SMT is disabled and therefore we're inherently core_wide. */
    if !smt_on() {
        return true;
    }

    cpu_topology__core_wide(online_topology(), user_requested_cpu_list)
}
