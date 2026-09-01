/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from the original header:
// stdio.h, unistd.h, errno.h, string.h, assert.h, regex.h, stdlib.h,
// stdarg.h, time.h, signal.h, linux/*, sys/*, pthread.h, bpf/*,
// test_iptunnel_common.h, bpf_util.h, trace_helpers.h, testing_helpers.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __sum16 = __u16;
pub type size_t = usize;
pub type pid_t = c_int;
pub type pthread_t = c_ulong;
pub type timer_t = *mut c_void;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct bpf_prog_info {
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct bpf_link_info {
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum verbosity {
    VERBOSE_NONE,
    VERBOSE_NORMAL,
    VERBOSE_VERY,
    VERBOSE_SUPER,
}

#[repr(C)]
pub struct test_filter {
    pub name: *mut c_char,
    pub subtests: *mut *mut c_char,
    pub subtest_cnt: c_int,
}

#[repr(C)]
pub struct test_filter_set {
    pub tests: *mut test_filter,
    pub cnt: c_int,
}

#[repr(C)]
pub struct test_selector {
    pub whitelist: test_filter_set,
    pub blacklist: test_filter_set,
    pub num_set: *mut bool,
    pub num_set_len: c_int,
}

#[repr(C)]
pub struct subtest_state {
    pub name: *mut c_char,
    pub log_cnt: size_t,
    pub log_buf: *mut c_char,
    pub error_cnt: c_int,
    pub skipped: bool,
    pub filtered: bool,
    pub should_tmon: bool,

    pub stdout_saved: *mut FILE,
}

#[repr(C)]
pub struct test_state {
    pub tested: bool,
    pub force_log: bool,

    pub error_cnt: c_int,
    pub skip_cnt: c_int,
    pub sub_succ_cnt: c_int,

    pub subtest_states: *mut subtest_state,
    pub subtest_num: c_int,

    pub log_cnt: size_t,
    pub log_buf: *mut c_char,

    pub stdout_saved: *mut FILE,
}

unsafe extern "C" {
    pub static mut env_verbosity: c_int;
}

#[repr(C)]
pub struct prog_test_def {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum test_env_watchdog_state {
    WD_NOTIFY,
    WD_KILL,
}

#[repr(C)]
pub struct test_env {
    pub test_selector: test_selector,
    pub subtest_selector: test_selector,
    pub tmon_selector: test_selector,
    pub verifier_stats: bool,
    pub debug: bool,
    pub error_summary: bool,
    pub verbosity: verbosity,

    pub jit_enabled: bool,
    pub has_testmod: bool,
    pub get_test_cnt: bool,
    pub list_test_names: bool,

    pub test: *mut prog_test_def, /* current running test */
    pub test_state: *mut test_state, /* current running test state */
    pub subtest_state: *mut subtest_state, /* current running subtest state */

    pub stdout_saved: *mut FILE,
    pub stderr_saved: *mut FILE,
    pub nr_cpus: c_int,
    pub json: *mut FILE,

    pub succ_cnt: c_int, /* successful tests */
    pub sub_succ_cnt: c_int, /* successful sub-tests */
    pub fail_cnt: c_int, /* failed tests */
    pub skip_cnt: c_int, /* skipped tests */
    pub not_built_cnt: c_int, /* tests not built */

    pub saved_netns_fd: c_int,
    pub workers: c_int, /* number of worker process */
    pub worker_id: c_int, /* id number of current worker, main process is -1 */
    pub worker_pids: *mut pid_t, /* array of worker pids */
    pub worker_socks: *mut c_int, /* array of worker socks */
    pub worker_current_test: *mut c_int, /* array of current running test for each worker */

    pub main_thread: pthread_t,
    pub secs_till_notify: c_int,
    pub secs_till_kill: c_int,
    pub watchdog: timer_t, /* watch for stalled tests/subtests */
    pub watchdog_state: test_env_watchdog_state,
}

pub const MAX_LOG_TRUNK_SIZE: usize = 8192;
pub const MAX_SUBTEST_NAME: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum msg_type {
    MSG_DO_TEST = 0,
    MSG_TEST_DONE = 1,
    MSG_TEST_LOG = 2,
    MSG_SUBTEST_DONE = 3,
    MSG_EXIT = 255,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_do_test {
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_test_done {
    pub num: c_int,
    pub sub_succ_cnt: c_int,
    pub error_cnt: c_int,
    pub skip_cnt: c_int,
    pub have_log: bool,
    pub subtest_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_test_log {
    pub log_buf: [c_char; MAX_LOG_TRUNK_SIZE + 1],
    pub is_last: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_subtest_done {
    pub num: c_int,
    pub name: [c_char; MAX_SUBTEST_NAME + 1],
    pub error_cnt: c_int,
    pub skipped: bool,
    pub filtered: bool,
    pub have_log: bool,
}

#[repr(C)]
pub union msg_data {
    pub do_test: msg_do_test,
    pub test_done: msg_test_done,
    pub test_log: msg_test_log,
    pub subtest_done: msg_subtest_done,
}

#[repr(C)]
pub struct msg {
    pub type_: msg_type,
    pub data: msg_data,
}

unsafe extern "C" {
    pub static mut env: test_env;
    pub static mut stdout: *mut FILE;
    pub static mut errno: c_int;

    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn system(command: *const c_char) -> c_int;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn libbpf_get_error(ptr: *const c_void) -> c_int;
    pub fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *__u32) -> c_int;
    pub fn bpf_link_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *__u32) -> c_int;

    pub fn test__force_log();
    pub fn test__start_subtest_with_desc(name: *const c_char, description: *const c_char) -> bool;
    pub fn test__start_subtest(name: *const c_char) -> bool;
    pub fn test__end_subtest();
    pub fn test__skip();
    pub fn test__fail();
    pub fn test__join_cgroup(path: *const c_char) -> c_int;
    pub fn hexdump(prefix: *const c_char, buf: *const c_void, len: size_t);
}

#[macro_export]
macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const ::core::ffi::c_char
    };
}

#[macro_export]
macro_rules! PRINT_FAIL {
    ($format:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            test__fail();
            fprintf(stdout, c_str!("%s:FAIL:%d "), c_str!(""), line!() as ::core::ffi::c_int);
            fprintf(stdout, $format $(, $args)*);
        }
    }};
}

#[macro_export]
macro_rules! _CHECK {
    ($condition:expr, $tag:expr, $duration:expr, $format:expr $(, $args:expr)* $(,)?) => {{
        let __ret: ::core::ffi::c_int = if $condition { 1 } else { 0 };
        let __save_errno = unsafe { errno };
        unsafe {
            if __ret != 0 {
                test__fail();
                fprintf(stdout, c_str!("%s:FAIL:%s "), c_str!(""), $tag);
                fprintf(stdout, $format $(, $args)*);
            } else {
                fprintf(stdout, c_str!("%s:PASS:%s %d nsec\n"), c_str!(""), $tag, $duration);
            }
            errno = __save_errno;
        }
        __ret
    }};
}

#[macro_export]
macro_rules! CHECK_FAIL {
    ($condition:expr) => {{
        let __ret: ::core::ffi::c_int = if $condition { 1 } else { 0 };
        let __save_errno = unsafe { errno };
        unsafe {
            if __ret != 0 {
                test__fail();
                fprintf(stdout, c_str!("%s:FAIL:%d\n"), c_str!(""), line!() as ::core::ffi::c_int);
            }
            errno = __save_errno;
        }
        __ret
    }};
}

#[macro_export]
macro_rules! CHECK {
    ($condition:expr, $tag:expr, $format:expr $(, $args:expr)* $(,)?) => {
        _CHECK!($condition, $tag, duration, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! CHECK_ATTR {
    ($condition:expr, $tag:expr, $format:expr $(, $args:expr)* $(,)?) => {
        _CHECK!($condition, $tag, tattr.duration, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! ASSERT_FAIL {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        let duration: ::core::ffi::c_int = 0;
        CHECK!(false, c_str!(""), concat!($fmt, "\n\0").as_ptr() as *const ::core::ffi::c_char $(, $args)*);
        false
    }};
}

#[macro_export]
macro_rules! ASSERT_TRUE {
    ($actual:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___ok: bool = $actual;
        CHECK!(!___ok, $name, c_str!("unexpected %s: got FALSE\n"), $name);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_FALSE {
    ($actual:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___ok: bool = !$actual;
        CHECK!(!___ok, $name, c_str!("unexpected %s: got TRUE\n"), $name);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_EQ {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act == ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld != expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_NEQ {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act != ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld == expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_LT {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act < ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld >= expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_LE {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act <= ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld > expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_GT {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act > ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld <= expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_GE {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = ___act >= ___exp;
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual %lld < expected %lld\n"), $name, ___act as i64, ___exp as i64);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_STREQ {
    ($actual:expr, $expected:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___ok: bool = unsafe { strcmp(___act, ___exp) == 0 };
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual '%s' != expected '%s'\n"), $name, ___act, ___exp);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_STRNEQ {
    ($actual:expr, $expected:expr, $len:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___act = $actual;
        let ___exp = $expected;
        let ___len: ::core::ffi::c_int = $len;
        let ___ok: bool = unsafe { strncmp(___act, ___exp, ___len as usize) == 0 };
        CHECK!(!___ok, $name, c_str!("unexpected %s: actual '%.*s' != expected '%.*s'\n"), $name, ___len, ___act, ___len, ___exp);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_HAS_SUBSTR {
    ($str:expr, $substr:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___str = $str;
        let ___substr = $substr;
        let ___ok: bool = unsafe { !strstr(___str, ___substr).is_null() };
        CHECK!(!___ok, $name, c_str!("unexpected %s: '%s' is not a substring of '%s'\n"), $name, ___substr, ___str);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_MEMEQ {
    ($actual:expr, $expected:expr, $len:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let __act = $actual as *const ::core::ffi::c_void;
        let __exp = $expected as *const ::core::ffi::c_void;
        let __len: ::core::ffi::c_int = $len;
        let ___ok: bool = unsafe { memcmp(__act, __exp, __len as usize) == 0 };
        CHECK!(!___ok, $name, c_str!("unexpected memory mismatch\n"));
        unsafe {
            fprintf(stdout, c_str!("actual:\n"));
            hexdump(c_str!("\t"), __act, __len as usize);
            fprintf(stdout, c_str!("expected:\n"));
            hexdump(c_str!("\t"), __exp, __len as usize);
        }
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_OK {
    ($res:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___res: i64 = $res as i64;
        let ___ok: bool = ___res == 0;
        CHECK!(!___ok, $name, c_str!("unexpected error: %lld (errno %d)\n"), ___res, unsafe { errno });
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_ERR {
    ($res:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___res: i64 = $res as i64;
        let ___ok: bool = ___res < 0;
        CHECK!(!___ok, $name, c_str!("unexpected success: %lld\n"), ___res);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_NULL {
    ($ptr:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___res = $ptr as *const ::core::ffi::c_void;
        let ___ok: bool = ___res.is_null();
        CHECK!(!___ok, $name, c_str!("unexpected pointer: %p\n"), ___res);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_OK_PTR {
    ($ptr:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___res = $ptr as *const ::core::ffi::c_void;
        let ___err = unsafe { libbpf_get_error(___res) };
        let ___ok: bool = ___err == 0;
        CHECK!(!___ok, $name, c_str!("unexpected error: %d\n"), ___err);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_ERR_PTR {
    ($ptr:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___res = $ptr as *const ::core::ffi::c_void;
        let ___err = unsafe { libbpf_get_error(___res) };
        let ___ok: bool = ___err != 0;
        CHECK!(!___ok, $name, c_str!("unexpected pointer: %p\n"), ___res);
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_OK_FD {
    ($fd:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___fd: ::core::ffi::c_int = $fd;
        let ___ok: bool = ___fd >= 0;
        CHECK!(!___ok, $name, c_str!("unexpected fd: %d (errno %d)\n"), ___fd, unsafe { errno });
        ___ok
    }};
}

#[macro_export]
macro_rules! ASSERT_ERR_FD {
    ($fd:expr, $name:expr) => {{
        let duration: ::core::ffi::c_int = 0;
        let ___fd: ::core::ffi::c_int = $fd;
        let ___ok: bool = ___fd < 0;
        CHECK!(!___ok, $name, c_str!("unexpected fd: %d\n"), ___fd);
        ___ok
    }};
}

#[macro_export]
macro_rules! SYS {
    ($goto_label:lifetime, $fmt:expr $(, $args:expr)* $(,)?) => {{
        let mut cmd = [0 as ::core::ffi::c_char; 1024];
        unsafe {
            snprintf(cmd.as_mut_ptr(), cmd.len(), $fmt $(, $args)*);
            if !ASSERT_OK!(system(cmd.as_ptr()), cmd.as_ptr()) {
                break $goto_label;
            }
        }
    }};
}

#[macro_export]
macro_rules! SYS_FAIL {
    ($goto_label:lifetime, $fmt:expr $(, $args:expr)* $(,)?) => {{
        let mut cmd = [0 as ::core::ffi::c_char; 1024];
        unsafe {
            snprintf(cmd.as_mut_ptr(), cmd.len(), $fmt $(, $args)*);
            if !ASSERT_NEQ!(0, system(cmd.as_ptr()), cmd.as_ptr()) {
                break $goto_label;
            }
        }
    }};
}

pub const ALL_TO_DEV_NULL: &[u8] = b" >/dev/null 2>&1\0";

#[macro_export]
macro_rules! SYS_NOFAIL {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        let mut cmd = [0 as ::core::ffi::c_char; 1024];
        unsafe {
            let n = snprintf(cmd.as_mut_ptr(), cmd.len(), $fmt $(, $args)*);
            if n < cmd.len() as ::core::ffi::c_int
                && cmd.len() - n as usize >= ALL_TO_DEV_NULL.len()
            {
                strcat(cmd.as_mut_ptr(), ALL_TO_DEV_NULL.as_ptr() as *const ::core::ffi::c_char);
            }
            system(cmd.as_ptr());
        }
    }};
}

unsafe extern "C" {
    pub fn start_libbpf_log_capture() -> c_int;
    pub fn stop_libbpf_log_capture() -> *mut c_char;
}

#[inline]
pub unsafe fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    ptr as c_ulong as __u64
}

#[inline]
pub unsafe fn u64_to_ptr(ptr: __u64) -> *mut c_void {
    ptr as c_ulong as *mut c_void
}

#[inline]
pub unsafe fn id_from_prog_fd(fd: c_int) -> __u32 {
    let mut prog_info: bpf_prog_info = core::mem::zeroed();
    let mut prog_info_len: __u32 = core::mem::size_of_val(&prog_info) as __u32;
    let err: c_int;

    err = bpf_obj_get_info_by_fd(
        fd,
        &mut prog_info as *mut bpf_prog_info as *mut c_void,
        &mut prog_info_len,
    );
    if !ASSERT_OK!(err, c_str!("id_from_prog_fd")) {
        return 0;
    }

    ASSERT_NEQ!(prog_info.id, 0, c_str!("prog_info.id"));
    prog_info.id
}

#[inline]
pub unsafe fn id_from_link_fd(fd: c_int) -> __u32 {
    let mut link_info: bpf_link_info = core::mem::zeroed();
    let mut link_info_len: __u32 = core::mem::size_of_val(&link_info) as __u32;
    let err: c_int;

    err = bpf_link_get_info_by_fd(
        fd,
        &mut link_info as *mut bpf_link_info as *mut c_void,
        &mut link_info_len,
    );
    if !ASSERT_OK!(err, c_str!("id_from_link_fd")) {
        return 0;
    }

    ASSERT_NEQ!(link_info.id, 0, c_str!("link_info.id"));
    link_info.id
}

unsafe extern "C" {
    pub fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    pub fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
    pub fn compare_stack_ips(smap_fd: c_int, amap_fd: c_int, stack_trace_len: c_int) -> c_int;
    pub fn trigger_module_test_read(read_sz: c_int) -> c_int;
    pub fn trigger_module_test_write(write_sz: c_int) -> c_int;
    pub fn write_sysctl(sysctl: *const c_char, value: *const c_char) -> c_int;
    pub fn get_bpf_max_tramp_links_from(btf: *mut btf) -> c_int;
    pub fn get_bpf_max_tramp_links() -> c_int;

    pub fn netns_new(name: *const c_char, open: bool) -> *mut netns_obj;
    pub fn netns_free(netns: *mut netns_obj);
}

#[cfg(target_arch = "x86_64")]
pub const SYS_NANOSLEEP_KPROBE_NAME: &[u8] = b"__x64_sys_nanosleep\0";
#[cfg(target_arch = "s390x")]
pub const SYS_NANOSLEEP_KPROBE_NAME: &[u8] = b"__s390x_sys_nanosleep\0";
#[cfg(target_arch = "aarch64")]
pub const SYS_NANOSLEEP_KPROBE_NAME: &[u8] = b"__arm64_sys_nanosleep\0";
#[cfg(target_arch = "riscv64")]
pub const SYS_NANOSLEEP_KPROBE_NAME: &[u8] = b"__riscv_sys_nanosleep\0";
#[cfg(not(any(target_arch = "x86_64", target_arch = "s390x", target_arch = "aarch64", target_arch = "riscv64")))]
pub const SYS_NANOSLEEP_KPROBE_NAME: &[u8] = b"sys_nanosleep\0";

pub const BPF_TESTMOD_TEST_FILE: &[u8] = b"/sys/kernel/bpf_testmod\0";

pub type pre_execution_cb = Option<unsafe extern "C" fn(obj: *mut bpf_object) -> c_int>;

#[repr(C)]
pub struct test_loader {
    pub log_buf: *mut c_char,
    pub log_buf_sz: size_t,
    pub pre_execution_cb: pre_execution_cb,

    pub obj: *mut bpf_object,
}

#[inline]
pub unsafe fn test_loader__set_pre_execution_cb(tester: *mut test_loader, cb: pre_execution_cb) {
    (*tester).pre_execution_cb = cb;
}

pub type skel_elf_bytes_fn = Option<unsafe extern "C" fn(sz: *mut size_t) -> *const c_void>;

unsafe extern "C" {
    pub fn test_loader__run_subtests(
        tester: *mut test_loader,
        skel_name: *const c_char,
        elf_bytes_factory: skel_elf_bytes_fn,
    );

    pub fn test_loader_fini(tester: *mut test_loader);
}

#[macro_export]
macro_rules! RUN_TESTS {
    ($skel:ident) => {{
        let mut tester: test_loader = unsafe { ::core::mem::zeroed() };
        unsafe {
            test_loader__run_subtests(
                &mut tester,
                concat!(stringify!($skel), "\0").as_ptr() as *const ::core::ffi::c_char,
                Some($skel__elf_bytes),
            );
            test_loader_fini(&mut tester);
        }
    }};
}

#[repr(C)]
pub struct expect_msg {
    pub substr: *const c_char, /* substring match */
    pub regex: regex_t,
    pub is_regex: bool,
    pub on_next_line: bool,
    pub negative: bool,
}

#[repr(C)]
pub struct expected_msgs {
    pub patterns: *mut expect_msg,
    pub cnt: size_t,
}

unsafe extern "C" {
    pub fn validate_msgs(
        log_buf: *const c_char,
        msgs: *mut expected_msgs,
        emit_fn: Option<unsafe extern "C" fn(buf: *const c_char, force: bool)>,
    );
    pub fn free_msgs(msgs: *mut expected_msgs);
    pub fn verify_test_stderr(obj: *mut bpf_object, prog: *mut bpf_program);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
