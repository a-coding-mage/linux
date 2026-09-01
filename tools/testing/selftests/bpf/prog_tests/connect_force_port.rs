// SPDX-License-Identifier: GPL-2.0

// Translated from connect_force_port.c. Original dependencies:
// <test_progs.h>, "cgroup_helpers.h", and "network_helpers.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u16 = u16;
type __u32 = u32;
type socklen_t = u32;
type size_t = usize;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __data: [u8; 126],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: __u16,
    sin_addr: [u8; 4],
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: __u16,
    sin6_flowinfo: __u32,
    sin6_addr: [u8; 16],
    sin6_scope_id: __u32,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    static AF_INET: c_int;
    static AF_INET6: c_int;
    static SOCK_STREAM: c_int;
    static SOCK_DGRAM: c_int;
    static BPF_CGROUP_INET4_CONNECT: c_int;
    static BPF_CGROUP_INET6_CONNECT: c_int;
    static BPF_CGROUP_INET4_GETPEERNAME: c_int;
    static BPF_CGROUP_INET6_GETPEERNAME: c_int;
    static BPF_CGROUP_INET4_GETSOCKNAME: c_int;
    static BPF_CGROUP_INET6_GETSOCKNAME: c_int;
    static EIO: c_int;

    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn ntohs(netshort: __u16) -> __u16;
    fn close(fd: c_int) -> c_int;

    fn log_err(fmt: *const c_char, ...) -> c_int;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: bool) -> bool;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__initial_value(map: *mut bpf_map, size: *mut size_t) -> *mut c_void;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_uint)
        -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn get_socket_local_port(fd: c_int) -> __u16;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
}

unsafe fn verify_ports(
    family: c_int,
    fd: c_int,
    expected_local: __u16,
    expected_peer: __u16,
) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let mut port: __u16;

    if getsockname(fd, &mut addr as *mut _ as *mut sockaddr, &mut len) != 0 {
        log_err(c"Failed to get server addr".as_ptr());
        return -1;
    }

    if family == AF_INET {
        port = (&addr as *const _ as *const sockaddr_in).as_ref().unwrap().sin_port;
    } else {
        port = (&addr as *const _ as *const sockaddr_in6).as_ref().unwrap().sin6_port;
    }

    if ntohs(port) != expected_local {
        log_err(
            c"Unexpected local port %d, expected %d".as_ptr(),
            ntohs(port) as c_int,
            expected_local as c_int,
        );
        return -1;
    }

    if getpeername(fd, &mut addr as *mut _ as *mut sockaddr, &mut len) != 0 {
        log_err(c"Failed to get peer addr".as_ptr());
        return -1;
    }

    if family == AF_INET {
        port = (&addr as *const _ as *const sockaddr_in).as_ref().unwrap().sin_port;
    } else {
        port = (&addr as *const _ as *const sockaddr_in6).as_ref().unwrap().sin6_port;
    }

    if ntohs(port) != expected_peer {
        log_err(
            c"Unexpected peer port %d, expected %d".as_ptr(),
            ntohs(port) as c_int,
            expected_peer as c_int,
        );
        return -1;
    }

    0
}

