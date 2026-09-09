// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct BpfLink {
    _private: [u8; 0],
}

#[repr(C)]
struct BpfProgram {
    _private: [u8; 0],
}

#[repr(C)]
struct PerfBuffer {
    _private: [u8; 0],
}

#[repr(C)]
struct BpfObject {
    _private: [u8; 0],
}

const CLOCK_MONOTONIC: c_int = 1;
const SIGINT: c_int = 2;
const MAX_CNT: i64 = 100000;

unsafe extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn snprintf(string: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut c_void;
    fn kill(pid: c_int, sig: c_int) -> c_int;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut BpfObject;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut BpfObject) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut BpfObject, name: *const c_char) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut BpfObject,
        name: *const c_char,
    ) -> *mut BpfProgram;
    fn bpf_program__attach(prog: *mut BpfProgram) -> *mut BpfLink;
    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, u32)>,
        ctx: *mut c_void,
        lost_cb: *mut c_void,
        opts: *mut c_void,
    ) -> *mut PerfBuffer;
    fn perf_buffer__poll(pb: *mut PerfBuffer, timeout_ms: c_int) -> c_int;
    fn bpf_link__destroy(link: *mut BpfLink);
    fn bpf_object__close(obj: *mut BpfObject);
}

static mut START_TIME: u64 = 0;
static mut CNT: u64 = 0;

unsafe fn time_get_ns() -> u64 {
    let mut ts = Timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as u64).wrapping_mul(1_000_000_000u64).wrapping_add(ts.tv_nsec as u64)
}

unsafe extern "C" fn print_bpf_output(_ctx: *mut c_void, _cpu: c_int, data: *mut c_void, size: u32) {
    #[repr(C)]
    struct Event {
        pid: u64,
        cookie: u64,
    }

    let e = data as *const Event;
    if (*e).cookie != 0x12345678 {
        printf(
            c"BUG pid %llx cookie %llx sized %d\n".as_ptr(),
            (*e).pid,
            (*e).cookie,
            size,
        );
        return;
    }

    CNT = CNT.wrapping_add(1);

    if CNT == MAX_CNT as u64 {
        printf(
            c"recv %lld events per sec\n".as_ptr(),
            (MAX_CNT * 1_000_000_000i64) / (time_get_ns() - START_TIME) as i64,
        );
        return;
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut link: *mut BpfLink = core::ptr::null_mut();
    let mut prog: *mut BpfProgram;
    let pb: *mut PerfBuffer;
    let obj: *mut BpfObject;
    let mut map_fd: c_int;
    let mut ret: c_int = 0;
    let mut filename = [0i8; 256];
    let f: *mut c_void;

    let _ = argc;
    snprintf(filename.as_mut_ptr(), filename.len(), c"%s.bpf.o".as_ptr(), *argv);
    obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj.cast()) != 0 {
        fprintf(core::ptr::null_mut(), c"ERROR: opening BPF object file failed\n".as_ptr());
        return 0;
    }

    /* load BPF program */
    if bpf_object__load(obj) != 0 {
        fprintf(core::ptr::null_mut(), c"ERROR: loading BPF object file failed\n".as_ptr());
        return goto_cleanup(link, obj, ret);
    }

    map_fd = bpf_object__find_map_fd_by_name(obj, c"my_map".as_ptr());
    if map_fd < 0 {
        fprintf(core::ptr::null_mut(), c"ERROR: finding a map in obj file failed\n".as_ptr());
        return goto_cleanup(link, obj, ret);
    }

    prog = bpf_object__find_program_by_name(obj, c"bpf_prog1".as_ptr());
    if libbpf_get_error(prog.cast()) != 0 {
        fprintf(core::ptr::null_mut(), c"ERROR: finding a prog in obj file failed\n".as_ptr());
        return goto_cleanup(link, obj, ret);
    }

    link = bpf_program__attach(prog);
    if libbpf_get_error(link.cast()) != 0 {
        fprintf(core::ptr::null_mut(), c"ERROR: bpf_program__attach failed\n".as_ptr());
        link = core::ptr::null_mut();
        return goto_cleanup(link, obj, ret);
    }

    pb = perf_buffer__new(map_fd, 8, Some(print_bpf_output), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    ret = libbpf_get_error(pb.cast()) as c_int;
    if ret != 0 {
        printf(c"failed to setup perf_buffer: %d\n".as_ptr(), ret);
        return 1;
    }

    f = popen(c"taskset 1 dd if=/dev/zero of=/dev/null\0".as_ptr(), c"r\0".as_ptr());
    let _ = f;

    START_TIME = time_get_ns();
    while {
        ret = perf_buffer__poll(pb, 1000);
        ret >= 0 && CNT < MAX_CNT as u64
    } {}
    kill(0, SIGINT);

    bpf_link__destroy(link);
    bpf_object__close(obj);
    ret
}

unsafe fn goto_cleanup(link: *mut BpfLink, obj: *mut BpfObject, ret: c_int) -> c_int {
    bpf_link__destroy(link);
    bpf_object__close(obj);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
