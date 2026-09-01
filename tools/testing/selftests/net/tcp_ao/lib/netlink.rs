// SPDX-License-Identifier: GPL-2.0
/* Original from tools/testing/selftests/net/ipsec.c */
/*
 * C dependencies removed from executable Rust:
 * <linux/netlink.h>, <linux/random.h>, <linux/rtnetlink.h>,
 * <linux/veth.h>, <net/if.h>, <stdint.h>, <string.h>,
 * <sys/socket.h>, and "aolib.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr;

const MAX_PAYLOAD: usize = 2048;

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn randomize_buffer(buf: *mut c_void, buflen: usize);
    fn test_print(fmt: *const c_char, ...);
    fn test_error(fmt: *const c_char, ...) -> !;
}

unsafe fn netlink_sock(mut sock: *mut c_int, mut seq_nr: *mut u32, proto: c_int) -> c_int {
    if *sock > 0 {
        seq_nr = seq_nr.add(1);
        let _ = seq_nr;
        return 0;
    }

    *sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, proto);
    if *sock < 0 {
        test_print(c"socket(AF_NETLINK)".as_ptr());
        return -1;
    }

    randomize_buffer(seq_nr as *mut c_void, size_of::<u32>());

    0
}

#[repr(C)]
struct nlmsgerror {
    hdr: nlmsghdr,
    error: c_int,
    orig_msg: nlmsghdr,
}

unsafe fn netlink_check_answer(sock: c_int, quite: bool) -> c_int {
    let mut answer: nlmsgerror = zeroed();

    if recv(
        sock,
        &mut answer as *mut nlmsgerror as *mut c_void,
        size_of::<nlmsgerror>(),
        0,
    ) < 0
    {
        test_print(c"recv()".as_ptr());
        return -1;
    } else if answer.hdr.nlmsg_type != NLMSG_ERROR {
        test_print(
            c"expected NLMSG_ERROR, got %d".as_ptr(),
            answer.hdr.nlmsg_type as c_int,
        );
        return -1;
    } else if answer.error != 0 {
        if !quite {
            test_print(
                c"NLMSG_ERROR: %d: %s".as_ptr(),
                answer.error,
                strerror(-answer.error),
            );
        }
        return answer.error;
    }

    0
}

unsafe fn rtattr_hdr(nh: *mut nlmsghdr) -> *mut rtattr {
    (nh as *mut c_char).add(RTA_ALIGN((*nh).nlmsg_len as usize)) as *mut rtattr
}

unsafe fn rtattr_pack(
    nh: *mut nlmsghdr,
    req_sz: usize,
    rta_type: c_ushort,
    payload: *const c_void,
    size: usize,
) -> c_int {
    /* NLMSG_ALIGNTO == RTA_ALIGNTO, nlmsg_len already aligned */
    let attr = rtattr_hdr(nh);
    let nl_size = RTA_ALIGN((*nh).nlmsg_len as usize) + RTA_LENGTH(size) as usize;

    if req_sz < nl_size {
        test_print(
            c"req buf is too small: %zu < %zu".as_ptr(),
            req_sz,
            nl_size,
        );
        return -1;
    }
    (*nh).nlmsg_len = nl_size as _;

    (*attr).rta_len = RTA_LENGTH(size) as _;
    (*attr).rta_type = rta_type;
    memcpy(RTA_DATA(attr), payload, size);

    0
}

unsafe fn _rtattr_begin(
    nh: *mut nlmsghdr,
    req_sz: usize,
    rta_type: c_ushort,
    payload: *const c_void,
    size: usize,
) -> *mut rtattr {
    let ret = rtattr_hdr(nh);

    if rtattr_pack(nh, req_sz, rta_type, payload, size) != 0 {
        return ptr::null_mut();
    }

    ret
}

unsafe fn rtattr_begin(nh: *mut nlmsghdr, req_sz: usize, rta_type: c_ushort) -> *mut rtattr {
    _rtattr_begin(nh, req_sz, rta_type, ptr::null(), 0)
}

unsafe fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    let nlmsg_end = (nh as *mut c_char).add((*nh).nlmsg_len as usize);

    (*attr).rta_len = nlmsg_end.offset_from(attr as *mut c_char) as _;
}

