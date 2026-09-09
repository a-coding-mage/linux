// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Facebook
 */

// C headers and project headers are supplied by the surrounding build.

use std::ffi::{c_char, c_int, c_void};
use std::mem::size_of;
use std::process;

extern "C" {
    fn rand() -> c_int;
    fn connect(fd: c_int, addr: *const c_void, len: u32) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    static mut stderr: *mut c_void;
}

#[repr(C)]
struct bpf_map_info {
    _data: [u8; 256],
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

extern "C" {
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut u32) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_object__open_file(filename: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const AF_INET6: u16 = 10;
const EBADF: c_int = 9;

static mut MAP_FD: [c_int; 7] = [0; 7];
static TEST_NAMES: [&[u8]; 3] = [b"Array of Array\0", b"Hash of Array\0", b"Hash of Hash\0"];

unsafe fn check_map_id(inner_map_fd: c_int, map_in_map_fd: c_int, key: u32) {
    let mut info = bpf_map_info { _data: [0; 256] };
    let mut info_len = size_of::<bpf_map_info>() as u32;
    let mut id: c_int = 0;
    let ret = bpf_map_get_info_by_fd(inner_map_fd, &mut info, &mut info_len);
    assert!(ret == 0);
    let ret = bpf_map_lookup_elem(map_in_map_fd, &key as *const _ as *const c_void, &mut id as *mut _ as *mut c_void);
    assert!(ret == 0);
    assert!(id == i32::from_ne_bytes([info._data[0], info._data[1], info._data[2], info._data[3]]));
}

unsafe fn populate_map(port_key: u32, magic_result: c_int) {
    let ret = bpf_map_update_elem(MAP_FD[0], &port_key as *const _ as *const c_void, &magic_result as *const _ as *const c_void, BPF_ANY);
    assert!(ret == 0);
    let ret = bpf_map_update_elem(MAP_FD[1], &port_key as *const _ as *const c_void, &magic_result as *const _ as *const c_void, BPF_NOEXIST);
    assert!(ret == 0);
    let ret = bpf_map_update_elem(MAP_FD[4], &port_key as *const _ as *const c_void, &MAP_FD[0] as *const _ as *const c_void, BPF_ANY);
    assert!(ret == 0); check_map_id(MAP_FD[0], MAP_FD[4], port_key);
    let ret = bpf_map_update_elem(MAP_FD[5], &port_key as *const _ as *const c_void, &MAP_FD[0] as *const _ as *const c_void, BPF_NOEXIST);
    assert!(ret == 0); check_map_id(MAP_FD[0], MAP_FD[5], port_key);
    let ret = bpf_map_update_elem(MAP_FD[6], &port_key as *const _ as *const c_void, &MAP_FD[1] as *const _ as *const c_void, BPF_NOEXIST);
    assert!(ret == 0); check_map_id(MAP_FD[1], MAP_FD[6], port_key);
}

unsafe fn test_map_in_map() {
    #[repr(C)]
    struct In6Addr { s6_addr16: [u16; 8] }
    #[repr(C)]
    struct SockaddrIn6 { sin6_family: u16, sin6_port: u16, sin6_flowinfo: u32, sin6_addr: In6Addr, sin6_scope_id: u32 }
    let mut in6 = SockaddrIn6 { sin6_family: AF_INET6, sin6_port: 0, sin6_flowinfo: 0, sin6_addr: In6Addr { s6_addr16: [0; 8] }, sin6_scope_id: 0 };
    let mut result_key: u32 = 0;
    let port_key = (rand() & 0x00ff) as u32;
    populate_map(port_key, 0xfaceb00c_u32 as c_int);
    in6.sin6_addr.s6_addr16[0] = 0xdead;
    in6.sin6_addr.s6_addr16[1] = 0xbeef;
    in6.sin6_port = port_key as u16;
    for i in 0..3 {
        printf(b"%s: \0".as_ptr() as *const c_char, TEST_NAMES[i].as_ptr());
        in6.sin6_addr.s6_addr16[7] = i as u16;
        let ret = connect(-1, &in6 as *const _ as *const c_void, size_of::<SockaddrIn6>() as u32);
        assert!(ret == -1 && EBADF == EBADF);
        let mut result = 0;
        let mut inline_result = 0;
        let ret = bpf_map_lookup_elem(MAP_FD[2], &result_key as *const _ as *const c_void, &mut result as *mut _ as *mut c_void);
        assert!(ret == 0);
        let ret = bpf_map_lookup_elem(MAP_FD[3], &result_key as *const _ as *const c_void, &mut inline_result as *mut _ as *mut c_void);
        assert!(ret == 0);
        if result != 0xfaceb00c_u32 as c_int || inline_result != 0xfaceb00c_u32 as c_int { process::exit(1); }
        bpf_map_delete_elem(MAP_FD[2], &mut result_key as *mut _ as *const c_void);
        bpf_map_delete_elem(MAP_FD[3], &mut result_key as *mut _ as *const c_void);
        printf(b"Pass\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn run_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut link: *mut bpf_link = std::ptr::null_mut();
    let mut filename = [0i8; 256];
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s.bpf.o\0".as_ptr() as *const c_char, *argv);
    let obj = bpf_object__open_file(filename.as_ptr(), std::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 { fprintf(stderr, b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char); return 0; }
    let prog = bpf_object__find_program_by_name(obj, b"trace_sys_connect\0".as_ptr() as *const c_char);
    if prog.is_null() { printf(b"finding a prog in obj file failed\n\0".as_ptr() as *const c_char); bpf_object__close(obj); return 0; }
    if bpf_object__load(obj) != 0 { fprintf(stderr, b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char); bpf_object__close(obj); return 0; }
    let names = [b"port_a\0", b"port_h\0", b"reg_result_h\0", b"inline_result_h\0", b"a_of_port_a\0", b"h_of_port_a\0", b"h_of_port_h\0"];
    for i in 0..7 { MAP_FD[i] = bpf_object__find_map_fd_by_name(obj, names[i].as_ptr() as *const c_char); }
    if MAP_FD.iter().any(|&fd| fd < 0) { fprintf(stderr, b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char); bpf_object__close(obj); return 0; }
    link = bpf_program__attach(prog);
    if libbpf_get_error(link as *const c_void) != 0 { fprintf(stderr, b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const c_char); link = std::ptr::null_mut(); }
    if !link.is_null() { test_map_in_map(); bpf_link__destroy(link); }
    bpf_object__close(obj);
    let _ = argc;
    0
}

fn main() { unsafe { std::process::exit(run_main(0, std::ptr::null_mut())); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
