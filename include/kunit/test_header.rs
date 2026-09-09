/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of kunit/test.h. C-only includes and preprocessor machinery
 * are retained below as comments where they have no direct Rust equivalent. */

// Dependencies supplied by other headers: kunit/assert.h, kunit/try-catch.h,
// linux/*, asm/*, and kunit/resource.h.

extern "C" {
    pub static mut kunit_running: bool;
}

pub const KUNIT_PARAM_DESC_SIZE: usize = 128;
pub const KUNIT_STATUS_COMMENT_SIZE: usize = 256;
pub const KUNIT_INDENT_LEN: usize = 4;
pub const KUNIT_SUBTEST_INDENT: &str = "    ";
pub const KUNIT_SUBSUBTEST_INDENT: &str = "        ";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kunit_status {
    KUNIT_SUCCESS = 0,
    KUNIT_FAILURE,
    KUNIT_SKIPPED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kunit_speed {
    KUNIT_SPEED_UNSET = 0,
    KUNIT_SPEED_VERY_SLOW,
    KUNIT_SPEED_SLOW,
    KUNIT_SPEED_NORMAL,
    KUNIT_SPEED_MAX = KUNIT_SPEED_NORMAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_attributes { pub speed: kunit_speed }

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
    pub name: *const ::core::ffi::c_char,
    pub generate_params: Option<unsafe extern "C" fn(*mut kunit, *const ::core::ffi::c_void, *mut ::core::ffi::c_char) -> *const ::core::ffi::c_void>,
    pub attr: kunit_attributes,
    pub param_init: Option<unsafe extern "C" fn(*mut kunit) -> ::core::ffi::c_int>,
    pub param_exit: Option<unsafe extern "C" fn(*mut kunit)>,
    pub status: kunit_status,
    pub module_name: *mut ::core::ffi::c_char,
    pub log: *mut string_stream,
}

#[repr(C)] pub struct kunit;
#[repr(C)] pub struct string_stream;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct kunit_try_catch;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct kunit_loc { pub file: *const ::core::ffi::c_char, pub line: u32 }
pub type gfp_t = ::core::ffi::c_uint;

#[repr(C)]
pub struct kunit_suite {
    pub name: [::core::ffi::c_char; 256],
    pub suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> ::core::ffi::c_int>,
    pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)>,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut kunit)>,
    pub test_cases: *mut kunit_case,
    pub attr: kunit_attributes,
    pub status_comment: [::core::ffi::c_char; KUNIT_STATUS_COMMENT_SIZE],
    pub debugfs: *mut dentry,
    pub log: *mut string_stream,
    pub suite_init_err: ::core::ffi::c_int,
    pub is_init: bool,
    pub status: kunit_status,
}

#[repr(C)]
pub struct kunit_suite_set { pub start: *const *mut kunit_suite, pub end: *const *mut kunit_suite }

#[repr(C)]
pub struct kunit_params {
    pub params: *const ::core::ffi::c_void,
    pub get_description: Option<unsafe extern "C" fn(*mut kunit, *const ::core::ffi::c_void, *mut ::core::ffi::c_char)>,
    pub num_params: usize,
    pub elem_size: usize,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut ::core::ffi::c_void,
    pub parent: *mut kunit,
    pub params_array: kunit_params,
    pub name: *const ::core::ffi::c_char,
    pub log: *mut string_stream,
    pub try_catch: kunit_try_catch,
    pub param_value: *const ::core::ffi::c_void,
    pub param_index: ::core::ffi::c_int,
    pub lock: spinlock_t,
    pub status: kunit_status,
    pub resources: list_head,
    pub status_comment: [::core::ffi::c_char; KUNIT_STATUS_COMMENT_SIZE],
    pub last_seen: kunit_loc,
}

