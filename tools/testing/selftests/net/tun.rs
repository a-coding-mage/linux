// SPDX-License-Identifier: GPL-2.0

// Translated from C source. Original includes:
// errno.h, fcntl.h, stdio.h, stdlib.h, string.h, unistd.h,
// linux/if_tun.h, sys/ioctl.h, sys/socket.h,
// "kselftest_harness.h", "tuntap_helpers.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use libc::*;

static param_dev_geneve_name: &[u8; 8] = b"geneve1\0";
static mut param_hwaddr_outer_dst: [c_uchar; 6] = [0x00, 0xfe, 0x98, 0x14, 0x22, 0x42];
static mut param_hwaddr_outer_src: [c_uchar; 6] = [0x00, 0xfe, 0x98, 0x94, 0xd2, 0x43];
static mut param_hwaddr_inner_dst: [c_uchar; 6] = [0x00, 0xfe, 0x98, 0x94, 0x22, 0xcc];
static mut param_hwaddr_inner_src: [c_uchar; 6] = [0x00, 0xfe, 0x98, 0x94, 0xd2, 0xdd];

const fn __constant_htonl(x: u32) -> u32 {
    x.to_be()
}

static mut param_ipaddr4_outer_dst: in_addr = in_addr { s_addr: __constant_htonl(0xac100001) };
static mut param_ipaddr4_outer_src: in_addr = in_addr { s_addr: __constant_htonl(0xac100002) };
static mut param_ipaddr4_inner_dst: in_addr = in_addr { s_addr: __constant_htonl(0xac100101) };
static mut param_ipaddr4_inner_src: in_addr = in_addr { s_addr: __constant_htonl(0xac100102) };

static mut param_ipaddr6_outer_dst: in6_addr = in6_addr { s6_addr: [0xfd, 0x00, 0x0d, 0xb8, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] };
static mut param_ipaddr6_outer_src: in6_addr = in6_addr { s6_addr: [0xfd, 0x00, 0x0d, 0xb8, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2] };
static mut param_ipaddr6_inner_dst: in6_addr = in6_addr { s6_addr: [0xfd, 0x00, 0x0d, 0xb8, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] };
static mut param_ipaddr6_inner_src: in6_addr = in6_addr { s6_addr: [0xfd, 0x00, 0x0d, 0xb8, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2] };

const fn BIT(nr: usize) -> c_ulong {
    1u64.wrapping_shl(nr as u32) as c_ulong
}

const VN_ID: c_int = 1;
const VN_PORT: c_int = 4789;
const UDP_SRC_PORT: c_int = 22;
const UDP_DST_PORT: c_int = 48878;
const IPPREFIX_LEN: c_int = 24;
const IP6PREFIX_LEN: c_int = 64;
const TIMEOUT_SEC: c_long = 10;
const TIMEOUT_USEC: useconds_t = 100000;
const MAX_RETRIES: c_int = 20;

const UDP_TUNNEL_GENEVE_4IN4: c_int = 0x01;
const UDP_TUNNEL_GENEVE_6IN4: c_int = 0x02;
const UDP_TUNNEL_GENEVE_4IN6: c_int = 0x04;
const UDP_TUNNEL_GENEVE_6IN6: c_int = 0x08;

const UDP_TUNNEL_MAX_SEGMENTS: c_int = BIT(7) as c_int;

const UDP_TUNNEL_OUTER_IPV4: c_int = UDP_TUNNEL_GENEVE_4IN4 | UDP_TUNNEL_GENEVE_6IN4;
const UDP_TUNNEL_INNER_IPV4: c_int = UDP_TUNNEL_GENEVE_4IN4 | UDP_TUNNEL_GENEVE_4IN6;

const ETH_ALEN: usize = 6;
const ETH_HLEN: c_int = 14;
const ETH_DATA_LEN: c_int = 1500;
const ETH_MAX_MTU: c_int = 65535;
const GENEVE_HLEN: c_int = 8;

const fn UDP_TUNNEL_GENEVE_4IN4_HDRLEN() -> c_int {
    ETH_HLEN + 2 * core::mem::size_of::<iphdr>() as c_int + GENEVE_HLEN + 2 * core::mem::size_of::<udphdr>() as c_int
}
const fn UDP_TUNNEL_GENEVE_6IN6_HDRLEN() -> c_int {
    ETH_HLEN + 2 * core::mem::size_of::<ipv6hdr>() as c_int + GENEVE_HLEN + 2 * core::mem::size_of::<udphdr>() as c_int
}
const fn UDP_TUNNEL_GENEVE_4IN6_HDRLEN() -> c_int {
    ETH_HLEN + core::mem::size_of::<iphdr>() as c_int + core::mem::size_of::<ipv6hdr>() as c_int + GENEVE_HLEN + 2 * core::mem::size_of::<udphdr>() as c_int
}
const fn UDP_TUNNEL_GENEVE_6IN4_HDRLEN() -> c_int {
    ETH_HLEN + core::mem::size_of::<ipv6hdr>() as c_int + core::mem::size_of::<iphdr>() as c_int + GENEVE_HLEN + 2 * core::mem::size_of::<udphdr>() as c_int
}

const fn UDP_TUNNEL_HDRLEN(tunnel_type: c_int) -> c_int {
    if tunnel_type == UDP_TUNNEL_GENEVE_4IN4 {
        UDP_TUNNEL_GENEVE_4IN4_HDRLEN()
    } else if tunnel_type == UDP_TUNNEL_GENEVE_6IN6 {
        UDP_TUNNEL_GENEVE_6IN6_HDRLEN()
    } else if tunnel_type == UDP_TUNNEL_GENEVE_4IN6 {
        UDP_TUNNEL_GENEVE_4IN6_HDRLEN()
    } else if tunnel_type == UDP_TUNNEL_GENEVE_6IN4 {
        UDP_TUNNEL_GENEVE_6IN4_HDRLEN()
    } else {
        0
    }
}

const fn UDP_TUNNEL_MSS(tunnel_type: c_int) -> c_int {
    ETH_DATA_LEN - UDP_TUNNEL_HDRLEN(tunnel_type)
}

