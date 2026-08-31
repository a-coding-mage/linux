// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Facebook

// Translated from bpf/bpftool/net.c. C include dependencies are intentionally
// left as external declarations or unresolved constants/types from the future
// integration context.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type size_t = usize;
type socklen_t = u32;

const SOL_NETLINK: c_int = 270;

#[repr(C)]
struct ip_devname_ifindex {
    devname: [c_char; 64],
    ifindex: c_int,
}

#[repr(C)]
struct bpf_netdev_t {
    devices: *mut ip_devname_ifindex,
    used_len: c_int,
    array_len: c_int,
    filter_idx: c_int,
}

#[repr(C)]
struct tc_kind_handle {
    kind: [c_char; 64],
    handle: c_int,
}

#[repr(C)]
struct bpf_tcinfo_t {
    handle_array: *mut tc_kind_handle,
    used_len: c_int,
    array_len: c_int,
    is_qdisc: bool,
}

#[repr(C)]
struct bpf_filter_t {
    kind: *const c_char,
    devname: *const c_char,
    ifindex: c_int,
}

#[repr(C)]
struct bpf_attach_info {
    flow_dissector_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum net_attach_type {
    NET_ATTACH_TYPE_XDP = 0,
    NET_ATTACH_TYPE_XDP_GENERIC,
    NET_ATTACH_TYPE_XDP_DRIVER,
    NET_ATTACH_TYPE_XDP_OFFLOAD,
    NET_ATTACH_TYPE_TCX_INGRESS,
    NET_ATTACH_TYPE_TCX_EGRESS,
}

static ATTACH_TYPE_XDP: &[u8] = b"xdp\0";
static ATTACH_TYPE_XDP_GENERIC: &[u8] = b"xdpgeneric\0";
static ATTACH_TYPE_XDP_DRIVER: &[u8] = b"xdpdrv\0";
static ATTACH_TYPE_XDP_OFFLOAD: &[u8] = b"xdpoffload\0";
static ATTACH_TYPE_TCX_INGRESS: &[u8] = b"tcx_ingress\0";
static ATTACH_TYPE_TCX_EGRESS: &[u8] = b"tcx_egress\0";

static attach_type_strings: [*const c_char; 6] = [
    ATTACH_TYPE_XDP.as_ptr() as *const c_char,
    ATTACH_TYPE_XDP_GENERIC.as_ptr() as *const c_char,
    ATTACH_TYPE_XDP_DRIVER.as_ptr() as *const c_char,
    ATTACH_TYPE_XDP_OFFLOAD.as_ptr() as *const c_char,
    ATTACH_TYPE_TCX_INGRESS.as_ptr() as *const c_char,
    ATTACH_TYPE_TCX_EGRESS.as_ptr() as *const c_char,
];

static ATTACH_LOC_TCX_INGRESS: &[u8] = b"tcx/ingress\0";
static ATTACH_LOC_TCX_EGRESS: &[u8] = b"tcx/egress\0";
static ATTACH_LOC_NETKIT_PRIMARY: &[u8] = b"netkit/primary\0";
static ATTACH_LOC_NETKIT_PEER: &[u8] = b"netkit/peer\0";

static attach_loc_strings: [*const c_char; 4] = [
    ATTACH_LOC_TCX_INGRESS.as_ptr() as *const c_char,
    ATTACH_LOC_TCX_EGRESS.as_ptr() as *const c_char,
    ATTACH_LOC_NETKIT_PRIMARY.as_ptr() as *const c_char,
    ATTACH_LOC_NETKIT_PEER.as_ptr() as *const c_char,
];

pub const net_attach_type_size: size_t = attach_type_strings.len();

type dump_nlmsg_t = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *mut nlattr) -> c_int>;
type __dump_nlmsg_t =
    Option<unsafe extern "C" fn(*mut nlmsghdr, dump_nlmsg_t, *mut c_void) -> c_int>;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: u16,
    nl_pad: u16,
    nl_pid: __u32,
    nl_groups: __u32,
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: __u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: __u32,
    nlmsg_pid: __u32,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
}

#[repr(C)]
struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
struct tcmsg {
    tcm_family: u8,
    tcm__pad1: u8,
    tcm__pad2: u16,
    tcm_ifindex: c_int,
    tcm_handle: c_uint,
    tcm_parent: c_uint,
    tcm_info: c_uint,
}

#[repr(C)]
struct ifinfomsg {
    ifi_family: u8,
    __ifi_pad: u8,
    ifi_type: u16,
    ifi_index: c_int,
    ifi_flags: c_uint,
    ifi_change: c_uint,
}

#[repr(C)]
struct bpf_prog_info {
    name: [c_char; 16],
}

#[repr(C)]
struct bpf_link_info_netfilter {
    pf: c_uint,
    hooknum: c_uint,
    priority: c_int,
    flags: c_uint,
}

#[repr(C)]
struct bpf_link_info {
    type_: __u32,
    id: __u32,
    prog_id: __u32,
    netfilter: bpf_link_info_netfilter,
}

#[repr(C)]
struct bpf_prog_query_opts {
    sz: size_t,
    prog_ids: *mut __u32,
    prog_attach_flags: *mut __u32,
    link_ids: *mut __u32,
    link_attach_flags: *mut __u32,
    count: __u32,
}

#[repr(C)]
struct bpf_prog_attach_opts {
    sz: size_t,
    flags: __u32,
}

#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