unsafe fn veth_pack_peerb(
    nh: *mut nlmsghdr,
    req_sz: usize,
    peer: *const c_char,
    ns: c_int,
) -> c_int {
    let mut pi: ifinfomsg = zeroed();
    let peer_attr: *mut rtattr;

    memset(
        &mut pi as *mut ifinfomsg as *mut c_void,
        0,
        size_of::<ifinfomsg>(),
    );
    pi.ifi_family = AF_UNSPEC as _;
    pi.ifi_change = 0xFFFFFFFF;

    peer_attr = _rtattr_begin(
        nh,
        req_sz,
        VETH_INFO_PEER as _,
        &pi as *const ifinfomsg as *const c_void,
        size_of::<ifinfomsg>(),
    );
    if peer_attr.is_null() {
        return -1;
    }

    if rtattr_pack(
        nh,
        req_sz,
        IFLA_IFNAME as _,
        peer as *const c_void,
        strlen(peer),
    ) != 0
    {
        return -1;
    }

    if rtattr_pack(
        nh,
        req_sz,
        IFLA_NET_NS_FD as _,
        &ns as *const c_int as *const c_void,
        size_of::<c_int>(),
    ) != 0
    {
        return -1;
    }

    rtattr_end(nh, peer_attr);

    0
}

#[repr(C)]
struct newlink_req {
    nh: nlmsghdr,
    info: ifinfomsg,
    attrbuf: [c_char; MAX_PAYLOAD],
}

unsafe fn __add_veth(
    sock: c_int,
    seq: u32,
    name: *const c_char,
    ns_a: c_int,
    ns_b: c_int,
) -> c_int {
    let flags: u16 = (NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE) as u16;
    let mut req: newlink_req = zeroed();
    static VETH_TYPE: &[u8; 5] = b"veth\0";
    let link_info: *mut rtattr;
    let info_data: *mut rtattr;

    memset(
        &mut req as *mut newlink_req as *mut c_void,
        0,
        size_of::<newlink_req>(),
    );
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()) as _;
    req.nh.nlmsg_type = RTM_NEWLINK as _;
    req.nh.nlmsg_flags = flags as _;
    req.nh.nlmsg_seq = seq;
    req.info.ifi_family = AF_UNSPEC as _;
    req.info.ifi_change = 0xFFFFFFFF;

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_IFNAME as _,
        name as *const c_void,
        strlen(name),
    ) != 0
    {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_NET_NS_FD as _,
        &ns_a as *const c_int as *const c_void,
        size_of::<c_int>(),
    ) != 0
    {
        return -1;
    }

    link_info = rtattr_begin(&mut req.nh, size_of::<newlink_req>(), IFLA_LINKINFO as _);
    if link_info.is_null() {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_INFO_KIND as _,
        VETH_TYPE.as_ptr() as *const c_void,
        size_of_val(VETH_TYPE),
    ) != 0
    {
        return -1;
    }

    info_data = rtattr_begin(&mut req.nh, size_of::<newlink_req>(), IFLA_INFO_DATA as _);
    if info_data.is_null() {
        return -1;
    }

    if veth_pack_peerb(&mut req.nh, size_of::<newlink_req>(), name, ns_b) != 0 {
        return -1;
    }

    rtattr_end(&mut req.nh, info_data);
    rtattr_end(&mut req.nh, link_info);

    if send(
        sock,
        &req as *const newlink_req as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) < 0
    {
        test_print(c"send()".as_ptr());
        return -1;
    }
    netlink_check_answer(sock, false)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_veth(name: *const c_char, nsfda: c_int, nsfdb: c_int) -> c_int {
    let mut route_sock: c_int = -1;
    let ret: c_int;
    let mut route_seq: u32 = 0;

    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 {
        test_error(c"Failed to open netlink route socket\n".as_ptr());
    }

    ret = __add_veth(route_sock, {
        let old = route_seq;
        route_seq = route_seq.wrapping_add(1);
        old
    }, name, nsfda, nsfdb);
    close(route_sock);
    ret
}

#[repr(C)]
struct newaddr_req {
    nh: nlmsghdr,
    info: ifaddrmsg,
    attrbuf: [c_char; MAX_PAYLOAD],
}

