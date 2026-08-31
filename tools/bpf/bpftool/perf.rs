// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Facebook
// Author: Yonghong Song <yhs@fb.com>

// C source included GNU/libc, libbpf, and bpftool main definitions.
// Their items are referenced here as external dependencies.

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [i8; 256],
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const i8,
    pub func: Option<unsafe extern "C" fn(i32, *mut *mut i8) -> i32>,
}

const O_RDONLY: i32 = 0;
const ENOTSUPP: i32 = 524;

// Values are provided by libbpf headers in the original C translation unit.
unsafe extern "C" {
    static BPF_FD_TYPE_RAW_TRACEPOINT: __u32;
    static BPF_FD_TYPE_TRACEPOINT: __u32;
    static BPF_FD_TYPE_KPROBE: __u32;
    static BPF_FD_TYPE_KRETPROBE: __u32;
    static BPF_FD_TYPE_UPROBE: __u32;
    static BPF_FD_TYPE_URETPROBE: __u32;
}

unsafe extern "C" {
    static mut errno: i32;
    static mut json_output: bool;
    static mut json_wtr: *mut json_writer_t;
    static mut bin_name: *const i8;

    fn open(pathname: *const i8, flags: i32, ...) -> i32;
    fn close(fd: i32) -> i32;
    fn getpid() -> i32;
    fn strerror(errnum: i32) -> *mut i8;
    fn fprintf(stream: *mut FILE, format: *const i8, ...) -> i32;
    static mut stderr: *mut FILE;
    fn printf(format: *const i8, ...) -> i32;
    fn snprintf(str_: *mut i8, size: usize, format: *const i8, ...) -> i32;
    fn opendir(name: *const i8) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> i32;
    fn isdigit(c: i32) -> i32;

    fn bpf_task_fd_query(
        pid: i32,
        fd: i32,
        flags: u32,
        buf: *mut i8,
        buf_len: *mut __u32,
        prog_id: *mut __u32,
        fd_type: *mut __u32,
        probe_offset: *mut __u64,
        probe_addr: *mut __u64,
    ) -> i32;

    fn p_err(format: *const i8, ...) -> i32;
    fn jsonw_start_object(wtr: *mut json_writer_t);
    fn jsonw_end_object(wtr: *mut json_writer_t);
    fn jsonw_start_array(wtr: *mut json_writer_t);
    fn jsonw_end_array(wtr: *mut json_writer_t);
    fn jsonw_int_field(wtr: *mut json_writer_t, name: *const i8, value: i32);
    fn jsonw_uint_field(wtr: *mut json_writer_t, name: *const i8, value: u32);
    fn jsonw_lluint_field(wtr: *mut json_writer_t, name: *const i8, value: u64);
    fn jsonw_string_field(wtr: *mut json_writer_t, name: *const i8, value: *const i8);
    fn cmd_select(
        cmds: *const cmd,
        argc: i32,
        argv: *mut *mut i8,
        help: unsafe extern "C" fn(i32, *mut *mut i8) -> i32,
    ) -> i32;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* 0: undecided, 1: supported, 2: not supported */
static mut perf_query_supported: i32 = 0;

unsafe fn has_perf_query_support() -> bool {
    let mut probe_offset: __u64 = 0;
    let mut probe_addr: __u64 = 0;
    let mut len: __u32;
    let mut prog_id: __u32 = 0;
    let mut fd_type: __u32 = 0;
    let mut buf: [i8; 256] = [0; 256];
    let fd: i32;

    if perf_query_supported != 0 {
        return perf_query_supported == 1;
    }

    fd = open(c"/".as_ptr(), O_RDONLY);
    if fd < 0 {
        p_err(
            c"perf_query_support: cannot open directory \"/\" (%s)".as_ptr(),
            strerror(errno),
        );
        return perf_query_supported == 1;
    }

    /* the following query will fail as no bpf attachment,
     * the expected errno is ENOTSUPP
     */
    errno = 0;
    len = core::mem::size_of_val(&buf) as __u32;
    bpf_task_fd_query(
        getpid(),
        fd,
        0,
        buf.as_mut_ptr(),
        &mut len,
        &mut prog_id,
        &mut fd_type,
        &mut probe_offset,
        &mut probe_addr,
    );

    if errno == ENOTSUPP {
        perf_query_supported = 1;
        close(fd);
        return perf_query_supported == 1;
    }

    perf_query_supported = 2;
    p_err(c"perf_query_support: %s".as_ptr(), strerror(errno));
    fprintf(
        stderr,
        c"HINT: non root or kernel doesn't support TASK_FD_QUERY\n".as_ptr(),
    );

    close(fd);
    perf_query_supported == 1
}

unsafe fn print_perf_json(
    pid: i32,
    fd: i32,
    prog_id: __u32,
    fd_type: __u32,
    buf: *mut i8,
    probe_offset: __u64,
    probe_addr: __u64,
) {
    jsonw_start_object(json_wtr);
    jsonw_int_field(json_wtr, c"pid".as_ptr(), pid);
    jsonw_int_field(json_wtr, c"fd".as_ptr(), fd);
    jsonw_uint_field(json_wtr, c"prog_id".as_ptr(), prog_id);
    if fd_type == BPF_FD_TYPE_RAW_TRACEPOINT {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"raw_tracepoint".as_ptr());
        jsonw_string_field(json_wtr, c"tracepoint".as_ptr(), buf);
    } else if fd_type == BPF_FD_TYPE_TRACEPOINT {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"tracepoint".as_ptr());
        jsonw_string_field(json_wtr, c"tracepoint".as_ptr(), buf);
    } else if fd_type == BPF_FD_TYPE_KPROBE {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"kprobe".as_ptr());
        if *buf != 0 {
            jsonw_string_field(json_wtr, c"func".as_ptr(), buf);
            jsonw_lluint_field(json_wtr, c"offset".as_ptr(), probe_offset);
        } else {
            jsonw_lluint_field(json_wtr, c"addr".as_ptr(), probe_addr);
        }
    } else if fd_type == BPF_FD_TYPE_KRETPROBE {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"kretprobe".as_ptr());
        if *buf != 0 {
            jsonw_string_field(json_wtr, c"func".as_ptr(), buf);
            jsonw_lluint_field(json_wtr, c"offset".as_ptr(), probe_offset);
        } else {
            jsonw_lluint_field(json_wtr, c"addr".as_ptr(), probe_addr);
        }
    } else if fd_type == BPF_FD_TYPE_UPROBE {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"uprobe".as_ptr());
        jsonw_string_field(json_wtr, c"filename".as_ptr(), buf);
        jsonw_lluint_field(json_wtr, c"offset".as_ptr(), probe_offset);
    } else if fd_type == BPF_FD_TYPE_URETPROBE {
        jsonw_string_field(json_wtr, c"fd_type".as_ptr(), c"uretprobe".as_ptr());
        jsonw_string_field(json_wtr, c"filename".as_ptr(), buf);
        jsonw_lluint_field(json_wtr, c"offset".as_ptr(), probe_offset);
    }
    jsonw_end_object(json_wtr);
}

