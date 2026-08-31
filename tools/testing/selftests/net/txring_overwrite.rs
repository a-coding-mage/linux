// SPDX-License-Identifier: GPL-2.0

/*
 * Verify that consecutive sends over packet tx_ring are mirrored
 * with their original content intact.
 */

// C dependencies translated from:
// arpa/inet.h, error.h, errno.h, linux/if_packet.h, net/ethernet.h,
// net/if.h, netinet/in.h, netinet/ip.h, netinet/udp.h, sys/mman.h,
// sys/socket.h, unistd.h, stdint.h, stdio.h, stdlib.h, string.h.

use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::mem;
use std::ptr;

type size_t = usize;
type socklen_t = u32;

const PF_PACKET: c_int = 17;
const AF_PACKET: u16 = 17;
const SOCK_RAW: c_int = 3;
const ETH_P_IP: u16 = 0x0800;
const IPPROTO_UDP: u8 = 17;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const SOL_PACKET: c_int = 263;
const PACKET_TX_RING: c_int = 13;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const TP_STATUS_AVAILABLE: u64 = 0;
const TP_STATUS_SEND_REQUEST: u64 = 1;
const EXIT_FAILURE: c_int = 1;

const ETH_ALEN: usize = 6;
const TPACKET_ALIGNMENT: usize = 16;

const fn tpacket_align(x: usize) -> usize {
    (x + TPACKET_ALIGNMENT - 1) & !(TPACKET_ALIGNMENT - 1)
}

const SOCKADDR_LL_SIZE: usize = mem::size_of::<sockaddr_ll>();
const TPACKET_HDRLEN: usize =
    tpacket_align(mem::size_of::<tpacket_hdr>()) + mem::size_of::<sockaddr_ll>();

const eth_off: c_int = (TPACKET_HDRLEN - SOCKADDR_LL_SIZE) as c_int;
const cfg_frame_size: c_int = 1000;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: u16,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
struct tpacket_req {
    tp_block_size: c_uint,
    tp_block_nr: c_uint,
    tp_frame_size: c_uint,
    tp_frame_nr: c_uint,
}

#[repr(C)]
struct timeval {
    tv_sec: isize,
    tv_usec: isize,
}

#[repr(C)]
union tpacket_hdr_variant {
    tp_rxhash: u32,
    tp_vlan_tci: u32,
}

#[repr(C)]
struct tpacket_hdr {
    tp_status: u64,
    tp_len: c_uint,
    tp_snaplen: c_uint,
    tp_mac: u16,
    tp_net: u16,
    tp_sec: c_uint,
    tp_usec: c_uint,
    variant: tpacket_hdr_variant,
}

#[repr(C, packed)]
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

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn getpagesize() -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn printf(format: *const c_char, ... ) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> isize;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn build_packet(buffer: *mut c_void, blen: size_t, payload_char: c_char) {
    let mut off: size_t = 0;

    ptr::write_bytes(buffer, 0, blen);

    let eth = buffer as *mut ethhdr;
    (*eth).h_proto = htons(ETH_P_IP);

    off += mem::size_of::<ethhdr>();
    let iph = (buffer as *mut u8).add(off) as *mut iphdr;
    (*iph).ttl = 8;
    (*iph).set_ihl(5);
    (*iph).set_version(4);
    (*iph).saddr = htonl(INADDR_LOOPBACK);
    (*iph).daddr = htonl(INADDR_LOOPBACK + 1);
    (*iph).protocol = IPPROTO_UDP;
    (*iph).tot_len = htons((blen - off) as u16);
    (*iph).check = 0;

    off += mem::size_of::<iphdr>();
    let udph = (buffer as *mut u8).add(off) as *mut udphdr;
    (*udph).dest = htons(8000);
    (*udph).source = htons(8001);
    (*udph).len = htons((blen - off) as u16);
    (*udph).check = 0;

    off += mem::size_of::<udphdr>();
    ptr::write_bytes((buffer as *mut u8).add(off), payload_char as u8, blen - off);
}

unsafe fn setup_rx() -> c_int {
    let fdr: c_int;

    fdr = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_IP) as c_int);
    if fdr == -1 {
        error(EXIT_FAILURE, errno(), c"socket r".as_ptr());
    }

    fdr
}

