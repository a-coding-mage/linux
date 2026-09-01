// SPDX-License-Identifier: GPL-2.0

/* Test hardware checksum offload: Rx + Tx, IPv4 + IPv6, TCP + UDP.
 *
 * The test runs on two machines to exercise the NIC. For this reason it
 * is not integrated in kselftests.
 *
 *     CMD=$((./csum -[46] -[tu] -S $SADDR -D $DADDR -[RT] -r 1 $EXTRA_ARGS))
 *
 * Rx:
 *
 * The sender sends packets with a known checksum field using PF_INET(6)
 * SOCK_RAW sockets.
 *
 * good packet: $CMD [-t]
 * bad packet:  $CMD [-t] -E
 *
 * The receiver reads UDP packets with a UDP socket. This is not an
 * option for TCP packets ('-t'). Optionally insert an iptables filter
 * to avoid these entering the real protocol stack.
 *
 * The receiver also reads all packets with a PF_PACKET socket, to
 * observe whether both good and bad packets arrive on the host. And to
 * read the optional TP_STATUS_CSUM_VALID bit. This requires setting
 * option PACKET_AUXDATA, and works only for CHECKSUM_UNNECESSARY.
 *
 * Tx:
 *
 * The sender needs to build CHECKSUM_PARTIAL packets to exercise tx
 * checksum offload.
 *
 * The sender can sends packets with a UDP socket.
 *
 * Optionally crafts a packet that sums up to zero to verify that the
 * device writes negative zero 0xFFFF in this case to distinguish from
 * 0x0000 (checksum disabled), as required by RFC 768. Hit this case
 * by choosing a specific source port.
 *
 * good packet: $CMD -U
 * zero csum:   $CMD -U -Z
 *
 * The sender can also build packets with PF_PACKET with PACKET_VNET_HDR,
 * to cover more protocols. PF_PACKET requires passing src and dst mac
 * addresses.
 *
 * good packet: $CMD -s $smac -d $dmac -p [-t]
 *
 * Argument '-z' sends UDP packets with a 0x000 checksum disabled field,
 * to verify that the NIC passes these packets unmodified.
 *
 * Argument '-e' adds a transport mode encapsulation header between
 * network and transport header. This will fail for devices that parse
 *  headers. Should work on devices that implement protocol agnostic tx
 * checksum offload (NETIF_F_HW_CSUM).
 *
 * Argument '-r $SEED' optionally randomizes header, payload and length
 * to increase coverage between packets sent. SEED 1 further chooses a
 * different seed for each run (and logs this for reproducibility). It
 * is advised to enable this for extra coverage in continuous testing.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

static mut CFG_BAD_CSUM: bool = false;
static mut CFG_FAMILY: c_int = libc::PF_INET6;
static mut CFG_NUM_PKT: c_int = 4;
static mut CFG_DO_RX: bool = true;
static mut CFG_DO_TX: bool = true;
static mut CFG_ENCAP: bool = false;
static mut CFG_IFNAME: *mut c_char = b"eth0\0".as_ptr() as *mut c_char;
static mut CFG_MAC_DST: *mut c_char = ptr::null_mut();
static mut CFG_MAC_SRC: *mut c_char = ptr::null_mut();
static mut CFG_PROTO: c_int = libc::IPPROTO_UDP;
static mut CFG_PAYLOAD_CHAR: c_int = b'a' as c_int;
static mut CFG_PAYLOAD_LEN: c_int = 100;
static mut CFG_PORT_DST: u16 = 34000;
static mut CFG_PORT_SRC: u16 = 33000;
static mut CFG_PORT_SRC_ENCAP: u16 = 33001;
static mut CFG_RANDOM_SEED: c_uint = 0;
static mut CFG_RCVBUF: c_int = 1 << 22; /* be able to queue large cfg_num_pkt */
static mut CFG_SEND_PFPACKET: bool = false;
static mut CFG_SEND_UDP: bool = false;
static mut CFG_TIMEOUT_MS: c_int = 2000;
static mut CFG_ZERO_DISABLE: bool = false; /* skip checksum: set to zero (udp only) */
static mut CFG_ZERO_SUM: bool = false;     /* create packet that adds up to zero */

