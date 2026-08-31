// SPDX-License-Identifier: GPL-2.0-only
/*
 * vsock_diag_test - vsock_diag.ko test suite
 *
 * Copyright (C) 2017 Red Hat, Inc.
 *
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type __u8 = u8;
type __u32 = u32;

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

const SOCK_DGRAM: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const SOCK_SEQPACKET: c_int = 5;

const AF_NETLINK: c_int = 16;
const AF_VSOCK: c_int = 40;
const NETLINK_SOCK_DIAG: c_int = 4;

const SOCK_DIAG_BY_FAMILY: u16 = 20;
const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_DUMP: u16 = 0x300;
const NLMSG_DONE: u16 = 0x3;
const NLMSG_ERROR: u16 = 0x2;

const TCP_CLOSE: c_int = 7;
const TCP_SYN_SENT: c_int = 2;
const TCP_ESTABLISHED: c_int = 1;
const TCP_CLOSING: c_int = 11;
const TCP_LISTEN: c_int = 10;

const VMADDR_CID_ANY: c_uint = 0xffff_ffff;
const VMADDR_PORT_ANY: c_uint = 0xffff_ffff;
const DEFAULT_PEER_PORT: c_uint = 1234;

const EINTR: c_int = 4;

const TEST_MODE_UNSET: c_int = 0;
const TEST_MODE_CLIENT: c_int = 1;
const TEST_MODE_SERVER: c_int = 2;

const required_argument: c_int = 1;
const no_argument: c_int = 0;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct vsock_diag_msg {
    vdiag_family: __u8,
    vdiag_type: __u8,
    vdiag_state: __u8,
    vdiag_shutdown: __u8,
    vdiag_src_cid: __u32,
    vdiag_src_port: __u32,
    vdiag_dst_cid: __u32,
    vdiag_dst_port: __u32,
    vdiag_ino: __u32,
    vdiag_cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct vsock_diag_req {
    sdiag_family: __u8,
    sdiag_protocol: __u8,
    pad: __u16,
    vdiag_states: __u32,
    vdiag_ino: __u32,
    vdiag_show: __u32,
    vdiag_cookie: [__u32; 2],
}

type __u16 = u16;

#[repr(C)]
struct vsock_stat {
    list: list_head,
    msg: vsock_diag_msg,
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: __u32,
    nlmsg_type: __u16,
    nlmsg_flags: __u16,
    nlmsg_seq: __u32,
    nlmsg_pid: __u32,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: u16,
    nl_pad: u16,
    nl_pid: u32,
    nl_groups: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_vm {
    svm_family: u16,
    svm_reserved1: u16,
    svm_port: c_uint,
    svm_cid: c_uint,
    svm_zero: [u8; 4],
}

#[repr(C)]
union sockaddr_union {
    sa: core::mem::ManuallyDrop<sockaddr>,
    svm: core::mem::ManuallyDrop<sockaddr_vm>,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    _rest: [u8; 128],
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct test_opts {
    mode: c_int,
    peer_cid: c_uint,
    peer_port: c_uint,
}

#[repr(C)]
struct test_case {
    name: *const c_char,
    run_client: Option<unsafe extern "C" fn(*const test_opts)>,
    run_server: Option<unsafe extern "C" fn(*const test_opts)>,
    skip: bool,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;

    fn init_signals();
    fn control_expectln(str_: *const c_char);
    fn control_writeln(str_: *const c_char);
    fn control_init(host: *const c_char, port: *const c_char, server: bool);
    fn control_cleanup();
    fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_stream_accept(cid: c_uint, port: c_uint, clientaddrp: *mut sockaddr_vm) -> c_int;
    fn parse_cid(str_: *const c_char) -> c_uint;
    fn parse_port(str_: *const c_char) -> c_uint;
    fn list_tests(test_cases: *mut test_case);
    fn skip_test(test_cases: *mut test_case, test_cases_len: size_t, test_id_str: *const c_char);
    fn run_tests(test_cases: *mut test_case, opts: *const test_opts);
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

unsafe fn container_of_vsock_stat_list(ptr: *mut list_head) -> *mut vsock_stat {
    ptr.cast::<u8>()
        .wrapping_sub(offset_of!(vsock_stat, list))
        .cast::<vsock_stat>()
}

const fn NLMSG_ALIGN(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn NLMSG_LENGTH(len: usize) -> usize {
    len + NLMSG_ALIGN(size_of::<nlmsghdr>())
}

unsafe fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut c_void {
    unsafe {
        (nlh.cast::<u8>() as *mut u8)
            .add(NLMSG_LENGTH(0))
            .cast::<c_void>()
    }
}

unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: ssize_t) -> bool {
    unsafe {
        len >= size_of::<nlmsghdr>() as ssize_t
            && (*nlh).nlmsg_len >= size_of::<nlmsghdr>() as __u32
            && ((*nlh).nlmsg_len as ssize_t) <= len
    }
}

unsafe fn NLMSG_NEXT(nlh: *const nlmsghdr, len: &mut ssize_t) -> *const nlmsghdr {
    unsafe {
        let aligned = NLMSG_ALIGN((*nlh).nlmsg_len as usize) as ssize_t;
        *len -= aligned;
        nlh.cast::<u8>().add(aligned as usize).cast::<nlmsghdr>()
    }
}

unsafe fn sock_type_str(type_: c_int) -> *const c_char {
    match type_ {
        SOCK_DGRAM => c"DGRAM".as_ptr(),
        SOCK_STREAM => c"STREAM".as_ptr(),
        SOCK_SEQPACKET => c"SEQPACKET".as_ptr(),
        _ => c"INVALID TYPE".as_ptr(),
    }
}

unsafe fn sock_state_str(state: c_int) -> *const c_char {
    match state {
        TCP_CLOSE => c"UNCONNECTED".as_ptr(),
        TCP_SYN_SENT => c"CONNECTING".as_ptr(),
        TCP_ESTABLISHED => c"CONNECTED".as_ptr(),
        TCP_CLOSING => c"DISCONNECTING".as_ptr(),
        TCP_LISTEN => c"LISTEN".as_ptr(),
        _ => c"INVALID STATE".as_ptr(),
    }
}

unsafe fn sock_shutdown_str(shutdown: c_int) -> *const c_char {
    match shutdown {
        1 => c"RCV_SHUTDOWN".as_ptr(),
        2 => c"SEND_SHUTDOWN".as_ptr(),
        3 => c"RCV_SHUTDOWN | SEND_SHUTDOWN".as_ptr(),
        _ => c"0".as_ptr(),
    }
}

unsafe fn print_vsock_addr(fp: *mut FILE, cid: c_uint, port: c_uint) {
    unsafe {
        if cid == VMADDR_CID_ANY {
            fprintf(fp, c"*:".as_ptr());
        } else {
            fprintf(fp, c"%u:".as_ptr(), cid);
        }

        if port == VMADDR_PORT_ANY {
            fprintf(fp, c"*".as_ptr());
        } else {
            fprintf(fp, c"%u".as_ptr(), port);
        }
    }
}

unsafe fn print_vsock_stat(fp: *mut FILE, st: *mut vsock_stat) {
    unsafe {
        print_vsock_addr(fp, (*st).msg.vdiag_src_cid, (*st).msg.vdiag_src_port);
        fprintf(fp, c" ".as_ptr());
        print_vsock_addr(fp, (*st).msg.vdiag_dst_cid, (*st).msg.vdiag_dst_port);
        fprintf(
            fp,
            c" %s %s %s %u\n".as_ptr(),
            sock_type_str((*st).msg.vdiag_type as c_int),
            sock_state_str((*st).msg.vdiag_state as c_int),
            sock_shutdown_str((*st).msg.vdiag_shutdown as c_int),
            (*st).msg.vdiag_ino,
        );
    }
}

unsafe fn print_vsock_stats(fp: *mut FILE, head: *mut list_head) {
    unsafe {
        let mut node = (*head).next;
        while node != head {
            let st = container_of_vsock_stat_list(node);
            print_vsock_stat(fp, st);
            node = (*node).next;
        }
    }
}

unsafe fn find_vsock_stat(head: *mut list_head, fd: c_int) -> *mut vsock_stat {
    unsafe {
        let mut statbuf: stat = core::mem::zeroed();

        if fstat(fd, &mut statbuf) < 0 {
            perror(c"fstat".as_ptr());
            exit(EXIT_FAILURE);
        }

        let mut node = (*head).next;
        while node != head {
            let st = container_of_vsock_stat_list(node);
            if (*st).msg.vdiag_ino as u64 == statbuf.st_ino {
                return st;
            }
            node = (*node).next;
        }

        fprintf(stderr, c"cannot find fd %d\n".as_ptr(), fd);
        exit(EXIT_FAILURE);
    }
}

unsafe fn check_no_sockets(head: *mut list_head) {
    unsafe {
        if !list_empty(head) {
            fprintf(stderr, c"expected no sockets\n".as_ptr());
            print_vsock_stats(stderr, head);
            exit(1);
        }
    }
}

unsafe fn check_num_sockets(head: *mut list_head, expected: c_int) {
    unsafe {
        let mut node: *mut list_head;
        let mut n = 0;

        node = (*head).next;
        while node != head {
            n += 1;
            node = (*node).next;
        }

        if n != expected {
            fprintf(
                stderr,
                c"expected %d sockets, found %d\n".as_ptr(),
                expected,
                n,
            );
            print_vsock_stats(stderr, head);
            exit(EXIT_FAILURE);
        }
    }
}

unsafe fn check_socket_state(st: *mut vsock_stat, state: __u8) {
    unsafe {
        if (*st).msg.vdiag_state != state {
            fprintf(
                stderr,
                c"expected socket state %#x, got %#x\n".as_ptr(),
                state as c_int,
                (*st).msg.vdiag_state as c_int,
            );
            exit(EXIT_FAILURE);
        }
    }
}

#[repr(C)]
struct req {
    nlh: nlmsghdr,
    vreq: vsock_diag_req,
}

unsafe fn send_req(fd: c_int) {
    unsafe {
        let mut nladdr = sockaddr_nl {
            nl_family: AF_NETLINK as u16,
            nl_pad: 0,
            nl_pid: 0,
            nl_groups: 0,
        };
        let mut req = req {
            nlh: nlmsghdr {
                nlmsg_len: size_of::<req>() as __u32,
                nlmsg_type: SOCK_DIAG_BY_FAMILY,
                nlmsg_flags: NLM_F_REQUEST | NLM_F_DUMP,
                nlmsg_seq: 0,
                nlmsg_pid: 0,
            },
            vreq: vsock_diag_req {
                sdiag_family: AF_VSOCK as __u8,
                sdiag_protocol: 0,
                pad: 0,
                vdiag_states: !0 as __u32,
                vdiag_ino: 0,
                vdiag_show: 0,
                vdiag_cookie: [0; 2],
            },
        };
        let mut iov = iovec {
            iov_base: (&mut req as *mut req).cast::<c_void>(),
            iov_len: size_of::<req>(),
        };
        let mut msg = msghdr {
            msg_name: (&mut nladdr as *mut sockaddr_nl).cast::<c_void>(),
            msg_namelen: size_of::<sockaddr_nl>() as socklen_t,
            msg_iov: &mut iov,
            msg_iovlen: 1,
            msg_control: ptr::null_mut(),
            msg_controllen: 0,
            msg_flags: 0,
        };

        loop {
            if sendmsg(fd, &msg, 0) < 0 {
                if errno == EINTR {
                    continue;
                }

                perror(c"sendmsg".as_ptr());
                exit(EXIT_FAILURE);
            }

            return;
        }
    }
}

unsafe fn recv_resp(fd: c_int, buf: *mut c_void, len: size_t) -> ssize_t {
    unsafe {
        let mut nladdr = sockaddr_nl {
            nl_family: AF_NETLINK as u16,
            nl_pad: 0,
            nl_pid: 0,
            nl_groups: 0,
        };
        let mut iov = iovec {
            iov_base: buf,
            iov_len: len,
        };
        let mut msg = msghdr {
            msg_name: (&mut nladdr as *mut sockaddr_nl).cast::<c_void>(),
            msg_namelen: size_of::<sockaddr_nl>() as socklen_t,
            msg_iov: &mut iov,
            msg_iovlen: 1,
            msg_control: ptr::null_mut(),
            msg_controllen: 0,
            msg_flags: 0,
        };
        let mut ret: ssize_t;

        loop {
            ret = recvmsg(fd, &mut msg, 0);
            if !(ret < 0 && errno == EINTR) {
                break;
            }
        }

        if ret < 0 {
            perror(c"recvmsg".as_ptr());
            exit(EXIT_FAILURE);
        }

        ret
    }
}

unsafe fn add_vsock_stat(sockets: *mut list_head, resp: *const vsock_diag_msg) {
    unsafe {
        let st = malloc(size_of::<vsock_stat>()) as *mut vsock_stat;
        if st.is_null() {
            perror(c"malloc".as_ptr());
            exit(EXIT_FAILURE);
        }

        (*st).msg = *resp;
        list_add_tail(&mut (*st).list, sockets);
    }
}

/*
 * Read vsock stats into a list.
 */
