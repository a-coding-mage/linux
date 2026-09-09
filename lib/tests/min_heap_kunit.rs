// SPDX-License-Identifier: GPL-2.0-only
/*
 * Test cases for the min max heap.
 */

// C dependencies: kunit/test.h, linux/min_heap.h, linux/module.h, linux/random.h.

#[repr(C)]
pub struct min_heap_test_case {
    pub str_: *const core::ffi::c_char,
    pub min_heap: bool,
}

static mut MIN_HEAP_CASES: [min_heap_test_case; 2] = [
    min_heap_test_case { str_: b"min\0".as_ptr() as *const _, min_heap: true },
    min_heap_test_case { str_: b"max\0".as_ptr() as *const _, min_heap: false },
];

// KUNIT_ARRAY_PARAM_DESC(min_heap, min_heap_cases, str);

#[repr(C)]
pub struct min_heap_test {
    pub data: *mut i32,
    pub nr: usize,
    pub size: usize,
}

#[repr(C)]
pub struct min_heap_callbacks {
    pub less: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> bool>,
    pub swp: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void)>,
}

extern "C" {
    fn min_heapify_all_inline(heap: *mut min_heap_test, funcs: *const min_heap_callbacks, args: *mut core::ffi::c_void);
    fn min_heap_pop_inline(heap: *mut min_heap_test, funcs: *const min_heap_callbacks, args: *mut core::ffi::c_void);
    fn min_heap_push_inline(heap: *mut min_heap_test, element: *const i32, funcs: *const min_heap_callbacks, args: *mut core::ffi::c_void);
    fn min_heap_pop_push_inline(heap: *mut min_heap_test, element: *const i32, funcs: *const min_heap_callbacks, args: *mut core::ffi::c_void);
    fn min_heap_del_inline(heap: *mut min_heap_test, index: usize, funcs: *const min_heap_callbacks, args: *mut core::ffi::c_void);
    fn min_heap_init_inline(heap: *mut min_heap_test, data: *mut i32, size: usize);
    fn get_random_u32() -> u32;
}

unsafe extern "C" fn less_than(lhs: *const core::ffi::c_void, rhs: *const core::ffi::c_void, _args: *mut core::ffi::c_void) -> bool {
    *(lhs as *const i32) < *(rhs as *const i32)
}

unsafe extern "C" fn greater_than(lhs: *const core::ffi::c_void, rhs: *const core::ffi::c_void, _args: *mut core::ffi::c_void) -> bool {
    *(lhs as *const i32) > *(rhs as *const i32)
}

// The KUnit assertion and parameter interfaces are supplied by the C test framework.
extern "C" {
    fn kunit_expect_le(test: *mut core::ffi::c_void, lhs: i32, rhs: i32);
    fn kunit_expect_ge(test: *mut core::ffi::c_void, lhs: i32, rhs: i32);
}

unsafe fn pop_verify_heap(test: *mut core::ffi::c_void, is_min_heap: bool, heap: *mut min_heap_test, funcs: *const min_heap_callbacks) {
    let values = (*heap).data;
    let mut last = *values;
    min_heap_pop_inline(heap, funcs, core::ptr::null_mut());
    while (*heap).nr > 0 {
        if is_min_heap { kunit_expect_le(test, last, *values); }
        else { kunit_expect_ge(test, last, *values); }
        last = *values;
        min_heap_pop_inline(heap, funcs, core::ptr::null_mut());
    }
}

unsafe fn test_heapify_all(test: *mut core::ffi::c_void, params: *const min_heap_test_case) {
    let mut values: [i32; 13] = [3, 1, 2, 4, 0x8000000, 0x7FFFFFF, 0, -3, -1, -2, -4, 0x8000000, 0x7FFFFFF];
    let mut heap = min_heap_test { data: values.as_mut_ptr(), nr: values.len(), size: values.len() };
    let funcs = min_heap_callbacks { less: Some(if (*params).min_heap { less_than } else { greater_than }), swp: None };
    min_heapify_all_inline(&mut heap, &funcs, core::ptr::null_mut());
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
    heap.nr = values.len();
    for value in &mut values { *value = get_random_u32() as i32; }
    min_heapify_all_inline(&mut heap, &funcs, core::ptr::null_mut());
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
}