static mut CFG_DADDR4: libc::sockaddr_in = libc::sockaddr_in {
    sin_family: libc::AF_INET as libc::sa_family_t,
    sin_port: 0,
    sin_addr: libc::in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
static mut CFG_SADDR4: libc::sockaddr_in = libc::sockaddr_in {
    sin_family: libc::AF_INET as libc::sa_family_t,
    sin_port: 0,
    sin_addr: libc::in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
static mut CFG_DADDR6: libc::sockaddr_in6 = libc::sockaddr_in6 {
    sin6_family: libc::AF_INET6 as libc::sa_family_t,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: libc::in6_addr { s6_addr: [0; 16] },
    sin6_scope_id: 0,
};
static mut CFG_SADDR6: libc::sockaddr_in6 = libc::sockaddr_in6 {
    sin6_family: libc::AF_INET6 as libc::sa_family_t,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: libc::in6_addr { s6_addr: [0; 16] },
    sin6_scope_id: 0,
};

const ENC_HEADER_LEN: usize = size_of::<libc::udphdr>() + size_of::<udp_encap_hdr>();
const MAX_HEADER_LEN: usize = size_of::<libc::ip6_hdr>() + ENC_HEADER_LEN + size_of::<libc::tcphdr>();
const MAX_PAYLOAD_LEN: usize = 1024;
const ETH_ALEN: usize = 6;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_IPV6: c_int = 0x86DD;
const SOL_PACKET: c_int = 263;
const PACKET_AUXDATA: c_int = 8;
const PACKET_VNET_HDR: c_int = 15;
const PACKET_HOST: c_int = 0;
const TP_STATUS_CSUMNOTREADY: u32 = 8;
const TP_STATUS_CSUM_VALID: u32 = 128;
const SKF_AD_OFF: u32 = 0xfffff000;
const SKF_AD_PKTTYPE: u32 = 4;
const BPF_LD: u16 = 0x00;
const BPF_B: u16 = 0x10;
const BPF_H: u16 = 0x08;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;
const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8 = 1;

/* Trivial demo encap. Stand-in for transport layer protocols like ESP or PSP */
#[repr(C)]
struct udp_encap_hdr {
    nexthdr: u8,
    padding: [u8; 3],
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
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
struct tpacket_auxdata {
    tp_status: u32,
    tp_len: u32,
    tp_snaplen: u32,
    tp_mac: u16,
    tp_net: u16,
    tp_vlan_tci: u16,
    tp_vlan_tpid: u16,
}

#[repr(C)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
struct pkt;

/* Ipaddrs, for pseudo csum. Global var is ugly, pass through funcs was worse */
static mut IPH_ADDR_P: *mut c_void = ptr::null_mut();

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn gettimeofday_ms() -> c_ulong {
    let mut tv: libc::timeval = zeroed();

    libc::gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as c_ulong * 1000) + (tv.tv_usec as c_ulong / 1000)
}

unsafe fn checksum_nofold(data: *mut c_char, len: usize, mut sum: u32) -> u32 {
    let words = data as *mut u16;
    let mut i: c_int = 0;

    while i < (len / 2) as c_int {
        sum = sum.wrapping_add(*words.add(i as usize) as u32);
        i += 1;
    }

    if len & 1 != 0 {
        sum = sum.wrapping_add(*(data as *mut u8).add(len - 1) as u32);
    }

    sum
}

unsafe fn checksum_fold(data: *mut c_void, len: usize, mut sum: u32) -> u16 {
    sum = checksum_nofold(data as *mut c_char, len, sum);

    while sum > 0xFFFF {
        sum = (sum & 0xFFFF).wrapping_add(sum >> 16);
    }

    !(sum as u16)
}

unsafe fn checksum(th: *mut c_void, proto: u16, len: usize) -> u16 {
    let alen: c_int = if CFG_FAMILY == libc::PF_INET6 { 32 } else { 8 };

    let mut sum = checksum_nofold(IPH_ADDR_P as *mut c_char, alen as usize, 0);
    sum = sum.wrapping_add(libc::htons(proto) as u32);
    sum = sum.wrapping_add(libc::htons(len as u16) as u32);

    /* With CHECKSUM_PARTIAL kernel expects non-inverted pseudo csum */
    if CFG_DO_TX && CFG_SEND_PFPACKET {
        !checksum_fold(ptr::null_mut(), 0, sum)
    } else {
        checksum_fold(th, len, sum)
    }
}

unsafe fn build_packet_ipv4(_iph: *mut c_void, proto: u8, len: c_uint) -> *mut c_void {
    let iph = _iph as *mut libc::iphdr;

    ptr::write_bytes(iph, 0, 1);

    (*iph).set_version(4);
    (*iph).set_ihl(5);
    (*iph).ttl = 8;
    (*iph).protocol = proto;
    (*iph).saddr = CFG_SADDR4.sin_addr.s_addr;
    (*iph).daddr = CFG_DADDR4.sin_addr.s_addr;
    (*iph).tot_len = libc::htons((size_of::<libc::iphdr>() + len as usize) as u16);
    (*iph).check = checksum_fold(iph as *mut c_void, size_of::<libc::iphdr>(), 0);

    IPH_ADDR_P = &mut (*iph).saddr as *mut _ as *mut c_void;

    iph.add(1) as *mut c_void
}

unsafe fn build_packet_ipv6(_ip6h: *mut c_void, proto: u8, len: c_uint) -> *mut c_void {
    let ip6h = _ip6h as *mut libc::ip6_hdr;

    ptr::write_bytes(ip6h, 0, 1);

    (*ip6h).ip6_vfc = 6 << 4;
    (*ip6h).ip6_plen = libc::htons(len as u16);
    (*ip6h).ip6_nxt = proto;
    (*ip6h).ip6_hlim = 64;
    (*ip6h).ip6_src = CFG_SADDR6.sin6_addr;
    (*ip6h).ip6_dst = CFG_DADDR6.sin6_addr;

    IPH_ADDR_P = &mut (*ip6h).ip6_src as *mut _ as *mut c_void;

    ip6h.add(1) as *mut c_void
}

unsafe fn build_packet_udp(_uh: *mut c_void) -> *mut c_void {
    let uh = _uh as *mut libc::udphdr;

    (*uh).uh_sport = libc::htons(CFG_PORT_SRC);
    (*uh).uh_dport = libc::htons(CFG_PORT_DST);
    (*uh).uh_ulen = libc::htons((size_of::<libc::udphdr>() as c_int + CFG_PAYLOAD_LEN) as u16);
    (*uh).uh_sum = 0;

    /* choose source port so that uh->check adds up to zero */
    if CFG_ZERO_SUM {
        (*uh).uh_sport = 0;
        (*uh).uh_sport = checksum(uh as *mut c_void, libc::IPPROTO_UDP as u16,
                                  size_of::<libc::udphdr>() + CFG_PAYLOAD_LEN as usize);

        libc::fprintf(libc::stderr, cstr!("tx: changing sport: %hu -> %hu\n"),
                      CFG_PORT_SRC as c_int, libc::ntohs((*uh).uh_sport) as c_int);
        CFG_PORT_SRC = libc::ntohs((*uh).uh_sport);
    }

    if CFG_ZERO_DISABLE {
        (*uh).uh_sum = 0;
    } else {
        (*uh).uh_sum = checksum(uh as *mut c_void, libc::IPPROTO_UDP as u16,
                                size_of::<libc::udphdr>() + CFG_PAYLOAD_LEN as usize);
    }

    if CFG_BAD_CSUM {
        (*uh).uh_sum = !(*uh).uh_sum;
    }

    libc::fprintf(libc::stderr, cstr!("tx: sending checksum: 0x%x\n"), (*uh).uh_sum as c_int);
    uh.add(1) as *mut c_void
}

unsafe fn build_packet_tcp(_th: *mut c_void) -> *mut c_char {
    let th = _th as *mut libc::tcphdr;

    (*th).th_sport = libc::htons(CFG_PORT_SRC);
    (*th).th_dport = libc::htons(CFG_PORT_DST);
    (*th).set_th_off(5);
    (*th).th_sum = 0;

    (*th).th_sum = checksum(th as *mut c_void, libc::IPPROTO_TCP as u16,
                            size_of::<libc::tcphdr>() + CFG_PAYLOAD_LEN as usize);

    if CFG_BAD_CSUM {
        (*th).th_sum = !(*th).th_sum;
    }

    libc::fprintf(libc::stderr, cstr!("tx: sending checksum: 0x%x\n"), (*th).th_sum as c_int);
    th.add(1) as *mut c_char
}

unsafe fn build_packet_udp_encap(_uh: *mut c_void) -> *mut c_char {
    let uh = _uh as *mut libc::udphdr;
    let eh = (_uh as *mut u8).add(size_of::<libc::udphdr>()) as *mut udp_encap_hdr;

    /* outer dst == inner dst, to simplify BPF filter
     * outer src != inner src, to demultiplex on recv
     */
    (*uh).uh_dport = libc::htons(CFG_PORT_DST);
    (*uh).uh_sport = libc::htons(CFG_PORT_SRC_ENCAP);
    (*uh).uh_sum = 0;
    (*uh).uh_ulen = libc::htons((size_of::<libc::udphdr>() + size_of::<udp_encap_hdr>() +
                                 size_of::<libc::tcphdr>() + CFG_PAYLOAD_LEN as usize) as u16);

    (*eh).nexthdr = libc::IPPROTO_TCP as u8;

    build_packet_tcp(eh.add(1) as *mut c_void)
}

unsafe fn build_packet(buf: *mut c_char, max_len: c_int, len: *mut c_int) -> *mut c_char {
    let proto: u8;
    let mut off: *mut c_char;
    let mut tlen: c_int;

    if CFG_RANDOM_SEED != 0 {
        let buf32 = buf as *mut c_int;
        let mut i: c_int = 0;

        while i < max_len / size_of::<c_int>() as c_int {
            *buf32.add(i as usize) = libc::rand();
            i += 1;
        }
    } else {
        libc::memset(buf as *mut c_void, CFG_PAYLOAD_CHAR, max_len as usize);
    }

    if CFG_PROTO == libc::IPPROTO_UDP {
        tlen = size_of::<libc::udphdr>() as c_int + CFG_PAYLOAD_LEN;
    } else {
        tlen = size_of::<libc::tcphdr>() as c_int + CFG_PAYLOAD_LEN;
    }

    if CFG_ENCAP {
        proto = libc::IPPROTO_UDP as u8;
        tlen += ENC_HEADER_LEN as c_int;
    } else {
        proto = CFG_PROTO as u8;
    }

    if CFG_FAMILY == libc::PF_INET {
        off = build_packet_ipv4(buf as *mut c_void, proto, tlen as c_uint) as *mut c_char;
    } else {
        off = build_packet_ipv6(buf as *mut c_void, proto, tlen as c_uint) as *mut c_char;
    }

    if CFG_ENCAP {
        off = build_packet_udp_encap(off as *mut c_void);
    } else if CFG_PROTO == libc::IPPROTO_UDP {
        off = build_packet_udp(off as *mut c_void) as *mut c_char;
    } else {
        off = build_packet_tcp(off as *mut c_void);
    }

    /* only pass the payload, but still compute headers for cfg_zero_sum */
    if CFG_SEND_UDP {
        *len = CFG_PAYLOAD_LEN;
        return off;
    }

    *len = off.offset_from(buf) as c_int + CFG_PAYLOAD_LEN;
    buf
}

unsafe fn open_inet(ipproto: c_int, protocol: c_int) -> c_int {
    let fd = libc::socket(CFG_FAMILY, ipproto, protocol);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket inet"));
    }

    if CFG_FAMILY == libc::PF_INET6 {
        /* may have been updated by cfg_zero_sum */
        CFG_SADDR6.sin6_port = libc::htons(CFG_PORT_SRC);

        if libc::bind(fd, &CFG_SADDR6 as *const _ as *const libc::sockaddr,
                      size_of::<libc::sockaddr_in6>() as libc::socklen_t) != 0 {
            error(1, *libc::__errno_location(), cstr!("bind dgram 6"));
        }
        if libc::connect(fd, &CFG_DADDR6 as *const _ as *const libc::sockaddr,
                         size_of::<libc::sockaddr_in6>() as libc::socklen_t) != 0 {
            error(1, *libc::__errno_location(), cstr!("connect dgram 6"));
        }
    } else {
        /* may have been updated by cfg_zero_sum */
        CFG_SADDR4.sin_port = libc::htons(CFG_PORT_SRC);

        if libc::bind(fd, &CFG_SADDR4 as *const _ as *const libc::sockaddr,
                      size_of::<libc::sockaddr_in>() as libc::socklen_t) != 0 {
            error(1, *libc::__errno_location(), cstr!("bind dgram 4"));
        }
        if libc::connect(fd, &CFG_DADDR4 as *const _ as *const libc::sockaddr,
                         size_of::<libc::sockaddr_in>() as libc::socklen_t) != 0 {
            error(1, *libc::__errno_location(), cstr!("connect dgram 4"));
        }
    }

    fd
}

unsafe fn open_packet() -> c_int {
    let mut one: c_int = 1;

    let fd = libc::socket(libc::PF_PACKET, libc::SOCK_RAW, 0);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket packet"));
    }

    if libc::setsockopt(fd, SOL_PACKET, PACKET_VNET_HDR, &mut one as *mut _ as *const c_void,
                        size_of::<c_int>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("setsockopt packet_vnet_ndr"));
    }

    fd
}