unsafe fn read_vsock_stat(sockets: *mut list_head) {
    unsafe {
        let mut buf: [c_long; 8192 / size_of::<c_long>()] = [0; 8192 / size_of::<c_long>()];
        let fd: c_int;

        fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
        if fd < 0 {
            perror(c"socket".as_ptr());
            exit(EXIT_FAILURE);
        }

        send_req(fd);

        loop {
            let mut h: *const nlmsghdr;
            let mut ret: ssize_t;

            ret = recv_resp(fd, buf.as_mut_ptr().cast::<c_void>(), size_of_val(&buf));
            if ret == 0 {
                break;
            }
            if ret < size_of::<nlmsghdr>() as ssize_t {
                fprintf(stderr, c"short read of %zd bytes\n".as_ptr(), ret);
                exit(EXIT_FAILURE);
            }

            h = buf.as_ptr().cast::<nlmsghdr>();

            while NLMSG_OK(h, ret) {
                if (*h).nlmsg_type == NLMSG_DONE {
                    close(fd);
                    return;
                }

                if (*h).nlmsg_type == NLMSG_ERROR {
                    let err = NLMSG_DATA(h).cast::<nlmsgerr>();

                    if (*h).nlmsg_len < NLMSG_LENGTH(size_of::<nlmsgerr>()) as __u32 {
                        fprintf(stderr, c"NLMSG_ERROR\n".as_ptr());
                    } else {
                        errno = -(*err).error;
                        perror(c"NLMSG_ERROR".as_ptr());
                    }

                    exit(EXIT_FAILURE);
                }

                if (*h).nlmsg_type != SOCK_DIAG_BY_FAMILY {
                    fprintf(
                        stderr,
                        c"unexpected nlmsg_type %#x\n".as_ptr(),
                        (*h).nlmsg_type as c_int,
                    );
                    exit(EXIT_FAILURE);
                }
                if (*h).nlmsg_len < NLMSG_LENGTH(size_of::<vsock_diag_msg>()) as __u32 {
                    fprintf(stderr, c"short vsock_diag_msg\n".as_ptr());
                    exit(EXIT_FAILURE);
                }

                add_vsock_stat(sockets, NLMSG_DATA(h).cast::<vsock_diag_msg>());

                h = NLMSG_NEXT(h, &mut ret);
            }
        }

        close(fd);
    }
}