const fn UDP_TUNNEL_MAX(tunnel_type: c_int, is_tap: bool) -> c_int {
    ETH_MAX_MTU - UDP_TUNNEL_HDRLEN(tunnel_type) - if is_tap { ETH_HLEN } else { 0 }
}

const TUN_VNET_TNL_SIZE: usize = core::mem::size_of::<virtio_net_hdr_v1_hash_tunnel>();
const MAX_VNET_TUNNEL_PACKET_SZ: usize =
    TUN_VNET_TNL_SIZE + ETH_HLEN as usize + UDP_TUNNEL_GENEVE_6IN6_HDRLEN() as usize + ETH_MAX_MTU as usize;

#[repr(C)]
union geneve_remote {
    r4: in_addr,
    r6: in6_addr,
}

#[repr(C)]
struct geneve_setup_config {
    family: c_int,
    remote: geneve_remote,
    vnid: u32,
    vnport: u16,
    hwaddr: [c_uchar; 6],
    csum: u8,
}

extern "C" {
    fn ip_link_del(dev: *const c_char) -> c_int;
    fn ip_link_add(dev: *const c_char, kind: *const c_char, fill: extern "C" fn(*mut rt_link_newlink_req, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn ip_addr_add(intf: *const c_char, family: c_int, addr: *mut c_void, prefix: c_int) -> c_int;
    fn ip_neigh_add(intf: *const c_char, family: c_int, addr: *mut c_void, hwaddr: *mut c_void) -> c_int;
    fn ip_route_get(intf: *const c_char, family: c_int, table: u8, addr: *mut c_void, parse: extern "C" fn(*mut rt_route_getroute_rsp, *mut c_void), data: *mut c_void) -> c_int;
    fn rt_link_newlink_req_set_address(req: *mut rt_link_newlink_req, addr: *mut c_uchar, len: usize);
    fn rt_link_newlink_req_set_linkinfo_data_geneve_id(req: *mut rt_link_newlink_req, id: u32);
    fn rt_link_newlink_req_set_linkinfo_data_geneve_port(req: *mut rt_link_newlink_req, port: u16);
    fn rt_link_newlink_req_set_linkinfo_data_geneve_udp_csum(req: *mut rt_link_newlink_req, csum: u8);
    fn rt_link_newlink_req_set_linkinfo_data_geneve_remote(req: *mut rt_link_newlink_req, remote: u32);
    fn rt_link_newlink_req_set_linkinfo_data_geneve_remote6(req: *mut rt_link_newlink_req, remote: *mut in6_addr, len: usize);
    fn build_virtio_net_hdr_v1_hash_tunnel(cur: *mut u8, is_tap: bool, hlen: c_int, gso_size: c_int, outer_family: c_int, inner_family: c_int) -> c_int;
    fn build_eth(cur: *mut u8, proto: c_int, dst: *mut c_void, src: *mut c_void) -> c_int;
    fn build_ipv4_header(cur: *mut u8, proto: c_int, pktlen: c_int, dst: *mut c_void, src: *mut c_void) -> c_int;
    fn build_ipv6_header(cur: *mut u8, proto: c_int, flow: c_int, pktlen: c_int, dst: *mut c_void, src: *mut c_void) -> c_int;
    fn build_udp_header(cur: *mut u8, src: c_int, dst: c_int, len: c_int) -> c_int;
    fn build_geneve_header(cur: *mut u8, vnid: c_int) -> c_int;
    fn build_udp_packet(cur: *mut u8, dst: c_int, src: c_int, payload_len: c_int, family: c_int, csum: bool) -> c_int;
    fn build_udp_packet_csum(udph: *mut u8, family: c_int, csum: bool);
}

#[repr(C)]
struct rt_link_newlink_req {
    _private: [u8; 0],
}

#[repr(C)]
struct rtmsg_hdr {
    rtm_type: u8,
}

#[repr(C)]
struct rt_route_getroute_rsp {
    _hdr: rtmsg_hdr,
}

#[repr(C)]
struct iphdr {
    version_ihl: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

impl iphdr {
    unsafe fn version(&self) -> u8 {
        self.version_ihl >> 4
    }
}

#[repr(C)]
struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: u16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

impl ipv6hdr {
    unsafe fn version(&self) -> u8 {
        self.priority_version >> 4
    }
}

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
struct virtio_net_hdr_v1 {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
    num_buffers: u16,
}

#[repr(C)]
struct virtio_net_hdr_v1_hash {
    hdr: virtio_net_hdr_v1,
}

#[repr(C)]
struct virtio_net_hdr_v1_hash_tunnel {
    hash_hdr: virtio_net_hdr_v1_hash,
}

const O_RDWR: c_int = libc::O_RDWR;
const IFF_ATTACH_QUEUE: c_short = 0x0200;
const IFF_DETACH_QUEUE: c_short = 0x0400;
const IFF_TUN: c_int = 0x0001;
const IFF_TAP: c_int = 0x0002;
const IFF_NAPI: c_int = 0x0010;
const IFF_NO_PI: c_int = 0x1000;
const IFF_MULTI_QUEUE: c_int = 0x0100;
const IFF_VNET_HDR: c_int = 0x4000;
const IFF_UP: c_short = 0x1;
const IFF_RUNNING: c_short = 0x40;
const TUNSETIFF: c_ulong = 0x400454ca;
const TUNSETQUEUE: c_ulong = 0x400454d9;
const TUNSETVNETHDRSZ: c_ulong = 0x400454d8;
const TUNSETOFFLOAD: c_ulong = 0x400454d0;
const TUN_F_CSUM: c_int = 0x01;
const TUN_F_UDP_TUNNEL_GSO: c_int = 0x40;
const TUN_F_UDP_TUNNEL_GSO_CSUM: c_int = 0x80;
const TUN_F_USO4: c_int = 0x10;
const TUN_F_USO6: c_int = 0x20;
const ARPHRD_ETHER: sa_family_t = 1;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_IPV6: c_int = 0x86DD;
const SOL_UDP: c_int = 17;
const UDP_SEGMENT: c_int = 103;
const IPPROTO_UDP: c_int = 17;
const IP_MTU_DISCOVER: c_int = 10;
const IP_PMTUDISC_DO: c_int = 2;
const IPV6_MTU_DISCOVER: c_int = 23;
const IPV6_PMTUDISC_DO: c_int = 2;
const RT_TABLE_LOCAL: u8 = 255;
const RTN_LOCAL: u8 = 2;
const VIRTIO_NET_HDR_GSO_UDP_L4: c_int = 5;
const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4: c_int = 0x10;
const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6: c_int = 0x20;

unsafe fn tun_attach(fd: c_int, dev: *mut c_char) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    strcpy(ifr.ifr_name.as_mut_ptr(), dev);
    ifr.ifr_ifru.ifru_flags = IFF_ATTACH_QUEUE;
    ioctl(fd, TUNSETQUEUE, &mut ifr as *mut _ as *mut c_void)
}

unsafe fn tun_detach(fd: c_int, dev: *mut c_char) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    strcpy(ifr.ifr_name.as_mut_ptr(), dev);
    ifr.ifr_ifru.ifru_flags = IFF_DETACH_QUEUE;
    ioctl(fd, TUNSETQUEUE, &mut ifr as *mut _ as *mut c_void)
}

unsafe fn tun_alloc(dev: *mut c_char) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    let fd = open(c"/dev/net/tun".as_ptr(), O_RDWR);
    if fd < 0 {
        fprintf(stderr, c"can't open tun: %s\n".as_ptr(), strerror(*__errno_location()));
        return fd;
    }

    strcpy(ifr.ifr_name.as_mut_ptr(), dev);
    ifr.ifr_ifru.ifru_flags = (IFF_TAP | IFF_NAPI | IFF_MULTI_QUEUE) as c_short;

    let err = ioctl(fd, TUNSETIFF, &mut ifr as *mut _ as *mut c_void);
    if err < 0 {
        fprintf(stderr, c"can't TUNSETIFF: %s\n".as_ptr(), strerror(*__errno_location()));
        close(fd);
        return err;
    }
    strcpy(dev, ifr.ifr_name.as_ptr());
    fd
}

