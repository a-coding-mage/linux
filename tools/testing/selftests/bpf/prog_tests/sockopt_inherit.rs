// SPDX-License-Identifier: GPL-2.0
// Translated from C. External test, cgroup, network helper, libbpf, pthread,
// libc, and sockopt_inherit skeleton symbols are provided by the surrounding
// selftest build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SOL_CUSTOM: c_int = 0xdeadbeefu32 as c_int;
const CUSTOM_INHERIT1: c_int = 0;
const CUSTOM_INHERIT2: c_int = 1;
const CUSTOM_LISTENER: c_int = 2;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const _SC_PAGESIZE: c_int = 30;

type socklen_t = c_uint;
type pthread_t = usize;

#[repr(C)]
pub struct pthread_mutex_t {
    __private: [u8; 40],
}

#[repr(C)]
pub struct pthread_cond_t {
    __private: [u8; 48],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [u8; 118],
    pub __ss_align: u64,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockopt_inherit_bss {
    pub page_size: c_long,
}

#[repr(C)]
pub struct sockopt_inherit_progs {
    pub _getsockopt: *mut bpf_program,
    pub _setsockopt: *mut bpf_program,
}

#[repr(C)]
pub struct sockopt_inherit {
    pub bss: *mut sockopt_inherit_bss,
    pub progs: sockopt_inherit_progs,
}

#[repr(C)]
pub struct network_helper_opts {
    pub post_socket_cb: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
}

unsafe extern "C" {
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn sysconf(name: c_int) -> c_long;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn log_err(format: *const c_char, ...);

    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;

    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn start_server_addr(
        sock_type: c_int,
        addr: *mut sockaddr_storage,
        addrlen: socklen_t,
        opts: *mut network_helper_opts,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;

    fn sockopt_inherit__open_and_load() -> *mut sockopt_inherit;
    fn sockopt_inherit__destroy(obj: *mut sockopt_inherit);
}

// C initializers are static opaque pthread objects. The surrounding C build
// supplies exact layout requirements; these zeroed values preserve file-local
// initialization intent.
static mut server_started_mtx: pthread_mutex_t = pthread_mutex_t { __private: [0; 40] };
static mut server_started: pthread_cond_t = pthread_cond_t { __private: [0; 48] };

unsafe fn verify_sockopt(fd: c_int, optname: c_int, msg: *const c_char, expected: c_char) -> c_int {
    let mut optlen: socklen_t = 1;
    let mut buf: c_char = 0;
    let mut err: c_int;

    err = getsockopt(
        fd,
        SOL_CUSTOM,
        optname,
        &mut buf as *mut c_char as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"%s: failed to call getsockopt".as_ptr(), msg);
        return 1;
    }

    printf(
        c"%s %d: got=0x%x ? expected=0x%x\n".as_ptr(),
        msg,
        optname,
        buf as c_int,
        expected as c_int,
    );

    if buf != expected {
        log_err(
            c"%s: unexpected getsockopt value %d != %d".as_ptr(),
            msg,
            buf as c_int,
            expected as c_int,
        );
        return 1;
    }

    0
}

unsafe extern "C" fn server_thread(arg: *mut c_void) -> *mut c_void {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let fd: c_int = *(arg as *mut c_int);
    let mut client_fd: c_int;
    let mut err: c_int = 0;

    err = listen(fd, 1);

    pthread_mutex_lock(&raw mut server_started_mtx);
    pthread_cond_signal(&raw mut server_started);
    pthread_mutex_unlock(&raw mut server_started_mtx);

    if !ASSERT_GE(err as c_long, 0, c"listed on socket".as_ptr()) {
        return ptr::null_mut();
    }

    err += verify_sockopt(fd, CUSTOM_INHERIT1, c"listen".as_ptr(), 1);
    err += verify_sockopt(fd, CUSTOM_INHERIT2, c"listen".as_ptr(), 1);
    err += verify_sockopt(fd, CUSTOM_LISTENER, c"listen".as_ptr(), 1);

    client_fd = accept(fd, &mut addr as *mut sockaddr_storage as *mut sockaddr, &mut len);
    if !ASSERT_GE(client_fd as c_long, 0, c"accept client".as_ptr()) {
        return ptr::null_mut();
    }

    err += verify_sockopt(client_fd, CUSTOM_INHERIT1, c"accept".as_ptr(), 1);
    err += verify_sockopt(client_fd, CUSTOM_INHERIT2, c"accept".as_ptr(), 1);
    err += verify_sockopt(client_fd, CUSTOM_LISTENER, c"accept".as_ptr(), 0);

    close(client_fd);

    err as c_long as *mut c_void
}

unsafe extern "C" fn custom_cb(fd: c_int, _opts: *mut c_void) -> c_int {
    let mut buf: c_char;
    let mut err: c_int;
    let mut i: c_int;

    i = CUSTOM_INHERIT1;
    while i <= CUSTOM_LISTENER {
        buf = 0x01;
        err = setsockopt(
            fd,
            SOL_CUSTOM,
            i,
            &buf as *const c_char as *const c_void,
            1,
        );
        if err != 0 {
            log_err(c"Failed to call setsockopt(%d)".as_ptr(), i);
            return -1;
        }
        i += 1;
    }

    0
}

unsafe fn run_test(cgroup_fd: c_int) {
    let mut link_getsockopt: *mut bpf_link = ptr::null_mut();
    let mut link_setsockopt: *mut bpf_link = ptr::null_mut();
    let mut opts = network_helper_opts {
        post_socket_cb: Some(custom_cb),
    };
    let mut server_fd: c_int = -1;
    let mut client_fd: c_int;
    let mut addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr {
            s_addr: htonl(INADDR_LOOPBACK),
        },
        sin_zero: [0; 8],
    };
    let obj: *mut sockopt_inherit;
    let mut server_err: *mut c_void = ptr::null_mut();
    let mut tid: pthread_t = 0;
    let mut err: c_int;

