// SPDX-License-Identifier: GPL-2.0-or-later
// C dependencies: tests/basic_api.h, tests/alloc_api.h,
// tests/alloc_helpers_api.h, tests/alloc_nid_api.h,
// tests/alloc_exact_nid_api.h, tests/common.h

use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_args(argc: c_int, argv: *mut *mut c_char);
    fn memblock_basic_checks();
    fn memblock_alloc_checks();
    fn memblock_alloc_helpers_checks();
    fn memblock_alloc_nid_checks();
    fn memblock_alloc_exact_nid_checks();
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_args(argc, argv);
    memblock_basic_checks();
    memblock_alloc_checks();
    memblock_alloc_helpers_checks();
    memblock_alloc_nid_checks();
    memblock_alloc_exact_nid_checks();

    0
}
