// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/kprobe_multi_test.c.
// Dependencies from the original C includes are expected to be supplied by the
// surrounding test harness and generated skeleton bindings.

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong, c_void};
use core::ptr;

type __u64 = u64;

const BPF_TRACE_KPROBE_MULTI: c_int = 0;
const BPF_F_KPROBE_MULTI_RETURN: u32 = 1;
const BPF_F_SLEEPABLE: u32 = 1 << 4;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const ENOENT: c_int = 2;
const INT_MAX: usize = c_int::MAX as usize;

#[cfg(target_arch = "x86_64")]
const SYS_PREFIX: &str = "__x64_";
#[cfg(not(target_arch = "x86_64"))]
const SYS_PREFIX: &str = "";

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct bpf_link_create_opts_kprobe_multi {
    pub flags: u32,
    pub addrs: *const c_ulong,
    pub syms: *const *const c_char,
    pub cnt: usize,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    pub kprobe_multi: bpf_link_create_opts_kprobe_multi,
}

impl Default for bpf_link_create_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct bpf_kprobe_multi_opts {
    pub addrs: *const c_ulong,
    pub syms: *const *const c_char,
    pub cnt: usize,
    pub cookies: *const __u64,
    pub retprobe: bool,
    pub unique_match: bool,
}

impl Default for bpf_kprobe_multi_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct ksyms {
    pub filtered_syms: *mut *const c_char,
    pub filtered_cnt: usize,
}

#[repr(C)]
pub struct kprobe_multi_bss {
    pub pid: c_int,
    pub kprobe_test1_result: c_int,
    pub kprobe_test2_result: c_int,
    pub kprobe_test3_result: c_int,
    pub kprobe_test4_result: c_int,
    pub kprobe_test5_result: c_int,
    pub kprobe_test6_result: c_int,
    pub kprobe_test7_result: c_int,
    pub kprobe_test8_result: c_int,
    pub kretprobe_test1_result: c_int,
    pub kretprobe_test2_result: c_int,
    pub kretprobe_test3_result: c_int,
    pub kretprobe_test4_result: c_int,
    pub kretprobe_test5_result: c_int,
    pub kretprobe_test6_result: c_int,
    pub kretprobe_test7_result: c_int,
    pub kretprobe_test8_result: c_int,
}

