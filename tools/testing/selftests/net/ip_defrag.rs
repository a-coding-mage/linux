// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/net/ip_defrag.c.
// C dependencies: arpa/inet.h, errno.h, error.h, linux/in.h, netinet/ip.h,
// netinet/ip6.h, netinet/udp.h, stdbool.h, stdio.h, stdlib.h, string.h,
// time.h, unistd.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type SocklenT = u32;
type SsizeT = isize;
type TimeT = i64;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_RAW: c_int = 3;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const IPPROTO_RAW: c_int = 255;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_FRAGMENT: u8 = 44;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const IP_MAXPACKET: usize = 65535;
const ETIMEDOUT: c_int = 110;
const EAGAIN: c_int = 11;
const EPERM: c_int = 1;

const IP4_HLEN: usize = size_of::<Ip>();
const IP6_HLEN: usize = size_of::<Ip6Hdr>();
const UDP_HLEN: usize = size_of::<UdpHdr>();

/* IPv6 fragment header lenth. */
const FRAG_HLEN: usize = 8;

const MSG_LEN_MAX: usize = 10000; /* Max UDP payload length. */

const IP4_MF: c_uint = 1u32 << 13; /* IPv4 MF flag. */
const IP6_MF: c_uint = 1; /* IPv6 MF flag. */

const CSUM_MANGLED_0: u16 = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct In6Addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct Sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct SockaddrIn {
    sin_family: u16,
    sin_port: u16,
    sin_addr: InAddr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct SockaddrIn6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: In6Addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct Ip {
    ip_hl_v: u8,
    ip_tos: u8,
    ip_len: u16,
    ip_id: u16,
    ip_off: u16,
    ip_ttl: u8,
    ip_p: u8,
    ip_sum: u16,
    ip_src: InAddr,
    ip_dst: InAddr,
}

impl Ip {
    unsafe fn set_ip_hl(&mut self, val: u8) {
        self.ip_hl_v = (self.ip_hl_v & 0xf0) | (val & 0x0f);
    }

    unsafe fn set_ip_v(&mut self, val: u8) {
        self.ip_hl_v = (self.ip_hl_v & 0x0f) | ((val & 0x0f) << 4);
    }
}

#[repr(C)]
struct Ip6Hdr {
    ip6_flow: u32,
    ip6_plen: u16,
    ip6_nxt: u8,
    ip6_hops: u8,
    ip6_src: In6Addr,
    ip6_dst: In6Addr,
}

#[repr(C)]
struct Ip6Frag {
    ip6f_nxt: u8,
    ip6f_reserved: u8,
    ip6f_offlg: u16,
    ip6f_ident: u32,
}

#[repr(C)]
struct UdpHdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

unsafe extern "C" {
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> SsizeT;
    fn sendto(
        fd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const Sockaddr,
        addrlen: SocklenT,
    ) -> SsizeT;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const Sockaddr, addrlen: SocklenT) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn rand() -> c_int;
    fn srand(seed: c_uint);
    fn time(tloc: *mut TimeT) -> TimeT;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn ntohs(netshort: u16) -> u16;

    static mut errno: c_int;
    static mut stderr: *mut c_void;
}

static mut CFG_DO_IPV4: bool = false;
static mut CFG_DO_IPV6: bool = false;
static mut CFG_VERBOSE: bool = false;
static mut CFG_OVERLAP: bool = false;
static mut CFG_PERMISSIVE: bool = false;
static mut CFG_PORT: u16 = 9000;

static ADDR4: InAddr = InAddr {
    s_addr: (INADDR_LOOPBACK + 2).to_be(),
};
static ADDR6: In6Addr = In6Addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

static mut PAYLOAD_LEN: c_int = 0;
static mut MAX_FRAG_LEN: c_int = 0;

static mut UDP_PAYLOAD: [u8; MSG_LEN_MAX] = [0; MSG_LEN_MAX];
static mut IP_FRAME: [u8; IP_MAXPACKET] = [0; IP_MAXPACKET];
static mut IP_ID: u32 = 0xabcd;
static mut MSG_COUNTER: c_int = 0;
static mut FRAG_COUNTER: c_int = 0;
static mut SEED: c_uint = 0;

