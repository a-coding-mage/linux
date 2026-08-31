// SPDX-License-Identifier: GPL-2.0
/* Copyright 2026 Google LLC */

/*
 * Translated from C. The original file included Linux and kselftest headers:
 * linux/if.h, linux/in6.h, linux/mroute.h, linux/mroute6.h, linux/netlink.h,
 * linux/rtnetlink.h, linux/socket.h, sched.h, sys/ioctl.h, sys/socket.h, and
 * kselftest_harness.h.
 *
 * The declarations, constants, macros, and C library calls from those headers
 * are treated as external dependencies of this translated source.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

extern "C" {
    static mut errno: c_int;

    fn unshare(flags: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: c_uint,
    ) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: usize, flags: c_int) -> isize;
    fn recv(socket: c_int, buffer: *mut c_void, length: usize, flags: c_int) -> isize;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

extern "C" {
    static AF_INET: c_int;
    static AF_INET6: c_int;
    static AF_NETLINK: c_int;
    static CLONE_NEWNET: c_int;
    static EADDRNOTAVAIL: c_int;
    static ENODEV: c_int;
    static ENFILE: c_int;
    static IPPROTO_ICMPV6: c_int;
    static IPPROTO_IGMP: c_int;
    static IPPROTO_IP: c_int;
    static IPPROTO_IPV6: c_int;
    static MIFF_REGISTER: c_int;
    static MRT_ADD_MFC: c_int;
    static MRT_ADD_MFC_PROXY: c_int;
    static MRT_ADD_VIF: c_int;
    static MRT_ASSERT: c_int;
    static MRT_BASE: c_int;
    static MRT_DEL_MFC: c_int;
    static MRT_DEL_MFC_PROXY: c_int;
    static MRT_DEL_VIF: c_int;
    static MRT_DONE: c_int;
    static MRT_FLUSH: c_int;
    static MRT_FLUSH_MFC: c_int;
    static MRT_FLUSH_MFC_STATIC: c_int;
    static MRT_FLUSH_VIFS: c_int;
    static MRT_FLUSH_VIFS_STATIC: c_int;
    static MRT_INIT: c_int;
    static MRT_MAX: c_int;
    static MRT_PIM: c_int;
    static MRT_TABLE: c_int;
    static MRT_VERSION: c_int;
    static MRT6_ADD_MFC: c_int;
    static MRT6_ADD_MFC_PROXY: c_int;
    static MRT6_ADD_MIF: c_int;
    static MRT6_ASSERT: c_int;
    static MRT6_DEL_MFC: c_int;
    static MRT6_DEL_MFC_PROXY: c_int;
    static MRT6_DEL_MIF: c_int;
    static MRT6_DONE: c_int;
    static MRT6_FLUSH: c_int;
    static MRT6_FLUSH_MFC: c_int;
    static MRT6_FLUSH_MFC_STATIC: c_int;
    static MRT6_FLUSH_MIFS: c_int;
    static MRT6_FLUSH_MIFS_STATIC: c_int;
    static MRT6_INIT: c_int;
    static MRT6_PIM: c_int;
    static MRT6_TABLE: c_int;
    static MRT6_VERSION: c_int;
    static NETLINK_ROUTE: c_int;
    static NLM_F_ACK: c_int;
    static NLM_F_REQUEST: c_int;
    static NLMSG_ERROR: c_int;
    static RTA_DST: c_int;
    static RTA_IIF: c_int;
    static RTA_PREFSRC: c_int;
    static RTA_SRC: c_int;
    static RTA_TABLE: c_int;
    static RTM_DELROUTE: u16;
    static RTM_NEWROUTE: u16;
    static RTNL_FAMILY_IP6MR: u8;
    static RTNL_FAMILY_IPMR: u8;
    static RTN_MULTICAST: u8;
    static RTPROT_MROUTED: u8;
    static RT_SCOPE_UNIVERSE: u8;
    static RT_TABLE_DEFAULT: c_int;
    static SIOCGIFINDEX: c_uint;
    static SOCK_RAW: c_int;
    static VIFF_REGISTER: c_int;
    static VIFF_USE_IFINDEX: c_int;
}

extern "C" {
    fn RTA_DATA(rta: *mut rtattr) -> *mut c_void;
    fn RTA_LENGTH(len: c_int) -> u16;
    fn RTA_NEXT(rta: *mut rtattr, attrlen: c_int) -> *mut rtattr;
    fn NLMSG_ALIGN(len: u32) -> u32;
    fn NLMSG_LENGTH(len: usize) -> u32;
    fn NLMSG_OK(nlh: *mut nlmsghdr, len: isize) -> bool;
    fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void;
}

extern "C" {
    fn ASSERT_EQ<T, U>(expected: T, actual: U);
    fn ASSERT_LE<T, U>(expected: T, actual: U);
    fn ASSERT_LT<T, U>(expected: T, actual: U);
    fn ASSERT_TRUE(value: bool);
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtmsg {
    pub rtm_family: u8,
    pub rtm_dst_len: u8,
    pub rtm_src_len: u8,
    pub rtm_tos: u8,
    pub rtm_table: u8,
    pub rtm_protocol: u8,
    pub rtm_scope: u8,
    pub rtm_type: u8,
    pub rtm_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vifctl {
    pub vifc_vifi: u16,
    pub vifc_flags: c_int,
    pub vifc_threshold: u8,
    pub vifc_rate_limit: u32,
    pub vifc_lcl_addr: u32,
    pub vifc_rmt_addr: u32,
    pub vifc_lcl_ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mif6ctl {
    pub mif6c_mifi: u16,
    pub mif6c_flags: c_int,
    pub vifc_threshold: u8,
    pub mif6c_pifi: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfcctl {
    pub _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mf6cctl {
    pub _private: [u8; 0],
}

#[repr(C)]
pub union IfReqIfrIfru {
    pub ifr_ifindex: c_int,
}

#[repr(C)]
pub struct ifreq {
    pub ifr_name: [c_char; 16],
    pub ifr_ifru: IfReqIfrIfru,
}

#[repr(C)]
pub union IpmrVifUnion {
    pub vif: vifctl,
    pub vif6: mif6ctl,
}

#[repr(C)]
pub union IpmrMfcUnion {
    pub mfc: mfcctl,
    pub mfc6: mf6cctl,
}

#[repr(C)]
pub struct IpmrFixture {
    pub netlink_sk: c_int,
    pub raw_sk: c_int,
    pub veth_ifindex: c_int,
    pub vif_u: IpmrVifUnion,
    pub mfc_u: IpmrMfcUnion,
}

#[repr(C)]
pub struct IpmrFixtureVariant {
    pub family: c_int,
    pub protocol: c_int,
    pub level: c_int,
    pub rtm_family: c_int,
    pub opts: [c_int; 13],
    pub flush_flags: c_int,
    pub vif_size: c_int,
    pub vif_check_cmd_pimreg: [c_char; 64],
    pub vif_check_cmd_veth: [c_char; 64],
    pub mfc_size: c_int,
    pub mfc_check_cmd: [c_char; 1024],
}

#[repr(C)]
pub struct mfc_attr {
    pub table: c_int,
    pub origin: u32,
    pub group: u32,
    pub ifindex: c_int,
    pub proxy: bool,
}

fn cstr_array<const N: usize>(s: &[u8]) -> [c_char; N] {
    let mut out = [0 as c_char; N];
    let mut i = 0usize;

    while i < s.len() && i < N {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

pub unsafe fn ipmr_ipv4_variant() -> IpmrFixtureVariant {
    IpmrFixtureVariant {
        family: AF_INET,
        protocol: IPPROTO_IGMP,
        level: IPPROTO_IP,
        rtm_family: RTNL_FAMILY_IPMR as c_int,
        opts: [
            MRT_INIT,
            MRT_DONE,
            MRT_ADD_VIF,
            MRT_DEL_VIF,
            MRT_ADD_MFC,
            MRT_DEL_MFC,
            MRT_VERSION,
            MRT_ASSERT,
            MRT_PIM,
            MRT_TABLE,
            MRT_ADD_MFC_PROXY,
            MRT_DEL_MFC_PROXY,
            MRT_FLUSH,
        ],
        flush_flags: MRT_FLUSH_MFC | MRT_FLUSH_MFC_STATIC | MRT_FLUSH_VIFS | MRT_FLUSH_VIFS_STATIC,
        vif_size: size_of::<vifctl>() as c_int,
        vif_check_cmd_pimreg: cstr_array(b"cat /proc/net/ip_mr_vif | grep -q pimreg\0"),
        vif_check_cmd_veth: cstr_array(b"cat /proc/net/ip_mr_vif | grep -q veth\0"),
        mfc_size: size_of::<mfcctl>() as c_int,
        mfc_check_cmd: cstr_array(b"cat /proc/net/ip_mr_cache | grep -q '00000000 00000000'\0"),
    }
}

pub unsafe fn ipmr_ipv6_variant() -> IpmrFixtureVariant {
    IpmrFixtureVariant {
        family: AF_INET6,
        protocol: IPPROTO_ICMPV6,
        level: IPPROTO_IPV6,
        rtm_family: RTNL_FAMILY_IP6MR as c_int,
        opts: [
            MRT6_INIT,
            MRT6_DONE,
            MRT6_ADD_MIF,
            MRT6_DEL_MIF,
            MRT6_ADD_MFC,
            MRT6_DEL_MFC,
            MRT6_VERSION,
            MRT6_ASSERT,
            MRT6_PIM,
            MRT6_TABLE,
            MRT6_ADD_MFC_PROXY,
            MRT6_DEL_MFC_PROXY,
            MRT6_FLUSH,
        ],
        flush_flags: MRT6_FLUSH_MFC | MRT6_FLUSH_MFC_STATIC | MRT6_FLUSH_MIFS | MRT6_FLUSH_MIFS_STATIC,
        vif_size: size_of::<mif6ctl>() as c_int,
        vif_check_cmd_pimreg: cstr_array(b"cat /proc/net/ip6_mr_vif | grep -q pim6reg\0"),
        vif_check_cmd_veth: cstr_array(b"cat /proc/net/ip6_mr_vif | grep -q veth\0"),
        mfc_size: size_of::<mf6cctl>() as c_int,
        mfc_check_cmd: cstr_array(
            b"cat /proc/net/ip6_mr_cache | grep -q '0000:0000:0000:0000:0000:0000:0000:0000 0000:0000:0000:0000:0000:0000:0000:0000'\0",
        ),
    }
}

unsafe fn nl_add_rtattr(
    nlmsg: *mut nlmsghdr,
    rta: *mut rtattr,
    type_: c_int,
    data: *const c_void,
    len: c_int,
) -> *mut rtattr {
    let unused: c_int = 0;

    (*rta).rta_type = type_ as u16;
    (*rta).rta_len = RTA_LENGTH(len);
    memcpy(RTA_DATA(rta), data, len as usize);

    (*nlmsg).nlmsg_len = (*nlmsg).nlmsg_len.wrapping_add(NLMSG_ALIGN((*rta).rta_len as u32));

    RTA_NEXT(rta, unused)
}

#[repr(C)]
struct NlSendmsgMfcReq {
    nlmsg: nlmsghdr,
    rtm: rtmsg,
    buf: [c_char; 4096],
}

unsafe fn nl_sendmsg_mfc(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
    nlmsg_type: u16,
    mfc_attr: *mut mfc_attr,
) -> c_int {
    let mut req = NlSendmsgMfcReq {
        nlmsg: nlmsghdr {
            nlmsg_len: NLMSG_LENGTH(size_of::<rtmsg>()),
            /* ipmr does not care about NLM_F_CREATE and NLM_F_EXCL ... */
            nlmsg_flags: (NLM_F_REQUEST | NLM_F_ACK) as u16,
            nlmsg_type,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        },
        rtm: rtmsg {
            /* hard requirements in rtm_to_ipmr_mfcc() */
            rtm_family: (*variant).rtm_family as u8,
            rtm_dst_len: 32,
            rtm_src_len: 0,
            rtm_tos: 0,
            rtm_table: 0,
            rtm_type: RTN_MULTICAST,
            rtm_scope: RT_SCOPE_UNIVERSE,
            rtm_protocol: RTPROT_MROUTED,
            rtm_flags: 0,
        },
        buf: [0; 4096],
    };
    let nlmsg: *mut nlmsghdr = &mut req.nlmsg;
    let errmsg: *mut nlmsgerr;
    let mut rta: *mut rtattr;
    let mut err: c_int;

    rta = req.buf.as_mut_ptr() as *mut rtattr;
    rta = nl_add_rtattr(
        nlmsg,
        rta,
        RTA_TABLE,
        &(*mfc_attr).table as *const c_int as *const c_void,
        size_of::<c_int>() as c_int,
    );
    rta = nl_add_rtattr(
        nlmsg,
        rta,
        RTA_SRC,
        &(*mfc_attr).origin as *const u32 as *const c_void,
        size_of::<u32>() as c_int,
    );
    rta = nl_add_rtattr(
        nlmsg,
        rta,
        RTA_DST,
        &(*mfc_attr).group as *const u32 as *const c_void,
        size_of::<u32>() as c_int,
    );
    if (*mfc_attr).ifindex != 0 {
        rta = nl_add_rtattr(
            nlmsg,
            rta,
            RTA_IIF,
            &(*mfc_attr).ifindex as *const c_int as *const c_void,
            size_of::<c_int>() as c_int,
        );
    }
    if (*mfc_attr).proxy {
        rta = nl_add_rtattr(nlmsg, rta, RTA_PREFSRC, ptr::null(), 0);
    }

    err = send(
        (*self_).netlink_sk,
        &req as *const NlSendmsgMfcReq as *const c_void,
        req.nlmsg.nlmsg_len as usize,
        0,
    ) as c_int;
    ASSERT_EQ(err, req.nlmsg.nlmsg_len);

    memset(
        &mut req as *mut NlSendmsgMfcReq as *mut c_void,
        0,
        size_of::<NlSendmsgMfcReq>(),
    );

    err = recv(
        (*self_).netlink_sk,
        &mut req as *mut NlSendmsgMfcReq as *mut c_void,
        size_of::<NlSendmsgMfcReq>(),
        0,
    ) as c_int;
    ASSERT_TRUE(NLMSG_OK(nlmsg, err as isize));
    ASSERT_EQ(NLMSG_ERROR, (*nlmsg).nlmsg_type as c_int);

    errmsg = NLMSG_DATA(nlmsg) as *mut nlmsgerr;
    (*errmsg).error
}

