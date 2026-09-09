// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook
 */
// C dependencies supplied by the surrounding build are intentionally kept as
// external declarations.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(string: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut errno: c_int;
    static mut stderr: *mut c_void;
    fn close(fd: c_int) -> c_int;

    fn bpf_obj_get(pathname: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
}

unsafe fn usage() {
    printf(b"Usage: tc_l2_ipip_redirect [...]\0".as_ptr() as *const c_char);
    printf(b"       -U <file>   Update an already pinned BPF array\n\0".as_ptr() as *const c_char);
    printf(b"       -i <ifindex> Interface index\n\0".as_ptr() as *const c_char);
    printf(b"       -h          Display this help\n\0".as_ptr() as *const c_char);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut pinned_file: *const c_char = core::ptr::null();
    let mut ifindex: c_int = -1;
    let mut array_key: c_int = 0;
    let mut array_fd: c_int = -1;
    let mut ret: c_int = -1;
    let mut opt: c_int;

    while {
        opt = getopt(argc, argv, b"F:U:i:\0".as_ptr() as *const c_char);
        opt != -1
    } {
        match opt {
            // General args
            85 => {
                pinned_file = optarg;
            }
            105 => {
                ifindex = atoi(optarg);
            }
            _ => {
                usage();
                if array_fd != -1 {
                    close(array_fd);
                }
                return ret;
            }
        }
    }

    if ifindex < 0 || pinned_file.is_null() {
        usage();
    } else {
        array_fd = bpf_obj_get(pinned_file);
        if array_fd < 0 {
            let format = b"bpf_obj_get(%s): %s(%d)\n\0";
            fprintf(
                stderr,
                format.as_ptr() as *const c_char,
                pinned_file,
                strerror(errno),
                errno,
            );
        } else {
            // bpf_tunnel_key.remote_ipv4 expects host byte orders
            ret = bpf_map_update_elem(
                array_fd,
                &array_key as *const c_int as *const c_void,
                &ifindex as *const c_int as *const c_void,
                0,
            );
            if ret != 0 {
                perror(b"bpf_map_update_elem\0".as_ptr() as *const c_char);
            }
        }
    }

    if array_fd != -1 {
        close(array_fd);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