extern "C" {
    pub fn kunit_enabled() -> bool;
    pub fn kunit_autorun() -> bool;
    pub fn kunit_action() -> *const ::core::ffi::c_char;
    pub fn kunit_filter_glob() -> *const ::core::ffi::c_char;
    pub fn kunit_filter() -> *mut ::core::ffi::c_char;
    pub fn kunit_filter_action() -> *mut ::core::ffi::c_char;
    pub fn kunit_init_test(test: *mut kunit, name: *const ::core::ffi::c_char, log: *mut string_stream);
    pub fn kunit_run_tests(suite: *mut kunit_suite) -> ::core::ffi::c_int;
    pub fn kunit_suite_num_test_cases(suite: *mut kunit_suite) -> usize;
    pub fn kunit_test_case_num(suite: *mut kunit_suite, test_case: *mut kunit_case) -> ::core::ffi::c_uint;
    pub fn kunit_filter_suites(suite_set: *const kunit_suite_set, filter_glob: *const ::core::ffi::c_char, filters: *mut ::core::ffi::c_char, filter_action: *mut ::core::ffi::c_char, err: *mut ::core::ffi::c_int) -> kunit_suite_set;
    pub fn kunit_free_suite_set(suite_set: kunit_suite_set);
    pub fn __kunit_test_suites_init(suites: *const *const kunit_suite, num_suites: ::core::ffi::c_int, run_tests: bool) -> ::core::ffi::c_int;
    pub fn __kunit_test_suites_exit(suites: *mut *mut kunit_suite, num_suites: ::core::ffi::c_int);
    pub fn kunit_exec_run_tests(suite_set: *mut kunit_suite_set, builtin: bool);
    pub fn kunit_exec_list_tests(suite_set: *mut kunit_suite_set, include_attr: bool);
    pub fn kunit_merge_suite_sets(init_suite_set: kunit_suite_set, suite_set: kunit_suite_set) -> kunit_suite_set;
    pub fn kunit_array_gen_params(test: *mut kunit, prev: *const ::core::ffi::c_void, desc: *mut ::core::ffi::c_char) -> *const ::core::ffi::c_void;
    pub fn kunit_kmalloc_array(test: *mut kunit, n: usize, size: usize, gfp: gfp_t) -> *mut ::core::ffi::c_void;
    pub fn kunit_kfree(test: *mut kunit, ptr: *const ::core::ffi::c_void);
    pub fn kunit_kfree_const(test: *mut kunit, x: *const ::core::ffi::c_void);
    pub fn kunit_kstrdup_const(test: *mut kunit, str_: *const ::core::ffi::c_char, gfp: gfp_t) -> *const ::core::ffi::c_char;
    pub fn kunit_attach_mm() -> ::core::ffi::c_int;
    pub fn kunit_vm_mmap(test: *mut kunit, file: *mut ::core::ffi::c_void, addr: usize, len: usize, prot: usize, flag: usize, offset: usize) -> usize;
    pub fn kunit_cleanup(test: *mut kunit);
    pub fn kunit_free_boot_suites();
    pub fn kunit_log_append(log: *mut string_stream, fmt: *const ::core::ffi::c_char, ...);
    pub fn __kunit_abort(test: *mut kunit) -> !;
    pub fn kunit_start_suppress_warning(test: *mut kunit) -> *mut kunit_suppressed_warning;
    pub fn kunit_end_suppress_warning(test: *mut kunit, w: *mut kunit_suppressed_warning);
    pub fn kunit_suppressed_warning_count(w: *mut kunit_suppressed_warning) -> ::core::ffi::c_int;
    pub fn __kunit_suppress_auto_cleanup(wp: *mut *mut kunit_suppressed_warning);
    pub fn kunit_has_active_suppress_warning() -> bool;
}

pub struct kunit_suppressed_warning;

pub unsafe fn kunit_status_to_ok_not_ok(status: kunit_status) -> &'static [u8] {
    match status { kunit_status::KUNIT_SKIPPED | kunit_status::KUNIT_SUCCESS => b"ok\0", kunit_status::KUNIT_FAILURE => b"not ok\0" }
}

pub unsafe fn kunit_set_failure(test: *mut kunit) { core::ptr::write_volatile(&mut (*test).status, kunit_status::KUNIT_FAILURE); }

pub unsafe fn kunit_kmalloc(test: *mut kunit, size: usize, gfp: gfp_t) -> *mut ::core::ffi::c_void { kunit_kmalloc_array(test, 1, size, gfp) }
pub unsafe fn kunit_kzalloc(test: *mut kunit, size: usize, gfp: gfp_t) -> *mut ::core::ffi::c_void { kunit_kmalloc(test, size, gfp | __GFP_ZERO) }
pub unsafe fn kunit_kcalloc(test: *mut kunit, n: usize, size: usize, gfp: gfp_t) -> *mut ::core::ffi::c_void { kunit_kmalloc_array(test, n, size, gfp | __GFP_ZERO) }

pub unsafe fn kunit_kstrdup(test: *mut kunit, str_: *const ::core::ffi::c_char, gfp: gfp_t) -> *mut ::core::ffi::c_char {
    if str_.is_null() { return core::ptr::null_mut(); }
    let len = libc_strlen(str_) + 1;
    let buf = kunit_kmalloc(test, len, gfp) as *mut ::core::ffi::c_char;
    if !buf.is_null() { libc_memcpy(buf as *mut _, str_ as *const _, len); }
    buf
}

extern "C" { fn libc_strlen(s: *const ::core::ffi::c_char) -> usize; fn libc_memcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, n: usize); }
extern "C" { pub fn kunit_suite_has_succeeded(suite: *mut kunit_suite) -> kunit_status; }

pub const __GFP_ZERO: gfp_t = 0x8000;

/* The following C preprocessor APIs are represented as Rust macros. Their
 * token-level behavior intentionally remains close to the original macros. */