/* Receive a UDP packet. Validate it matches udp_payload. */
unsafe fn recv_validate_udp(fd_udp: c_int) {
    let ret: SsizeT;
    static mut RECV_BUFF: [u8; MSG_LEN_MAX] = [0; MSG_LEN_MAX];

    ret = recv(
        fd_udp,
        RECV_BUFF.as_mut_ptr() as *mut c_void,
        PAYLOAD_LEN as usize,
        0,
    );
    MSG_COUNTER += 1;

    if CFG_OVERLAP {
        if ret == -1 && (errno == ETIMEDOUT || errno == EAGAIN) {
            return; /* OK */
        }
        if !CFG_PERMISSIVE {
            if ret != -1 {
                error(
                    1,
                    0,
                    c"recv: expected timeout; got %d".as_ptr(),
                    ret as c_int,
                );
            }
            error(
                1,
                errno,
                c"recv: expected timeout: %d".as_ptr(),
                errno,
            );
        }
    }

    if ret == -1 {
        error(
            1,
            errno,
            c"recv: payload_len = %d max_frag_len = %d".as_ptr(),
            PAYLOAD_LEN,
            MAX_FRAG_LEN,
        );
    }
    if ret != PAYLOAD_LEN as SsizeT {
        error(
            1,
            0,
            c"recv: wrong size: %d vs %d".as_ptr(),
            ret as c_int,
            PAYLOAD_LEN,
        );
    }
    if memcmp(
        UDP_PAYLOAD.as_ptr() as *const c_void,
        RECV_BUFF.as_ptr() as *const c_void,
        PAYLOAD_LEN as usize,
    ) != 0
    {
        error(1, 0, c"recv: wrong data".as_ptr());
    }
}

unsafe fn raw_checksum(buf: *mut u8, len: c_int, mut sum: u32) -> u32 {
    let mut i: c_int;

    i = 0;
    while i < (len & !(1u32 as c_int)) {
        sum = sum.wrapping_add(ntohs(*(buf.add(i as usize) as *mut u16)) as u32);
        if sum > 0xffff {
            sum -= 0xffff;
        }
        i += 2;
    }

    if i < len {
        sum = sum.wrapping_add((*(buf.add(i as usize)) as u32) << 8);
        if sum > 0xffff {
            sum -= 0xffff;
        }
    }

    sum
}

unsafe fn udp_checksum(iphdr: *mut Ip, udphdr: *mut UdpHdr) -> u16 {
    let mut sum: u32 = 0;
    let res: u16;

    sum = raw_checksum(
        ptr::addr_of_mut!((*iphdr).ip_src) as *mut u8,
        (2 * size_of::<InAddr>()) as c_int,
        IPPROTO_UDP as u32 + (UDP_HLEN + PAYLOAD_LEN as usize) as u32,
    );
    sum = raw_checksum(udphdr as *mut u8, UDP_HLEN as c_int, sum);
    sum = raw_checksum(UDP_PAYLOAD.as_mut_ptr(), PAYLOAD_LEN, sum);
    res = 0xffff & !sum as u16;
    if res != 0 {
        htons(res)
    } else {
        CSUM_MANGLED_0
    }
}

unsafe fn udp6_checksum(iphdr: *mut Ip6Hdr, udphdr: *mut UdpHdr) -> u16 {
    let mut sum: u32 = 0;
    let res: u16;

    sum = raw_checksum(
        ptr::addr_of_mut!((*iphdr).ip6_src) as *mut u8,
        (2 * size_of::<In6Addr>()) as c_int,
        IPPROTO_UDP as u32,
    );
    sum = raw_checksum(
        ptr::addr_of_mut!((*udphdr).len) as *mut u8,
        size_of::<u16>() as c_int,
        sum,
    );
    sum = raw_checksum(udphdr as *mut u8, UDP_HLEN as c_int, sum);
    sum = raw_checksum(UDP_PAYLOAD.as_mut_ptr(), PAYLOAD_LEN, sum);
    res = 0xffff & !sum as u16;
    if res != 0 {
        htons(res)
    } else {
        CSUM_MANGLED_0
    }
}

