// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/uprobe_multi_test.c.
// External libbpf, libc, skeleton, and test helper symbols are expected from
// the surrounding selftest build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type __u64 = u64;
type pid_t = c_int;
type pthread_t = c_ulong;

const NULL: *mut c_void = ptr::null_mut();
const INT_MAX: c_int = c_int::MAX;
const CONSUMER_MAX: usize = 16;
const TASKS: usize = 3;

const CLONE_VM: c_int = 0x00000100;
const SIGCHLD: c_int = 17;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const EFAULT: c_int = 14;
const EBADF: c_int = 9;
const O_RDONLY: c_int = 0;
const BPF_TRACE_UPROBE_MULTI: c_int = 0;
const BPF_F_UPROBE_MULTI_RETURN: __u64 = 1;
const BPF_F_UPROBE_MULTI_PATH_FD: __u64 = 1 << 1;
const STT_FUNC: c_int = 2;
const STT_OBJECT: c_int = 1;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_uprobe_multi_opts {
    pub sz: usize,
    pub syms: *const *const c_char,
    pub offsets: *const c_ulong,
    pub ref_ctr_offsets: *const c_ulong,
    pub cookies: *const __u64,
    pub cnt: usize,
    pub retprobe: bool,
    pub session: bool,
}

#[repr(C)]
pub struct bpf_link_create_opts_uprobe_multi {
    pub path: *const c_char,
    pub path_fd: c_int,
    pub offsets: *const c_ulong,
    pub ref_ctr_offsets: *const c_ulong,
    pub cookies: *const __u64,
    pub cnt: usize,
    pub flags: __u64,
    pub pid: pid_t,
}

#[repr(C)]
pub struct bpf_link_create_opts_kprobe_multi {
    pub flags: __u64,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    pub sz: usize,
    pub uprobe_multi: bpf_link_create_opts_uprobe_multi,
    pub kprobe_multi: bpf_link_create_opts_kprobe_multi,
}

impl Default for bpf_uprobe_multi_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            syms: ptr::null(),
            offsets: ptr::null(),
            ref_ctr_offsets: ptr::null(),
            cookies: ptr::null(),
            cnt: 0,
            retprobe: false,
            session: false,
        }
    }
}

impl Default for bpf_link_create_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            uprobe_multi: bpf_link_create_opts_uprobe_multi {
                path: ptr::null(),
                path_fd: 0,
                offsets: ptr::null(),
                ref_ctr_offsets: ptr::null(),
                cookies: ptr::null(),
                cnt: 0,
                flags: 0,
                pid: 0,
            },
            kprobe_multi: bpf_link_create_opts_kprobe_multi { flags: 0 },
        }
    }
}

#[repr(C)]
pub struct uprobe_multi_bss {
    pub uprobe_multi_func_1_addr: __u64,
    pub uprobe_multi_func_2_addr: __u64,
    pub uprobe_multi_func_3_addr: __u64,
    pub user_ptr: *mut c_char,
    pub pid: pid_t,
    pub expect_pid: pid_t,
    pub uprobe_multi_func_1_result: __u64,
    pub uprobe_multi_func_2_result: __u64,
    pub uprobe_multi_func_3_result: __u64,
    pub uretprobe_multi_func_1_result: __u64,
    pub uretprobe_multi_func_2_result: __u64,
    pub uretprobe_multi_func_3_result: __u64,
    pub uprobe_multi_sleep_result: __u64,
    pub bad_pid_seen: bool,
    pub child_pid: pid_t,
    pub child_tid: pid_t,
    pub bad_pid_seen_usdt: bool,
    pub child_pid_usdt: pid_t,
    pub child_tid_usdt: pid_t,
}

#[repr(C)]
pub struct uprobe_multi_progs {
    pub uprobe: *mut bpf_program,
    pub uretprobe: *mut bpf_program,
    pub uprobe_sleep: *mut bpf_program,
    pub uretprobe_sleep: *mut bpf_program,
    pub uprobe_extra: *mut bpf_program,
    pub usdt_pid: *mut bpf_program,
    pub usdt_extra: *mut bpf_program,
}

#[repr(C)]
pub struct uprobe_multi_links {
    pub uprobe: *mut bpf_link,
    pub uretprobe: *mut bpf_link,
    pub uprobe_sleep: *mut bpf_link,
    pub uretprobe_sleep: *mut bpf_link,
    pub uprobe_extra: *mut bpf_link,
    pub usdt_pid: *mut bpf_link,
    pub usdt_extra: *mut bpf_link,
}

#[repr(C)]
pub struct uprobe_multi {
    pub bss: *mut uprobe_multi_bss,
    pub progs: uprobe_multi_progs,
    pub links: uprobe_multi_links,
}

#[repr(C)]
pub struct uprobe_multi_consumers_bss {
    pub uprobe_result: [__u64; 4],
}

#[repr(C)]
pub struct uprobe_multi_consumers_progs {
    pub uprobe_0: *mut bpf_program,
    pub uprobe_1: *mut bpf_program,
    pub uprobe_2: *mut bpf_program,
    pub uprobe_3: *mut bpf_program,
}

#[repr(C)]
pub struct uprobe_multi_consumers_links {
    pub uprobe_0: *mut bpf_link,
    pub uprobe_1: *mut bpf_link,
    pub uprobe_2: *mut bpf_link,
    pub uprobe_3: *mut bpf_link,
}

#[repr(C)]
pub struct uprobe_multi_consumers {
    pub bss: *mut uprobe_multi_consumers_bss,
    pub progs: uprobe_multi_consumers_progs,
    pub links: uprobe_multi_consumers_links,
}

#[repr(C)]
pub struct uprobe_multi_pid_filter_bss {
    pub test: [[__u64; 2]; TASKS],
    pub pids: [pid_t; TASKS],
}

#[repr(C)]
pub struct uprobe_multi_pid_filter_progs {
    pub uprobe_multi_0: *mut bpf_program,
    pub uprobe_multi_1: *mut bpf_program,
    pub uprobe_multi_2: *mut bpf_program,
}

#[repr(C)]
pub struct uprobe_multi_pid_filter {
    pub bss: *mut uprobe_multi_pid_filter_bss,
    pub progs: uprobe_multi_pid_filter_progs,
}

#[repr(C)]
pub struct uprobe_multi_session_bss {
    pub pid: pid_t,
    pub user_ptr: *mut c_char,
    pub uprobe_multi_func_1_addr: __u64,
    pub uprobe_multi_func_2_addr: __u64,
    pub uprobe_multi_func_3_addr: __u64,
    pub uprobe_session_result: [__u64; 3],
    pub uprobe_multi_sleep_result: __u64,
}

#[repr(C)]
pub struct uprobe_multi_session {
    pub bss: *mut uprobe_multi_session_bss,
}

#[repr(C)]
pub struct uprobe_multi_session_single_bss {
    pub pid: pid_t,
    pub uprobe_session_result: [__u64; 3],
}

#[repr(C)]
pub struct uprobe_multi_session_single {
    pub bss: *mut uprobe_multi_session_single_bss,
}

#[repr(C)]
pub struct uprobe_multi_session_cookie_bss {
    pub pid: pid_t,
    pub test_uprobe_1_result: __u64,
    pub test_uprobe_2_result: __u64,
    pub test_uprobe_3_result: __u64,
}

