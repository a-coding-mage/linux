// SPDX-License-Identifier: GPL-2.0
/* Base unit test (KUnit) API. */

// C headers and kernel-provided symbols are supplied by the surrounding crate.

static mut ENABLE_PARAM: bool = true;
static mut KUNIT_BASE_TIMEOUT: ::std::os::raw::c_ulong = CONFIG_KUNIT_DEFAULT_TIMEOUT;
static mut KUNIT_STATS_ENABLED: ::std::os::raw::c_int = 1;

#[repr(C)]
pub struct kunit_result_stats { pub passed: usize, pub skipped: usize, pub failed: usize, pub total: usize }

unsafe fn kunit_should_print_stats(stats: *mut kunit_result_stats) -> bool {
    if KUNIT_STATS_ENABLED == 0 { return false; }
    if KUNIT_STATS_ENABLED == 2 { return true; }
    (*stats).total > 1
}

unsafe fn kunit_print_test_stats(test: *mut kunit, stats: *mut kunit_result_stats) {
    if !kunit_should_print_stats(stats) { return; }
    kunit_log(KERN_INFO, test, KUNIT_SUBTEST_INDENT "# %s: pass:%lu fail:%lu skip:%lu total:%lu", (*test).name, (*stats).passed, (*stats).failed, (*stats).skipped, (*stats).total);
}

pub unsafe extern "C" fn __kunit_fail_current_test_impl(file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int, fmt: *const ::std::os::raw::c_char, mut args: ...) {
    if (*current).kunit_test.is_null() { return; }
    kunit_set_failure((*current).kunit_test);
    let mut ap: va_list = va_list::new();
    let len = vsnprintf(::std::ptr::null_mut(), 0, fmt, ap) + 1;
    let buffer = kunit_kmalloc((*current).kunit_test, len as usize, GFP_KERNEL);
    if buffer.is_null() { return; }
    kunit_err((*current).kunit_test, "%s:%d: %s", file, line, buffer);
    kunit_kfree((*current).kunit_test, buffer);
}

#[repr(C)]
pub struct kunit_result_stats_c { pub passed: ::std::os::raw::c_ulong, pub skipped: ::std::os::raw::c_ulong, pub failed: ::std::os::raw::c_ulong, pub total: ::std::os::raw::c_ulong }

unsafe fn kunit_suite_num_test_cases(suite: *mut kunit_suite) -> usize {
    let mut n = 0; let mut tc = ::std::ptr::null_mut();
    kunit_suite_for_each_test_case(suite, &mut tc, || { n += 1; }); n
}

const KUNIT_LEVEL_SUITE: ::std::os::raw::c_int = 0;
const KUNIT_LEVEL_CASE: ::std::os::raw::c_int = 1;
const KUNIT_LEVEL_CASE_PARAM: ::std::os::raw::c_int = 2;

unsafe fn kunit_print_suite_start(suite: *mut kunit_suite) {
    pr_info(KUNIT_SUBTEST_INDENT "KTAP version 1\n");
    pr_info(KUNIT_SUBTEST_INDENT "# Subtest: %s\n", (*suite).name);
    kunit_print_attr(suite as *mut _, false, KUNIT_LEVEL_CASE);
    pr_info(KUNIT_SUBTEST_INDENT "1..%zd\n", kunit_suite_num_test_cases(suite));
}

unsafe fn kunit_print_ok_not_ok(test: *mut kunit, level: ::std::os::raw::c_uint, status: kunit_status, number: usize, description: *const ::std::os::raw::c_char, directive: *const ::std::os::raw::c_char) {
    let header = if status == KUNIT_SKIPPED { " # SKIP " } else { "" };
    let body = if status == KUNIT_SKIPPED { directive } else { ::std::ptr::null() };
    WARN(!test.is_null() && level != 0, "suite test level can't be %u!\n", level);
    if test.is_null() { pr_info("%s %zd %s%s%s\n", kunit_status_to_ok_not_ok(status), number, description, header, body); }
    else { kunit_log(KERN_INFO, test, "%*s%s %zd %s%s%s", KUNIT_INDENT_LEN * level, "", kunit_status_to_ok_not_ok(status), number, description, header, body); }
}