unsafe fn run_test(cgroup_fd: c_int, server_fd: c_int, family: c_int, type_: c_int) -> c_int {
    let v4: bool = family == AF_INET;
    let expected_local_port: __u16 = if v4 { 22222 } else { 22223 };
    let expected_peer_port: __u16 = 60000;
    let mut prog: *mut bpf_program;
    let mut map: *mut bpf_map;
    let mut port_ptr: *mut __u16;
    let mut port_size: size_t = 0;
    let obj_file: *const c_char = if v4 {
        c"connect_force_port4.bpf.o".as_ptr()
    } else {
        c"connect_force_port6.bpf.o".as_ptr()
    };
    let mut fd: c_int;
    let mut err: c_int;
    let _duration: __u32 = 0;
    let _type = type_;

    let obj: *mut bpf_object = bpf_object__open_file(obj_file, ptr::null());
    if !ASSERT_OK_PTR(obj as *const c_void, c"bpf_obj_open".as_ptr()) {
        return -1;
    }

    'close_bpf_object: {
        map = bpf_object__find_map_by_name(obj, c".bss".as_ptr());
        if !ASSERT_OK_PTR(map as *const c_void, c"find bss map".as_ptr()) {
            err = -EIO;
            break 'close_bpf_object;
        }

        port_ptr = bpf_map__initial_value(map, &mut port_size) as *mut __u16;
        if !ASSERT_OK_PTR(port_ptr as *const c_void, c"get bss initial value".as_ptr()) {
            err = -EIO;
            break 'close_bpf_object;
        }

        /* Auto assigns the port according to availability */
        *port_ptr = ntohs(get_socket_local_port(server_fd));

        err = bpf_object__load(obj);
        if !ASSERT_OK(err, c"bpf_obj_load".as_ptr()) {
            err = -EIO;
            break 'close_bpf_object;
        }

        prog = bpf_object__find_program_by_name(
            obj,
            if v4 {
                c"connect4".as_ptr()
            } else {
                c"connect6".as_ptr()
            },
        );
        if CHECK(
            prog.is_null(),
            c"find_prog".as_ptr(),
            c"connect prog not found\n".as_ptr(),
        ) {
            err = -EIO;
            break 'close_bpf_object;
        }

        err = bpf_prog_attach(
            bpf_program__fd(prog),
            cgroup_fd,
            if v4 {
                BPF_CGROUP_INET4_CONNECT
            } else {
                BPF_CGROUP_INET6_CONNECT
            },
            0,
        );
        if err != 0 {
            log_err(c"Failed to attach BPF program".as_ptr());
            break 'close_bpf_object;
        }

        prog = bpf_object__find_program_by_name(
            obj,
            if v4 {
                c"getpeername4".as_ptr()
            } else {
                c"getpeername6".as_ptr()
            },
        );
        if CHECK(
            prog.is_null(),
            c"find_prog".as_ptr(),
            c"getpeername prog not found\n".as_ptr(),
        ) {
            err = -EIO;
            break 'close_bpf_object;
        }

        err = bpf_prog_attach(
            bpf_program__fd(prog),
            cgroup_fd,
            if v4 {
                BPF_CGROUP_INET4_GETPEERNAME
            } else {
                BPF_CGROUP_INET6_GETPEERNAME
            },
            0,
        );
        if err != 0 {
            log_err(c"Failed to attach BPF program".as_ptr());
            break 'close_bpf_object;
        }

        prog = bpf_object__find_program_by_name(
            obj,
            if v4 {
                c"getsockname4".as_ptr()
            } else {
                c"getsockname6".as_ptr()
            },
        );
        if CHECK(
            prog.is_null(),
            c"find_prog".as_ptr(),
            c"getsockname prog not found\n".as_ptr(),
        ) {
            err = -EIO;
            break 'close_bpf_object;
        }

        err = bpf_prog_attach(
            bpf_program__fd(prog),
            cgroup_fd,
            if v4 {
                BPF_CGROUP_INET4_GETSOCKNAME
            } else {
                BPF_CGROUP_INET6_GETSOCKNAME
            },
            0,
        );
        if err != 0 {
            log_err(c"Failed to attach BPF program".as_ptr());
            break 'close_bpf_object;
        }

        fd = connect_to_fd(server_fd, 0);
        if fd < 0 {
            err = -1;
            break 'close_bpf_object;
        }

        err = verify_ports(family, fd, expected_local_port, expected_peer_port);
        close(fd);
    }

    bpf_object__close(obj);
    err
}

#[no_mangle]
pub unsafe extern "C" fn test_connect_force_port() {
    let mut server_fd: c_int;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/connect_force_port".as_ptr());
    if CHECK_FAIL(cgroup_fd < 0) {
        return;
    }

    'close_cgroup_fd: {
        server_fd = start_server(AF_INET, SOCK_STREAM, ptr::null(), 0, 0);
        if CHECK_FAIL(server_fd < 0) {
            break 'close_cgroup_fd;
        }
        CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET, SOCK_STREAM) != 0);
        close(server_fd);

        server_fd = start_server(AF_INET6, SOCK_STREAM, ptr::null(), 0, 0);
        if CHECK_FAIL(server_fd < 0) {
            break 'close_cgroup_fd;
        }
        CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET6, SOCK_STREAM) != 0);
        close(server_fd);

        server_fd = start_server(AF_INET, SOCK_DGRAM, ptr::null(), 0, 0);
        if CHECK_FAIL(server_fd < 0) {
            break 'close_cgroup_fd;
        }
        CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET, SOCK_DGRAM) != 0);
        close(server_fd);

        server_fd = start_server(AF_INET6, SOCK_DGRAM, ptr::null(), 0, 0);
        if CHECK_FAIL(server_fd < 0) {
            break 'close_cgroup_fd;
        }
        CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET6, SOCK_DGRAM) != 0);
        close(server_fd);
    }

    close(cgroup_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