extern "C" {
    static mut errno: c_int;
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;
    static mut bin_name: *const c_char;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> isize;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> isize;
    fn time(tloc: *mut isize) -> isize;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );

    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn p_err(format: *const c_char, ...);
    fn libbpf_nla_parse(
        tb: *mut *mut nlattr,
        maxtype: c_int,
        head: *mut nlattr,
        len: c_int,
        policy: *mut c_void,
    ) -> c_int;
    fn libbpf_nla_dump_errormsg(nlh: *mut nlmsghdr);
    fn libbpf_nla_getattr_str(attr: *mut nlattr) -> *const c_char;
    fn libbpf_nla_data(attr: *mut nlattr) -> *mut c_void;
    fn libbpf_strerror(err: c_int, buf: *mut c_char, size: size_t) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn do_xdp_dump(ifinfo: *mut ifinfomsg, tb: *mut *mut nlattr) -> c_int;
    fn do_filter_dump(
        msg: *mut tcmsg,
        tb: *mut *mut nlattr,
        kind: *const c_char,
        devname: *const c_char,
        ifindex: c_int,
    ) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *__u32) -> c_int;
    fn get_prog_full_name(info: *mut bpf_prog_info, fd: c_int, name: *mut c_char, len: size_t);
    fn bpf_prog_query_opts(target_fd: c_int, attach_type: c_int, opts: *mut bpf_prog_query_opts)
        -> c_int;
    fn bpf_prog_query(
        target_fd: c_int,
        attach_type: c_int,
        query_flags: __u32,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: __u32, opts: *const c_void) -> c_int;
    fn bpf_prog_attach_opts(
        progfd: c_int,
        targetfd: c_int,
        type_: c_int,
        opts: *mut bpf_prog_attach_opts,
    ) -> c_int;
    fn bpf_prog_attach(progfd: c_int, targetfd: c_int, type_: c_int, flags: __u32) -> c_int;
    fn bpf_prog_detach(targetfd: c_int, type_: c_int) -> c_int;
    fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, len: *mut __u32) -> c_int;
    fn netfilter_dump_json(info: *mut bpf_link_info, wtr: *mut c_void);
    fn netfilter_dump_plain(info: *mut bpf_link_info);
    fn jsonw_null(wtr: *mut c_void);
    fn jsonw_start_array(wtr: *mut c_void);
    fn jsonw_end_array(wtr: *mut c_void);
    fn usage();
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
    ) -> c_int;
}

const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_EXT_ACK: c_int = 11;
const AF_UNSPEC: c_int = 0;
const AF_PACKET: c_int = 17;
const O_RDONLY: c_int = 0;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const INT_MAX: c_uint = c_int::MAX as c_uint;
const LIBBPF_ERRNO__INTERNAL: c_int = 4000;
const LIBBPF_ERRNO__WRNGPID: c_int = 4001;
const LIBBPF_ERRNO__INVSEQ: c_int = 4002;
const LIBBPF_ERRNO__NLPARSE: c_int = 4003;
const NLM_F_MULTI: u16 = 2;
const NLM_F_REQUEST: u16 = 1;
const NLM_F_DUMP: u16 = 0x300;
const NLMSG_ERROR: u16 = 2;
const NLMSG_DONE: u16 = 3;
const RTM_GETLINK: u16 = 18;
const RTM_GETQDISC: u16 = 38;
const RTM_GETTCLASS: u16 = 40;
const RTM_GETTFILTER: u16 = 44;
const IFLA_IFNAME: usize = 3;
const IFLA_MAX: usize = 64;
const TCA_KIND: usize = 1;
const TCA_MAX: usize = 16;
const TC_H_ROOT: c_int = -1i32;
const TC_H_CLSACT: c_uint = 0xfffffff1;
const TC_H_MIN_INGRESS: c_uint = 0xfff2;
const TC_H_MIN_EGRESS: c_uint = 0xfff3;
const BPF_FLOW_DISSECTOR: c_int = 21;
const XDP_FLAGS_UPDATE_IF_NOEXIST: __u32 = 1;
const XDP_FLAGS_SKB_MODE: __u32 = 2;
const XDP_FLAGS_DRV_MODE: __u32 = 4;
const XDP_FLAGS_HW_MODE: __u32 = 8;
const BPF_TCX_INGRESS: c_int = 0;
const BPF_TCX_EGRESS: c_int = 1;
const BPF_NETKIT_PRIMARY: c_int = 2;
const BPF_NETKIT_PEER: c_int = 3;
const BPF_F_BEFORE: __u32 = 1 << 3;
const BPF_LINK_TYPE_NETFILTER: __u32 = 10;
const MAX_PROG_FULL_NAME: usize = 128;

unsafe fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

unsafe fn nlmsg_length(len: usize) -> __u32 {
    (nlmsg_align(size_of::<nlmsghdr>()) + len) as __u32
}

unsafe fn nlmsg_ok(nlh: *mut nlmsghdr, len: c_uint) -> bool {
    len as usize >= size_of::<nlmsghdr>()
        && (*nlh).nlmsg_len as usize >= size_of::<nlmsghdr>()
        && (*nlh).nlmsg_len <= len
}

unsafe fn nlmsg_next(nlh: *mut nlmsghdr, len: *mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(nlmsg_align(size_of::<nlmsghdr>())) as *mut c_void
}

unsafe fn tc_h_make(maj: c_uint, min: c_uint) -> c_int {
    (maj | min) as c_int
}

unsafe fn net_start_object() {}
unsafe fn net_end_object() {}
unsafe fn net_end_object_final() {}
unsafe fn net_start_array(_json: *const c_char, _plain: *const c_char) {}
unsafe fn net_end_array(_plain: *const c_char) {}
unsafe fn net_dump_str(_json: *const c_char, _fmt: *const c_char, _value: *const c_char) {}
unsafe fn net_dump_uint(_json: *const c_char, _fmt: *const c_char, _value: c_uint) {}
unsafe fn net_dump_uint_only(_value: c_uint) {}

unsafe fn parse_attach_type(str_: *const c_char) -> size_t {
    let mut type_: size_t = 0;

    while type_ < net_attach_type_size {
        if !attach_type_strings[type_].is_null() && is_prefix(str_, attach_type_strings[type_]) {
            return type_;
        }
        type_ += 1;
    }

    net_attach_type_size
}

unsafe extern "C" fn netlink_open(nl_pid: *__u32) -> c_int {
    let mut sa: sockaddr_nl = zeroed();
    let mut addrlen: socklen_t;
    let one: c_int = 1;
    let mut ret: c_int;
    let sock: c_int;

    sa.nl_family = AF_NETLINK as u16;

    sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    if sock < 0 {
        return -errno;
    }

    if setsockopt(
        sock,
        SOL_NETLINK,
        NETLINK_EXT_ACK,
        &one as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        p_err(b"Netlink error reporting not supported\0".as_ptr() as *const c_char);
    }

    if bind(
        sock,
        &sa as *const _ as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        ret = -errno;
        close(sock);
        return ret;
    }

    addrlen = size_of::<sockaddr_nl>() as socklen_t;
    if getsockname(
        sock,
        &mut sa as *mut _ as *mut sockaddr,
        &mut addrlen,
    ) < 0
    {
        ret = -errno;
        close(sock);
        return ret;
    }

    if addrlen as usize != size_of::<sockaddr_nl>() {
        ret = -LIBBPF_ERRNO__INTERNAL;
        close(sock);
        return ret;
    }

    *nl_pid = sa.nl_pid;
    sock
}

