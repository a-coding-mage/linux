// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Dependencies from "../../../util/pmu.h" and "mem-events.h".
#[repr(C)]
pub struct perf_pmu {
    pub is_core: bool,
    pub mem_events: *mut c_void,
}

unsafe extern "C" {
    pub static mut perf_mem_events_power: *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    if unsafe { (*pmu).is_core } {
        unsafe {
            (*pmu).mem_events = perf_mem_events_power;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