unsafe fn tun_delete(dev: *mut c_char) -> c_int {
    ip_link_del(dev)
}

unsafe fn tun_open(dev: *mut c_char, flags: c_int, hdrlen: c_int, features: c_int, mac_addr: *const c_uchar) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    let mut sk: c_int = -1;

    let mut fd = open(c"/dev/net/tun".as_ptr(), O_RDWR);
    if fd < 0 {
        perror(c"open".as_ptr());
        return -1;
    }

    ifr.ifr_ifru.ifru_flags = flags as c_short;
    if ioctl(fd, TUNSETIFF, &mut ifr as *mut _ as *mut c_void) < 0 {
        perror(c"ioctl(TUNSETIFF)".as_ptr());
        close(fd);
        return -1;
    }
    strcpy(dev, ifr.ifr_name.as_ptr());

    if hdrlen > 0 {
        if ioctl(fd, TUNSETVNETHDRSZ, &hdrlen as *const _ as *mut c_void) < 0 {
            perror(c"ioctl(TUNSETVNETHDRSZ)".as_ptr());
            close(fd);
            return -1;
        }
    }

    if features != 0 {
        if ioctl(fd, TUNSETOFFLOAD, features) < 0 {
            perror(c"ioctl(TUNSETOFFLOAD)".as_ptr());
            close(fd);
            return -1;
        }
    }

    sk = socket(PF_INET, SOCK_DGRAM, 0);
    if sk < 0 {
        perror(c"socket".as_ptr());
        close(fd);
        return -1;
    }

    if ioctl(sk, SIOCGIFFLAGS as c_ulong, &mut ifr as *mut _ as *mut c_void) < 0 {
        perror(c"ioctl(SIOCGIFFLAGS)".as_ptr());
        close(sk);
        close(fd);
        return -1;
    }

    ifr.ifr_ifru.ifru_flags |= IFF_UP | IFF_RUNNING;
    if ioctl(sk, SIOCSIFFLAGS as c_ulong, &mut ifr as *mut _ as *mut c_void) < 0 {
        perror(c"ioctl(SIOCSIFFLAGS)".as_ptr());
        close(sk);
        close(fd);
        return -1;
    }

    if !mac_addr.is_null() && (flags & IFF_TAP) != 0 {
        ifr.ifr_ifru.ifru_hwaddr.sa_family = ARPHRD_ETHER;
        memcpy(ifr.ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *mut c_void, mac_addr as *const c_void, ETH_ALEN);

        if ioctl(sk, SIOCSIFHWADDR as c_ulong, &mut ifr as *mut _ as *mut c_void) < 0 {
            perror(c"ioctl(SIOCSIFHWADDR)".as_ptr());
            close(sk);
            close(fd);
            return -1;
        }
    }

    if sk >= 0 {
        close(sk);
    }
    fd
}

const fn sockaddr_len(family: c_int) -> usize {
    if family == AF_INET {
        core::mem::size_of::<sockaddr_in>()
    } else {
        core::mem::size_of::<sockaddr_in6>()
    }
}

extern "C" fn geneve_fill_newlink(req: *mut rt_link_newlink_req, data: *mut c_void) -> c_int {
    unsafe {
        let cfg = data as *mut geneve_setup_config;

        rt_link_newlink_req_set_address(req, (*cfg).hwaddr.as_mut_ptr(), ETH_ALEN);
        rt_link_newlink_req_set_linkinfo_data_geneve_id(req, (*cfg).vnid);
        rt_link_newlink_req_set_linkinfo_data_geneve_port(req, (*cfg).vnport);
        rt_link_newlink_req_set_linkinfo_data_geneve_udp_csum(req, (*cfg).csum);

        if (*cfg).family == AF_INET {
            rt_link_newlink_req_set_linkinfo_data_geneve_remote(req, (*cfg).remote.r4.s_addr);
        } else {
            rt_link_newlink_req_set_linkinfo_data_geneve_remote6(req, &mut (*cfg).remote.r6, core::mem::size_of::<in6_addr>());
        }

        0
    }
}

