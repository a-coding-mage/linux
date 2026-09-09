// SPDX-License-Identifier: GPL-2.0

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const BPF_PROG_TYPE_SOCKET_FILTER: c_uint = 1;
const SOL_SOCKET: c_int = 1;
const SO_ATTACH_BPF: c_int = 50;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_ICMP: c_int = 1;

unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const bpf_object_open_opts,
    ) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__next_program(
        object: *const bpf_object,
        previous: *const bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__set_type(program: *mut bpf_program, program_type: c_uint);
    fn bpf_object__load(object: *mut bpf_object) -> c_int;
    fn bpf_program__fd(program: *const bpf_program) -> c_int;
    fn bpf_object__find_map_fd_by_name(object: *const bpf_object, name: *const c_char) -> c_int;
    fn open_raw_sock(name: *const c_char) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn printf(format: *const c_char, ...);
    fn sleep(seconds: c_uint) -> c_uint;
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = ac;
    let mut obj: *mut bpf_object;
    let mut prog: *mut bpf_program;
    let mut map_fd: c_int;
    let mut prog_fd: c_int;
    let mut filename = [0 as c_char; 256];
    let mut i: c_int;
    let mut sock: c_int;
    let mut err: c_int;
    let mut f: *mut FILE;

    snprintf(
        filename.as_mut_ptr(),
        filename.len(),
        b"%s_kern.o\0".as_ptr() as *const c_char,
        *argv,
    );

    obj = bpf_object__open_file(filename.as_ptr(), std::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        return 1;
    }

    prog = bpf_object__next_program(obj, std::ptr::null());
    bpf_program__set_type(prog, BPF_PROG_TYPE_SOCKET_FILTER);

    err = bpf_object__load(obj);
    if err != 0 {
        return 1;
    }

    prog_fd = bpf_program__fd(prog);
    map_fd = bpf_object__find_map_fd_by_name(obj, b"my_map\0".as_ptr() as *const c_char);

    sock = open_raw_sock(b"lo\0".as_ptr() as *const c_char);

    assert!(
        setsockopt(
            sock,
            SOL_SOCKET,
            SO_ATTACH_BPF,
            &prog_fd as *const c_int as *const c_void,
            std::mem::size_of::<c_int>() as u32,
        ) == 0
    );

    f = popen(
        b"ping -4 -c5 localhost\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    let _ = f;

    for _ in 0..5 {
        let mut tcp_cnt: i64 = 0;
        let mut udp_cnt: i64 = 0;
        let mut icmp_cnt: i64 = 0;
        let mut key: c_int;

        key = IPPROTO_TCP;
        assert!(
            bpf_map_lookup_elem(
                map_fd,
                &key as *const c_int as *const c_void,
                &mut tcp_cnt as *mut i64 as *mut c_void,
            ) == 0
        );

        key = IPPROTO_UDP;
        assert!(
            bpf_map_lookup_elem(
                map_fd,
                &key as *const c_int as *const c_void,
                &mut udp_cnt as *mut i64 as *mut c_void,
            ) == 0
        );

        key = IPPROTO_ICMP;
        assert!(
            bpf_map_lookup_elem(
                map_fd,
                &key as *const c_int as *const c_void,
                &mut icmp_cnt as *mut i64 as *mut c_void,
            ) == 0
        );

        printf(
            b"TCP %lld UDP %lld ICMP %lld bytes\n\0".as_ptr() as *const c_char,
            tcp_cnt,
            udp_cnt,
            icmp_cnt,
        );
        sleep(1);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
