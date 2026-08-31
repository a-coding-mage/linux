// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/topdown.c.
// Original dependencies: "topdown.h", <linux/kernel.h>.

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

// C source declares this as __weak.
#[no_mangle]
pub unsafe extern "C" fn arch_topdown_sample_read(_leader: *mut evsel) -> bool {
    false
}