unsafe fn size_of_val<T>(val: &T) -> usize {
    size_of::<T>()
}

unsafe fn free_sock_stat(sockets: *mut list_head) {
    unsafe {
        let mut node = (*sockets).next;
        while node != sockets {
            let next = (*node).next;
            let st = container_of_vsock_stat_list(node);
            free(st.cast::<c_void>());
            node = next;
        }
    }
}

unsafe extern "C" fn test_no_sockets(_opts: *const test_opts) {
    unsafe {
        let mut sockets = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        INIT_LIST_HEAD(&mut sockets);

        read_vsock_stat(&mut sockets);

        check_no_sockets(&mut sockets);
    }
}

unsafe extern "C" fn test_listen_socket_server(opts: *const test_opts) {
    unsafe {
        let mut addr = sockaddr_union {
            svm: core::mem::ManuallyDrop::new(sockaddr_vm {
                svm_family: AF_VSOCK as u16,
                svm_reserved1: 0,
                svm_port: (*opts).peer_port,
                svm_cid: VMADDR_CID_ANY,
                svm_zero: [0; 4],
            }),
        };
        let mut sockets = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        INIT_LIST_HEAD(&mut sockets);
        let mut st: *mut vsock_stat;
        let fd: c_int;

        fd = socket(AF_VSOCK, SOCK_STREAM, 0);

        if bind(
            fd,
            (&mut addr.sa as *mut core::mem::ManuallyDrop<sockaddr>).cast::<sockaddr>(),
            size_of::<sockaddr_vm>() as socklen_t,
        ) < 0
        {
            perror(c"bind".as_ptr());
            exit(EXIT_FAILURE);
        }

        if listen(fd, 1) < 0 {
            perror(c"listen".as_ptr());
            exit(EXIT_FAILURE);
        }

        read_vsock_stat(&mut sockets);

        check_num_sockets(&mut sockets, 1);
        st = find_vsock_stat(&mut sockets, fd);
        check_socket_state(st, TCP_LISTEN as __u8);

        close(fd);
        free_sock_stat(&mut sockets);
    }
}

