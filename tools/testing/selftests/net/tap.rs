// SPDX-License-Identifier: GPL-2.0

// Translated from C. Original dependency intent:
// _GNU_SOURCE plus errno, fcntl, stdio, stdlib, string, unistd, net/if.h,
// linux/if_tun.h, linux/netlink.h, linux/rtnetlink.h, sys/ioctl.h,
// sys/socket.h, linux/virtio_net.h, netinet/ip.h, netinet/udp.h, and
// kselftest_harness.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;

const AF_NETLINK: c_int = 16;
const AF_UNSPEC: c_int = 0;
const SOCK_DGRAM: c_int = 2;
const NETLINK_ROUTE: c_int = 0;
const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_CREATE: u16 = 0x400;
const RTM_NEWLINK: u16 = 16;
const RTM_DELLINK: u16 = 17;
const IFLA_IFNAME: u16 = 3;
const IFLA_LINK: u16 = 5;
const IFLA_ADDRESS: u16 = 1;
const IFLA_LINKINFO: u16 = 18;
const IFLA_INFO_KIND: u16 = 1;
const IFLA_INFO_DATA: u16 = 2;
const IFF_BROADCAST: c_uint = 0x2;
const IFF_UP: c_uint = 0x1;
const IFF_TAP: c_short = 0x0002;
const IFF_NO_PI: c_short = 0x1000;
const IFF_VNET_HDR: c_short = 0x4000;
const IFF_MULTI_QUEUE: c_short = 0x0100;
const O_RDWR: c_int = 0o2;
const O_NONBLOCK: c_int = 0o4000;
const TUNSETIFF: c_ulong = 0x400454ca;
const ETH_ALEN: usize = 6;
const ETH_HLEN: usize = 14;
const ETH_DATA_LEN: usize = 1500;
const ETH_MAX_MTU: usize = 0xFFFF;
const ETH_P_IP: u16 = 0x0800;
const IPPROTO_UDP: c_int = 17;
const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8 = 1;
const VIRTIO_NET_HDR_F_DATA_VALID: u8 = 2;
const VIRTIO_NET_HDR_GSO_NONE: u8 = 0;
const VIRTIO_NET_HDR_GSO_UDP: u8 = 3;
const EINVAL: c_int = 22;

type c_short = i16;
type c_ulong = u64;

const fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn rta_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn nlmsg_length(len: usize) -> u32 {
    (nlmsg_align(size_of::<nlmsghdr>()) + len) as u32
}

const fn rta_length(len: usize) -> u16 {
    (rta_align(size_of::<rtattr>()) + len) as u16
}

unsafe fn rta_data(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_length(0) as usize) as *mut c_void
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
struct ifinfomsg {
    ifi_family: u8,
    __ifi_pad: u8,
    ifi_type: u16,
    ifi_index: i32,
    ifi_flags: u32,
    ifi_change: u32,
}

#[repr(C)]
struct rtattr {
    rta_len: u16,
    rta_type: u16,
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}

#[repr(C)]
struct iphdr {
    ihl_version: u8,
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
    unsafe fn set_ihl(&mut self, ihl: u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (ihl & 0x0f);
    }

    unsafe fn ihl(&self) -> u8 {
        self.ihl_version & 0x0f
    }

