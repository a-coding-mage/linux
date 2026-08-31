// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Intel Corporation. All rights reserved.

unsafe extern "C" {
    pub fn pmem_test() -> ::core::ffi::c_int;
    pub fn libnvdimm_test() -> ::core::ffi::c_int;
    pub fn acpi_nfit_test() -> ::core::ffi::c_int;
    pub fn device_dax_test() -> ::core::ffi::c_int;
    pub fn dax_pmem_test() -> ::core::ffi::c_int;
    pub fn dax_pmem_core_test() -> ::core::ffi::c_int;
    pub fn dax_pmem_compat_test() -> ::core::ffi::c_int;
}

/*
 * dummy routine for nfit_test to validate it is linking to the properly
 * mocked module and not the standard one from the base tree.
 */
macro_rules! nfit_test_watermark {
    ($x:ident) => {
        // TODO: C's `x##_test` token pasting needs an identifier-concatenation
        // facility at the call site to form and export the generated symbol.
        pub extern "C" fn $x() -> ::core::ffi::c_int {
            pr_debug!("%s for nfit_test\n", KBUILD_MODNAME);
            0
        }

        EXPORT_SYMBOL!($x);
    };
}