#[repr(C)]
pub struct kprobe_multi_progs {
    pub trigger: *mut bpf_program,
    pub test_kprobe: *mut bpf_program,
    pub test_kretprobe: *mut bpf_program,
    pub test_kprobe_manual: *mut bpf_program,
    pub test_kretprobe_manual: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi {
    pub bss: *mut kprobe_multi_bss,
    pub progs: kprobe_multi_progs,
}

#[repr(C)]
pub struct kprobe_multi_empty_progs {
    pub test_kprobe_empty: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi_empty {
    pub progs: kprobe_multi_empty_progs,
}

#[repr(C)]
pub struct kprobe_multi_override_bss {
    pub pid: c_int,
}

#[repr(C)]
pub struct kprobe_multi_override_progs {
    pub test_override: *mut bpf_program,
    pub test_kprobe_override: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi_override_links {
    pub test_override: *mut bpf_link,
    pub test_kprobe_override: *mut bpf_link,
}

#[repr(C)]
pub struct kprobe_multi_override {
    pub bss: *mut kprobe_multi_override_bss,
    pub progs: kprobe_multi_override_progs,
    pub links: kprobe_multi_override_links,
}

#[repr(C)]
pub struct kprobe_multi_session_bss {
    pub pid: c_int,
    pub kprobe_session_result: [c_int; 8],
}

#[repr(C)]
pub struct kprobe_multi_session_progs {
    pub trigger: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi_session {
    pub bss: *mut kprobe_multi_session_bss,
    pub progs: kprobe_multi_session_progs,
}

#[repr(C)]
pub struct kprobe_multi_session_cookie_bss {
    pub pid: c_int,
    pub test_kprobe_1_result: c_int,
    pub test_kprobe_2_result: c_int,
    pub test_kprobe_3_result: c_int,
}

#[repr(C)]
pub struct kprobe_multi_session_cookie_progs {
    pub trigger: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi_session_cookie {
    pub bss: *mut kprobe_multi_session_cookie_bss,
    pub progs: kprobe_multi_session_cookie_progs,
}

#[repr(C)]
pub struct kprobe_multi_sleepable_bss {
    pub user_ptr: *mut c_void,
}

#[repr(C)]
pub struct kprobe_multi_sleepable_progs {
    pub handle_kprobe_multi_sleepable: *mut bpf_program,
    pub fentry: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi_sleepable {
    pub bss: *mut kprobe_multi_sleepable_bss,
    pub progs: kprobe_multi_sleepable_progs,
}

#[repr(C)]
pub struct kprobe_write_ctx_progs {
    pub kprobe_multi_write_ctx: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_write_ctx {
    pub progs: kprobe_write_ctx_progs,
}

extern "C" {
    static mut errno: c_int;

    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *mut bpf_link_create_opts,
    ) -> c_int;
    fn bpf_program__attach_kprobe_multi_opts(
        prog: *mut bpf_program,
        pattern: *const c_char,
        opts: *mut bpf_kprobe_multi_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_kprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        func_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_kprobe_opts(
        prog: *mut bpf_program,
        func_name: *const c_char,
        opts: *mut c_void,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: u32) -> c_int;

    fn ksym_get_addr(sym: *const c_char) -> c_ulonglong;
    fn load_kallsyms() -> c_int;
    fn bpf_get_ksyms(ksyms: *mut *mut ksyms, kernel: bool) -> c_int;
    fn bpf_get_addrs(addrs: *mut *mut c_ulong, cnt: *mut usize, kernel: bool) -> c_int;
    fn free_kallsyms_local(ksyms: *mut ksyms);
    fn get_time_ns() -> i64;

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_kprobe_multi_verifier();

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;

    fn kprobe_multi__open_and_load() -> *mut kprobe_multi;
    fn kprobe_multi__attach(skel: *mut kprobe_multi) -> c_int;
    fn kprobe_multi__destroy(skel: *mut kprobe_multi);

    fn kprobe_multi_empty__open_and_load() -> *mut kprobe_multi_empty;
    fn kprobe_multi_empty__destroy(skel: *mut kprobe_multi_empty);

    fn kprobe_multi_override__open_and_load() -> *mut kprobe_multi_override;
    fn kprobe_multi_override__destroy(skel: *mut kprobe_multi_override);

    fn kprobe_multi_session__open_and_load() -> *mut kprobe_multi_session;
    fn kprobe_multi_session__attach(skel: *mut kprobe_multi_session) -> c_int;
    fn kprobe_multi_session__destroy(skel: *mut kprobe_multi_session);

    fn kprobe_multi_session_cookie__open_and_load() -> *mut kprobe_multi_session_cookie;
    fn kprobe_multi_session_cookie__attach(skel: *mut kprobe_multi_session_cookie) -> c_int;
    fn kprobe_multi_session_cookie__destroy(skel: *mut kprobe_multi_session_cookie);

    fn kprobe_multi_sleepable__open() -> *mut kprobe_multi_sleepable;
    fn kprobe_multi_sleepable__load(skel: *mut kprobe_multi_sleepable) -> c_int;
    fn kprobe_multi_sleepable__destroy(skel: *mut kprobe_multi_sleepable);

    fn kprobe_write_ctx__open_and_load() -> *mut kprobe_write_ctx;
    fn kprobe_write_ctx__destroy(skel: *mut kprobe_write_ctx);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn get_addr(sym: *const c_char, addr: &mut c_ulonglong) -> bool {
    *addr = ksym_get_addr(sym);
    ASSERT_NEQ(*addr, 0, cstr!("kallsyms load failed"))
}

unsafe fn kprobe_multi_test_run(skel: *mut kprobe_multi, test_return: bool) {
    let mut topts = bpf_test_run_opts::default();
    let prog_fd: c_int;
    let err: c_int;

    prog_fd = bpf_program__fd((*skel).progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, cstr!("test_run"));
    ASSERT_EQ(topts.retval, 0, cstr!("test_run"));

    ASSERT_EQ((*(*skel).bss).kprobe_test1_result, 1, cstr!("kprobe_test1_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test2_result, 1, cstr!("kprobe_test2_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test3_result, 1, cstr!("kprobe_test3_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test4_result, 1, cstr!("kprobe_test4_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test5_result, 1, cstr!("kprobe_test5_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test6_result, 1, cstr!("kprobe_test6_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test7_result, 1, cstr!("kprobe_test7_result"));
    ASSERT_EQ((*(*skel).bss).kprobe_test8_result, 1, cstr!("kprobe_test8_result"));

    if test_return {
        ASSERT_EQ((*(*skel).bss).kretprobe_test1_result, 1, cstr!("kretprobe_test1_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test2_result, 1, cstr!("kretprobe_test2_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test3_result, 1, cstr!("kretprobe_test3_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test4_result, 1, cstr!("kretprobe_test4_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test5_result, 1, cstr!("kretprobe_test5_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test6_result, 1, cstr!("kretprobe_test6_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test7_result, 1, cstr!("kretprobe_test7_result"));
        ASSERT_EQ((*(*skel).bss).kretprobe_test8_result, 1, cstr!("kretprobe_test8_result"));
    }
}

unsafe fn test_skel_api() {
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let err: c_int;

    'cleanup: loop {
        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi__open_and_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();
        err = kprobe_multi__attach(skel);
        if !ASSERT_OK(err, cstr!("kprobe_multi__attach")) {
            break 'cleanup;
        }

        kprobe_multi_test_run(skel, true);
        break 'cleanup;
    }

    kprobe_multi__destroy(skel);
}

unsafe fn test_link_api(opts: *mut bpf_link_create_opts) {
    let mut prog_fd: c_int;
    let mut link1_fd: c_int = -1;
    let mut link2_fd: c_int = -1;
    let mut skel: *mut kprobe_multi = ptr::null_mut();

    'cleanup: loop {
        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("fentry_raw_skel_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();
        prog_fd = bpf_program__fd((*skel).progs.test_kprobe);
        link1_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_KPROBE_MULTI, opts);
        if !ASSERT_GE(link1_fd, 0, cstr!("link_fd")) {
            break 'cleanup;
        }

        (*opts).kprobe_multi.flags = BPF_F_KPROBE_MULTI_RETURN;
        prog_fd = bpf_program__fd((*skel).progs.test_kretprobe);
        link2_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_KPROBE_MULTI, opts);
        if !ASSERT_GE(link2_fd, 0, cstr!("link_fd")) {
            break 'cleanup;
        }

        kprobe_multi_test_run(skel, true);
        break 'cleanup;
    }

    if link1_fd != -1 {
        close(link1_fd);
    }
    if link2_fd != -1 {
        close(link2_fd);
    }
    kprobe_multi__destroy(skel);
}

unsafe fn test_link_api_addrs() {
    let mut opts = bpf_link_create_opts::default();
    let mut addrs: [c_ulonglong; 8] = [0; 8];

    if !get_addr(cstr!("bpf_fentry_test1"), &mut addrs[0]) { return; }
    if !get_addr(cstr!("bpf_fentry_test2"), &mut addrs[1]) { return; }
    if !get_addr(cstr!("bpf_fentry_test3"), &mut addrs[2]) { return; }
    if !get_addr(cstr!("bpf_fentry_test4"), &mut addrs[3]) { return; }
    if !get_addr(cstr!("bpf_fentry_test5"), &mut addrs[4]) { return; }
    if !get_addr(cstr!("bpf_fentry_test6"), &mut addrs[5]) { return; }
    if !get_addr(cstr!("bpf_fentry_test7"), &mut addrs[6]) { return; }
    if !get_addr(cstr!("bpf_fentry_test8"), &mut addrs[7]) { return; }

    opts.kprobe_multi.addrs = addrs.as_ptr() as *const c_ulong;
    opts.kprobe_multi.cnt = addrs.len();
    test_link_api(&mut opts);
}

unsafe fn test_link_api_syms() {
    let mut opts = bpf_link_create_opts::default();
    let syms: [*const c_char; 8] = [
        cstr!("bpf_fentry_test1"),
        cstr!("bpf_fentry_test2"),
        cstr!("bpf_fentry_test3"),
        cstr!("bpf_fentry_test4"),
        cstr!("bpf_fentry_test5"),
        cstr!("bpf_fentry_test6"),
        cstr!("bpf_fentry_test7"),
        cstr!("bpf_fentry_test8"),
    ];

    opts.kprobe_multi.syms = syms.as_ptr();
    opts.kprobe_multi.cnt = syms.len();
    test_link_api(&mut opts);
}

unsafe fn test_attach_api(pattern: *const c_char, opts: *mut bpf_kprobe_multi_opts) {
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut skel: *mut kprobe_multi = ptr::null_mut();

    'cleanup: loop {
        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("fentry_raw_skel_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();
        link1 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, pattern, opts);
        if !ASSERT_OK_PTR(link1, cstr!("bpf_program__attach_kprobe_multi_opts")) {
            break 'cleanup;
        }

        if !opts.is_null() {
            (*opts).retprobe = true;
            link2 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kretprobe_manual, pattern, opts);
            if !ASSERT_OK_PTR(link2, cstr!("bpf_program__attach_kprobe_multi_opts")) {
                break 'cleanup;
            }
        }

        kprobe_multi_test_run(skel, !opts.is_null());
        break 'cleanup;
    }

    bpf_link__destroy(link2);
    bpf_link__destroy(link1);
    kprobe_multi__destroy(skel);
}

unsafe fn test_attach_api_pattern() {
    let mut opts = bpf_kprobe_multi_opts::default();

    test_attach_api(cstr!("bpf_fentry_test*"), &mut opts);
    test_attach_api(cstr!("bpf_fentry_test?"), ptr::null_mut());
}

unsafe fn test_attach_api_addrs() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut addrs: [c_ulonglong; 8] = [0; 8];

    if !get_addr(cstr!("bpf_fentry_test1"), &mut addrs[0]) { return; }
    if !get_addr(cstr!("bpf_fentry_test2"), &mut addrs[1]) { return; }
    if !get_addr(cstr!("bpf_fentry_test3"), &mut addrs[2]) { return; }
    if !get_addr(cstr!("bpf_fentry_test4"), &mut addrs[3]) { return; }
    if !get_addr(cstr!("bpf_fentry_test5"), &mut addrs[4]) { return; }
    if !get_addr(cstr!("bpf_fentry_test6"), &mut addrs[5]) { return; }
    if !get_addr(cstr!("bpf_fentry_test7"), &mut addrs[6]) { return; }
    if !get_addr(cstr!("bpf_fentry_test8"), &mut addrs[7]) { return; }

    opts.addrs = addrs.as_ptr() as *const c_ulong;
    opts.cnt = addrs.len();
    test_attach_api(ptr::null(), &mut opts);
}

unsafe fn test_attach_api_syms() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let syms: [*const c_char; 8] = [
        cstr!("bpf_fentry_test1"),
        cstr!("bpf_fentry_test2"),
        cstr!("bpf_fentry_test3"),
        cstr!("bpf_fentry_test4"),
        cstr!("bpf_fentry_test5"),
        cstr!("bpf_fentry_test6"),
        cstr!("bpf_fentry_test7"),
        cstr!("bpf_fentry_test8"),
    ];

    opts.syms = syms.as_ptr();
    opts.cnt = syms.len();
    test_attach_api(ptr::null(), &mut opts);
}

unsafe fn test_attach_api_fails() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut topts = bpf_test_run_opts::default();
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let mut sl_skel: *mut kprobe_multi_sleepable = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut addrs: [c_ulonglong; 2] = [0; 2];
    let syms: [*const c_char; 2] = [cstr!("bpf_fentry_test1"), cstr!("bpf_fentry_test2")];
    let cookies: [__u64; 2] = [0; 2];
    let mut saved_error: c_int;
    let mut err: c_int;

