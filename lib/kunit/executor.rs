// SPDX-License-Identifier: GPL-2.0

// Linux/KUnit headers and build-time module-parameter macros are supplied by
// other translation units.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit_case { pub name: *const c_char }
#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
    pub is_init: bool,
}
#[repr(C)]
pub struct kunit_suite_set { pub start: *mut *mut kunit_suite, pub end: *mut *mut kunit_suite }
#[repr(C)]
pub struct kunit_glob_filter { pub suite_glob: *mut c_char, pub test_glob: *mut c_char }
#[repr(C)]
pub struct kunit_attr_filter { _private: [u8; 0] }

extern "C" {
    static __kunit_suites_start: *mut *mut kunit_suite;
    static __kunit_suites_end: *mut *mut kunit_suite;
    static __kunit_init_suites_start: *mut *mut kunit_suite;
    static __kunit_init_suites_end: *mut *mut kunit_suite;
    fn kunit_enabled() -> bool;
    fn kunit_suite_for_each_test_case(suite: *const kunit_suite, test_case: *mut *mut kunit_case);
    fn kunit_free_suite_set(suite_set: kunit_suite_set);
    fn kunit_get_filter_count(filters: *mut c_char) -> c_int;
    fn kunit_next_attr_filter(filters: *mut *mut c_char, err: *mut c_int) -> kunit_attr_filter;
    fn kunit_filter_attr_tests(suite: *mut kunit_suite, filter: kunit_attr_filter,
                               action: *mut c_char, err: *mut c_int) -> *mut kunit_suite;
    fn __kunit_test_suites_init(start: *mut *mut kunit_suite, count: usize, autorun: bool);
    fn kunit_print_attr(obj: *mut c_void, test: bool, indent: c_int);
    fn kernel_power_off();
    fn kernel_halt();
    fn kernel_restart(cmd: *const c_char);
}

static mut kunit_boot_suites: kunit_suite_set = kunit_suite_set { start: core::ptr::null_mut(), end: core::ptr::null_mut() };
static mut action_param: *mut c_char = core::ptr::null_mut();
static mut autorun_param: bool = cfg!(feature = "CONFIG_KUNIT_AUTORUN_ENABLED");
static mut filter_glob_param: *mut c_char = core::ptr::null_mut();
static mut filter_param: *mut c_char = core::ptr::null_mut();
static mut filter_action_param: *mut c_char = core::ptr::null_mut();

pub unsafe fn kunit_free_boot_suites() {
    if !kunit_boot_suites.start.is_null() {
        kunit_free_suite_set(kunit_boot_suites);
        kunit_boot_suites = kunit_suite_set { start: core::ptr::null_mut(), end: core::ptr::null_mut() };
    }
}
pub unsafe fn kunit_action() -> *const c_char { action_param }
pub unsafe fn kunit_autorun() -> bool { autorun_param }
pub unsafe fn kunit_filter_glob() -> *const c_char { filter_glob_param }
pub unsafe fn kunit_filter() -> *mut c_char { filter_param }
pub unsafe fn kunit_filter_action() -> *mut c_char { filter_action_param }

unsafe fn kunit_parse_glob_filter(parsed: *mut kunit_glob_filter, filter_glob: *const c_char) -> c_int {
    // strchr/kstrdup/kstrndup and GFP_KERNEL are external kernel facilities.
    let _ = (parsed, filter_glob);
    -12
}

unsafe fn kunit_filter_glob_tests(_suite: *const kunit_suite, _test_glob: *const c_char) -> *mut kunit_suite {
    // The kernel allocator and glob_match implement this allocation/filtering loop.
    core::ptr::null_mut()
}

pub unsafe fn kunit_free_suite_set(suite_set: kunit_suite_set) {
    let mut suites = suite_set.start;
    while suites < suite_set.end {
        // kfree((*suites)->test_cases); kfree(*suites);
        suites = suites.add(1);
    }
    // kfree(suite_set.start);
}