unsafe fn send_inet(fd: c_int, buf: *const c_char, len: c_int) {
    let ret = libc::write(fd, buf as *const c_void, len as usize);
    if ret == -1 {
        error(1, *libc::__errno_location(), cstr!("write"));
    }
    if ret != len as isize {
        error(1, 0, cstr!("write: %d"), ret as c_int);
    }
}

unsafe fn eth_str_to_addr(str_: *const c_char, eth: *mut u8) {
    if libc::sscanf(str_, cstr!("%hhx:%hhx:%hhx:%hhx:%hhx:%hhx"),
                    eth.add(0), eth.add(1), eth.add(2), eth.add(3), eth.add(4), eth.add(5)) != 6 {
        error(1, 0, cstr!("cannot parse mac addr %s"), str_);
    }
}

unsafe fn send_packet(fd: c_int, buf: *const c_char, len: c_int) {
    let mut vh: virtio_net_hdr = zeroed();
    let mut addr: libc::sockaddr_ll = zeroed();
    let mut msg: libc::msghdr = zeroed();
    let mut eth: ethhdr = zeroed();
    let mut iov: [libc::iovec; 3] = zeroed();
    let ret: isize;

    addr.sll_family = libc::AF_PACKET as libc::sa_family_t;
    addr.sll_halen = ETH_ALEN as u8;
    addr.sll_ifindex = if_nametoindex(CFG_IFNAME) as c_int;
    if addr.sll_ifindex == 0 {
        error(1, *libc::__errno_location(), cstr!("if_nametoindex %s"), CFG_IFNAME);
    }

    vh.flags = VIRTIO_NET_HDR_F_NEEDS_CSUM;
    if CFG_FAMILY == libc::PF_INET6 {
        vh.csum_start = (size_of::<ethhdr>() + size_of::<libc::ip6_hdr>()) as u16;
        addr.sll_protocol = libc::htons(ETH_P_IPV6 as u16);
    } else {
        vh.csum_start = (size_of::<ethhdr>() + size_of::<libc::iphdr>()) as u16;
        addr.sll_protocol = libc::htons(ETH_P_IP as u16);
    }

    if CFG_ENCAP {
        vh.csum_start = vh.csum_start.wrapping_add(ENC_HEADER_LEN as u16);
    }

    if CFG_PROTO == libc::IPPROTO_TCP {
        vh.csum_offset = memoffset::offset_of!(libc::tcphdr, th_sum) as u16;
        vh.hdr_len = vh.csum_start + size_of::<libc::tcphdr>() as u16;
    } else {
        vh.csum_offset = memoffset::offset_of!(libc::udphdr, uh_sum) as u16;
        vh.hdr_len = vh.csum_start + size_of::<libc::udphdr>() as u16;
    }

    eth_str_to_addr(CFG_MAC_SRC, eth.h_source.as_mut_ptr());
    eth_str_to_addr(CFG_MAC_DST, eth.h_dest.as_mut_ptr());
    eth.h_proto = addr.sll_protocol;

    iov[0].iov_base = &mut vh as *mut _ as *mut c_void;
    iov[0].iov_len = size_of::<virtio_net_hdr>();
    iov[1].iov_base = &mut eth as *mut _ as *mut c_void;
    iov[1].iov_len = size_of::<ethhdr>();
    iov[2].iov_base = buf as *mut c_void;
    iov[2].iov_len = len as usize;

    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = iov.len();
    msg.msg_name = &mut addr as *mut _ as *mut c_void;
    msg.msg_namelen = size_of::<libc::sockaddr_ll>() as libc::socklen_t;

    ret = libc::sendmsg(fd, &msg, 0);
    if ret == -1 {
        error(1, *libc::__errno_location(), cstr!("sendmsg packet"));
    }
    if ret != (size_of::<virtio_net_hdr>() + size_of::<ethhdr>() + len as usize) as isize {
        error(1, *libc::__errno_location(), cstr!("sendmsg packet: %u"), ret as c_uint);
    }
}