    addrs[0] = ksym_get_addr(cstr!("bpf_fentry_test1"));
    addrs[1] = ksym_get_addr(cstr!("bpf_fentry_test2"));

    'cleanup: loop {
        if !ASSERT_FALSE(addrs[0] == 0 || addrs[1] == 0, cstr!("ksym_get_addr")) {
            break 'cleanup;
        }

        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("fentry_raw_skel_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();

        /* fail_1 - pattern and opts NULL */
        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, ptr::null(), ptr::null_mut());
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_1")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_1_error")) {
            break 'cleanup;
        }

        /* fail_2 - both addrs and syms set */
        opts.addrs = addrs.as_ptr() as *const c_ulong;
        opts.syms = syms.as_ptr();
        opts.cnt = syms.len();
        opts.cookies = ptr::null();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, ptr::null(), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_2")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_2_error")) {
            break 'cleanup;
        }

        /* fail_3 - pattern and addrs set */
        opts.addrs = addrs.as_ptr() as *const c_ulong;
        opts.syms = ptr::null();
        opts.cnt = syms.len();
        opts.cookies = ptr::null();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("ksys_*"), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_3")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_3_error")) {
            break 'cleanup;
        }

        /* fail_4 - pattern and cnt set */
        opts.addrs = ptr::null();
        opts.syms = ptr::null();
        opts.cnt = syms.len();
        opts.cookies = ptr::null();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("ksys_*"), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_4")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_4_error")) {
            break 'cleanup;
        }

        /* fail_5 - pattern and cookies */
        opts.addrs = ptr::null();
        opts.syms = ptr::null();
        opts.cnt = 0;
        opts.cookies = cookies.as_ptr();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("ksys_*"), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_5")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_5_error")) {
            break 'cleanup;
        }

        /* fail_6 - abnormal cnt */
        opts.addrs = addrs.as_ptr() as *const c_ulong;
        opts.syms = ptr::null();
        opts.cnt = INT_MAX;
        opts.cookies = ptr::null();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, ptr::null(), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_6")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -E2BIG, cstr!("fail_6_error")) {
            break 'cleanup;
        }

        /* fail_7 - non-existent wildcard pattern (slow path) */
        opts = bpf_kprobe_multi_opts::default();

        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("__nonexistent_func_xyz_*"), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_7")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -ENOENT, cstr!("fail_7_error")) {
            break 'cleanup;
        }

        /* fail_8 - non-existent exact name (fast path), same error as wildcard */
        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("__nonexistent_func_xyz_123"), &mut opts);
        saved_error = -errno;
        if !ASSERT_ERR_PTR(link, cstr!("fail_8")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -ENOENT, cstr!("fail_8_error")) {
            break 'cleanup;
        }

        /* fail_9 - sleepable kprobe multi should not attach */
        sl_skel = kprobe_multi_sleepable__open();
        if !ASSERT_OK_PTR(sl_skel, cstr!("sleep_skel_open")) {
            break 'cleanup;
        }

        (*(*sl_skel).bss).user_ptr = sl_skel as *mut c_void;

        err = bpf_program__set_flags((*sl_skel).progs.handle_kprobe_multi_sleepable, BPF_F_SLEEPABLE);
        if !ASSERT_OK(err, cstr!("sleep_skel_set_flags")) {
            break 'cleanup;
        }

        err = kprobe_multi_sleepable__load(sl_skel);
        if !ASSERT_OK(err, cstr!("sleep_skel_load")) {
            break 'cleanup;
        }

        link = bpf_program__attach_kprobe_multi_opts((*sl_skel).progs.handle_kprobe_multi_sleepable, cstr!("bpf_fentry_test1"), ptr::null_mut());
        saved_error = -errno;

        if !ASSERT_ERR_PTR(link, cstr!("fail_9")) {
            break 'cleanup;
        }

        if !ASSERT_EQ(saved_error, -EINVAL, cstr!("fail_9_error")) {
            break 'cleanup;
        }

        err = bpf_prog_test_run_opts(bpf_program__fd((*sl_skel).progs.fentry), &mut topts);
        ASSERT_OK(err, cstr!("bpf_prog_test_run_opts"));
        break 'cleanup;
    }

    bpf_link__destroy(link);
    kprobe_multi__destroy(skel);
    kprobe_multi_sleepable__destroy(sl_skel);
}