unsafe extern "C" fn netlink_recv(
    sock: c_int,
    nl_pid: __u32,
    seq: __u32,
    _fn: __dump_nlmsg_t,
    fn_: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut multipart = true;
    let mut err: *mut nlmsgerr;
    let mut nh: *mut nlmsghdr;
    let mut buf = [0i8; 8192];
    let mut len: c_int;
    let mut ret: c_int;

    while multipart {
        multipart = false;
        len = recv(sock, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) as c_int;
        if len < 0 {
            ret = -errno;
            return ret;
        }

        if len == 0 {
            break;
        }

        nh = buf.as_mut_ptr() as *mut nlmsghdr;
        while nlmsg_ok(nh, len as c_uint) {
            if (*nh).nlmsg_pid != nl_pid {
                return -LIBBPF_ERRNO__WRNGPID;
            }
            if (*nh).nlmsg_seq != seq {
                return -LIBBPF_ERRNO__INVSEQ;
            }
            if ((*nh).nlmsg_flags & NLM_F_MULTI) != 0 {
                multipart = true;
            }
            match (*nh).nlmsg_type {
                NLMSG_ERROR => {
                    err = nlmsg_data(nh) as *mut nlmsgerr;
                    if (*err).error == 0 {
                        nh = nlmsg_next(nh, &mut len);
                        continue;
                    }
                    ret = (*err).error;
                    libbpf_nla_dump_errormsg(nh);
                    return ret;
                }
                NLMSG_DONE => return 0,
                _ => {}
            }
            if let Some(cb) = _fn {
                ret = cb(nh, fn_, cookie);
                if ret != 0 {
                    return ret;
                }
            }
            nh = nlmsg_next(nh, &mut len);
        }

        if len != 0 {
            p_err(
                b"Invalid message or trailing data in Netlink response: %d bytes left\0".as_ptr()
                    as *const c_char,
                len,
            );
        }
    }
    0
}