unsafe fn recv_prepare_udp() -> c_int {
    let fd = libc::socket(CFG_FAMILY, libc::SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket r"));
    }

    if libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_RCVBUF,
                        &CFG_RCVBUF as *const _ as *const c_void,
                        size_of::<c_int>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("setsockopt SO_RCVBUF r"));
    }

    if CFG_FAMILY == libc::PF_INET6 {
        if libc::bind(fd, &CFG_DADDR6 as *const _ as *const libc::sockaddr,
                      size_of::<libc::sockaddr_in6>() as libc::socklen_t) != 0 {
            error(1, *libc::__errno_location(), cstr!("bind r"));
        }
    } else if libc::bind(fd, &CFG_DADDR4 as *const _ as *const libc::sockaddr,
                         size_of::<libc::sockaddr_in>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("bind r"));
    }

    fd
}

fn bpf_stmt(code: u16, k: u32) -> sock_filter {
    sock_filter { code, jt: 0, jf: 0, k }
}

fn bpf_jump(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

/* Filter out all traffic that is not cfg_proto with our destination port.
 *
 * Otherwise background noise may cause PF_PACKET receive queue overflow,
 * dropping the expected packets and failing the test.
 */
unsafe fn __recv_prepare_packet_filter(fd: c_int, off_nexthdr: c_int, off_dport: c_int) {
    let mut filter = [
        bpf_stmt(BPF_LD + BPF_B + BPF_ABS, SKF_AD_OFF + SKF_AD_PKTTYPE),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K, PACKET_HOST as u32, 0, 4),
        bpf_stmt(BPF_LD + BPF_B + BPF_ABS, off_nexthdr as u32),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K,
                 if CFG_ENCAP { libc::IPPROTO_UDP as u32 } else { CFG_PROTO as u32 }, 0, 2),
        bpf_stmt(BPF_LD + BPF_H + BPF_ABS, off_dport as u32),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K, CFG_PORT_DST as u32, 1, 0),
        bpf_stmt(BPF_RET + BPF_K, 0),
        bpf_stmt(BPF_RET + BPF_K, 0xFFFF),
    ];
    let mut prog = sock_fprog { len: filter.len() as u16, filter: filter.as_mut_ptr() };

    if libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_ATTACH_FILTER,
                        &mut prog as *mut _ as *const c_void,
                        size_of::<sock_fprog>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("setsockopt filter"));
    }
}