unsafe fn send_fragment(
    fd_raw: c_int,
    addr: *mut Sockaddr,
    alen: SocklenT,
    offset: c_int,
    ipv6: bool,
) {
    let mut frag_len: c_int;
    let res: c_int;
    let payload_offset: c_int = if offset > 0 { offset - UDP_HLEN as c_int } else { 0 };
    let frag_start: *mut u8 = if ipv6 {
        IP_FRAME.as_mut_ptr().add(IP6_HLEN + FRAG_HLEN)
    } else {
        IP_FRAME.as_mut_ptr().add(IP4_HLEN)
    };

    if offset == 0 {
        let mut udphdr = UdpHdr {
            source: htons(CFG_PORT + 1),
            dest: htons(CFG_PORT),
            len: htons((UDP_HLEN as c_int + PAYLOAD_LEN) as u16),
            check: 0,
        };
        if ipv6 {
            udphdr.check = udp6_checksum(IP_FRAME.as_mut_ptr() as *mut Ip6Hdr, &mut udphdr);
        } else {
            udphdr.check = udp_checksum(IP_FRAME.as_mut_ptr() as *mut Ip, &mut udphdr);
        }
        memcpy(
            frag_start as *mut c_void,
            &udphdr as *const UdpHdr as *const c_void,
            UDP_HLEN,
        );
    }

    if ipv6 {
        let ip6hdr = IP_FRAME.as_mut_ptr() as *mut Ip6Hdr;
        let fraghdr = IP_FRAME.as_mut_ptr().add(IP6_HLEN) as *mut Ip6Frag;
        if PAYLOAD_LEN - payload_offset <= MAX_FRAG_LEN && offset > 0 {
            /* This is the last fragment. */
            frag_len = FRAG_HLEN as c_int + PAYLOAD_LEN - payload_offset;
            (*fraghdr).ip6f_offlg = htons(offset as u16);
        } else {
            frag_len = FRAG_HLEN as c_int + MAX_FRAG_LEN;
            (*fraghdr).ip6f_offlg = htons((offset as c_uint | IP6_MF) as u16);
        }
        (*ip6hdr).ip6_plen = htons(frag_len as u16);
        if offset == 0 {
            memcpy(
                frag_start.add(UDP_HLEN) as *mut c_void,
                UDP_PAYLOAD.as_ptr() as *const c_void,
                (frag_len - FRAG_HLEN as c_int - UDP_HLEN as c_int) as usize,
            );
        } else {
            memcpy(
                frag_start as *mut c_void,
                UDP_PAYLOAD.as_ptr().add(payload_offset as usize) as *const c_void,
                (frag_len - FRAG_HLEN as c_int) as usize,
            );
        }
        frag_len += IP6_HLEN as c_int;
    } else {
        let iphdr = IP_FRAME.as_mut_ptr() as *mut Ip;
        if PAYLOAD_LEN - payload_offset <= MAX_FRAG_LEN && offset > 0 {
            /* This is the last fragment. */
            frag_len = IP4_HLEN as c_int + PAYLOAD_LEN - payload_offset;
            (*iphdr).ip_off = htons((offset / 8) as u16);
        } else {
            frag_len = IP4_HLEN as c_int + MAX_FRAG_LEN;
            (*iphdr).ip_off = htons(((offset / 8) as c_uint | IP4_MF) as u16);
        }
        (*iphdr).ip_len = htons(frag_len as u16);
        if offset == 0 {
            memcpy(
                frag_start.add(UDP_HLEN) as *mut c_void,
                UDP_PAYLOAD.as_ptr() as *const c_void,
                (frag_len - IP4_HLEN as c_int - UDP_HLEN as c_int) as usize,
            );
        } else {
            memcpy(
                frag_start as *mut c_void,
                UDP_PAYLOAD.as_ptr().add(payload_offset as usize) as *const c_void,
                (frag_len - IP4_HLEN as c_int) as usize,
            );
        }
    }

    res = sendto(
        fd_raw,
        IP_FRAME.as_ptr() as *const c_void,
        frag_len as usize,
        0,
        addr,
        alen,
    ) as c_int;
    if res < 0 && errno != EPERM {
        error(1, errno, c"send_fragment".as_ptr());
    }
    if res >= 0 && res != frag_len {
        error(1, 0, c"send_fragment: %d vs %d".as_ptr(), res, frag_len);
    }

    FRAG_COUNTER += 1;
}