    unsafe fn set_version(&mut self, version: u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((version & 0x0f) << 4);
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
struct virtio_net_hdr {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
    num_buffers: u16,
}

#[repr(C)]
union ifr_ifrn {
    ifrn_name: [c_char; 16],
}

#[repr(C)]
union ifr_ifru {
    ifru_flags: c_short,
}

#[repr(C)]
struct ifreq {
    ifr_ifrn: ifr_ifrn,
    ifr_ifru: ifr_ifru,
}

extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    static mut stderr: *mut c_void;
}

static param_dev_tap_name: &[u8; 10] = b"xmacvtap0\0";
static param_dev_dummy_name: &[u8; 8] = b"xdummy0\0";
static mut param_hwaddr_src: [c_uchar; ETH_ALEN] = [0x00, 0xfe, 0x98, 0x14, 0x22, 0x42];
static mut param_hwaddr_dest: [c_uchar; ETH_ALEN] = [0x00, 0xfe, 0x98, 0x94, 0xd2, 0x43];

const MAX_RTNL_PAYLOAD: usize = 2048;
const PKT_DATA: c_int = 0xCB;
const TEST_PACKET_SZ: usize = size_of::<virtio_net_hdr>() + ETH_HLEN + ETH_MAX_MTU;

#[repr(C)]
struct dev_req {
    nh: nlmsghdr,
    info: ifinfomsg,
    data: [c_uchar; MAX_RTNL_PAYLOAD],
}

unsafe fn rtattr_add(nh: *mut nlmsghdr, type_: c_ushort, len: c_ushort) -> *mut rtattr {
    let rta = (nh as *mut uint8_t).add(rta_align((*nh).nlmsg_len as usize)) as *mut rtattr;
    (*rta).rta_type = type_;
    (*rta).rta_len = rta_length(len as usize);
    (*nh).nlmsg_len =
        (rta_align((*nh).nlmsg_len as usize) + rta_align((*rta).rta_len as usize)) as u32;
    rta
}

type c_ushort = u16;

unsafe fn rtattr_begin(nh: *mut nlmsghdr, type_: c_ushort) -> *mut rtattr {
    rtattr_add(nh, type_, 0)
}

unsafe fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    let end = (nh as *mut uint8_t).add((*nh).nlmsg_len as usize);

    (*attr).rta_len = end.offset_from(attr as *mut uint8_t) as u16;
}

unsafe fn rtattr_add_str(nh: *mut nlmsghdr, type_: c_ushort, s: *const c_char) -> *mut rtattr {
    let strsz = strlen(s) + 1;
    let rta: *mut rtattr;

    rta = rtattr_add(nh, type_, strsz as c_ushort);

    ptr::copy_nonoverlapping(s as *const c_void, rta_data(rta), strsz);
    rta
}

extern "C" {
    fn strlen(s: *const c_char) -> size_t;
}

unsafe fn rtattr_add_any(
    nh: *mut nlmsghdr,
    type_: c_ushort,
    arr: *const c_void,
    len: size_t,
) -> *mut rtattr {
    let rta = rtattr_add(nh, type_, len as c_ushort);

    ptr::copy_nonoverlapping(arr, rta_data(rta), len);
    rta
}

unsafe fn dev_create(
    dev: *const c_char,
    link_type: *const c_char,
    fill_rtattr: Option<unsafe fn(nh: *mut nlmsghdr) -> c_int>,
    fill_info_data: Option<unsafe fn(nh: *mut nlmsghdr) -> c_int>,
) -> c_int {
    let mut req: dev_req;
    let mut link_info: *mut rtattr;
    let mut info_data: *mut rtattr;
    let mut ret: c_int;
    let rtnl: c_int;

    rtnl = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_ROUTE);
    if rtnl < 0 {
        fprintf(stderr, c"%s: socket %s\n".as_ptr(), c"dev_create".as_ptr(), strerror(errno));
        return 1;
    }

    req = core::mem::zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<ifinfomsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_CREATE;
    req.nh.nlmsg_type = RTM_NEWLINK;

    req.info.ifi_family = AF_UNSPEC as u8;
    req.info.ifi_type = 1;
    req.info.ifi_index = 0;
    req.info.ifi_flags = IFF_BROADCAST | IFF_UP;
    req.info.ifi_change = 0xffffffff;

    rtattr_add_str(&mut req.nh, IFLA_IFNAME, dev);

    if let Some(fill_rtattr_fn) = fill_rtattr {
        ret = fill_rtattr_fn(&mut req.nh);
        if ret != 0 {
            return ret;
        }
    }

    link_info = rtattr_begin(&mut req.nh, IFLA_LINKINFO);

    rtattr_add_str(&mut req.nh, IFLA_INFO_KIND, link_type);

    if let Some(fill_info_data_fn) = fill_info_data {
        info_data = rtattr_begin(&mut req.nh, IFLA_INFO_DATA);
        ret = fill_info_data_fn(&mut req.nh);
        if ret != 0 {
            return ret;
        }
        rtattr_end(&mut req.nh, info_data);
    }

    rtattr_end(&mut req.nh, link_info);

    ret = send(rtnl, &req as *const _ as *const c_void, req.nh.nlmsg_len as size_t, 0) as c_int;
    if ret < 0 {
        fprintf(stderr, c"%s: send %s\n".as_ptr(), c"dev_create".as_ptr(), strerror(errno));
    }
    ret = ((ret as c_uint) != req.nh.nlmsg_len) as c_int;

    close(rtnl);
    ret
}