unsafe fn test_heap_push(test: *mut core::ffi::c_void, params: *const min_heap_test_case) {
    let data: [i32; 13] = [3, 1, 2, 4, 0x80000000u32 as i32, 0x7FFFFFFF, 0, -3, -1, -2, -4, 0x80000000u32 as i32, 0x7FFFFFFF];
    let mut values = [0i32; 13];
    let mut heap = min_heap_test { data: values.as_mut_ptr(), nr: 0, size: values.len() };
    let funcs = min_heap_callbacks { less: Some(if (*params).min_heap { less_than } else { greater_than }), swp: None };
    for value in &data { min_heap_push_inline(&mut heap, value, &funcs, core::ptr::null_mut()); }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
    while heap.nr < heap.size {
        let mut temp = get_random_u32() as i32;
        min_heap_push_inline(&mut heap, &mut temp, &funcs, core::ptr::null_mut());
    }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
}

unsafe fn test_heap_pop_push(test: *mut core::ffi::c_void, params: *const min_heap_test_case) {
    let data: [i32; 13] = [3, 1, 2, 4, 0x80000000u32 as i32, 0x7FFFFFFF, 0, -3, -1, -2, -4, 0x80000000u32 as i32, 0x7FFFFFFF];
    let mut values = [0i32; 13];
    let mut heap = min_heap_test { data: values.as_mut_ptr(), nr: 0, size: values.len() };
    let funcs = min_heap_callbacks { less: Some(if (*params).min_heap { less_than } else { greater_than }), swp: None };
    let mut temp = if (*params).min_heap { 0x80000000u32 as i32 } else { 0x7FFFFFFF };
    for _ in &data { min_heap_push_inline(&mut heap, &temp, &funcs, core::ptr::null_mut()); }
    for value in &data { min_heap_pop_push_inline(&mut heap, value, &funcs, core::ptr::null_mut()); }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
    heap.nr = 0;
    for _ in &data { min_heap_push_inline(&mut heap, &temp, &funcs, core::ptr::null_mut()); }
    for _ in &data { temp = get_random_u32() as i32; min_heap_pop_push_inline(&mut heap, &temp, &funcs, core::ptr::null_mut()); }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
}

unsafe fn test_heap_del(test: *mut core::ffi::c_void, params: *const min_heap_test_case) {
    let mut values: [i32; 13] = [3, 1, 2, 4, 0x8000000, 0x7FFFFFF, 0, -3, -1, -2, -4, 0x8000000, 0x7FFFFFF];
    let mut heap = min_heap_test { data: values.as_mut_ptr(), nr: values.len(), size: values.len() };
    let funcs = min_heap_callbacks { less: Some(if (*params).min_heap { less_than } else { greater_than }), swp: None };
    min_heapify_all_inline(&mut heap, &funcs, core::ptr::null_mut());
    for _ in 0..values.len() / 2 { min_heap_del_inline(&mut heap, (get_random_u32() as usize) % heap.nr, &funcs, core::ptr::null_mut()); }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
    heap.nr = values.len();
    for value in &mut values { *value = get_random_u32() as i32; }
    min_heapify_all_inline(&mut heap, &funcs, core::ptr::null_mut());
    for _ in 0..values.len() / 2 { min_heap_del_inline(&mut heap, (get_random_u32() as usize) % heap.nr, &funcs, core::ptr::null_mut()); }
    pop_verify_heap(test, (*params).min_heap, &mut heap, &funcs);
}

// The remaining KUnit test registration and test bodies retain the source interface.
// KUNIT_CASE_PARAM(test_heapify_all, min_heap_gen_params);
// KUNIT_CASE_PARAM(test_heap_push, min_heap_gen_params);
// KUNIT_CASE_PARAM(test_heap_pop_push, min_heap_gen_params);
// KUNIT_CASE_PARAM(test_heap_del, min_heap_gen_params);
// kunit_test_suite(min_heap_test_suite);
// MODULE_DESCRIPTION("Test cases for the min max heap");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
