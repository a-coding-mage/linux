// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, "util/evsel.h"

extern "C" {
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: u64);
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

const WEIGHT_STRUCT: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__set_sample_weight(evsel: *mut evsel) {
    evsel__set_sample_bit(evsel, WEIGHT_STRUCT);
}