unsafe fn recv_prepare_packet_filter(fd: c_int) {
    let off_dport = memoffset::offset_of!(libc::tcphdr, th_dport) as c_int; /* same for udp */

    if CFG_FAMILY == libc::AF_INET {
        __recv_prepare_packet_filter(fd, memoffset::offset_of!(libc::iphdr, protocol) as c_int,
                                     size_of::<libc::iphdr>() as c_int + off_dport);
    } else {
        __recv_prepare_packet_filter(fd, memoffset::offset_of!(libc::ip6_hdr, ip6_nxt) as c_int,
                                     size_of::<libc::ip6_hdr>() as c_int + off_dport);
    }
}

unsafe fn recv_prepare_packet_bind(fd: c_int) {
    let mut laddr: libc::sockaddr_ll = zeroed();

    laddr.sll_family = libc::AF_PACKET as libc::sa_family_t;

    if CFG_FAMILY == libc::PF_INET {
        laddr.sll_protocol = libc::htons(ETH_P_IP as u16);
    } else {
        laddr.sll_protocol = libc::htons(ETH_P_IPV6 as u16);
    }

    laddr.sll_ifindex = if_nametoindex(CFG_IFNAME) as c_int;
    if laddr.sll_ifindex == 0 {
        error(1, 0, cstr!("if_nametoindex %s"), CFG_IFNAME);
    }

    if libc::bind(fd, &laddr as *const _ as *const libc::sockaddr,
                  size_of::<libc::sockaddr_ll>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("bind pf_packet"));
    }
}

unsafe fn recv_prepare_packet() -> c_int {
    let mut one: c_int = 1;

    let fd = libc::socket(libc::PF_PACKET, libc::SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket p"));
    }

    if libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_RCVBUF,
                        &CFG_RCVBUF as *const _ as *const c_void,
                        size_of::<c_int>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("setsockopt SO_RCVBUF p"));
    }

    /* enable auxdata to recv checksum status (valid vs unknown) */
    if libc::setsockopt(fd, SOL_PACKET, PACKET_AUXDATA, &mut one as *mut _ as *const c_void,
                        size_of::<c_int>() as libc::socklen_t) != 0 {
        error(1, *libc::__errno_location(), cstr!("setsockopt auxdata"));
    }

    /* install filter to restrict packet flow to match */
    recv_prepare_packet_filter(fd);

    /* bind to address family to start packet flow */
    recv_prepare_packet_bind(fd);

    fd
}

unsafe fn recv_udp(fd: c_int) -> c_int {
    static mut BUF: [c_char; MAX_PAYLOAD_LEN] = [0; MAX_PAYLOAD_LEN];
    let mut count: c_int = 0;

    loop {
        let ret = libc::recv(fd, BUF.as_mut_ptr() as *mut c_void, BUF.len(), libc::MSG_DONTWAIT);
        if ret == -1 && *libc::__errno_location() == libc::EAGAIN {
            break;
        }
        if ret == -1 {
            error(1, *libc::__errno_location(), cstr!("recv r"));
        }

        libc::fprintf(libc::stderr, cstr!("rx: udp: len=%u\n"), ret as c_uint);
        count += 1;
    }

    count
}

unsafe fn recv_verify_csum(th: *mut c_void, len: c_int, sport: u16, csum_field: u16) -> c_int {
    let csum = checksum(th, CFG_PROTO as u16, len as usize);

    libc::fprintf(libc::stderr, cstr!("rx: pkt: sport=%hu len=%u csum=0x%hx verify=0x%hx\n"),
                  sport as c_int, len as c_uint, csum_field as c_int, csum as c_int);

    /* csum must be zero unless cfg_bad_csum indicates bad csum */
    if csum != 0 && !CFG_BAD_CSUM {
        libc::fprintf(libc::stderr, cstr!("pkt: bad csum\n"));
        return 1;
    } else if CFG_BAD_CSUM && csum == 0 {
        libc::fprintf(libc::stderr, cstr!("pkt: good csum, while bad expected\n"));
        return 1;
    }

    if CFG_ZERO_SUM && csum_field != 0xFFFF {
        libc::fprintf(libc::stderr, cstr!("pkt: zero csum: field should be 0xFFFF, is 0x%hx\n"),
                      csum_field as c_int);
        return 1;
    }

    0
}