unsafe fn setup_tx(ring: *mut *mut c_char) -> c_int {
    let mut laddr: sockaddr_ll = mem::zeroed();
    let mut req: tpacket_req = mem::zeroed();
    let fdt: c_int;

    fdt = socket(PF_PACKET, SOCK_RAW, 0);
    if fdt == -1 {
        error(EXIT_FAILURE, errno(), c"socket t".as_ptr());
    }

    laddr.sll_family = AF_PACKET;
    laddr.sll_protocol = htons(0);
    laddr.sll_ifindex = if_nametoindex(c"lo".as_ptr()) as c_int;
    if laddr.sll_ifindex == 0 {
        error(EXIT_FAILURE, errno(), c"if_nametoindex".as_ptr());
    }

    if bind(
        fdt,
        (&laddr as *const sockaddr_ll).cast::<sockaddr>(),
        mem::size_of_val(&laddr) as socklen_t,
    ) != 0
    {
        error(EXIT_FAILURE, errno(), c"bind fdt".as_ptr());
    }

    req.tp_block_size = getpagesize() as c_uint;
    req.tp_block_nr = 1;
    req.tp_frame_size = getpagesize() as c_uint;
    req.tp_frame_nr = 1;

    if setsockopt(
        fdt,
        SOL_PACKET,
        PACKET_TX_RING,
        (&req as *const tpacket_req).cast::<c_void>(),
        mem::size_of_val(&req) as socklen_t,
    ) != 0
    {
        error(EXIT_FAILURE, errno(), c"setsockopt ring".as_ptr());
    }

    *ring = mmap(
        ptr::null_mut(),
        (req.tp_block_size * req.tp_block_nr) as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fdt,
        0,
    ) as *mut c_char;
    if *ring == (-1isize as *mut c_char) {
        error(EXIT_FAILURE, errno(), c"mmap".as_ptr());
    }

    fdt
}

unsafe fn send_pkt(fdt: c_int, slot: *mut c_void, payload_char: c_char) {
    let header = slot as *mut tpacket_hdr;
    let ret: c_int;

    while (*header).tp_status != TP_STATUS_AVAILABLE {
        usleep(1000);
    }

    build_packet(
        (slot as *mut u8).add(eth_off as usize).cast::<c_void>(),
        cfg_frame_size as size_t,
        payload_char,
    );

    (*header).tp_len = cfg_frame_size as c_uint;
    (*header).tp_status = TP_STATUS_SEND_REQUEST;

    ret = sendto(
        fdt,
        ptr::null(),
        0,
        0,
        ptr::null(),
        0,
    ) as c_int;
    if ret == -1 {
        error(EXIT_FAILURE, errno(), c"kick tx".as_ptr());
    }
}

unsafe fn read_verify_pkt(fdr: c_int, payload_char: c_char) -> c_int {
    let mut buf: [c_char; 100] = [0; 100];
    let ret: c_int;

    ret = read(fdr, buf.as_mut_ptr().cast::<c_void>(), mem::size_of_val(&buf)) as c_int;
    if ret != mem::size_of_val(&buf) as c_int {
        error(EXIT_FAILURE, errno(), c"read".as_ptr());
    }

    if buf[60] != payload_char {
        printf(
            c"wrong pattern: 0x%x != 0x%x\n".as_ptr(),
            buf[60] as c_int,
            payload_char as c_int,
        );
        return 1;
    }

    printf(c"read: %c (0x%x)\n".as_ptr(), buf[60] as c_int, buf[60] as c_int);
    0
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let payload_patterns = *b"ab\0";
    let mut ring: *mut c_char = ptr::null_mut();
    let fdr: c_int;
    let fdt: c_int;
    let mut ret: c_int = 0;

    fdr = setup_rx();
    fdt = setup_tx(&mut ring);

    send_pkt(fdt, ring.cast::<c_void>(), payload_patterns[0] as c_char);
    send_pkt(fdt, ring.cast::<c_void>(), payload_patterns[1] as c_char);

    ret |= read_verify_pkt(fdr, payload_patterns[0] as c_char);
    ret |= read_verify_pkt(fdr, payload_patterns[1] as c_char);

    if close(fdt) != 0 {
        error(EXIT_FAILURE, errno(), c"close t".as_ptr());
    }
    if close(fdr) != 0 {
        error(EXIT_FAILURE, errno(), c"close r".as_ptr());
    }

    ret
}

fn main() {
    unsafe {
        let mut argv: Vec<*mut c_char> = std::env::args()
            .filter_map(|arg| std::ffi::CString::new(arg).ok())
            .map(|arg| arg.into_raw())
            .collect();
        argv.push(ptr::null_mut());
        let argc = (argv.len() - 1) as c_int;
        let ret = c_main(argc, argv.as_mut_ptr());
        for arg in argv.into_iter().take(argc as usize) {
            let _ = CStr::from_ptr(arg).to_owned();
            let _ = std::ffi::CString::from_raw(arg);
        }
        std::process::exit(ret);
    }
}
