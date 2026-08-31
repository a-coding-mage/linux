// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external Rust dependencies:
// errno.h, stdint.h, string.h, sys/socket.h, sys/time.h, unistd.h,
// linux/genetlink.h, and "netlink_helper.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct get_family_id_req {
    nlh: nlmsghdr,
    genl: genlmsghdr,
    buf: [c_char; 256],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn sendto(
        socket: c_int,
        message: *const c_void,
        length: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> ssize_t;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn nla_data(nla: *mut nlattr) -> *mut c_void;
    fn nla_ok(nla: *const nlattr, remaining: c_int) -> bool;
    fn nla_next(nla: *mut nlattr, remaining: *mut c_int) -> *mut nlattr;
}

unsafe extern "C" {
    fn NLMSG_LENGTH(len: c_int) -> u32;
    fn NLMSG_ALIGN(len: u32) -> u32;
    fn NLA_ALIGN(len: u16) -> u32;
    fn NLMSG_OK(nlh: *const nlmsghdr, len: c_int) -> bool;
    fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: *mut c_int) -> *mut nlmsghdr;
    fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void;
}

unsafe extern "C" {
    static ACCT_RCV_TIMEOUT_SEC: time_t;
    static AF_NETLINK: c_int;
    static SOCK_RAW: c_int;
    static NETLINK_GENERIC: c_int;
    static SOL_SOCKET: c_int;
    static SO_RCVTIMEO: c_int;
    static GENL_HDRLEN: c_int;
    static GENL_ID_CTRL: u16;
    static NLM_F_REQUEST: u16;
    static CTRL_CMD_GETFAMILY: u8;
    static CTRL_ATTR_FAMILY_NAME: u16;
    static CTRL_ATTR_FAMILY_ID: u16;
    static NLA_HDRLEN: u16;
    static NLMSG_ERROR: u16;
    static NLMSG_HDRLEN: u32;
    static ENOENT: c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netlink_open() -> c_int {
    let tv = timeval {
        tv_sec: ACCT_RCV_TIMEOUT_SEC,
        tv_usec: 0,
    };
    let addr = sockaddr_nl {
        nl_family: AF_NETLINK as sa_family_t,
        nl_pad: 0,
        nl_pid: getpid() as u32,
        nl_groups: 0,
    };
    let fd: c_int;

    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if fd < 0 {
        return -errno;
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const timeval as *const c_void,
        size_of::<timeval>() as socklen_t,
    ) < 0
    {
        let err = -errno;

        close(fd);
        return err;
    }

    if bind(
        fd,
        &addr as *const sockaddr_nl as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        let err = -errno;

        close(fd);
        return err;
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_request(fd: c_int, buf: *mut c_void, len: size_t) -> c_int {
    let addr = sockaddr_nl {
        nl_family: AF_NETLINK as sa_family_t,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };

    if sendto(
        fd,
        buf,
        len,
        0,
        &addr as *const sockaddr_nl as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        return -errno;
    }

    0
}

/*
 * Resolve the generic netlink family ID for @name.
 * Returns the family ID (>= 0) on success, negative errno on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_family_id(fd: c_int, name: *const c_char) -> c_int {
    let mut req: get_family_id_req = core::mem::zeroed();
    let mut resp: [c_char; 8192] = [0; 8192];
    let mut nlh: *mut nlmsghdr;
    let mut genl: *mut genlmsghdr;
    let mut na: *mut nlattr;
    let mut len: c_int;
    let mut rem: c_int;
    let ret: c_int;

    req.nlh.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN);
    req.nlh.nlmsg_type = GENL_ID_CTRL;
    req.nlh.nlmsg_flags = NLM_F_REQUEST;
    req.nlh.nlmsg_seq = 1;
    req.nlh.nlmsg_pid = getpid() as u32;

    req.genl.cmd = CTRL_CMD_GETFAMILY;
    req.genl.version = 1;

    na = (&mut req as *mut get_family_id_req as *mut c_char)
        .add(NLMSG_ALIGN(req.nlh.nlmsg_len) as usize) as *mut nlattr;
    (*na).nla_type = CTRL_ATTR_FAMILY_NAME;
    (*na).nla_len = (NLA_HDRLEN as size_t + strlen(name) + 1) as u16;
    memcpy(nla_data(na), name as *const c_void, strlen(name) + 1);
    req.nlh.nlmsg_len = NLMSG_ALIGN(req.nlh.nlmsg_len) + NLA_ALIGN((*na).nla_len);

    ret = send_request(
        fd,
        &mut req as *mut get_family_id_req as *mut c_void,
        req.nlh.nlmsg_len as size_t,
    );
    if ret != 0 {
        return ret;
    }

    len = recv(
        fd,
        resp.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 8192]>() as size_t,
        0,
    ) as c_int;
    if len < 0 {
        return -errno;
    }

    nlh = resp.as_mut_ptr() as *mut nlmsghdr;
    while NLMSG_OK(nlh, len) {
        if (*nlh).nlmsg_type == NLMSG_ERROR {
            let err: *mut nlmsgerr = NLMSG_DATA(nlh) as *mut nlmsgerr;

            return if (*err).error != 0 {
                (*err).error
            } else {
                -ENOENT
            };
        }

        genl = NLMSG_DATA(nlh) as *mut genlmsghdr;
        rem = (*nlh).nlmsg_len as c_int - NLMSG_HDRLEN as c_int - GENL_HDRLEN;
        na = (genl as *mut c_char).add(GENL_HDRLEN as usize) as *mut nlattr;
        while nla_ok(na, rem) {
            if (*na).nla_type == CTRL_ATTR_FAMILY_ID {
                return *(nla_data(na) as *mut u16) as c_int;
            }
            na = nla_next(na, &mut rem);
        }

        nlh = NLMSG_NEXT(nlh, &mut len);
    }

    -ENOENT
}
