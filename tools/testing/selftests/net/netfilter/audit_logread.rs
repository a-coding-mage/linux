// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type socklen_t = u32;
type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

static mut fd: c_int = 0;

const MAX_AUDIT_MESSAGE_LENGTH: usize = 8970;

const EINTR: c_int = 4;
const AF_NETLINK: c_int = 16;
const PF_NETLINK: c_int = AF_NETLINK;
const SOCK_RAW: c_int = 3;
const NETLINK_AUDIT: c_int = 9;
const SIGTERM: c_int = 15;
const SIGINT: c_int = 2;
const NLMSG_ERROR: u16 = 2;
const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_ACK: u16 = 0x04;
const AUDIT_SET: u16 = 1002;
const AUDIT_STATUS_ENABLED: u32 = 0x0001;
const AUDIT_STATUS_PID: u32 = 0x0004;
const AUDIT_NETFILTER_CFG: u16 = 1325;

const fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn NLMSG_SPACE(len: usize) -> u32 {
    (nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(len)) as u32
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
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
struct audit_status {
    mask: u32,
    enabled: u32,
    failure: u32,
    pid: u32,
    rate_limit: u32,
    backlog_limit: u32,
    lost: u32,
    backlog: u32,
    feature_bitmap: u32,
    backlog_wait_time: u32,
    backlog_wait_time_actual: u32,
}

#[repr(C)]
union audit_message_u {
    s: core::mem::ManuallyDrop<audit_status>,
    data: [c_char; MAX_AUDIT_MESSAGE_LENGTH],
}

#[repr(C)]
struct audit_message {
    nlh: nlmsghdr,
    u: audit_message_u,
}

#[repr(C)]
struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_mask: usize,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn recvfrom(
        socket: c_int,
        buffer: *mut c_void,
        length: size_t,
        flags: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> ssize_t;
    fn sendto(
        socket: c_int,
        message: *const c_void,
        length: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> ssize_t;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn perror(s: *const c_char);
    fn close(fd: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn exit(status: c_int) -> !;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;

    static mut stdout: *mut c_void;
}

unsafe fn audit_recv(fd: c_int, rep: *mut audit_message) -> c_int {
    let mut addr: sockaddr_nl = zeroed();
    let mut addrlen: socklen_t = size_of::<sockaddr_nl>() as socklen_t;
    let mut ret: c_int;

    loop {
        ret = recvfrom(
            fd,
            rep as *mut c_void,
            size_of::<audit_message>(),
            0,
            &mut addr as *mut sockaddr_nl as *mut sockaddr,
            &mut addrlen,
        ) as c_int;
        if !(ret < 0 && *__errno_location() == EINTR) {
            break;
        }
    }

    if ret < 0
        || addrlen != size_of::<sockaddr_nl>() as socklen_t
        || addr.nl_pid != 0
        || (*rep).nlh.nlmsg_type == NLMSG_ERROR
    {
        /* short-cut for now */
        return -1;
    }

    ret
}

unsafe fn audit_send(fd: c_int, type_: u16, key: u32, val: u32) -> c_int {
    static mut seq: c_int = 0;

    seq += 1;
    let msg = audit_message {
        nlh: nlmsghdr {
            nlmsg_len: NLMSG_SPACE(size_of::<audit_status>()),
            nlmsg_type: type_,
            nlmsg_flags: NLM_F_REQUEST | NLM_F_ACK,
            nlmsg_seq: seq as u32,
            nlmsg_pid: 0,
        },
        u: audit_message_u {
            s: core::mem::ManuallyDrop::new(audit_status {
                mask: key,
                enabled: if key == AUDIT_STATUS_ENABLED { val } else { 0 },
                failure: 0,
                pid: if key == AUDIT_STATUS_PID { val } else { 0 },
                rate_limit: 0,
                backlog_limit: 0,
                lost: 0,
                backlog: 0,
                feature_bitmap: 0,
                backlog_wait_time: 0,
                backlog_wait_time_actual: 0,
            }),
        },
    };
    let addr = sockaddr_nl {
        nl_family: AF_NETLINK as u16,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut ret: c_int;

    loop {
        ret = sendto(
            fd,
            &msg as *const audit_message as *const c_void,
            msg.nlh.nlmsg_len as size_t,
            0,
            &addr as *const sockaddr_nl as *const sockaddr,
            size_of::<sockaddr_nl>() as socklen_t,
        ) as c_int;
        if !(ret < 0 && *__errno_location() == EINTR) {
            break;
        }
    }

    if ret != msg.nlh.nlmsg_len as c_int {
        return -1;
    }
    0
}

unsafe fn audit_set(fd: c_int, key: u32, val: u32) -> c_int {
    let mut rep: audit_message = zeroed();
    let mut ret: c_int;

    ret = audit_send(fd, AUDIT_SET, key, val);
    if ret != 0 {
        return ret;
    }

    ret = audit_recv(fd, &mut rep);
    if ret < 0 {
        return ret;
    }
    0
}

unsafe fn readlog(fd: c_int) -> c_int {
    let mut rep: audit_message = zeroed();
    let ret = audit_recv(fd, &mut rep);
    let mut sep = c"".as_ptr();
    let mut k: *mut c_char;
    let mut v: *mut c_char;

    if ret < 0 {
        return ret;
    }

    if rep.nlh.nlmsg_type != AUDIT_NETFILTER_CFG {
        return 0;
    }

    /* skip the initial "audit(...): " part */
    strtok(rep.u.data.as_mut_ptr(), c" ".as_ptr());

    loop {
        k = strtok(ptr::null_mut(), c"=".as_ptr());
        if k.is_null() {
            break;
        }
        v = strtok(ptr::null_mut(), c" ".as_ptr());

        /* these vary and/or are uninteresting, ignore */
        if strcmp(k, c"pid".as_ptr()) == 0
            || strcmp(k, c"comm".as_ptr()) == 0
            || strcmp(k, c"subj".as_ptr()) == 0
        {
            continue;
        }

        /* strip the varying sequence number */
        if strcmp(k, c"table".as_ptr()) == 0 {
            *strchrnul(v, ':' as c_int) = '\0' as c_char;
        }

        printf(c"%s%s=%s".as_ptr(), sep, k, v);
        sep = c" ".as_ptr();
    }
    if *sep != 0 {
        printf(c"\n".as_ptr());
        fflush(stdout);
    }
    0
}

extern "C" fn cleanup(sig: c_int) {
    unsafe {
        audit_set(fd, AUDIT_STATUS_ENABLED, 0);
        close(fd);
        if sig != 0 {
            exit(0);
        }
    }
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let act = sigaction {
        sa_handler: cleanup,
        sa_mask: 0,
        sa_flags: 0,
        sa_restorer: ptr::null_mut(),
    };

    fd = socket(PF_NETLINK, SOCK_RAW, NETLINK_AUDIT);
    if fd < 0 {
        perror(c"Can't open netlink socket".as_ptr());
        return -1;
    }

    if sigaction(SIGTERM, &act, ptr::null_mut()) < 0 || sigaction(SIGINT, &act, ptr::null_mut()) < 0 {
        perror(c"Can't set signal handler".as_ptr());
        close(fd);
        return -1;
    }

    audit_set(fd, AUDIT_STATUS_ENABLED, 1);
    audit_set(fd, AUDIT_STATUS_PID, getpid() as u32);

    loop {
        readlog(fd);
    }
}

fn main() {
    unsafe {
        let argv: *mut *mut c_char = ptr::null_mut();
        c_main(0, argv);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