unsafe fn test_session_skel_api() {
    let mut skel: *mut kprobe_multi_session = ptr::null_mut();
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut topts = bpf_test_run_opts::default();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut i: c_int;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = kprobe_multi_session__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi_session__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    'cleanup: loop {
        err = kprobe_multi_session__attach(skel);
        if !ASSERT_OK(err, cstr!(" kprobe_multi_session__attach")) {
            break 'cleanup;
        }

        prog_fd = bpf_program__fd((*skel).progs.trigger);
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, cstr!("test_run"));
        ASSERT_EQ(topts.retval, 0, cstr!("test_run"));

        /*
         * bpf_fentry_test1 is hit by both the wildcard probe and the exact
         * name probe (test_kprobe_syms), so entry + return fires twice: 4.
         * bpf_fentry_test2-4 are hit only by the wildcard probe: 2.
         */
        ASSERT_EQ((*(*skel).bss).kprobe_session_result[0], 4, cstr!("kprobe_session_result"));
        i = 1;
        while i < 4 {
            ASSERT_EQ((*(*skel).bss).kprobe_session_result[i as usize], 2, cstr!("kprobe_session_result"));
            i += 1;
        }

        /* bpf_fentry_test5-8 trigger only entry probe, result is 1 */
        i = 4;
        while i < 8 {
            ASSERT_EQ((*(*skel).bss).kprobe_session_result[i as usize], 1, cstr!("kprobe_session_result"));
            i += 1;
        }
        break 'cleanup;
    }

    let _ = &mut opts;
    bpf_link__destroy(link);
    kprobe_multi_session__destroy(skel);
}

