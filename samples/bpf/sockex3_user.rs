// SPDX-License-Identifier: GPL-2.0
// C headers: <stdio.h>, <assert.h>, <bpf/bpf.h>, <bpf/libbpf.h>,
// "sock_example.h", <unistd.h>, and <arpa/inet.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct flow_key_record {
    pub src: u32,
    pub dst: u32,
    pub ports: ports_union,
    pub ip_proto: u32,
}

#[repr(C)]
pub union ports_union {
    pub ports: u32,
    pub port16: [u16; 2],
}

#[repr(C)]
pub struct pair {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_object__close(obj: *mut bpf_object);
    fn open_raw_sock(name: *const c_char) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: u32) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut c_void;
    fn sleep(seconds: u32) -> u32;
    fn printf(format: *const c_char, ...);
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn inet_ntoa(addr: in_addr) -> *const c_char;
    fn htonl(hostlong: u32) -> u32;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

const SOL_SOCKET: c_int = 1;
const SO_ATTACH_BPF: c_int = 50;

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut sock: c_int;
    let mut fd: c_int;
    let mut main_prog_fd: c_int;
    let mut hash_map_fd: c_int;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut filename = [0 as c_char; 256];
    let mut f: *mut c_void;

    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const c_char, *argv);

    obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        fprintf(core::ptr::null_mut(), b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char);
        return 0;
    }

    /* load BPF program */
    if bpf_object__load(obj) != 0 {
        fprintf(core::ptr::null_mut(), b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char);
        bpf_object__close(obj);
        return 0;
    }

    hash_map_fd = bpf_object__find_map_fd_by_name(obj, b"hash_map\0".as_ptr() as *const c_char);
    if hash_map_fd < 0 {
        fprintf(core::ptr::null_mut(), b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char);
        bpf_object__close(obj);
        return 0;
    }

    /* find BPF main program */
    main_prog_fd = 0;
    prog = core::ptr::null_mut();
    while !prog.is_null() {
        fd = bpf_program__fd(prog);
        if strcmp(bpf_program__name(prog), b"main_prog\0".as_ptr() as *const c_char) == 0 {
            main_prog_fd = fd;
        }
        break;
    }

    if main_prog_fd == 0 {
        fprintf(core::ptr::null_mut(), b"ERROR: can't find main_prog\n\0".as_ptr() as *const c_char);
        bpf_object__close(obj);
        return 0;
    }

    sock = open_raw_sock(b"lo\0".as_ptr() as *const c_char);

    /* attach BPF program to socket */
    assert!(setsockopt(sock, SOL_SOCKET, SO_ATTACH_BPF, &main_prog_fd as *const _ as *const c_void, core::mem::size_of::<u32>() as u32) == 0);

    if argc > 1 {
        f = popen(b"ping -4 -c5 localhost\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    } else {
        f = popen(b"netperf -l 4 localhost\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    }
    let _ = f;

    i = 0;
    while i < 5 {
        let mut key = flow_key_record { src: 0, dst: 0, ports: ports_union { ports: 0 }, ip_proto: 0 };
        let mut next_key: flow_key_record = core::mem::zeroed();
        let mut value: pair = core::mem::zeroed();

        sleep(1);
        printf(b"IP     src.port -> dst.port               bytes      packets\n\0".as_ptr() as *const c_char);
        while bpf_map_get_next_key(hash_map_fd, &key as *const _ as *const c_void, &mut next_key as *mut _ as *mut c_void) == 0 {
            bpf_map_lookup_elem(hash_map_fd, &next_key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
            printf(b"%s.%05d -> %s.%05d %12lld %12lld\n\0".as_ptr() as *const c_char,
                inet_ntoa(in_addr { s_addr: htonl(next_key.src) }),
                next_key.ports.port16[0],
                inet_ntoa(in_addr { s_addr: htonl(next_key.dst) }),
                next_key.ports.port16[1], value.bytes, value.packets);
            key = next_key;
        }
        i += 1;
    }

    bpf_object__close(obj);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