unsafe fn send_udp_frags(fd_raw: c_int, addr: *mut Sockaddr, alen: SocklenT, ipv6: bool) {
    let iphdr = IP_FRAME.as_mut_ptr() as *mut Ip;
    let ip6hdr = IP_FRAME.as_mut_ptr() as *mut Ip6Hdr;
    let mut res: c_int;
    let mut offset: c_int;
    let mut frag_len: c_int;

    /* Send the UDP datagram using raw IP fragments: the 0th fragment
     * has the UDP header; other fragments are pieces of udp_payload
     * split in chunks of frag_len size.
     *
     * Odd fragments (1st, 3rd, 5th, etc.) are sent out first, then
     * even fragments (0th, 2nd, etc.) are sent out.
     */
    if ipv6 {
        let fraghdr = IP_FRAME.as_mut_ptr().add(IP6_HLEN) as *mut Ip6Frag;
        (*(addr as *mut SockaddrIn6)).sin6_port = 0;
        memset(ip6hdr as *mut c_void, 0, size_of::<Ip6Hdr>());
        (*ip6hdr).ip6_flow = htonl(6 << 28); /* Version. */
        (*ip6hdr).ip6_nxt = IPPROTO_FRAGMENT;
        (*ip6hdr).ip6_hops = 255;
        (*ip6hdr).ip6_src = ADDR6;
        (*ip6hdr).ip6_dst = ADDR6;
        (*fraghdr).ip6f_nxt = IPPROTO_UDP as u8;
        (*fraghdr).ip6f_reserved = 0;
        (*fraghdr).ip6f_ident = htonl(IP_ID);
        IP_ID = IP_ID.wrapping_add(1);
    } else {
        memset(iphdr as *mut c_void, 0, size_of::<Ip>());
        (*iphdr).set_ip_hl(5);
        (*iphdr).set_ip_v(4);
        (*iphdr).ip_tos = 0;
        (*iphdr).ip_id = htons(IP_ID as u16);
        IP_ID = IP_ID.wrapping_add(1);
        (*iphdr).ip_ttl = 0x40;
        (*iphdr).ip_p = IPPROTO_UDP as u8;
        (*iphdr).ip_src.s_addr = htonl(INADDR_LOOPBACK);
        (*iphdr).ip_dst = ADDR4;
        (*iphdr).ip_sum = 0;
    }

    /* Occasionally test in-order fragments. */
    if !CFG_OVERLAP && (rand() % 100 < 15) {
        offset = 0;
        while offset < (UDP_HLEN as c_int + PAYLOAD_LEN) {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
            offset += MAX_FRAG_LEN;
        }
        return;
    }

    /* Occasionally test IPv4 "runs" (see net/ipv4/ip_fragment.c) */
    if !CFG_OVERLAP && (rand() % 100 < 20) && (PAYLOAD_LEN > 9 * MAX_FRAG_LEN) {
        offset = 6 * MAX_FRAG_LEN;
        while offset < (UDP_HLEN as c_int + PAYLOAD_LEN) {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
            offset += MAX_FRAG_LEN;
        }
        offset = 3 * MAX_FRAG_LEN;
        while offset < 6 * MAX_FRAG_LEN {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
            offset += MAX_FRAG_LEN;
        }
        offset = 0;
        while offset < 3 * MAX_FRAG_LEN {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
            offset += MAX_FRAG_LEN;
        }
        return;
    }

    /* Odd fragments. */
    offset = MAX_FRAG_LEN;
    while offset < (UDP_HLEN as c_int + PAYLOAD_LEN) {
        send_fragment(fd_raw, addr, alen, offset, ipv6);
        /* IPv4 ignores duplicates, so randomly send a duplicate. */
        if rand() % 100 == 1 {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
        }
        offset += 2 * MAX_FRAG_LEN;
    }

    if CFG_OVERLAP {
        /* Send an extra random fragment.
         *
         * Duplicates and some fragments completely inside
         * previously sent fragments are dropped/ignored. So
         * random offset and frag_len can result in a dropped
         * fragment instead of a dropped queue/packet. Thus we
         * hard-code offset and frag_len.
         */
        if MAX_FRAG_LEN * 4 < PAYLOAD_LEN || MAX_FRAG_LEN < 16 {
            /* not enough payload for random offset and frag_len. */
            offset = 8;
            frag_len = UDP_HLEN as c_int + MAX_FRAG_LEN;
        } else {
            offset = rand() % (PAYLOAD_LEN / 2);
            frag_len = 2 * MAX_FRAG_LEN + 1 + rand() % 256;
        }
        if ipv6 {
            let fraghdr = IP_FRAME.as_mut_ptr().add(IP6_HLEN) as *mut Ip6Frag;
            /* sendto() returns EINVAL if offset + frag_len is too small. */
            /* In IPv6 if !!(frag_len % 8), the fragment is dropped. */
            frag_len &= !0x7;
            (*fraghdr).ip6f_offlg = htons(((offset / 8) as c_uint | IP6_MF) as u16);
            (*ip6hdr).ip6_plen = htons(frag_len as u16);
            frag_len += IP6_HLEN as c_int;
        } else {
            frag_len += IP4_HLEN as c_int;
            (*iphdr).ip_off = htons(((offset / 8) as c_uint | IP4_MF) as u16);
            (*iphdr).ip_len = htons(frag_len as u16);
        }
        res = sendto(
            fd_raw,
            IP_FRAME.as_ptr() as *const c_void,
            frag_len as usize,
            0,
            addr,
            alen,
        ) as c_int;
        if res < 0 && errno != EPERM {
            error(1, errno, c"sendto overlap: %d".as_ptr(), frag_len);
        }
        if res >= 0 && res != frag_len {
            error(1, 0, c"sendto overlap: %d vs %d".as_ptr(), res, frag_len);
        }
        FRAG_COUNTER += 1;
    }

    /* Event fragments. */
    offset = 0;
    while offset < (UDP_HLEN as c_int + PAYLOAD_LEN) {
        send_fragment(fd_raw, addr, alen, offset, ipv6);
        /* IPv4 ignores duplicates, so randomly send a duplicate. */
        if rand() % 100 == 1 {
            send_fragment(fd_raw, addr, alen, offset, ipv6);
        }
        offset += 2 * MAX_FRAG_LEN;
    }
}

