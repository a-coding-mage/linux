// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (C) 2019 Netronome Systems, Inc. */
/* Copyright (C) 2020 Facebook, Inc. */
/*
 * Rust translation of testing_helpers.c.
 *
 * Original C dependencies included ctype.h, stdlib.h, string.h, errno.h,
 * sys/mman.h, alloca.h, bpf/bpf.h, bpf/libbpf.h, disasm.h, test_progs.h,
 * testing_helpers.h, and linux/membarrier.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub type size_t = usize;
pub type __u32 = u32;
pub type __u64 = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EAGAIN: c_int = 11;

const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const _SC_PAGESIZE: c_int = 30;

const __NR_finit_module: c_long = 313;
const __NR_delete_module: c_long = 176;
const __NR_membarrier: c_long = 324;
const MEMBARRIER_CMD_SHARED: c_int = 1;

const BPF_F_TEST_RND_HI32: c_int = 1 << 2;
const BPF_F_TEST_REG_INVARIANTS: c_int = 1 << 3;
const BPF_REG_0: c_int = 0;
const BPF_PROG_TYPE_UNSPEC: bpf_prog_type = 0;
const BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type = 1;

pub type bpf_prog_type = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
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
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_link_info {
    pub prog_id: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub xlated_prog_len: __u32,
    pub xlated_prog_insns: __u64,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: size_t,
    pub kern_version: __u32,
    pub prog_flags: __u32,
    pub log_level: __u32,
    pub log_buf: *mut c_char,
    pub log_size: size_t,
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: size_t,
    pub kernel_log_level: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(
        str: *mut c_char,
        delim: *const c_char,
        saveptr: *mut *mut c_char,
    ) -> *mut c_char;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn bzero(s: *mut c_void, n: size_t);
    fn isspace(c: c_int) -> c_int;

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);

    fn syscall(num: c_long, ...) -> c_long;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;

    fn bpf_link__fd(link: *const bpf_link) -> c_int;
    fn bpf_link_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_link_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const bpf_object_open_opts,
    ) -> *mut bpf_object;
    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__type(prog: *const bpf_program) -> bpf_prog_type;
    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: bpf_prog_type);
    fn bpf_program__flags(prog: *const bpf_program) -> __u32;
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: __u32);
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_prog_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
}

const fn bpf_mov64_imm(dst: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_src: dst as u8,
        off: 0,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_src: 0,
        off: 0,
        imm: 0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_num_list(
    mut s: *const c_char,
    num_set: *mut *mut bool,
    num_set_len: *mut c_int,
) -> c_int {
    unsafe {
        let mut i: c_int;
        let mut set_len: c_int = 0;
        let mut new_len: c_int;
        let mut num: c_int;
        let mut start: c_int = 0;
        let mut end: c_int = -1;
        let mut set: *mut bool = ptr::null_mut();
        let mut tmp: *mut bool;
        let mut parsing_end = false;
        let mut next: *mut c_char = ptr::null_mut();

        while *s.add(0) != 0 {
            errno = 0;
            num = strtol(s, &mut next, 10) as c_int;
            if errno != 0 {
                return -errno;
            }

            if parsing_end {
                end = num;
            } else {
                start = num;
            }

            if !parsing_end && *next == b'-' as c_char {
                s = next.add(1);
                parsing_end = true;
                continue;
            } else if *next == b',' as c_char {
                parsing_end = false;
                s = next.add(1);
                end = num;
            } else if *next == 0 {
                parsing_end = false;
                s = next;
                end = num;
            } else {
                return -EINVAL;
            }

            if start > end {
                return -EINVAL;
            }

            if end + 1 > set_len {
                new_len = end + 1;
                tmp = realloc(set as *mut c_void, new_len as size_t) as *mut bool;
                if tmp.is_null() {
                    free(set as *mut c_void);
                    return -ENOMEM;
                }
                i = set_len;
                while i < start {
                    *tmp.offset(i as isize) = false;
                    i += 1;
                }
                set = tmp;
                set_len = new_len;
            }
            i = start;
            while i <= end {
                *set.offset(i as isize) = true;
                i += 1;
            }
        }

        if set.is_null() || parsing_end {
            return -EINVAL;
        }

        *num_set = set;
        *num_set_len = set_len;

        0
    }
}

unsafe fn do_insert_test(
    set: *mut test_filter_set,
    test_str: *mut c_char,
    subtest_str: *mut c_char,
) -> c_int {
    unsafe {
        let mut tmp: *mut test_filter;
        let mut test: *mut test_filter = ptr::null_mut();
        let mut ctmp: *mut *mut c_char;
        let mut i: c_int;

        i = 0;
        while i < (*set).cnt {
            test = (*set).tests.offset(i as isize);

            if strcmp(test_str, (*test).name) == 0 {
                free(test_str as *mut c_void);
                break;
            }
            i += 1;
        }

        if i == (*set).cnt {
            tmp = realloc(
                (*set).tests as *mut c_void,
                size_of::<test_filter>() * ((*set).cnt as usize + 1),
            ) as *mut test_filter;
            if tmp.is_null() {
                return -ENOMEM;
            }

            (*set).tests = tmp;
            test = (*set).tests.offset((*set).cnt as isize);

            (*test).name = test_str;
            (*test).subtests = ptr::null_mut();
            (*test).subtest_cnt = 0;

            (*set).cnt += 1;
        }

        if subtest_str.is_null() {
            return 0;
        }

        i = 0;
        while i < (*test).subtest_cnt {
            if strcmp(subtest_str, *(*test).subtests.offset(i as isize)) == 0 {
                free(subtest_str as *mut c_void);
                return 0;
            }
            i += 1;
        }

        ctmp = realloc(
            (*test).subtests as *mut c_void,
            size_of::<*mut c_char>() * ((*test).subtest_cnt as usize + 1),
        ) as *mut *mut c_char;
        if ctmp.is_null() {
            return -ENOMEM;
        }

        (*test).subtests = ctmp;
        *(*test).subtests.offset((*test).subtest_cnt as isize) = subtest_str;

        (*test).subtest_cnt += 1;

        0
    }
}

unsafe fn insert_test(
    set: *mut test_filter_set,
    test_spec: *mut c_char,
    is_glob_pattern: bool,
) -> c_int {
    unsafe {
        let pattern: *const c_char;
        let mut subtest_str: *mut c_char;
        let mut ext_test_str: *mut c_char = ptr::null_mut();
        let mut ext_subtest_str: *mut c_char = ptr::null_mut();
        let mut glob_chars: c_int = 0;

        if is_glob_pattern {
            pattern = c"%s".as_ptr();
        } else {
            pattern = c"*%s*".as_ptr();
            glob_chars = 2;
        }

        subtest_str = strchr(test_spec, b'/' as c_int);
        if !subtest_str.is_null() {
            *subtest_str = 0;
            subtest_str = subtest_str.add(1);
        }

        ext_test_str = malloc(strlen(test_spec) + glob_chars as usize + 1) as *mut c_char;
        if ext_test_str.is_null() {
            free(ext_test_str as *mut c_void);
            free(ext_subtest_str as *mut c_void);
            return -ENOMEM;
        }

        sprintf(ext_test_str, pattern, test_spec);

        if !subtest_str.is_null() {
            ext_subtest_str =
                malloc(strlen(subtest_str) + glob_chars as usize + 1) as *mut c_char;
            if ext_subtest_str.is_null() {
                free(ext_test_str as *mut c_void);
                free(ext_subtest_str as *mut c_void);
                return -ENOMEM;
            }

            sprintf(ext_subtest_str, pattern, subtest_str);
        }

        do_insert_test(set, ext_test_str, ext_subtest_str)
    }
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_test_list_file(
    path: *const c_char,
    set: *mut test_filter_set,
    is_glob_pattern: bool,
) -> c_int {
    unsafe {
        let mut buf: *mut c_char = ptr::null_mut();
        let mut capture_start: *mut c_char;
        let mut capture_end: *mut c_char;
        let mut scan_end: *mut c_char;
        let mut buflen: size_t = 0;
        let mut err: c_int = 0;
        let f: *mut FILE;

        f = fopen(path, c"r".as_ptr());
        if f.is_null() {
            err = -errno;
            fprintf(stderr, c"Failed to open '%s': %d\n".as_ptr(), path, err);
            return err;
        }

        while getline(&mut buf, &mut buflen, f) != -1 {
            capture_start = buf;

            while isspace(*capture_start as c_int) != 0 {
                capture_start = capture_start.add(1);
            }

            capture_end = capture_start;
            scan_end = capture_start;

            while *scan_end != 0 && *scan_end != b'#' as c_char {
                if isspace(*scan_end as c_int) == 0 {
                    capture_end = scan_end;
                }

                scan_end = scan_end.add(1);
            }

            if capture_end == capture_start {
                continue;
            }

            capture_end = capture_end.add(1);
            *capture_end = 0;

            err = insert_test(set, capture_start, is_glob_pattern);
            if err != 0 {
                break;
            }
        }

        free(buf as *mut c_void);
        fclose(f);
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_test_list(
    s: *const c_char,
    set: *mut test_filter_set,
    is_glob_pattern: bool,
) -> c_int {
    unsafe {
        let mut input: *mut c_char;
        let mut state: *mut c_char = ptr::null_mut();
        let mut test_spec: *mut c_char;
        let mut err: c_int = 0;
        let mut cnt: c_int = 0;

        input = strdup(s);
        if input.is_null() {
            return -ENOMEM;
        }

        loop {
            test_spec = strtok_r(
                if {
                    let old = cnt;
                    cnt += 1;
                    old
                } != 0
                {
                    ptr::null_mut()
                } else {
                    input
                },
                c",".as_ptr(),
                &mut state,
            );
            if test_spec.is_null() {
                break;
            }
            err = insert_test(set, test_spec, is_glob_pattern);
            if err != 0 {
                break;
            }
        }

        free(input as *mut c_void);
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn link_info_prog_id(
    link: *const bpf_link,
    info: *mut bpf_link_info,
) -> __u32 {
    unsafe {
        let mut info_len: __u32 = size_of::<bpf_link_info>() as __u32;
        let err: c_int;

        memset(info as *mut c_void, 0, size_of::<bpf_link_info>());
        err = bpf_link_get_info_by_fd(bpf_link__fd(link), info, &mut info_len);
        if err != 0 {
            printf(c"failed to get link info: %d\n".as_ptr(), -errno);
            return 0;
        }
        (*info).prog_id
    }
}

#[unsafe(no_mangle)]
pub static mut extra_prog_load_log_flags: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn testing_prog_flags() -> c_int {
    static mut CACHED_FLAGS: c_int = -1;
    static PROG_FLAGS: [c_int; 2] = [BPF_F_TEST_RND_HI32, BPF_F_TEST_REG_INVARIANTS];
    static INSNS: [bpf_insn; 2] = [bpf_mov64_imm(BPF_REG_0, 0), bpf_exit_insn()];

    unsafe {
        let insn_cnt: c_int = INSNS.len() as c_int;
        let mut i: usize;
        let mut fd: c_int;
        let mut flags: c_int = 0;
        let mut opts = bpf_prog_load_opts {
            sz: size_of::<bpf_prog_load_opts>(),
            kern_version: 0,
            prog_flags: 0,
            log_level: 0,
            log_buf: ptr::null_mut(),
            log_size: 0,
        };

        if CACHED_FLAGS >= 0 {
            return CACHED_FLAGS;
        }

        i = 0;
        while i < PROG_FLAGS.len() {
            opts.prog_flags = PROG_FLAGS[i] as __u32;
            fd = bpf_prog_load(
                BPF_PROG_TYPE_SOCKET_FILTER,
                c"flag-test".as_ptr(),
                c"GPL".as_ptr(),
                INSNS.as_ptr(),
                insn_cnt as size_t,
                &opts,
            );
            if fd >= 0 {
                flags |= PROG_FLAGS[i];
                close(fd);
            }
            i += 1;
        }

        CACHED_FLAGS = flags;
        CACHED_FLAGS
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog_test_load(
    file: *const c_char,
    type_: bpf_prog_type,
    pobj: *mut *mut bpf_object,
    prog_fd: *mut c_int,
) -> c_int {
    unsafe {
        let opts = bpf_object_open_opts {
            sz: size_of::<bpf_object_open_opts>(),
            kernel_log_level: extra_prog_load_log_flags,
        };
        let obj: *mut bpf_object;
        let prog: *mut bpf_program;
        let flags: __u32;
        let mut err: c_int;

        obj = bpf_object__open_file(file, &opts);
        if obj.is_null() {
            return -errno;
        }

        prog = bpf_object__next_program(obj, ptr::null_mut());
        if prog.is_null() {
            err = -ENOENT;
            bpf_object__close(obj);
            return err;
        }

        if type_ != BPF_PROG_TYPE_UNSPEC && bpf_program__type(prog) != type_ {
            bpf_program__set_type(prog, type_);
        }

        flags = bpf_program__flags(prog) | testing_prog_flags() as __u32;
        bpf_program__set_flags(prog, flags);

        err = bpf_object__load(obj);
        if err != 0 {
            bpf_object__close(obj);
            return err;
        }

        *pobj = obj;
        *prog_fd = bpf_program__fd(prog);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_test_load_program(
    type_: bpf_prog_type,
    insns: *const bpf_insn,
    insns_cnt: size_t,
    license: *const c_char,
    kern_version: __u32,
    log_buf: *mut c_char,
    log_buf_sz: size_t,
) -> c_int {
    unsafe {
        let opts = bpf_prog_load_opts {
            sz: size_of::<bpf_prog_load_opts>(),
            kern_version,
            prog_flags: testing_prog_flags() as __u32,
            log_level: extra_prog_load_log_flags as __u32,
            log_buf,
            log_size: log_buf_sz,
        };

        bpf_prog_load(type_, ptr::null(), license, insns, insns_cnt, &opts)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_perf_max_sample_freq() -> __u64 {
    unsafe {
        let mut sample_freq: __u64 = 5000; /* fallback to 5000 on error */
        let f: *mut FILE;

        f = fopen(
            c"/proc/sys/kernel/perf_event_max_sample_rate".as_ptr(),
            c"r".as_ptr(),
        );
        if f.is_null() {
            printf(
                c"Failed to open /proc/sys/kernel/perf_event_max_sample_rate: err %d\nreturn default value: 5000\n"
                    .as_ptr(),
                -errno,
            );
            return sample_freq;
        }
        if fscanf(f, c"%llu".as_ptr(), &mut sample_freq) != 1 {
            printf(
                c"Failed to parse /proc/sys/kernel/perf_event_max_sample_rate: err %d\nreturn default value: 5000\n"
                    .as_ptr(),
                -errno,
            );
        }

        fclose(f);
        sample_freq
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn finit_module(
    fd: c_int,
    param_values: *const c_char,
    flags: c_int,
) -> c_int {
    unsafe { syscall(__NR_finit_module, fd, param_values, flags) as c_int }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_module(name: *const c_char, flags: c_int) -> c_int {
    unsafe { syscall(__NR_delete_module, name, flags) as c_int }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn try_unload_module(
    name: *const c_char,
    retries: c_int,
    verbose: bool,
) -> c_int {
    unsafe {
        let mut ret: c_int;
        let mut cnt: c_int = 0;

        if kern_sync_rcu() != 0 {
            fprintf(stdout, c"Failed to trigger kernel-side RCU sync!\n".as_ptr());
        }

        loop {
            ret = delete_module(name, 0);
            if ret == 0 || errno != EAGAIN {
                break;
            }
            cnt += 1;
            if cnt > retries {
                fprintf(stdout, c"Unload of %s timed out\n".as_ptr(), name);
                break;
            }
            usleep(100);
        }

        if ret != 0 {
            if errno == ENOENT {
                if verbose {
                    fprintf(stdout, c"%s.ko is already unloaded.\n".as_ptr(), name);
                }
                return -1;
            }
            fprintf(
                stdout,
                c"Failed to unload %s.ko from kernel: %d\n".as_ptr(),
                name,
                -errno,
            );
            return -1;
        }
        if verbose {
            fprintf(stdout, c"Successfully unloaded %s.ko.\n".as_ptr(), name);
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload_module(name: *const c_char, verbose: bool) -> c_int {
    unsafe { try_unload_module(name, 10000, verbose) }
}

unsafe fn __load_module(
    path: *const c_char,
    param_values: *const c_char,
    verbose: bool,
) -> c_int {
    unsafe {
        let fd: c_int;

        if verbose {
            fprintf(stdout, c"Loading %s...\n".as_ptr(), path);
        }

        fd = open(path, O_RDONLY);
        if fd < 0 {
            fprintf(stdout, c"Can't find %s kernel module: %d\n".as_ptr(), path, -errno);
            return -ENOENT;
        }
        if finit_module(fd, param_values, 0) != 0 {
            fprintf(
                stdout,
                c"Failed to load %s into the kernel: %d\n".as_ptr(),
                path,
                -errno,
            );
            close(fd);
            return -EINVAL;
        }
        close(fd);

        if verbose {
            fprintf(stdout, c"Successfully loaded %s.\n".as_ptr(), path);
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_module_params(
    path: *const c_char,
    param_values: *const c_char,
    verbose: bool,
) -> c_int {
    unsafe { __load_module(path, param_values, verbose) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_module(path: *const c_char, verbose: bool) -> c_int {
    unsafe { __load_module(path, c"".as_ptr(), verbose) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload_bpf_testmod(verbose: bool) -> c_int {
    unsafe { unload_module(c"bpf_testmod".as_ptr(), verbose) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_bpf_testmod(verbose: bool) -> c_int {
    unsafe { load_module(c"bpf_testmod.ko".as_ptr(), verbose) }
}

/*
 * Trigger synchronize_rcu() in kernel.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kern_sync_rcu() -> c_int {
    unsafe { syscall(__NR_membarrier, MEMBARRIER_CMD_SHARED, 0, 0) as c_int }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_xlated_program(
    fd_prog: c_int,
    buf: *mut *mut bpf_insn,
    cnt: *mut __u32,
) -> c_int {
    unsafe {
        let buf_element_size: __u32 = size_of::<bpf_insn>() as __u32;
        let mut info: bpf_prog_info = core::mem::zeroed();
        let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
        let xlated_prog_len: __u32;

        if bpf_prog_get_info_by_fd(fd_prog, &mut info, &mut info_len) != 0 {
            perror(c"bpf_prog_get_info_by_fd failed".as_ptr());
            return -1;
        }

        xlated_prog_len = info.xlated_prog_len;
        if xlated_prog_len % buf_element_size != 0 {
            printf(
                c"Program length %u is not multiple of %u\n".as_ptr(),
                xlated_prog_len,
                buf_element_size,
            );
            return -1;
        }

        *cnt = xlated_prog_len / buf_element_size;
        *buf = calloc(*cnt as size_t, buf_element_size as size_t) as *mut bpf_insn;
        if (*buf).is_null() {
            perror(c"can't allocate xlated program buffer".as_ptr());
            return -ENOMEM;
        }

        bzero(&mut info as *mut _ as *mut c_void, size_of::<bpf_prog_info>());
        info.xlated_prog_len = xlated_prog_len;
        info.xlated_prog_insns = *buf as c_ulong as __u64;
        if bpf_prog_get_info_by_fd(fd_prog, &mut info, &mut info_len) != 0 {
            perror(c"second bpf_prog_get_info_by_fd failed".as_ptr());
            free(*buf as *mut c_void);
            *buf = ptr::null_mut();
            return -1;
        }

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_jit_enabled() -> bool {
    unsafe {
        let jit_sysctl: *const c_char = c"/proc/sys/net/core/bpf_jit_enable".as_ptr();
        let mut enabled = false;
        let sysctl_fd: c_int;

        sysctl_fd = open(jit_sysctl, O_RDONLY);
        if sysctl_fd != -1 {
            let mut tmpc: c_char = 0;

            if read(
                sysctl_fd,
                &mut tmpc as *mut _ as *mut c_void,
                size_of::<c_char>(),
            ) == 1
            {
                enabled = tmpc != b'0' as c_char;
            }
            close(sysctl_fd);
        }

        enabled
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_mprotect() -> c_int {
    unsafe {
        let buf: *mut u8;
        let sz: c_long;
        let ret: c_int;

        sz = sysconf(_SC_PAGESIZE);
        if sz < 0 {
            return sz as c_int;
        }

        let mut storage = vec![0u8; (sz * 3) as usize];
        buf = storage.as_mut_ptr();
        ret = mprotect(
            (((buf.offset(sz as isize) as c_ulong) & !((sz - 1) as c_ulong)) as *mut c_void),
            sz as size_t,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );
        ret
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