unsafe fn __ip_addr_add(
    sock: c_int,
    seq: u32,
    intf: *const c_char,
    family: c_int,
    addr: tcp_addr,
    prefix: u8,
) -> c_int {
    let flags: u16 = (NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE) as u16;
    let mut req: newaddr_req = zeroed();
    let addr_len = if family == AF_INET {
        size_of::<in_addr>()
    } else {
        size_of::<in6_addr>()
    };

    memset(
        &mut req as *mut newaddr_req as *mut c_void,
        0,
        size_of::<newaddr_req>(),
    );
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifaddrmsg>()) as _;
    req.nh.nlmsg_type = RTM_NEWADDR as _;
    req.nh.nlmsg_flags = flags as _;
    req.nh.nlmsg_seq = seq;
    req.info.ifa_family = family as _;
    req.info.ifa_prefixlen = prefix;
    req.info.ifa_index = if_nametoindex(intf);
    req.info.ifa_flags = IFA_F_NODAD as _;

    if rtattr_pack(
        &mut req.nh,
        size_of::<newaddr_req>(),
        IFA_LOCAL as _,
        &addr as *const tcp_addr as *const c_void,
        addr_len,
    ) != 0
    {
        return -1;
    }

    if send(
        sock,
        &req as *const newaddr_req as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) < 0
    {
        test_print(c"send()".as_ptr());
        return -1;
    }
    netlink_check_answer(sock, true)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip_addr_add(
    intf: *const c_char,
    family: c_int,
    addr: tcp_addr,
    prefix: u8,
) -> c_int {
    let mut route_sock: c_int = -1;
    let ret: c_int;
    let mut route_seq: u32 = 0;

    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 {
        test_error(c"Failed to open netlink route socket\n".as_ptr());
    }

    ret = __ip_addr_add(route_sock, {
        let old = route_seq;
        route_seq = route_seq.wrapping_add(1);
        old
    }, intf, family, addr, prefix);

    close(route_sock);
    ret
}

#[repr(C)]
struct newroute_req {
    nh: nlmsghdr,
    rt: rtmsg,
    attrbuf: [c_char; MAX_PAYLOAD],
}