    obj = sockopt_inherit__open_and_load();
    if !ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) {
        return;
    }

    (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE);

    link_getsockopt = bpf_program__attach_cgroup((*obj).progs._getsockopt, cgroup_fd);
    if !ASSERT_OK_PTR(link_getsockopt as *const c_void, c"cg-attach-getsockopt".as_ptr()) {
        goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
        return;
    }

    link_setsockopt = bpf_program__attach_cgroup((*obj).progs._setsockopt, cgroup_fd);
    if !ASSERT_OK_PTR(link_setsockopt as *const c_void, c"cg-attach-setsockopt".as_ptr()) {
        goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
        return;
    }

    server_fd = start_server_addr(
        SOCK_STREAM,
        &mut addr as *mut sockaddr_in as *mut sockaddr_storage,
        size_of::<sockaddr_in>() as socklen_t,
        &mut opts,
    );
    if !ASSERT_GE(server_fd as c_long, 0, c"start_server".as_ptr()) {
        goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
        return;
    }

    pthread_mutex_lock(&raw mut server_started_mtx);
    if !ASSERT_OK(
        pthread_create(
            &mut tid,
            ptr::null(),
            server_thread,
            &mut server_fd as *mut c_int as *mut c_void,
        ),
        c"pthread_create".as_ptr(),
    ) {
        pthread_mutex_unlock(&raw mut server_started_mtx);
        close(server_fd);
        goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
        return;
    }
    pthread_cond_wait(&raw mut server_started, &raw mut server_started_mtx);
    pthread_mutex_unlock(&raw mut server_started_mtx);

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_GE(client_fd as c_long, 0, c"connect_to_server".as_ptr()) {
        close(server_fd);
        goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
        return;
    }

    ASSERT_OK(
        verify_sockopt(client_fd, CUSTOM_INHERIT1, c"connect".as_ptr(), 0),
        c"verify_sockopt1".as_ptr(),
    );
    ASSERT_OK(
        verify_sockopt(client_fd, CUSTOM_INHERIT2, c"connect".as_ptr(), 0),
        c"verify_sockopt2".as_ptr(),
    );
    ASSERT_OK(
        verify_sockopt(client_fd, CUSTOM_LISTENER, c"connect".as_ptr(), 0),
        c"verify_sockopt ener".as_ptr(),
    );

    pthread_join(tid, &mut server_err);

    err = server_err as c_long as c_int;
    ASSERT_OK(err, c"pthread_join retval".as_ptr());

    close(client_fd);

    close(server_fd);
    goto_close_bpf_object(link_getsockopt, link_setsockopt, obj);
}

unsafe fn goto_close_bpf_object(
    link_getsockopt: *mut bpf_link,
    link_setsockopt: *mut bpf_link,
    obj: *mut sockopt_inherit,
) {
    bpf_link__destroy(link_getsockopt);
    bpf_link__destroy(link_setsockopt);

    sockopt_inherit__destroy(obj);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockopt_inherit() {
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/sockopt_inherit".as_ptr());
    if !ASSERT_GE(cgroup_fd as c_long, 0, c"join_cgroup".as_ptr()) {
        return;
    }

    run_test(cgroup_fd);
    close(cgroup_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
