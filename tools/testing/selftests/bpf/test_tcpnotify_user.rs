// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external declarations:
// pthread.h, inttypes.h, stdio.h, stdlib.h, unistd.h, asm/types.h,
// sys/syscall.h, errno.h, string.h, linux/bpf.h, sys/socket.h,
// bpf/bpf.h, bpf/libbpf.h, sys/ioctl.h, linux/rtnetlink.h,
// linux/perf_event.h, cgroup_helpers.h, test_tcpnotify.h,
// testing_helpers.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type __u32 = u32;
type pthread_t = usize;

const EXIT_FAILURE: c_int = 1;
const EINTR: c_int = 4;
const LIBBPF_STRICT_ALL: c_int = 0xffffffffu32 as c_int;
const BPF_PROG_TYPE_SOCK_OPS: c_int = 13;
const BPF_CGROUP_SOCK_OPS: c_int = 15;

// From test_tcpnotify.h.
extern "C" {
    static TESTPORT: c_int;
}

#[repr(C)]
pub struct tcp_notifier {
    pub type_: __u32,
    pub subtype: __u32,
    pub source: __u32,
    pub hash: __u32,
}

#[repr(C)]
pub struct tcpnotify_globals {
    pub ncalls: c_int,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_buffer {
    _private: [u8; 0],
}

extern "C" {
    static mut errno: c_int;

    fn getpagesize() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, type_: c_int, flags: c_uint) -> c_int;
    fn bpf_prog_detach(target_fd: c_int, type_: c_int) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
        lost_cb: *mut c_void,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__free(pb: *mut perf_buffer);

    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
}

fn SOCKET_BUFFER_SIZE() -> c_int {
    unsafe {
        if getpagesize() < 8192 {
            getpagesize()
        } else {
            8192
        }
    }
}

static mut tid: pthread_t = 0;
static mut exit_thread: bool = false;

static mut rx_callbacks: c_int = 0;

unsafe extern "C" fn dummyfn(_ctx: *mut c_void, _cpu: c_int, data: *mut c_void, _size: __u32) {
    let t: *mut tcp_notifier = data as *mut tcp_notifier;

    if (*t).type_ != 0xde || (*t).subtype != 0xad || (*t).source != 0xbe || (*t).hash != 0xef {
        return;
    }
    rx_callbacks += 1;
}

unsafe fn tcp_notifier_poller(pb: *mut perf_buffer) {
    let mut err: c_int;

    while !exit_thread {
        err = perf_buffer__poll(pb, 100);
        if err < 0 && err != -EINTR {
            printf(b"failed perf_buffer__poll: %d\n\0".as_ptr() as *const c_char, err);
            return;
        }
    }
}

unsafe extern "C" fn poller_thread(arg: *mut c_void) -> *mut c_void {
    let pb: *mut perf_buffer = arg as *mut perf_buffer;

    tcp_notifier_poller(pb);
    arg
}