unsafe fn geneve_create(dev: *const c_char, family: c_int, remote: *mut c_void, hwaddr: *mut c_void) -> c_int {
    let mut geneve: geneve_setup_config = core::mem::zeroed();
    geneve.vnid = VN_ID as u32;
    geneve.vnport = htons(VN_PORT as u16);
    geneve.csum = 1;
    geneve.family = family;
    if family == AF_INET {
        memcpy(&mut geneve.remote.r4 as *mut _ as *mut c_void, remote, core::mem::size_of::<in_addr>());
    } else {
        memcpy(&mut geneve.remote.r6 as *mut _ as *mut c_void, remote, core::mem::size_of::<in6_addr>());
    }
    memcpy(geneve.hwaddr.as_mut_ptr() as *mut c_void, hwaddr, ETH_ALEN);

    ip_link_add(dev, c"geneve".as_ptr(), geneve_fill_newlink, &mut geneve as *mut _ as *mut c_void)
}

unsafe fn set_pmtu_discover(fd: c_int, is_ipv4: bool) -> c_int {
    let (level, name, val) = if is_ipv4 {
        (SOL_IP, IP_MTU_DISCOVER, IP_PMTUDISC_DO)
    } else {
        (SOL_IPV6, IPV6_MTU_DISCOVER, IPV6_PMTUDISC_DO)
    };

    setsockopt(fd, level, name, &val as *const _ as *const c_void, core::mem::size_of_val(&val) as socklen_t)
}

unsafe fn udp_socket_open(ssa: *mut sockaddr_storage, do_frag: bool, do_connect: bool, dsa: *mut sockaddr_storage) -> c_int {
    let mut to = timeval { tv_sec: TIMEOUT_SEC, tv_usec: 0 };
    let family = (*ssa).ss_family as c_int;
    let salen = sockaddr_len(family) as socklen_t;

    let fd = socket(family, SOCK_DGRAM, 0);
    if fd < 0 {
        return -1;
    }

    if bind(fd, ssa as *mut sockaddr, salen) < 0 {
        perror(c"bind".as_ptr());
        close(fd);
        return -1;
    }

    if do_connect && connect(fd, dsa as *mut sockaddr, salen) < 0 {
        perror(c"connect".as_ptr());
        close(fd);
        return -1;
    }

    if setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &mut to as *mut _ as *mut c_void, core::mem::size_of_val(&to) as socklen_t) < 0 {
        perror(c"setsockopt(SO_RCVTIMEO)".as_ptr());
        close(fd);
        return -1;
    }

    if !do_frag && set_pmtu_discover(fd, family == AF_INET) < 0 {
        perror(c"set_pmtu_discover".as_ptr());
        close(fd);
        return -1;
    }
    fd
}

extern "C" fn parse_route_rsp(rsp: *mut rt_route_getroute_rsp, rtm_type: *mut c_void) {
    unsafe {
        *(rtm_type as *mut u8) = (*rsp)._hdr.rtm_type;
    }
}

unsafe fn ip_route_check(intf: *const c_char, family: c_int, addr: *mut c_void) -> c_int {
    let mut rtm_type: u8 = 0;
    let table: u8 = RT_TABLE_LOCAL;
    let mut retries = MAX_RETRIES;

    while {
        let old = retries;
        retries -= 1;
        old > 0
    } {
        if ip_route_get(intf, family, table, addr, parse_route_rsp, &mut rtm_type as *mut _ as *mut c_void) == 0
            && rtm_type == RTN_LOCAL
        {
            break;
        }

        usleep(TIMEOUT_USEC);
    }

    if retries < 0 {
        return -1;
    }

    0
}

unsafe fn send_gso_udp_msg(socket_fd: c_int, addr: *mut sockaddr_storage, send_buf: *mut u8, send_len: c_int, gso_size: c_int) -> c_int {
    let mut control = [0u8; 64];
    let alen = sockaddr_len((*addr).ss_family as c_int) as socklen_t;
    let mut msg: msghdr = core::mem::zeroed();
    let mut iov: iovec = core::mem::zeroed();

    iov.iov_base = send_buf as *mut c_void;
    iov.iov_len = send_len as usize;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_name = addr as *mut c_void;
    msg.msg_namelen = alen;

    if gso_size > 0 {
        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = control.len();

        let cmsg = CMSG_FIRSTHDR(&msg);
        (*cmsg).cmsg_level = SOL_UDP;
        (*cmsg).cmsg_type = UDP_SEGMENT;
        (*cmsg).cmsg_len = CMSG_LEN(core::mem::size_of::<u16>() as c_uint) as usize;
        *(CMSG_DATA(cmsg) as *mut u16) = gso_size as u16;
    }

    let ret = sendmsg(socket_fd, &msg, 0) as c_int;
    if ret < 0 {
        perror(c"sendmsg".as_ptr());
    }

    ret
}

unsafe fn validate_hdrlen(cur: *mut *mut u8, len: *mut c_int, x: c_int) -> c_int {
    if *len < x {
        return -1;
    }
    *cur = (*cur).add(x as usize);
    *len -= x;
    0
}