unsafe fn dev_delete(dev: *const c_char) -> c_int {
    let mut req: dev_req;
    let mut ret: c_int;
    let rtnl: c_int;

    rtnl = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_ROUTE);
    if rtnl < 0 {
        fprintf(stderr, c"%s: socket %s\n".as_ptr(), c"dev_delete".as_ptr(), strerror(errno));
        return 1;
    }

    req = core::mem::zeroed();
    req.nh.nlmsg_len = nlmsg_length(size_of::<ifinfomsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST;
    req.nh.nlmsg_type = RTM_DELLINK;

    req.info.ifi_family = AF_UNSPEC as u8;

    rtattr_add_str(&mut req.nh, IFLA_IFNAME, dev);

    ret = send(rtnl, &req as *const _ as *const c_void, req.nh.nlmsg_len as size_t, 0) as c_int;
    if ret < 0 {
        fprintf(stderr, c"%s: send %s\n".as_ptr(), c"dev_delete".as_ptr(), strerror(errno));
    }

    ret = ((ret as c_uint) != req.nh.nlmsg_len) as c_int;

    close(rtnl);
    ret
}

unsafe fn macvtap_fill_rtattr(nh: *mut nlmsghdr) -> c_int {
    let ifindex: c_int;

    ifindex = if_nametoindex(param_dev_dummy_name.as_ptr() as *const c_char) as c_int;
    if ifindex == 0 {
        fprintf(
            stderr,
            c"%s: ifindex  %s\n".as_ptr(),
            c"macvtap_fill_rtattr".as_ptr(),
            strerror(errno),
        );
        return -errno;
    }

    rtattr_add_any(nh, IFLA_LINK, &ifindex as *const _ as *const c_void, size_of::<c_int>());
    rtattr_add_any(
        nh,
        IFLA_ADDRESS,
        param_hwaddr_src.as_ptr() as *const c_void,
        ETH_ALEN,
    );

    0
}

unsafe fn opentap(devname: *const c_char) -> c_int {
    let ifindex: c_int;
    let mut buf: [c_char; 256] = [0; 256];
    let fd: c_int;
    let mut ifr: ifreq;

    ifindex = if_nametoindex(devname) as c_int;
    if ifindex == 0 {
        fprintf(stderr, c"%s: ifindex %s\n".as_ptr(), c"opentap".as_ptr(), strerror(errno));
        return -errno;
    }

    sprintf(buf.as_mut_ptr(), c"/dev/tap%d".as_ptr(), ifindex);
    fd = open(buf.as_ptr(), O_RDWR | O_NONBLOCK);
    if fd < 0 {
        fprintf(stderr, c"%s: open %s\n".as_ptr(), c"opentap".as_ptr(), strerror(errno));
        return -errno;
    }

    ifr = core::mem::zeroed();
    strcpy((*(&mut ifr.ifr_ifrn as *mut ifr_ifrn)).ifrn_name.as_mut_ptr(), devname);
    ifr.ifr_ifru.ifru_flags = IFF_TAP | IFF_NO_PI | IFF_VNET_HDR | IFF_MULTI_QUEUE;
    if ioctl(fd, TUNSETIFF, &ifr as *const _ as *const c_void, size_of::<ifreq>()) < 0 {
        return -errno;
    }
    fd
}

extern "C" {
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
}

pub unsafe fn build_eth(buf: *mut uint8_t, proto: uint16_t) -> size_t {
    let eth = buf as *mut ethhdr;

    (*eth).h_proto = htons(proto);
    ptr::copy_nonoverlapping(param_hwaddr_src.as_ptr(), (*eth).h_source.as_mut_ptr(), ETH_ALEN);
    ptr::copy_nonoverlapping(param_hwaddr_dest.as_ptr(), (*eth).h_dest.as_mut_ptr(), ETH_ALEN);

    ETH_HLEN
}

unsafe fn add_csum(buf: *const uint8_t, mut len: c_int) -> uint32_t {
    let mut sum: uint32_t = 0;
    let mut sbuf = buf as *mut uint16_t;

    while len > 1 {
        sum = sum.wrapping_add(*sbuf as uint32_t);
        sbuf = sbuf.add(1);
        len -= 2;
    }

    if len != 0 {
        sum = sum.wrapping_add(*(sbuf as *mut uint8_t) as uint32_t);
    }

    sum
}

