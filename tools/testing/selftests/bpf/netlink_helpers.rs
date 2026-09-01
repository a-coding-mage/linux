// SPDX-License-Identifier: GPL-2.0-or-later
/* Taken & modified from iproute2's libnetlink.c
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies from C headers and "netlink_helpers.h" are expected to be
// provided by the surrounding crate/bindings.
extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;

    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> c_int;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[allow(non_camel_case_types)]
type socklen_t = u32;
#[allow(non_camel_case_types)]
type time_t = i64;
#[allow(non_camel_case_types)]
type __u8 = u8;
#[allow(non_camel_case_types)]
type __u16 = u16;
#[allow(non_camel_case_types)]
type __u32 = u32;
#[allow(non_camel_case_types)]
type __u64 = u64;
#[allow(non_camel_case_types)]
type nl_ext_ack_fn_t = Option<unsafe extern "C" fn()>;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_nl {
    pub nl_family: u16,
    pub nl_pad: u16,
    pub nl_pid: u32,
    pub nl_groups: u32,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut c_void,
    pub msg_controllen: usize,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[repr(C)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

#[repr(C)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}

#[repr(C)]
pub struct rtnl_handle {
    pub fd: c_int,
    pub local: sockaddr_nl,
    pub peer: sockaddr_nl,
    pub seq: c_uint,
    pub dump: c_uint,
    pub proto: c_int,
}

const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOL_SOCKET: c_int = 1;
const SO_SNDBUF: c_int = 7;
const SO_RCVBUF: c_int = 8;
const SOL_NETLINK: c_int = 270;
const NETLINK_EXT_ACK: c_int = 11;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_SOCK_DIAG: c_int = 4;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const ENODATA: c_int = 61;
const ENOMEM: c_int = 12;
const MSG_PEEK: c_int = 2;
const MSG_TRUNC: c_int = 0x20;
const NLMSG_ERROR: u16 = 2;
const NLM_F_ACK: u16 = 4;

static mut rcvbuf: c_int = 1024 * 1024;

#[inline]
fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

#[inline]
fn rta_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

#[inline]
fn rta_length(len: c_int) -> c_int {
    (rta_align(size_of::<rtattr>()) + len as usize) as c_int
}

#[inline]
unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut c_char).add(nlmsg_align(size_of::<nlmsghdr>())) as *mut c_void
}

#[inline]
unsafe fn rta_data(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut c_char).add(rta_align(size_of::<rtattr>())) as *mut c_void
}

#[inline]
unsafe fn nlmsg_tail(nmsg: *mut nlmsghdr) -> *mut rtattr {
    (nmsg as *mut c_char).add(nlmsg_align((*nmsg).nlmsg_len as usize)) as *mut rtattr
}

#[no_mangle]
pub unsafe extern "C" fn rtnl_close(rth: *mut rtnl_handle) {
    if (*rth).fd >= 0 {
        close((*rth).fd);
        (*rth).fd = -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rtnl_open_byproto(
    rth: *mut rtnl_handle,
    subscriptions: c_uint,
    protocol: c_int,
) -> c_int {
    let mut addr_len: socklen_t;
    let sndbuf: c_int = 32768;
    let one: c_int = 1;

    memset(rth as *mut c_void, 0, size_of::<rtnl_handle>());
    (*rth).proto = protocol;
    (*rth).fd = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, protocol);
    if (*rth).fd < 0 {
        perror(c"Cannot open netlink socket".as_ptr());
        return -1;
    }
    if setsockopt(
        (*rth).fd,
        SOL_SOCKET,
        SO_SNDBUF,
        &sndbuf as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        perror(c"SO_SNDBUF".as_ptr());
        goto_err(rth);
        return -1;
    }
    if setsockopt(
        (*rth).fd,
        SOL_SOCKET,
        SO_RCVBUF,
        &raw const rcvbuf as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        perror(c"SO_RCVBUF".as_ptr());
        goto_err(rth);
        return -1;
    }

    /* Older kernels may no support extended ACK reporting */
    setsockopt(
        (*rth).fd,
        SOL_NETLINK,
        NETLINK_EXT_ACK,
        &one as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    );

    memset(
        &mut (*rth).local as *mut _ as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    (*rth).local.nl_family = AF_NETLINK as u16;
    (*rth).local.nl_groups = subscriptions;

    if bind(
        (*rth).fd,
        &(*rth).local as *const _ as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        perror(c"Cannot bind netlink socket".as_ptr());
        goto_err(rth);
        return -1;
    }
    addr_len = size_of::<sockaddr_nl>() as socklen_t;
    if getsockname(
        (*rth).fd,
        &mut (*rth).local as *mut _ as *mut sockaddr,
        &mut addr_len,
    ) < 0
    {
        perror(c"Cannot getsockname".as_ptr());
        goto_err(rth);
        return -1;
    }
    if addr_len as usize != size_of::<sockaddr_nl>() {
        fprintf(stderr, c"Wrong address length %d\n".as_ptr(), addr_len);
        goto_err(rth);
        return -1;
    }
    if (*rth).local.nl_family as c_int != AF_NETLINK {
        fprintf(
            stderr,
            c"Wrong address family %d\n".as_ptr(),
            (*rth).local.nl_family as c_int,
        );
        goto_err(rth);
        return -1;
    }
    (*rth).seq = time(ptr::null_mut()) as c_uint;
    0
}