unsafe fn parse_udp_tunnel_vnet_packet(buf: *mut u8, mut len: c_int, tunnel_type: c_int, is_tap: bool) -> c_int {
    let mut cur = buf;

    if validate_hdrlen(&mut cur, &mut len, TUN_VNET_TNL_SIZE as c_int) != 0 {
        return -1;
    }

    if is_tap {
        if validate_hdrlen(&mut cur, &mut len, ETH_HLEN) != 0 {
            return -1;
        }
    }

    if (tunnel_type & UDP_TUNNEL_OUTER_IPV4) != 0 {
        let iph4 = cur as *mut iphdr;
        if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<iphdr>() as c_int) != 0 {
            return -1;
        }
        if (*iph4).version() != 4 || (*iph4).protocol != IPPROTO_UDP as u8 {
            return -1;
        }
    } else {
        let iph6 = cur as *mut ipv6hdr;
        if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<ipv6hdr>() as c_int) != 0 {
            return -1;
        }
        if (*iph6).version() != 6 || (*iph6).nexthdr != IPPROTO_UDP as u8 {
            return -1;
        }
    }

    let mut udph = cur as *mut udphdr;
    if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<udphdr>() as c_int) != 0 {
        return -1;
    }
    if ntohs((*udph).dest) as c_int != VN_PORT {
        return -1;
    }

    if validate_hdrlen(&mut cur, &mut len, GENEVE_HLEN) != 0 {
        return -1;
    }
    if validate_hdrlen(&mut cur, &mut len, ETH_HLEN) != 0 {
        return -1;
    }

    if (tunnel_type & UDP_TUNNEL_INNER_IPV4) != 0 {
        let iph4 = cur as *mut iphdr;
        if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<iphdr>() as c_int) != 0 {
            return -1;
        }
        if (*iph4).version() != 4 || (*iph4).protocol != IPPROTO_UDP as u8 {
            return -1;
        }
    } else {
        let iph6 = cur as *mut ipv6hdr;
        if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<ipv6hdr>() as c_int) != 0 {
            return -1;
        }
        if (*iph6).version() != 6 || (*iph6).nexthdr != IPPROTO_UDP as u8 {
            return -1;
        }
    }

    udph = cur as *mut udphdr;
    if validate_hdrlen(&mut cur, &mut len, core::mem::size_of::<udphdr>() as c_int) != 0 {
        return -1;
    }
    if ntohs((*udph).dest) as c_int != UDP_DST_PORT {
        return -1;
    }

    len
}

#[repr(C)]
struct tun {
    ifname: [c_char; IFNAMSIZ],
    fd: c_int,
    fd2: c_int,
}

unsafe fn tun_setup(self_: *mut tun) {
    memset((*self_).ifname.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*self_).ifname));

    (*self_).fd = tun_alloc((*self_).ifname.as_mut_ptr());
    assert!((*self_).fd >= 0);

    (*self_).fd2 = tun_alloc((*self_).ifname.as_mut_ptr());
    assert!((*self_).fd2 >= 0);
}

unsafe fn tun_teardown(self_: *mut tun) {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
    if (*self_).fd2 >= 0 {
        close((*self_).fd2);
    }
}

unsafe fn tun_delete_detach_close(self_: *mut tun) {
    assert_eq!(tun_delete((*self_).ifname.as_mut_ptr()), 0);
    assert_eq!(tun_detach((*self_).fd, (*self_).ifname.as_mut_ptr()), -1);
    assert_eq!(*__errno_location(), 22);
}