unsafe fn finish_ip_csum(sum: uint32_t) -> uint16_t {
    let lo: uint16_t = (sum & 0xffff) as uint16_t;
    let hi: uint16_t = (sum >> 16) as uint16_t;

    !(lo.wrapping_add(hi))
}

unsafe fn build_ip_csum(buf: *const uint8_t, len: c_int, mut sum: uint32_t) -> uint16_t {
    sum = sum.wrapping_add(add_csum(buf, len));
    finish_ip_csum(sum)
}

unsafe fn build_ipv4_header(buf: *mut uint8_t, payload_len: c_int) -> c_int {
    let iph = buf as *mut iphdr;

    (*iph).set_ihl(5);
    (*iph).set_version(4);
    (*iph).ttl = 8;
    (*iph).tot_len = htons((size_of::<iphdr>() + size_of::<udphdr>() + payload_len as usize) as u16);
    (*iph).id = htons(1337);
    (*iph).protocol = IPPROTO_UDP as u8;
    (*iph).saddr = htonl((172 << 24) | (17 << 16) | 2);
    (*iph).daddr = htonl((172 << 24) | (17 << 16) | 1);
    (*iph).check = build_ip_csum(buf, ((*iph).ihl() << 2) as c_int, 0);

    ((*iph).ihl() << 2) as c_int
}

unsafe fn build_udp_packet(buf: *mut uint8_t, payload_len: c_int, csum_off: bool) -> c_int {
    let ip4alen: c_int = size_of::<uint32_t>() as c_int;
    let udph = buf as *mut udphdr;
    let len: c_int = size_of::<udphdr>() as c_int + payload_len;
    let mut sum: uint32_t = 0;

    (*udph).source = htons(22);
    (*udph).dest = htons(58822);
    (*udph).len = htons(len as u16);

    ptr::write_bytes(buf.add(size_of::<udphdr>()), PKT_DATA as u8, payload_len as usize);

    sum = add_csum(buf.sub((2 * ip4alen) as usize), 2 * ip4alen);
    sum = sum.wrapping_add(htons(IPPROTO_UDP as u16) as u32 + (*udph).len as u32);

    if !csum_off {
        sum = sum.wrapping_add(add_csum(buf, len));
    }

    (*udph).check = finish_ip_csum(sum);

    size_of::<udphdr>() as c_int + payload_len
}

pub unsafe fn build_test_packet_valid_udp_gso(buf: *mut uint8_t, payload_len: size_t) -> size_t {
    let mut cur = buf;
    let vh = buf as *mut virtio_net_hdr;

    (*vh).hdr_len = (ETH_HLEN + size_of::<iphdr>() + size_of::<udphdr>()) as u16;
    (*vh).flags = VIRTIO_NET_HDR_F_NEEDS_CSUM;
    (*vh).csum_start = (ETH_HLEN + size_of::<iphdr>()) as u16;
    (*vh).csum_offset = offset_of!(udphdr, check) as u16;
    (*vh).gso_type = VIRTIO_NET_HDR_GSO_UDP;
    (*vh).gso_size = (ETH_DATA_LEN - size_of::<iphdr>()) as u16;
    cur = cur.add(size_of::<virtio_net_hdr>());

    cur = cur.add(build_eth(cur, ETH_P_IP));
    cur = cur.add(build_ipv4_header(cur, payload_len as c_int) as usize);
    cur = cur.add(build_udp_packet(cur, payload_len as c_int, true) as usize);

    cur.offset_from(buf) as size_t
}

pub unsafe fn build_test_packet_valid_udp_csum(buf: *mut uint8_t, payload_len: size_t) -> size_t {
    let mut cur = buf;
    let vh = buf as *mut virtio_net_hdr;

    (*vh).flags = VIRTIO_NET_HDR_F_DATA_VALID;
    (*vh).gso_type = VIRTIO_NET_HDR_GSO_NONE;
    cur = cur.add(size_of::<virtio_net_hdr>());

    cur = cur.add(build_eth(cur, ETH_P_IP));
    cur = cur.add(build_ipv4_header(cur, payload_len as c_int) as usize);
    cur = cur.add(build_udp_packet(cur, payload_len as c_int, false) as usize);

    cur.offset_from(buf) as size_t
}