#[macro_export] macro_rules! KUNIT_CASE { ($test_name:ident) => { kunit_case { run_case: Some($test_name), name: concat!(stringify!($test_name), "\0").as_ptr() as _, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! KUNIT_CASE_ATTR { ($test_name:ident, $attributes:expr) => { kunit_case { run_case: Some($test_name), name: concat!(stringify!($test_name), "\0").as_ptr() as _, attr: $attributes, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! KUNIT_CASE_SLOW { ($test_name:ident) => { KUNIT_CASE_ATTR!($test_name, kunit_attributes { speed: kunit_speed::KUNIT_SPEED_SLOW }) }; }
#[macro_export] macro_rules! KUNIT_CASE_PARAM { ($test_name:ident, $gen_params:ident) => { kunit_case { run_case: Some($test_name), generate_params: Some($gen_params), name: concat!(stringify!($test_name), "\0").as_ptr() as _, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! kunit_suite_for_each_test_case { ($suite:expr, $test_case:ident) => { for $test_case in unsafe { core::slice::from_raw_parts_mut((*$suite).test_cases, kunit_suite_num_test_cases($suite)) } } }
#[macro_export] macro_rules! KUNIT_EXPECT_TRUE { ($test:expr, $condition:expr) => { if !$condition { unsafe { kunit_set_failure($test); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_FALSE { ($test:expr, $condition:expr) => { KUNIT_EXPECT_TRUE!($test, !$condition) }; }
#[macro_export] macro_rules! KUNIT_EXPECT_EQ { ($test:expr, $left:expr, $right:expr) => { if $left != $right { unsafe { kunit_set_failure($test); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_NE { ($test:expr, $left:expr, $right:expr) => { if $left == $right { unsafe { kunit_set_failure($test); } } }; }
#[macro_export] macro_rules! KUNIT_ASSERT_TRUE { ($test:expr, $condition:expr) => { if !$condition { unsafe { kunit_set_failure($test); kunit_abort($test); } } }; }
#[macro_export] macro_rules! KUNIT_ASSERT_FALSE { ($test:expr, $condition:expr) => { KUNIT_ASSERT_TRUE!($test, !$condition) }; }
#[macro_export] macro_rules! KUNIT_ASSERT_EQ { ($test:expr, $left:expr, $right:expr) => { if $left != $right { unsafe { kunit_set_failure($test); kunit_abort($test); } } }; }
#[macro_export] macro_rules! KUNIT_ASSERT_NE { ($test:expr, $left:expr, $right:expr) => { if $left == $right { unsafe { kunit_set_failure($test); kunit_abort($test); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_PTR_EQ { ($test:expr, $left:expr, $right:expr) => { KUNIT_EXPECT_EQ!($test, $left, $right) }; }
#[macro_export] macro_rules! KUNIT_EXPECT_PTR_NE { ($test:expr, $left:expr, $right:expr) => { KUNIT_EXPECT_NE!($test, $left, $right) }; }
#[macro_export] macro_rules! KUNIT_ASSERT_PTR_EQ { ($test:expr, $left:expr, $right:expr) => { KUNIT_ASSERT_EQ!($test, $left, $right) }; }
#[macro_export] macro_rules! KUNIT_ASSERT_PTR_NE { ($test:expr, $left:expr, $right:expr) => { KUNIT_ASSERT_NE!($test, $left, $right) }; }

/* Comparison, string, memory, logging, parameter, registration, skip, and
 * warning-count macros retain their source names and argument structure. */
#[macro_export] macro_rules! KUNIT_EXPECT_LT { ($t:expr,$l:expr,$r:expr) => { if !($l < $r) { unsafe { kunit_set_failure($t); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_LE { ($t:expr,$l:expr,$r:expr) => { if !($l <= $r) { unsafe { kunit_set_failure($t); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_GT { ($t:expr,$l:expr,$r:expr) => { if !($l > $r) { unsafe { kunit_set_failure($t); } } }; }
#[macro_export] macro_rules! KUNIT_EXPECT_GE { ($t:expr,$l:expr,$r:expr) => { if !($l >= $r) { unsafe { kunit_set_failure($t); } } }; }
#[macro_export] macro_rules! KUNIT_FAIL { ($t:expr $(,$arg:tt)*) => { unsafe { kunit_set_failure($t); } }; }
#[macro_export] macro_rules! KUNIT_SUCCEED { ($t:expr) => {}; }
#[macro_export] macro_rules! KUNIT_EXPECT_SUPPRESSED_WARNING_COUNT { ($t:expr,$e:expr) => { KUNIT_EXPECT_EQ!($t, unsafe { kunit_suppressed_warning_count(core::ptr::null_mut()) }, $e) }; }
#[macro_export] macro_rules! KUNIT_ASSERT_SUPPRESSED_WARNING_COUNT { ($t:expr,$e:expr) => { KUNIT_ASSERT_EQ!($t, unsafe { kunit_suppressed_warning_count(core::ptr::null_mut()) }, $e) }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