unsafe fn tun_detach_delete_close(self_: *mut tun) {
    assert_eq!(tun_detach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    assert_eq!(tun_delete((*self_).ifname.as_mut_ptr()), 0);
}

unsafe fn tun_detach_close_delete(self_: *mut tun) {
    assert_eq!(tun_detach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    close((*self_).fd);
    (*self_).fd = -1;
    assert_eq!(tun_delete((*self_).ifname.as_mut_ptr()), 0);
}

unsafe fn tun_reattach_delete_close(self_: *mut tun) {
    assert_eq!(tun_detach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    assert_eq!(tun_attach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    assert_eq!(tun_delete((*self_).ifname.as_mut_ptr()), 0);
}

unsafe fn tun_reattach_close_delete(self_: *mut tun) {
    assert_eq!(tun_detach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    assert_eq!(tun_attach((*self_).fd, (*self_).ifname.as_mut_ptr()), 0);
    close((*self_).fd);
    (*self_).fd = -1;
    assert_eq!(tun_delete((*self_).ifname.as_mut_ptr()), 0);
}

#[repr(C)]
struct tun_vnet_udptnl {
    ifname: [c_char; IFNAMSIZ],
    fd: c_int,
    sock: c_int,
}

#[repr(C)]
struct tun_vnet_udptnl_variant {
    tunnel_type: c_int,
    gso_size: c_int,
    data_size: c_int,
    r_num_mss: c_int,
    is_tap: bool,
    no_gso: bool,
}

// clang-format off
// Original C used TUN_VNET_UDPTNL_VARIANT_ADD to register kselftest variants.
// The direct Rust equivalent is this table of variant data with comments preserved by name.
static tun_vnet_udptnl_variants: &[(&str, tun_vnet_udptnl_variant)] = &[
    ("4in4_nogsosz_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 0, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_nogsosz_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_nogsosz_gtmss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) + 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 1, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_ltgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) + 1, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in4_gtgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) + 1, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("4in4_2mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) * 2, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("4in4_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), data_size: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_4IN4, true), r_num_mss: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_4IN4, true) / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) + 1, is_tap: true, no_gso: false }),
    ("4in4_over_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4), data_size: ETH_MAX_MTU, r_num_mss: ETH_MAX_MTU / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN4) + 1, is_tap: true, no_gso: false }),
    ("4in4_maxsegs", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 1, data_size: UDP_TUNNEL_MAX_SEGMENTS, r_num_mss: UDP_TUNNEL_MAX_SEGMENTS, is_tap: true, no_gso: false }),
    ("4in4_5byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN4, gso_size: 2, data_size: 5, r_num_mss: 3, is_tap: true, no_gso: false }),
    ("6in4_nogsosz_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 0, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_nogsosz_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_nogsosz_gtmss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) + 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 1, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_ltgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) + 1, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in4_gtgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) + 1, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("6in4_2mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) * 2, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("6in4_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), data_size: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_6IN4, true), r_num_mss: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_6IN4, true) / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) + 1, is_tap: true, no_gso: false }),
    ("6in4_over_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4), data_size: ETH_MAX_MTU, r_num_mss: ETH_MAX_MTU / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN4) + 1, is_tap: true, no_gso: false }),
    ("6in4_maxsegs", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 1, data_size: UDP_TUNNEL_MAX_SEGMENTS, r_num_mss: UDP_TUNNEL_MAX_SEGMENTS, is_tap: true, no_gso: false }),
    ("6in4_5byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN4, gso_size: 2, data_size: 5, r_num_mss: 3, is_tap: true, no_gso: false }),
    ("4in6_nogsosz_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 0, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_nogsosz_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_nogsosz_gtmss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) + 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 1, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_ltgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) + 1, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("4in6_gtgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) + 1, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("4in6_2mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) * 2, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("4in6_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), data_size: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_4IN6, true), r_num_mss: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_4IN6, true) / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) + 1, is_tap: true, no_gso: false }),
    ("4in6_over_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6), data_size: ETH_MAX_MTU, r_num_mss: ETH_MAX_MTU / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_4IN6) + 1, is_tap: true, no_gso: false }),
    ("4in6_maxsegs", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 1, data_size: UDP_TUNNEL_MAX_SEGMENTS, r_num_mss: UDP_TUNNEL_MAX_SEGMENTS, is_tap: true, no_gso: false }),
    ("4in6_5byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_4IN6, gso_size: 2, data_size: 5, r_num_mss: 3, is_tap: true, no_gso: false }),
    ("6in6_nogsosz_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 0, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_nogsosz_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_nogsosz_gtmss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 0, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) + 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_1byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 1, data_size: 1, r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_1mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_ltgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) + 1, data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), r_num_mss: 1, is_tap: true, no_gso: true }),
    ("6in6_gtgso", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) + 1, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("6in6_2mss", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), data_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) * 2, r_num_mss: 2, is_tap: true, no_gso: false }),
    ("6in6_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), data_size: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_6IN6, true), r_num_mss: UDP_TUNNEL_MAX(UDP_TUNNEL_GENEVE_6IN6, true) / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) + 1, is_tap: true, no_gso: false }),
    ("6in6_over_maxbytes", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6), data_size: ETH_MAX_MTU, r_num_mss: ETH_MAX_MTU / UDP_TUNNEL_MSS(UDP_TUNNEL_GENEVE_6IN6) + 1, is_tap: true, no_gso: false }),
    ("6in6_maxsegs", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 1, data_size: UDP_TUNNEL_MAX_SEGMENTS, r_num_mss: UDP_TUNNEL_MAX_SEGMENTS, is_tap: true, no_gso: false }),
    ("6in6_5byte", tun_vnet_udptnl_variant { tunnel_type: UDP_TUNNEL_GENEVE_6IN6, gso_size: 2, data_size: 5, r_num_mss: 3, is_tap: true, no_gso: false }),
];
// clang-format on

unsafe fn assign_ifaddr_vars(family: c_int, is_outer: c_int, srcip: *mut *mut c_void, dstip: *mut *mut c_void, srcmac: *mut *mut c_void, dstmac: *mut *mut c_void) {
    if is_outer != 0 {
        if family == AF_INET {
            *srcip = &mut param_ipaddr4_outer_src as *mut _ as *mut c_void;
            *dstip = &mut param_ipaddr4_outer_dst as *mut _ as *mut c_void;
        } else {
            *srcip = &mut param_ipaddr6_outer_src as *mut _ as *mut c_void;
            *dstip = &mut param_ipaddr6_outer_dst as *mut _ as *mut c_void;
        }
        *srcmac = param_hwaddr_outer_src.as_mut_ptr() as *mut c_void;
        *dstmac = param_hwaddr_outer_dst.as_mut_ptr() as *mut c_void;
    } else {
        if family == AF_INET {
            *srcip = &mut param_ipaddr4_inner_src as *mut _ as *mut c_void;
            *dstip = &mut param_ipaddr4_inner_dst as *mut _ as *mut c_void;
        } else {
            *srcip = &mut param_ipaddr6_inner_src as *mut _ as *mut c_void;
            *dstip = &mut param_ipaddr6_inner_dst as *mut _ as *mut c_void;
        }
        *srcmac = param_hwaddr_inner_src.as_mut_ptr() as *mut c_void;
        *dstmac = param_hwaddr_inner_dst.as_mut_ptr() as *mut c_void;
    }
}

unsafe fn assign_sockaddr_vars(family: c_int, is_outer: c_int, src: *mut sockaddr_storage, dst: *mut sockaddr_storage) {
    (*src).ss_family = family as sa_family_t;
    (*dst).ss_family = family as sa_family_t;

    if family == AF_INET {
        let s4 = src as *mut sockaddr_in;
        let d4 = dst as *mut sockaddr_in;

        (*s4).sin_addr = if is_outer != 0 { param_ipaddr4_outer_src } else { param_ipaddr4_inner_src };
        (*d4).sin_addr = if is_outer != 0 { param_ipaddr4_outer_dst } else { param_ipaddr4_inner_dst };
        if is_outer == 0 {
            (*s4).sin_port = htons(UDP_SRC_PORT as u16);
            (*d4).sin_port = htons(UDP_DST_PORT as u16);
        }
    } else {
        let s6 = src as *mut sockaddr_in6;
        let d6 = dst as *mut sockaddr_in6;

        (*s6).sin6_addr = if is_outer != 0 { param_ipaddr6_outer_src } else { param_ipaddr6_inner_src };
        (*d6).sin6_addr = if is_outer != 0 { param_ipaddr6_outer_dst } else { param_ipaddr6_inner_dst };
        if is_outer == 0 {
            (*s6).sin6_port = htons(UDP_SRC_PORT as u16);
            (*d6).sin6_port = htons(UDP_DST_PORT as u16);
        }
    }
}

unsafe fn tun_vnet_udptnl_setup(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant) {
    let mut ret: c_int;
    let mut family: c_int;
    let mut prefix: c_int;
    let flags: c_int;
    let features: c_int;
    let tunnel_type = (*variant).tunnel_type;
    let mut ssa: sockaddr_storage = core::mem::zeroed();
    let mut dsa: sockaddr_storage = core::mem::zeroed();
    let mut sip: *mut c_void = core::ptr::null_mut();
    let mut dip: *mut c_void = core::ptr::null_mut();
    let mut smac: *mut c_void = core::ptr::null_mut();
    let mut dmac: *mut c_void = core::ptr::null_mut();

    flags = (if (*variant).is_tap { IFF_TAP } else { IFF_TUN }) | IFF_VNET_HDR | IFF_MULTI_QUEUE | IFF_NO_PI;
    features = TUN_F_CSUM | TUN_F_UDP_TUNNEL_GSO | TUN_F_UDP_TUNNEL_GSO_CSUM | TUN_F_USO4 | TUN_F_USO6;
    (*self_).fd = tun_open((*self_).ifname.as_mut_ptr(), flags, TUN_VNET_TNL_SIZE as c_int, features, param_hwaddr_outer_src.as_ptr());
    assert!((*self_).fd >= 0);

    family = if (tunnel_type & UDP_TUNNEL_OUTER_IPV4) != 0 { AF_INET } else { AF_INET6 };
    prefix = if family == AF_INET { IPPREFIX_LEN } else { IP6PREFIX_LEN };
    assign_ifaddr_vars(family, 1, &mut sip, &mut dip, &mut smac, &mut dmac);

    ret = ip_addr_add((*self_).ifname.as_mut_ptr(), family, sip, prefix);
    assert_eq!(ret, 0);
    ret = ip_neigh_add((*self_).ifname.as_mut_ptr(), family, dip, dmac);
    assert_eq!(ret, 0);
    ret = ip_route_check((*self_).ifname.as_mut_ptr(), family, sip);
    assert_eq!(ret, 0);

    ret = geneve_create(param_dev_geneve_name.as_ptr() as *const c_char, family, dip, param_hwaddr_inner_src.as_mut_ptr() as *mut c_void);
    assert_eq!(ret, 0);

    family = if (tunnel_type & UDP_TUNNEL_INNER_IPV4) != 0 { AF_INET } else { AF_INET6 };
    prefix = if family == AF_INET { IPPREFIX_LEN } else { IP6PREFIX_LEN };
    assign_ifaddr_vars(family, 0, &mut sip, &mut dip, &mut smac, &mut dmac);

    ret = ip_addr_add(param_dev_geneve_name.as_ptr() as *const c_char, family, sip, prefix);
    assert_eq!(ret, 0);
    ret = ip_neigh_add(param_dev_geneve_name.as_ptr() as *const c_char, family, dip, dmac);
    assert_eq!(ret, 0);
    ret = ip_route_check(param_dev_geneve_name.as_ptr() as *const c_char, family, sip);
    assert_eq!(ret, 0);

    assign_sockaddr_vars(family, 0, &mut ssa, &mut dsa);
    (*self_).sock = udp_socket_open(&mut ssa, false, true, &mut dsa);
    assert!((*self_).sock >= 0);
}

unsafe fn tun_vnet_udptnl_teardown(self_: *mut tun_vnet_udptnl) {
    let mut ret: c_int;

    if (*self_).sock != -1 {
        close((*self_).sock);
    }

    ret = ip_link_del(param_dev_geneve_name.as_ptr() as *const c_char);
    assert_eq!(ret, 0);

    ret = tun_delete((*self_).ifname.as_mut_ptr());
    assert_eq!(ret, 0);
}

unsafe fn build_gso_packet_into_tun(variant: *const tun_vnet_udptnl_variant, buf: *mut u8) -> c_int {
    let mut pktlen: c_int;
    let hlen: c_int;
    let mut proto: c_int;
    let inner_family: c_int;
    let outer_family: c_int;
    let tunnel_type = (*variant).tunnel_type;
    let payload_len = (*variant).data_size;
    let gso_size = (*variant).gso_size;
    let mut cur = buf;
    let outer_udph: *mut u8;
    let mut sip: *mut c_void = core::ptr::null_mut();
    let mut dip: *mut c_void = core::ptr::null_mut();
    let mut smac: *mut c_void = core::ptr::null_mut();
    let mut dmac: *mut c_void = core::ptr::null_mut();
    let is_tap = (*variant).is_tap;

    hlen = (if is_tap { ETH_HLEN } else { 0 }) + UDP_TUNNEL_HDRLEN(tunnel_type);
    inner_family = if (tunnel_type & UDP_TUNNEL_INNER_IPV4) != 0 { AF_INET } else { AF_INET6 };
    outer_family = if (tunnel_type & UDP_TUNNEL_OUTER_IPV4) != 0 { AF_INET } else { AF_INET6 };

    cur = cur.add(build_virtio_net_hdr_v1_hash_tunnel(cur, is_tap, hlen, gso_size, outer_family, inner_family) as usize);

    pktlen = hlen + payload_len;
    assign_ifaddr_vars(outer_family, 1, &mut sip, &mut dip, &mut smac, &mut dmac);

    if is_tap {
        proto = if outer_family == AF_INET { ETH_P_IP } else { ETH_P_IPV6 };
        pktlen -= ETH_HLEN;
        cur = cur.add(build_eth(cur, proto, dmac, smac) as usize);
    }

    if outer_family == AF_INET {
        pktlen -= core::mem::size_of::<iphdr>() as c_int;
        cur = cur.add(build_ipv4_header(cur, IPPROTO_UDP, pktlen, dip, sip) as usize);
    } else {
        pktlen -= core::mem::size_of::<ipv6hdr>() as c_int;
        cur = cur.add(build_ipv6_header(cur, IPPROTO_UDP, 0, pktlen, dip, sip) as usize);
    }

    outer_udph = cur;
    assign_ifaddr_vars(inner_family, 0, &mut sip, &mut dip, &mut smac, &mut dmac);

    pktlen -= core::mem::size_of::<udphdr>() as c_int;
    proto = if inner_family == AF_INET { ETH_P_IP } else { ETH_P_IPV6 };
    cur = cur.add(build_udp_header(cur, UDP_SRC_PORT, VN_PORT, pktlen) as usize);
    cur = cur.add(build_geneve_header(cur, VN_ID) as usize);
    cur = cur.add(build_eth(cur, proto, dmac, smac) as usize);

    pktlen = core::mem::size_of::<udphdr>() as c_int + payload_len;
    if inner_family == AF_INET {
        cur = cur.add(build_ipv4_header(cur, IPPROTO_UDP, pktlen, dip, sip) as usize);
    } else {
        cur = cur.add(build_ipv6_header(cur, IPPROTO_UDP, 0, pktlen, dip, sip) as usize);
    }

    cur = cur.add(build_udp_packet(cur, UDP_DST_PORT, UDP_SRC_PORT, payload_len, inner_family, false) as usize);

    build_udp_packet_csum(outer_udph, outer_family, false);

    cur.offset_from(buf) as c_int
}

unsafe fn receive_gso_packet_from_tunnel(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant, r_num_mss: *mut c_int) -> c_int {
    let mut packet_buf = [0u8; MAX_VNET_TUNNEL_PACKET_SZ];
    let mut total_len: c_int = 0;
    let socket_fd = (*self_).sock;
    let payload_len = (*variant).data_size;

    while total_len < payload_len {
        let len = recv(socket_fd, packet_buf.as_mut_ptr() as *mut c_void, packet_buf.len(), 0) as c_int;
        if len <= 0 {
            if len < 0 && *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                perror(c"recv".as_ptr());
            }
            break;
        }

        *r_num_mss += 1;
        total_len += len;
    }

    total_len
}

