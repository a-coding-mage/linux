// SPDX-License-Identifier: GPL-2.0

// C dependencies: time.h, libmnl/libmnl.h, netinet/ip.h,
// linux/netlink.h, linux/netfilter/nfnetlink.h,
// linux/netfilter/nfnetlink_conntrack.h, linux/netfilter/nf_conntrack_tcp.h,
// kselftest_harness.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

const TEST_ZONE_ID: u16 = 123;
const NF_CT_DEFAULT_ZONE_ID: u16 = 0;

const MNL_SOCKET_BUFFER_SIZE: usize = 8192;
const MNL_SOCKET_AUTOPID: c_uint = 0;
const MNL_CB_OK: c_int = 1;
const MNL_CB_STOP: c_int = 0;

const NETLINK_NETFILTER: c_int = 12;
const AF_UNSPEC: u8 = 0;
const AF_INET: u8 = 2;
const AF_INET6: u8 = 10;

const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_ACK: u16 = 0x04;
const NLM_F_EXCL: u16 = 0x200;
const NLM_F_CREATE: u16 = 0x400;
const NLM_F_DUMP: u16 = 0x300;

const NFNL_SUBSYS_CTNETLINK: u16 = 1;
const IPCTNL_MSG_CT_NEW: u16 = 0;
const IPCTNL_MSG_CT_GET: u16 = 1;
const IPCTNL_MSG_CT_DELETE: u16 = 2;
const NFNETLINK_V0: u8 = 0;

const CTA_TUPLE_ORIG: c_int = 1;
const CTA_TUPLE_REPLY: c_int = 2;
const CTA_TIMEOUT: c_int = 7;
const CTA_PROTOINFO: c_int = 8;
const CTA_ZONE: c_int = 18;

const CTA_TUPLE_IP: c_int = 1;
const CTA_TUPLE_PROTO: c_int = 2;

const CTA_IP_V4_SRC: c_int = 1;
const CTA_IP_V4_DST: c_int = 2;
const CTA_IP_V6_SRC: c_int = 3;
const CTA_IP_V6_DST: c_int = 4;

const CTA_PROTO_NUM: c_int = 1;
const CTA_PROTO_SRC_PORT: c_int = 2;
const CTA_PROTO_DST_PORT: c_int = 3;

const CTA_PROTOINFO_TCP: c_int = 1;
const CTA_PROTOINFO_TCP_STATE: c_int = 1;
const CTA_PROTOINFO_TCP_FLAGS_ORIGINAL: c_int = 2;
const CTA_PROTOINFO_TCP_FLAGS_REPLY: c_int = 3;

const TCP_CONNTRACK_ESTABLISHED: u8 = 3;

const EEXIST: c_int = 17;
const EPERM: c_int = 1;
const EOPNOTSUPP: c_int = 95;

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
struct nfgenmsg {
    nfgen_family: u8,
    version: u8,
    res_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    __u6_addr32: [u32; 4],
}

#[repr(C)]
struct mnl_socket {
    _private: [u8; 0],
}

type MnlCb = Option<unsafe extern "C" fn(*const nlmsghdr, *mut c_void) -> c_int>;

unsafe extern "C" {
    fn mnl_attr_nest_start(nlh: *mut nlmsghdr, type_: c_int) -> *mut nlattr;
    fn mnl_attr_nest_end(nlh: *mut nlmsghdr, start: *mut nlattr);
    fn mnl_attr_put_u32(nlh: *mut nlmsghdr, type_: c_int, data: u32);
    fn mnl_attr_put_u16(nlh: *mut nlmsghdr, type_: c_int, data: u16);
    fn mnl_attr_put_u8(nlh: *mut nlmsghdr, type_: c_int, data: u8);
    fn mnl_attr_put(nlh: *mut nlmsghdr, type_: c_int, len: usize, data: *const c_void);
    fn mnl_socket_get_portid(nl: *const mnl_socket) -> c_uint;
    fn mnl_socket_sendto(nl: *const mnl_socket, buf: *const c_void, len: usize) -> c_int;
    fn mnl_socket_recvfrom(nl: *const mnl_socket, buf: *mut c_void, bufsiz: usize) -> c_int;
    fn mnl_cb_run(
        buf: *const c_void,
        numbytes: c_int,
        seq: c_uint,
        portid: c_uint,
        cb_data: MnlCb,
        data: *mut c_void,
    ) -> c_int;
    fn mnl_nlmsg_put_header(buf: *mut c_void) -> *mut nlmsghdr;
    fn mnl_nlmsg_put_extra_header(nlh: *mut nlmsghdr, size: usize) -> *mut c_void;
    fn mnl_socket_open(bus: c_int) -> *mut mnl_socket;
    fn mnl_socket_bind(nl: *mut mnl_socket, groups: c_uint, pid: c_uint) -> c_int;
    fn perror(s: *const c_char);
    fn time(tloc: *mut isize) -> isize;
}

