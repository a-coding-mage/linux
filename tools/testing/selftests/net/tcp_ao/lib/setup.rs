// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/net/tcp_ao/lib/setup.c.
// C include dependencies preserved as external items/macros from the surrounding test library.

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type time_t = libc::time_t;
type FILE = libc::FILE;
type pthread_t = libc::pthread_t;
type pthread_mutex_t = libc::pthread_mutex_t;
type pthread_cond_t = libc::pthread_cond_t;
type thread_fn = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub bytes: [u8; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_flags: c_int,
    sa_mask: libc::sigset_t,
}

const O_RDONLY: c_int = libc::O_RDONLY;
const F_OK: c_int = libc::F_OK;
const CLONE_NEWNET: c_int = libc::CLONE_NEWNET;
const SA_RESTART: c_int = libc::SA_RESTART;
const SIGINT: c_int = libc::SIGINT;
const KSFT_SKIP: c_int = 4;
const KSFT_FAIL: c_int = 1;
const KSFT_PASS: c_int = 0;

const KCONFIG_NET_NS: usize = 0;
const KCONFIG_VETH: usize = 1;
const KCONFIG_TCP_AO: usize = 2;

unsafe extern "C" {
    static tests_skip_reason: [*const c_char; 0];

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_xfail(fmt: *const c_char, ...);
    fn ksft_test_result_error(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
    fn ksft_print_cnts();
    fn ksft_set_plan(ntests: c_uint);
    fn ksft_print_header();

    fn exit(status: c_int) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn srand(seed: c_uint);
    fn sigemptyset(set: *mut libc::sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn link_set_up(veth: *const c_char) -> c_int;
    fn ip_addr_add(veth: *const c_char, family: c_int, addr: tcp_addr, prefix: u8) -> c_int;
    fn ip_route_add(veth: *const c_char, family: c_int, addr: tcp_addr, dest: tcp_addr) -> c_int;
    fn kernel_config_has(config: usize) -> bool;
    fn test_init_ftrace(nsfd_parent: c_int, nsfd_child: c_int);
    fn add_veth(veth: *const c_char, nsfd_parent: c_int, nsfd_child: c_int) -> c_int;
}

// Prevent overriding of one thread's output by another
static mut ksft_print_lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;

#[no_mangle]
pub unsafe extern "C" fn __test_msg(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_print_msg(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

#[no_mangle]
pub unsafe extern "C" fn __test_ok(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_test_result_pass(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

#[no_mangle]
pub unsafe extern "C" fn __test_fail(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_test_result_fail(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

#[no_mangle]
pub unsafe extern "C" fn __test_xfail(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_test_result_xfail(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

#[no_mangle]
pub unsafe extern "C" fn __test_error(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_test_result_error(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

#[no_mangle]
pub unsafe extern "C" fn __test_skip(buf: *const c_char) {
    pthread_mutex_lock(&raw mut ksft_print_lock);
    ksft_test_result_skip(c"%s".as_ptr(), buf);
    pthread_mutex_unlock(&raw mut ksft_print_lock);
}

static mut failed: c_int = 0;
static mut skipped: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn test_failed() {
    failed = 1;
}

unsafe extern "C" fn test_exit() {
    if failed != 0 {
        ksft_exit_fail();
    } else if skipped != 0 {
        // ksft_exit_skip() is different from ksft_exit_*()
        ksft_print_cnts();
        exit(KSFT_SKIP);
    } else {
        ksft_exit_pass();
    }
}

#[repr(C)]
struct dlist_t {
    destruct: Option<unsafe extern "C" fn()>,
    next: *mut dlist_t,
}

static mut destructors_list: *mut dlist_t = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn test_add_destructor(d: Option<unsafe extern "C" fn()>) {
    let p = malloc(core::mem::size_of::<dlist_t>()) as *mut dlist_t;
    if p.is_null() {
        test_error!(c"malloc() failed".as_ptr());
    }

    (*p).next = destructors_list;
    (*p).destruct = d;
    destructors_list = p;
}

// C used __attribute__((destructor)); preserve intent for the surrounding Rust build integration.
unsafe extern "C" fn test_destructor() {
    while !destructors_list.is_null() {
        let p = (*destructors_list).next;

        if let Some(destruct) = (*destructors_list).destruct {
            destruct();
        }
        free(destructors_list as *mut c_void);
        destructors_list = p;
    }
    test_exit();
}

unsafe extern "C" fn sig_int(_signo: c_int) {
    test_error!(c"Caught SIGINT - exiting".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn open_netns() -> c_int {
    let netns_path = c"/proc/thread-self/ns/net".as_ptr();
    let fd = open(netns_path, O_RDONLY);
    if fd < 0 {
        test_error!(c"open(%s)".as_ptr(), netns_path);
    }
    fd
}

#[no_mangle]
pub unsafe extern "C" fn unshare_open_netns() -> c_int {
    if unshare(CLONE_NEWNET) != 0 {
        test_error!(c"unshare()".as_ptr());
    }

    open_netns()
}

#[no_mangle]
pub unsafe extern "C" fn switch_ns(fd: c_int) {
    if setns(fd, CLONE_NEWNET) != 0 {
        test_error!(c"setns()".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn switch_save_ns(new_ns: c_int) -> c_int {
    let ret = open_netns();

    switch_ns(new_ns);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn switch_close_ns(fd: c_int) {
    if setns(fd, CLONE_NEWNET) != 0 {
        test_error!(c"setns()".as_ptr());
    }
    close(fd);
}

static mut nsfd_outside: c_int = -1;
static mut nsfd_parent: c_int = -1;
static mut nsfd_child: c_int = -1;

#[no_mangle]
pub static veth_name: [c_char; 10] = [
    b'k' as c_char,
    b't' as c_char,
    b's' as c_char,
    b't' as c_char,
    b'-' as c_char,
    b'v' as c_char,
    b'e' as c_char,
    b't' as c_char,
    b'h' as c_char,
    0,
];

unsafe extern "C" fn init_namespaces() {
    nsfd_outside = open_netns();
    nsfd_parent = unshare_open_netns();
    nsfd_child = unshare_open_netns();
}

unsafe extern "C" fn link_init(
    veth: *const c_char,
    family: c_int,
    prefix: u8,
    addr: tcp_addr,
    dest: tcp_addr,
) {
    if link_set_up(veth) != 0 {
        test_error!(c"Failed to set link up".as_ptr());
    }
    if ip_addr_add(veth, family, addr, prefix) != 0 {
        test_error!(c"Failed to add ip address".as_ptr());
    }
    if ip_route_add(veth, family, addr, dest) != 0 {
        test_error!(c"Failed to add route".as_ptr());
    }
}

static mut nr_threads: c_uint = 1;

static mut sync_lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;
static mut sync_cond: pthread_cond_t = libc::PTHREAD_COND_INITIALIZER;
static mut stage_threads: [c_uint; 2] = [0; 2];
static mut stage_nr: c_uint = 0;

// synchronize all threads in the same stage
#[no_mangle]
pub unsafe extern "C" fn synchronize_threads() {
    let q = stage_nr;

    pthread_mutex_lock(&raw mut sync_lock);
    stage_threads[q as usize] = stage_threads[q as usize].wrapping_add(1);
    if stage_threads[q as usize] == nr_threads {
        stage_nr ^= 1;
        stage_threads[stage_nr as usize] = 0;
        pthread_cond_signal(&raw mut sync_cond);
    }
    while stage_threads[q as usize] < nr_threads {
        pthread_cond_wait(&raw mut sync_cond, &raw mut sync_lock);
    }
    pthread_mutex_unlock(&raw mut sync_lock);
}

#[thread_local]
#[no_mangle]
pub static mut this_ip_addr: tcp_addr = tcp_addr { bytes: [0; 16] };

#[thread_local]
#[no_mangle]
pub static mut this_ip_dest: tcp_addr = tcp_addr { bytes: [0; 16] };

#[no_mangle]
pub static mut test_family: c_int = 0;

#[repr(C)]
struct new_pthread_arg {
    func: thread_fn,
    my_ip: tcp_addr,
    dest_ip: tcp_addr,
}

unsafe extern "C" fn new_pthread_entry(arg: *mut c_void) -> *mut c_void {
    let p = arg as *mut new_pthread_arg;

    this_ip_addr = (*p).my_ip;
    this_ip_dest = (*p).dest_ip;
    ((*p).func)(core::ptr::null_mut()); // shouldn't return
    exit(KSFT_FAIL);
}

unsafe extern "C" fn __test_skip_all(msg: *const c_char) {
    ksft_set_plan(1);
    ksft_print_header();
    skipped = 1;
    test_skip!(c"%s".as_ptr(), msg);
    exit(KSFT_SKIP);
}

#[no_mangle]
pub unsafe extern "C" fn __test_init(
    ntests: c_uint,
    family: c_int,
    prefix: c_uint,
    addr1: tcp_addr,
    addr2: tcp_addr,
    peer1: thread_fn,
    peer2: Option<thread_fn>,
) {
    let mut sa = sigaction {
        sa_handler: Some(sig_int),
        sa_flags: SA_RESTART,
        sa_mask: core::mem::zeroed(),
    };
    let seed = time(core::ptr::null_mut());

    sigemptyset(&mut sa.sa_mask);
    if sigaction(SIGINT, &sa, core::ptr::null_mut()) != 0 {
        test_error!(c"Can't set SIGINT handler".as_ptr());
    }

    test_family = family;
    if !kernel_config_has(KCONFIG_NET_NS) {
        __test_skip_all(tests_skip_reason[KCONFIG_NET_NS]);
    }
    if !kernel_config_has(KCONFIG_VETH) {
        __test_skip_all(tests_skip_reason[KCONFIG_VETH]);
    }
    if !kernel_config_has(KCONFIG_TCP_AO) {
        __test_skip_all(tests_skip_reason[KCONFIG_TCP_AO]);
    }

    ksft_set_plan(ntests);
    test_print!(c"rand seed %u".as_ptr(), seed as c_uint);
    srand(seed as c_uint);

    ksft_print_header();
    init_namespaces();
    test_init_ftrace(nsfd_parent, nsfd_child);

    if add_veth(veth_name.as_ptr(), nsfd_parent, nsfd_child) != 0 {
        test_error!(c"Failed to add veth".as_ptr());
    }

    switch_ns(nsfd_child);
    link_init(veth_name.as_ptr(), family, prefix as u8, addr2, addr1);
    if let Some(peer2_fn) = peer2 {
        let mut targ = new_pthread_arg {
            func: peer2_fn,
            my_ip: addr2,
            dest_ip: addr1,
        };
        let mut t: pthread_t = core::mem::zeroed();

        nr_threads = nr_threads.wrapping_add(1);
        if pthread_create(
            &mut t,
            core::ptr::null(),
            new_pthread_entry,
            &mut targ as *mut new_pthread_arg as *mut c_void,
        ) != 0
        {
            test_error!(c"Failed to create pthread".as_ptr());
        }
    }
    switch_ns(nsfd_parent);
    link_init(veth_name.as_ptr(), family, prefix as u8, addr1, addr2);

    this_ip_addr = addr1;
    this_ip_dest = addr2;
    peer1(core::ptr::null_mut());
    if failed != 0 {
        exit(KSFT_FAIL);
    } else {
        exit(KSFT_PASS);
    }
}

// /proc/sys/net/core/optmem_max artifically limits the amount of memory
// that can be allocated with sock_kmalloc() on each socket in the system.
// It is not virtualized in v6.7, so it has to written outside test
// namespaces. To be nice a test will revert optmem back to the old value.
// Keeping it simple without any file lock, which means the tests that
// need to set/increase optmem value shouldn't run in parallel.
// Also, not re-entrant.
// Since commit f5769faeec36 ("net: Namespace-ify sysctl_optmem_max")
// it is per-namespace, keeping logic for non-virtualized optmem_max
// for v6.7, which supports TCP-AO.
static optmem_file: *const c_char = c"/proc/sys/net/core/optmem_max".as_ptr();
static mut saved_optmem: size_t = 0;
static mut optmem_ns: c_int = -1;

unsafe extern "C" fn is_optmem_namespaced() -> bool {
    if optmem_ns == -1 {
        let old_ns = switch_save_ns(nsfd_child);

        optmem_ns = (access(optmem_file, F_OK) == 0) as c_int;
        switch_close_ns(old_ns);
    }
    optmem_ns != 0
}

#[no_mangle]
pub unsafe extern "C" fn test_get_optmem() -> size_t {
    let mut old_ns: c_int = 0;
    let foptmem: *mut FILE;
    let mut ret: size_t = 0;

    if !is_optmem_namespaced() {
        old_ns = switch_save_ns(nsfd_outside);
    }
    foptmem = fopen(optmem_file, c"r".as_ptr());
    if foptmem.is_null() {
        test_error!(c"failed to open %s".as_ptr(), optmem_file);
    }

    if fscanf(foptmem, c"%zu".as_ptr(), &mut ret as *mut size_t) != 1 {
        test_error!(c"can't read from %s".as_ptr(), optmem_file);
    }
    fclose(foptmem);
    if !is_optmem_namespaced() {
        switch_close_ns(old_ns);
    }
    ret
}

unsafe extern "C" fn __test_set_optmem(new: size_t, old: *mut size_t) {
    let mut old_ns: c_int = 0;
    let foptmem: *mut FILE;

    if !old.is_null() {
        *old = test_get_optmem();
    }

    if !is_optmem_namespaced() {
        old_ns = switch_save_ns(nsfd_outside);
    }
    foptmem = fopen(optmem_file, c"w".as_ptr());
    if foptmem.is_null() {
        test_error!(c"failed to open %s".as_ptr(), optmem_file);
    }

    if fprintf(foptmem, c"%zu".as_ptr(), new) <= 0 {
        test_error!(c"can't write %zu to %s".as_ptr(), new, optmem_file);
    }
    fclose(foptmem);
    if !is_optmem_namespaced() {
        switch_close_ns(old_ns);
    }
}

unsafe extern "C" fn test_revert_optmem() {
    if saved_optmem == 0 {
        return;
    }

    __test_set_optmem(saved_optmem, core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn test_set_optmem(value: size_t) {
    if saved_optmem == 0 {
        __test_set_optmem(value, &raw mut saved_optmem);
        test_add_destructor(Some(test_revert_optmem));
    } else {
        __test_set_optmem(value, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