unsafe fn print_perf_plain(
    pid: i32,
    fd: i32,
    prog_id: __u32,
    fd_type: __u32,
    buf: *mut i8,
    probe_offset: __u64,
    probe_addr: __u64,
) {
    printf(c"pid %d  fd %d: prog_id %u  ".as_ptr(), pid, fd, prog_id);
    if fd_type == BPF_FD_TYPE_RAW_TRACEPOINT {
        printf(c"raw_tracepoint  %s\n".as_ptr(), buf);
    } else if fd_type == BPF_FD_TYPE_TRACEPOINT {
        printf(c"tracepoint  %s\n".as_ptr(), buf);
    } else if fd_type == BPF_FD_TYPE_KPROBE {
        if *buf != 0 {
            printf(c"kprobe  func %s  offset %llu\n".as_ptr(), buf, probe_offset);
        } else {
            printf(c"kprobe  addr %llu\n".as_ptr(), probe_addr);
        }
    } else if fd_type == BPF_FD_TYPE_KRETPROBE {
        if *buf != 0 {
            printf(c"kretprobe  func %s  offset %llu\n".as_ptr(), buf, probe_offset);
        } else {
            printf(c"kretprobe  addr %llu\n".as_ptr(), probe_addr);
        }
    } else if fd_type == BPF_FD_TYPE_UPROBE {
        printf(c"uprobe  filename %s  offset %llu\n".as_ptr(), buf, probe_offset);
    } else if fd_type == BPF_FD_TYPE_URETPROBE {
        printf(c"uretprobe  filename %s  offset %llu\n".as_ptr(), buf, probe_offset);
    }
}

