/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependency: "tests/tests.h" */

#[repr(C)]
pub struct test_suite {
    _unused: [u8; 0],
}

/* Tests */
unsafe extern "C" {
    pub fn test__rdpmc(test: *mut test_suite, subtest: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    /* #ifdef HAVE_EXTRA_TESTS */
    pub fn test__insn_x86(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn test__intel_pt_pkt_decoder(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn test__intel_pt_hybrid_compat(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn test__bp_modify(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn test__amd_ibs_via_core_pmu(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn test__amd_ibs_period(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn test__hybrid(
        test: *mut test_suite,
        subtest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /*
     * DECLARE_SUITE(x86_topdown);
     * The macro definition is supplied by "tests/tests.h".
     */

    pub static mut arch_tests: *mut *mut test_suite;
}