unsafe fn ipmr_setup(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut ifr = ifreq {
        ifr_name: cstr_array(b"veth0\0"),
        ifr_ifru: IfReqIfrIfru { ifr_ifindex: 0 },
    };
    let mut err: c_int;

    err = unshare(CLONE_NEWNET);
    ASSERT_EQ(0, err);

    (*self_).netlink_sk = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    ASSERT_LE(0, (*self_).netlink_sk);

    (*self_).raw_sk = socket((*variant).family, SOCK_RAW, (*variant).protocol);
    ASSERT_LT(0, (*self_).raw_sk);

    err = system(b"ip link add veth0 type veth peer veth1\0".as_ptr() as *const c_char);
    ASSERT_EQ(0, err);

    err = ioctl((*self_).raw_sk, SIOCGIFINDEX, &mut ifr);
    ASSERT_EQ(0, err);

    (*self_).veth_ifindex = ifr.ifr_ifru.ifr_ifindex;

    if (*variant).family == AF_INET {
        (*self_).vif_u.vif = vifctl {
            vifc_flags: VIFF_USE_IFINDEX,
            vifc_lcl_ifindex: (*self_).veth_ifindex,
            ..zeroed()
        };
    } else {
        (*self_).vif_u.vif6 = mif6ctl {
            mif6c_flags: 0,
            mif6c_pifi: (*self_).veth_ifindex,
            ..zeroed()
        };
    }
}