unsafe extern "C" {
    static mut errno: c_int;
}

static mut reply_counter: c_int = 0;

const fn htons(hostshort: u16) -> u16 {
    hostshort.to_be()
}

const fn htonl(hostlong: u32) -> u32 {
    hostlong.to_be()
}

unsafe fn build_cta_tuple_v4(
    nlh: *mut nlmsghdr,
    type_: c_int,
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
) -> c_int {
    let nest: *mut nlattr;
    let nest_ip: *mut nlattr;
    let nest_proto: *mut nlattr;

    nest = unsafe { mnl_attr_nest_start(nlh, type_) };
    if nest.is_null() {
        return -1;
    }

    nest_ip = unsafe { mnl_attr_nest_start(nlh, CTA_TUPLE_IP) };
    if nest_ip.is_null() {
        return -1;
    }
    unsafe { mnl_attr_put_u32(nlh, CTA_IP_V4_SRC, src_ip) };
    unsafe { mnl_attr_put_u32(nlh, CTA_IP_V4_DST, dst_ip) };
    unsafe { mnl_attr_nest_end(nlh, nest_ip) };

    nest_proto = unsafe { mnl_attr_nest_start(nlh, CTA_TUPLE_PROTO) };
    if nest_proto.is_null() {
        return -1;
    }
    unsafe { mnl_attr_put_u8(nlh, CTA_PROTO_NUM, 6) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTO_SRC_PORT, htons(src_port)) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTO_DST_PORT, htons(dst_port)) };
    unsafe { mnl_attr_nest_end(nlh, nest_proto) };

    unsafe { mnl_attr_nest_end(nlh, nest) };

    0
}

unsafe fn build_cta_tuple_v6(
    nlh: *mut nlmsghdr,
    type_: c_int,
    src_ip: in6_addr,
    dst_ip: in6_addr,
    src_port: u16,
    dst_port: u16,
) -> c_int {
    let nest: *mut nlattr;
    let nest_ip: *mut nlattr;
    let nest_proto: *mut nlattr;

    nest = unsafe { mnl_attr_nest_start(nlh, type_) };
    if nest.is_null() {
        return -1;
    }

    nest_ip = unsafe { mnl_attr_nest_start(nlh, CTA_TUPLE_IP) };
    if nest_ip.is_null() {
        return -1;
    }
    unsafe {
        mnl_attr_put(
            nlh,
            CTA_IP_V6_SRC,
            core::mem::size_of::<in6_addr>(),
            &src_ip as *const in6_addr as *const c_void,
        )
    };
    unsafe {
        mnl_attr_put(
            nlh,
            CTA_IP_V6_DST,
            core::mem::size_of::<in6_addr>(),
            &dst_ip as *const in6_addr as *const c_void,
        )
    };
    unsafe { mnl_attr_nest_end(nlh, nest_ip) };

    nest_proto = unsafe { mnl_attr_nest_start(nlh, CTA_TUPLE_PROTO) };
    if nest_proto.is_null() {
        return -1;
    }
    unsafe { mnl_attr_put_u8(nlh, CTA_PROTO_NUM, 6) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTO_SRC_PORT, htons(src_port)) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTO_DST_PORT, htons(dst_port)) };
    unsafe { mnl_attr_nest_end(nlh, nest_proto) };

    unsafe { mnl_attr_nest_end(nlh, nest) };

    0
}