unsafe fn goto_err(rth: *mut rtnl_handle) {
    rtnl_close(rth);
}

#[no_mangle]
pub unsafe extern "C" fn rtnl_open(rth: *mut rtnl_handle, subscriptions: c_uint) -> c_int {
    rtnl_open_byproto(rth, subscriptions, NETLINK_ROUTE)
}

unsafe fn __rtnl_recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> c_int {
    let mut len: c_int;

    'outer: loop {
        len = recvmsg(fd, msg, flags);
        if !(len < 0 && (errno == EINTR || errno == EAGAIN)) {
            break;
        }
    }
    if len < 0 {
        fprintf(
            stderr,
            c"netlink receive error %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
        return -errno;
    }
    if len == 0 {
        fprintf(stderr, c"EOF on netlink\n".as_ptr());
        return -ENODATA;
    }
    len
}

unsafe fn rtnl_recvmsg(fd: c_int, msg: *mut msghdr, answer: *mut *mut c_char) -> c_int {
    let iov: *mut iovec = (*msg).msg_iov;
    let mut buf: *mut c_char;
    let mut len: c_int;

    (*iov).iov_base = ptr::null_mut();
    (*iov).iov_len = 0;

    len = __rtnl_recvmsg(fd, msg, MSG_PEEK | MSG_TRUNC);
    if len < 0 {
        return len;
    }
    if len < 32768 {
        len = 32768;
    }
    buf = malloc(len as usize) as *mut c_char;
    if buf.is_null() {
        fprintf(stderr, c"malloc error: not enough buffer\n".as_ptr());
        return -ENOMEM;
    }
    (*iov).iov_base = buf as *mut c_void;
    (*iov).iov_len = len as usize;
    len = __rtnl_recvmsg(fd, msg, 0);
    if len < 0 {
        free(buf as *mut c_void);
        return len;
    }
    if !answer.is_null() {
        *answer = buf;
    } else {
        free(buf as *mut c_void);
    }
    len
}

unsafe fn rtnl_talk_error(_h: *mut nlmsghdr, err: *mut nlmsgerr, _errfn: nl_ext_ack_fn_t) {
    fprintf(
        stderr,
        c"RTNETLINK answers: %s\n".as_ptr(),
        strerror(-(*err).error),
    );
}