unsafe fn ipmr_teardown(self_: *mut IpmrFixture) {
    close((*self_).raw_sk);
    close((*self_).netlink_sk);
}

unsafe fn test_mrt_init(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut val: c_int = 0; /* any value is ok, but size must be int for MRT_INIT. */
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_INIT - MRT_BASE) as usize],
        &mut val as *mut c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    );
    ASSERT_EQ(0, err);

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_DONE - MRT_BASE) as usize],
        &mut val as *mut c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    );
    ASSERT_EQ(0, err);
}

unsafe fn test_mrt_add_vif_register(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut err: c_int;

    memset(
        &mut (*self_).vif_u.vif as *mut vifctl as *mut c_void,
        0,
        (*variant).vif_size as usize,
    );

    if (*variant).family == AF_INET {
        (*self_).vif_u.vif.vifc_flags = VIFF_REGISTER;
    } else {
        (*self_).vif_u.vif6.mif6c_flags = MIFF_REGISTER;
    }

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    err = system((*variant).vif_check_cmd_pimreg.as_ptr());
    ASSERT_EQ(0, err);

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_DEL_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);
}

unsafe fn test_mrt_del_vif_unreg(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    err = system((*variant).vif_check_cmd_veth.as_ptr());
    ASSERT_EQ(0, err);

    /* VIF is removed along with its device. */
    err = system(b"ip link del veth0\0".as_ptr() as *const c_char);
    ASSERT_EQ(0, err);

    /* mrt->vif_table[veth_ifindex]->dev is NULL. */
    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_DEL_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(-1, err);
    ASSERT_EQ(EADDRNOTAVAIL, errno);
}