unsafe fn build_cta_proto(nlh: *mut nlmsghdr) -> c_int {
    let nest: *mut nlattr;
    let nest_proto: *mut nlattr;

    nest = unsafe { mnl_attr_nest_start(nlh, CTA_PROTOINFO) };
    if nest.is_null() {
        return -1;
    }

    nest_proto = unsafe { mnl_attr_nest_start(nlh, CTA_PROTOINFO_TCP) };
    if nest_proto.is_null() {
        return -1;
    }
    unsafe { mnl_attr_put_u8(nlh, CTA_PROTOINFO_TCP_STATE, TCP_CONNTRACK_ESTABLISHED) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTOINFO_TCP_FLAGS_ORIGINAL, 0x0a0a) };
    unsafe { mnl_attr_put_u16(nlh, CTA_PROTOINFO_TCP_FLAGS_REPLY, 0x0a0a) };
    unsafe { mnl_attr_nest_end(nlh, nest_proto) };

    unsafe { mnl_attr_nest_end(nlh, nest) };

    0
}

unsafe fn conntrack_data_insert(sock: *mut mnl_socket, nlh: *mut nlmsghdr, zone: u16) -> c_int {
    let mut buf = [0 as c_char; MNL_SOCKET_BUFFER_SIZE];
    let portid: c_uint;
    let mut ret: c_int;

    portid = unsafe { mnl_socket_get_portid(sock) };

    ret = unsafe { build_cta_proto(nlh) };
    if ret < 0 {
        unsafe { perror(c"build_cta_proto".as_ptr()) };
        return -1;
    }
    unsafe { mnl_attr_put_u32(nlh, CTA_TIMEOUT, htonl(20000)) };
    unsafe { mnl_attr_put_u16(nlh, CTA_ZONE, htons(zone)) };

    if unsafe { mnl_socket_sendto(sock, nlh as *const c_void, (*nlh).nlmsg_len as usize) } < 0 {
        unsafe { perror(c"mnl_socket_sendto".as_ptr()) };
        return -1;
    }

    ret = unsafe { mnl_socket_recvfrom(sock, buf.as_mut_ptr() as *mut c_void, MNL_SOCKET_BUFFER_SIZE) };
    if ret < 0 {
        unsafe { perror(c"mnl_socket_recvfrom".as_ptr()) };
        return ret;
    }

    ret = unsafe {
        mnl_cb_run(
            buf.as_ptr() as *const c_void,
            ret,
            (*nlh).nlmsg_seq,
            portid,
            None,
            core::ptr::null_mut(),
        )
    };
    if ret < 0 {
        if unsafe { errno } == EEXIST {
            /* The entries are probably still there from a previous
             * run. So we are good
             */
            return 0;
        }
        unsafe { perror(c"mnl_cb_run".as_ptr()) };
        return ret;
    }

    0
}

unsafe fn conntrack_data_generate_v4(
    sock: *mut mnl_socket,
    src_ip: u32,
    dst_ip: u32,
    zone: u16,
) -> c_int {
    let mut buf = [0 as c_char; MNL_SOCKET_BUFFER_SIZE];
    let nlh: *mut nlmsghdr;
    let nfh: *mut nfgenmsg;
    let mut ret: c_int;

    nlh = unsafe { mnl_nlmsg_put_header(buf.as_mut_ptr() as *mut c_void) };
    unsafe {
        (*nlh).nlmsg_type = (NFNL_SUBSYS_CTNETLINK << 8) | IPCTNL_MSG_CT_NEW;
        (*nlh).nlmsg_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_ACK | NLM_F_EXCL;
        (*nlh).nlmsg_seq = time(core::ptr::null_mut()) as u32;
    }

    nfh = unsafe {
        mnl_nlmsg_put_extra_header(nlh, core::mem::size_of::<nfgenmsg>()) as *mut nfgenmsg
    };
    unsafe {
        (*nfh).nfgen_family = AF_INET;
        (*nfh).version = NFNETLINK_V0;
        (*nfh).res_id = 0;
    }

    ret = unsafe { build_cta_tuple_v4(nlh, CTA_TUPLE_ORIG, src_ip, dst_ip, 12345, 443) };
    if ret < 0 {
        unsafe { perror(c"build_cta_tuple_v4".as_ptr()) };
        return ret;
    }
    ret = unsafe { build_cta_tuple_v4(nlh, CTA_TUPLE_REPLY, dst_ip, src_ip, 443, 12345) };
    if ret < 0 {
        unsafe { perror(c"build_cta_tuple_v4".as_ptr()) };
        return ret;
    }
    unsafe { conntrack_data_insert(sock, nlh, zone) }
}