#[repr(C)]
pub struct uprobe_multi_session_cookie {
    pub bss: *mut uprobe_multi_session_cookie_bss,
}

#[repr(C)]
pub struct uprobe_multi_session_recursive_bss {
    pub pid: pid_t,
    pub test_uprobe_cookie_entry: [__u64; 6],
    pub test_uprobe_cookie_return: [__u64; 3],
    pub idx_entry: __u64,
    pub idx_return: __u64,
}

#[repr(C)]
pub struct uprobe_multi_session_recursive {
    pub bss: *mut uprobe_multi_session_recursive_bss,
}

#[repr(C)]
pub struct uprobe_multi_bench_bss {
    pub count: __u64,
}

#[repr(C)]
pub struct uprobe_multi_bench {
    pub bss: *mut uprobe_multi_bench_bss,
}

#[repr(C)]
pub struct uprobe_multi_usdt_bss {
    pub count: __u64,
}

#[repr(C)]
pub struct uprobe_multi_usdt_progs {
    pub usdt0: *mut bpf_program,
}

#[repr(C)]
pub struct uprobe_multi_usdt_links {
    pub usdt0: *mut bpf_link,
}

#[repr(C)]
pub struct uprobe_multi_usdt {
    pub bss: *mut uprobe_multi_usdt_bss,
    pub progs: uprobe_multi_usdt_progs,
    pub links: uprobe_multi_usdt_links,
}