unsafe extern "C" fn test_connect_client(opts: *const test_opts) {
    unsafe {
        let fd: c_int;
        let mut sockets = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        INIT_LIST_HEAD(&mut sockets);
        let mut st: *mut vsock_stat;

        fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
        if fd < 0 {
            perror(c"connect".as_ptr());
            exit(EXIT_FAILURE);
        }

        read_vsock_stat(&mut sockets);

        check_num_sockets(&mut sockets, 1);
        st = find_vsock_stat(&mut sockets, fd);
        check_socket_state(st, TCP_ESTABLISHED as __u8);

        control_expectln(c"DONE".as_ptr());
        control_writeln(c"DONE".as_ptr());

        close(fd);
        free_sock_stat(&mut sockets);
    }
}

unsafe extern "C" fn test_connect_server(opts: *const test_opts) {
    unsafe {
        let mut st: *mut vsock_stat;
        let mut sockets = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        INIT_LIST_HEAD(&mut sockets);
        let client_fd: c_int;

        client_fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
        if client_fd < 0 {
            perror(c"accept".as_ptr());
            exit(EXIT_FAILURE);
        }

        read_vsock_stat(&mut sockets);

        check_num_sockets(&mut sockets, 1);
        st = find_vsock_stat(&mut sockets, client_fd);
        check_socket_state(st, TCP_ESTABLISHED as __u8);

        control_writeln(c"DONE".as_ptr());
        control_expectln(c"DONE".as_ptr());

        close(client_fd);
        free_sock_stat(&mut sockets);
    }
}