unsafe fn test_mrt_del_vif_netns_dismantle(
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    /* Let cleanup_net() remove veth0 and VIF. */
}

unsafe fn test_mrt_add_mfc(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut err: c_int;

    /* MRT_ADD_MFC / MRT_ADD_MFC_PROXY does not need vif to exist (unlike netlink). */
    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_MFC - MRT_BASE) as usize],
        &mut (*self_).mfc_u.mfc as *mut mfcctl as *const c_void,
        (*variant).mfc_size as c_uint,
    );
    ASSERT_EQ(0, err);

    /* (0.0.0.0 -> 0.0.0.0) */
    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_DEL_MFC - MRT_BASE) as usize],
        &mut (*self_).mfc_u.mfc as *mut mfcctl as *const c_void,
        (*variant).mfc_size as c_uint,
    );
}

unsafe fn test_mrt_add_mfc_proxy(self_: *mut IpmrFixture, variant: *const IpmrFixtureVariant) {
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_MFC_PROXY - MRT_BASE) as usize],
        &mut (*self_).mfc_u.mfc as *mut mfcctl as *const c_void,
        (*variant).mfc_size as c_uint,
    );
    ASSERT_EQ(0, err);

    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_DEL_MFC_PROXY - MRT_BASE) as usize],
        &mut (*self_).mfc_u.mfc as *mut mfcctl as *const c_void,
        (*variant).mfc_size as c_uint,
    );
}

