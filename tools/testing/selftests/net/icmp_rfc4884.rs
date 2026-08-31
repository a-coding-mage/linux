// SPDX-License-Identifier: GPL-2.0

// Rust translation of testing/selftests/net/icmp_rfc4884.c.
// C include dependencies intentionally remain external.

use core::ffi::{c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type socklen_t = u32;

const src_port: c_ushort = 44444;
const dst_port: c_ushort = 55555;
const min_orig_dgram_len: c_int = 128;
const min_payload_len_v4: c_int = min_orig_dgram_len - 20 - 8;
const min_payload_len_v6: c_int = min_orig_dgram_len - 40 - 8;
const orig_payload_byte: u8 = 0xAA;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_ICMPV6: c_int = 58;
const IP_RECVERR: c_int = 11;
const IPV6_RECVERR: c_int = 25;
const IP_RECVERR_RFC4884: c_int = 26;
const IPV6_RECVERR_RFC4884: c_int = 31;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const ICMP_DEST_UNREACH: u8 = 3;
const ICMP_PORT_UNREACH: u8 = 3;
const ICMPV6_DEST_UNREACH: u8 = 1;
const ICMPV6_PORT_UNREACH: u8 = 4;
const MSG_ERRQUEUE: c_int = 0x2000;
const POLLERR: c_short = 0x0008;
const CLONE_NEWNET: c_int = 0x40000000;
const SIOCGIFFLAGS: c_ulong = 0x8913;
const SIOCSIFFLAGS: c_ulong = 0x8914;
const IFF_UP: c_short = 0x1;
const EINVAL: c_int = 22;
const SO_EE_RFC4884_FLAG_INVALID: u8 = 1;

type c_ulong = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

const in6addr_loopback: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr {
    sa_family: c_ushort,
    sa_data: [c_uchar; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: c_ushort,
    sin_port: c_ushort,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: c_ushort,
    sin6_port: c_ushort,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
union sockaddr_inet_addr {
    v6: sockaddr_in6,
    v4: sockaddr_in,
    sa: sockaddr,
}

#[repr(C)]
struct sockaddr_inet {
    addr: sockaddr_inet_addr,
    len: socklen_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ip_case_info {
    domain: c_int,
    level: c_int,
    opt1: c_int,
    opt2: c_int,
    proto: c_int,
    build_func: unsafe fn(*mut u8, ssize_t, bool, c_int, bool, bool, bool) -> c_int,
    min_payload: c_int,
}

#[repr(C)]
struct ifreq {
    ifr_name: [u8; 16],
    ifr_flags: c_short,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
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

#[repr(C)]
struct icmp_ext_hdr {
    version_reserved: u8,
    reserved1: u8,
    checksum: u16,
}

#[repr(C)]
struct icmp_extobj_hdr {
    length: u16,
    class_num: u8,
    class_type: u8,
}

#[repr(C)]
union icmphdr_un {
    reserved: [u8; 4],
}

#[repr(C)]
struct icmphdr {
    type_: u8,
    code: u8,
    checksum: u16,
    un: icmphdr_un,
}

#[repr(C)]
struct icmp6hdr {
    icmp6_type: u8,
    icmp6_code: u8,
    icmp6_cksum: u16,
    icmp6_datagram_len: u16,
    icmp6_unused: u16,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct sock_ee_rfc4884 {
    len: u16,
    flags: u8,
    reserved: u8,
}

#[repr(C)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
    ee_rfc4884: sock_ee_rfc4884,
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn recvmsg(socket: c_int, message: *mut msghdr, flags: c_int) -> ssize_t;
    fn sendto(
        socket: c_int,
        message: *const c_void,
        length: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> ssize_t;
    fn unshare(flags: c_int) -> c_int;
}

fn htons(x: u16) -> u16 {
    x.to_be()
}

fn htonl(x: u32) -> u32 {
    x.to_be()
}

unsafe fn memset(dst: *mut c_void, val: c_int, len: size_t) {
    ptr::write_bytes(dst, val as u8, len);
}

unsafe fn cmsg_align(len: size_t) -> size_t {
    (len + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1)
}

unsafe fn cmsg_firsthdr(mhdr: *const msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn cmsg_nxthdr(mhdr: *const msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len)) as *mut cmsghdr;
    let max = ((*mhdr).msg_control as *mut u8).add((*mhdr).msg_controllen);
    if (next as *mut u8).add(size_of::<cmsghdr>()) > max {
        ptr::null_mut()
    } else {
        next
    }
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(cmsg_align(size_of::<cmsghdr>()))
}

unsafe fn bringup_loopback() -> c_int {
    let mut ifr: ifreq = zeroed();
    ifr.ifr_name[..2].copy_from_slice(b"lo");
    let fd: c_int;

    fd = socket(AF_INET, SOCK_DGRAM, 0);
    if fd < 0 {
        return -1;
    }

    if ioctl(fd, SIOCGIFFLAGS, &mut ifr) < 0 {
        close(fd);
        return -1;
    }

    ifr.ifr_flags = ifr.ifr_flags | IFF_UP;

    if ioctl(fd, SIOCSIFFLAGS, &ifr) < 0 {
        close(fd);
        return -1;
    }

    close(fd);
    0
}

unsafe fn csum(buf: *const c_void, mut len: size_t) -> u16 {
    let mut data = buf as *const u8;
    let mut sum: u32 = 0;

    while len > 1 {
        sum = sum.wrapping_add((((*data.add(0)) as u32) << 8) | (*data.add(1)) as u32);
        data = data.add(2);
        len -= 2;
    }

    if len == 1 {
        sum = sum.wrapping_add(((*data.add(0)) as u32) << 8);
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF).wrapping_add(sum >> 16);
    }

    (!sum & 0xFFFF) as u16
}

unsafe fn poll_err(fd: c_int) -> c_int {
    let mut pfd: pollfd = zeroed();
    pfd.fd = fd;

    if poll(&mut pfd, 1, 5000) != 1 || pfd.revents != POLLERR {
        return -1;
    }

    0
}

unsafe fn set_addr(addr: *mut sockaddr_inet, domain: c_int, port: c_ushort) {
    memset(addr as *mut c_void, 0, size_of::<sockaddr_inet>());

    match domain {
        AF_INET => {
            (*addr).addr.v4.sin_family = AF_INET as c_ushort;
            (*addr).addr.v4.sin_port = htons(port);
            (*addr).addr.v4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            (*addr).len = size_of::<sockaddr_in>() as socklen_t;
        }
        AF_INET6 => {
            (*addr).addr.v6.sin6_family = AF_INET6 as c_ushort;
            (*addr).addr.v6.sin6_port = htons(port);
            (*addr).addr.v6.sin6_addr = in6addr_loopback;
            (*addr).len = size_of::<sockaddr_in6>() as socklen_t;
        }
        _ => {}
    }
}

unsafe fn bind_and_setsockopt(fd: c_int, info: *const ip_case_info) -> c_int {
    let mut addr: sockaddr_inet = zeroed();
    let opt: c_int = 1;

    set_addr(&mut addr, (*info).domain, src_port);

    if setsockopt(
        fd,
        (*info).level,
        (*info).opt1,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        return -1;
    }

    if setsockopt(
        fd,
        (*info).level,
        (*info).opt2,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) < 0
    {
        return -1;
    }

    bind(fd, &addr.addr.sa, addr.len)
}

unsafe fn build_rfc4884_ext(
    buf: *mut u8,
    buflen: size_t,
    bad_csum: bool,
    bad_len: bool,
    smaller_len: bool,
) -> c_int {
    let objh: *mut icmp_extobj_hdr;
    let exthdr: *mut icmp_ext_hdr;
    let mut obj_len: size_t;
    let ext_len: size_t;
    let sum: u16;

    /* Use an object payload of 4 bytes */
    obj_len = size_of::<icmp_extobj_hdr>() + size_of::<u32>();
    ext_len = size_of::<icmp_ext_hdr>() + obj_len;

    if ext_len > buflen {
        return -EINVAL;
    }

    exthdr = buf as *mut icmp_ext_hdr;
    objh = buf.add(size_of::<icmp_ext_hdr>()) as *mut icmp_extobj_hdr;

    (*exthdr).version_reserved = 2 << 4;
    /* When encoding a bad object length, either encode a length too small
     * to fit the object header or too big to fit in the packet.
     */
    if bad_len {
        obj_len = if smaller_len {
            size_of::<icmp_extobj_hdr>() - 1
        } else {
            obj_len * 2
        };
    }
    (*objh).length = htons(obj_len as u16);

    sum = csum(buf as *const c_void, ext_len);
    (*exthdr).checksum = htons(if bad_csum { sum.wrapping_sub(1) } else { sum });

    ext_len as c_int
}

unsafe fn build_orig_dgram_v4(buf: *mut u8, buflen: ssize_t, payload_len: c_int) -> c_int {
    let udph: *mut udphdr;
    let iph: *mut iphdr;
    let len: size_t;

    len = size_of::<iphdr>() + size_of::<udphdr>() + payload_len as size_t;
    if len > buflen as size_t {
        return -EINVAL;
    }

    iph = buf as *mut iphdr;
    udph = buf.add(size_of::<iphdr>()) as *mut udphdr;

    (*iph).ihl_version = (4 << 4) | 5;
    (*iph).protocol = IPPROTO_UDP as u8;
    (*iph).saddr = htonl(INADDR_LOOPBACK);
    (*iph).daddr = htonl(INADDR_LOOPBACK);
    (*iph).tot_len = htons(len as u16);
    (*iph).check = htons(csum(iph as *const c_void, size_of::<iphdr>()));

    (*udph).source = htons(src_port);
    (*udph).dest = htons(dst_port);
    (*udph).len = htons((size_of::<udphdr>() + payload_len as size_t) as u16);

    memset(
        buf.add(size_of::<iphdr>() + size_of::<udphdr>()) as *mut c_void,
        orig_payload_byte as c_int,
        payload_len as size_t,
    );

    len as c_int
}

unsafe fn build_orig_dgram_v6(buf: *mut u8, buflen: ssize_t, payload_len: c_int) -> c_int {
    let udph: *mut udphdr;
    let iph: *mut ipv6hdr;
    let len: size_t;

    len = size_of::<ipv6hdr>() + size_of::<udphdr>() + payload_len as size_t;
    if len > buflen as size_t {
        return -EINVAL;
    }

    iph = buf as *mut ipv6hdr;
    udph = buf.add(size_of::<ipv6hdr>()) as *mut udphdr;

    (*iph).priority_version = 6 << 4;
    (*iph).payload_len = htons((size_of::<udphdr>() + payload_len as size_t) as u16);
    (*iph).nexthdr = IPPROTO_UDP as u8;
    (*iph).saddr = in6addr_loopback;
    (*iph).daddr = in6addr_loopback;

    (*udph).source = htons(src_port);
    (*udph).dest = htons(dst_port);
    (*udph).len = htons((size_of::<udphdr>() + payload_len as size_t) as u16);

    memset(
        buf.add(size_of::<ipv6hdr>() + size_of::<udphdr>()) as *mut c_void,
        orig_payload_byte as c_int,
        payload_len as size_t,
    );

    len as c_int
}

unsafe fn build_icmpv4_pkt(
    buf: *mut u8,
    buflen: ssize_t,
    with_ext: bool,
    payload_len: c_int,
    bad_csum: bool,
    bad_len: bool,
    smaller_len: bool,
) -> c_int {
    let icmph: *mut icmphdr;
    let mut len: c_int;
    let mut ret: c_int;

    len = size_of::<icmphdr>() as c_int;
    memset(buf as *mut c_void, 0, buflen as size_t);

    icmph = buf as *mut icmphdr;
    (*icmph).type_ = ICMP_DEST_UNREACH;
    (*icmph).code = ICMP_PORT_UNREACH;
    (*icmph).checksum = 0;

    ret = build_orig_dgram_v4(buf.add(len as size_t), buflen - len as ssize_t, payload_len);
    if ret < 0 {
        return ret;
    }

    len += ret;

    (*icmph).un.reserved[1] = ((len - size_of::<icmphdr>() as c_int) / size_of::<u32>() as c_int) as u8;

    if with_ext {
        ret = build_rfc4884_ext(
            buf.add(len as size_t),
            (buflen - len as ssize_t) as size_t,
            bad_csum,
            bad_len,
            smaller_len,
        );
        if ret < 0 {
            return ret;
        }

        len += ret;
    }

    (*icmph).checksum = htons(csum(icmph as *const c_void, len as size_t));
    len
}

unsafe fn build_icmpv6_pkt(
    buf: *mut u8,
    buflen: ssize_t,
    with_ext: bool,
    payload_len: c_int,
    bad_csum: bool,
    bad_len: bool,
    smaller_len: bool,
) -> c_int {
    let icmph: *mut icmp6hdr;
    let mut len: c_int;
    let mut ret: c_int;

    len = size_of::<icmp6hdr>() as c_int;
    memset(buf as *mut c_void, 0, buflen as size_t);

    icmph = buf as *mut icmp6hdr;
    (*icmph).icmp6_type = ICMPV6_DEST_UNREACH;
    (*icmph).icmp6_code = ICMPV6_PORT_UNREACH;
    (*icmph).icmp6_cksum = 0;

    ret = build_orig_dgram_v6(buf.add(len as size_t), buflen - len as ssize_t, payload_len);
    if ret < 0 {
        return ret;
    }

    len += ret;

    (*icmph).icmp6_datagram_len =
        ((len - size_of::<icmp6hdr>() as c_int) / size_of::<u64>() as c_int) as u16;

    if with_ext {
        ret = build_rfc4884_ext(
            buf.add(len as size_t),
            (buflen - len as ssize_t) as size_t,
            bad_csum,
            bad_len,
            smaller_len,
        );
        if ret < 0 {
            return ret;
        }

        len += ret;
    }

    (*icmph).icmp6_cksum = htons(csum(icmph as *const c_void, len as size_t));
    len
}

// FIXTURE(rfc4884) {};

unsafe fn rfc4884_setup(_metadata: *mut __test_metadata) {
    let mut ret: c_int;

    ret = unshare(CLONE_NEWNET);
    assert_eq!(ret, 0, "unshare(CLONE_NEWNET) failed");

    ret = bringup_loopback();
    assert_eq!(ret, 0, "Failed to bring up loopback interface");
}

unsafe fn rfc4884_teardown(_metadata: *mut __test_metadata) {}

const ipv4_info: ip_case_info = ip_case_info {
    domain: AF_INET,
    level: SOL_IP,
    opt1: IP_RECVERR,
    opt2: IP_RECVERR_RFC4884,
    proto: IPPROTO_ICMP,
    build_func: build_icmpv4_pkt,
    min_payload: min_payload_len_v4,
};

const ipv6_info: ip_case_info = ip_case_info {
    domain: AF_INET6,
    level: SOL_IPV6,
    opt1: IPV6_RECVERR,
    opt2: IPV6_RECVERR_RFC4884,
    proto: IPPROTO_ICMPV6,
    build_func: build_icmpv6_pkt,
    min_payload: min_payload_len_v6,
};

#[repr(C)]
#[derive(Copy, Clone)]
struct rfc4884_variant {
    /* IPv4/v6 related information */
    info: ip_case_info,
    /* Whether to append an ICMP extension or not */
    with_ext: bool,
    /* UDP payload length */
    payload_len: c_int,
    /* Whether to generate a bad checksum in the ICMP extension structure */
    bad_csum: bool,
    /* Whether to generate a bad length in the ICMP object header */
    bad_len: bool,
    /* Whether it is too small to fit the object header or too big to fit
     * in the packet
     */
    smaller_len: bool,
}

/* Tests that a valid ICMPv4 error message with extension and the original
 * datagram is smaller than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_ext_small_payload: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: 64, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv4 error message with extension and 128 bytes original
 * datagram, generates an error with the expected offset, and does not raise the
 * SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_ext: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: min_payload_len_v4, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv4 error message with extension and the original
 * datagram is larger than 128 bytes, generates an error with the expected
 * offset, and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_ext_large_payload: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: 256, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv4 error message without extension and the original
 * datagram is smaller than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_no_ext_small_payload: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: false, payload_len: 64, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv4 error message without extension and 128 bytes
 * original datagram, generates an error with zero offset, and does not raise
 * the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_no_ext_min_payload: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: false, payload_len: min_payload_len_v4, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv4 error message without extension and the original
 * datagram is larger than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_no_ext_large_payload: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: false, payload_len: 256, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that an ICMPv4 error message with extension and an invalid checksum,
 * generates an error with the expected offset, and raises the
 * SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_invalid_ext_checksum: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: min_payload_len_v4, bad_csum: true, bad_len: false, smaller_len: false };

/* Tests that an ICMPv4 error message with extension and an object length
 * smaller than the object header, generates an error with the expected offset,
 * and raises the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_invalid_ext_length_small: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: min_payload_len_v4, bad_csum: false, bad_len: true, smaller_len: true };

/* Tests that an ICMPv4 error message with extension and an object length that
 * is too big to fit in the packet, generates an error with the expected offset,
 * and raises the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv4_invalid_ext_length_large: rfc4884_variant = rfc4884_variant { info: ipv4_info, with_ext: true, payload_len: min_payload_len_v4, bad_csum: false, bad_len: true, smaller_len: false };

/* Tests that a valid ICMPv6 error message with extension and the original
 * datagram is smaller than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_ext_small_payload: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: 64, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv6 error message with extension and 128 bytes original
 * datagram, generates an error with the expected offset, and does not raise the
 * SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_ext: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: min_payload_len_v6, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv6 error message with extension and the original
 * datagram is larger than 128 bytes, generates an error with the expected
 * offset, and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_ext_large_payload: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: 256, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv6 error message without extension and the original
 * datagram is smaller than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_no_ext_small_payload: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: false, payload_len: 64, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv6 error message without extension and 128 bytes
 * original datagram, generates an error with zero offset, and does not
 * raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_no_ext_min_payload: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: false, payload_len: min_payload_len_v6, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that a valid ICMPv6 error message without extension and the original
 * datagram is larger than 128 bytes, generates an error with zero offset,
 * and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_no_ext_large_payload: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: false, payload_len: 256, bad_csum: false, bad_len: false, smaller_len: false };

/* Tests that an ICMPv6 error message with extension and an invalid checksum,
 * generates an error with the expected offset, and raises the
 * SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_invalid_ext_checksum: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: min_payload_len_v6, bad_csum: true, bad_len: false, smaller_len: false };

/* Tests that an ICMPv6 error message with extension and an object length
 * smaller than the object header, generates an error with the expected offset,
 * and raises the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_invalid_ext_length_small: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: min_payload_len_v6, bad_csum: false, bad_len: true, smaller_len: true };

/* Tests that an ICMPv6 error message with extension and an object length that
 * is too big to fit in the packet, generates an error with the expected offset,
 * and raises the SO_EE_RFC4884_FLAG_INVALID flag.
 */
const ipv6_invalid_ext_length_large: rfc4884_variant = rfc4884_variant { info: ipv6_info, with_ext: true, payload_len: min_payload_len_v6, bad_csum: false, bad_len: true, smaller_len: false };

unsafe fn check_rfc4884_offset(_metadata: *mut __test_metadata, sock: c_int, v: *const rfc4884_variant) {
    let mut rxbuf = [0i8; 1024];
    let mut ctrl = [0i8; 1024];
    let mut iov = iovec {
        iov_base: rxbuf.as_mut_ptr() as *mut c_void,
        iov_len: size_of_val(&rxbuf),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ctrl.as_mut_ptr() as *mut c_void,
        msg_controllen: size_of_val(&ctrl),
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr;
    let recv: c_int;

    assert_eq!(poll_err(sock), 0);

    recv = recvmsg(sock, &mut msg, MSG_ERRQUEUE) as c_int;
    assert!(recv >= 0, "recvmsg(MSG_ERRQUEUE) failed");

    cmsg = cmsg_firsthdr(&msg);
    while !cmsg.is_null() {
        let is_invalid: bool;
        let expected_invalid: bool;
        let ee: *mut sock_extended_err;
        let expected_off: c_int;
        let off: u16;

        if (*cmsg).cmsg_level != (*v).info.level || (*cmsg).cmsg_type != (*v).info.opt1 {
            eprintln!("Unrelated cmsgs were encountered in recvmsg()");
            cmsg = cmsg_nxthdr(&msg, cmsg);
            continue;
        }

        ee = cmsg_data(cmsg) as *mut sock_extended_err;
        off = (*ee).ee_rfc4884.len;
        is_invalid = ((*ee).ee_rfc4884.flags & SO_EE_RFC4884_FLAG_INVALID) != 0;

        expected_invalid = (*v).bad_csum || (*v).bad_len;
        assert_eq!(
            is_invalid, expected_invalid,
            "Expected invalidity flag to be {}, but got {}",
            expected_invalid, is_invalid
        );

        expected_off = if (*v).with_ext && (*v).payload_len >= (*v).info.min_payload {
            (*v).payload_len
        } else {
            0
        };
        assert_eq!(
            off as c_int, expected_off,
            "Expected RFC4884 offset {}, got {}",
            expected_off, off
        );
        break;
    }
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

unsafe fn test_rfc4884(_metadata: *mut __test_metadata, variant: *const rfc4884_variant) {
    let v = variant;
    let mut addr: sockaddr_inet = zeroed();
    let mut pkt = [0u8; 1024];
    let dgram: c_int;
    let raw: c_int;
    let len: c_int;
    let sent: c_int;
    let err: c_int;

    dgram = socket((*v).info.domain, SOCK_DGRAM, 0);
    assert!(dgram >= 0, "Opening datagram socket failed");

    err = bind_and_setsockopt(dgram, &(*v).info);
    assert_eq!(err, 0, "Bind failed");

    raw = socket((*v).info.domain, SOCK_RAW, (*v).info.proto);
    assert!(raw >= 0, "Opening raw socket failed");

    len = ((*v).info.build_func)(
        pkt.as_mut_ptr(),
        size_of_val(&pkt) as ssize_t,
        (*v).with_ext,
        (*v).payload_len,
        (*v).bad_csum,
        (*v).bad_len,
        (*v).smaller_len,
    );
    assert!(len > 0, "Building packet failed");

    set_addr(&mut addr, (*v).info.domain, 0);
    sent = sendto(raw, pkt.as_ptr() as *const c_void, len as size_t, 0, &addr.addr.sa, addr.len) as c_int;
    assert_eq!(len, sent, "Sending packet failed");

    check_rfc4884_offset(_metadata, dgram, v);

    close(dgram);
    close(raw);
}

// TEST_HARNESS_MAIN
