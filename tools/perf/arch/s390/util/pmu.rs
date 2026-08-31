// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright IBM Corp. 2023
 * Author(s): Thomas Richter <tmricht@linux.ibm.com>
 */

use core::ffi::c_char;

// C dependency intent:
// #include <string.h>
// #include "../../../util/pmu.h"

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
}

const S390_PMUPAI_CRYPTO: &[u8; 11] = b"pai_crypto\0";
const S390_PMUPAI_EXT: &[u8; 8] = b"pai_ext\0";
const S390_PMUCPUM_CF: &[u8; 8] = b"cpum_cf\0";

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    if strcmp(
        (*pmu).name,
        S390_PMUPAI_CRYPTO.as_ptr() as *const c_char,
    ) == 0
        || strcmp((*pmu).name, S390_PMUPAI_EXT.as_ptr() as *const c_char) == 0
        || strcmp((*pmu).name, S390_PMUCPUM_CF.as_ptr() as *const c_char) == 0
    {
        (*pmu).selectable = true;
    }
}