unsafe fn test_session_cookie_skel_api() {
    let mut skel: *mut kprobe_multi_session_cookie = ptr::null_mut();
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut topts = bpf_test_run_opts::default();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut err: c_int;
    let prog_fd: c_int;

    skel = kprobe_multi_session_cookie__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr!("fentry_raw_skel_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    'cleanup: loop {
        err = kprobe_multi_session_cookie__attach(skel);
        if !ASSERT_OK(err, cstr!(" kprobe_multi_wrapper__attach")) {
            break 'cleanup;
        }

        prog_fd = bpf_program__fd((*skel).progs.trigger);
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, cstr!("test_run"));
        ASSERT_EQ(topts.retval, 0, cstr!("test_run"));

        ASSERT_EQ((*(*skel).bss).test_kprobe_1_result, 1, cstr!("test_kprobe_1_result"));
        ASSERT_EQ((*(*skel).bss).test_kprobe_2_result, 2, cstr!("test_kprobe_2_result"));
        ASSERT_EQ((*(*skel).bss).test_kprobe_3_result, 3, cstr!("test_kprobe_3_result"));
        break 'cleanup;
    }

    let _ = &mut opts;
    bpf_link__destroy(link);
    kprobe_multi_session_cookie__destroy(skel);
}

unsafe fn test_unique_match() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();

    skel = kprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi__open_and_load")) {
        return;
    }

    opts.unique_match = true;
    (*(*skel).bss).pid = getpid();
    link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("bpf_fentry_test*"), &mut opts);
    if !ASSERT_ERR_PTR(link, cstr!("bpf_program__attach_kprobe_multi_opts")) {
        bpf_link__destroy(link);
    }

    link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, cstr!("bpf_fentry_test8*"), &mut opts);
    if ASSERT_OK_PTR(link, cstr!("bpf_program__attach_kprobe_multi_opts")) {
        bpf_link__destroy(link);
    }

    kprobe_multi__destroy(skel);
}

