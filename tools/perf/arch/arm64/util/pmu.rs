// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// "../../../util/pmu.h"
// "../../../util/pmus.h"
// "../../../util/tool_pmu.h"
// <api/fs/fs.h>

use core::ffi::{c_char, c_int, c_ulonglong};

pub type u64 = u64;

pub const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

extern "C" {
    fn perf_pmus__find_core_pmu() -> *mut perf_pmu;
    fn perf_pmu__pathname_scnprintf(
        buf: *mut c_char,
        size: usize,
        pmu_name: *const c_char,
        filename: *const c_char,
    ) -> c_int;
    fn filename__read_ull(filename: *const c_char, value: *mut c_ulonglong) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__cpu_slots_per_cycle() -> u64 {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut slots: c_ulonglong = 0;
    let pmu: *mut perf_pmu = unsafe { perf_pmus__find_core_pmu() };

    if !pmu.is_null() {
        unsafe {
            perf_pmu__pathname_scnprintf(
                path.as_mut_ptr(),
                path.len(),
                (*pmu).name,
                c"caps/slots".as_ptr(),
            );
        }
        /*
         * The value of slots is not greater than 32 bits, but
         * filename__read_int can't read value with 0x prefix,
         * so use filename__read_ull instead.
         */
        unsafe {
            filename__read_ull(path.as_ptr(), &mut slots);
        }
    }

    slots as u64
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