unsafe fn conntrack_data_generate_v6(
    sock: *mut mnl_socket,
    src_ip: in6_addr,
    dst_ip: in6_addr,
    zone: u16,
) -> c_int {
    let mut buf = [0 as c_char; MNL_SOCKET_BUFFER_SIZE];
    let nlh: *mut nlmsghdr;
    let nfh: *mut nfgenmsg;
    let mut ret: c_int;

    nlh = unsafe { mnl_nlmsg_put_header(buf.as_mut_ptr() as *mut c_void) };
    unsafe {
        (*nlh).nlmsg_type = (NFNL_SUBSYS_CTNETLINK << 8) | IPCTNL_MSG_CT_NEW;
        (*nlh).nlmsg_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_ACK | NLM_F_EXCL;
        (*nlh).nlmsg_seq = time(core::ptr::null_mut()) as u32;
    }

    nfh = unsafe {
        mnl_nlmsg_put_extra_header(nlh, core::mem::size_of::<nfgenmsg>()) as *mut nfgenmsg
    };
    unsafe {
        (*nfh).nfgen_family = AF_INET6;
        (*nfh).version = NFNETLINK_V0;
        (*nfh).res_id = 0;
    }

    ret = unsafe { build_cta_tuple_v6(nlh, CTA_TUPLE_ORIG, src_ip, dst_ip, 12345, 443) };
    if ret < 0 {
        unsafe { perror(c"build_cta_tuple_v6".as_ptr()) };
        return ret;
    }
    ret = unsafe { build_cta_tuple_v6(nlh, CTA_TUPLE_REPLY, dst_ip, src_ip, 12345, 443) };
    if ret < 0 {
        unsafe { perror(c"build_cta_tuple_v6".as_ptr()) };
        return ret;
    }
    unsafe { conntrack_data_insert(sock, nlh, zone) }
}

unsafe extern "C" fn count_entries(_nlh: *const nlmsghdr, _data: *mut c_void) -> c_int {
    unsafe {
        reply_counter += 1;
    }
    MNL_CB_OK
}

unsafe fn conntrack_count_zone(sock: *mut mnl_socket, zone: u16) -> c_int {
    let mut buf = [0 as c_char; MNL_SOCKET_BUFFER_SIZE];
    let nlh: *mut nlmsghdr;
    let nfh: *mut nfgenmsg;
    let portid: c_uint;
    let mut ret: c_int;

    portid = unsafe { mnl_socket_get_portid(sock) };

    nlh = unsafe { mnl_nlmsg_put_header(buf.as_mut_ptr() as *mut c_void) };
    unsafe {
        (*nlh).nlmsg_type = (NFNL_SUBSYS_CTNETLINK << 8) | IPCTNL_MSG_CT_GET;
        (*nlh).nlmsg_flags = NLM_F_REQUEST | NLM_F_DUMP;
        (*nlh).nlmsg_seq = time(core::ptr::null_mut()) as u32;
    }

    nfh = unsafe {
        mnl_nlmsg_put_extra_header(nlh, core::mem::size_of::<nfgenmsg>()) as *mut nfgenmsg
    };
    unsafe {
        (*nfh).nfgen_family = AF_UNSPEC;
        (*nfh).version = NFNETLINK_V0;
        (*nfh).res_id = 0;
    }

    unsafe { mnl_attr_put_u16(nlh, CTA_ZONE, htons(zone)) };

    ret = unsafe { mnl_socket_sendto(sock, nlh as *const c_void, (*nlh).nlmsg_len as usize) };
    if ret < 0 {
        unsafe { perror(c"mnl_socket_sendto".as_ptr()) };
        return ret;
    }

    unsafe {
        reply_counter = 0;
    }
    ret = unsafe { mnl_socket_recvfrom(sock, buf.as_mut_ptr() as *mut c_void, MNL_SOCKET_BUFFER_SIZE) };
    while ret > 0 {
        ret = unsafe {
            mnl_cb_run(
                buf.as_ptr() as *const c_void,
                ret,
                (*nlh).nlmsg_seq,
                portid,
                Some(count_entries),
                core::ptr::null_mut(),
            )
        };
        if ret <= MNL_CB_STOP {
            break;
        }

        ret = unsafe {
            mnl_socket_recvfrom(sock, buf.as_mut_ptr() as *mut c_void, MNL_SOCKET_BUFFER_SIZE)
        };
    }
    if ret < 0 {
        unsafe { perror(c"mnl_socket_recvfrom".as_ptr()) };
        return ret;
    }

    unsafe { reply_counter }
}

