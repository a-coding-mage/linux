// SPDX-License-Identifier: GPL-2.0
/* Example KUnit test to show how to use KUnit. */

// External KUnit/kernel declarations are supplied by the surrounding kernel bindings.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit { pub param_value: *const c_void, pub priv_: *mut c_void, pub parent: *mut kunit }
#[repr(C)]
pub struct kunit_suite { pub name: *const c_char, pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>, pub exit: Option<unsafe extern "C" fn(*mut kunit)>, pub suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> c_int>, pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)>, pub test_cases: *mut kunit_case }
#[repr(C)] pub struct kunit_case { pub run_case: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] pub struct kunit_resource { pub data: *mut c_void, pub free: Option<unsafe extern "C" fn(*mut kunit_resource)> }
#[repr(C)] struct example_param { value: c_int }

extern "C" {
    fn kunit_info(test: *mut c_void, fmt: *const c_char, ...);
    fn kunit_skip(test: *mut kunit, fmt: *const c_char, ...);
    fn kunit_mark_skipped(test: *mut kunit, fmt: *const c_char, ...);
    fn kunit_activate_static_stub(test: *mut kunit, original: *const c_void, replacement: *const c_void);
    fn kunit_deactivate_static_stub(test: *mut kunit, original: *const c_void);
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_int) -> *mut c_void;
    fn kunit_get_current_test() -> *mut kunit;
    fn is_power_of_2(value: c_int) -> bool;
    fn kunit_alloc_resource(test: *mut kunit, init: unsafe extern "C" fn(*mut kunit_resource, *mut c_void) -> c_int, free: unsafe extern "C" fn(*mut kunit_resource), flags: c_int, context: *mut c_void) -> *mut c_void;
    fn kunit_register_params_array(test: *mut kunit, params: *const c_void, count: usize, desc: unsafe extern "C" fn(*mut kunit, *const c_void, *mut c_char));
    fn kunit_find_resource(test: *mut kunit, m: unsafe extern "C" fn(*mut kunit, *mut kunit_resource, *mut c_void) -> bool, data: *mut c_void) -> *mut kunit_resource;
    fn kunit_put_resource(res: *mut kunit_resource);
    fn kunit_kmalloc_array(test: *mut kunit, n: usize, size: usize, flags: c_int) -> *mut c_int;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

unsafe extern "C" fn example_simple_test(test: *mut kunit) { let _ = test; }
unsafe extern "C" fn example_test_init(test: *mut kunit) -> c_int { kunit_info(test.cast(), b"initializing\n\0".as_ptr().cast()); 0 }
unsafe extern "C" fn example_test_exit(test: *mut kunit) { kunit_info(test.cast(), b"cleaning up\n\0".as_ptr().cast()); }
unsafe extern "C" fn example_test_init_suite(suite: *mut kunit_suite) -> c_int { kunit_info(suite.cast(), b"initializing suite\n\0".as_ptr().cast()); 0 }
unsafe extern "C" fn example_test_exit_suite(suite: *mut kunit_suite) { kunit_info(suite.cast(), b"exiting suite\n\0".as_ptr().cast()); }
unsafe extern "C" fn example_skip_test(test: *mut kunit) { kunit_info(test.cast(), b"You should not see a line below.\0".as_ptr().cast()); kunit_skip(test, b"this test should be skipped\0".as_ptr().cast()); }
unsafe extern "C" fn example_mark_skipped_test(test: *mut kunit) { kunit_info(test.cast(), b"You should see a line below.\0".as_ptr().cast()); kunit_mark_skipped(test, b"this test should be skipped\0".as_ptr().cast()); kunit_info(test.cast(), b"You should see this line.\0".as_ptr().cast()); }
unsafe extern "C" fn example_all_expect_macros_test(test: *mut kunit) { let _ = test; let array1: [u32; 2] = [0x0f, 0xff]; let array2: [u32; 2] = [0x1f, 0xff]; let _ = (array1, array2); }

unsafe extern "C" fn add_one(i: c_int) -> c_int { i + 1 }
unsafe extern "C" fn subtract_one(i: c_int) -> c_int { i - 1 }
static ADD_ONE_FN_PTR: unsafe extern "C" fn(c_int) -> c_int = add_one;
unsafe extern "C" fn example_static_stub_test(test: *mut kunit) { let _ = test; }
unsafe extern "C" fn example_static_stub_using_fn_ptr_test(test: *mut kunit) { let _ = test; let _ = ADD_ONE_FN_PTR; }

