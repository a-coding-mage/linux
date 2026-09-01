// SPDX-License-Identifier: GPL-2.0

// Translated from C source. System headers and psock_lib.h are represented by
// libc items plus extern declarations for file-external helpers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_t = bool;
type uint16_t = u16;
type uint32_t = u32;
type socklen_t = u32;

const DATA_LEN: c_int = 100;
const DATA_CHAR: c_int = b'a' as c_int;

const AF_INET: c_int = 2;
const PF_INET: c_int = AF_INET;
const PF_PACKET: c_int = 17;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SO_RCVBUF: c_int = 8;
const SO_RCVTIMEO: c_int = 20;
const SOL_PACKET: c_int = 263;
const PACKET_HOST: c_int = 0;
const PACKET_OUTGOING: c_int = 4;
const PACKET_AUXDATA: c_int = 8;
const PACKET_STATISTICS: c_int = 6;
const PACKET_QDISC_BYPASS: c_int = 20;
const PACKET_VNET_HDR: c_int = 15;
const PACKET_IGNORE_OUTGOING: c_int = 23;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_ALL: c_int = 0x0003;
const ETH_P_8021Q: c_int = 0x8100;
const ETH_HLEN: c_int = 14;
const ETH_MAX_MTU: c_int = 0xFFFF;
const IPPROTO_UDP: c_int = 17;
const INADDR_ANY: c_int = 0;
const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8 = 1;
const VIRTIO_NET_HDR_GSO_UDP: u8 = 3;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const INT_MAX: c_int = c_int::MAX;

/* test sending up to max mtu + 1 */
const TEST_SZ: usize = size_of::<virtio_net_hdr>() + ETH_HLEN as usize + ETH_MAX_MTU as usize + 1;

const BURST_CNT: c_int = 1000;

#[repr(C)]
struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: uint16_t,
}

#[repr(C)]
struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: uint16_t,
    id: uint16_t,
    frag_off: uint16_t,
    ttl: u8,
    protocol: u8,
    check: uint16_t,
    saddr: uint32_t,
    daddr: uint32_t,
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
    source: uint16_t,
    dest: uint16_t,
    len: uint16_t,
    check: uint16_t,
}