unsafe fn send_gso_packet_into_tunnel(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant) -> c_int {
    let family = if ((*variant).tunnel_type & UDP_TUNNEL_INNER_IPV4) != 0 { AF_INET } else { AF_INET6 };
    let mut buf = [0u8; MAX_VNET_TUNNEL_PACKET_SZ];
    let payload_len = (*variant).data_size;
    let gso_size = (*variant).gso_size;
    let mut ssa: sockaddr_storage = core::mem::zeroed();
    let mut dsa: sockaddr_storage = core::mem::zeroed();

    assign_sockaddr_vars(family, 0, &mut ssa, &mut dsa);
    send_gso_udp_msg((*self_).sock, &mut dsa, buf.as_mut_ptr(), payload_len, gso_size)
}

unsafe fn receive_gso_packet_from_tun(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant, vnet_hdr: *mut virtio_net_hdr_v1_hash_tunnel) -> c_int {
    let mut timeout = timeval { tv_sec: TIMEOUT_SEC, tv_usec: 0 };
    let mut buf = [0u8; MAX_VNET_TUNNEL_PACKET_SZ];
    let tunnel_type = (*variant).tunnel_type;
    let payload_len = (*variant).data_size;
    let is_tap = (*variant).is_tap;
    let mut total_len: c_int = 0;
    let tun_fd = (*self_).fd;
    let mut fdset: fd_set = core::mem::zeroed();

    while total_len < payload_len {
        FD_ZERO(&mut fdset);
        FD_SET(tun_fd, &mut fdset);

        let ret = select(tun_fd + 1, &mut fdset, core::ptr::null_mut(), core::ptr::null_mut(), &mut timeout);
        if ret <= 0 {
            perror(c"select".as_ptr());
            break;
        }
        if !FD_ISSET(tun_fd, &mut fdset) {
            continue;
        }

        let mut len = read(tun_fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int;
        if len <= 0 {
            if len < 0 && *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                perror(c"read".as_ptr());
            }
            break;
        }

        len = parse_udp_tunnel_vnet_packet(buf.as_mut_ptr(), len, tunnel_type, is_tap);
        if len < 0 {
            continue;
        }

        if total_len == 0 {
            memcpy(vnet_hdr as *mut c_void, buf.as_ptr() as *const c_void, TUN_VNET_TNL_SIZE);
        }

        total_len += len;
    }

    total_len
}