unsafe fn __rtnl_talk_iov(
    rtnl: *mut rtnl_handle,
    iov: *mut iovec,
    iovlen: usize,
    answer: *mut *mut nlmsghdr,
    show_rtnl_err: bool,
    errfn: nl_ext_ack_fn_t,
) -> c_int {
    let mut nladdr = sockaddr_nl {
        nl_family: AF_NETLINK as u16,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut riov = iovec {
        iov_base: ptr::null_mut(),
        iov_len: 0,
    };
    let mut msg = msghdr {
        msg_name: &mut nladdr as *mut _ as *mut c_void,
        msg_namelen: size_of::<sockaddr_nl>() as socklen_t,
        msg_iov: iov,
        msg_iovlen: iovlen,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut seq: c_uint = 0;
    let mut h: *mut nlmsghdr;
    let mut i: c_int;
    let mut status: c_int;
    let mut buf: *mut c_char;

    i = 0;
    while (i as usize) < iovlen {
        h = (*iov.add(i as usize)).iov_base as *mut nlmsghdr;
        (*rtnl).seq = (*rtnl).seq.wrapping_add(1);
        seq = (*rtnl).seq;
        (*h).nlmsg_seq = seq;
        if answer.is_null() {
            (*h).nlmsg_flags |= NLM_F_ACK;
        }
        i += 1;
    }
    status = sendmsg((*rtnl).fd, &msg, 0);
    if status < 0 {
        perror(c"Cannot talk to rtnetlink".as_ptr());
        return -1;
    }
    /* change msg to use the response iov */
    msg.msg_iov = &mut riov;
    msg.msg_iovlen = 1;
    i = 0;
    loop {
        status = rtnl_recvmsg((*rtnl).fd, &mut msg, &mut buf);
        i += 1;
        if status < 0 {
            return status;
        }
        if msg.msg_namelen as usize != size_of::<sockaddr_nl>() {
            fprintf(
                stderr,
                c"Sender address length == %d!\n".as_ptr(),
                msg.msg_namelen,
            );
            exit(1);
        }
        h = buf as *mut nlmsghdr;
        while status as usize >= size_of::<nlmsghdr>() {
            let len: c_int = (*h).nlmsg_len as c_int;
            let l: c_int = len - size_of::<nlmsghdr>() as c_int;

            if l < 0 || len > status {
                if (msg.msg_flags & MSG_TRUNC) != 0 {
                    fprintf(stderr, c"Truncated message!\n".as_ptr());
                    free(buf as *mut c_void);
                    return -1;
                }
                fprintf(stderr, c"Malformed message: len=%d!\n".as_ptr(), len);
                exit(1);
            }
            if nladdr.nl_pid != 0
                || (*h).nlmsg_pid != (*rtnl).local.nl_pid
                || (*h).nlmsg_seq > seq
                || (*h).nlmsg_seq < seq.wrapping_sub(iovlen as c_uint)
            {
                /* Don't forget to skip that message. */
                status -= nlmsg_align(len as usize) as c_int;
                h = (h as *mut c_char).add(nlmsg_align(len as usize)) as *mut nlmsghdr;
                continue;
            }
            if (*h).nlmsg_type == NLMSG_ERROR {
                let err: *mut nlmsgerr = nlmsg_data(h) as *mut nlmsgerr;
                let error: c_int = (*err).error;

                if l < size_of::<nlmsgerr>() as c_int {
                    fprintf(stderr, c"ERROR truncated\n".as_ptr());
                    free(buf as *mut c_void);
                    return -1;
                }
                if error != 0 {
                    errno = -error;
                    if (*rtnl).proto != NETLINK_SOCK_DIAG && show_rtnl_err {
                        rtnl_talk_error(h, err, errfn);
                    }
                }
                if (i as usize) < iovlen {
                    free(buf as *mut c_void);
                    continue 'outer;
                }
                if error != 0 {
                    free(buf as *mut c_void);
                    return -i;
                }
                if !answer.is_null() {
                    *answer = buf as *mut nlmsghdr;
                } else {
                    free(buf as *mut c_void);
                }
                return 0;
            }
            if !answer.is_null() {
                *answer = buf as *mut nlmsghdr;
                return 0;
            }
            fprintf(stderr, c"Unexpected reply!\n".as_ptr());
            status -= nlmsg_align(len as usize) as c_int;
            h = (h as *mut c_char).add(nlmsg_align(len as usize)) as *mut nlmsghdr;
        }
        free(buf as *mut c_void);
        if (msg.msg_flags & MSG_TRUNC) != 0 {
            fprintf(stderr, c"Message truncated!\n".as_ptr());
            continue;
        }
        if status != 0 {
            fprintf(stderr, c"Remnant of size %d!\n".as_ptr(), status);
            exit(1);
        }
    }
}

unsafe fn __rtnl_talk(
    rtnl: *mut rtnl_handle,
    n: *mut nlmsghdr,
    answer: *mut *mut nlmsghdr,
    show_rtnl_err: bool,
    errfn: nl_ext_ack_fn_t,
) -> c_int {
    let mut iov = iovec {
        iov_base: n as *mut c_void,
        iov_len: (*n).nlmsg_len as usize,
    };

    __rtnl_talk_iov(rtnl, &mut iov, 1, answer, show_rtnl_err, errfn)
}

#[no_mangle]
pub unsafe extern "C" fn rtnl_talk(
    rtnl: *mut rtnl_handle,
    n: *mut nlmsghdr,
    answer: *mut *mut nlmsghdr,
) -> c_int {
    __rtnl_talk(rtnl, n, answer, true, None)
}

#[no_mangle]
pub unsafe extern "C" fn addattr(n: *mut nlmsghdr, maxlen: c_int, type_: c_int) -> c_int {
    addattr_l(n, maxlen, type_, ptr::null(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn addattr8(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    data: __u8,
) -> c_int {
    addattr_l(
        n,
        maxlen,
        type_,
        &data as *const _ as *const c_void,
        size_of::<__u8>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn addattr16(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    data: __u16,
) -> c_int {
    addattr_l(
        n,
        maxlen,
        type_,
        &data as *const _ as *const c_void,
        size_of::<__u16>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn addattr32(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    data: __u32,
) -> c_int {
    addattr_l(
        n,
        maxlen,
        type_,
        &data as *const _ as *const c_void,
        size_of::<__u32>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn addattr64(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    data: __u64,
) -> c_int {
    addattr_l(
        n,
        maxlen,
        type_,
        &data as *const _ as *const c_void,
        size_of::<__u64>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn addattrstrz(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    str_: *const c_char,
) -> c_int {
    addattr_l(n, maxlen, type_, str_ as *const c_void, strlen(str_) as c_int + 1)
}

#[no_mangle]
pub unsafe extern "C" fn addattr_l(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
    data: *const c_void,
    alen: c_int,
) -> c_int {
    let len: c_int = rta_length(alen);
    let rta: *mut rtattr;

    if nlmsg_align((*n).nlmsg_len as usize) + rta_align(len as usize) > maxlen as usize {
        fprintf(
            stderr,
            c"%s: Message exceeded bound of %d\n".as_ptr(),
            c"addattr_l".as_ptr(),
            maxlen,
        );
        return -1;
    }
    rta = nlmsg_tail(n);
    (*rta).rta_type = type_ as u16;
    (*rta).rta_len = len as u16;
    if alen != 0 {
        memcpy(rta_data(rta), data, alen as usize);
    }
    (*n).nlmsg_len =
        (nlmsg_align((*n).nlmsg_len as usize) + rta_align(len as usize)) as u32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn addraw_l(
    n: *mut nlmsghdr,
    maxlen: c_int,
    data: *const c_void,
    len: c_int,
) -> c_int {
    if nlmsg_align((*n).nlmsg_len as usize) + nlmsg_align(len as usize) > maxlen as usize {
        fprintf(
            stderr,
            c"%s: Message exceeded bound of %d\n".as_ptr(),
            c"addraw_l".as_ptr(),
            maxlen,
        );
        return -1;
    }

    memcpy(nlmsg_tail(n) as *mut c_void, data, len as usize);
    memset(
        (nlmsg_tail(n) as *mut c_char).add(len as usize) as *mut c_void,
        0,
        nlmsg_align(len as usize) - len as usize,
    );
    (*n).nlmsg_len =
        (nlmsg_align((*n).nlmsg_len as usize) + nlmsg_align(len as usize)) as u32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn addattr_nest(
    n: *mut nlmsghdr,
    maxlen: c_int,
    type_: c_int,
) -> *mut rtattr {
    let nest: *mut rtattr = nlmsg_tail(n);

    addattr_l(n, maxlen, type_, ptr::null(), 0);
    nest
}

#[no_mangle]
pub unsafe extern "C" fn addattr_nest_end(n: *mut nlmsghdr, nest: *mut rtattr) -> c_int {
    (*nest).rta_len = (nlmsg_tail(n) as *mut c_void)
        .cast::<c_char>()
        .offset_from((nest as *mut c_void).cast::<c_char>()) as u16;
    (*n).nlmsg_len as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