pub unsafe fn build_test_packet_crash_tap_invalid_eth_proto(
    buf: *mut uint8_t,
    payload_len: size_t,
) -> size_t {
    let mut cur = buf;
    let vh = buf as *mut virtio_net_hdr;

    (*vh).hdr_len = (ETH_HLEN + size_of::<iphdr>() + size_of::<udphdr>()) as u16;
    (*vh).flags = 0;
    (*vh).gso_type = VIRTIO_NET_HDR_GSO_UDP;
    (*vh).gso_size = (ETH_DATA_LEN - size_of::<iphdr>()) as u16;
    cur = cur.add(size_of::<virtio_net_hdr>());

    cur = cur.add(build_eth(cur, 0));
    cur = cur.add(size_of::<iphdr>() + size_of::<udphdr>());
    cur = cur.add(build_ipv4_header(cur, payload_len as c_int) as usize);
    cur = cur.add(build_udp_packet(cur, payload_len as c_int, true) as usize);
    cur = cur.add(payload_len);

    cur.offset_from(buf) as size_t
}

#[repr(C)]
struct tap {
    fd: c_int,
}

unsafe fn tap_setup(self_: *mut tap) {
    let mut ret: c_int;

    ret = dev_create(
        param_dev_dummy_name.as_ptr() as *const c_char,
        c"dummy".as_ptr(),
        None,
        None,
    );
    EXPECT_EQ(ret, 0);

    ret = dev_create(
        param_dev_tap_name.as_ptr() as *const c_char,
        c"macvtap".as_ptr(),
        Some(macvtap_fill_rtattr),
        None,
    );
    EXPECT_EQ(ret, 0);

    (*self_).fd = opentap(param_dev_tap_name.as_ptr() as *const c_char);
    ASSERT_GE((*self_).fd, 0);
}

unsafe fn tap_teardown(self_: *mut tap) {
    let mut ret: c_int;

    if (*self_).fd != -1 {
        close((*self_).fd);
    }

    ret = dev_delete(param_dev_tap_name.as_ptr() as *const c_char);
    EXPECT_EQ(ret, 0);

    ret = dev_delete(param_dev_dummy_name.as_ptr() as *const c_char);
    EXPECT_EQ(ret, 0);
}

unsafe fn test_packet_valid_udp_gso(self_: *mut tap) {
    let mut pkt: [uint8_t; TEST_PACKET_SZ] = [0; TEST_PACKET_SZ];
    let off: size_t;
    let ret: c_int;

    ptr::write_bytes(pkt.as_mut_ptr(), 0, size_of_val(&pkt));
    off = build_test_packet_valid_udp_gso(pkt.as_mut_ptr(), 1021);
    ret = write((*self_).fd, pkt.as_ptr() as *const c_void, off) as c_int;
    ASSERT_EQ(ret as ssize_t, off as ssize_t);
}

unsafe fn test_packet_valid_udp_csum(self_: *mut tap) {
    let mut pkt: [uint8_t; TEST_PACKET_SZ] = [0; TEST_PACKET_SZ];
    let off: size_t;
    let ret: c_int;

    ptr::write_bytes(pkt.as_mut_ptr(), 0, size_of_val(&pkt));
    off = build_test_packet_valid_udp_csum(pkt.as_mut_ptr(), 1024);
    ret = write((*self_).fd, pkt.as_ptr() as *const c_void, off) as c_int;
    ASSERT_EQ(ret as ssize_t, off as ssize_t);
}

unsafe fn test_packet_crash_tap_invalid_eth_proto(self_: *mut tap) {
    let mut pkt: [uint8_t; TEST_PACKET_SZ] = [0; TEST_PACKET_SZ];
    let off: size_t;
    let ret: c_int;

    ptr::write_bytes(pkt.as_mut_ptr(), 0, size_of_val(&pkt));
    off = build_test_packet_crash_tap_invalid_eth_proto(pkt.as_mut_ptr(), 1024);
    ret = write((*self_).fd, pkt.as_ptr() as *const c_void, off) as c_int;
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, EINVAL);
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

// kselftest_harness.h macro equivalents are external to this isolated file.
extern "Rust" {
    fn EXPECT_EQ<T, U>(left: T, right: U);
    fn ASSERT_GE<T, U>(left: T, right: U);
    fn ASSERT_EQ<T, U>(left: T, right: U);
}

// TEST_HARNESS_MAIN