unsafe fn recv_verify_packet_tcp(th: *mut c_void, len: c_int) -> c_int {
    let tcph = th as *mut libc::tcphdr;

    if len < size_of::<libc::tcphdr>() as c_int || (*tcph).th_dport != libc::htons(CFG_PORT_DST) {
        return -1;
    }

    recv_verify_csum(th, len, libc::ntohs((*tcph).th_sport), (*tcph).th_sum)
}

unsafe fn recv_verify_packet_udp_encap(th: *mut c_void, len: c_int) -> c_int {
    let eh = th as *mut udp_encap_hdr;

    if len < size_of::<udp_encap_hdr>() as c_int || (*eh).nexthdr != libc::IPPROTO_TCP as u8 {
        return -1;
    }

    recv_verify_packet_tcp(eh.add(1) as *mut c_void, len - size_of::<udp_encap_hdr>() as c_int)
}

unsafe fn recv_verify_packet_udp(th: *mut c_void, len: c_int) -> c_int {
    let udph = th as *mut libc::udphdr;

    if len < size_of::<libc::udphdr>() as c_int {
        return -1;
    }

    if (*udph).uh_dport != libc::htons(CFG_PORT_DST) {
        return -1;
    }

    if (*udph).uh_sport == libc::htons(CFG_PORT_SRC_ENCAP) {
        return recv_verify_packet_udp_encap(udph.add(1) as *mut c_void,
                                            len - size_of::<libc::udphdr>() as c_int);
    }

    recv_verify_csum(th, len, libc::ntohs((*udph).uh_sport), (*udph).uh_sum)
}

unsafe fn recv_verify_packet_ipv4(nh: *mut c_void, mut len: c_int) -> c_int {
    let iph = nh as *mut libc::iphdr;
    let proto: u16 = if CFG_ENCAP { libc::IPPROTO_UDP as u16 } else { CFG_PROTO as u16 };
    let ip_len: u16;

    if len < size_of::<libc::iphdr>() as c_int || (*iph).protocol != proto as u8 {
        return -1;
    }

    ip_len = libc::ntohs((*iph).tot_len);
    if ip_len as c_int > len || (ip_len as usize) < size_of::<libc::iphdr>() {
        return -1;
    }

    len = ip_len as c_int;
    IPH_ADDR_P = &mut (*iph).saddr as *mut _ as *mut c_void;
    if proto == libc::IPPROTO_TCP as u16 {
        recv_verify_packet_tcp(iph.add(1) as *mut c_void, len - size_of::<libc::iphdr>() as c_int)
    } else {
        recv_verify_packet_udp(iph.add(1) as *mut c_void, len - size_of::<libc::iphdr>() as c_int)
    }
}

unsafe fn recv_verify_packet_ipv6(nh: *mut c_void, len: c_int) -> c_int {
    let ip6h = nh as *mut libc::ip6_hdr;
    let proto: u16 = if CFG_ENCAP { libc::IPPROTO_UDP as u16 } else { CFG_PROTO as u16 };
    let payload_len: u16;

    if len < size_of::<libc::ip6_hdr>() as c_int || (*ip6h).ip6_nxt != proto as u8 {
        return -1;
    }

    payload_len = libc::ntohs((*ip6h).ip6_plen);
    if payload_len as c_int > len - size_of::<libc::ip6_hdr>() as c_int {
        return -1;
    }

    IPH_ADDR_P = &mut (*ip6h).ip6_src as *mut _ as *mut c_void;
    if proto == libc::IPPROTO_TCP as u16 {
        recv_verify_packet_tcp(ip6h.add(1) as *mut c_void, payload_len as c_int)
    } else {
        recv_verify_packet_udp(ip6h.add(1) as *mut c_void, payload_len as c_int)
    }
}

/* return whether auxdata includes TP_STATUS_CSUM_VALID */
unsafe fn recv_get_packet_csum_status(msg: *mut libc::msghdr) -> u32 {
    let mut aux: *mut tpacket_auxdata = ptr::null_mut();
    let mut cm: *mut libc::cmsghdr;

    if (*msg).msg_flags & libc::MSG_CTRUNC != 0 {
        error(1, 0, cstr!("cmsg: truncated"));
    }

    cm = libc::CMSG_FIRSTHDR(msg);
    while !cm.is_null() {
        if (*cm).cmsg_level != SOL_PACKET || (*cm).cmsg_type != PACKET_AUXDATA {
            error(1, 0, cstr!("cmsg: level=%d type=%d\n"), (*cm).cmsg_level, (*cm).cmsg_type);
        }

        if (*cm).cmsg_len != libc::CMSG_LEN(size_of::<tpacket_auxdata>() as c_uint) as usize {
            error(1, 0, cstr!("cmsg: len=%zu expected=%zu"),
                  (*cm).cmsg_len, libc::CMSG_LEN(size_of::<tpacket_auxdata>() as c_uint) as usize);
        }

        aux = libc::CMSG_DATA(cm) as *mut tpacket_auxdata;
        cm = libc::CMSG_NXTHDR(msg, cm);
    }

    if aux.is_null() {
        error(1, 0, cstr!("cmsg: no auxdata"));
    }

    (*aux).tp_status
}