unsafe fn verify_result(result: *const tcpnotify_globals) -> c_int {
    if (*result).ncalls > 0 && (*result).ncalls == rx_callbacks {
        0
    } else {
        1
    }
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let file: *const c_char = b"test_tcpnotify_kern.bpf.o\0".as_ptr() as *const c_char;
    let mut perf_map: *mut bpf_map;
    let mut global_map: *mut bpf_map;
    let mut g: tcpnotify_globals = core::mem::zeroed();
    let mut pb: *mut perf_buffer = ptr::null_mut();
    let cg_path: *const c_char = b"/foo\0".as_ptr() as *const c_char;
    let mut prog_fd: c_int = 0;
    let mut rv: c_int;
    let mut cg_fd: c_int = -1;
    let mut error: c_int = EXIT_FAILURE;
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut test_script: [c_char; 80] = [0; 80];
    let key: __u32 = 0;

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    cg_fd = cgroup_setup_and_join(cg_path);
    if cg_fd < 0 {
        goto_err(cg_fd, pb);
        return error;
    }

    if bpf_prog_test_load(file, BPF_PROG_TYPE_SOCK_OPS, &mut obj, &mut prog_fd) != 0 {
        printf(
            b"FAILED: load_bpf_file failed for: %s\n\0".as_ptr() as *const c_char,
            file,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    rv = bpf_prog_attach(prog_fd, cg_fd, BPF_CGROUP_SOCK_OPS, 0);
    if rv != 0 {
        printf(
            b"FAILED: bpf_prog_attach: %d (%s)\n\0".as_ptr() as *const c_char,
            error,
            strerror(errno),
        );
        goto_err(cg_fd, pb);
        return error;
    }

    perf_map = bpf_object__find_map_by_name(obj, b"perf_event_map\0".as_ptr() as *const c_char);
    if perf_map.is_null() {
        printf(
            b"FAIL:map '%s' not found\n\0".as_ptr() as *const c_char,
            b"perf_event_map\0".as_ptr() as *const c_char,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    global_map = bpf_object__find_map_by_name(obj, b"global_map\0".as_ptr() as *const c_char);
    if global_map.is_null() {
        printf(
            b"FAIL:map '%s' not found\n\0".as_ptr() as *const c_char,
            b"global_map\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    pb = perf_buffer__new(
        bpf_map__fd(perf_map),
        8,
        Some(dummyfn),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null(),
    );
    if pb.is_null() {
        goto_err(cg_fd, pb);
        return error;
    }

    pthread_create(
        &mut tid,
        ptr::null(),
        poller_thread,
        pb as *mut c_void,
    );

    sprintf(
        test_script.as_mut_ptr(),
        b"iptables -A INPUT -p tcp --dport %d -j DROP\0".as_ptr() as *const c_char,
        TESTPORT,
    );
    if system(test_script.as_ptr()) != 0 {
        printf(
            b"FAILED: execute command: %s, err %d\n\0".as_ptr() as *const c_char,
            test_script.as_ptr(),
            -errno,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    sprintf(
        test_script.as_mut_ptr(),
        b"nc 127.0.0.1 %d < /etc/passwd > /dev/null 2>&1 \0".as_ptr() as *const c_char,
        TESTPORT,
    );
    if system(test_script.as_ptr()) != 0 {
        printf(
            b"execute command: %s, err %d\n\0".as_ptr() as *const c_char,
            test_script.as_ptr(),
            -errno,
        );
    }

    sprintf(
        test_script.as_mut_ptr(),
        b"iptables -D INPUT -p tcp --dport %d -j DROP\0".as_ptr() as *const c_char,
        TESTPORT,
    );
    if system(test_script.as_ptr()) != 0 {
        printf(
            b"FAILED: execute command: %s, err %d\n\0".as_ptr() as *const c_char,
            test_script.as_ptr(),
            -errno,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    rv = bpf_map_lookup_elem(
        bpf_map__fd(global_map),
        &key as *const __u32 as *const c_void,
        &mut g as *mut tcpnotify_globals as *mut c_void,
    );
    if rv != 0 {
        printf(
            b"FAILED: bpf_map_lookup_elem returns %d\n\0".as_ptr() as *const c_char,
            rv,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    sleep(10);

    exit_thread = true;
    let ret: c_int = pthread_join(tid, ptr::null_mut());
    if ret != 0 {
        printf(b"FAILED: pthread_join\n\0".as_ptr() as *const c_char);
        goto_err(cg_fd, pb);
        return error;
    }

    if verify_result(&g) != 0 {
        printf(
            b"FAILED: Wrong stats Expected %d calls, got %d\n\0".as_ptr() as *const c_char,
            g.ncalls,
            rx_callbacks,
        );
        goto_err(cg_fd, pb);
        return error;
    }

    printf(b"PASSED!\n\0".as_ptr() as *const c_char);
    error = 0;
    goto_err(cg_fd, pb);
    error
}

unsafe fn goto_err(cg_fd: c_int, pb: *mut perf_buffer) {
    bpf_prog_detach(cg_fd, BPF_CGROUP_SOCK_OPS);
    close(cg_fd);
    cleanup_cgroup_environment();
    perf_buffer__free(pb);
}

fn main() {
    unsafe {
        let args: *mut *mut c_char = ptr::null_mut();
        std::process::exit(c_main(0, args));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