unsafe extern "C" {
    static mut errno: c_int;
    static __start_consumers: *const c_void;

    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn fork() -> pid_t;
    fn clone(
        f: extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn getpid() -> pid_t;
    fn sys_gettid() -> pid_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn fflush(stream: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn get_time_ns() -> c_long;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_FAIL(name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_uprobe_multi_verifier();

    fn uprobe_multi__open_and_load() -> *mut uprobe_multi;
    fn uprobe_multi__attach(skel: *mut uprobe_multi) -> c_int;
    fn uprobe_multi__destroy(skel: *mut uprobe_multi);
    fn uprobe_multi_consumers__open_and_load() -> *mut uprobe_multi_consumers;
    fn uprobe_multi_consumers__destroy(skel: *mut uprobe_multi_consumers);
    fn uprobe_multi_pid_filter__open_and_load() -> *mut uprobe_multi_pid_filter;
    fn uprobe_multi_pid_filter__destroy(skel: *mut uprobe_multi_pid_filter);
    fn uprobe_multi_session__open_and_load() -> *mut uprobe_multi_session;
    fn uprobe_multi_session__attach(skel: *mut uprobe_multi_session) -> c_int;
    fn uprobe_multi_session__destroy(skel: *mut uprobe_multi_session);
    fn uprobe_multi_session_single__open_and_load() -> *mut uprobe_multi_session_single;
    fn uprobe_multi_session_single__attach(skel: *mut uprobe_multi_session_single) -> c_int;
    fn uprobe_multi_session_single__destroy(skel: *mut uprobe_multi_session_single);
    fn uprobe_multi_session_cookie__open_and_load() -> *mut uprobe_multi_session_cookie;
    fn uprobe_multi_session_cookie__attach(skel: *mut uprobe_multi_session_cookie) -> c_int;
    fn uprobe_multi_session_cookie__destroy(skel: *mut uprobe_multi_session_cookie);
    fn uprobe_multi_session_recursive__open_and_load() -> *mut uprobe_multi_session_recursive;
    fn uprobe_multi_session_recursive__attach(skel: *mut uprobe_multi_session_recursive) -> c_int;
    fn uprobe_multi_session_recursive__destroy(skel: *mut uprobe_multi_session_recursive);
    fn uprobe_multi_bench__open_and_load() -> *mut uprobe_multi_bench;
    fn uprobe_multi_bench__attach(skel: *mut uprobe_multi_bench) -> c_int;
    fn uprobe_multi_bench__destroy(skel: *mut uprobe_multi_bench);
    fn uprobe_multi_usdt__open_and_load() -> *mut uprobe_multi_usdt;
    fn uprobe_multi_usdt__destroy(skel: *mut uprobe_multi_usdt);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    fn bpf_program__attach_uprobe_multi(
        prog: *mut bpf_program,
        pid: pid_t,
        binary_path: *const c_char,
        func_pattern: *const c_char,
        opts: *mut bpf_uprobe_multi_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_usdt(
        prog: *mut bpf_program,
        pid: pid_t,
        binary_path: *const c_char,
        usdt_provider: *const c_char,
        usdt_name: *const c_char,
        opts: *const c_void,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn elf_resolve_syms_offsets(
        binary_path: *const c_char,
        cnt: usize,
        syms: *const *const c_char,
        poffsets: *mut *mut c_ulong,
        st_type: c_int,
    ) -> c_int;
    fn get_uprobe_offset(func: *const c_void) -> c_long;
}

static mut test_data: [c_char; 10] = *b"test_data\0";
static mut sema_1: i16 = 0;
static mut sema_2: i16 = 0;

#[inline(never)]
pub extern "C" fn uprobe_multi_func_1() {
    core::hint::spin_loop();
}

#[inline(never)]
pub extern "C" fn uprobe_multi_func_2() {
    core::hint::spin_loop();
}

#[inline(never)]
pub extern "C" fn uprobe_multi_func_3() {
    core::hint::spin_loop();
}

#[inline(never)]
pub extern "C" fn usdt_trigger() {
    // STAP_PROBE(test, pid_filter_usdt);
}

#[inline(never)]
pub extern "C" fn uprobe_session_recursive(i: c_int) {
    if i != 0 {
        uprobe_session_recursive(i - 1);
    }
}

#[repr(C)]
pub struct child {
    pub go: [c_int; 2],
    pub c2p: [c_int; 2], /* child -> parent channel */
    pub pid: c_int,
    pub tid: c_int,
    pub thread: pthread_t,
    pub stack: [c_char; 65536],
}

unsafe fn release_child(child: *mut child) {
    let mut child_status: c_int = 0;

    if child.is_null() {
        return;
    }
    close((*child).go[1]);
    close((*child).go[0]);
    if (*child).thread != 0 {
        pthread_join((*child).thread, ptr::null_mut());
        (*child).thread = 0;
    }
    close((*child).c2p[0]);
    close((*child).c2p[1]);
    if (*child).pid > 0 {
        waitpid((*child).pid, &mut child_status, 0);
    }
}

unsafe fn kick_child(child: *mut child) {
    let c: c_char = 1;

    if !child.is_null() {
        write((*child).go[1], &c as *const _ as *const c_void, 1);
        release_child(child);
    }
    fflush(ptr::null_mut());
}

extern "C" fn child_func(arg: *mut c_void) -> c_int {
    unsafe {
        let child = arg as *mut child;
        let mut c: c_int = 0;

        close((*child).go[1]);

        /* wait for parent's kick */
        let err = read((*child).go[0], &mut c as *mut _ as *mut c_void, 1);
        if err != 1 {
            exit(err as c_int);
        }

        uprobe_multi_func_1();
        uprobe_multi_func_2();
        uprobe_multi_func_3();
        usdt_trigger();

        exit(errno);
    }
}

unsafe fn spawn_child_flag(child: *mut child, clone_vm: bool) -> c_int {
    /* pipe to notify child to execute the trigger functions */
    if pipe((*child).go.as_mut_ptr()) != 0 {
        return -1;
    }

    if clone_vm {
        let stack = (*child).stack.as_mut_ptr().add(core::mem::size_of_val(&(*child).stack) / 2);
        (*child).pid = clone(
            child_func,
            stack as *mut c_void,
            CLONE_VM | SIGCHLD,
            child as *mut c_void,
        );
        (*child).tid = (*child).pid;
    } else {
        (*child).pid = fork();
        (*child).tid = (*child).pid;
    }
    if (*child).pid < 0 {
        release_child(child);
        errno = EINVAL;
        return -1;
    }

    /* fork-ed child */
    if !clone_vm && (*child).pid == 0 {
        child_func(child as *mut c_void);
    }

    0
}

unsafe fn spawn_child(child: *mut child) -> c_int {
    spawn_child_flag(child, false)
}

extern "C" fn child_thread(ctx: *mut c_void) -> *mut c_void {
    unsafe {
        let child = ctx as *mut child;
        let mut c: c_int = 0;
        let mut err: c_int;

        (*child).tid = sys_gettid();

        /* let parent know we are ready */
        err = write((*child).c2p[1], &c as *const _ as *const c_void, 1) as c_int;
        if err != 1 {
            pthread_exit(&mut err as *mut _ as *mut c_void);
        }

        /* wait for parent's kick */
        err = read((*child).go[0], &mut c as *mut _ as *mut c_void, 1) as c_int;
        if err != 1 {
            pthread_exit(&mut err as *mut _ as *mut c_void);
        }

        uprobe_multi_func_1();
        uprobe_multi_func_2();
        uprobe_multi_func_3();
        usdt_trigger();

        err = 0;
        pthread_exit(&mut err as *mut _ as *mut c_void);
    }
}

unsafe fn spawn_thread(child: *mut child) -> c_int {
    let mut c: c_int = 0;
    let mut err: c_int;

    /* pipe to notify child to execute the trigger functions */
    if pipe((*child).go.as_mut_ptr()) != 0 {
        return -1;
    }
    /* pipe to notify parent that child thread is ready */
    if pipe((*child).c2p.as_mut_ptr()) != 0 {
        close((*child).go[0]);
        close((*child).go[1]);
        return -1;
    }

    (*child).pid = getpid();

    err = pthread_create(&mut (*child).thread, ptr::null(), child_thread, child as *mut c_void);
    if err != 0 {
        err = -errno;
        close((*child).go[0]);
        close((*child).go[1]);
        close((*child).c2p[0]);
        close((*child).c2p[1]);
        errno = -err;
        return -1;
    }

    err = read((*child).c2p[0], &mut c as *mut _ as *mut c_void, 1) as c_int;
    if !ASSERT_EQ(err, 1, c"child_thread_ready".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn uprobe_multi_test_run(skel: *mut uprobe_multi, child: *mut child) {
    (*(*skel).bss).uprobe_multi_func_1_addr = uprobe_multi_func_1 as usize as __u64;
    (*(*skel).bss).uprobe_multi_func_2_addr = uprobe_multi_func_2 as usize as __u64;
    (*(*skel).bss).uprobe_multi_func_3_addr = uprobe_multi_func_3 as usize as __u64;

    (*(*skel).bss).user_ptr = test_data.as_mut_ptr();

    /*
     * Disable pid check in bpf program if we are pid filter test,
     * because the probe should be executed only by child->pid
     * passed at the probe attach.
     */
    (*(*skel).bss).pid = if !child.is_null() { 0 } else { getpid() };
    (*(*skel).bss).expect_pid = if !child.is_null() { (*child).pid } else { 0 };

    /* trigger all probes, if we are testing child *process*, just to make
     * sure that PID filtering doesn't let through activations from wrong
     * PIDs; when we test child *thread*, we don't want to do this to
     * avoid double counting number of triggering events
     */
    if child.is_null() || (*child).thread == 0 {
        uprobe_multi_func_1();
        uprobe_multi_func_2();
        uprobe_multi_func_3();
        usdt_trigger();
    }

    if !child.is_null() {
        kick_child(child);
    }

    /*
     * There are 2 entry and 2 exit probe called for each uprobe_multi_func_[123]
     * function and each sleepable probe (6) increments uprobe_multi_sleep_result.
     */
    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_1_result, 2, c"uprobe_multi_func_1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_2_result, 2, c"uprobe_multi_func_2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_3_result, 2, c"uprobe_multi_func_3_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_1_result, 2, c"uretprobe_multi_func_1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_2_result, 2, c"uretprobe_multi_func_2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_3_result, 2, c"uretprobe_multi_func_3_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_multi_sleep_result, 6, c"uprobe_multi_sleep_result".as_ptr());
    ASSERT_FALSE((*(*skel).bss).bad_pid_seen, c"bad_pid_seen".as_ptr());

    if !child.is_null() {
        ASSERT_EQ((*(*skel).bss).child_pid, (*child).pid, c"uprobe_multi_child_pid".as_ptr());
        ASSERT_EQ((*(*skel).bss).child_tid, (*child).tid, c"uprobe_multi_child_tid".as_ptr());
    }
}

unsafe fn test_skel_api() {
    let mut skel: *mut uprobe_multi = ptr::null_mut();

    skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    let err = uprobe_multi__attach(skel);
    if ASSERT_OK(err, c"uprobe_multi__attach".as_ptr()) {
        uprobe_multi_test_run(skel, ptr::null_mut());
    }

    uprobe_multi__destroy(skel);
}

unsafe fn __test_attach_api(
    binary: *const c_char,
    pattern: *const c_char,
    opts: *mut bpf_uprobe_multi_opts,
    child: *mut child,
) {
    let pid: pid_t = if !child.is_null() { (*child).pid } else { -1 };
    let skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*opts).retprobe = false;
    (*skel).links.uprobe = bpf_program__attach_uprobe_multi((*skel).progs.uprobe, pid, binary, pattern, opts);
    if !ASSERT_OK_PTR((*skel).links.uprobe, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*opts).retprobe = true;
    (*skel).links.uretprobe = bpf_program__attach_uprobe_multi((*skel).progs.uretprobe, pid, binary, pattern, opts);
    if !ASSERT_OK_PTR((*skel).links.uretprobe, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*opts).retprobe = false;
    (*skel).links.uprobe_sleep = bpf_program__attach_uprobe_multi((*skel).progs.uprobe_sleep, pid, binary, pattern, opts);
    if !ASSERT_OK_PTR((*skel).links.uprobe_sleep, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*opts).retprobe = true;
    (*skel).links.uretprobe_sleep = bpf_program__attach_uprobe_multi((*skel).progs.uretprobe_sleep, pid, binary, pattern, opts);
    if !ASSERT_OK_PTR((*skel).links.uretprobe_sleep, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*opts).retprobe = false;
    (*skel).links.uprobe_extra = bpf_program__attach_uprobe_multi((*skel).progs.uprobe_extra, -1, binary, pattern, opts);
    if !ASSERT_OK_PTR((*skel).links.uprobe_extra, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    /* Attach (uprobe-backed) USDTs */
    (*skel).links.usdt_pid = bpf_program__attach_usdt((*skel).progs.usdt_pid, pid, binary, c"test".as_ptr(), c"pid_filter_usdt".as_ptr(), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.usdt_pid, c"attach_usdt_pid".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    (*skel).links.usdt_extra = bpf_program__attach_usdt((*skel).progs.usdt_extra, -1, binary, c"test".as_ptr(), c"pid_filter_usdt".as_ptr(), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.usdt_extra, c"attach_usdt_extra".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    uprobe_multi_test_run(skel, child);
    ASSERT_FALSE((*(*skel).bss).bad_pid_seen_usdt, c"bad_pid_seen_usdt".as_ptr());
    if !child.is_null() {
        ASSERT_EQ((*(*skel).bss).child_pid_usdt, (*child).pid, c"usdt_multi_child_pid".as_ptr());
        ASSERT_EQ((*(*skel).bss).child_tid_usdt, (*child).tid, c"usdt_multi_child_tid".as_ptr());
    }
    uprobe_multi__destroy(skel);
}

unsafe fn test_attach_api(binary: *const c_char, pattern: *const c_char, opts: *mut bpf_uprobe_multi_opts) {
    static mut child: child = child { go: [0; 2], c2p: [0; 2], pid: 0, tid: 0, thread: 0, stack: [0; 65536] };

    memset(&raw mut child as *mut c_void, 0, core::mem::size_of::<child>());
    __test_attach_api(binary, pattern, opts, ptr::null_mut());

    /* pid filter */
    if !ASSERT_OK(spawn_child(&raw mut child), c"spawn_child".as_ptr()) {
        return;
    }
    __test_attach_api(binary, pattern, opts, &raw mut child);

    /* pid filter (thread) */
    if !ASSERT_OK(spawn_thread(&raw mut child), c"spawn_thread".as_ptr()) {
        return;
    }
    __test_attach_api(binary, pattern, opts, &raw mut child);
}

unsafe fn test_attach_api_pattern() {
    let mut opts = bpf_uprobe_multi_opts::default();
    test_attach_api(c"/proc/self/exe".as_ptr(), c"uprobe_multi_func_*".as_ptr(), &mut opts);
    test_attach_api(c"/proc/self/exe".as_ptr(), c"uprobe_multi_func_?".as_ptr(), &mut opts);
}

unsafe fn test_attach_api_syms() {
    let mut opts = bpf_uprobe_multi_opts::default();
    let syms = [
        c"uprobe_multi_func_1".as_ptr(),
        c"uprobe_multi_func_2".as_ptr(),
        c"uprobe_multi_func_3".as_ptr(),
    ];
    opts.syms = syms.as_ptr();
    opts.cnt = syms.len();
    test_attach_api(c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
}

unsafe fn test_attach_api_fails() {
    let mut opts = bpf_link_create_opts::default();
    let path = c"/proc/self/exe".as_ptr();
    let mut link_fd: c_int = -1;
    let mut offset: c_ulong = 0;

    let skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    let prog_fd = bpf_program__fd((*skel).progs.uprobe_extra);

    /* abnormal cnt */
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = INT_MAX as usize;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -E2BIG, c"big cnt".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    /* cnt is 0 */
    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"cnt_is_zero".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    /* negative offset */
    offset = !0;
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"offset_is_negative".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    /* Representative translation of the remaining failure cases follows the
     * same LIBBPF_OPTS_RESET pattern from C.
     */
    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"offsets_is_null".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = 1 as *const c_ulong;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EFAULT, c"offsets_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    offset = 1;
    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"path_is_null".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = 1 as *const c_char;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EFAULT, c"path_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = c"/".as_ptr();
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EBADF, c"path_is_wrong_type".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cookies = 1 as *const __u64;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EFAULT, c"cookies_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cookies = &offset as *const _ as *const __u64;
    opts.uprobe_multi.ref_ctr_offsets = 1 as *const c_ulong;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EFAULT, c"ref_ctr_offsets_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.flags = 1u64 << 31;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"wrong_flags".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    opts.uprobe_multi.pid = -2;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EINVAL, c"pid_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = ptr::null();
    opts.uprobe_multi.path_fd = -1;
    opts.uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) || !ASSERT_EQ(link_fd, -EBADF, c"path_fd_is_wrong".as_ptr()) {
        uprobe_multi__destroy(skel);
        return;
    }

    opts = bpf_link_create_opts::default();
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.path_fd = 1;
    opts.uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if ASSERT_ERR(link_fd, c"link_fd".as_ptr()) {
        ASSERT_EQ(link_fd, -EINVAL, c"path_and_path_fd_together".as_ptr());
    }

    if link_fd >= 0 {
        close(link_fd);
    }
    uprobe_multi__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub extern "C" fn uprobe_multi_error_func() {
    // Original C emits a global label uprobe_multi_error_func_int3 and an int3 instruction.
    unsafe { core::arch::asm!(".globl uprobe_multi_error_func_int3", "uprobe_multi_error_func_int3:", "int3") };
}

unsafe fn attach_uprobe_fail_trap(skel: *mut uprobe_multi) {
    #[cfg(target_arch = "x86_64")]
    {
        let mut opts = bpf_uprobe_multi_opts::default();
        let syms = [
            c"uprobe_multi_func_1".as_ptr(),
            c"uprobe_multi_func_2".as_ptr(),
            c"uprobe_multi_func_3".as_ptr(),
            c"uprobe_multi_error_func_int3".as_ptr(),
        ];
        opts.syms = syms.as_ptr();
        opts.cnt = syms.len();
        (*skel).links.uprobe = bpf_program__attach_uprobe_multi((*skel).progs.uprobe, -1, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
        if !ASSERT_ERR_PTR((*skel).links.uprobe, c"bpf_program__attach_uprobe_multi".as_ptr()) {
            bpf_link__destroy((*skel).links.uprobe);
            (*skel).links.uprobe = ptr::null_mut();
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = skel;
    }
}

unsafe fn attach_uprobe_fail_refctr(skel: *mut uprobe_multi) {
    let mut tmp_offsets: *mut c_ulong = ptr::null_mut();
    let mut tmp_ref_ctr_offsets: *mut c_ulong = ptr::null_mut();
    let mut offsets = [0 as c_ulong; 3];
    let mut ref_ctr_offsets = [0 as c_ulong; 3];
    let mut opts = bpf_link_create_opts::default();
    let path = c"/proc/self/exe".as_ptr();
    let syms = [c"uprobe_multi_func_1".as_ptr(), c"uprobe_multi_func_2".as_ptr()];
    let sema = [c"sema_1".as_ptr(), c"sema_2".as_ptr()];
    let prog_fd = bpf_program__fd((*skel).progs.uprobe_extra);

    let mut err = elf_resolve_syms_offsets(c"/proc/self/exe".as_ptr(), 2, syms.as_ptr(), &mut tmp_offsets, STT_FUNC);
    if !ASSERT_OK(err, c"elf_resolve_syms_offsets_func".as_ptr()) {
        return;
    }

    err = elf_resolve_syms_offsets(c"/proc/self/exe".as_ptr(), 2, sema.as_ptr(), &mut tmp_ref_ctr_offsets, STT_OBJECT);
    if !ASSERT_OK(err, c"elf_resolve_syms_offsets_sema".as_ptr()) {
        free(tmp_offsets as *mut c_void);
        return;
    }

    /*
     * We attach to 3 uprobes on 2 functions, so 2 uprobes share single function,
     * but with different ref_ctr_offset which is not allowed and results in fail.
     */
    offsets[0] = *tmp_offsets.add(0); /* uprobe_multi_func_1 */
    offsets[1] = *tmp_offsets.add(1); /* uprobe_multi_func_2 */
    offsets[2] = *tmp_offsets.add(1); /* uprobe_multi_func_2 */

    ref_ctr_offsets[0] = *tmp_ref_ctr_offsets.add(0); /* sema_1 */
    ref_ctr_offsets[1] = *tmp_ref_ctr_offsets.add(1); /* sema_2 */
    ref_ctr_offsets[2] = *tmp_ref_ctr_offsets.add(0); /* sema_1, error */

    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = offsets.as_ptr();
    opts.uprobe_multi.ref_ctr_offsets = ref_ctr_offsets.as_ptr();
    opts.uprobe_multi.cnt = 3;

    let link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_ERR(link_fd, c"link_fd".as_ptr()) {
        close(link_fd);
    }

    free(tmp_ref_ctr_offsets as *mut c_void);
    free(tmp_offsets as *mut c_void);
}

unsafe fn test_attach_uprobe_fails() {
    let skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        return;
    }

    /* attach fails due to adding uprobe on trap instruction, x86_64 only */
    attach_uprobe_fail_trap(skel);

    /* attach fail due to wrong ref_ctr_offs on one of the uprobes */
    attach_uprobe_fail_refctr(skel);

    uprobe_multi__destroy(skel);
}

unsafe fn __test_link_api(child: *mut child) {
    let mut link1_fd = -1;
    let mut link2_fd = -1;
    let mut link3_fd = -1;
    let mut link4_fd = -1;
    let mut opts = bpf_link_create_opts::default();
    let path = c"/proc/self/exe".as_ptr();
    let mut offsets: *mut c_ulong = ptr::null_mut();
    let syms = [c"uprobe_multi_func_1".as_ptr(), c"uprobe_multi_func_2".as_ptr(), c"uprobe_multi_func_3".as_ptr()];
    let mut link_extra_fd = -1;

    let err = elf_resolve_syms_offsets(path, 3, syms.as_ptr(), &mut offsets, STT_FUNC);
    if !ASSERT_OK(err, c"elf_resolve_syms_offsets".as_ptr()) {
        return;
    }

    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = offsets;
    opts.uprobe_multi.cnt = syms.len();
    opts.uprobe_multi.pid = if !child.is_null() { (*child).pid } else { 0 };

    let skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        free(offsets as *mut c_void);
        return;
    }

    opts.kprobe_multi.flags = 0;
    link1_fd = bpf_link_create(bpf_program__fd((*skel).progs.uprobe), 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_GE(link1_fd, 0, c"link1_fd".as_ptr()) { goto_link_cleanup(skel, offsets, link1_fd, link2_fd, link3_fd, link4_fd, link_extra_fd); return; }

    opts.kprobe_multi.flags = BPF_F_UPROBE_MULTI_RETURN;
    link2_fd = bpf_link_create(bpf_program__fd((*skel).progs.uretprobe), 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_GE(link2_fd, 0, c"link2_fd".as_ptr()) { goto_link_cleanup(skel, offsets, link1_fd, link2_fd, link3_fd, link4_fd, link_extra_fd); return; }

    opts.kprobe_multi.flags = 0;
    link3_fd = bpf_link_create(bpf_program__fd((*skel).progs.uprobe_sleep), 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_GE(link3_fd, 0, c"link3_fd".as_ptr()) { goto_link_cleanup(skel, offsets, link1_fd, link2_fd, link3_fd, link4_fd, link_extra_fd); return; }

    opts.kprobe_multi.flags = BPF_F_UPROBE_MULTI_RETURN;
    link4_fd = bpf_link_create(bpf_program__fd((*skel).progs.uretprobe_sleep), 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if !ASSERT_GE(link4_fd, 0, c"link4_fd".as_ptr()) { goto_link_cleanup(skel, offsets, link1_fd, link2_fd, link3_fd, link4_fd, link_extra_fd); return; }

    opts.kprobe_multi.flags = 0;
    opts.uprobe_multi.pid = 0;
    link_extra_fd = bpf_link_create(bpf_program__fd((*skel).progs.uprobe_extra), 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if ASSERT_GE(link_extra_fd, 0, c"link_extra_fd".as_ptr()) {
        uprobe_multi_test_run(skel, child);
    }

    goto_link_cleanup(skel, offsets, link1_fd, link2_fd, link3_fd, link4_fd, link_extra_fd);
}

unsafe fn goto_link_cleanup(skel: *mut uprobe_multi, offsets: *mut c_ulong, link1_fd: c_int, link2_fd: c_int, link3_fd: c_int, link4_fd: c_int, link_extra_fd: c_int) {
    if link1_fd >= 0 { close(link1_fd); }
    if link2_fd >= 0 { close(link2_fd); }
    if link3_fd >= 0 { close(link3_fd); }
    if link4_fd >= 0 { close(link4_fd); }
    if link_extra_fd >= 0 { close(link_extra_fd); }
    uprobe_multi__destroy(skel);
    free(offsets as *mut c_void);
}

unsafe fn test_link_api() {
    static mut child: child = child { go: [0; 2], c2p: [0; 2], pid: 0, tid: 0, thread: 0, stack: [0; 65536] };
    __test_link_api(ptr::null_mut());
    if !ASSERT_OK(spawn_child(&raw mut child), c"spawn_child".as_ptr()) { return; }
    __test_link_api(&raw mut child);
    if !ASSERT_OK(spawn_thread(&raw mut child), c"spawn_thread".as_ptr()) { return; }
    __test_link_api(&raw mut child);
}

unsafe fn test_link_api_path_fd() {
    let mut opts = bpf_link_create_opts::default();
    let resolve_path = c"/proc/self/exe".as_ptr();
    let mut link_fd = -1;
    let mut path_fd = -1;
    let mut offsets: *mut c_ulong = ptr::null_mut();
    let syms = [c"uprobe_multi_func_1".as_ptr(), c"uprobe_multi_func_2".as_ptr(), c"uprobe_multi_func_3".as_ptr()];

    let err = elf_resolve_syms_offsets(resolve_path, syms.len(), syms.as_ptr(), &mut offsets, STT_FUNC);
    if !ASSERT_OK(err, c"elf_resolve_syms_offsets".as_ptr()) { return; }

    path_fd = open(resolve_path, O_RDONLY);
    if !ASSERT_GE(path_fd, 0, c"path_fd".as_ptr()) { free(offsets as *mut c_void); return; }

    opts.uprobe_multi.path_fd = path_fd;
    opts.uprobe_multi.offsets = offsets;
    opts.uprobe_multi.cnt = syms.len();
    opts.uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD;

    let skel = uprobe_multi__open_and_load();
    if ASSERT_OK_PTR(skel, c"uprobe_multi__open_and_load".as_ptr()) {
        let prog_fd = bpf_program__fd((*skel).progs.uprobe);
        link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
        if ASSERT_GE(link_fd, 0, c"bpf_link_create".as_ptr()) {
            (*(*skel).bss).uprobe_multi_func_1_addr = uprobe_multi_func_1 as usize as __u64;
            (*(*skel).bss).uprobe_multi_func_2_addr = uprobe_multi_func_2 as usize as __u64;
            (*(*skel).bss).uprobe_multi_func_3_addr = uprobe_multi_func_3 as usize as __u64;
            (*(*skel).bss).pid = getpid();
            uprobe_multi_func_1();
            uprobe_multi_func_2();
            uprobe_multi_func_3();
            ASSERT_EQ((*(*skel).bss).uprobe_multi_func_1_result, 1, c"uprobe_multi_func_1_result".as_ptr());
            ASSERT_EQ((*(*skel).bss).uprobe_multi_func_2_result, 1, c"uprobe_multi_func_2_result".as_ptr());
            ASSERT_EQ((*(*skel).bss).uprobe_multi_func_3_result, 1, c"uprobe_multi_func_3_result".as_ptr());
        }
        if link_fd >= 0 { close(link_fd); }
        uprobe_multi__destroy(skel);
    }
    if path_fd >= 0 { close(path_fd); }
    free(offsets as *mut c_void);
}

unsafe fn get_program(skel: *mut uprobe_multi_consumers, prog: c_int) -> *mut bpf_program {
    match prog {
        0 => (*skel).progs.uprobe_0,
        1 => (*skel).progs.uprobe_1,
        2 => (*skel).progs.uprobe_2,
        3 => (*skel).progs.uprobe_3,
        _ => {
            ASSERT_FAIL(c"get_program".as_ptr());
            ptr::null_mut()
        }
    }
}

unsafe fn get_link(skel: *mut uprobe_multi_consumers, link: c_int) -> *mut *mut bpf_link {
    match link {
        0 => &mut (*skel).links.uprobe_0,
        1 => &mut (*skel).links.uprobe_1,
        2 => &mut (*skel).links.uprobe_2,
        3 => &mut (*skel).links.uprobe_3,
        _ => {
            ASSERT_FAIL(c"get_link".as_ptr());
            ptr::null_mut()
        }
    }
}

unsafe fn uprobe_attach(skel: *mut uprobe_multi_consumers, idx: c_int, offset: c_ulong) -> c_int {
    let prog = get_program(skel, idx);
    let link = get_link(skel, idx);
    let mut opts = bpf_uprobe_multi_opts::default();

    if prog.is_null() || link.is_null() {
        return -1;
    }

    opts.offsets = &offset;
    opts.cnt = 1;
    /*
     * bit/prog: 0 uprobe entry
     * bit/prog: 1 uprobe return
     * bit/prog: 2 uprobe session without return
     * bit/prog: 3 uprobe session with return
     */
    opts.retprobe = idx == 1;
    opts.session = idx == 2 || idx == 3;

    *link = bpf_program__attach_uprobe_multi(prog, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if !ASSERT_OK_PTR(*link, c"bpf_program__attach_uprobe_multi".as_ptr()) { return -1; }
    0
}

unsafe fn uprobe_detach(skel: *mut uprobe_multi_consumers, idx: c_int) {
    let link = get_link(skel, idx);
    bpf_link__destroy(*link);
    *link = ptr::null_mut();
}

fn test_bit(bit: c_int, val: c_ulong) -> bool {
    (val & (1u64 << bit) as c_ulong) != 0
}

#[inline(never)]
pub unsafe extern "C" fn uprobe_consumer_test(skel: *mut uprobe_multi_consumers, before: c_ulong, after: c_ulong, offset: c_ulong) -> c_int {
    for idx in 0..4 {
        if test_bit(idx, before) && !test_bit(idx, after) {
            uprobe_detach(skel, idx);
        }
    }
    for idx in 0..4 {
        if !test_bit(idx, before) && test_bit(idx, after) {
            if !ASSERT_OK(uprobe_attach(skel, idx, offset), c"uprobe_attach_after".as_ptr()) { return -1; }
        }
    }
    0
}

type test_t = unsafe extern "C" fn(*mut uprobe_multi_consumers, c_ulong, c_ulong, c_ulong) -> c_int;

macro_rules! consumer_test_fn {
    ($name:ident) => {
        #[inline(never)]
        pub unsafe extern "C" fn $name(skel: *mut uprobe_multi_consumers, before: c_ulong, after: c_ulong, offset: c_ulong) -> c_int {
            uprobe_consumer_test(skel, before, after, offset)
        }
    };
}

consumer_test_fn!(consumer_test0);
consumer_test_fn!(consumer_test1);
consumer_test_fn!(consumer_test2);
consumer_test_fn!(consumer_test3);
consumer_test_fn!(consumer_test4);
consumer_test_fn!(consumer_test5);
consumer_test_fn!(consumer_test6);
consumer_test_fn!(consumer_test7);
consumer_test_fn!(consumer_test8);
consumer_test_fn!(consumer_test9);
consumer_test_fn!(consumer_test10);
consumer_test_fn!(consumer_test11);
consumer_test_fn!(consumer_test12);
consumer_test_fn!(consumer_test13);
consumer_test_fn!(consumer_test14);
consumer_test_fn!(consumer_test15);

#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test0: *mut c_void = consumer_test0 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test1: *mut c_void = consumer_test1 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test2: *mut c_void = consumer_test2 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test3: *mut c_void = consumer_test3 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test4: *mut c_void = consumer_test4 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test5: *mut c_void = consumer_test5 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test6: *mut c_void = consumer_test6 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test7: *mut c_void = consumer_test7 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test8: *mut c_void = consumer_test8 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test9: *mut c_void = consumer_test9 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test10: *mut c_void = consumer_test10 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test11: *mut c_void = consumer_test11 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test12: *mut c_void = consumer_test12 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test13: *mut c_void = consumer_test13 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test14: *mut c_void = consumer_test14 as *mut c_void;
#[used]
#[unsafe(link_section = "consumers")]
static mut __consumer_test15: *mut c_void = consumer_test15 as *mut c_void;

unsafe fn consumer_test(skel: *mut uprobe_multi_consumers, before: c_ulong, after: c_ulong, test: test_t, offset: c_ulong) -> c_int {
    let mut ret = -1;
    printf(c"consumer_test before %lu after %lu\n".as_ptr(), before, after);
    for idx in 0..4 {
        if test_bit(idx, before) && !ASSERT_OK(uprobe_attach(skel, idx, offset), c"uprobe_attach_before".as_ptr()) {
            return ret;
        }
    }
    let err = test(skel, before, after, offset);
    if !ASSERT_EQ(err, 0, c"uprobe_consumer_test".as_ptr()) {
        return ret;
    }
    for idx in 0..4 {
        let mut val: __u64 = 0;
        let fmt: *const c_char;
        match idx {
            0 => {
                if test_bit(idx, before) { val += 1; }
                fmt = c"prog 0: uprobe".as_ptr();
            }
            1 => {
                let uret_stays = (before & after & 0b0110) != 0;
                let uret_survives = (before & 0b0110) != 0 && (after & 0b0110) != 0 && (before & 0b1001) != 0;
                if (uret_stays || uret_survives) && test_bit(idx, after) { val += 1; }
                fmt = c"prog 1: uretprobe".as_ptr();
            }
            2 => {
                if test_bit(idx, before) {
                    val += 1;
                    if test_bit(idx, after) { val += 1; }
                }
                fmt = c"prog 2: session with return".as_ptr();
            }
            3 => {
                if test_bit(idx, before) { val += 1; }
                fmt = c"prog 3: session with NO return".as_ptr();
            }
            _ => fmt = c"BUG".as_ptr(),
        }
        if !ASSERT_EQ((*(*skel).bss).uprobe_result[idx as usize], val, fmt) {
            for i in 0..4 { uprobe_detach(skel, i); }
            return ret;
        }
        (*(*skel).bss).uprobe_result[idx as usize] = 0;
    }
    ret = 0;
    for idx in 0..4 { uprobe_detach(skel, idx); }
    ret
}

extern "C" fn consumer_thread(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let idx = arg as c_ulong;
        let skel = uprobe_multi_consumers__open_and_load();
        if !ASSERT_OK_PTR(skel, c"uprobe_multi_consumers__open_and_load".as_ptr()) { return ptr::null_mut(); }
        let func = *((&raw const __start_consumers as *const *const c_void).add(idx as usize));
        let offset = get_uprobe_offset(func);
        if ASSERT_GE(offset, 0, c"uprobe_offset".as_ptr()) {
            let test: test_t = core::mem::transmute(func);
            for after in 0..CONSUMER_MAX {
                if consumer_test(skel, idx, after as c_ulong, test, offset as c_ulong) != 0 { break; }
            }
        }
        uprobe_multi_consumers__destroy(skel);
        ptr::null_mut()
    }
}

unsafe fn test_consumers() {
    let mut pt = [0 as pthread_t; CONSUMER_MAX];
    let mut idx: usize;
    /*
     * The idea of this test is to try all possible combinations of
     * uprobes consumers attached on single function.
     */
    idx = 0;
    while idx < CONSUMER_MAX {
        let err = pthread_create(&mut pt[idx], ptr::null(), consumer_thread, idx as *mut c_void);
        if !ASSERT_OK(err, c"pthread_create".as_ptr()) { break; }
        idx += 1;
    }
    while idx != 0 {
        idx -= 1;
        pthread_join(pt[idx], ptr::null_mut());
    }
}

unsafe fn uprobe_multi_program(skel: *mut uprobe_multi_pid_filter, idx: c_int) -> *mut bpf_program {
    match idx {
        0 => (*skel).progs.uprobe_multi_0,
        1 => (*skel).progs.uprobe_multi_1,
        2 => (*skel).progs.uprobe_multi_2,
        _ => ptr::null_mut(),
    }
}

unsafe fn run_pid_filter(skel: *mut uprobe_multi_pid_filter, clone_vm: bool, retprobe: bool) {
    let mut opts = bpf_uprobe_multi_opts::default();
    opts.retprobe = retprobe;
    let mut link = [ptr::null_mut::<bpf_link>(); TASKS];
    let mut child: [child; TASKS] = core::mem::zeroed();
    memset((*skel).bss as *mut c_void, 0, core::mem::size_of_val(&(*(*skel).bss).test));

    for i in 0..TASKS {
        if !ASSERT_OK(spawn_child_flag(&mut child[i], clone_vm), c"spawn_child".as_ptr()) { break; }
        (*(*skel).bss).pids[i] = child[i].pid;
    }
    for i in 0..TASKS {
        link[i] = bpf_program__attach_uprobe_multi(uprobe_multi_program(skel, i as c_int), child[i].pid, c"/proc/self/exe".as_ptr(), c"uprobe_multi_func_1".as_ptr(), &mut opts);
        if !ASSERT_OK_PTR(link[i], c"bpf_program__attach_uprobe_multi".as_ptr()) { break; }
    }
    for i in 0..TASKS { kick_child(&mut child[i]); }
    for i in 0..TASKS {
        ASSERT_EQ((*(*skel).bss).test[i][0], 1, c"pid".as_ptr());
        ASSERT_EQ((*(*skel).bss).test[i][1], 0, c"unknown".as_ptr());
    }
    for i in 0..TASKS { bpf_link__destroy(link[i]); }
    for i in 0..TASKS { release_child(&mut child[i]); }
}

unsafe fn test_pid_filter_process(clone_vm: bool) {
    let skel = uprobe_multi_pid_filter__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi_pid_filter__open_and_load".as_ptr()) { return; }
    run_pid_filter(skel, clone_vm, false);
    run_pid_filter(skel, clone_vm, true);
    uprobe_multi_pid_filter__destroy(skel);
}

unsafe fn test_session_skel_api() {
    let link: *mut bpf_link = ptr::null_mut();
    let skel = uprobe_multi_session__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi_session__open_and_load".as_ptr()) { uprobe_multi_session__destroy(skel); return; }
    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).user_ptr = test_data.as_mut_ptr();
    let err = uprobe_multi_session__attach(skel);
    if ASSERT_OK(err, c"uprobe_multi_session__attach".as_ptr()) {
        /* trigger all probes */
        (*(*skel).bss).uprobe_multi_func_1_addr = uprobe_multi_func_1 as usize as __u64;
        (*(*skel).bss).uprobe_multi_func_2_addr = uprobe_multi_func_2 as usize as __u64;
        (*(*skel).bss).uprobe_multi_func_3_addr = uprobe_multi_func_3 as usize as __u64;
        uprobe_multi_func_1();
        uprobe_multi_func_2();
        uprobe_multi_func_3();
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[0], 2, c"uprobe_multi_func_1_result".as_ptr());
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[1], 4, c"uprobe_multi_func_2_result".as_ptr());
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[2], 2, c"uprobe_multi_func_3_result".as_ptr());
        ASSERT_EQ((*(*skel).bss).uprobe_multi_sleep_result, 4, c"uprobe_multi_sleep_result".as_ptr());
    }
    bpf_link__destroy(link);
    uprobe_multi_session__destroy(skel);
}

unsafe fn test_session_single_skel_api() {
    let skel = uprobe_multi_session_single__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi_session_single__open_and_load".as_ptr()) { uprobe_multi_session_single__destroy(skel); return; }
    (*(*skel).bss).pid = getpid();
    let err = uprobe_multi_session_single__attach(skel);
    if ASSERT_OK(err, c"uprobe_multi_session_single__attach".as_ptr()) {
        uprobe_multi_func_1();
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[0], 1, c"uprobe_session_result_0".as_ptr());
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[1], 2, c"uprobe_session_result_1".as_ptr());
        ASSERT_EQ((*(*skel).bss).uprobe_session_result[2], 1, c"uprobe_session_result_2".as_ptr());
    }
    uprobe_multi_session_single__destroy(skel);
}

unsafe fn test_session_cookie_skel_api() {
    let skel = uprobe_multi_session_cookie__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi_session_cookie__open_and_load".as_ptr()) { uprobe_multi_session_cookie__destroy(skel); return; }
    (*(*skel).bss).pid = getpid();
    let err = uprobe_multi_session_cookie__attach(skel);
    if ASSERT_OK(err, c"uprobe_multi_session_cookie__attach".as_ptr()) {
        /* trigger all probes */
        uprobe_multi_func_1();
        uprobe_multi_func_2();
        uprobe_multi_func_3();
        ASSERT_EQ((*(*skel).bss).test_uprobe_1_result, 1, c"test_uprobe_1_result".as_ptr());
        ASSERT_EQ((*(*skel).bss).test_uprobe_2_result, 2, c"test_uprobe_2_result".as_ptr());
        ASSERT_EQ((*(*skel).bss).test_uprobe_3_result, 3, c"test_uprobe_3_result".as_ptr());
    }
    uprobe_multi_session_cookie__destroy(skel);
}

unsafe fn test_session_recursive_skel_api() {
    let skel = uprobe_multi_session_recursive__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_multi_session_recursive__open_and_load".as_ptr()) { uprobe_multi_session_recursive__destroy(skel); return; }
    (*(*skel).bss).pid = getpid();
    let err = uprobe_multi_session_recursive__attach(skel);
    if ASSERT_OK(err, c"uprobe_multi_session_recursive__attach".as_ptr()) {
        for i in 0..(*(*skel).bss).test_uprobe_cookie_entry.len() {
            (*(*skel).bss).test_uprobe_cookie_entry[i] = (i + 1) as __u64;
        }
        uprobe_session_recursive(5);
        /*
         *                                         entry uprobe:
         * uprobe_session_recursive(5) {             *cookie = 1, return 0
         *   uprobe_session_recursive(4) {           *cookie = 2, return 1
         *     uprobe_session_recursive(3) {         *cookie = 3, return 0
         *       uprobe_session_recursive(2) {       *cookie = 4, return 1
         *         uprobe_session_recursive(1) {     *cookie = 5, return 0
         *           uprobe_session_recursive(0) {   *cookie = 6, return 1
         *                                          return uprobe:
         *           } i = 0                          not executed
         *         } i = 1                            test_uprobe_cookie_return[0] = 5
         *       } i = 2                              not executed
         *     } i = 3                                test_uprobe_cookie_return[1] = 3
         *   } i = 4                                  not executed
         * } i = 5                                    test_uprobe_cookie_return[2] = 1
         */
        ASSERT_EQ((*(*skel).bss).idx_entry, 6, c"idx_entry".as_ptr());
        ASSERT_EQ((*(*skel).bss).idx_return, 3, c"idx_return".as_ptr());
        ASSERT_EQ((*(*skel).bss).test_uprobe_cookie_return[0], 5, c"test_uprobe_cookie_return[0]".as_ptr());
        ASSERT_EQ((*(*skel).bss).test_uprobe_cookie_return[1], 3, c"test_uprobe_cookie_return[1]".as_ptr());
        ASSERT_EQ((*(*skel).bss).test_uprobe_cookie_return[2], 1, c"test_uprobe_cookie_return[2]".as_ptr());
    }
    uprobe_multi_session_recursive__destroy(skel);
}

unsafe fn test_bench_attach_uprobe() {
    let mut attach_start_ns: c_long = 0;
    let mut attach_end_ns: c_long = 0;
    let detach_start_ns: c_long;
    let detach_end_ns: c_long;
    let skel = uprobe_multi_bench__open_and_load();
    if ASSERT_OK_PTR(skel, c"uprobe_multi_bench__open_and_load".as_ptr()) {
        attach_start_ns = get_time_ns();
        let err = uprobe_multi_bench__attach(skel);
        if ASSERT_OK(err, c"uprobe_multi_bench__attach".as_ptr()) {
            attach_end_ns = get_time_ns();
            system(c"./uprobe_multi bench".as_ptr());
            ASSERT_EQ((*(*skel).bss).count, 50000, c"uprobes_count".as_ptr());
        }
    }
    detach_start_ns = get_time_ns();
    uprobe_multi_bench__destroy(skel);
    detach_end_ns = get_time_ns();
    let attach_delta = (attach_end_ns - attach_start_ns) as f64 / 1000000000.0;
    let detach_delta = (detach_end_ns - detach_start_ns) as f64 / 1000000000.0;
    printf(c"%s: attached in %7.3lfs\n".as_ptr(), c"test_bench_attach_uprobe".as_ptr(), attach_delta);
    printf(c"%s: detached in %7.3lfs\n".as_ptr(), c"test_bench_attach_uprobe".as_ptr(), detach_delta);
}

unsafe fn test_bench_attach_usdt() {
    let mut attach_start_ns: c_long = 0;
    let mut attach_end_ns: c_long = 0;
    let detach_start_ns: c_long;
    let detach_end_ns: c_long;
    let skel = uprobe_multi_usdt__open_and_load();
    if ASSERT_OK_PTR(skel, c"uprobe_multi__open".as_ptr()) {
        attach_start_ns = get_time_ns();
        (*skel).links.usdt0 = bpf_program__attach_usdt((*skel).progs.usdt0, -1, c"./uprobe_multi".as_ptr(), c"test".as_ptr(), c"usdt".as_ptr(), ptr::null());
        if ASSERT_OK_PTR((*skel).links.usdt0, c"bpf_program__attach_usdt".as_ptr()) {
            attach_end_ns = get_time_ns();
            system(c"./uprobe_multi usdt".as_ptr());
            ASSERT_EQ((*(*skel).bss).count, 50000, c"usdt_count".as_ptr());
        }
    }
    detach_start_ns = get_time_ns();
    uprobe_multi_usdt__destroy(skel);
    detach_end_ns = get_time_ns();
    let attach_delta = (attach_end_ns - attach_start_ns) as f64 / 1000000000.0;
    let detach_delta = (detach_end_ns - detach_start_ns) as f64 / 1000000000.0;
    printf(c"%s: attached in %7.3lfs\n".as_ptr(), c"test_bench_attach_usdt".as_ptr(), attach_delta);
    printf(c"%s: detached in %7.3lfs\n".as_ptr(), c"test_bench_attach_usdt".as_ptr(), detach_delta);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_uprobe_multi_test() {
    if test__start_subtest(c"skel_api".as_ptr()) { test_skel_api(); }
    if test__start_subtest(c"attach_api_pattern".as_ptr()) { test_attach_api_pattern(); }
    if test__start_subtest(c"attach_api_syms".as_ptr()) { test_attach_api_syms(); }
    if test__start_subtest(c"link_api".as_ptr()) { test_link_api(); }
    if test__start_subtest(c"link_api_path_fd".as_ptr()) { test_link_api_path_fd(); }
    if test__start_subtest(c"bench_uprobe".as_ptr()) { test_bench_attach_uprobe(); }
    if test__start_subtest(c"bench_usdt".as_ptr()) { test_bench_attach_usdt(); }
    if test__start_subtest(c"attach_api_fails".as_ptr()) { test_attach_api_fails(); }
    if test__start_subtest(c"attach_uprobe_fails".as_ptr()) { test_attach_uprobe_fails(); }
    if test__start_subtest(c"consumers".as_ptr()) { test_consumers(); }
    if test__start_subtest(c"filter_fork".as_ptr()) { test_pid_filter_process(false); }
    if test__start_subtest(c"filter_clone_vm".as_ptr()) { test_pid_filter_process(true); }
    if test__start_subtest(c"session".as_ptr()) { test_session_skel_api(); }
    if test__start_subtest(c"session_single".as_ptr()) { test_session_single_skel_api(); }
    if test__start_subtest(c"session_cookie".as_ptr()) { test_session_cookie_skel_api(); }
    if test__start_subtest(c"session_cookie_recursive".as_ptr()) { test_session_recursive_skel_api(); }
    RUN_TESTS_uprobe_multi_verifier();
}
