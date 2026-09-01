// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <test_progs.h>

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_raw_tracepoint_open(name: *const c_char, prog_fd: c_int) -> c_int;
    fn bpf_task_fd_query(
        pid: c_int,
        fd: c_int,
        flags: __u32,
        buf: *mut c_char,
        buf_len: *mut __u32,
        prog_id: *mut __u32,
        fd_type: *mut __u32,
        probe_offset: *mut __u64,
        probe_addr: *mut __u64,
    ) -> c_int;
    fn getpid() -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn bpf_object__close(obj: *mut bpf_object);
    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...) -> bool;
}

unsafe extern "C" {
    static BPF_PROG_TYPE_RAW_TRACEPOINT: c_int;
    static BPF_FD_TYPE_RAW_TRACEPOINT: __u32;
    static ENOSPC: c_int;
}

pub unsafe fn test_task_fd_query_rawtp() {
    let file: *const c_char = c"./test_get_stack_rawtp.bpf.o".as_ptr();
    let mut probe_offset: __u64 = 0;
    let mut probe_addr: __u64 = 0;
    let mut len: __u32 = 0;
    let mut prog_id: __u32 = 0;
    let mut fd_type: __u32 = 0;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut efd: c_int;
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let _duration: __u32 = 0;
    let mut buf = [0 as c_char; 256];

    err = unsafe {
        bpf_prog_test_load(
            file,
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            &mut obj,
            &mut prog_fd,
        )
    };
    if unsafe {
        CHECK(
            err != 0,
            c"prog_load raw tp".as_ptr(),
            c"err %d errno %d\n".as_ptr(),
            err,
            errno,
        )
    } {
        return;
    }

    'close_prog: {
        efd = unsafe { bpf_raw_tracepoint_open(c"sys_enter".as_ptr(), prog_fd) };
        if unsafe {
            CHECK(
                efd < 0,
                c"raw_tp_open".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                efd,
                errno,
            )
        } {
            break 'close_prog;
        }

        /* query (getpid(), efd) */
        len = core::mem::size_of_val(&buf) as __u32;
        err = unsafe {
            bpf_task_fd_query(
                getpid(),
                efd,
                0,
                buf.as_mut_ptr(),
                &mut len,
                &mut prog_id,
                &mut fd_type,
                &mut probe_offset,
                &mut probe_addr,
            )
        };
        if unsafe {
            CHECK(
                err < 0,
                c"bpf_task_fd_query".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            )
        } {
            break 'close_prog;
        }

        err = (unsafe { fd_type == BPF_FD_TYPE_RAW_TRACEPOINT }
            && unsafe { strcmp(buf.as_ptr(), c"sys_enter".as_ptr()) == 0 }) as c_int;
        if unsafe {
            CHECK(
                err == 0,
                c"check_results".as_ptr(),
                c"fd_type %d tp_name %s\n".as_ptr(),
                fd_type,
                buf.as_ptr(),
            )
        } {
            break 'close_prog;
        }

        /* test zero len */
        len = 0;
        err = unsafe {
            bpf_task_fd_query(
                getpid(),
                efd,
                0,
                buf.as_mut_ptr(),
                &mut len,
                &mut prog_id,
                &mut fd_type,
                &mut probe_offset,
                &mut probe_addr,
            )
        };
        if unsafe {
            CHECK(
                err < 0,
                c"bpf_task_fd_query (len = 0)".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            )
        } {
            break 'close_prog;
        }
        err = (unsafe { fd_type == BPF_FD_TYPE_RAW_TRACEPOINT }
            && len == unsafe { strlen(c"sys_enter".as_ptr()) } as __u32) as c_int;
        if unsafe {
            CHECK(
                err == 0,
                c"check_results".as_ptr(),
                c"fd_type %d len %u\n".as_ptr(),
                fd_type,
                len,
            )
        } {
            break 'close_prog;
        }

        /* test empty buffer */
        len = core::mem::size_of_val(&buf) as __u32;
        err = unsafe {
            bpf_task_fd_query(
                getpid(),
                efd,
                0,
                core::ptr::null_mut(),
                &mut len,
                &mut prog_id,
                &mut fd_type,
                &mut probe_offset,
                &mut probe_addr,
            )
        };
        if unsafe {
            CHECK(
                err < 0,
                c"bpf_task_fd_query (buf = 0)".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            )
        } {
            break 'close_prog;
        }
        err = (unsafe { fd_type == BPF_FD_TYPE_RAW_TRACEPOINT }
            && len == unsafe { strlen(c"sys_enter".as_ptr()) } as __u32) as c_int;
        if unsafe {
            CHECK(
                err == 0,
                c"check_results".as_ptr(),
                c"fd_type %d len %u\n".as_ptr(),
                fd_type,
                len,
            )
        } {
            break 'close_prog;
        }

        /* test smaller buffer */
        len = 3;
        err = unsafe {
            bpf_task_fd_query(
                getpid(),
                efd,
                0,
                buf.as_mut_ptr(),
                &mut len,
                &mut prog_id,
                &mut fd_type,
                &mut probe_offset,
                &mut probe_addr,
            )
        };
        if unsafe {
            CHECK(
                err >= 0 || errno != ENOSPC,
                c"bpf_task_fd_query (len = 3)".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            )
        } {
            break 'close_prog;
        }
        err = (unsafe { fd_type == BPF_FD_TYPE_RAW_TRACEPOINT }
            && len == unsafe { strlen(c"sys_enter".as_ptr()) } as __u32
            && unsafe { strcmp(buf.as_ptr(), c"sy".as_ptr()) == 0 }) as c_int;
        if unsafe {
            CHECK(
                err == 0,
                c"check_results".as_ptr(),
                c"fd_type %d len %u\n".as_ptr(),
                fd_type,
                len,
            )
        } {
            break 'close_prog;
        }
    }

    unsafe {
        bpf_object__close(obj);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
