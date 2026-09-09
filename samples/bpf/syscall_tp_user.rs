// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

type __u32 = u32;

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

const O_RDONLY: c_int = 0;
const BPF_ANY: __u32 = 0;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut errno: c_int;
    static mut stderr: *mut c_void;
    fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u32) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

unsafe fn usage(cmd: *const c_char) {
    printf(b"USAGE: %s [-i nr_tests] [-h]\n\0".as_ptr() as *const c_char, cmd);
    printf(b"       -i nr_tests      # rounds of test to run\n\0".as_ptr() as *const c_char);
    printf(b"       -h               # help\n\0".as_ptr() as *const c_char);
}

unsafe fn verify_map(map_id: c_int) {
    let key: __u32 = 0;
    let mut val: __u32 = 0;

    if bpf_map_lookup_elem(map_id, &key as *const _ as *const c_void, &mut val as *mut _ as *mut c_void) != 0 {
        fprintf(stderr, b"map_lookup failed: %s\n\0".as_ptr() as *const c_char, strerror(errno));
        return;
    }
    if val == 0 {
        fprintf(stderr, b"failed: map #%d returns value 0\n\0".as_ptr() as *const c_char, map_id);
        return;
    }
    printf(b"verify map:%d val: %d\n\0".as_ptr() as *const c_char, map_id, val);
    val = 0;
    if bpf_map_update_elem(map_id, &key as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY) != 0 {
        fprintf(stderr, b"map_update failed: %s\n\0".as_ptr() as *const c_char, strerror(errno));
    }
}

unsafe fn test(filename: *mut c_char, nr_tests: c_int) -> c_int {
    let mut map0_fds = vec![0; nr_tests as usize];
    let mut map1_fds = vec![0; nr_tests as usize];
    let mut objs: Vec<*mut bpf_object> = vec![ptr::null_mut(); nr_tests as usize];
    let mut links: *mut *mut bpf_link = ptr::null_mut();
    let mut j: c_int = 0;

    for i in 0..nr_tests as usize {
        objs[i] = bpf_object__open_file(filename, ptr::null());
        if libbpf_get_error(objs[i] as *const c_void) != 0 {
            fprintf(stderr, b"opening BPF object file failed\n\0".as_ptr() as *const c_char);
            objs[i] = ptr::null_mut();
            break;
        }

        /* One-time initialization. The bpf_object__for_each_program macro is
         * represented here by the corresponding program iteration dependency. */
        if links.is_null() {
            let nr_progs: usize = 0;
            links = calloc(nr_progs * nr_tests as usize, std::mem::size_of::<*mut bpf_link>()) as *mut *mut bpf_link;
            if links.is_null() { break; }
        }
        if bpf_object__load(objs[i]) != 0 {
            fprintf(stderr, b"loading BPF object file failed\n\0".as_ptr() as *const c_char);
            break;
        }
        map0_fds[i] = bpf_object__find_map_fd_by_name(objs[i], b"enter_open_map\0".as_ptr() as *const c_char);
        map1_fds[i] = bpf_object__find_map_fd_by_name(objs[i], b"exit_open_map\0".as_ptr() as *const c_char);
        if map0_fds[i] < 0 || map1_fds[i] < 0 { break; }
        /* bpf_object__for_each_program(prog, objs[i]) */
        printf(b"prog #%d: map ids %d %d\n\0".as_ptr() as *const c_char, i as c_int, map0_fds[i], map1_fds[i]);
    }

    let fd = open(filename, O_RDONLY);
    if fd < 0 {
        fprintf(stderr, b"open failed: %s\n\0".as_ptr() as *const c_char, strerror(errno));
        return 1;
    }
    close(fd);
    for i in 0..nr_tests as usize { verify_map(map0_fds[i]); verify_map(map1_fds[i]); }

    if !links.is_null() {
        while j > 0 { j -= 1; bpf_link__destroy(*links.offset(j as isize)); }
        free(links as *mut c_void);
    }
    for i in 0..nr_tests as usize { if !objs[i].is_null() { bpf_object__close(objs[i]); } }
    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut nr_tests = 1;
    let mut opt;
    while { opt = getopt(argc, argv, b"i:h\0".as_ptr() as *const c_char); opt != -1 } {
        match opt {
            105 => nr_tests = atoi(optarg),
            _ => { usage(*argv); return 0; }
        }
    }
    let mut filename = [0 as c_char; 256];
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const c_char, *argv);
    test(filename.as_mut_ptr(), nr_tests)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
