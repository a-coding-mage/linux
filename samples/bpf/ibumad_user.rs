// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB

/*
 * ibumad BPF sample user side
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General
 * Public License as published by the Free Software Foundation.
 *
 * Copyright(c) 2018 Ira Weiny, Intel Corporation
 */

// Dependencies supplied by the surrounding build:
// linux/bpf.h, bpf_util.h, bpf/bpf.h, and bpf/libbpf.h.

use std::ffi::{c_char, c_int, c_ulong, c_void};
use std::ptr;

type __u32 = u32;
type __u64 = u64;

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
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

extern "C" {
    static mut optarg: *mut c_char;

    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link) -> c_int;
    fn bpf_object__close(object: *mut bpf_object);
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__load(object: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(object: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(program: *mut bpf_program) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn snprintf(buffer: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strtoul(string: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sleep(seconds: c_ulong) -> c_uint;
    fn exit(status: c_int) -> !;
}

type c_long = i64;
type c_uint = u32;

const NO_ARGUMENT: c_int = 0;
const REQUIRED_ARGUMENT: c_int = 1;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const ULONG_MAX: c_ulong = c_ulong::MAX;

static mut TP_LINKS: [*mut bpf_link; 3] = [ptr::null_mut(); 3];
static mut OBJ: *mut bpf_object = ptr::null_mut();
static mut MAP_FD: [c_int; 2] = [0; 2];
static mut TP_CNT: c_int = 0;

unsafe fn dump_counts(fd: c_int) {
    let mut key: __u32;
    let mut value: __u64;

    key = 0;
    while key < 256 {
        value = 0;
        if bpf_map_lookup_elem(fd, &key as *const __u32 as *const c_void, &mut value as *mut __u64 as *mut c_void) != 0 {
            printf(b"failed to read key %u\n\0".as_ptr() as *const c_char, key);
            key = key.wrapping_add(1);
            continue;
        }
        if value != 0 {
            printf(b"0x%02x : %llu\n\0".as_ptr() as *const c_char, key, value);
        }
        key = key.wrapping_add(1);
    }
}

unsafe fn dump_all_counts() {
    printf(b"Read 'Class : count'\n\0".as_ptr() as *const c_char);
    dump_counts(MAP_FD[0]);
    printf(b"Write 'Class : count'\n\0".as_ptr() as *const c_char);
    dump_counts(MAP_FD[1]);
}

extern "C" fn dump_exit(_sig: c_int) {
    unsafe {
        dump_all_counts();
        /* Detach tracepoints */
        while TP_CNT != 0 {
            TP_CNT -= 1;
            bpf_link__destroy(TP_LINKS[TP_CNT as usize]);
        }
        bpf_object__close(OBJ);
        exit(0);
    }
}

static LONG_OPTIONS: [option; 3] = [
    option { name: b"help\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'h' as c_int },
    option { name: b"delay\0".as_ptr() as *const c_char, has_arg: REQUIRED_ARGUMENT, flag: ptr::null_mut(), val: b'd' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

unsafe fn usage(cmd: *mut c_char) {
    printf(
        b"eBPF test program to count packets from various IP addresses\nUsage: %s <options>\n       --help,   -h  this menu\n       --delay,  -d  <delay>  wait <delay> sec between prints [1 - 1000000]\n\0".as_ptr() as *const c_char,
        cmd,
    );
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut prog: *mut bpf_program;
    let mut delay: c_ulong = 5;
    let mut filename = [0 as c_char; 256];
    let mut longindex: c_int = 0;
    let mut opt: c_int;
    let mut err: c_int = -1;

    loop {
        opt = getopt_long(argc, argv, b"hd:rSw\0".as_ptr() as *const c_char, LONG_OPTIONS.as_ptr(), &mut longindex);
        if opt == -1 { break; }
        match opt {
            x if x == b'd' as c_int => {
                delay = strtoul(optarg, ptr::null_mut(), 0);
                if delay == ULONG_MAX || delay > 1_000_000 {
                    fprintf(ptr::null_mut(), b"ERROR: invalid delay : %s\n\0".as_ptr() as *const c_char, optarg);
                    usage(*argv);
                    return 1;
                }
            }
            _ => {
                usage(*argv);
                return 1;
            }
        }
    }

    signal(SIGINT, dump_exit);
    signal(SIGTERM, dump_exit);
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const c_char, *argv);
    OBJ = bpf_object__open_file(filename.as_ptr(), ptr::null());
    if libbpf_get_error(OBJ as *const c_void) != 0 {
        fprintf(ptr::null_mut(), b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char);
        return err;
    }
    if bpf_object__load(OBJ) != 0 {
        fprintf(ptr::null_mut(), b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup();
    }
    MAP_FD[0] = bpf_object__find_map_fd_by_name(OBJ, b"read_count\0".as_ptr() as *const c_char);
    MAP_FD[1] = bpf_object__find_map_fd_by_name(OBJ, b"write_count\0".as_ptr() as *const c_char);
    if MAP_FD[0] < 0 || MAP_FD[1] < 0 {
        fprintf(ptr::null_mut(), b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup();
    }
    // bpf_object__for_each_program(prog, OBJ)
    loop {
        prog = ptr::null_mut();
        if prog.is_null() { break; }
        TP_LINKS[TP_CNT as usize] = bpf_program__attach(prog);
        if libbpf_get_error(TP_LINKS[TP_CNT as usize] as *const c_void) != 0 {
            fprintf(ptr::null_mut(), b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const c_char);
            TP_LINKS[TP_CNT as usize] = ptr::null_mut();
            goto_cleanup();
        }
        TP_CNT += 1;
    }
    loop {
        sleep(delay);
        dump_all_counts();
    }

    fn goto_cleanup() -> ! {
        unsafe {
            while TP_CNT != 0 {
                TP_CNT -= 1;
                bpf_link__destroy(TP_LINKS[TP_CNT as usize]);
            }
            bpf_object__close(OBJ);
            exit(err);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