unsafe fn do_bench_test(skel: *mut kprobe_multi_empty, opts: *mut bpf_kprobe_multi_opts) {
    let attach_start_ns: i64;
    let attach_end_ns: i64;
    let detach_start_ns: i64;
    let detach_end_ns: i64;
    let attach_delta: f64;
    let detach_delta: f64;
    let mut link: *mut bpf_link = ptr::null_mut();

    attach_start_ns = get_time_ns();
    link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_empty, ptr::null(), opts);
    attach_end_ns = get_time_ns();

    if !ASSERT_OK_PTR(link, cstr!("bpf_program__attach_kprobe_multi_opts")) {
        return;
    }

    detach_start_ns = get_time_ns();
    bpf_link__destroy(link);
    detach_end_ns = get_time_ns();

    attach_delta = (attach_end_ns - attach_start_ns) as f64 / 1000000000.0;
    detach_delta = (detach_end_ns - detach_start_ns) as f64 / 1000000000.0;

    printf(cstr!("%s: found %lu functions\n"), cstr!("do_bench_test"), (*opts).cnt as c_ulong);
    printf(cstr!("%s: attached in %7.3lfs\n"), cstr!("do_bench_test"), attach_delta);
    printf(cstr!("%s: detached in %7.3lfs\n"), cstr!("do_bench_test"), detach_delta);
}