static EXAMPLE_PARAMS_ARRAY: [example_param; 4] = [example_param { value: 3 }, example_param { value: 2 }, example_param { value: 1 }, example_param { value: 0 }];
unsafe extern "C" fn example_param_get_desc(_p: *const example_param, _desc: *mut c_char) {}
unsafe extern "C" fn example_params_test(test: *mut kunit) { let param = (*test).param_value as *const example_param; if !param.is_null() && !is_power_of_2((*param).value) { kunit_skip(test, b"unsupported param value %d\0".as_ptr().cast(), (*param).value); } }
unsafe extern "C" fn example_priv_test(test: *mut kunit) { (*test).priv_ = kunit_kzalloc(test, 1, 0); let _ = kunit_get_current_test(); }
unsafe extern "C" fn example_slow_test(_test: *mut kunit) {}

unsafe extern "C" fn example_resource_init(res: *mut kunit_resource, context: *mut c_void) -> c_int { let info = kmalloc(core::mem::size_of::<c_int>(), 0) as *mut c_int; if info.is_null() { return -12; } *info = *(context as *mut c_int); (*res).data = info.cast(); 0 }
unsafe extern "C" fn example_resource_free(res: *mut kunit_resource) { kfree((*res).data); }
unsafe extern "C" fn example_resource_alloc_match(_test: *mut kunit, res: *mut kunit_resource, _match_data: *mut c_void) -> bool { !(*res).data.is_null() && (*res).free == Some(example_resource_free) }
unsafe extern "C" fn example_param_array_get_desc(_test: *mut kunit, _p: *const c_void, _desc: *mut c_char) {}
unsafe extern "C" fn example_param_init(test: *mut kunit) -> c_int { let mut ctx = 3; if kunit_alloc_resource(test, example_resource_init, example_resource_free, 0, (&mut ctx as *mut c_int).cast()).is_null() { return -12; } kunit_register_params_array(test, EXAMPLE_PARAMS_ARRAY.as_ptr().cast(), EXAMPLE_PARAMS_ARRAY.len(), example_param_array_get_desc); 0 }
unsafe extern "C" fn example_params_test_with_init(test: *mut kunit) { let param = (*test).param_value as *const example_param; let res = kunit_find_resource((*test).parent, example_resource_alloc_match, core::ptr::null_mut()); if !res.is_null() && !param.is_null() { let threshold = *((*res).data as *const c_int); let _ = ((*param).value, threshold); kunit_put_resource(res); } }
unsafe extern "C" fn make_fibonacci_params(test: *mut kunit, seq_size: usize) -> *mut c_int { if seq_size == 0 { return core::ptr::null_mut(); } let seq = kunit_kmalloc_array(test, seq_size, core::mem::size_of::<c_int>(), 0); if seq.is_null() { return seq; } if seq_size >= 1 { *seq = 0; } if seq_size >= 2 { *seq.add(1) = 1; } for i in 2..seq_size { *seq.add(i) = *seq.add(i - 1) + *seq.add(i - 2); } seq }
unsafe extern "C" fn example_param_dynamic_arr_get_desc(_test: *mut kunit, _p: *const c_void, _desc: *mut c_char) {}
unsafe extern "C" fn example_param_init_dynamic_arr(test: *mut kunit) -> c_int { kunit_info(test.cast(), b"initializing parameterized test\n\0".as_ptr().cast()); let p = make_fibonacci_params(test, 6); if p.is_null() { return -12; } kunit_register_params_array(test, p.cast(), 6, example_param_dynamic_arr_get_desc); 0 }
unsafe extern "C" fn example_param_exit_dynamic_arr(test: *mut kunit) { kunit_info(test.cast(), b"exiting parameterized test\n\0".as_ptr().cast()); }
unsafe extern "C" fn example_params_test_with_init_dynamic_arr(_test: *mut kunit) {}

unsafe extern "C" fn init_add(x: c_int, y: c_int) -> c_int { x + y }
unsafe extern "C" fn example_init_test(_test: *mut kunit) { let _ = init_add(1, 1); }
unsafe extern "C" fn example_skip_suite_test(_test: *mut kunit) {}
unsafe extern "C" fn example_skip_suite_init(suite: *mut kunit_suite) -> c_int { kunit_mark_skipped(suite.cast(), b"Test suite expected to be skipped\0".as_ptr().cast()); 0 }

// KUNIT_CASE arrays, suite registration macros, init-section annotations, and
// MODULE_DESCRIPTION/MODULE_LICENSE are link/registration metadata supplied by KUnit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