unsafe fn tun_vnet_udptnl_send_gso_packet(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant) {
    let mut pkt = [0u8; MAX_VNET_TUNNEL_PACKET_SZ];
    let mut r_num_mss: c_int = 0;

    memset(pkt.as_mut_ptr() as *mut c_void, 0, pkt.len());
    let off = build_gso_packet_into_tun(variant, pkt.as_mut_ptr());
    let mut ret = write((*self_).fd, pkt.as_ptr() as *const c_void, off as usize) as c_int;
    assert_eq!(ret, off);

    ret = receive_gso_packet_from_tunnel(self_, variant, &mut r_num_mss);
    assert_eq!(ret, (*variant).data_size);
    assert_eq!(r_num_mss, (*variant).r_num_mss);
}

unsafe fn tun_vnet_udptnl_recv_gso_packet(self_: *mut tun_vnet_udptnl, variant: *const tun_vnet_udptnl_variant) {
    let mut vnet_hdr: virtio_net_hdr_v1_hash_tunnel = core::mem::zeroed();
    let vh: *mut virtio_net_hdr_v1 = &mut vnet_hdr.hash_hdr.hdr;
    let mut gso_type = VIRTIO_NET_HDR_GSO_UDP_L4;

    let mut ret = send_gso_packet_into_tunnel(self_, variant);
    assert_eq!(ret, (*variant).data_size);

    memset(&mut vnet_hdr as *mut _ as *mut c_void, 0, core::mem::size_of_val(&vnet_hdr));
    ret = receive_gso_packet_from_tun(self_, variant, &mut vnet_hdr);
    assert_eq!(ret, (*variant).data_size);

    if !(*variant).no_gso {
        assert_eq!((*vh).gso_size, (*variant).gso_size as u16);
        gso_type |= if ((*variant).tunnel_type & UDP_TUNNEL_OUTER_IPV4) != 0 {
            VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4
        } else {
            VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6
        };
        assert_eq!((*vh).gso_type, gso_type as u8);
    }
}

// Original expected-failure registrations:
// XFAIL_ADD(tun_vnet_udptnl, 4in4_nogsosz_gtmss, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in4_nogsosz_gtmss, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 4in6_nogsosz_gtmss, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in6_nogsosz_gtmss, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 4in4_over_maxbytes, send_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in4_over_maxbytes, send_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 4in6_over_maxbytes, send_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in6_over_maxbytes, send_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 4in4_over_maxbytes, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in4_over_maxbytes, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 4in6_over_maxbytes, recv_gso_packet);
// XFAIL_ADD(tun_vnet_udptnl, 6in6_over_maxbytes, recv_gso_packet);

fn main() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