unsafe fn test_mrt_add_mfc_netlink(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut mfc_attr = mfc_attr {
        table: RT_TABLE_DEFAULT,
        origin: 0,
        group: 0,
        ifindex: (*self_).veth_ifindex,
        proxy: false,
    };
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    ASSERT_EQ(0, err);

    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_DELROUTE, &mut mfc_attr);
    ASSERT_EQ(0, err);
}

unsafe fn test_mrt_add_mfc_netlink_proxy(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut mfc_attr = mfc_attr {
        table: RT_TABLE_DEFAULT,
        origin: 0,
        group: 0,
        ifindex: (*self_).veth_ifindex,
        proxy: true,
    };
    let mut err: c_int;

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    ASSERT_EQ(0, err);

    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_DELROUTE, &mut mfc_attr);
    ASSERT_EQ(0, err);
}

unsafe fn test_mrt_add_mfc_netlink_no_vif(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut mfc_attr = mfc_attr {
        table: RT_TABLE_DEFAULT,
        origin: 0,
        group: 0,
        ifindex: 0,
        proxy: false,
    };
    let mut err: c_int;

    /* netlink always requires RTA_IIF of an existing vif. */
    mfc_attr.ifindex = 0;
    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    ASSERT_EQ(-ENFILE, err);

    /* netlink always requires RTA_IIF of an existing vif. */
    mfc_attr.ifindex = (*self_).veth_ifindex;
    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    ASSERT_EQ(-ENFILE, err);
}