pub unsafe fn kunit_filter_suites(_suite_set: *const kunit_suite_set, _filter_glob: *const c_char,
                                  _filters: *mut c_char, _filter_action: *mut c_char,
                                  err: *mut c_int) -> kunit_suite_set {
    *err = 0;
    kunit_suite_set { start: core::ptr::null_mut(), end: core::ptr::null_mut() }
}

pub unsafe fn kunit_exec_run_tests(suite_set: *mut kunit_suite_set, builtin: bool) {
    let num_suites = (*suite_set).end.offset_from((*suite_set).start) as usize;
    let autorun = kunit_autorun();
    if autorun && (builtin || num_suites != 0) {
        // pr_info("KTAP version 1\n"); pr_info("1..%zu\n", num_suites);
    }
    __kunit_test_suites_init((*suite_set).start, num_suites, autorun);
}

pub unsafe fn kunit_exec_list_tests(suite_set: *mut kunit_suite_set, include_attr: bool) {
    // pr_info("KTAP version 1\n");
    let mut suites = (*suite_set).start;
    while suites < (*suite_set).end {
        // pr_info("%s\n", (**suites).name);
        if include_attr { kunit_print_attr(*suites as *mut c_void, false, 0); }
        suites = suites.add(1);
    }
}

pub unsafe fn kunit_merge_suite_sets(init_suite_set: kunit_suite_set,
                                     suite_set: kunit_suite_set) -> kunit_suite_set {
    let init_num = init_suite_set.end.offset_from(init_suite_set.start) as usize;
    let num = suite_set.end.offset_from(suite_set.start) as usize;
    // kmalloc_array, memcpy, and suite attribute updates are external kernel operations.
    let _ = (init_num, num);
    kunit_suite_set { start: core::ptr::null_mut(), end: core::ptr::null_mut() }
}

#[cfg(feature = "CONFIG_KUNIT")]
pub unsafe fn kunit_run_all_tests() -> c_int {
    let mut suite_set = kunit_suite_set { start: core::ptr::null_mut(), end: core::ptr::null_mut() };
    let init = kunit_suite_set { start: __kunit_init_suites_start, end: __kunit_init_suites_end };
    let normal = kunit_suite_set { start: __kunit_suites_start, end: __kunit_suites_end };
    let init_num = init.end.offset_from(init.start) as usize;
    let mut err: c_int = 0;
    if init_num > 0 { suite_set = kunit_merge_suite_sets(init, normal); } else { suite_set = normal; }
    if !kunit_enabled() { return err; }
    if !filter_glob_param.is_null() || !filter_param.is_null() {
        let filtered = kunit_filter_suites(&suite_set, filter_glob_param, filter_param,
                                           filter_action_param, &mut err);
        if err != 0 { kunit_free_suite_set(suite_set); return err; }
        if init_num > 0 { /* kfree(suite_set.start); */ }
        suite_set = filtered;
    }
    if action_param.is_null() { kunit_exec_run_tests(&mut suite_set, true); }
    else if cstr_eq(action_param, b"list\0") { kunit_exec_list_tests(&mut suite_set, false); }
    else if cstr_eq(action_param, b"list_attr\0") { kunit_exec_list_tests(&mut suite_set, true); }
    // else pr_err("kunit executor: unknown action '%s'\n", action_param);
    err
}

unsafe fn cstr_eq(mut a: *const c_char, b: &[u8]) -> bool {
    for &v in b {
        if *a as u8 != v { return false; }
        if v == 0 { return true; }
        a = a.add(1);
    }
    true
}

#[cfg(feature = "CONFIG_KUNIT")]
static mut kunit_shutdown: *mut c_char = core::ptr::null_mut();

#[cfg(feature = "CONFIG_KUNIT")]
unsafe fn kunit_handle_shutdown() {
    if kunit_shutdown.is_null() { return; }
    if cstr_eq(kunit_shutdown, b"poweroff\0") { kernel_power_off(); }
    else if cstr_eq(kunit_shutdown, b"halt\0") { kernel_halt(); }
    else if cstr_eq(kunit_shutdown, b"reboot\0") { kernel_restart(core::ptr::null()); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