unsafe fn run_test(addr: *mut Sockaddr, alen: SocklenT, ipv6: bool) {
    let fd_tx_raw: c_int;
    let fd_rx_udp: c_int;
    /* Frag queue timeout is set to one second in the calling script;
     * socket timeout should be just a bit longer to avoid tests interfering
     * with each other.
     */
    let tv = Timeval {
        tv_sec: 1,
        tv_usec: 10,
    };
    let mut idx: c_int;
    let min_frag_len: c_int = 8;

    /* Initialize the payload. */
    idx = 0;
    while idx < MSG_LEN_MAX as c_int {
        UDP_PAYLOAD[idx as usize] = (idx % 256) as u8;
        idx += 1;
    }

    /* Open sockets. */
    fd_tx_raw = socket((*addr).sa_family as c_int, SOCK_RAW, IPPROTO_RAW);
    if fd_tx_raw == -1 {
        error(1, errno, c"socket tx_raw".as_ptr());
    }

    fd_rx_udp = socket((*addr).sa_family as c_int, SOCK_DGRAM, 0);
    if fd_rx_udp == -1 {
        error(1, errno, c"socket rx_udp".as_ptr());
    }
    if bind(fd_rx_udp, addr, alen) != 0 {
        error(1, errno, c"bind".as_ptr());
    }
    /* Fail fast. */
    if setsockopt(
        fd_rx_udp,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const Timeval as *const c_void,
        size_of::<Timeval>() as SocklenT,
    ) != 0
    {
        error(1, errno, c"setsockopt rcv timeout".as_ptr());
    }

    PAYLOAD_LEN = min_frag_len;
    while PAYLOAD_LEN < MSG_LEN_MAX as c_int {
        if CFG_VERBOSE {
            printf(c"payload_len: %d\n".as_ptr(), PAYLOAD_LEN);
        }

        if CFG_OVERLAP {
            /* With overlaps, one send/receive pair below takes
             * at least one second (== timeout) to run, so there
             * is not enough test time to run a nested loop:
             * the full overlap test takes 20-30 seconds.
             */
            MAX_FRAG_LEN = min_frag_len + rand() % (1500 - FRAG_HLEN as c_int - min_frag_len);
            send_udp_frags(fd_tx_raw, addr, alen, ipv6);
            recv_validate_udp(fd_rx_udp);
        } else {
            /* Without overlaps, each packet reassembly (== one
             * send/receive pair below) takes very little time to
             * run, so we can easily afford more thourough testing
             * with a nested loop: the full non-overlap test takes
             * less than one second).
             */
            MAX_FRAG_LEN = min_frag_len;
            loop {
                send_udp_frags(fd_tx_raw, addr, alen, ipv6);
                recv_validate_udp(fd_rx_udp);
                MAX_FRAG_LEN += 8 * (rand() % 8);
                if !(MAX_FRAG_LEN < (1500 - FRAG_HLEN as c_int) && MAX_FRAG_LEN <= PAYLOAD_LEN) {
                    break;
                }
            }
        }

        PAYLOAD_LEN += rand() % 4096;
    }

    /* Cleanup. */
    if close(fd_tx_raw) != 0 {
        error(1, errno, c"close tx_raw".as_ptr());
    }
    if close(fd_rx_udp) != 0 {
        error(1, errno, c"close rx_udp".as_ptr());
    }

    if CFG_VERBOSE {
        printf(
            c"processed %d messages, %d fragments\n".as_ptr(),
            MSG_COUNTER,
            FRAG_COUNTER,
        );
    }

    fprintf(stderr, c"PASS\n".as_ptr());
}