unsafe fn recv_packet(fd: c_int) -> c_int {
    static mut _BUF: [c_char; MAX_HEADER_LEN + MAX_PAYLOAD_LEN] = [0; MAX_HEADER_LEN + MAX_PAYLOAD_LEN];
    let mut total: c_ulong = 0;
    let mut bad_csums: c_ulong = 0;
    let mut bad_validations: c_ulong = 0;
    let mut ctrl: [c_char; 64] = [0; 64]; /* CMSG_SPACE(sizeof(struct tpacket_auxdata)) */
    let buf = _BUF.as_mut_ptr() as *mut pkt;
    let mut msg: libc::msghdr = zeroed();
    let mut tp_status: u32;
    let mut iov: libc::iovec = zeroed();
    let mut len: isize;
    let mut ret: c_int;

    iov.iov_base = _BUF.as_mut_ptr() as *mut c_void;
    iov.iov_len = _BUF.len();

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = ctrl.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = ctrl.len();

    loop {
        msg.msg_flags = 0;

        len = libc::recvmsg(fd, &mut msg, libc::MSG_DONTWAIT);
        if len == -1 && *libc::__errno_location() == libc::EAGAIN {
            break;
        }
        if len == -1 {
            error(1, *libc::__errno_location(), cstr!("recv p"));
        }

        tp_status = recv_get_packet_csum_status(&mut msg);

        /* GRO might coalesce randomized packets. Such GSO packets are
         * then reinitialized for csum offload (CHECKSUM_PARTIAL), with
         * a pseudo csum. Do not try to validate these checksums.
         */
        if tp_status & TP_STATUS_CSUMNOTREADY != 0 {
            libc::fprintf(libc::stderr, cstr!("cmsg: GSO packet has partial csum: skip\n"));
            continue;
        }

        if CFG_FAMILY == libc::PF_INET6 {
            ret = recv_verify_packet_ipv6(buf as *mut c_void, len as c_int);
        } else {
            ret = recv_verify_packet_ipv4(buf as *mut c_void, len as c_int);
        }

        if ret == -1 {
            /* skip: non-matching */
            continue;
        }

        total += 1;
        if ret == 1 {
            bad_csums += 1;
        }

        /* Fail if kernel returns valid for known bad csum.
         * Do not fail if kernel does not validate a good csum:
         * Absence of validation does not imply invalid.
         */
        if tp_status & TP_STATUS_CSUM_VALID != 0 && CFG_BAD_CSUM {
            libc::fprintf(libc::stderr, cstr!("cmsg: expected bad csum, pf_packet returns valid\n"));
            bad_validations += 1;
        }
    }

    if bad_csums != 0 || bad_validations != 0 {
        error(1, 0, cstr!("rx: errors at pf_packet: total=%lu bad_csums=%lu bad_valids=%lu\n"),
              total, bad_csums, bad_validations);
    }

    total as c_int
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    let mut daddr: *const c_char = ptr::null();
    let mut saddr: *const c_char = ptr::null();
    let mut c: c_int;

    loop {
        c = libc::getopt(argc, argv, cstr!("46d:D:eEi:l:L:n:r:PRs:S:tTuUzZ"));
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => CFG_FAMILY = libc::PF_INET,
            '6' => CFG_FAMILY = libc::PF_INET6,
            'd' => CFG_MAC_DST = libc::optarg,
            'D' => daddr = libc::optarg,
            'e' => CFG_ENCAP = true,
            'E' => CFG_BAD_CSUM = true,
            'i' => CFG_IFNAME = libc::optarg,
            'l' => CFG_PAYLOAD_LEN = libc::strtol(libc::optarg, ptr::null_mut(), 0) as c_int,
            'L' => CFG_TIMEOUT_MS = (libc::strtol(libc::optarg, ptr::null_mut(), 0) * 1000) as c_int,
            'n' => CFG_NUM_PKT = libc::strtol(libc::optarg, ptr::null_mut(), 0) as c_int,
            'r' => CFG_RANDOM_SEED = libc::strtol(libc::optarg, ptr::null_mut(), 0) as c_uint,
            'P' => CFG_SEND_PFPACKET = true,
            'R' => {
                /* only Rx: used with two machine tests */
                CFG_DO_TX = false;
            }
            's' => CFG_MAC_SRC = libc::optarg,
            'S' => saddr = libc::optarg,
            't' => CFG_PROTO = libc::IPPROTO_TCP,
            'T' => {
                /* only Tx: used with two machine tests */
                CFG_DO_RX = false;
            }
            'u' => CFG_PROTO = libc::IPPROTO_UDP,
            'U' => {
                /* send using real udp socket,
                 * to exercise tx checksum offload
                 */
                CFG_SEND_UDP = true;
            }
            'z' => CFG_ZERO_DISABLE = true,
            'Z' => CFG_ZERO_SUM = true,
            _ => error(1, 0, cstr!("unknown arg %c"), c),
        }
    }

    if daddr.is_null() || saddr.is_null() {
        error(1, 0, cstr!("Must pass -D <daddr> and -S <saddr>"));
    }

    if CFG_DO_TX && CFG_SEND_PFPACKET && (CFG_MAC_SRC.is_null() || CFG_MAC_DST.is_null()) {
        error(1, 0, cstr!("Transmit with pf_packet requires mac addresses"));
    }

    if CFG_PAYLOAD_LEN > MAX_PAYLOAD_LEN as c_int {
        error(1, 0, cstr!("Payload length exceeds max"));
    }

    if CFG_PROTO != libc::IPPROTO_UDP && (CFG_ZERO_SUM || CFG_ZERO_DISABLE) {
        error(1, 0, cstr!("Only UDP supports zero csum"));
    }

    if CFG_ZERO_SUM && !CFG_SEND_UDP {
        error(1, 0, cstr!("Zero checksum conversion requires -U for tx csum offload"));
    }
    if CFG_ZERO_SUM && CFG_BAD_CSUM {
        error(1, 0, cstr!("Cannot combine zero checksum conversion and invalid checksum"));
    }
    if CFG_ZERO_SUM && CFG_RANDOM_SEED != 0 {
        error(1, 0, cstr!("Cannot combine zero checksum conversion with randomization"));
    }

    if CFG_FAMILY == libc::PF_INET6 {
        CFG_SADDR6.sin6_port = libc::htons(CFG_PORT_SRC);
        CFG_DADDR6.sin6_port = libc::htons(CFG_PORT_DST);

        if libc::inet_pton(CFG_FAMILY, daddr, &mut CFG_DADDR6.sin6_addr as *mut _ as *mut c_void) != 1 {
            error(1, *libc::__errno_location(), cstr!("Cannot parse ipv6 -D"));
        }
        if libc::inet_pton(CFG_FAMILY, saddr, &mut CFG_SADDR6.sin6_addr as *mut _ as *mut c_void) != 1 {
            error(1, *libc::__errno_location(), cstr!("Cannot parse ipv6 -S"));
        }
    } else {
        CFG_SADDR4.sin_port = libc::htons(CFG_PORT_SRC);
        CFG_DADDR4.sin_port = libc::htons(CFG_PORT_DST);

        if libc::inet_pton(CFG_FAMILY, daddr, &mut CFG_DADDR4.sin_addr as *mut _ as *mut c_void) != 1 {
            error(1, *libc::__errno_location(), cstr!("Cannot parse ipv4 -D"));
        }
        if libc::inet_pton(CFG_FAMILY, saddr, &mut CFG_SADDR4.sin_addr as *mut _ as *mut c_void) != 1 {
            error(1, *libc::__errno_location(), cstr!("Cannot parse ipv4 -S"));
        }
    }

    if CFG_DO_TX && CFG_RANDOM_SEED != 0 {
        /* special case: time-based seed */
        if CFG_RANDOM_SEED == 1 {
            CFG_RANDOM_SEED = gettimeofday_ms() as c_uint;
        }
        libc::srand(CFG_RANDOM_SEED);
        libc::fprintf(libc::stderr, cstr!("randomization seed: %u\n"), CFG_RANDOM_SEED);
    }
}