pub unsafe extern "C" fn kunit_suite_has_succeeded(suite: *mut kunit_suite) -> kunit_status {
    if (*suite).status == KUNIT_SKIPPED { return KUNIT_SKIPPED; }
    if (*suite).suite_init_err != 0 { return KUNIT_FAILURE; }
    let mut status = KUNIT_SKIPPED; let mut tc = ::std::ptr::null_mut();
    kunit_suite_for_each_test_case(suite, &mut tc, || {
        if (*tc).status == KUNIT_FAILURE { (*suite).status = KUNIT_FAILURE; status = KUNIT_FAILURE; }
        else if (*tc).status == KUNIT_SUCCESS { status = KUNIT_SUCCESS; }
    }); status
}

static mut KUNIT_SUITE_COUNTER: usize = 1;
unsafe fn kunit_print_suite_end(suite: *mut kunit_suite) { kunit_print_ok_not_ok(::std::ptr::null_mut(), KUNIT_LEVEL_SUITE as _, kunit_suite_has_succeeded(suite), KUNIT_SUITE_COUNTER, (*suite).name, (*suite).status_comment); KUNIT_SUITE_COUNTER += 1; }

pub unsafe extern "C" fn kunit_test_case_num(suite: *mut kunit_suite, wanted: *mut kunit_case) -> u32 { let mut i=1; let mut tc=::std::ptr::null_mut(); let mut out=0; kunit_suite_for_each_test_case(suite,&mut tc,||{if tc==wanted {out=i;} i+=1;}); out }

unsafe fn kunit_update_stats(s: *mut kunit_result_stats_c, status: kunit_status) { match status { KUNIT_SUCCESS=>(*s).passed+=1, KUNIT_SKIPPED=>(*s).skipped+=1, KUNIT_FAILURE=>(*s).failed+=1, _=>{} } (*s).total+=1; }
unsafe fn kunit_accumulate_stats(t:*mut kunit_result_stats_c,a:kunit_result_stats_c){(*t).passed+=a.passed;(*t).skipped+=a.skipped;(*t).failed+=a.failed;(*t).total+=a.total;}

pub unsafe extern "C" fn kunit_array_gen_params(test:*mut kunit,_prev:*const ::std::ffi::c_void,desc:*mut ::std::os::raw::c_char)->*const ::std::ffi::c_void { let p=&mut (*test).params_array; if (*test).param_index<p.num_params { let v=(p.params as *const u8).add((*test).param_index*p.elem_size) as *const _; if let Some(f)=p.get_description { f(test,v,desc); } v } else {::std::ptr::null()} }

pub unsafe extern "C" fn kunit_init_test(test:*mut kunit,name:*const ::std::os::raw::c_char,log:*mut string_stream){spin_lock_init(&mut (*test).lock);INIT_LIST_HEAD(&mut (*test).resources);(*test).name=name;(*test).log=log;if !log.is_null(){string_stream_clear(log);}(*test).status=KUNIT_SUCCESS;(*test).status_comment[0]=0;(*test).params_array.params=::std::ptr::null_mut();(*test).params_array.get_description=None;(*test).params_array.num_params=0;(*test).params_array.elem_size=0;}

pub unsafe extern "C" fn kunit_run_tests(suite:*mut kunit_suite)->::std::os::raw::c_int { let mut ss=kunit_result_stats_c{passed:0,skipped:0,failed:0,total:0};let mut ts=ss;add_taint(TAINT_TEST,LOCKDEP_STILL_OK);if (*suite).status!=KUNIT_SKIPPED{kunit_print_suite_start(suite);let mut tc=::std::ptr::null_mut();kunit_suite_for_each_test_case(suite,&mut tc,||{(*tc).status=KUNIT_SUCCESS;kunit_update_stats(&mut ss,(*tc).status);});}kunit_print_suite_end(suite);0 }

pub unsafe extern "C" fn kunit_enabled()->bool{ENABLE_PARAM}

pub unsafe extern "C" fn kunit_kfree(test:*mut kunit,ptr:*const ::std::ffi::c_void){if !ptr.is_null(){kunit_release_action(test,kfree_action_wrapper,ptr as *mut _);}}
pub unsafe extern "C" fn kunit_cleanup(test:*mut kunit){let mut flags=0;while !list_empty(&(*test).resources){spin_lock_irqsave(&mut (*test).lock,&mut flags);let res=list_last_entry(&mut (*test).resources);spin_unlock_irqrestore(&mut (*test).lock,flags);kunit_remove_resource(test,res);}(*current).kunit_test=::std::ptr::null_mut();}

// Remaining externally supplied KUnit entry points and module registration are declared by kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