unsafe fn conntrack_flush_zone(sock: *mut mnl_socket, zone: u16) -> c_int {
    let mut buf = [0 as c_char; MNL_SOCKET_BUFFER_SIZE];
    let nlh: *mut nlmsghdr;
    let nfh: *mut nfgenmsg;
    let portid: c_uint;
    let mut ret: c_int;

    portid = unsafe { mnl_socket_get_portid(sock) };

    nlh = unsafe { mnl_nlmsg_put_header(buf.as_mut_ptr() as *mut c_void) };
    unsafe {
        (*nlh).nlmsg_type = (NFNL_SUBSYS_CTNETLINK << 8) | IPCTNL_MSG_CT_DELETE;
        (*nlh).nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        (*nlh).nlmsg_seq = time(core::ptr::null_mut()) as u32;
    }

    nfh = unsafe {
        mnl_nlmsg_put_extra_header(nlh, core::mem::size_of::<nfgenmsg>()) as *mut nfgenmsg
    };
    unsafe {
        (*nfh).nfgen_family = AF_UNSPEC;
        (*nfh).version = NFNETLINK_V0;
        (*nfh).res_id = 0;
    }

    unsafe { mnl_attr_put_u16(nlh, CTA_ZONE, htons(zone)) };

    ret = unsafe { mnl_socket_sendto(sock, nlh as *const c_void, (*nlh).nlmsg_len as usize) };
    if ret < 0 {
        unsafe { perror(c"mnl_socket_sendto".as_ptr()) };
        return ret;
    }

    ret = unsafe { mnl_socket_recvfrom(sock, buf.as_mut_ptr() as *mut c_void, MNL_SOCKET_BUFFER_SIZE) };
    if ret < 0 {
        unsafe { perror(c"mnl_socket_recvfrom".as_ptr()) };
        return ret;
    }

    ret = unsafe {
        mnl_cb_run(
            buf.as_ptr() as *const c_void,
            ret,
            (*nlh).nlmsg_seq,
            portid,
            None,
            core::ptr::null_mut(),
        )
    };
    if ret < 0 {
        unsafe { perror(c"mnl_cb_run".as_ptr()) };
        return ret;
    }

    0
}

#[repr(C)]
struct conntrack_dump_flush {
    sock: *mut mnl_socket,
}