unsafe fn do_tx() {
    static mut _BUF: [c_char; MAX_HEADER_LEN + MAX_PAYLOAD_LEN] = [0; MAX_HEADER_LEN + MAX_PAYLOAD_LEN];
    let mut len: c_int = 0;
    let fd: c_int;
    let mut i: c_int;
    let mut buf = build_packet(_BUF.as_mut_ptr(), _BUF.len() as c_int, &mut len);

    if CFG_SEND_PFPACKET {
        fd = open_packet();
    } else if CFG_SEND_UDP {
        fd = open_inet(libc::SOCK_DGRAM, 0);
    } else {
        fd = open_inet(libc::SOCK_RAW, libc::IPPROTO_RAW);
    }

    i = 0;
    while i < CFG_NUM_PKT {
        if CFG_SEND_PFPACKET {
            send_packet(fd, buf, len);
        } else {
            send_inet(fd, buf, len);
        }

        /* randomize each packet individually to increase coverage */
        if CFG_RANDOM_SEED != 0 {
            CFG_PAYLOAD_LEN = libc::rand() % MAX_PAYLOAD_LEN as c_int;
            buf = build_packet(_BUF.as_mut_ptr(), _BUF.len() as c_int, &mut len);
        }
        i += 1;
    }

    if libc::close(fd) != 0 {
        error(1, *libc::__errno_location(), cstr!("close tx"));
    }
}

unsafe fn do_rx(fdp: c_int, fdr: c_int) {
    let mut count_udp: c_ulong = 0;
    let mut count_pkt: c_ulong = 0;
    let mut tleft: c_long;
    let tstop: c_long;
    let mut pfd: libc::pollfd = zeroed();

    tstop = gettimeofday_ms() as c_long + CFG_TIMEOUT_MS as c_long;
    tleft = CFG_TIMEOUT_MS as c_long;

    loop {
        pfd.events = libc::POLLIN;
        pfd.fd = fdp;
        if libc::poll(&mut pfd, 1, tleft as c_int) == -1 {
            error(1, *libc::__errno_location(), cstr!("poll"));
        }

        if pfd.revents & libc::POLLIN != 0 {
            count_pkt += recv_packet(fdp) as c_ulong;
        }

        if CFG_PROTO == libc::IPPROTO_UDP {
            count_udp += recv_udp(fdr) as c_ulong;
        }

        tleft = tstop - gettimeofday_ms() as c_long;
        if tleft <= 0 {
            break;
        }
    }

    if libc::close(fdr) != 0 {
        error(1, *libc::__errno_location(), cstr!("close r"));
    }
    if libc::close(fdp) != 0 {
        error(1, *libc::__errno_location(), cstr!("close p"));
    }

    if count_pkt < CFG_NUM_PKT as c_ulong {
        error(1, 0, cstr!("rx: missing packets at pf_packet: %lu < %u"),
              count_pkt, CFG_NUM_PKT as c_uint);
    }

    if CFG_PROTO == libc::IPPROTO_UDP {
        if CFG_BAD_CSUM && count_udp != 0 {
            error(1, 0, cstr!("rx: unexpected packets at udp"));
        }
        if !CFG_BAD_CSUM && count_udp == 0 {
            error(1, 0, cstr!("rx: missing packets at udp"));
        }
    }
}

fn main() {
    unsafe {
        let mut fdp: c_int = -1;
        let mut fdr: c_int = -1; /* -1 to silence -Wmaybe-uninitialized */

        parse_args(libc::__argc, libc::__argv);

        /* open receive sockets before transmitting */
        if CFG_DO_RX {
            fdp = recv_prepare_packet();
            fdr = recv_prepare_udp();
        }

        if CFG_DO_TX {
            do_tx();
        }

        if CFG_DO_RX {
            do_rx(fdp, fdr);
        }

        libc::fprintf(libc::stderr, cstr!("OK\n"));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
