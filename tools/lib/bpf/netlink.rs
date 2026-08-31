// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2018 Facebook */

/* Translated from lib/bpf/netlink.c.  C include dependencies are expected to be
 * provided by the surrounding translated crate/bindings.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;
type time_t = i64;

const SOL_NETLINK: c_int = 270;

const NL_CONT: c_int = 0;
const NL_NEXT: c_int = 1;
const NL_DONE: c_int = 2;

type libbpf_dump_nlmsg_t =
    Option<unsafe extern "C" fn(cookie: *mut c_void, msg: *mut c_void, tb: *mut *mut nlattr) -> c_int>;
type __dump_nlmsg_t = Option<
    unsafe extern "C" fn(
        nlmsg: *mut nlmsghdr,
        cb: libbpf_dump_nlmsg_t,
        cookie: *mut c_void,
    ) -> c_int,
>;
type qdisc_config_t =
    Option<unsafe extern "C" fn(req: *mut libbpf_nla_req, hook: *const bpf_tc_hook) -> c_int>;

#[repr(C)]
struct sockaddr {
    sa_family: __u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: __u16,
    nl_pad: __u16,
    nl_pid: __u32,
    nl_groups: __u32,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: c_int,
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
struct nlattr {
    nla_len: __u16,
    nla_type: __u16,
}

#[repr(C)]
struct genlmsghdr {
    cmd: __u8,
    version: __u8,
    reserved: __u16,
}

#[repr(C)]
struct ifinfomsg {
    ifi_family: __u8,
    __ifi_pad: __u8,
    ifi_type: __u16,
    ifi_index: c_int,
    ifi_flags: c_uint,
    ifi_change: c_uint,
}

#[repr(C)]
struct tcmsg {
    tcm_family: __u8,
    tcm__pad1: __u8,
    tcm__pad2: __u16,
    tcm_ifindex: c_int,
    tcm_handle: __u32,
    tcm_parent: __u32,
    tcm_info: __u32,
}

#[repr(C)]
struct bpf_prog_info {
    id: __u32,
    _pad0: [u8; 4],
    name: [c_char; 16],
}

#[repr(C)]
struct bpf_xdp_attach_opts {
    sz: usize,
    old_prog_fd: c_int,
}

#[repr(C)]
struct bpf_xdp_query_opts {
    sz: usize,
    prog_id: __u32,
    drv_prog_id: __u32,
    hw_prog_id: __u32,
    skb_prog_id: __u32,
    attach_mode: __u8,
    _pad0: [u8; 3],
    feature_flags: __u64,
    xdp_zc_max_segs: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum bpf_tc_attach_point {
    BPF_TC_INGRESS = 1,
    BPF_TC_EGRESS = 2,
    BPF_TC_CUSTOM = 4,
    BPF_TC_QDISC = 8,
}

#[repr(C)]
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_int,
    parent: __u32,
    handle: __u32,
    qdisc: *const c_char,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
    flags: __u32,
    prog_id: __u32,
    handle: __u32,
    priority: __u32,
}

#[repr(C)]
struct libbpf_nla_req {
    nh: nlmsghdr,
    gnl: genlmsghdr,
    ifinfo: ifinfomsg,
    tc: tcmsg,
    buf: [u8; 4096],
}

#[repr(C)]
struct xdp_link_info {
    prog_id: __u32,
    drv_prog_id: __u32,
    hw_prog_id: __u32,
    skb_prog_id: __u32,
    attach_mode: __u8,
}

#[repr(C)]
struct xdp_id_md {
    ifindex: c_int,
    flags: __u32,
    info: xdp_link_info,
    feature_flags: __u64,
}

#[repr(C)]
struct xdp_features_md {
    ifindex: c_int,
    xdp_zc_max_segs: __u32,
    flags: __u64,
}

#[repr(C)]
struct bpf_cb_ctx {
    opts: *mut bpf_tc_opts,
    processed: bool,
}

unsafe extern "C" {
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn time(tloc: *mut time_t) -> time_t;
    fn htons(hostshort: __u16) -> __u16;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn pr_warn(format: *const c_char, ...);
    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_nla_dump_errormsg(nh: *mut nlmsghdr);
    fn libbpf_nla_parse(tb: *mut *mut nlattr, maxtype: c_int, head: *mut nlattr, len: c_int, policy: *const c_void) -> c_int;
    fn libbpf_nla_parse_nested(tb: *mut *mut nlattr, maxtype: c_int, nla: *mut nlattr, policy: *const c_void) -> c_int;
    fn libbpf_nla_getattr_u8(nla: *mut nlattr) -> __u8;
    fn libbpf_nla_getattr_u16(nla: *mut nlattr) -> __u16;
    fn libbpf_nla_getattr_u32(nla: *mut nlattr) -> __u32;
    fn libbpf_nla_getattr_u64(nla: *mut nlattr) -> __u64;
    fn nlattr_add(req: *mut libbpf_nla_req, ty: c_int, data: *const c_void, len: usize) -> c_int;
    fn nlattr_begin_nested(req: *mut libbpf_nla_req, ty: c_int) -> *mut nlattr;
    fn nlattr_end_nested(req: *mut libbpf_nla_req, nla: *mut nlattr);
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

const AF_NETLINK: c_int = 16;
const AF_UNSPEC: c_int = 0;
const AF_PACKET: c_int = 17;
const SOCK_RAW: c_int = 3;
const SOCK_CLOEXEC: c_int = 0o2000000;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_GENERIC: c_int = 16;
const NETLINK_EXT_ACK: c_int = 11;
const MSG_PEEK: c_int = 2;
const MSG_TRUNC: c_int = 0x20;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EMSGSIZE: c_int = 90;
const EOPNOTSUPP: c_int = 95;
const ENAMETOOLONG: c_int = 36;
const UINT16_MAX: __u32 = 65535;

const LIBBPF_ERRNO__INTERNAL: c_int = 4000;
const LIBBPF_ERRNO__WRNGPID: c_int = 4001;
const LIBBPF_ERRNO__INVSEQ: c_int = 4002;
const LIBBPF_ERRNO__NLPARSE: c_int = 4003;

const NLM_F_REQUEST: __u16 = 0x01;
const NLM_F_MULTI: __u16 = 0x02;
const NLM_F_ACK: __u16 = 0x04;
const NLM_F_ECHO: __u16 = 0x08;
const NLM_F_EXCL: __u16 = 0x200;
const NLM_F_CREATE: __u16 = 0x400;
const NLM_F_REPLACE: __u16 = 0x100;
const NLM_F_DUMP: __u16 = 0x300;
const NLMSG_ERROR: __u16 = 0x2;
const NLMSG_DONE: __u16 = 0x3;
const RTM_SETLINK: __u16 = 19;
const RTM_GETLINK: __u16 = 18;
const RTM_NEWQDISC: __u16 = 36;
const RTM_DELQDISC: __u16 = 37;
const RTM_NEWTFILTER: __u16 = 44;
const RTM_DELTFILTER: __u16 = 45;
const RTM_GETTFILTER: __u16 = 46;
const GENL_ID_CTRL: __u16 = 0x10;
const CTRL_CMD_GETFAMILY: __u8 = 3;
const CTRL_ATTR_FAMILY_ID: c_int = 1;
const CTRL_ATTR_FAMILY_NAME: c_int = 2;
const GENL_HDRLEN: usize = 4;

const IFLA_MAX: c_int = 64;
const IFLA_XDP: c_int = 43;
const IFLA_XDP_MAX: c_int = 8;
const IFLA_XDP_FD: c_int = 1;
const IFLA_XDP_ATTACHED: c_int = 2;
const IFLA_XDP_FLAGS: c_int = 3;
const IFLA_XDP_PROG_ID: c_int = 4;
const IFLA_XDP_DRV_PROG_ID: c_int = 5;
const IFLA_XDP_SKB_PROG_ID: c_int = 6;
const IFLA_XDP_HW_PROG_ID: c_int = 7;
const IFLA_XDP_EXPECTED_FD: c_int = 8;
const XDP_ATTACHED_NONE: __u8 = 0;
const XDP_ATTACHED_MULTI: __u8 = 4;
const XDP_FLAGS_UPDATE_IF_NOEXIST: __u32 = 1;
const XDP_FLAGS_SKB_MODE: __u32 = 2;
const XDP_FLAGS_DRV_MODE: __u32 = 4;
const XDP_FLAGS_HW_MODE: __u32 = 8;
const XDP_FLAGS_REPLACE: __u32 = 16;
const XDP_FLAGS_MODES: __u32 = XDP_FLAGS_SKB_MODE | XDP_FLAGS_DRV_MODE | XDP_FLAGS_HW_MODE;
const XDP_FLAGS_MASK: __u32 = XDP_FLAGS_UPDATE_IF_NOEXIST | XDP_FLAGS_MODES | XDP_FLAGS_REPLACE;

const NETDEV_CMD_DEV_GET: __u8 = 1;
const NETDEV_CMD_MAX: c_int = 16;
const NETDEV_A_DEV_IFINDEX: c_int = 1;
const NETDEV_A_DEV_XDP_FEATURES: c_int = 2;
const NETDEV_A_DEV_XDP_ZC_MAX_SEGS: c_int = 3;

const ETH_P_ALL: __u16 = 0x0003;
const TC_H_CLSACT: __u32 = 0xfffffff1;
const TC_H_ROOT: __u32 = 0xffffffff;
const TC_H_MIN_INGRESS: __u32 = 0xfff2;
const TC_H_MIN_EGRESS: __u32 = 0xfff3;
const TCA_KIND: c_int = 1;
const TCA_OPTIONS: c_int = 2;
const TCA_MAX: c_int = 32;
const TCA_BPF_MAX: c_int = 16;
const TCA_BPF_FD: c_int = 1;
const TCA_BPF_NAME: c_int = 2;
const TCA_BPF_FLAGS: c_int = 3;
const TCA_BPF_ID: c_int = 7;
const TCA_BPF_FLAG_ACT_DIRECT: __u32 = 1;
const BPF_TC_F_REPLACE: __u32 = 1;

fn nlmsg_align(len: usize) -> usize {
    (len + 3) & !3
}

unsafe fn nlmsg_length(len: usize) -> __u32 {
    (len + nlmsg_align(size_of::<nlmsghdr>())) as __u32
}

unsafe fn nlmsg_data(nh: *mut nlmsghdr) -> *mut c_void {
    (nh as *mut u8).add(nlmsg_align(size_of::<nlmsghdr>())) as *mut c_void
}

unsafe fn nlmsg_payload(nh: *mut nlmsghdr, len: usize) -> c_int {
    ((*nh).nlmsg_len as usize - nlmsg_length(len) as usize) as c_int
}

unsafe fn nlmsg_ok(nh: *mut nlmsghdr, len: c_int) -> bool {
    len >= size_of::<nlmsghdr>() as c_int
        && (*nh).nlmsg_len >= size_of::<nlmsghdr>() as __u32
        && (*nh).nlmsg_len as c_int <= len
}

unsafe fn nlmsg_next(nh: *mut nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

fn tc_h_make(maj: __u32, min: __u32) -> __u32 {
    maj | min
}

fn tc_h_maj(h: __u32) -> __u32 {
    h & 0xffff0000
}

unsafe fn opts_valid<T>(opts: *const T) -> bool {
    !opts.is_null()
}

unsafe fn opts_has<T>(opts: *const T, _field_offset: usize) -> bool {
    !opts.is_null()
}

unsafe extern "C" fn libbpf_netlink_open(nl_pid: *mut __u32, proto: c_int) -> c_int {
    let mut sa: sockaddr_nl = zeroed();
    let mut addrlen: socklen_t;
    let one: c_int = 1;
    let mut ret: c_int;

    sa.nl_family = AF_NETLINK as __u16;

    let sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, proto);
    if sock < 0 {
        return -errno();
    }

    if setsockopt(
        sock,
        SOL_NETLINK,
        NETLINK_EXT_ACK,
        &one as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        pr_warn(c"Netlink error reporting not supported\n".as_ptr());
    }

    if bind(
        sock,
        &sa as *const _ as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        ret = -errno();
        close(sock);
        return ret;
    }

    addrlen = size_of::<sockaddr_nl>() as socklen_t;
    if getsockname(sock, &mut sa as *mut _ as *mut sockaddr, &mut addrlen) < 0 {
        ret = -errno();
        close(sock);
        return ret;
    }

    if addrlen != size_of::<sockaddr_nl>() as socklen_t {
        ret = -LIBBPF_ERRNO__INTERNAL;
        close(sock);
        return ret;
    }

    *nl_pid = sa.nl_pid;
    sock
}

unsafe extern "C" fn libbpf_netlink_close(sock: c_int) {
    close(sock);
}

unsafe extern "C" fn netlink_recvmsg(sock: c_int, mhdr: *mut msghdr, flags: c_int) -> c_int {
    let mut len: c_int;
    loop {
        len = recvmsg(sock, mhdr, flags);
        if !(len < 0 && (errno() == EINTR || errno() == EAGAIN)) {
            break;
        }
    }
    if len < 0 {
        return -errno();
    }
    len
}

unsafe extern "C" fn alloc_iov(iov: *mut iovec, len: c_int) -> c_int {
    let nbuf = realloc((*iov).iov_base, len as usize);
    if nbuf.is_null() {
        return -ENOMEM;
    }

    (*iov).iov_base = nbuf;
    (*iov).iov_len = len as usize;
    0
}

unsafe extern "C" fn libbpf_netlink_recv(
    sock: c_int,
    nl_pid: __u32,
    seq: c_int,
    _fn: __dump_nlmsg_t,
    func: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut iov: iovec = zeroed();
    let mut mhdr: msghdr = zeroed();
    let mut multipart = true;
    let mut ret: c_int;

    mhdr.msg_iov = &mut iov;
    mhdr.msg_iovlen = 1;

    ret = alloc_iov(&mut iov, 8192);
    if ret != 0 {
        free(iov.iov_base);
        return ret;
    }

    'outer: while multipart {
        multipart = false;
        let mut len = netlink_recvmsg(sock, &mut mhdr, MSG_PEEK | MSG_TRUNC);
        if len < 0 {
            ret = len;
            break;
        }

        if len as usize > iov.iov_len {
            ret = alloc_iov(&mut iov, len);
            if ret != 0 {
                break;
            }
        }

        len = netlink_recvmsg(sock, &mut mhdr, 0);
        if len < 0 {
            ret = len;
            break;
        }

        if len == 0 {
            ret = 0;
            break;
        }

        let mut nh = iov.iov_base as *mut nlmsghdr;
        while nlmsg_ok(nh, len) {
            if (*nh).nlmsg_pid != nl_pid {
                ret = -LIBBPF_ERRNO__WRNGPID;
                break 'outer;
            }
            if (*nh).nlmsg_seq != seq as __u32 {
                ret = -LIBBPF_ERRNO__INVSEQ;
                break 'outer;
            }
            if ((*nh).nlmsg_flags & NLM_F_MULTI) != 0 {
                multipart = true;
            }
            match (*nh).nlmsg_type {
                NLMSG_ERROR => {
                    let err = nlmsg_data(nh) as *mut nlmsgerr;
                    if (*err).error == 0 {
                        let mut tmp_len = len;
                        nh = nlmsg_next(nh, &mut tmp_len);
                        len = tmp_len;
                        continue;
                    }
                    ret = (*err).error;
                    libbpf_nla_dump_errormsg(nh);
                    break 'outer;
                }
                NLMSG_DONE => {
                    ret = 0;
                    break 'outer;
                }
                _ => {}
            }
            if let Some(cb) = _fn {
                ret = cb(nh, func, cookie);
                match ret {
                    NL_CONT => {}
                    NL_NEXT => continue 'outer,
                    NL_DONE => {
                        ret = 0;
                        break 'outer;
                    }
                    _ => break 'outer,
                }
            }
            nh = nlmsg_next(nh, &mut len);
        }
        if len != 0 {
            pr_warn(
                c"Invalid message or trailing data in Netlink response: %d bytes left\n".as_ptr(),
                len,
            );
        }
        ret = 0;
    }

    free(iov.iov_base);
    ret
}

unsafe extern "C" fn libbpf_netlink_send_recv(
    req: *mut libbpf_nla_req,
    proto: c_int,
    parse_msg: __dump_nlmsg_t,
    parse_attr: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut nl_pid: __u32 = 0;
    let mut ret: c_int;
    let sock = libbpf_netlink_open(&mut nl_pid, proto);
    if sock < 0 {
        return sock;
    }

    (*req).nh.nlmsg_pid = 0;
    (*req).nh.nlmsg_seq = time(ptr::null_mut()) as __u32;

    if send(sock, req as *const c_void, (*req).nh.nlmsg_len as usize, 0) < 0 {
        ret = -errno();
    } else {
        ret = libbpf_netlink_recv(sock, nl_pid, (*req).nh.nlmsg_seq as c_int, parse_msg, parse_attr, cookie);
    }
    libbpf_netlink_close(sock);
    ret
}

unsafe extern "C" fn parse_genl_family_id(
    nh: *mut nlmsghdr,
    _fn: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let gnl = nlmsg_data(nh) as *mut genlmsghdr;
    let na = (gnl as *mut u8).add(GENL_HDRLEN) as *mut nlattr;
    let mut tb: [*mut nlattr; CTRL_ATTR_FAMILY_ID as usize + 1] =
        [ptr::null_mut(); CTRL_ATTR_FAMILY_ID as usize + 1];
    let id = cookie as *mut __u16;

    libbpf_nla_parse(
        tb.as_mut_ptr(),
        CTRL_ATTR_FAMILY_ID,
        na,
        nlmsg_payload(nh, size_of::<genlmsghdr>()),
        ptr::null(),
    );
    if tb[CTRL_ATTR_FAMILY_ID as usize].is_null() {
        return NL_CONT;
    }

    *id = libbpf_nla_getattr_u16(tb[CTRL_ATTR_FAMILY_ID as usize]);
    NL_DONE
}

unsafe extern "C" fn libbpf_netlink_resolve_genl_family_id(
    name: *const c_char,
    len: __u16,
    id: *mut __u16,
) -> c_int {
    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(GENL_HDRLEN);
    req.nh.nlmsg_type = GENL_ID_CTRL;
    req.nh.nlmsg_flags = NLM_F_REQUEST;
    req.gnl.cmd = CTRL_CMD_GETFAMILY;
    req.gnl.version = 2;

    let err = nlattr_add(&mut req, CTRL_ATTR_FAMILY_NAME, name as *const c_void, len as usize);
    if err < 0 {
        return err;
    }

    libbpf_netlink_send_recv(
        &mut req,
        NETLINK_GENERIC,
        Some(parse_genl_family_id),
        None,
        id as *mut c_void,
    )
}

unsafe extern "C" fn __bpf_set_link_xdp_fd_replace(
    ifindex: c_int,
    fd: c_int,
    old_fd: c_int,
    flags: __u32,
) -> c_int {
    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<ifinfomsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    req.nh.nlmsg_type = RTM_SETLINK;
    req.ifinfo.ifi_family = AF_UNSPEC as __u8;
    req.ifinfo.ifi_index = ifindex;

    let nla = nlattr_begin_nested(&mut req, IFLA_XDP);
    if nla.is_null() {
        return -EMSGSIZE;
    }
    let mut ret = nlattr_add(&mut req, IFLA_XDP_FD, &fd as *const _ as *const c_void, size_of::<c_int>());
    if ret < 0 {
        return ret;
    }
    if flags != 0 {
        ret = nlattr_add(&mut req, IFLA_XDP_FLAGS, &flags as *const _ as *const c_void, size_of::<__u32>());
        if ret < 0 {
            return ret;
        }
    }
    if (flags & XDP_FLAGS_REPLACE) != 0 {
        ret = nlattr_add(
            &mut req,
            IFLA_XDP_EXPECTED_FD,
            &old_fd as *const _ as *const c_void,
            size_of::<c_int>(),
        );
        if ret < 0 {
            return ret;
        }
    }
    nlattr_end_nested(&mut req, nla);

    libbpf_netlink_send_recv(&mut req, NETLINK_ROUTE, None, None, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn bpf_xdp_attach(
    ifindex: c_int,
    prog_fd: c_int,
    mut flags: __u32,
    opts: *const bpf_xdp_attach_opts,
) -> c_int {
    if !opts_valid(opts) {
        return libbpf_err(-EINVAL);
    }

    let mut old_prog_fd = if opts.is_null() { 0 } else { (*opts).old_prog_fd };
    if old_prog_fd != 0 {
        flags |= XDP_FLAGS_REPLACE;
    } else {
        old_prog_fd = -1;
    }

    let err = __bpf_set_link_xdp_fd_replace(ifindex, prog_fd, old_prog_fd, flags);
    libbpf_err(err)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_xdp_detach(
    ifindex: c_int,
    flags: __u32,
    opts: *const bpf_xdp_attach_opts,
) -> c_int {
    bpf_xdp_attach(ifindex, -1, flags, opts)
}

unsafe extern "C" fn __dump_link_nlmsg(
    nlh: *mut nlmsghdr,
    dump_link_nlmsg: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut tb: [*mut nlattr; IFLA_MAX as usize + 1] = [ptr::null_mut(); IFLA_MAX as usize + 1];
    let ifi = nlmsg_data(nlh) as *mut ifinfomsg;
    let len = (*nlh).nlmsg_len as c_int - nlmsg_length(size_of::<ifinfomsg>()) as c_int;
    let attr = (ifi as *mut u8).add(nlmsg_align(size_of::<ifinfomsg>())) as *mut nlattr;

    if libbpf_nla_parse(tb.as_mut_ptr(), IFLA_MAX, attr, len, ptr::null()) != 0 {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    dump_link_nlmsg.unwrap()(cookie, ifi as *mut c_void, tb.as_mut_ptr())
}

unsafe extern "C" fn get_xdp_info(cookie: *mut c_void, msg: *mut c_void, tb: *mut *mut nlattr) -> c_int {
    let mut xdp_tb: [*mut nlattr; IFLA_XDP_MAX as usize + 1] =
        [ptr::null_mut(); IFLA_XDP_MAX as usize + 1];
    let xdp_id = cookie as *mut xdp_id_md;
    let ifinfo = msg as *mut ifinfomsg;

    if (*xdp_id).ifindex != 0 && (*xdp_id).ifindex != (*ifinfo).ifi_index {
        return 0;
    }
    if (*tb.add(IFLA_XDP as usize)).is_null() {
        return 0;
    }

    let ret = libbpf_nla_parse_nested(
        xdp_tb.as_mut_ptr(),
        IFLA_XDP_MAX,
        *tb.add(IFLA_XDP as usize),
        ptr::null(),
    );
    if ret != 0 {
        return ret;
    }
    if xdp_tb[IFLA_XDP_ATTACHED as usize].is_null() {
        return 0;
    }

    (*xdp_id).info.attach_mode = libbpf_nla_getattr_u8(xdp_tb[IFLA_XDP_ATTACHED as usize]);
    if (*xdp_id).info.attach_mode == XDP_ATTACHED_NONE {
        return 0;
    }
    if !xdp_tb[IFLA_XDP_PROG_ID as usize].is_null() {
        (*xdp_id).info.prog_id = libbpf_nla_getattr_u32(xdp_tb[IFLA_XDP_PROG_ID as usize]);
    }
    if !xdp_tb[IFLA_XDP_SKB_PROG_ID as usize].is_null() {
        (*xdp_id).info.skb_prog_id = libbpf_nla_getattr_u32(xdp_tb[IFLA_XDP_SKB_PROG_ID as usize]);
    }
    if !xdp_tb[IFLA_XDP_DRV_PROG_ID as usize].is_null() {
        (*xdp_id).info.drv_prog_id = libbpf_nla_getattr_u32(xdp_tb[IFLA_XDP_DRV_PROG_ID as usize]);
    }
    if !xdp_tb[IFLA_XDP_HW_PROG_ID as usize].is_null() {
        (*xdp_id).info.hw_prog_id = libbpf_nla_getattr_u32(xdp_tb[IFLA_XDP_HW_PROG_ID as usize]);
    }

    0
}

unsafe extern "C" fn parse_xdp_features(
    nh: *mut nlmsghdr,
    _fn: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let gnl = nlmsg_data(nh) as *mut genlmsghdr;
    let na = (gnl as *mut u8).add(GENL_HDRLEN) as *mut nlattr;
    let mut tb: [*mut nlattr; NETDEV_CMD_MAX as usize + 1] =
        [ptr::null_mut(); NETDEV_CMD_MAX as usize + 1];
    let md = cookie as *mut xdp_features_md;

    libbpf_nla_parse(
        tb.as_mut_ptr(),
        NETDEV_CMD_MAX,
        na,
        nlmsg_payload(nh, size_of::<genlmsghdr>()),
        ptr::null(),
    );

    if tb[NETDEV_A_DEV_IFINDEX as usize].is_null()
        || tb[NETDEV_A_DEV_XDP_FEATURES as usize].is_null()
    {
        return NL_CONT;
    }

    let ifindex = libbpf_nla_getattr_u32(tb[NETDEV_A_DEV_IFINDEX as usize]);
    if ifindex != (*md).ifindex as __u32 {
        return NL_CONT;
    }

    (*md).flags = libbpf_nla_getattr_u64(tb[NETDEV_A_DEV_XDP_FEATURES as usize]);
    if !tb[NETDEV_A_DEV_XDP_ZC_MAX_SEGS as usize].is_null() {
        (*md).xdp_zc_max_segs =
            libbpf_nla_getattr_u32(tb[NETDEV_A_DEV_XDP_ZC_MAX_SEGS as usize]);
    }
    NL_DONE
}

#[no_mangle]
pub unsafe extern "C" fn bpf_xdp_query(
    ifindex: c_int,
    mut xdp_flags: c_int,
    opts: *mut bpf_xdp_query_opts,
) -> c_int {
    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<ifinfomsg>());
    req.nh.nlmsg_type = RTM_GETLINK;
    req.nh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    req.ifinfo.ifi_family = AF_PACKET as __u8;

    let mut xdp_id: xdp_id_md = zeroed();
    let mut md: xdp_features_md = zeroed();
    md.ifindex = ifindex;
    let mut id: __u16 = 0;

    if !opts_valid(opts) {
        return libbpf_err(-EINVAL);
    }
    if (xdp_flags as __u32 & !XDP_FLAGS_MASK) != 0 {
        return libbpf_err(-EINVAL);
    }

    /* Check whether the single {HW,DRV,SKB} mode is set */
    xdp_flags &= (XDP_FLAGS_SKB_MODE | XDP_FLAGS_DRV_MODE | XDP_FLAGS_HW_MODE) as c_int;
    if (xdp_flags & (xdp_flags - 1)) != 0 {
        return libbpf_err(-EINVAL);
    }

    xdp_id.ifindex = ifindex;
    xdp_id.flags = xdp_flags as __u32;

    let mut err = libbpf_netlink_send_recv(
        &mut req,
        NETLINK_ROUTE,
        Some(__dump_link_nlmsg),
        Some(get_xdp_info),
        &mut xdp_id as *mut _ as *mut c_void,
    );
    if err != 0 {
        return libbpf_err(err);
    }

    (*opts).prog_id = xdp_id.info.prog_id;
    (*opts).drv_prog_id = xdp_id.info.drv_prog_id;
    (*opts).hw_prog_id = xdp_id.info.hw_prog_id;
    (*opts).skb_prog_id = xdp_id.info.skb_prog_id;
    (*opts).attach_mode = xdp_id.info.attach_mode;

    if !opts_has(opts, 0) {
        return 0;
    }

    err = libbpf_netlink_resolve_genl_family_id(c"netdev".as_ptr(), size_of::<[u8; 7]>() as __u16, &mut id);
    if err < 0 {
        if err == -ENOENT {
            (*opts).feature_flags = 0;
        } else {
            return libbpf_err(err);
        }
    } else {
        ptr::write_bytes(&mut req as *mut _, 0, 1);
        req.nh.nlmsg_len = nlmsg_length(GENL_HDRLEN);
        req.nh.nlmsg_flags = NLM_F_REQUEST;
        req.nh.nlmsg_type = id;
        req.gnl.cmd = NETDEV_CMD_DEV_GET;
        req.gnl.version = 2;

        err = nlattr_add(
            &mut req,
            NETDEV_A_DEV_IFINDEX,
            &ifindex as *const _ as *const c_void,
            size_of::<c_int>(),
        );
        if err < 0 {
            return libbpf_err(err);
        }

        err = libbpf_netlink_send_recv(
            &mut req,
            NETLINK_GENERIC,
            Some(parse_xdp_features),
            None,
            &mut md as *mut _ as *mut c_void,
        );
        if err != 0 {
            return libbpf_err(err);
        }

        (*opts).feature_flags = md.flags;
        (*opts).xdp_zc_max_segs = md.xdp_zc_max_segs;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_xdp_query_id(ifindex: c_int, mut flags: c_int, prog_id: *mut __u32) -> c_int {
    let mut opts: bpf_xdp_query_opts = zeroed();
    opts.sz = size_of::<bpf_xdp_query_opts>();

    let ret = bpf_xdp_query(ifindex, flags, &mut opts);
    if ret != 0 {
        return libbpf_err(ret);
    }

    flags &= XDP_FLAGS_MODES as c_int;
    if opts.attach_mode != XDP_ATTACHED_MULTI && flags == 0 {
        *prog_id = opts.prog_id;
    } else if (flags & XDP_FLAGS_DRV_MODE as c_int) != 0 {
        *prog_id = opts.drv_prog_id;
    } else if (flags & XDP_FLAGS_HW_MODE as c_int) != 0 {
        *prog_id = opts.hw_prog_id;
    } else if (flags & XDP_FLAGS_SKB_MODE as c_int) != 0 {
        *prog_id = opts.skb_prog_id;
    } else {
        *prog_id = 0;
    }

    0
}

unsafe extern "C" fn clsact_config(req: *mut libbpf_nla_req, _hook: *const bpf_tc_hook) -> c_int {
    (*req).tc.tcm_parent = TC_H_CLSACT;
    (*req).tc.tcm_handle = tc_h_make(TC_H_CLSACT, 0);
    nlattr_add(req, TCA_KIND, c"clsact".as_ptr() as *const c_void, size_of::<[u8; 7]>())
}

unsafe extern "C" fn qdisc_config(req: *mut libbpf_nla_req, hook: *const bpf_tc_hook) -> c_int {
    let qdisc = (*hook).qdisc;
    (*req).tc.tcm_parent = if (*hook).parent != 0 { (*hook).parent } else { TC_H_ROOT };
    (*req).tc.tcm_handle = (*hook).handle;
    nlattr_add(req, TCA_KIND, qdisc as *const c_void, strlen(qdisc) + 1)
}

unsafe extern "C" fn attach_point_to_config(hook: *mut bpf_tc_hook, config: *mut qdisc_config_t) -> c_int {
    match (*hook).attach_point {
        x if x == bpf_tc_attach_point::BPF_TC_INGRESS as c_int
            || x == bpf_tc_attach_point::BPF_TC_EGRESS as c_int
            || x == (bpf_tc_attach_point::BPF_TC_INGRESS as c_int | bpf_tc_attach_point::BPF_TC_EGRESS as c_int) =>
        {
            if (*hook).parent != 0 {
                return -EINVAL;
            }
            *config = Some(clsact_config);
            0
        }
        x if x == bpf_tc_attach_point::BPF_TC_CUSTOM as c_int => -EOPNOTSUPP,
        x if x == bpf_tc_attach_point::BPF_TC_QDISC as c_int => {
            *config = Some(qdisc_config);
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn tc_get_tcm_parent(attach_point: bpf_tc_attach_point, parent: *mut __u32) -> c_int {
    match attach_point as c_int {
        x if x == bpf_tc_attach_point::BPF_TC_INGRESS as c_int
            || x == bpf_tc_attach_point::BPF_TC_EGRESS as c_int =>
        {
            if *parent != 0 {
                return -EINVAL;
            }
            *parent = tc_h_make(
                TC_H_CLSACT,
                if attach_point as c_int == bpf_tc_attach_point::BPF_TC_INGRESS as c_int {
                    TC_H_MIN_INGRESS
                } else {
                    TC_H_MIN_EGRESS
                },
            );
        }
        x if x == bpf_tc_attach_point::BPF_TC_CUSTOM as c_int => {
            if *parent == 0 {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn tc_qdisc_modify(hook: *mut bpf_tc_hook, cmd: c_int, flags: c_int) -> c_int {
    let mut config: qdisc_config_t = None;
    let mut ret = attach_point_to_config(hook, &mut config);
    if ret < 0 {
        return ret;
    }

    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nh.nlmsg_flags = (NLM_F_REQUEST | NLM_F_ACK) | flags as __u16;
    req.nh.nlmsg_type = cmd as __u16;
    req.tc.tcm_family = AF_UNSPEC as __u8;
    req.tc.tcm_ifindex = (*hook).ifindex;

    ret = config.unwrap()(&mut req, hook);
    if ret < 0 {
        return ret;
    }

    libbpf_netlink_send_recv(&mut req, NETLINK_ROUTE, None, None, ptr::null_mut())
}

unsafe extern "C" fn tc_qdisc_create_excl(hook: *mut bpf_tc_hook) -> c_int {
    tc_qdisc_modify(hook, RTM_NEWQDISC as c_int, (NLM_F_CREATE | NLM_F_EXCL) as c_int)
}

unsafe extern "C" fn tc_qdisc_delete(hook: *mut bpf_tc_hook) -> c_int {
    tc_qdisc_modify(hook, RTM_DELQDISC as c_int, 0)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int {
    if hook.is_null() || !opts_valid(hook) || (*hook).ifindex <= 0 {
        return libbpf_err(-EINVAL);
    }

    let ret = tc_qdisc_create_excl(hook);
    libbpf_err(ret)
}

unsafe extern "C" fn __bpf_tc_detach(
    hook: *const bpf_tc_hook,
    opts: *const bpf_tc_opts,
    flush: bool,
) -> c_int {
    if hook.is_null() || !opts_valid(hook) || !opts_valid(opts) {
        return -EINVAL;
    }

    let ifindex = (*hook).ifindex;
    let mut parent = (*hook).parent;
    let attach_point = (*hook).attach_point;
    let handle = (*opts).handle;
    let priority = (*opts).priority;
    let prog_fd = (*opts).prog_fd;
    let prog_id = (*opts).prog_id;
    let flags = (*opts).flags;
    let mut protocol: __u16 = 0;

    if ifindex <= 0 || flags != 0 || prog_fd != 0 || prog_id != 0 {
        return -EINVAL;
    }
    if priority > UINT16_MAX {
        return -EINVAL;
    }
    if !flush {
        if handle == 0 || priority == 0 {
            return -EINVAL;
        }
        protocol = ETH_P_ALL;
    } else if handle != 0 || priority != 0 {
        return -EINVAL;
    }

    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    req.nh.nlmsg_type = RTM_DELTFILTER;
    req.tc.tcm_family = AF_UNSPEC as __u8;
    req.tc.tcm_ifindex = ifindex;
    if !flush {
        req.tc.tcm_handle = handle;
        req.tc.tcm_info = tc_h_make(priority << 16, htons(protocol) as __u32);
    }

    let ret = tc_get_tcm_parent(core::mem::transmute(attach_point), &mut parent);
    if ret < 0 {
        return ret;
    }
    req.tc.tcm_parent = parent;

    if !flush {
        let ret2 = nlattr_add(&mut req, TCA_KIND, c"bpf".as_ptr() as *const c_void, size_of::<[u8; 4]>());
        if ret2 < 0 {
            return ret2;
        }
    }

    libbpf_netlink_send_recv(&mut req, NETLINK_ROUTE, None, None, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int {
    if hook.is_null() || !opts_valid(hook) || (*hook).ifindex <= 0 {
        return libbpf_err(-EINVAL);
    }

    match (*hook).attach_point {
        x if x == bpf_tc_attach_point::BPF_TC_INGRESS as c_int
            || x == bpf_tc_attach_point::BPF_TC_EGRESS as c_int =>
        {
            libbpf_err(__bpf_tc_detach(hook, ptr::null(), true))
        }
        x if x == bpf_tc_attach_point::BPF_TC_QDISC as c_int
            || x == (bpf_tc_attach_point::BPF_TC_INGRESS as c_int | bpf_tc_attach_point::BPF_TC_EGRESS as c_int) =>
        {
            libbpf_err(tc_qdisc_delete(hook))
        }
        x if x == bpf_tc_attach_point::BPF_TC_CUSTOM as c_int => libbpf_err(-EOPNOTSUPP),
        _ => libbpf_err(-EINVAL),
    }
}

unsafe extern "C" fn __get_tc_info(
    cookie: *mut c_void,
    tc: *mut tcmsg,
    tb: *mut *mut nlattr,
    unicast: bool,
) -> c_int {
    let mut tbb: [*mut nlattr; TCA_BPF_MAX as usize + 1] =
        [ptr::null_mut(); TCA_BPF_MAX as usize + 1];
    let info = cookie as *mut bpf_cb_ctx;

    if info.is_null() || (*info).opts.is_null() {
        return -EINVAL;
    }
    if unicast && (*info).processed {
        return -EINVAL;
    }
    if (*tb.add(TCA_OPTIONS as usize)).is_null() {
        return NL_CONT;
    }

    libbpf_nla_parse_nested(
        tbb.as_mut_ptr(),
        TCA_BPF_MAX,
        *tb.add(TCA_OPTIONS as usize),
        ptr::null(),
    );
    if tbb[TCA_BPF_ID as usize].is_null() {
        return -EINVAL;
    }

    (*(*info).opts).prog_id = libbpf_nla_getattr_u32(tbb[TCA_BPF_ID as usize]);
    (*(*info).opts).handle = (*tc).tcm_handle;
    (*(*info).opts).priority = tc_h_maj((*tc).tcm_info) >> 16;

    (*info).processed = true;
    if unicast { NL_NEXT } else { NL_DONE }
}

unsafe extern "C" fn get_tc_info(
    nh: *mut nlmsghdr,
    _fn: libbpf_dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let tc = nlmsg_data(nh) as *mut tcmsg;
    let mut tb: [*mut nlattr; TCA_MAX as usize + 1] = [ptr::null_mut(); TCA_MAX as usize + 1];

    libbpf_nla_parse(
        tb.as_mut_ptr(),
        TCA_MAX,
        (tc as *mut u8).add(nlmsg_align(size_of::<tcmsg>())) as *mut nlattr,
        nlmsg_payload(nh, size_of::<tcmsg>()),
        ptr::null(),
    );
    if tb[TCA_KIND as usize].is_null() {
        return NL_CONT;
    }
    __get_tc_info(cookie, tc, tb.as_mut_ptr(), ((*nh).nlmsg_flags & NLM_F_ECHO) != 0)
}

unsafe extern "C" fn tc_add_fd_and_name(req: *mut libbpf_nla_req, fd: c_int) -> c_int {
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut name: [c_char; 256] = [0; 256];

    let mut ret = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
    if ret < 0 {
        return ret;
    }

    ret = nlattr_add(req, TCA_BPF_FD, &fd as *const _ as *const c_void, size_of::<c_int>());
    if ret < 0 {
        return ret;
    }
    let len = snprintf(
        name.as_mut_ptr(),
        name.len(),
        c"%s:[%u]".as_ptr(),
        info.name.as_ptr(),
        info.id,
    );
    if len < 0 {
        return -errno();
    }
    if len as usize >= name.len() {
        return -ENAMETOOLONG;
    }
    nlattr_add(req, TCA_BPF_NAME, name.as_ptr() as *const c_void, len as usize + 1)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tc_attach(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int {
    if hook.is_null() || opts.is_null() || !opts_valid(hook) || !opts_valid(opts) {
        return libbpf_err(-EINVAL);
    }

    let ifindex = (*hook).ifindex;
    let mut parent = (*hook).parent;
    let attach_point = (*hook).attach_point;
    let handle = (*opts).handle;
    let priority = (*opts).priority;
    let prog_fd = (*opts).prog_fd;
    let prog_id = (*opts).prog_id;
    let mut flags = (*opts).flags;

    if ifindex <= 0 || prog_fd == 0 || prog_id != 0 {
        return libbpf_err(-EINVAL);
    }
    if priority > UINT16_MAX {
        return libbpf_err(-EINVAL);
    }
    if (flags & !BPF_TC_F_REPLACE) != 0 {
        return libbpf_err(-EINVAL);
    }

    flags = if (flags & BPF_TC_F_REPLACE) != 0 {
        NLM_F_REPLACE as __u32
    } else {
        NLM_F_EXCL as __u32
    };
    let protocol = ETH_P_ALL;

    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_CREATE | NLM_F_ECHO | flags as __u16;
    req.nh.nlmsg_type = RTM_NEWTFILTER;
    req.tc.tcm_family = AF_UNSPEC as __u8;
    req.tc.tcm_ifindex = ifindex;
    req.tc.tcm_handle = handle;
    req.tc.tcm_info = tc_h_make(priority << 16, htons(protocol) as __u32);

    let mut ret = tc_get_tcm_parent(core::mem::transmute(attach_point), &mut parent);
    if ret < 0 {
        return libbpf_err(ret);
    }
    req.tc.tcm_parent = parent;

    ret = nlattr_add(&mut req, TCA_KIND, c"bpf".as_ptr() as *const c_void, size_of::<[u8; 4]>());
    if ret < 0 {
        return libbpf_err(ret);
    }
    let nla = nlattr_begin_nested(&mut req, TCA_OPTIONS);
    if nla.is_null() {
        return libbpf_err(-EMSGSIZE);
    }
    ret = tc_add_fd_and_name(&mut req, prog_fd);
    if ret < 0 {
        return libbpf_err(ret);
    }
    let bpf_flags: __u32 = TCA_BPF_FLAG_ACT_DIRECT;
    ret = nlattr_add(
        &mut req,
        TCA_BPF_FLAGS,
        &bpf_flags as *const _ as *const c_void,
        size_of::<__u32>(),
    );
    if ret < 0 {
        return libbpf_err(ret);
    }
    nlattr_end_nested(&mut req, nla);

    let mut info: bpf_cb_ctx = zeroed();
    info.opts = opts;

    ret = libbpf_netlink_send_recv(
        &mut req,
        NETLINK_ROUTE,
        Some(get_tc_info),
        None,
        &mut info as *mut _ as *mut c_void,
    );
    if ret < 0 {
        return libbpf_err(ret);
    }
    if !info.processed {
        return libbpf_err(-ENOENT);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tc_detach(hook: *const bpf_tc_hook, opts: *const bpf_tc_opts) -> c_int {
    if opts.is_null() {
        return libbpf_err(-EINVAL);
    }
    let ret = __bpf_tc_detach(hook, opts, false);
    libbpf_err(ret)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tc_query(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int {
    if hook.is_null() || opts.is_null() || !opts_valid(hook) || !opts_valid(opts) {
        return libbpf_err(-EINVAL);
    }

    let ifindex = (*hook).ifindex;
    let mut parent = (*hook).parent;
    let attach_point = (*hook).attach_point;
    let handle = (*opts).handle;
    let priority = (*opts).priority;
    let prog_fd = (*opts).prog_fd;
    let prog_id = (*opts).prog_id;
    let flags = (*opts).flags;

    if ifindex <= 0 || flags != 0 || prog_fd != 0 || prog_id != 0 || handle == 0 || priority == 0 {
        return libbpf_err(-EINVAL);
    }
    if priority > UINT16_MAX {
        return libbpf_err(-EINVAL);
    }

    let protocol = ETH_P_ALL;

    let mut req: libbpf_nla_req = zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST;
    req.nh.nlmsg_type = RTM_GETTFILTER;
    req.tc.tcm_family = AF_UNSPEC as __u8;
    req.tc.tcm_ifindex = ifindex;
    req.tc.tcm_handle = handle;
    req.tc.tcm_info = tc_h_make(priority << 16, htons(protocol) as __u32);

    let mut ret = tc_get_tcm_parent(core::mem::transmute(attach_point), &mut parent);
    if ret < 0 {
        return libbpf_err(ret);
    }
    req.tc.tcm_parent = parent;

    ret = nlattr_add(&mut req, TCA_KIND, c"bpf".as_ptr() as *const c_void, size_of::<[u8; 4]>());
    if ret < 0 {
        return libbpf_err(ret);
    }

    let mut info: bpf_cb_ctx = zeroed();
    info.opts = opts;

    ret = libbpf_netlink_send_recv(
        &mut req,
        NETLINK_ROUTE,
        Some(get_tc_info),
        None,
        &mut info as *mut _ as *mut c_void,
    );
    if ret < 0 {
        return libbpf_err(ret);
    }
    if !info.processed {
        return libbpf_err(-ENOENT);
    }
    ret
}