unsafe fn conntrack_dump_flush_setup(self_: *mut conntrack_dump_flush) {
    let mut src: in6_addr;
    let mut dst: in6_addr;
    let mut ret: c_int;

    unsafe {
        (*self_).sock = mnl_socket_open(NETLINK_NETFILTER);
    }
    if unsafe { (*self_).sock.is_null() } {
        unsafe { perror(c"mnl_socket_open".as_ptr()) };
        // SKIP(return, "cannot open netlink_netfilter socket");
        return;
    }

    ret = unsafe { mnl_socket_bind((*self_).sock, 0, MNL_SOCKET_AUTOPID) };
    assert_eq!(ret, 0);

    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID) };
    if ret < 0 && unsafe { errno } == EPERM {
        // SKIP(return, "Needs to be run as root");
        return;
    } else if ret < 0 && unsafe { errno } == EOPNOTSUPP {
        // SKIP(return, "Kernel does not seem to support conntrack zones");
        return;
    }

    ret = unsafe { conntrack_data_generate_v4((*self_).sock, 0xf0f0f0f0, 0xf1f1f1f1, TEST_ZONE_ID) };
    assert_eq!(ret, 0);
    ret = unsafe { conntrack_data_generate_v4((*self_).sock, 0xf2f2f2f2, 0xf3f3f3f3, TEST_ZONE_ID + 1) };
    assert_eq!(ret, 0);
    ret = unsafe { conntrack_data_generate_v4((*self_).sock, 0xf4f4f4f4, 0xf5f5f5f5, TEST_ZONE_ID + 2) };
    assert_eq!(ret, 0);
    ret = unsafe {
        conntrack_data_generate_v4((*self_).sock, 0xf6f6f6f6, 0xf7f7f7f7, NF_CT_DEFAULT_ZONE_ID)
    };
    assert_eq!(ret, 0);

    src = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x01000000],
    };
    dst = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x02000000],
    };
    ret = unsafe { conntrack_data_generate_v6((*self_).sock, src, dst, TEST_ZONE_ID) };
    assert_eq!(ret, 0);
    src = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x03000000],
    };
    dst = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x04000000],
    };
    ret = unsafe { conntrack_data_generate_v6((*self_).sock, src, dst, TEST_ZONE_ID + 1) };
    assert_eq!(ret, 0);
    src = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x05000000],
    };
    dst = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x06000000],
    };
    ret = unsafe { conntrack_data_generate_v6((*self_).sock, src, dst, TEST_ZONE_ID + 2) };
    assert_eq!(ret, 0);

    src = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x07000000],
    };
    dst = in6_addr {
        __u6_addr32: [0xb80d0120, 0x00000000, 0x00000000, 0x08000000],
    };
    ret = unsafe { conntrack_data_generate_v6((*self_).sock, src, dst, NF_CT_DEFAULT_ZONE_ID) };
    assert_eq!(ret, 0);

    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID) };
    assert!(ret >= 2);
    if ret > 2 {
        // SKIP(return, "kernel does not support filtering by zone");
        return;
    }
}

unsafe fn conntrack_dump_flush_teardown(_self: *mut conntrack_dump_flush) {}

unsafe fn test_dump_by_zone(self_: *mut conntrack_dump_flush) {
    let ret: c_int;

    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID) };
    assert_eq!(ret, 2);
}

unsafe fn test_flush_by_zone(self_: *mut conntrack_dump_flush) {
    let mut ret: c_int;

    ret = unsafe { conntrack_flush_zone((*self_).sock, TEST_ZONE_ID) };
    assert_eq!(ret, 0);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID) };
    assert_eq!(ret, 0);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID + 1) };
    assert_eq!(ret, 2);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID + 2) };
    assert_eq!(ret, 2);
    ret = unsafe { conntrack_count_zone((*self_).sock, NF_CT_DEFAULT_ZONE_ID) };
    assert_eq!(ret, 2);
}

unsafe fn test_flush_by_zone_default(self_: *mut conntrack_dump_flush) {
    let mut ret: c_int;

    ret = unsafe { conntrack_flush_zone((*self_).sock, NF_CT_DEFAULT_ZONE_ID) };
    assert_eq!(ret, 0);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID) };
    assert_eq!(ret, 2);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID + 1) };
    assert_eq!(ret, 2);
    ret = unsafe { conntrack_count_zone((*self_).sock, TEST_ZONE_ID + 2) };
    assert_eq!(ret, 2);
    ret = unsafe { conntrack_count_zone((*self_).sock, NF_CT_DEFAULT_ZONE_ID) };
    assert_eq!(ret, 0);
}

// TEST_HARNESS_MAIN