unsafe fn __ip_route_add(
    sock: c_int,
    seq: u32,
    intf: *const c_char,
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,
    vrf: u8,
) -> c_int {
    let mut req: newroute_req = zeroed();
    let index: c_uint = if_nametoindex(intf);
    let addr_len = if family == AF_INET {
        size_of::<in_addr>()
    } else {
        size_of::<in6_addr>()
    };

    memset(
        &mut req as *mut newroute_req as *mut c_void,
        0,
        size_of::<newroute_req>(),
    );
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<rtmsg>()) as _;
    req.nh.nlmsg_type = RTM_NEWROUTE as _;
    req.nh.nlmsg_flags = (NLM_F_REQUEST | NLM_F_ACK | NLM_F_CREATE) as _;
    req.nh.nlmsg_seq = seq;
    req.rt.rtm_family = family as _;
    req.rt.rtm_dst_len = if family == AF_INET { 32 } else { 128 };
    req.rt.rtm_table = vrf;
    req.rt.rtm_protocol = RTPROT_BOOT as _;
    req.rt.rtm_scope = RT_SCOPE_UNIVERSE as _;
    req.rt.rtm_type = RTN_UNICAST as _;

    if rtattr_pack(
        &mut req.nh,
        size_of::<newroute_req>(),
        RTA_DST as _,
        &dst as *const tcp_addr as *const c_void,
        addr_len,
    ) != 0
    {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newroute_req>(),
        RTA_PREFSRC as _,
        &src as *const tcp_addr as *const c_void,
        addr_len,
    ) != 0
    {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newroute_req>(),
        RTA_OIF as _,
        &index as *const c_uint as *const c_void,
        size_of::<c_uint>(),
    ) != 0
    {
        return -1;
    }

    if send(
        sock,
        &req as *const newroute_req as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) < 0
    {
        test_print(c"send()".as_ptr());
        return -1;
    }

    netlink_check_answer(sock, true)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip_route_add_vrf(
    intf: *const c_char,
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,
    vrf: u8,
) -> c_int {
    let mut route_sock: c_int = -1;
    let ret: c_int;
    let mut route_seq: u32 = 0;

    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 {
        test_error(c"Failed to open netlink route socket\n".as_ptr());
    }

    ret = __ip_route_add(route_sock, {
        let old = route_seq;
        route_seq = route_seq.wrapping_add(1);
        old
    }, intf, family, src, dst, vrf);

    close(route_sock);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip_route_add(
    intf: *const c_char,
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,
) -> c_int {
    ip_route_add_vrf(intf, family, src, dst, RT_TABLE_MAIN as u8)
}

unsafe fn __link_set_up(sock: c_int, seq: u32, intf: *const c_char) -> c_int {
    let mut req: newlink_req = zeroed();

    memset(
        &mut req as *mut newlink_req as *mut c_void,
        0,
        size_of::<newlink_req>(),
    );
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()) as _;
    req.nh.nlmsg_type = RTM_NEWLINK as _;
    req.nh.nlmsg_flags = (NLM_F_REQUEST | NLM_F_ACK) as _;
    req.nh.nlmsg_seq = seq;
    req.info.ifi_family = AF_UNSPEC as _;
    req.info.ifi_change = 0xFFFFFFFF;
    req.info.ifi_index = if_nametoindex(intf) as _;
    req.info.ifi_flags = IFF_UP as _;
    req.info.ifi_change = IFF_UP as _;

    if send(
        sock,
        &req as *const newlink_req as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) < 0
    {
        test_print(c"send()".as_ptr());
        return -1;
    }
    netlink_check_answer(sock, false)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn link_set_up(intf: *const c_char) -> c_int {
    let mut route_sock: c_int = -1;
    let ret: c_int;
    let mut route_seq: u32 = 0;

    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 {
        test_error(c"Failed to open netlink route socket\n".as_ptr());
    }

    ret = __link_set_up(route_sock, {
        let old = route_seq;
        route_seq = route_seq.wrapping_add(1);
        old
    }, intf);

    close(route_sock);
    ret
}

unsafe fn __add_vrf(
    sock: c_int,
    seq: u32,
    name: *const c_char,
    tabid: u32,
    ifindex: c_int,
    nsfd: c_int,
) -> c_int {
    let flags: u16 = (NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE) as u16;
    let mut req: newlink_req = zeroed();
    static VRF_TYPE: &[u8; 4] = b"vrf\0";
    let link_info: *mut rtattr;
    let info_data: *mut rtattr;

    memset(
        &mut req as *mut newlink_req as *mut c_void,
        0,
        size_of::<newlink_req>(),
    );
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()) as _;
    req.nh.nlmsg_type = RTM_NEWLINK as _;
    req.nh.nlmsg_flags = flags as _;
    req.nh.nlmsg_seq = seq;
    req.info.ifi_family = AF_UNSPEC as _;
    req.info.ifi_change = 0xFFFFFFFF;
    req.info.ifi_index = ifindex;

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_IFNAME as _,
        name as *const c_void,
        strlen(name),
    ) != 0
    {
        return -1;
    }

    if nsfd >= 0 {
        if rtattr_pack(
            &mut req.nh,
            size_of::<newlink_req>(),
            IFLA_NET_NS_FD as _,
            &nsfd as *const c_int as *const c_void,
            size_of::<c_int>(),
        ) != 0
        {
            return -1;
        }
    }

    link_info = rtattr_begin(&mut req.nh, size_of::<newlink_req>(), IFLA_LINKINFO as _);
    if link_info.is_null() {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_INFO_KIND as _,
        VRF_TYPE.as_ptr() as *const c_void,
        size_of_val(VRF_TYPE),
    ) != 0
    {
        return -1;
    }

    info_data = rtattr_begin(&mut req.nh, size_of::<newlink_req>(), IFLA_INFO_DATA as _);
    if info_data.is_null() {
        return -1;
    }

    if rtattr_pack(
        &mut req.nh,
        size_of::<newlink_req>(),
        IFLA_VRF_TABLE as _,
        &tabid as *const u32 as *const c_void,
        size_of::<u32>(),
    ) != 0
    {
        return -1;
    }

    rtattr_end(&mut req.nh, info_data);
    rtattr_end(&mut req.nh, link_info);

    if send(
        sock,
        &req as *const newlink_req as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) < 0
    {
        test_print(c"send()".as_ptr());
        return -1;
    }
    netlink_check_answer(sock, true)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_vrf(
    name: *const c_char,
    tabid: u32,
    ifindex: c_int,
    nsfd: c_int,
) -> c_int {
    let mut route_sock: c_int = -1;
    let ret: c_int;
    let mut route_seq: u32 = 0;

    if netlink_sock(&mut route_sock, &mut route_seq, NETLINK_ROUTE) != 0 {
        test_error(c"Failed to open netlink route socket\n".as_ptr());
    }

    ret = __add_vrf(route_sock, {
        let old = route_seq;
        route_seq = route_seq.wrapping_add(1);
        old
    }, name, tabid, ifindex, nsfd);
    close(route_sock);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
