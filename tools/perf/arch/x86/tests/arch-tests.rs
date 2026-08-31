// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// #include <string.h>
// #include "tests/tests.h"
// #include "arch-tests.h"

use core::ptr;

extern "C" {
    static mut suite__dwarf_unwind: test_suite;
    static mut suite__x86_topdown: test_suite;
}

// Original C condition: #ifdef HAVE_EXTRA_TESTS
#[cfg(HAVE_EXTRA_TESTS)]
DEFINE_SUITE!("x86 instruction decoder - new instructions", insn_x86);

static mut intel_pt_tests: [test_case; 3] = [
    TEST_CASE!("Intel PT packet decoder", intel_pt_pkt_decoder),
    TEST_CASE!("Intel PT hybrid CPU compatibility", intel_pt_hybrid_compat),
    test_case {
        name: ptr::null(),
        ..unsafe { core::mem::zeroed() }
    },
];

#[no_mangle]
pub static mut suite__intel_pt: test_suite = test_suite {
    desc: c_str!("Intel PT"),
    test_cases: unsafe { intel_pt_tests.as_mut_ptr() },
};

// Original C condition: #if defined(__x86_64__)
#[cfg(target_arch = "x86_64")]
DEFINE_SUITE!("x86 bp modify", bp_modify);

DEFINE_SUITE!("AMD IBS via core pmu", amd_ibs_via_core_pmu);
DEFINE_SUITE_EXCLUSIVE!("AMD IBS sample period", amd_ibs_period);

static mut hybrid_tests: [test_case; 2] = [
    TEST_CASE_REASON!("x86 hybrid event parsing", hybrid, "not hybrid"),
    test_case {
        name: ptr::null(),
        ..unsafe { core::mem::zeroed() }
    },
];

#[no_mangle]
pub static mut suite__hybrid: test_suite = test_suite {
    desc: c_str!("x86 hybrid"),
    test_cases: unsafe { hybrid_tests.as_mut_ptr() },
};

#[no_mangle]
pub static mut arch_tests: [*mut test_suite; 8] = [
    // Original C condition: #ifdef HAVE_DWARF_UNWIND_SUPPORT
    #[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
    unsafe {
        &mut suite__dwarf_unwind
    },
    // Original C condition: #ifdef HAVE_EXTRA_TESTS
    #[cfg(HAVE_EXTRA_TESTS)]
    unsafe {
        &mut suite__insn_x86
    },
    unsafe { &mut suite__intel_pt },
    // Original C condition: #if defined(__x86_64__)
    #[cfg(target_arch = "x86_64")]
    unsafe {
        &mut suite__bp_modify
    },
    unsafe { &mut suite__amd_ibs_via_core_pmu },
    unsafe { &mut suite__amd_ibs_period },
    unsafe { &mut suite__hybrid },
    unsafe { &mut suite__x86_topdown },
    ptr::null_mut(),
];
