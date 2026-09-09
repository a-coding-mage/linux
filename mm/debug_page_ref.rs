// SPDX-License-Identifier: GPL-2.0

// External kernel type and tracepoint functions supplied by other translation units.
pub struct page;

unsafe extern "C" {
    fn trace_page_ref_set(page: *mut page, v: core::ffi::c_int);
    fn trace_page_ref_mod(page: *mut page, v: core::ffi::c_int);
    fn trace_page_ref_mod_and_test(
        page: *mut page,
        v: core::ffi::c_int,
        ret: core::ffi::c_int,
    );
    fn trace_page_ref_mod_and_return(
        page: *mut page,
        v: core::ffi::c_int,
        ret: core::ffi::c_int,
    );
    fn trace_page_ref_mod_unless(
        page: *mut page,
        v: core::ffi::c_int,
        u: core::ffi::c_int,
    );
    fn trace_page_ref_freeze(
        page: *mut page,
        v: core::ffi::c_int,
        ret: core::ffi::c_int,
    );
    fn trace_page_ref_unfreeze(page: *mut page, v: core::ffi::c_int);
}

// EXPORT_SYMBOL(__page_ref_set);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_set);
pub unsafe fn __page_ref_set(page: *mut page, v: core::ffi::c_int) {
    unsafe { trace_page_ref_set(page, v) };
}

// EXPORT_SYMBOL(__page_ref_mod);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_mod);
pub unsafe fn __page_ref_mod(page: *mut page, v: core::ffi::c_int) {
    unsafe { trace_page_ref_mod(page, v) };
}

// EXPORT_SYMBOL(__page_ref_mod_and_test);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_and_test);
pub unsafe fn __page_ref_mod_and_test(
    page: *mut page,
    v: core::ffi::c_int,
    ret: core::ffi::c_int,
) {
    unsafe { trace_page_ref_mod_and_test(page, v, ret) };
}

// EXPORT_SYMBOL(__page_ref_mod_and_return);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_and_return);
pub unsafe fn __page_ref_mod_and_return(
    page: *mut page,
    v: core::ffi::c_int,
    ret: core::ffi::c_int,
) {
    unsafe { trace_page_ref_mod_and_return(page, v, ret) };
}

// EXPORT_SYMBOL(__page_ref_mod_unless);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_unless);
pub unsafe fn __page_ref_mod_unless(
    page: *mut page,
    v: core::ffi::c_int,
    u: core::ffi::c_int,
) {
    unsafe { trace_page_ref_mod_unless(page, v, u) };
}

// EXPORT_SYMBOL(__page_ref_freeze);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_freeze);
pub unsafe fn __page_ref_freeze(
    page: *mut page,
    v: core::ffi::c_int,
    ret: core::ffi::c_int,
) {
    unsafe { trace_page_ref_freeze(page, v, ret) };
}

// EXPORT_SYMBOL(__page_ref_unfreeze);
// EXPORT_TRACEPOINT_SYMBOL(page_ref_unfreeze);
pub unsafe fn __page_ref_unfreeze(page: *mut page, v: core::ffi::c_int) {
    unsafe { trace_page_ref_unfreeze(page, v) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