unsafe fn test_kprobe_multi_bench_attach(kernel: bool) {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut skel: *mut kprobe_multi_empty = ptr::null_mut();
    let mut ksyms: *mut ksyms = ptr::null_mut();

    if !ASSERT_OK(bpf_get_ksyms(&mut ksyms, kernel), cstr!("bpf_get_ksyms")) {
        return;
    }

    'cleanup: loop {
        skel = kprobe_multi_empty__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi_empty__open_and_load")) {
            break 'cleanup;
        }

        opts.syms = (*ksyms).filtered_syms as *const *const c_char;
        opts.cnt = (*ksyms).filtered_cnt;

        do_bench_test(skel, &mut opts);
        break 'cleanup;
    }

    kprobe_multi_empty__destroy(skel);
    free_kallsyms_local(ksyms);
}

unsafe fn test_kprobe_multi_bench_attach_addr(kernel: bool) {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut skel: *mut kprobe_multi_empty = ptr::null_mut();
    let mut addrs: *mut c_ulong = ptr::null_mut();
    let mut cnt: usize = 0;
    let err: c_int;

    err = bpf_get_addrs(&mut addrs, &mut cnt, kernel);
    if err == -ENOENT {
        test__skip();
        return;
    }

    if !ASSERT_OK(err, cstr!("bpf_get_addrs")) {
        return;
    }

    'cleanup: loop {
        skel = kprobe_multi_empty__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi_empty__open_and_load")) {
            break 'cleanup;
        }

        opts.addrs = addrs;
        opts.cnt = cnt;

        do_bench_test(skel, &mut opts);
        break 'cleanup;
    }

    kprobe_multi_empty__destroy(skel);
    free(addrs as *mut c_void);
}

unsafe fn test_attach_override() {
    let mut skel: *mut kprobe_multi_override = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();

    'cleanup: loop {
        skel = kprobe_multi_override__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi_empty__open_and_load")) {
            break 'cleanup;
        }

        /* The test_override calls bpf_override_return so it should fail
         * to attach to bpf_fentry_test1 function, which is not on error
         * injection list.
         */
        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_override, cstr!("bpf_fentry_test1"), ptr::null_mut());
        if !ASSERT_ERR_PTR(link, cstr!("override_attached_bpf_fentry_test1")) {
            bpf_link__destroy(link);
            break 'cleanup;
        }

        /* The should_fail_bio function is on error injection list,
         * attach should succeed.
         */
        link = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_override, cstr!("should_fail_bio"), ptr::null_mut());
        if !ASSERT_OK_PTR(link, cstr!("override_attached_should_fail_bio")) {
            break 'cleanup;
        }

        bpf_link__destroy(link);
        break 'cleanup;
    }

    kprobe_multi_override__destroy(skel);
}

unsafe fn test_override() {
    let mut skel: *mut kprobe_multi_override = ptr::null_mut();
    let mut err: c_int;

    'cleanup: loop {
        skel = kprobe_multi_override__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi_empty__open_and_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();

        /* no override */
        err = prctl(0xffff, 0);
        ASSERT_EQ(err, -1, cstr!("err"));

        /* kprobe.multi override */
        (*skel).links.test_override = bpf_program__attach_kprobe_multi_opts(
            (*skel).progs.test_override,
            concat!("__x64_", "sys_prctl", "\0").as_ptr() as *const c_char,
            ptr::null_mut(),
        );
        if !ASSERT_OK_PTR((*skel).links.test_override, cstr!("bpf_program__attach_kprobe_multi_opts")) {
            break 'cleanup;
        }

        err = prctl(0xffff, 0);
        ASSERT_EQ(err, 123, cstr!("err"));

        bpf_link__destroy((*skel).links.test_override);
        (*skel).links.test_override = ptr::null_mut();

        /* kprobe override */
        (*skel).links.test_kprobe_override = bpf_program__attach_kprobe(
            (*skel).progs.test_kprobe_override,
            false,
            concat!("__x64_", "sys_prctl", "\0").as_ptr() as *const c_char,
        );
        if !ASSERT_OK_PTR((*skel).links.test_kprobe_override, cstr!("bpf_program__attach_kprobe")) {
            break 'cleanup;
        }

        err = prctl(0xffff, 0);
        ASSERT_EQ(err, 123, cstr!("err"));
        break 'cleanup;
    }

    kprobe_multi_override__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_attach_write_ctx() {
    let mut skel: *mut kprobe_write_ctx = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();

    skel = kprobe_write_ctx__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr!("kprobe_write_ctx__open_and_load")) {
        return;
    }

    link = bpf_program__attach_kprobe_opts((*skel).progs.kprobe_multi_write_ctx, cstr!("bpf_fentry_test1"), ptr::null_mut());
    if !ASSERT_ERR_PTR(link, cstr!("bpf_program__attach_kprobe_opts")) {
        bpf_link__destroy(link);
    }

    kprobe_write_ctx__destroy(skel);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn test_attach_write_ctx() {
    test__skip();
}