static mut test_cases: [test_case; 4] = [
    test_case {
        name: c"No sockets".as_ptr(),
        run_client: None,
        run_server: Some(test_no_sockets),
        skip: false,
    },
    test_case {
        name: c"Listen socket".as_ptr(),
        run_client: None,
        run_server: Some(test_listen_socket_server),
        skip: false,
    },
    test_case {
        name: c"Connect".as_ptr(),
        run_client: Some(test_connect_client),
        run_server: Some(test_connect_server),
        skip: false,
    },
    test_case {
        name: ptr::null(),
        run_client: None,
        run_server: None,
        skip: false,
    },
];

static optstring: &[u8] = b"\0";
static longopts: [option; 8] = [
    option {
        name: c"control-host".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'H' as c_int,
    },
    option {
        name: c"control-port".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'P' as c_int,
    },
    option {
        name: c"mode".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'm' as c_int,
    },
    option {
        name: c"peer-cid".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'p' as c_int,
    },
    option {
        name: c"peer-port".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'q' as c_int,
    },
    option {
        name: c"list".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 'l' as c_int,
    },
    option {
        name: c"skip".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 's' as c_int,
    },
    option {
        name: c"help".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: '?' as c_int,
    },
];

unsafe fn usage() {
    unsafe {
        fprintf(
            stderr,
            c"Usage: vsock_diag_test [--help] [--control-host=<host>] --control-port=<port> --mode=client|server --peer-cid=<cid> [--peer-port=<port>] [--list] [--skip=<test_id>]\n\n  Server: vsock_diag_test --control-port=1234 --mode=server --peer-cid=3\n  Client: vsock_diag_test --control-host=192.168.0.1 --control-port=1234 --mode=client --peer-cid=2\n\nRun vsock_diag.ko tests.  Must be launched in both\nguest and host.  One side must use --mode=client and\nthe other side must use --mode=server.\n\nA TCP control socket connection is used to coordinate tests\nbetween the client and the server.  The server requires a\nlisten address and the client requires an address to\nconnect to.\n\nThe CID of the other side must be given with --peer-cid=<cid>.\n\nOptions:\n  --help                 This help message\n  --control-host <host>  Server IP address to connect to\n  --control-port <port>  Server port to listen on/connect to\n  --mode client|server   Server or client mode\n  --peer-cid <cid>       CID of the other side\n  --peer-port <port>     AF_VSOCK port used for the test [default: %d]\n  --list                 List of tests that will be executed\n  --skip <test_id>       Test ID to skip;\n                         use multiple --skip options to skip more tests\n".as_ptr(),
            DEFAULT_PEER_PORT,
        );
        exit(EXIT_FAILURE);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut control_host: *const c_char = ptr::null();
        let mut control_port: *const c_char = ptr::null();
        let mut opts = test_opts {
            mode: TEST_MODE_UNSET,
            peer_cid: VMADDR_CID_ANY,
            peer_port: DEFAULT_PEER_PORT,
        };

        init_signals();

        loop {
            let opt = getopt_long(
                argc,
                argv,
                optstring.as_ptr().cast::<c_char>(),
                longopts.as_ptr(),
                ptr::null_mut(),
            );

            if opt == -1 {
                break;
            }

            match opt {
                x if x == 'H' as c_int => {
                    control_host = optarg;
                }
                x if x == 'm' as c_int => {
                    if strcmp(optarg, c"client".as_ptr()) == 0 {
                        opts.mode = TEST_MODE_CLIENT;
                    } else if strcmp(optarg, c"server".as_ptr()) == 0 {
                        opts.mode = TEST_MODE_SERVER;
                    } else {
                        fprintf(stderr, c"--mode must be \"client\" or \"server\"\n".as_ptr());
                        return EXIT_FAILURE;
                    }
                }
                x if x == 'p' as c_int => {
                    opts.peer_cid = parse_cid(optarg);
                }
                x if x == 'q' as c_int => {
                    opts.peer_port = parse_port(optarg);
                }
                x if x == 'P' as c_int => {
                    control_port = optarg;
                }
                x if x == 'l' as c_int => {
                    list_tests(&raw mut test_cases as *mut test_case);
                }
                x if x == 's' as c_int => {
                    skip_test(
                        &raw mut test_cases as *mut test_case,
                        (size_of::<[test_case; 4]>() / size_of::<test_case>()) - 1,
                        optarg,
                    );
                }
                x if x == '?' as c_int => {
                    usage();
                }
                _ => {
                    usage();
                }
            }
        }

        if control_port.is_null() {
            usage();
        }
        if opts.mode == TEST_MODE_UNSET {
            usage();
        }
        if opts.peer_cid == VMADDR_CID_ANY {
            usage();
        }

        if control_host.is_null() {
            if opts.mode != TEST_MODE_SERVER {
                usage();
            }
            control_host = c"0.0.0.0".as_ptr();
        }

        control_init(control_host, control_port, opts.mode == TEST_MODE_SERVER);

        run_tests(&raw mut test_cases as *mut test_case, &opts);

        control_cleanup();
        EXIT_SUCCESS
    }
}
