// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */
/* Depends on Linux module/printk facilities from the original header. */

unsafe extern "C" {
    pub fn cxl_acpi_test() -> ::core::ffi::c_int;
    pub fn cxl_core_test() -> ::core::ffi::c_int;
    pub fn cxl_mem_test() -> ::core::ffi::c_int;
    pub fn cxl_pmem_test() -> ::core::ffi::c_int;
    pub fn cxl_port_test() -> ::core::ffi::c_int;
}

/*
 * dummy routine for cxl_test to validate it is linking to the properly
 * mocked module and not the standard one from the base tree.
 */
macro_rules! cxl_test_watermark {
    ($x:ident) => {
        paste::paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$x _test>]() -> ::core::ffi::c_int {
                pr_debug!("%s for cxl_test\n", KBUILD_MODNAME);
                0
            }

            /* EXPORT_SYMBOL([<$x _test>]) */
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