unsafe fn show_proc() -> i32 {
    let mut proc_de: *mut dirent;
    let mut pid_fd_de: *mut dirent;
    let mut probe_offset: __u64 = 0;
    let mut probe_addr: __u64 = 0;
    let mut len: __u32;
    let mut prog_id: __u32 = 0;
    let mut fd_type: __u32 = 0;
    let mut proc: *mut DIR;
    let mut pid_fd: *mut DIR;
    let mut err: i32;
    let mut pid: i32;
    let mut fd: i32;
    let mut pch: *const i8;
    let mut buf: [i8; 4096] = [0; 4096];

    proc = opendir(c"/proc".as_ptr());
    if proc.is_null() {
        return -1;
    }

    loop {
        proc_de = readdir(proc);
        if proc_de.is_null() {
            break;
        }

        pid = 0;
        pch = (*proc_de).d_name.as_ptr();

        /* pid should be all numbers */
        while isdigit(*pch as i32) != 0 {
            pid = pid * 10 + *pch as i32 - '0' as i32;
            pch = pch.add(1);
        }
        if *pch != 0 {
            continue;
        }

        err = snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf),
            c"/proc/%s/fd".as_ptr(),
            (*proc_de).d_name.as_ptr(),
        );
        if err < 0 || err >= core::mem::size_of_val(&buf) as i32 {
            continue;
        }

        pid_fd = opendir(buf.as_ptr());
        if pid_fd.is_null() {
            continue;
        }

        loop {
            pid_fd_de = readdir(pid_fd);
            if pid_fd_de.is_null() {
                break;
            }

            fd = 0;
            pch = (*pid_fd_de).d_name.as_ptr();

            /* fd should be all numbers */
            while isdigit(*pch as i32) != 0 {
                fd = fd * 10 + *pch as i32 - '0' as i32;
                pch = pch.add(1);
            }
            if *pch != 0 {
                continue;
            }

            /* query (pid, fd) for potential perf events */
            len = core::mem::size_of_val(&buf) as __u32;
            err = bpf_task_fd_query(
                pid,
                fd,
                0,
                buf.as_mut_ptr(),
                &mut len,
                &mut prog_id,
                &mut fd_type,
                &mut probe_offset,
                &mut probe_addr,
            );
            if err < 0 {
                continue;
            }

            if json_output {
                print_perf_json(
                    pid,
                    fd,
                    prog_id,
                    fd_type,
                    buf.as_mut_ptr(),
                    probe_offset,
                    probe_addr,
                );
            } else {
                print_perf_plain(
                    pid,
                    fd,
                    prog_id,
                    fd_type,
                    buf.as_mut_ptr(),
                    probe_offset,
                    probe_addr,
                );
            }
        }
        closedir(pid_fd);
    }
    closedir(proc);
    0
}

unsafe extern "C" fn do_show(_argc: i32, _argv: *mut *mut i8) -> i32 {
    let err: i32;

    if !has_perf_query_support() {
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    }
    err = show_proc();
    if json_output {
        jsonw_end_array(json_wtr);
    }

    err
}

unsafe extern "C" fn do_help(_argc: i32, argv: *mut *mut i8) -> i32 {
    fprintf(
        stderr,
        c"Usage: %1$s %2$s { show | list }\n       %1$s %2$s help\n\n       OPTIONS := { {-j|--json} [{-p|--pretty}] | {-d|--debug} | {-l|--legacy} | {-V|--version} }\n".as_ptr(),
        bin_name,
        *argv.offset(-2),
    );

    0
}

static cmds: [cmd; 4] = [
    cmd {
        cmd: c"show".as_ptr(),
        func: Some(do_show),
    },
    cmd {
        cmd: c"list".as_ptr(),
        func: Some(do_show),
    },
    cmd {
        cmd: c"help".as_ptr(),
        func: Some(do_help),
    },
    cmd {
        cmd: core::ptr::null(),
        func: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn do_perf(argc: i32, argv: *mut *mut i8) -> i32 {
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}