unsafe fn run_test_v4() {
    let mut addr = SockaddrIn {
        sin_family: 0,
        sin_port: 0,
        sin_addr: InAddr { s_addr: 0 },
        sin_zero: [0; 8],
    };

    addr.sin_family = AF_INET as u16;
    addr.sin_port = htons(CFG_PORT);
    addr.sin_addr = ADDR4;

    run_test(
        &mut addr as *mut SockaddrIn as *mut Sockaddr,
        size_of::<SockaddrIn>() as SocklenT,
        false, /* !ipv6 */
    );
}

unsafe fn run_test_v6() {
    let mut addr = SockaddrIn6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: In6Addr { s6_addr: [0; 16] },
        sin6_scope_id: 0,
    };

    addr.sin6_family = AF_INET6 as u16;
    addr.sin6_port = htons(CFG_PORT);
    addr.sin6_addr = ADDR6;

    run_test(
        &mut addr as *mut SockaddrIn6 as *mut Sockaddr,
        size_of::<SockaddrIn6>() as SocklenT,
        true, /* ipv6 */
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, c"46opv".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                CFG_DO_IPV4 = true;
            }
            '6' => {
                CFG_DO_IPV6 = true;
            }
            'o' => {
                CFG_OVERLAP = true;
            }
            'p' => {
                CFG_PERMISSIVE = true;
            }
            'v' => {
                CFG_VERBOSE = true;
            }
            _ => {
                error(1, 0, c"%s: parse error".as_ptr(), *argv);
            }
        }
    }
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(ptr::null_mut());

        parse_opts((args.len() - 1) as c_int, args.as_mut_ptr());
        SEED = time(ptr::null_mut()) as c_uint;
        srand(SEED);
        /* Print the seed to track/reproduce potential failures. */
        printf(c"seed = %d\n".as_ptr(), SEED);

        if CFG_DO_IPV4 {
            run_test_v4();
        }
        if CFG_DO_IPV6 {
            run_test_v6();
        }

        for arg in args {
            if !arg.is_null() {
                let _ = std::ffi::CString::from_raw(arg);
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
