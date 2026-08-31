// SPDX-License-Identifier: GPL-2.0
// C source included pthread.h, sched.h, sys/socket.h, and test_progs.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::{mem, ptr};

const MAX_CNT_RAWTP: u64 = 10u64;
const MAX_STACK_RAWTP: usize = 100;

static mut duration: c_int = 0;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
struct bpf_stack_build_id {
    _private: [u8; 0],
}

#[repr(C)]
struct get_stack_trace_t {
    pid: c_int,
    kern_stack_size: c_int,
    user_stack_size: c_int,
    user_stack_buildid_size: c_int,
    kern_stack: [__u64; MAX_STACK_RAWTP],
    user_stack: [__u64; MAX_STACK_RAWTP],
    user_stack_buildid: [bpf_stack_build_id; MAX_STACK_RAWTP],
}

#[repr(C)]
struct ksym {
    name: *const c_char,
}

#[repr(C)]
struct env_t {
    jit_enabled: bool,
}

#[repr(C)]
struct perf_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

#[repr(C)]
struct cpu_set_t {
    _private: [u8; 0],
}

const BPF_PROG_TYPE_RAW_TRACEPOINT: c_int = 0;

unsafe extern "C" {
    static mut env: env_t;
    static mut errno: c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn ksym_search(addr: __u64) -> *mut ksym;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn load_kallsyms() -> c_int;
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn pthread_self() -> usize;
    fn pthread_setaffinity_np(thread: usize, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
        lost_cb: *mut c_void,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut perf_buffer;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn perf_buffer__free(pb: *mut perf_buffer);
    fn bpf_object__close(obj: *mut bpf_object);
}

unsafe extern "C" fn get_stack_print_output(
    _ctx: *mut c_void,
    _cpu: c_int,
    data: *mut c_void,
    size: __u32,
) {
    let mut good_kern_stack = false;
    let mut good_user_stack = false;
    let nonjit_func = b"___bpf_prog_run\0".as_ptr() as *const c_char;
    /* perfbuf-submitted data is 4-byte aligned, but we need 8-byte
     * alignment, so copy data into a local variable, for simplicity
     */
    let mut e: get_stack_trace_t = mem::zeroed();
    let mut i: c_int;
    let mut num_stack: c_int;
    let mut ks: *mut ksym;

    ptr::write_bytes(
        &mut e as *mut get_stack_trace_t as *mut u8,
        0,
        mem::size_of::<get_stack_trace_t>(),
    );
    ptr::copy_nonoverlapping(
        data as *const u8,
        &mut e as *mut get_stack_trace_t as *mut u8,
        if (size as usize) <= mem::size_of::<get_stack_trace_t>() {
            size as usize
        } else {
            mem::size_of::<get_stack_trace_t>()
        },
    );

    if (size as usize) < mem::size_of::<get_stack_trace_t>() {
        let raw_data = data as *mut __u64;
        let mut found = false;

        num_stack = (size as usize / mem::size_of::<__u64>()) as c_int;
        /* If jit is enabled, we do not have a good way to
         * verify the sanity of the kernel stack. So we
         * just assume it is good if the stack is not empty.
         * This could be improved in the future.
         */
        if env.jit_enabled {
            found = num_stack > 0;
        } else {
            i = 0;
            while i < num_stack {
                ks = ksym_search(*raw_data.add(i as usize));
                if !ks.is_null() && strcmp((*ks).name, nonjit_func) == 0 {
                    found = true;
                    break;
                }
                i += 1;
            }
        }
        if found {
            good_kern_stack = true;
            good_user_stack = true;
        }
    } else {
        num_stack = (e.kern_stack_size as usize / mem::size_of::<__u64>()) as c_int;
        if env.jit_enabled {
            good_kern_stack = num_stack > 0;
        } else {
            i = 0;
            while i < num_stack {
                ks = ksym_search(e.kern_stack[i as usize]);
                if !ks.is_null() && strcmp((*ks).name, nonjit_func) == 0 {
                    good_kern_stack = true;
                    break;
                }
                i += 1;
            }
        }
        if e.user_stack_size > 0 && e.user_stack_buildid_size > 0 {
            good_user_stack = true;
        }
    }

    if !good_kern_stack {
        CHECK(
            !good_kern_stack,
            b"kern_stack\0".as_ptr() as *const c_char,
            b"corrupted kernel stack\n\0".as_ptr() as *const c_char,
        );
    }
    if !good_user_stack {
        CHECK(
            !good_user_stack,
            b"user_stack\0".as_ptr() as *const c_char,
            b"corrupted user stack\n\0".as_ptr() as *const c_char,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_get_stack_raw_tp() {
    let file = b"./test_get_stack_rawtp.bpf.o\0".as_ptr() as *const c_char;
    let file_err = b"./test_get_stack_rawtp_err.bpf.o\0".as_ptr() as *const c_char;
    let prog_name = b"bpf_prog1\0".as_ptr() as *const c_char;
    let mut i: c_int;
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut exp_cnt: c_int = MAX_CNT_RAWTP as c_int;
    let mut pb: *mut perf_buffer = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();
    let tv = timespec {
        tv_sec: 0,
        tv_nsec: 10,
    };
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut map: *mut bpf_map;
    let mut cpu_set: cpu_set_t = mem::zeroed();

    err = bpf_prog_test_load(
        file_err,
        BPF_PROG_TYPE_RAW_TRACEPOINT,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK(
        err >= 0,
        b"prog_load raw tp\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        return;
    }

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_RAW_TRACEPOINT, &mut obj, &mut prog_fd);
    if CHECK(
        err != 0,
        b"prog_load raw tp\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        return;
    }

    prog = bpf_object__find_program_by_name(obj, prog_name);
    if CHECK(
        prog.is_null(),
        b"find_probe\0".as_ptr() as *const c_char,
        b"prog '%s' not found\n\0".as_ptr() as *const c_char,
        prog_name,
    ) {
        goto_close_prog(link, pb, obj);
        return;
    }

    map = bpf_object__find_map_by_name(obj, b"perfmap\0".as_ptr() as *const c_char);
    if CHECK(
        map.is_null(),
        b"bpf_find_map\0".as_ptr() as *const c_char,
        b"not found\n\0".as_ptr() as *const c_char,
    ) {
        goto_close_prog(link, pb, obj);
        return;
    }

    err = load_kallsyms();
    if CHECK(
        err < 0,
        b"load_kallsyms\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_prog(link, pb, obj);
        return;
    }

    CPU_ZERO(&mut cpu_set);
    CPU_SET(0, &mut cpu_set);
    err = pthread_setaffinity_np(pthread_self(), mem::size_of_val(&cpu_set), &cpu_set);
    if CHECK(
        err != 0,
        b"set_affinity\0".as_ptr() as *const c_char,
        b"err %d, errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_prog(link, pb, obj);
        return;
    }

    link = bpf_program__attach_raw_tracepoint(prog, b"sys_enter\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_raw_tp\0".as_ptr() as *const c_char) {
        goto_close_prog(link, pb, obj);
        return;
    }

    pb = perf_buffer__new(
        bpf_map__fd(map),
        8,
        Some(get_stack_print_output),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if !ASSERT_OK_PTR(pb as *mut c_void, b"perf_buf__new\0".as_ptr() as *const c_char) {
        goto_close_prog(link, pb, obj);
        return;
    }

    /* trigger some syscall action */
    i = 0;
    while i < MAX_CNT_RAWTP as c_int {
        nanosleep(&tv, ptr::null_mut());
        i += 1;
    }

    while exp_cnt > 0 {
        err = perf_buffer__poll(pb, 100);
        if err < 0
            && CHECK(
                err < 0,
                b"pb__poll\0".as_ptr() as *const c_char,
                b"err %d\n\0".as_ptr() as *const c_char,
                err,
            )
        {
            goto_close_prog(link, pb, obj);
            return;
        }
        exp_cnt -= err;
    }

    goto_close_prog(link, pb, obj);
}

unsafe fn goto_close_prog(link: *mut bpf_link, pb: *mut perf_buffer, obj: *mut bpf_object) {
    bpf_link__destroy(link);
    perf_buffer__free(pb);
    bpf_object__close(obj);
}
