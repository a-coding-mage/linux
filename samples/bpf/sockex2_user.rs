// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the original C headers are referenced as external
// declarations below.

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct pair {
    pub packets: u64,
    pub bytes: u64,
}

pub const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;
pub const SOL_SOCKET: c_int = 1;
pub const SO_ATTACH_BPF: c_int = 50;

extern "C" {
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const c_void,
    ) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn bpf_object__next_program(
        obj: *mut bpf_object,
        prev: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: u32) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn open_raw_sock(name: *const c_char) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    fn bpf_map_get_next_key(
        fd: c_int,
        key: *const c_void,
        next_key: *mut c_void,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn inet_ntoa(addr: in_addr) -> *const c_char;
    fn htonl(hostlong: u32) -> u32;
    fn sleep(seconds: u32) -> u32;
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = ac;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut map_fd: c_int;
    let mut prog_fd: c_int;
    let mut filename = [0i8; 256];
    let mut i: c_int;
    let mut sock: c_int;
    let mut err: c_int;
    let mut f: *mut FILE;

    let suffix = CString::new("_kern.o").unwrap();
    let arg0 = CStr::from_ptr(*argv as *const c_char).to_bytes();
    let mut name = Vec::with_capacity(arg0.len() + suffix.as_bytes().len() + 1);
    name.extend_from_slice(arg0);
    name.extend_from_slice(suffix.as_bytes());
    name.push(0);
    let copy_len = name.len().min(filename.len());
    ptr::copy_nonoverlapping(name.as_ptr() as *const i8, filename.as_mut_ptr(), copy_len);

    obj = bpf_object__open_file(filename.as_ptr(), ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        return 1;
    }

    prog = bpf_object__next_program(obj, ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_SOCKET_FILTER);

    err = bpf_object__load(obj);
    if err != 0 {
        return 1;
    }

    prog_fd = bpf_program__fd(prog);
    let map_name = CString::new("hash_map").unwrap();
    map_fd = bpf_object__find_map_fd_by_name(obj, map_name.as_ptr());

    let lo = CString::new("lo").unwrap();
    sock = open_raw_sock(lo.as_ptr());

    assert!(setsockopt(
        sock,
        SOL_SOCKET,
        SO_ATTACH_BPF,
        &prog_fd as *const c_int as *const c_void,
        std::mem::size_of::<c_int>() as u32,
    ) == 0);

    let ping = CString::new("ping -4 -c5 localhost").unwrap();
    let read_mode = CString::new("r").unwrap();
    f = popen(ping.as_ptr(), read_mode.as_ptr());
    let _ = f;

    i = 0;
    while i < 5 {
        let mut key: c_int = 0;
        let mut next_key: c_int;
        let mut value: pair;

        while bpf_map_get_next_key(
            map_fd,
            &key as *const c_int as *const c_void,
            &mut next_key as *mut c_int as *mut c_void,
        ) == 0 {
            bpf_map_lookup_elem(
                map_fd,
                &next_key as *const c_int as *const c_void,
                &mut value as *mut pair as *mut c_void,
            );
            let addr = in_addr { s_addr: htonl(next_key as u32) };
            printf(
                b"ip %s bytes %lld packets %lld\n\0".as_ptr() as *const c_char,
                inet_ntoa(addr),
                value.bytes,
                value.packets,
            );
            key = next_key;
        }
        sleep(1);
        i += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
