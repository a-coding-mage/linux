// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external interfaces:
// errno.h, stdio.h, stdlib.h, string.h, unistd.h, arpa/inet.h,
// sys/socket.h, netdb.h, and ../../net/lib/ksft.h.

use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::mem::{size_of, zeroed};
use std::ptr;

const AF_UNSPEC: c_int = 0;
const SOCK_STREAM: c_int = 1;
const AI_PASSIVE: c_int = 0x0001;
const IPPROTO_TCP: c_int = 6;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_INCOMING_NAPI_ID: c_int = 56;
const EAFNOSUPPORT: c_int = 97;

type socklen_t = u32;
type ssize_t = isize;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct addrinfo {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: socklen_t,
    ai_addr: *mut sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut addrinfo,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn freeaddrinfo(res: *mut addrinfo);
    fn gai_strerror(errcode: c_int) -> *const c_char;

    static mut stderr: *mut c_void;

    fn ksft_ready();
    fn ksft_wait();
}

unsafe fn c_string(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn main() {
    unsafe {
        let argv: Vec<CString> = std::env::args().map(|arg| c_string(&arg)).collect();

        let mut address: sockaddr_storage = zeroed();
        let mut result: *mut addrinfo = ptr::null_mut();
        let mut hints: addrinfo = zeroed();
        let mut napi_id: c_uint = 0;
        let mut addr_len: socklen_t;
        let mut optlen: socklen_t;
        let mut buf = [0 as c_char; 1024];
        let opt: c_int = 1;
        let family: c_int;
        let server: c_int;
        let client: c_int;
        let mut ret: c_int;

        hints.ai_family = AF_UNSPEC;
        hints.ai_socktype = SOCK_STREAM;
        hints.ai_flags = AI_PASSIVE;

        ret = getaddrinfo(
            argv[1].as_ptr(),
            argv[2].as_ptr(),
            &hints,
            &mut result,
        );
        if ret != 0 {
            let fmt = CStr::from_bytes_with_nul_unchecked(b"getaddrinfo: %s\n\0");
            fprintf(stderr, fmt.as_ptr(), gai_strerror(ret));
            std::process::exit(1);
        }

        family = (*result).ai_family;
        addr_len = (*result).ai_addrlen;

        server = socket(family, SOCK_STREAM, IPPROTO_TCP);
        if server < 0 {
            perror(c_string("socket creation failed").as_ptr());
            freeaddrinfo(result);
            if errno == EAFNOSUPPORT {
                std::process::exit(-1);
            }
            std::process::exit(1);
        }

        if setsockopt(
            server,
            SOL_SOCKET,
            SO_REUSEADDR,
            &opt as *const c_int as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
        {
            perror(c_string("setsockopt").as_ptr());
            freeaddrinfo(result);
            std::process::exit(1);
        }

        memcpy(
            &mut address as *mut sockaddr_storage as *mut c_void,
            (*result).ai_addr as *const c_void,
            (*result).ai_addrlen as usize,
        );
        freeaddrinfo(result);

        if bind(server, &address as *const sockaddr_storage as *const sockaddr, addr_len) < 0 {
            perror(c_string("bind failed").as_ptr());
            std::process::exit(1);
        }

        if listen(server, 1) < 0 {
            perror(c_string("listen").as_ptr());
            std::process::exit(1);
        }

        ksft_ready();

        client = accept(server, ptr::null_mut(), ptr::null_mut());
        if client < 0 {
            perror(c_string("accept").as_ptr());
            std::process::exit(1);
        }

        optlen = size_of::<c_uint>() as socklen_t;
        ret = getsockopt(
            client,
            SOL_SOCKET,
            SO_INCOMING_NAPI_ID,
            &mut napi_id as *mut c_uint as *mut c_void,
            &mut optlen,
        );
        if ret != 0 {
            perror(c_string("getsockopt").as_ptr());
            std::process::exit(1);
        }

        read(client, buf.as_mut_ptr() as *mut c_void, 1024);

        ksft_wait();

        if napi_id == 0 {
            let fmt = CStr::from_bytes_with_nul_unchecked(b"napi ID is 0\n\0");
            fprintf(stderr, fmt.as_ptr());
            std::process::exit(1);
        }

        close(client);
        close(server);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