/*
 * Test kprobe_multi handles shadow symbols (vmlinux + module duplicate).
 * bpf_fentry_shadow_test exists in both vmlinux and bpf_testmod.
 * kprobe_multi resolves via ftrace_lookup_symbols() which finds the
 * vmlinux symbol first and stops, so this should always succeed.
 */
unsafe fn test_attach_probe_dup_sym() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let syms: [*const c_char; 1] = [cstr!("bpf_fentry_shadow_test")];
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();

    'cleanup: loop {
        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel, cstr!("kprobe_multi__open_and_load")) {
            break 'cleanup;
        }

        (*(*skel).bss).pid = getpid();
        opts.syms = syms.as_ptr();
        opts.cnt = syms.len();

        link1 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe_manual, ptr::null(), &mut opts);
        if !ASSERT_OK_PTR(link1, cstr!("attach_kprobe_multi_dup_sym")) {
            break 'cleanup;
        }

        opts.retprobe = true;
        link2 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kretprobe_manual, ptr::null(), &mut opts);
        if !ASSERT_OK_PTR(link2, cstr!("attach_kretprobe_multi_dup_sym")) {
            break 'cleanup;
        }
        break 'cleanup;
    }

    bpf_link__destroy(link2);
    bpf_link__destroy(link1);
    kprobe_multi__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_kprobe_multi_bench_attach() {
    if test__start_subtest(cstr!("kernel")) {
        test_kprobe_multi_bench_attach(true);
    }
    if test__start_subtest(cstr!("modules")) {
        test_kprobe_multi_bench_attach(false);
    }
    if test__start_subtest(cstr!("kernel")) {
        test_kprobe_multi_bench_attach_addr(true);
    }
    if test__start_subtest(cstr!("modules")) {
        test_kprobe_multi_bench_attach_addr(false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_kprobe_multi_test() {
    if !ASSERT_OK(load_kallsyms(), cstr!("load_kallsyms")) {
        return;
    }

    if test__start_subtest(cstr!("skel_api")) {
        test_skel_api();
    }
    if test__start_subtest(cstr!("link_api_addrs")) {
        test_link_api_syms();
    }
    if test__start_subtest(cstr!("link_api_syms")) {
        test_link_api_addrs();
    }
    if test__start_subtest(cstr!("attach_api_pattern")) {
        test_attach_api_pattern();
    }
    if test__start_subtest(cstr!("attach_api_addrs")) {
        test_attach_api_addrs();
    }
    if test__start_subtest(cstr!("attach_api_syms")) {
        test_attach_api_syms();
    }
    if test__start_subtest(cstr!("attach_api_fails")) {
        test_attach_api_fails();
    }
    if test__start_subtest(cstr!("attach_override")) {
        test_attach_override();
    }
    if test__start_subtest(cstr!("override")) {
        test_override();
    }
    if test__start_subtest(cstr!("session")) {
        test_session_skel_api();
    }
    if test__start_subtest(cstr!("session_cookie")) {
        test_session_cookie_skel_api();
    }
    if test__start_subtest(cstr!("unique_match")) {
        test_unique_match();
    }
    if test__start_subtest(cstr!("attach_write_ctx")) {
        test_attach_write_ctx();
    }
    if test__start_subtest(cstr!("dup_sym")) {
        test_attach_probe_dup_sym();
    }
    RUN_TESTS_kprobe_multi_verifier();
}