unsafe extern "C" fn __dump_class_nlmsg(
    nlh: *mut nlmsghdr,
    dump_class_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut tb: [*mut nlattr; TCA_MAX + 1] = [ptr::null_mut(); TCA_MAX + 1];
    let mut attr: *mut nlattr;
    let t = nlmsg_data(nlh) as *mut tcmsg;
    let len: c_int;

    len = (*nlh).nlmsg_len as c_int - nlmsg_length(size_of::<tcmsg>()) as c_int;
    attr = (t as *mut u8).add(nlmsg_align(size_of::<tcmsg>())) as *mut nlattr;
    if libbpf_nla_parse(tb.as_mut_ptr(), TCA_MAX as c_int, attr, len, ptr::null_mut()) != 0 {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    dump_class_nlmsg.unwrap()(cookie, t as *mut c_void, tb.as_mut_ptr())
}

unsafe extern "C" fn netlink_get_class(
    sock: c_int,
    nl_pid: c_uint,
    ifindex: c_int,
    dump_class_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    #[repr(C)]
    struct Req {
        nlh: nlmsghdr,
        t: tcmsg,
    }
    let mut req: Req = zeroed();
    let seq = time(ptr::null_mut()) as c_int;

    req.nlh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nlh.nlmsg_type = RTM_GETTCLASS;
    req.nlh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    req.t.tcm_family = AF_UNSPEC as u8;
    req.t.tcm_ifindex = ifindex;
    req.nlh.nlmsg_seq = seq as __u32;
    if send(sock, &req as *const _ as *const c_void, req.nlh.nlmsg_len as size_t, 0) < 0 {
        return -errno;
    }

    netlink_recv(sock, nl_pid, seq as __u32, Some(__dump_class_nlmsg), dump_class_nlmsg, cookie)
}

unsafe extern "C" fn __dump_qdisc_nlmsg(
    nlh: *mut nlmsghdr,
    dump_qdisc_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    __dump_class_nlmsg(nlh, dump_qdisc_nlmsg, cookie)
}

unsafe extern "C" fn netlink_get_qdisc(
    sock: c_int,
    nl_pid: c_uint,
    ifindex: c_int,
    dump_qdisc_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    #[repr(C)]
    struct Req {
        nlh: nlmsghdr,
        t: tcmsg,
    }
    let mut req: Req = zeroed();
    let seq = time(ptr::null_mut()) as c_int;

    req.nlh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nlh.nlmsg_type = RTM_GETQDISC;
    req.nlh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    req.t.tcm_family = AF_UNSPEC as u8;
    req.t.tcm_ifindex = ifindex;
    req.nlh.nlmsg_seq = seq as __u32;
    if send(sock, &req as *const _ as *const c_void, req.nlh.nlmsg_len as size_t, 0) < 0 {
        return -errno;
    }

    netlink_recv(sock, nl_pid, seq as __u32, Some(__dump_qdisc_nlmsg), dump_qdisc_nlmsg, cookie)
}

unsafe extern "C" fn __dump_filter_nlmsg(
    nlh: *mut nlmsghdr,
    dump_filter_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    __dump_class_nlmsg(nlh, dump_filter_nlmsg, cookie)
}

unsafe extern "C" fn netlink_get_filter(
    sock: c_int,
    nl_pid: c_uint,
    ifindex: c_int,
    handle: c_int,
    dump_filter_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    #[repr(C)]
    struct Req {
        nlh: nlmsghdr,
        t: tcmsg,
    }
    let mut req: Req = zeroed();
    let seq = time(ptr::null_mut()) as c_int;

    req.nlh.nlmsg_len = nlmsg_length(size_of::<tcmsg>());
    req.nlh.nlmsg_type = RTM_GETTFILTER;
    req.nlh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    req.t.tcm_family = AF_UNSPEC as u8;
    req.t.tcm_ifindex = ifindex;
    req.t.tcm_parent = handle as c_uint;
    req.nlh.nlmsg_seq = seq as __u32;
    if send(sock, &req as *const _ as *const c_void, req.nlh.nlmsg_len as size_t, 0) < 0 {
        return -errno;
    }

    netlink_recv(sock, nl_pid, seq as __u32, Some(__dump_filter_nlmsg), dump_filter_nlmsg, cookie)
}

unsafe extern "C" fn __dump_link_nlmsg(
    nlh: *mut nlmsghdr,
    dump_link_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    let mut tb: [*mut nlattr; IFLA_MAX + 1] = [ptr::null_mut(); IFLA_MAX + 1];
    let attr: *mut nlattr;
    let ifi = nlmsg_data(nlh) as *mut ifinfomsg;
    let len: c_int;

    len = (*nlh).nlmsg_len as c_int - nlmsg_length(size_of::<ifinfomsg>()) as c_int;
    attr = (ifi as *mut u8).add(nlmsg_align(size_of::<ifinfomsg>())) as *mut nlattr;
    if libbpf_nla_parse(tb.as_mut_ptr(), IFLA_MAX as c_int, attr, len, ptr::null_mut()) != 0 {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    dump_link_nlmsg.unwrap()(cookie, ifi as *mut c_void, tb.as_mut_ptr())
}

unsafe extern "C" fn netlink_get_link(
    sock: c_int,
    nl_pid: c_uint,
    dump_link_nlmsg: dump_nlmsg_t,
    cookie: *mut c_void,
) -> c_int {
    #[repr(C)]
    struct Req {
        nlh: nlmsghdr,
        ifm: ifinfomsg,
    }
    let mut req: Req = zeroed();
    let seq = time(ptr::null_mut()) as c_int;

    req.nlh.nlmsg_len = nlmsg_length(size_of::<ifinfomsg>());
    req.nlh.nlmsg_type = RTM_GETLINK;
    req.nlh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    req.ifm.ifi_family = AF_PACKET as u8;
    req.nlh.nlmsg_seq = seq as __u32;
    if send(sock, &req as *const _ as *const c_void, req.nlh.nlmsg_len as size_t, 0) < 0 {
        return -errno;
    }

    netlink_recv(sock, nl_pid, seq as __u32, Some(__dump_link_nlmsg), dump_link_nlmsg, cookie)
}

unsafe extern "C" fn dump_link_nlmsg(
    cookie: *mut c_void,
    msg: *mut c_void,
    tb: *mut *mut nlattr,
) -> c_int {
    let netinfo = cookie as *mut bpf_netdev_t;
    let ifinfo = msg as *mut ifinfomsg;
    let tmp: *mut ip_devname_ifindex;

    if (*netinfo).filter_idx > 0 && (*netinfo).filter_idx != (*ifinfo).ifi_index {
        return 0;
    }

    if (*netinfo).used_len == (*netinfo).array_len {
        tmp = realloc(
            (*netinfo).devices as *mut c_void,
            ((*netinfo).array_len + 16) as size_t * size_of::<ip_devname_ifindex>(),
        ) as *mut ip_devname_ifindex;
        if tmp.is_null() {
            return -ENOMEM;
        }

        (*netinfo).devices = tmp;
        (*netinfo).array_len += 16;
    }
    (*(*netinfo).devices.add((*netinfo).used_len as usize)).ifindex = (*ifinfo).ifi_index;
    snprintf(
        (*(*netinfo).devices.add((*netinfo).used_len as usize)).devname.as_mut_ptr(),
        size_of::<[c_char; 64]>(),
        b"%s\0".as_ptr() as *const c_char,
        if !(*tb.add(IFLA_IFNAME)).is_null() {
            libbpf_nla_getattr_str(*tb.add(IFLA_IFNAME))
        } else {
            b"\0".as_ptr() as *const c_char
        },
    );
    (*netinfo).used_len += 1;

    do_xdp_dump(ifinfo, tb)
}

unsafe extern "C" fn dump_class_qdisc_nlmsg(
    cookie: *mut c_void,
    msg: *mut c_void,
    tb: *mut *mut nlattr,
) -> c_int {
    let tcinfo = cookie as *mut bpf_tcinfo_t;
    let info = msg as *mut tcmsg;
    let tmp: *mut tc_kind_handle;

    if (*tcinfo).is_qdisc {
        /* skip clsact qdisc */
        if !(*tb.add(TCA_KIND)).is_null()
            && strcmp(libbpf_nla_data(*tb.add(TCA_KIND)) as *const c_char, b"clsact\0".as_ptr() as *const c_char) == 0
        {
            return 0;
        }
        if (*info).tcm_handle == 0 {
            return 0;
        }
    }

    if (*tcinfo).used_len == (*tcinfo).array_len {
        tmp = realloc(
            (*tcinfo).handle_array as *mut c_void,
            ((*tcinfo).array_len + 16) as size_t * size_of::<tc_kind_handle>(),
        ) as *mut tc_kind_handle;
        if tmp.is_null() {
            return -ENOMEM;
        }

        (*tcinfo).handle_array = tmp;
        (*tcinfo).array_len += 16;
    }
    (*(*tcinfo).handle_array.add((*tcinfo).used_len as usize)).handle = (*info).tcm_handle as c_int;
    snprintf(
        (*(*tcinfo).handle_array.add((*tcinfo).used_len as usize)).kind.as_mut_ptr(),
        size_of::<[c_char; 64]>(),
        b"%s\0".as_ptr() as *const c_char,
        if !(*tb.add(TCA_KIND)).is_null() {
            libbpf_nla_getattr_str(*tb.add(TCA_KIND))
        } else {
            b"unknown\0".as_ptr() as *const c_char
        },
    );
    (*tcinfo).used_len += 1;

    0
}

unsafe extern "C" fn dump_filter_nlmsg(
    cookie: *mut c_void,
    msg: *mut c_void,
    tb: *mut *mut nlattr,
) -> c_int {
    let filter_info = cookie as *const bpf_filter_t;

    do_filter_dump(
        msg as *mut tcmsg,
        tb,
        (*filter_info).kind,
        (*filter_info).devname,
        (*filter_info).ifindex,
    )
}

unsafe extern "C" fn __show_dev_tc_bpf_name(id: __u32, name: *mut c_char, len: size_t) -> c_int {
    let mut info: bpf_prog_info = zeroed();
    let mut ilen: __u32 = size_of::<bpf_prog_info>() as __u32;
    let fd: c_int;
    let mut ret: c_int;

    fd = bpf_prog_get_fd_by_id(id);
    if fd < 0 {
        return fd;
    }
    ret = bpf_obj_get_info_by_fd(fd, &mut info as *mut _ as *mut c_void, &mut ilen);
    if ret < 0 {
        close(fd);
        return ret;
    }
    ret = -ENOENT;
    if info.name[0] != 0 {
        get_prog_full_name(&mut info, fd, name, len);
        ret = 0;
    }
    close(fd);
    ret
}

unsafe extern "C" fn __show_dev_tc_bpf(dev: *const ip_devname_ifindex, loc: c_int) {
    let mut prog_flags: [__u32; 64] = [0; 64];
    let mut link_flags: [__u32; 64] = [0; 64];
    let mut prog_ids: [__u32; 64] = [0; 64];
    let mut link_ids: [__u32; 64] = [0; 64];
    let mut i: __u32;
    let mut j: __u32;
    let mut optq: bpf_prog_query_opts = zeroed();
    let mut prog_name: [c_char; MAX_PROG_FULL_NAME] = [0; MAX_PROG_FULL_NAME];
    let mut ret: c_int;

    optq.sz = size_of::<bpf_prog_query_opts>();
    optq.prog_ids = prog_ids.as_mut_ptr();
    optq.prog_attach_flags = prog_flags.as_mut_ptr();
    optq.link_ids = link_ids.as_mut_ptr();
    optq.link_attach_flags = link_flags.as_mut_ptr();
    optq.count = prog_ids.len() as __u32;

    ret = bpf_prog_query_opts((*dev).ifindex, loc, &mut optq);
    if ret != 0 {
        return;
    }
    i = 0;
    while i < optq.count {
        net_start_object();
        net_dump_str(b"devname\0".as_ptr() as *const c_char, b"%s\0".as_ptr() as *const c_char, (*dev).devname.as_ptr());
        net_dump_uint(b"ifindex\0".as_ptr() as *const c_char, b"(%u)\0".as_ptr() as *const c_char, (*dev).ifindex as c_uint);
        net_dump_str(b"kind\0".as_ptr() as *const c_char, b" %s\0".as_ptr() as *const c_char, attach_loc_strings[loc as usize]);
        ret = __show_dev_tc_bpf_name(prog_ids[i as usize], prog_name.as_mut_ptr(), prog_name.len());
        if ret == 0 {
            net_dump_str(b"name\0".as_ptr() as *const c_char, b" %s\0".as_ptr() as *const c_char, prog_name.as_ptr());
        }
        net_dump_uint(b"prog_id\0".as_ptr() as *const c_char, b" prog_id %u \0".as_ptr() as *const c_char, prog_ids[i as usize]);
        if prog_flags[i as usize] != 0 || json_output {
            net_start_array(b"prog_flags\0".as_ptr() as *const c_char, b"%s \0".as_ptr() as *const c_char);
            j = 0;
            while prog_flags[i as usize] != 0 && j < 32 {
                if (prog_flags[i as usize] & (1u32 << j)) != 0 {
                    net_dump_uint_only(1u32 << j);
                }
                j += 1;
            }
            net_end_array(b"\0".as_ptr() as *const c_char);
        }
        if link_ids[i as usize] != 0 || json_output {
            net_dump_uint(b"link_id\0".as_ptr() as *const c_char, b"link_id %u \0".as_ptr() as *const c_char, link_ids[i as usize]);
            if link_flags[i as usize] != 0 || json_output {
                net_start_array(b"link_flags\0".as_ptr() as *const c_char, b"%s \0".as_ptr() as *const c_char);
                j = 0;
                while link_flags[i as usize] != 0 && j < 32 {
                    if (link_flags[i as usize] & (1u32 << j)) != 0 {
                        net_dump_uint_only(1u32 << j);
                    }
                    j += 1;
                }
                net_end_array(b"\0".as_ptr() as *const c_char);
            }
        }
        net_end_object_final();
        i += 1;
    }
}

unsafe extern "C" fn show_dev_tc_bpf(dev: *mut ip_devname_ifindex) {
    __show_dev_tc_bpf(dev, BPF_TCX_INGRESS);
    __show_dev_tc_bpf(dev, BPF_TCX_EGRESS);
    __show_dev_tc_bpf(dev, BPF_NETKIT_PRIMARY);
    __show_dev_tc_bpf(dev, BPF_NETKIT_PEER);
}

unsafe extern "C" fn show_dev_tc_bpf_classic(
    sock: c_int,
    nl_pid: c_uint,
    dev: *mut ip_devname_ifindex,
) -> c_int {
    let mut filter_info: bpf_filter_t = zeroed();
    let mut tcinfo: bpf_tcinfo_t = zeroed();
    let mut i: c_int;
    let mut handle: c_int;
    let mut ret: c_int = 0;

    tcinfo.handle_array = ptr::null_mut();
    tcinfo.used_len = 0;
    tcinfo.array_len = 0;

    tcinfo.is_qdisc = false;
    ret = netlink_get_class(sock, nl_pid, (*dev).ifindex, Some(dump_class_qdisc_nlmsg), &mut tcinfo as *mut _ as *mut c_void);
    if ret != 0 {
        free(tcinfo.handle_array as *mut c_void);
        return 0;
    }

    tcinfo.is_qdisc = true;
    ret = netlink_get_qdisc(sock, nl_pid, (*dev).ifindex, Some(dump_class_qdisc_nlmsg), &mut tcinfo as *mut _ as *mut c_void);
    if ret != 0 {
        free(tcinfo.handle_array as *mut c_void);
        return 0;
    }

    filter_info.devname = (*dev).devname.as_ptr();
    filter_info.ifindex = (*dev).ifindex;
    i = 0;
    while i < tcinfo.used_len {
        filter_info.kind = (*tcinfo.handle_array.add(i as usize)).kind.as_ptr();
        ret = netlink_get_filter(sock, nl_pid, (*dev).ifindex, (*tcinfo.handle_array.add(i as usize)).handle, Some(dump_filter_nlmsg), &mut filter_info as *mut _ as *mut c_void);
        if ret != 0 {
            free(tcinfo.handle_array as *mut c_void);
            return 0;
        }
        i += 1;
    }

    /* root, ingress and egress handle */
    handle = TC_H_ROOT;
    filter_info.kind = b"root\0".as_ptr() as *const c_char;
    ret = netlink_get_filter(sock, nl_pid, (*dev).ifindex, handle, Some(dump_filter_nlmsg), &mut filter_info as *mut _ as *mut c_void);
    if ret != 0 {
        free(tcinfo.handle_array as *mut c_void);
        return 0;
    }

    handle = tc_h_make(TC_H_CLSACT, TC_H_MIN_INGRESS);
    filter_info.kind = b"clsact/ingress\0".as_ptr() as *const c_char;
    ret = netlink_get_filter(sock, nl_pid, (*dev).ifindex, handle, Some(dump_filter_nlmsg), &mut filter_info as *mut _ as *mut c_void);
    if ret != 0 {
        free(tcinfo.handle_array as *mut c_void);
        return 0;
    }

    handle = tc_h_make(TC_H_CLSACT, TC_H_MIN_EGRESS);
    filter_info.kind = b"clsact/egress\0".as_ptr() as *const c_char;
    ret = netlink_get_filter(sock, nl_pid, (*dev).ifindex, handle, Some(dump_filter_nlmsg), &mut filter_info as *mut _ as *mut c_void);
    if ret != 0 {
        free(tcinfo.handle_array as *mut c_void);
        return 0;
    }

    free(tcinfo.handle_array as *mut c_void);
    0
}

unsafe extern "C" fn query_flow_dissector(attach_info: *mut bpf_attach_info) -> c_int {
    let mut attach_flags: __u32 = 0;
    let mut prog_ids: [__u32; 1] = [0; 1];
    let mut prog_cnt: __u32;
    let mut err: c_int;
    let fd: c_int;

    fd = open(b"/proc/self/ns/net\0".as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        p_err(b"can't open /proc/self/ns/net: %s\0".as_ptr() as *const c_char, strerror(errno));
        return -1;
    }
    prog_cnt = prog_ids.len() as __u32;
    err = bpf_prog_query(fd, BPF_FLOW_DISSECTOR, 0, &mut attach_flags, prog_ids.as_mut_ptr(), &mut prog_cnt);
    close(fd);
    if err != 0 {
        if err == -EINVAL {
            /* Older kernel's don't support querying
             * flow dissector programs.
             */
            errno = 0;
            return 0;
        }
        p_err(b"can't query prog: %s\0".as_ptr() as *const c_char, strerror(-err));
        return -1;
    }

    if prog_cnt == 1 {
        (*attach_info).flow_dissector_id = prog_ids[0];
    }

    0
}

unsafe extern "C" fn net_parse_dev(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    let ifindex: c_int;

    if is_prefix(**argv, b"dev\0".as_ptr() as *const c_char) {
        *argv = (*argv).add(1);
        *argc -= 1;

        ifindex = if_nametoindex(**argv) as c_int;
        if ifindex == 0 {
            p_err(b"invalid devname %s\0".as_ptr() as *const c_char, **argv);
        }

        *argv = (*argv).add(1);
        *argc -= 1;
    } else {
        p_err(b"expected 'dev', got: '%s'?\0".as_ptr() as *const c_char, **argv);
        return -1;
    }

    ifindex
}

unsafe extern "C" fn do_attach_detach_xdp(
    progfd: c_int,
    attach_type: size_t,
    ifindex: c_int,
    overwrite: bool,
) -> c_int {
    let mut flags: __u32 = 0;

    if !overwrite {
        flags = XDP_FLAGS_UPDATE_IF_NOEXIST;
    }
    if attach_type == net_attach_type::NET_ATTACH_TYPE_XDP_GENERIC as size_t {
        flags |= XDP_FLAGS_SKB_MODE;
    }
    if attach_type == net_attach_type::NET_ATTACH_TYPE_XDP_DRIVER as size_t {
        flags |= XDP_FLAGS_DRV_MODE;
    }
    if attach_type == net_attach_type::NET_ATTACH_TYPE_XDP_OFFLOAD as size_t {
        flags |= XDP_FLAGS_HW_MODE;
    }

    bpf_xdp_attach(ifindex, progfd, flags, ptr::null())
}

unsafe extern "C" fn get_tcx_type(attach_type: size_t) -> c_int {
    match attach_type {
        x if x == net_attach_type::NET_ATTACH_TYPE_TCX_INGRESS as size_t => BPF_TCX_INGRESS,
        x if x == net_attach_type::NET_ATTACH_TYPE_TCX_EGRESS as size_t => BPF_TCX_EGRESS,
        _ => -1,
    }
}

unsafe extern "C" fn do_attach_tcx(
    progfd: c_int,
    attach_type: size_t,
    ifindex: c_int,
    prepend: bool,
) -> c_int {
    let type_ = get_tcx_type(attach_type);

    if prepend {
        let mut opts: bpf_prog_attach_opts = zeroed();
        opts.sz = size_of::<bpf_prog_attach_opts>();
        opts.flags = BPF_F_BEFORE;
        return bpf_prog_attach_opts(progfd, ifindex, type_, &mut opts);
    }
    bpf_prog_attach(progfd, ifindex, type_, 0)
}

unsafe extern "C" fn do_detach_tcx(targetfd: c_int, attach_type: size_t) -> c_int {
    let type_ = get_tcx_type(attach_type);

    bpf_prog_detach(targetfd, type_)
}

unsafe extern "C" fn do_attach(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let attach_type: size_t;
    let progfd: c_int;
    let ifindex: c_int;
    let mut err: c_int = 0;
    let mut overwrite = false;
    let mut prepend = false;

    /* parse attach args */
    if argc < 5 {
        return -EINVAL;
    }

    attach_type = parse_attach_type(*argv);
    if attach_type == net_attach_type_size {
        p_err(b"invalid net attach/detach type: %s\0".as_ptr() as *const c_char, *argv);
        return -EINVAL;
    }
    argv = argv.add(1);
    argc -= 1;

    progfd = prog_parse_fd(&mut argc, &mut argv);
    if progfd < 0 {
        return -EINVAL;
    }

    ifindex = net_parse_dev(&mut argc, &mut argv);
    if ifindex < 1 {
        err = -EINVAL;
        close(progfd);
        return err;
    }

    if argc != 0 {
        if is_prefix(*argv, b"overwrite\0".as_ptr() as *const c_char) {
            if attach_type != net_attach_type::NET_ATTACH_TYPE_XDP as size_t
                && attach_type != net_attach_type::NET_ATTACH_TYPE_XDP_GENERIC as size_t
                && attach_type != net_attach_type::NET_ATTACH_TYPE_XDP_DRIVER as size_t
                && attach_type != net_attach_type::NET_ATTACH_TYPE_XDP_OFFLOAD as size_t
            {
                p_err(b"'overwrite' is only supported for xdp types\0".as_ptr() as *const c_char);
                err = -EINVAL;
                close(progfd);
                return err;
            }
            overwrite = true;
        } else if is_prefix(*argv, b"prepend\0".as_ptr() as *const c_char) {
            if attach_type != net_attach_type::NET_ATTACH_TYPE_TCX_INGRESS as size_t
                && attach_type != net_attach_type::NET_ATTACH_TYPE_TCX_EGRESS as size_t
            {
                p_err(b"'prepend' is only supported for tcx_ingress/tcx_egress\0".as_ptr() as *const c_char);
                err = -EINVAL;
                close(progfd);
                return err;
            }
            prepend = true;
        } else {
            p_err(b"expected 'overwrite' or 'prepend', got: '%s'?\0".as_ptr() as *const c_char, *argv);
            err = -EINVAL;
            close(progfd);
            return err;
        }
    }

    match attach_type {
        x if x == net_attach_type::NET_ATTACH_TYPE_XDP as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_GENERIC as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_DRIVER as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_OFFLOAD as size_t =>
        {
            err = do_attach_detach_xdp(progfd, attach_type, ifindex, overwrite);
        }
        x if x == net_attach_type::NET_ATTACH_TYPE_TCX_INGRESS as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_TCX_EGRESS as size_t =>
        {
            err = do_attach_tcx(progfd, attach_type, ifindex, prepend);
        }
        _ => {}
    }

    if err != 0 {
        p_err(b"interface %s attach failed: %s\0".as_ptr() as *const c_char, attach_type_strings[attach_type], strerror(-err));
        close(progfd);
        return err;
    }

    if json_output {
        jsonw_null(json_wtr);
    }
    close(progfd);
    err
}

unsafe extern "C" fn do_detach(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let attach_type: size_t;
    let progfd: c_int;
    let ifindex: c_int;
    let mut err: c_int = 0;

    /* parse detach args */
    if argc < 3 {
        return -EINVAL;
    }

    attach_type = parse_attach_type(*argv);
    if attach_type == net_attach_type_size {
        p_err(b"invalid net attach/detach type: %s\0".as_ptr() as *const c_char, *argv);
        return -EINVAL;
    }
    argv = argv.add(1);
    argc -= 1;

    ifindex = net_parse_dev(&mut argc, &mut argv);
    if ifindex < 1 {
        return -EINVAL;
    }

    match attach_type {
        x if x == net_attach_type::NET_ATTACH_TYPE_XDP as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_GENERIC as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_DRIVER as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_XDP_OFFLOAD as size_t =>
        {
            progfd = -1;
            err = do_attach_detach_xdp(progfd, attach_type, ifindex, false);
        }
        x if x == net_attach_type::NET_ATTACH_TYPE_TCX_INGRESS as size_t
            || x == net_attach_type::NET_ATTACH_TYPE_TCX_EGRESS as size_t =>
        {
            err = do_detach_tcx(ifindex, attach_type);
        }
        _ => {}
    }

    if err < 0 {
        p_err(b"interface %s detach failed: %s\0".as_ptr() as *const c_char, attach_type_strings[attach_type], strerror(-err));
        return err;
    }

    if json_output {
        jsonw_null(json_wtr);
    }

    0
}

unsafe extern "C" fn netfilter_link_compar(a: *const c_void, b: *const c_void) -> c_int {
    let nfa = a as *const bpf_link_info;
    let nfb = b as *const bpf_link_info;
    let mut delta: c_int;

    delta = (*nfa).netfilter.pf as c_int - (*nfb).netfilter.pf as c_int;
    if delta != 0 {
        return delta;
    }

    delta = (*nfa).netfilter.hooknum as c_int - (*nfb).netfilter.hooknum as c_int;
    if delta != 0 {
        return delta;
    }

    if (*nfa).netfilter.priority < (*nfb).netfilter.priority {
        return -1;
    }
    if (*nfa).netfilter.priority > (*nfb).netfilter.priority {
        return 1;
    }

    (*nfa).netfilter.flags as c_int - (*nfb).netfilter.flags as c_int
}

unsafe extern "C" fn show_link_netfilter() {
    let mut nf_link_len: c_uint = 0;
    let mut nf_link_count: c_uint = 0;
    let mut nf_link_info: *mut bpf_link_info = ptr::null_mut();
    let mut id: __u32 = 0;

    loop {
        let mut info: bpf_link_info = zeroed();
        let fd: c_int;
        let mut err: c_int;
        let mut len: __u32;

        err = bpf_link_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT {
                break;
            }
            p_err(b"can't get next link: %s (id %u)\0".as_ptr() as *const c_char, strerror(errno), id);
            break;
        }

        fd = bpf_link_get_fd_by_id(id);
        if fd < 0 {
            p_err(b"can't get link by id (%u): %s\0".as_ptr() as *const c_char, id, strerror(errno));
            continue;
        }

        len = size_of::<bpf_link_info>() as __u32;

        err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);

        close(fd);

        if err != 0 {
            p_err(b"can't get link info for fd %d: %s\0".as_ptr() as *const c_char, fd, strerror(errno));
            continue;
        }

        if info.type_ != BPF_LINK_TYPE_NETFILTER {
            continue;
        }

        if nf_link_count >= nf_link_len {
            let max_link_count: c_uint = INT_MAX / size_of::<bpf_link_info>() as c_uint;
            let expand: *mut bpf_link_info;

            if nf_link_count > max_link_count {
                p_err(b"cannot handle more than %u links\n\0".as_ptr() as *const c_char, max_link_count);
                break;
            }

            nf_link_len += 16;

            expand = realloc(
                nf_link_info as *mut c_void,
                nf_link_len as size_t * size_of::<bpf_link_info>(),
            ) as *mut bpf_link_info;
            if expand.is_null() {
                p_err(b"realloc: %s\0".as_ptr() as *const c_char, strerror(errno));
                break;
            }

            nf_link_info = expand;
        }

        *nf_link_info.add(nf_link_count as usize) = info;
        nf_link_count += 1;
    }

    if nf_link_info.is_null() {
        return;
    }

    qsort(
        nf_link_info as *mut c_void,
        nf_link_count as size_t,
        size_of::<bpf_link_info>(),
        Some(netfilter_link_compar),
    );

    id = 0;
    while id < nf_link_count {
        net_start_object();
        if json_output {
            netfilter_dump_json(nf_link_info.add(id as usize), json_wtr);
        } else {
            netfilter_dump_plain(nf_link_info.add(id as usize));
        }

        net_dump_uint(b"id\0".as_ptr() as *const c_char, b" prog_id %u\0".as_ptr() as *const c_char, (*nf_link_info.add(id as usize)).prog_id);
        net_end_object();
        id += 1;
    }

    free(nf_link_info as *mut c_void);
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut attach_info: bpf_attach_info = zeroed();
    let mut i: c_int;
    let sock: c_int;
    let mut ret: c_int;
    let mut filter_idx: c_int = -1;
    let mut dev_array: bpf_netdev_t = zeroed();
    let mut nl_pid: c_uint = 0;
    let mut err_buf: [c_char; 256] = [0; 256];

    if argc == 2 {
        filter_idx = net_parse_dev(&mut argc, &mut argv);
        if filter_idx < 1 {
            return -1;
        }
    } else if argc != 0 {
        usage();
    }

    ret = query_flow_dissector(&mut attach_info);
    if ret != 0 {
        return -1;
    }

    sock = netlink_open(&mut nl_pid);
    if sock < 0 {
        fprintf(stderr, b"failed to open netlink sock\n\0".as_ptr() as *const c_char);
        return -1;
    }

    dev_array.devices = ptr::null_mut();
    dev_array.used_len = 0;
    dev_array.array_len = 0;
    dev_array.filter_idx = filter_idx;

    if json_output {
        jsonw_start_array(json_wtr);
    }
    net_start_object();
    net_start_array(b"xdp\0".as_ptr() as *const c_char, b"%s:\n\0".as_ptr() as *const c_char);
    ret = netlink_get_link(sock, nl_pid, Some(dump_link_nlmsg), &mut dev_array as *mut _ as *mut c_void);
    net_end_array(b"\n\0".as_ptr() as *const c_char);

    if ret == 0 {
        net_start_array(b"tc\0".as_ptr() as *const c_char, b"%s:\n\0".as_ptr() as *const c_char);
        i = 0;
        while i < dev_array.used_len {
            show_dev_tc_bpf(dev_array.devices.add(i as usize));
            ret = show_dev_tc_bpf_classic(sock, nl_pid, dev_array.devices.add(i as usize));
            if ret != 0 {
                break;
            }
            i += 1;
        }
        net_end_array(b"\n\0".as_ptr() as *const c_char);
    }

    net_start_array(b"flow_dissector\0".as_ptr() as *const c_char, b"%s:\n\0".as_ptr() as *const c_char);
    if attach_info.flow_dissector_id > 0 {
        net_dump_uint(b"id\0".as_ptr() as *const c_char, b"id %u\0".as_ptr() as *const c_char, attach_info.flow_dissector_id);
    }
    net_end_array(b"\n\0".as_ptr() as *const c_char);

    net_start_array(b"netfilter\0".as_ptr() as *const c_char, b"%s:\n\0".as_ptr() as *const c_char);
    show_link_netfilter();
    net_end_array(b"\n\0".as_ptr() as *const c_char);

    net_end_object();
    if json_output {
        jsonw_end_array(json_wtr);
    }

    if ret != 0 {
        if json_output {
            jsonw_null(json_wtr);
        }
        libbpf_strerror(ret, err_buf.as_mut_ptr(), err_buf.len());
        fprintf(stderr, b"Error: %s\n\0".as_ptr() as *const c_char, err_buf.as_ptr());
    }
    free(dev_array.devices as *mut c_void);
    close(sock);
    ret
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        b"Usage: %1$s %2$s { show | list } [dev <devname>]\n       %1$s %2$s attach ATTACH_TYPE PROG dev <devname> [ overwrite | prepend ]\n       %1$s %2$s detach ATTACH_TYPE dev <devname>\n       %1$s %2$s help\n\n       HELP_SPEC_PROGRAM\n       ATTACH_TYPE := { xdp | xdpgeneric | xdpdrv | xdpoffload | tcx_ingress\n                        | tcx_egress }\n       HELP_SPEC_OPTIONS }\n\nNote: Only xdp, tcx, tc, netkit, flow_dissector and netfilter attachments\n      are currently supported.\n      For progs attached to cgroups, use \"bpftool cgroup\"\n      to dump program attachments. For program types\n      sk_{filter,skb,msg,reuseport} and lwt/seg6, please\n      consult iproute2.\n\0".as_ptr() as *const c_char,
        bin_name,
        *argv.offset(-2),
    );

    0
}

static CMD_SHOW: &[u8] = b"show\0";
static CMD_LIST: &[u8] = b"list\0";
static CMD_ATTACH: &[u8] = b"attach\0";
static CMD_DETACH: &[u8] = b"detach\0";
static CMD_HELP: &[u8] = b"help\0";

static cmds: [cmd; 6] = [
    cmd { cmd: CMD_SHOW.as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: CMD_LIST.as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: CMD_ATTACH.as_ptr() as *const c_char, func: Some(do_attach) },
    cmd { cmd: CMD_DETACH.as_ptr() as *const c_char, func: Some(do_detach) },
    cmd { cmd: CMD_HELP.as_ptr() as *const c_char, func: Some(do_help) },
    cmd { cmd: ptr::null(), func: None },
];

#[no_mangle]
pub unsafe extern "C" fn do_net(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(cmds.as_ptr(), argc, argv, Some(do_help))
}