unsafe fn test_mrt_del_mfc_netlink_netns_dismantle(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut vifs = [
        vifctl {
            vifc_vifi: 0,
            vifc_flags: VIFF_USE_IFINDEX,
            vifc_lcl_ifindex: (*self_).veth_ifindex,
            ..zeroed()
        },
        vifctl {
            vifc_vifi: 1,
            vifc_flags: VIFF_REGISTER,
            ..zeroed()
        },
    ];
    let mut mfc_attr = mfc_attr {
        table: RT_TABLE_DEFAULT,
        origin: 0,
        group: 0,
        ifindex: (*self_).veth_ifindex,
        proxy: false,
    };
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < 2 {
        /* Create 2 VIFs just to avoid -ENFILE later. */
        err = setsockopt(
            (*self_).raw_sk,
            (*variant).level,
            (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
            &mut vifs[i as usize] as *mut vifctl as *const c_void,
            size_of::<vifctl>() as c_uint,
        );
        ASSERT_EQ(0, err);
        i += 1;
    }

    /* Create a MFC for mrt->vif_table[0]. */
    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    ASSERT_EQ(0, err);

    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    /* Remove mrt->vif_table[0]. */
    err = system(b"ip link del veth0\0".as_ptr() as *const c_char);
    ASSERT_EQ(0, err);

    /* MFC entry is NOT removed even if the tied VIF is removed... */
    err = system((*variant).mfc_check_cmd.as_ptr());
    ASSERT_EQ(0, err);

    /* ... and netlink is not capable of removing such an entry
     * because netlink always requires a valid RTA_IIF ... :/
     */
    err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_DELROUTE, &mut mfc_attr);
    ASSERT_EQ(-ENODEV, err);

    /* It can be removed by setsockopt(), but let cleanup_net() remove this time. */
}

unsafe fn test_mrt_table_flush(
    _metadata: *mut __test_metadata,
    self_: *mut IpmrFixture,
    variant: *const IpmrFixtureVariant,
) {
    let mut mfc_attr = mfc_attr {
        table: 0,
        origin: 0,
        group: 0,
        ifindex: (*self_).veth_ifindex,
        proxy: false,
    };
    let mut table_id: c_int = 92;
    let mut err: c_int;

    /* Set a random table id rather than RT_TABLE_DEFAULT.
     * Note that /proc/net/ip_mr_{vif,cache} only supports RT_TABLE_DEFAULT.
     */
    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_TABLE - MRT_BASE) as usize],
        &mut table_id as *mut c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    );
    ASSERT_EQ(0, err);

    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_ADD_VIF - MRT_BASE) as usize],
        &mut (*self_).vif_u.vif as *mut vifctl as *const c_void,
        (*variant).vif_size as c_uint,
    );
    ASSERT_EQ(0, err);

    if (*variant).family == AF_INET {
        mfc_attr.table = table_id;
        err = nl_sendmsg_mfc(_metadata, self_, variant, RTM_NEWROUTE, &mut mfc_attr);
    } else {
        err = setsockopt(
            (*self_).raw_sk,
            (*variant).level,
            (*variant).opts[(MRT_ADD_MFC - MRT_BASE) as usize],
            &mut (*self_).mfc_u.mfc as *mut mfcctl as *const c_void,
            (*variant).mfc_size as c_uint,
        );
    }
    ASSERT_EQ(0, err);

    /* Flush mrt->vif_table[] and all caches. */
    err = setsockopt(
        (*self_).raw_sk,
        (*variant).level,
        (*variant).opts[(MRT_FLUSH - MRT_BASE) as usize],
        &(*variant).flush_flags as *const c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    );
    ASSERT_EQ(0, err);
}

/*
 * XFAIL_ADD(ipmr, ipv6, mrt_add_mfc_netlink);
 * XFAIL_ADD(ipmr, ipv6, mrt_add_mfc_netlink_proxy);
 * XFAIL_ADD(ipmr, ipv6, mrt_add_mfc_netlink_no_vif);
 * XFAIL_ADD(ipmr, ipv6, mrt_del_mfc_netlink_netns_dismantle);
 *
 * TEST_HARNESS_MAIN
 */