#[repr(C)]
struct virtio_net_hdr {
    flags: u8,
    gso_type: u8,
    hdr_len: uint16_t,
    gso_size: uint16_t,
    csum_start: uint16_t,
    csum_offset: uint16_t,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in_addr {
    s_addr: uint32_t,
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: uint16_t,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: uint16_t,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
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
struct cmsghdr {
    cmsg_len: usize,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct tpacket_auxdata {
    tp_status: uint32_t,
    tp_len: uint32_t,
    tp_snaplen: uint32_t,
    tp_mac: uint16_t,
    tp_net: uint16_t,
    tp_vlan_tci: uint16_t,
    tp_vlan_tpid: uint16_t,
}

#[repr(C)]
struct tpacket_stats {
    tp_packets: c_uint,
    tp_drops: c_uint,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn htons(hostshort: uint16_t) -> uint16_t;
    fn htonl(hostlong: uint32_t) -> uint32_t;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> isize;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> isize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn system(command: *const c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;

    static mut stderr: *mut c_void;

    fn pair_udp_setfilter(fd: c_int);
}

static mut cfg_use_bind: bool_t = false;
static mut cfg_use_csum_off: bool_t = false;
static mut cfg_use_csum_off_bad: bool_t = false;
static mut cfg_use_dgram: bool_t = false;
static mut cfg_use_gso: bool_t = false;
static mut cfg_use_qdisc_bypass: bool_t = false;
static mut cfg_use_vlan: bool_t = false;
static mut cfg_use_vnet: bool_t = false;
static mut cfg_drop: bool_t = false;
static mut cfg_aux_data: bool_t = false;
static mut cfg_ignore_outgoing: bool_t = false;

static mut cfg_ifname: *mut c_char = c"lo".as_ptr() as *mut c_char;
static mut cfg_mtu: c_int = 1500;
static mut cfg_payload_len: c_int = DATA_LEN;
static mut cfg_truncate_len: c_int = INT_MAX;
static mut cfg_port: uint16_t = 8000;

static mut tbuf: [c_char; TEST_SZ] = [0; TEST_SZ];
static mut rbuf: [c_char; TEST_SZ] = [0; TEST_SZ];

unsafe fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_void {
    (cmsg as *mut u8).add(CMSG_ALIGN(size_of::<cmsghdr>())) as *mut c_void
}

unsafe fn CMSG_FIRSTHDR(mhdr: *mut msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn add_csum_hword(start: *const uint16_t, num_u16: c_int) -> c_ulong {
    let mut sum: c_ulong = 0;
    let mut i: c_int = 0;

    while i < num_u16 {
        sum = sum.wrapping_add(*start.add(i as usize) as c_ulong);
        i += 1;
    }

    sum
}

unsafe fn build_ip_csum(start: *const uint16_t, num_u16: c_int, mut sum: c_ulong) -> uint16_t {
    sum = sum.wrapping_add(add_csum_hword(start, num_u16));

    while (sum >> 16) != 0 {
        sum = (sum & 0xffff).wrapping_add(sum >> 16);
    }

    !(sum as uint16_t)
}

unsafe fn build_vnet_header(header: *mut c_void) -> c_int {
    let vh = header as *mut virtio_net_hdr;

    (*vh).hdr_len = (ETH_HLEN as usize + size_of::<iphdr>() + size_of::<udphdr>()) as uint16_t;

    if cfg_use_csum_off {
        (*vh).flags |= VIRTIO_NET_HDR_F_NEEDS_CSUM;
        (*vh).csum_start = (ETH_HLEN as usize + size_of::<iphdr>()) as uint16_t;
        (*vh).csum_offset = offset_of!(udphdr, check) as uint16_t;

        /* position check field exactly one byte beyond end of packet */
        if cfg_use_csum_off_bad {
            (*vh).csum_start = (*vh).csum_start.wrapping_add(
                (size_of::<udphdr>() as c_int + cfg_payload_len - (*vh).csum_offset as c_int - 1)
                    as uint16_t,
            );
        }
    }

    if cfg_use_gso {
        (*vh).gso_type = VIRTIO_NET_HDR_GSO_UDP;
        (*vh).gso_size = (cfg_mtu as usize - size_of::<iphdr>()) as uint16_t;
    }

    size_of::<virtio_net_hdr>() as c_int
}

unsafe fn build_eth_header(header: *mut c_void) -> c_int {
    let eth = header as *mut ethhdr;

    if cfg_use_vlan {
        let tag = (header as *mut u8).add(ETH_HLEN as usize) as *mut uint16_t;

        (*eth).h_proto = htons(ETH_P_8021Q as uint16_t);
        *tag.add(1) = htons(ETH_P_IP as uint16_t);
        return ETH_HLEN + 4;
    }

    (*eth).h_proto = htons(ETH_P_IP as uint16_t);
    ETH_HLEN
}

unsafe fn build_ipv4_header(header: *mut c_void, payload_len: c_int) -> c_int {
    let iph = header as *mut iphdr;

    (*iph).set_ihl(5);
    (*iph).set_version(4);
    (*iph).ttl = 8;
    (*iph).tot_len = htons((size_of::<iphdr>() + size_of::<udphdr>() + payload_len as usize) as uint16_t);
    (*iph).id = htons(1337);
    (*iph).protocol = IPPROTO_UDP as u8;
    (*iph).saddr = htonl(((172 << 24) | (17 << 16) | 2) as uint32_t);
    (*iph).daddr = htonl(((172 << 24) | (17 << 16) | 1) as uint32_t);
    (*iph).check = build_ip_csum(iph as *const uint16_t, ((*iph).ihl() as c_int) << 1, 0);

    ((*iph).ihl() as c_int) << 2
}

unsafe fn build_udp_header(header: *mut c_void, payload_len: c_int) -> c_int {
    let alen: c_int = size_of::<uint32_t>() as c_int;
    let udph = header as *mut udphdr;
    let len: c_int = size_of::<udphdr>() as c_int + payload_len;

    (*udph).source = htons(9);
    (*udph).dest = htons(cfg_port);
    (*udph).len = htons(len as uint16_t);

    if cfg_use_csum_off {
        (*udph).check = build_ip_csum(
            (header as *mut u8).offset(-(2 * alen) as isize) as *const uint16_t,
            alen,
            htons(IPPROTO_UDP as uint16_t) as c_ulong + (*udph).len as c_ulong,
        );
    } else {
        (*udph).check = 0;
    }

    size_of::<udphdr>() as c_int
}

unsafe fn build_packet(payload_len: c_int) -> c_int {
    let mut off: c_int = 0;

    off += build_vnet_header(tbuf.as_mut_ptr() as *mut c_void);
    off += build_eth_header(tbuf.as_mut_ptr().add(off as usize) as *mut c_void);
    off += build_ipv4_header(tbuf.as_mut_ptr().add(off as usize) as *mut c_void, payload_len);
    off += build_udp_header(tbuf.as_mut_ptr().add(off as usize) as *mut c_void, payload_len);

    if off + payload_len > size_of_val(&tbuf) as c_int {
        error(1, 0, c"payload length exceeds max".as_ptr());
    }

    memset(
        tbuf.as_mut_ptr().add(off as usize) as *mut c_void,
        DATA_CHAR,
        payload_len as usize,
    );

    off + payload_len
}

unsafe fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

unsafe fn do_bind_proto(fd: c_int, proto: uint16_t) {
    let mut laddr: sockaddr_ll = core::mem::zeroed();

    laddr.sll_family = AF_PACKET as u16;
    laddr.sll_protocol = htons(proto);
    laddr.sll_ifindex = if_nametoindex(cfg_ifname) as c_int;
    if laddr.sll_ifindex == 0 {
        error(1, errno, c"if_nametoindex".as_ptr());
    }

    if bind(fd, &laddr as *const _ as *const sockaddr, size_of::<sockaddr_ll>() as socklen_t) != 0 {
        error(1, errno, c"bind".as_ptr());
    }
}

unsafe fn do_bind(fd: c_int) {
    do_bind_proto(fd, ETH_P_IP as uint16_t);
}

unsafe fn do_send(fd: c_int, mut buf: *mut c_char, mut len: c_int) {
    let ret: isize;

    if !cfg_use_vnet {
        buf = buf.add(size_of::<virtio_net_hdr>());
        len -= size_of::<virtio_net_hdr>() as c_int;
    }
    if cfg_use_dgram {
        buf = buf.add(ETH_HLEN as usize);
        len -= ETH_HLEN;
    }

    if cfg_use_bind {
        ret = write(fd, buf as *const c_void, len as usize);
    } else {
        let mut laddr: sockaddr_ll = core::mem::zeroed();

        laddr.sll_protocol = htons(ETH_P_IP as uint16_t);
        laddr.sll_ifindex = if_nametoindex(cfg_ifname) as c_int;
        if laddr.sll_ifindex == 0 {
            error(1, errno, c"if_nametoindex".as_ptr());
        }

        ret = sendto(
            fd,
            buf as *const c_void,
            len as usize,
            0,
            &laddr as *const _ as *const sockaddr,
            size_of::<sockaddr_ll>() as socklen_t,
        );
    }

    if ret == -1 {
        error(1, errno, c"write".as_ptr());
    }
    if ret != len as isize {
        error(1, 0, c"write: %u %u".as_ptr(), ret as c_uint, len as c_uint);
    }

    if !cfg_drop {
        fprintf(stderr, c"tx: %u\n".as_ptr(), ret as c_uint);
    }
}

unsafe fn do_tx() -> c_int {
    let one: c_int = 1;
    let mut i: c_int;
    let fd: c_int;
    let mut len: c_int;

    fd = socket(PF_PACKET, if cfg_use_dgram { SOCK_DGRAM } else { SOCK_RAW }, 0);
    if fd == -1 {
        error(1, errno, c"socket t".as_ptr());
    }

    if cfg_use_bind {
        do_bind(fd);
    }

    if cfg_use_qdisc_bypass
        && setsockopt(
            fd,
            SOL_PACKET,
            PACKET_QDISC_BYPASS,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
    {
        error(1, errno, c"setsockopt qdisc bypass".as_ptr());
    }

    if cfg_use_vnet
        && setsockopt(
            fd,
            SOL_PACKET,
            PACKET_VNET_HDR,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
    {
        error(1, errno, c"setsockopt vnet".as_ptr());
    }

    len = build_packet(cfg_payload_len);

    if cfg_truncate_len < len {
        len = cfg_truncate_len;
    }

    do_send(fd, tbuf.as_mut_ptr(), len);

    if cfg_drop {
        i = 0;
        while i < BURST_CNT {
            do_send(fd, tbuf.as_mut_ptr(), len);
            i += 1;
        }
    }

    if close(fd) != 0 {
        error(1, errno, c"close t".as_ptr());
    }

    len
}

unsafe fn setup_rx() -> c_int {
    let tv = timeval {
        tv_sec: 0,
        tv_usec: 100 * 1000,
    };
    let mut raddr: sockaddr_in = core::mem::zeroed();
    let fd: c_int;

    fd = socket(PF_INET, SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, errno, c"socket r".as_ptr());
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const _ as *const c_void,
        size_of::<timeval>() as socklen_t,
    ) != 0
    {
        error(1, errno, c"setsockopt rcv timeout".as_ptr());
    }

    raddr.sin_family = AF_INET as u16;
    raddr.sin_port = htons(cfg_port);
    raddr.sin_addr.s_addr = htonl(INADDR_ANY as uint32_t);

    if bind(fd, &raddr as *const _ as *const sockaddr, size_of::<sockaddr_in>() as socklen_t) != 0 {
        error(1, errno, c"bind r".as_ptr());
    }

    fd
}

unsafe fn check_aux_data(cmsg: *mut cmsghdr, expected_len: c_int) {
    let adata: *mut tpacket_auxdata;

    if cmsg.is_null() {
        error(1, 0, c"auxdata null".as_ptr());
    }

    if (*cmsg).cmsg_level != SOL_PACKET {
        error(1, 0, c"cmsg_level != SOL_PACKET".as_ptr());
    }

    if (*cmsg).cmsg_type != PACKET_AUXDATA {
        error(1, 0, c"cmsg_type != PACKET_AUXDATA".as_ptr());
    }

    adata = CMSG_DATA(cmsg) as *mut tpacket_auxdata;

    if (*adata).tp_net != ETH_HLEN as uint16_t {
        error(1, 0, c"cmsg tp_net != ETH_HLEN".as_ptr());
    }

    if (*adata).tp_len != expected_len as uint32_t {
        error(1, 0, c"cmsg tp_len != %u".as_ptr(), expected_len as c_uint);
    }

    if (*adata).tp_snaplen != expected_len as uint32_t {
        error(1, 0, c"cmsg tp_snaplen != %u".as_ptr(), expected_len as c_uint);
    }
}

/* expected_pkttype < 0 skips the sll_pkttype check. */
unsafe fn do_rx(
    fd: c_int,
    expected_len: c_int,
    expected: *mut c_char,
    is_psock: bool_t,
    expected_pkttype: c_int,
) {
    let mut cmsg_buf: [u64; 128] = [0; 128];
    let aux = is_psock && cfg_aux_data;
    let mut saddr: sockaddr_ll = core::mem::zeroed();
    let mut iov = iovec {
        iov_base: rbuf.as_mut_ptr() as *mut c_void,
        iov_len: size_of_val(&rbuf),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let ret: isize;

    if aux {
        msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = size_of_val(&cmsg_buf);
    }
    if is_psock {
        msg.msg_name = &mut saddr as *mut _ as *mut c_void;
        msg.msg_namelen = size_of::<sockaddr_ll>() as socklen_t;
    }

    ret = recvmsg(fd, &mut msg, 0);
    if ret == -1 {
        error(1, errno, c"recv".as_ptr());
    }
    if ret != expected_len as isize {
        error(1, 0, c"recv: %u != %u".as_ptr(), ret as c_uint, expected_len as c_uint);
    }

    if memcmp(rbuf.as_ptr() as *const c_void, expected as *const c_void, ret as usize) != 0 {
        error(1, 0, c"recv: data mismatch".as_ptr());
    }

    if aux {
        check_aux_data(CMSG_FIRSTHDR(&mut msg), expected_len);
    }

    if expected_pkttype >= 0 && saddr.sll_pkttype as c_int != expected_pkttype {
        error(
            1,
            0,
            c"recv: sll_pkttype %d != %d".as_ptr(),
            saddr.sll_pkttype as c_int,
            expected_pkttype,
        );
    }

    fprintf(stderr, c"rx: %u\n".as_ptr(), ret as c_uint);
}

unsafe fn setup_sniffer() -> c_int {
    let tv = timeval {
        tv_sec: 0,
        tv_usec: 100 * 1000,
    };
    let one: c_int = 1;
    let fd: c_int;

    fd = socket(PF_PACKET, SOCK_RAW, 0);
    if fd == -1 {
        error(1, errno, c"socket p".as_ptr());
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const _ as *const c_void,
        size_of::<timeval>() as socklen_t,
    ) != 0
    {
        error(1, errno, c"setsockopt rcv timeout".as_ptr());
    }

    if cfg_drop {
        if setsockopt(
            fd,
            SOL_SOCKET,
            SO_RCVBUF,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
        {
            error(1, errno, c"setsockopt SO_RCVBUF".as_ptr());
        }
    }

    if cfg_aux_data {
        if setsockopt(
            fd,
            SOL_PACKET,
            PACKET_AUXDATA,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
        {
            error(1, errno, c"setsockopt PACKET_AUXDATA".as_ptr());
        }
    }

    pair_udp_setfilter(fd);

    /* binding to ETH_P_ALL adds the sniffer to ptype_all, which will see
     * the dev_queue_xmit_nit copy. ignore_outgoing should suppress this.
     */
    if cfg_ignore_outgoing {
        do_bind_proto(fd, ETH_P_ALL as uint16_t);
    } else {
        do_bind(fd);
    }

    fd
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, c"abcCdDgil:qt:vV".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'a' => {
                cfg_aux_data = true;
            }
            'b' => {
                cfg_use_bind = true;
            }
            'c' => {
                cfg_use_csum_off = true;
            }
            'C' => {
                cfg_use_csum_off_bad = true;
            }
            'd' => {
                cfg_use_dgram = true;
            }
            'D' => {
                cfg_drop = true;
            }
            'g' => {
                cfg_use_gso = true;
            }
            'i' => {
                cfg_ignore_outgoing = true;
            }
            'l' => {
                cfg_payload_len = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'q' => {
                cfg_use_qdisc_bypass = true;
            }
            't' => {
                cfg_truncate_len = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'v' => {
                cfg_use_vnet = true;
            }
            'V' => {
                cfg_use_vlan = true;
            }
            _ => {
                error(1, 0, c"%s: parse error".as_ptr(), *argv);
            }
        }
    }

    if cfg_use_vlan && cfg_use_dgram {
        error(1, 0, c"option vlan (-V) conflicts with dgram (-d)".as_ptr());
    }

    if cfg_use_csum_off && !cfg_use_vnet {
        error(1, 0, c"option csum offload (-c) requires vnet (-v)".as_ptr());
    }

    if cfg_use_csum_off_bad && !cfg_use_csum_off {
        error(1, 0, c"option csum bad (-C) requires csum offload (-c)".as_ptr());
    }

    if cfg_use_gso && !cfg_use_csum_off {
        error(1, 0, c"option gso (-g) requires csum offload (-c)".as_ptr());
    }

    if cfg_aux_data && cfg_drop {
        error(1, 0, c"option aux data (-a) conflicts with drop (-D)".as_ptr());
    }

    if cfg_ignore_outgoing && (cfg_drop || cfg_aux_data) {
        error(
            1,
            0,
            c"option ignore outgoing (-i) conflicts with -D and -a".as_ptr(),
        );
    }
}

unsafe fn check_packet_stats(fd: c_int, expected_packets: c_uint) {
    let mut st: tpacket_stats = core::mem::zeroed();
    let mut len: socklen_t = size_of::<tpacket_stats>() as socklen_t;

    if getsockopt(
        fd,
        SOL_PACKET,
        PACKET_STATISTICS,
        &mut st as *mut _ as *mut c_void,
        &mut len,
    ) != 0
    {
        error(1, errno, c"getsockopt packet statistics".as_ptr());
    }

    if cfg_drop {
        /* PACKET_STATISTICS reports all packets seen (including
         * drops) in tp_packets
         */
        if st.tp_packets < st.tp_drops {
            error(
                1,
                0,
                c"stats: tp_packets %u < tp_drops %u".as_ptr(),
                st.tp_packets,
                st.tp_drops,
            );
        }

        if st.tp_drops == 0 {
            error(1, 0, c"stats: expected drops but tp_drops == 0".as_ptr());
        }
    } else {
        if st.tp_packets != expected_packets {
            error(
                1,
                0,
                c"stats: tp_packets %u != %u".as_ptr(),
                st.tp_packets,
                expected_packets,
            );
        }

        if st.tp_drops != 0 {
            error(1, 0, c"stats: tp_drops %u != 0".as_ptr(), st.tp_drops);
        }
    }

    /* verify clear on read */
    memset(
        &mut st as *mut _ as *mut c_void,
        0xff,
        size_of::<tpacket_stats>(),
    );
    len = size_of::<tpacket_stats>() as socklen_t;

    if getsockopt(
        fd,
        SOL_PACKET,
        PACKET_STATISTICS,
        &mut st as *mut _ as *mut c_void,
        &mut len,
    ) != 0
    {
        error(1, errno, c"getsockopt packet statistics".as_ptr());
    }

    if st.tp_packets != 0 {
        error(
            1,
            0,
            c"stats: tp_packets %u != 0 after clear".as_ptr(),
            st.tp_packets,
        );
    }

    if st.tp_drops != 0 {
        error(
            1,
            0,
            c"stats: tp_drops %u != 0 after clear".as_ptr(),
            st.tp_drops,
        );
    }
}

unsafe fn set_ignore_outgoing(fd: c_int, val: c_int) {
    let mut len: socklen_t = size_of::<c_int>() as socklen_t;
    let mut got: c_int = -1;

    if setsockopt(
        fd,
        SOL_PACKET,
        PACKET_IGNORE_OUTGOING,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(1, errno, c"setsockopt PACKET_IGNORE_OUTGOING %d".as_ptr(), val);
    }

    if getsockopt(
        fd,
        SOL_PACKET,
        PACKET_IGNORE_OUTGOING,
        &mut got as *mut _ as *mut c_void,
        &mut len,
    ) != 0
    {
        error(1, errno, c"getsockopt PACKET_IGNORE_OUTGOING".as_ptr());
    }
    if got != val {
        error(1, 0, c"getsockopt: expected %d got %d".as_ptr(), val, got);
    }
}

unsafe fn check_ignore_outgoing_range(fd: c_int) {
    let mut val: c_int;

    /* Values outside [0, 1] must be rejected with -EINVAL. */
    val = 2;
    if setsockopt(
        fd,
        SOL_PACKET,
        PACKET_IGNORE_OUTGOING,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != -1
        || errno != EINVAL
    {
        error(
            1,
            errno,
            c"setsockopt PACKET_IGNORE_OUTGOING val=2: expected EINVAL".as_ptr(),
        );
    }

    val = -1;
    if setsockopt(
        fd,
        SOL_PACKET,
        PACKET_IGNORE_OUTGOING,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != -1
        || errno != EINVAL
    {
        error(
            1,
            errno,
            c"setsockopt PACKET_IGNORE_OUTGOING val=-1: expected EINVAL".as_ptr(),
        );
    }
}

unsafe fn test_ignore_outgoing(fds: c_int) {
    let expected = tbuf.as_mut_ptr().add(size_of::<virtio_net_hdr>());
    let mut expected_len: c_int;

    /* ptype_all sniffer on loopback should produce two copies per packet
     * (RX and TX).
     */
    expected_len = do_tx();
    expected_len -= size_of::<virtio_net_hdr>() as c_int;
    do_rx(fds, expected_len, expected, true, PACKET_OUTGOING);
    do_rx(fds, expected_len, expected, true, PACKET_HOST);
    check_packet_stats(fds, 2);

    /* 0 and 1 accepted; anything else rejected. */
    set_ignore_outgoing(fds, 0);
    set_ignore_outgoing(fds, 1);
    check_ignore_outgoing_range(fds);

    /* With PACKET_IGNORE_OUTGOING set, only the rx copy survives. */
    do_tx();
    do_rx(fds, expected_len, expected, true, PACKET_HOST);
    if recv(fds, rbuf.as_mut_ptr() as *mut c_void, size_of_val(&rbuf), 0) != -1 || errno != EAGAIN {
        error(1, errno, c"expected EAGAIN, got extra packet".as_ptr());
    }
    check_packet_stats(fds, 1);
}

unsafe fn run_test() {
    let fdr: c_int;
    let fds: c_int;
    let total_len: c_int;

    fdr = setup_rx();
    fds = setup_sniffer();

    if cfg_ignore_outgoing {
        test_ignore_outgoing(fds);
    } else {
        total_len = do_tx();

        if cfg_drop {
            check_packet_stats(fds, 0);
        } else {
            /* BPF filter accepts only this length, vlan changes MAC */
            if cfg_payload_len == DATA_LEN && !cfg_use_vlan {
                do_rx(
                    fds,
                    total_len - size_of::<virtio_net_hdr>() as c_int,
                    tbuf.as_mut_ptr().add(size_of::<virtio_net_hdr>()),
                    true,
                    -1,
                );
                check_packet_stats(fds, 1);
            }

            do_rx(
                fdr,
                cfg_payload_len,
                tbuf.as_mut_ptr().add((total_len - cfg_payload_len) as usize),
                false,
                -1,
            );
        }
    }

    if close(fds) != 0 {
        error(1, errno, c"close s".as_ptr());
    }
    if close(fdr) != 0 {
        error(1, errno, c"close r".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_opts(argc, argv);

    if system(c"ip link set dev lo mtu 1500".as_ptr()) != 0 {
        error(1, errno, c"ip link set mtu".as_ptr());
    }
    if system(c"ip addr add dev lo 172.17.0.1/24".as_ptr()) != 0 {
        error(1, errno, c"ip addr add".as_ptr());
    }
    if system(c"sysctl -w net.ipv4.conf.lo.accept_local=1".as_ptr()) != 0 {
        error(1, errno, c"sysctl lo.accept_local".as_ptr());
    }

    run_test();

    fprintf(stderr, c"OK\n\n".as_ptr());
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
