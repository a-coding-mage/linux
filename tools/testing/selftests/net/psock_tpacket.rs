// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013 Red Hat, Inc.
 * Author: Daniel Borkmann <dborkman@redhat.com>
 *         Chetan Loke <loke.chetan@gmail.com> (TPACKET_V3 usage example)
 *
 * A basic test of packet socket's TPACKET_V1/TPACKET_V2/TPACKET_V3 behavior.
 *
 * Control:
 *   Test the setup of the TPACKET socket with different patterns that are
 *   known to fail (TODO) resp. succeed (OK).
 *
 * Datapath:
 *   Open a pair of packet sockets and send resp. receive an a priori known
 *   packet pattern across the sockets and check if it was received resp.
 *   sent correctly. Fanout in combination with RX_RING is currently not
 *   tested here.
 *
 *   The test currently runs for
 *   - TPACKET_V1: RX_RING, TX_RING
 *   - TPACKET_V2: RX_RING, TX_RING
 *   - TPACKET_V3: RX_RING
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{fence, Ordering};

const NUM_PACKETS: c_uint = 100;

const PF_PACKET: c_int = 17;
const SOCK_RAW: c_int = 3;
const SOL_PACKET: c_int = 263;
const PACKET_VERSION: c_int = 10;
const PACKET_RX_RING: c_int = 5;
const PACKET_TX_RING: c_int = 13;
const PACKET_LOSS: c_int = 14;
const TPACKET_V1: c_int = 0;
const TPACKET_V2: c_int = 1;
const TPACKET_V3: c_int = 2;
const TP_STATUS_KERNEL: c_ulong = 0;
const TP_STATUS_USER: c_ulong = 1;
const TP_STATUS_SEND_REQUEST: c_ulong = 1;
const TP_STATUS_SENDING: c_ulong = 2;
const TP_FT_REQ_FILL_RXHASH: c_uint = 1;
const TPACKET_ALIGNMENT: c_uint = 16;
const ETH_ALEN: usize = 6;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_ALL: c_int = 0x0003;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_LOCKED: c_int = 0x2000;
const MAP_POPULATE: c_int = 0x8000;
const O_RDONLY: c_int = 0;
const POLLIN: i16 = 0x0001;
const POLLOUT: i16 = 0x0004;
const POLLERR: i16 = 0x0008;
const KSFT_SKIP: c_int = 4;

const DATA_LEN: usize = 32;
const DATA_CHAR: c_int = 0x42;
const PORT_BASE: c_int = 8000;

const fn tpacket_align(x: usize) -> usize {
    (x + TPACKET_ALIGNMENT as usize - 1) & !(TPACKET_ALIGNMENT as usize - 1)
}

const fn align_8(x: usize) -> usize {
    (x + 8 - 1) & !(8 - 1)
}

const TPACKET_HDRLEN: usize = tpacket_align(size_of::<tpacket_hdr>()) + size_of::<sockaddr_ll>();
const TPACKET2_HDRLEN: usize = tpacket_align(size_of::<tpacket2_hdr>()) + size_of::<sockaddr_ll>();
const TPACKET3_HDRLEN: usize = tpacket_align(size_of::<tpacket3_hdr>()) + size_of::<sockaddr_ll>();

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

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
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
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

    unsafe fn set_version(&mut self, version: u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((version & 0x0f) << 4);
    }
}

#[repr(C)]
struct tpacket_req {
    tp_block_size: c_uint,
    tp_block_nr: c_uint,
    tp_frame_size: c_uint,
    tp_frame_nr: c_uint,
}

#[repr(C)]
struct tpacket_req3 {
    tp_block_size: c_uint,
    tp_block_nr: c_uint,
    tp_frame_size: c_uint,
    tp_frame_nr: c_uint,
    tp_retire_blk_tov: c_uint,
    tp_sizeof_priv: c_uint,
    tp_feature_req_word: c_uint,
}

#[repr(C)]
struct tpacket_hdr {
    tp_status: c_ulong,
    tp_len: c_uint,
    tp_snaplen: c_uint,
    tp_mac: u16,
    tp_net: u16,
    tp_sec: c_uint,
    tp_usec: c_uint,
}

#[repr(C)]
struct tpacket2_hdr {
    tp_status: c_uint,
    tp_len: c_uint,
    tp_snaplen: c_uint,
    tp_mac: u16,
    tp_net: u16,
    tp_sec: c_uint,
    tp_nsec: c_uint,
    tp_vlan_tci: u16,
    tp_vlan_tpid: u16,
    tp_padding: [u8; 4],
}

#[repr(C)]
struct tpacket3_hdr {
    tp_next_offset: c_uint,
    tp_sec: c_uint,
    tp_nsec: c_uint,
    tp_snaplen: c_uint,
    tp_len: c_uint,
    tp_status: c_uint,
    tp_mac: u16,
    tp_net: u16,
    hv1: [u8; 8],
    tp_padding: [u8; 8],
}

#[repr(C)]
struct tpacket_hdr_v1 {
    block_status: c_uint,
    num_pkts: c_uint,
    offset_to_first_pkt: c_uint,
    blk_len: c_uint,
    seq_num: u64,
    ts_first_pkt: [u8; 16],
    ts_last_pkt: [u8; 16],
}

#[repr(C)]
union RingReq {
    req: core::mem::ManuallyDrop<tpacket_req>,
    req3: core::mem::ManuallyDrop<tpacket_req3>,
}

#[repr(C)]
struct ring {
    rd: *mut iovec,
    mm_space: *mut u8,
    mm_len: usize,
    rd_len: usize,
    ll: sockaddr_ll,
    walk: Option<unsafe fn(c_int, *mut ring)>,
    type_: c_int,
    rd_num: c_int,
    flen: c_int,
    version: c_int,
    u: RingReq,
}

#[repr(C)]
struct block_desc {
    version: u32,
    offset_to_priv: u32,
    h1: tpacket_hdr_v1,
}

#[repr(C)]
struct frame_map_v1 {
    tp_h: tpacket_hdr,
    s_ll: sockaddr_ll,
}

#[repr(C)]
struct frame_map_v2 {
    tp_h: tpacket2_hdr,
    s_ll: sockaddr_ll,
}

#[repr(C)]
union frame_map {
    v1: *mut frame_map_v1,
    v2: *mut frame_map_v2,
    raw: *mut c_void,
}

static mut TOTAL_PACKETS: c_uint = 0;
static mut TOTAL_BYTES: c_uint = 0;
static mut __v3_prev_block_seq_num: u64 = 0;

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn rand() -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: usize,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut u32,
    ) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn isspace(c: c_int) -> c_int;

    static mut stderr: *mut c_void;

    fn pair_udp_open(fds: *mut c_int, port: c_int);
    fn pair_udp_send(fds: *mut c_int, num: c_uint);
    fn pair_udp_close(fds: *mut c_int);
    fn pair_udp_setfilter(sock: c_int);
}

unsafe fn bug_on(cond: bool) {
    if cond {
        core::intrinsics::abort();
    }
}

unsafe fn pfsocket(ver: c_int) -> c_int {
    let mut ret: c_int;
    let sock = socket(PF_PACKET, SOCK_RAW, 0);
    if sock == -1 {
        perror(c"socket".as_ptr());
        exit(1);
    }

    ret = setsockopt(
        sock,
        SOL_PACKET,
        PACKET_VERSION,
        &ver as *const c_int as *const c_void,
        size_of::<c_int>() as u32,
    );
    if ret == -1 {
        perror(c"setsockopt".as_ptr());
        exit(1);
    }

    sock
}

unsafe fn status_bar_update() {
    if TOTAL_PACKETS % 10 == 0 {
        fprintf(stderr, c".".as_ptr());
        fflush(stderr);
    }
}

unsafe fn test_payload(pay: *mut c_void, len: usize) {
    let eth = pay as *mut ethhdr;

    if len < size_of::<ethhdr>() {
        fprintf(stderr, c"test_payload: packet too small: %zu bytes!\n".as_ptr(), len);
        exit(1);
    }

    if (*eth).h_proto != htons(ETH_P_IP as u16) {
        fprintf(
            stderr,
            c"test_payload: wrong ethernet type: 0x%x!\n".as_ptr(),
            ntohs((*eth).h_proto) as c_int,
        );
        exit(1);
    }
}

unsafe fn create_payload(pay: *mut c_void, len: *mut usize) {
    let mut i: c_int;
    let eth = pay as *mut ethhdr;
    let ip = (pay as *mut u8).add(size_of::<ethhdr>()) as *mut iphdr;

    /*
     * Lets create some broken crap, that still passes
     * our BPF filter.
     */

    *len = DATA_LEN + 42;

    memset(pay, 0xff, ETH_ALEN * 2);
    (*eth).h_proto = htons(ETH_P_IP as u16);

    i = 0;
    while (i as usize) < size_of::<iphdr>() {
        *((pay as *mut u8).add(i as usize + size_of::<ethhdr>())) = rand() as u8;
        i += 1;
    }

    (*ip).set_ihl(5);
    (*ip).set_version(4);
    (*ip).protocol = 0x11;
    (*ip).frag_off = 0;
    (*ip).ttl = 64;
    (*ip).tot_len = htons((*len as u16).wrapping_sub(size_of::<ethhdr>() as u16));

    (*ip).saddr = htonl(INADDR_LOOPBACK);
    (*ip).daddr = htonl(INADDR_LOOPBACK);

    memset(
        (pay as *mut u8).add(size_of::<ethhdr>() + size_of::<iphdr>()) as *mut c_void,
        DATA_CHAR,
        DATA_LEN,
    );
}

unsafe fn __v1_rx_kernel_ready(hdr: *mut tpacket_hdr) -> c_int {
    (((*hdr).tp_status & TP_STATUS_USER) == TP_STATUS_USER) as c_int
}

unsafe fn __v1_rx_user_ready(hdr: *mut tpacket_hdr) {
    (*hdr).tp_status = TP_STATUS_KERNEL;
    fence(Ordering::SeqCst);
}

unsafe fn __v2_rx_kernel_ready(hdr: *mut tpacket2_hdr) -> c_int {
    (((*hdr).tp_status as c_ulong & TP_STATUS_USER) == TP_STATUS_USER) as c_int
}

unsafe fn __v2_rx_user_ready(hdr: *mut tpacket2_hdr) {
    (*hdr).tp_status = TP_STATUS_KERNEL as c_uint;
    fence(Ordering::SeqCst);
}

unsafe fn __v1_v2_rx_kernel_ready(base: *mut c_void, version: c_int) -> c_int {
    match version {
        TPACKET_V1 => __v1_rx_kernel_ready(base as *mut tpacket_hdr),
        TPACKET_V2 => __v2_rx_kernel_ready(base as *mut tpacket2_hdr),
        _ => {
            bug_on(true);
            0
        }
    }
}

unsafe fn __v1_v2_rx_user_ready(base: *mut c_void, version: c_int) {
    match version {
        TPACKET_V1 => __v1_rx_user_ready(base as *mut tpacket_hdr),
        TPACKET_V2 => __v2_rx_user_ready(base as *mut tpacket2_hdr),
        _ => {}
    }
}

unsafe fn walk_v1_v2_rx(sock: c_int, ring: *mut ring) {
    let mut pfd: pollfd = core::mem::zeroed();
    let mut udp_sock: [c_int; 2] = [0; 2];
    let mut ppd: frame_map = core::mem::zeroed();
    let mut frame_num: c_uint = 0;

    bug_on((*ring).type_ != PACKET_RX_RING);

    pair_udp_open(udp_sock.as_mut_ptr(), PORT_BASE);

    pfd.fd = sock;
    pfd.events = POLLIN | POLLERR;
    pfd.revents = 0;

    pair_udp_send(udp_sock.as_mut_ptr(), NUM_PACKETS);

    while TOTAL_PACKETS < NUM_PACKETS * 2 {
        while __v1_v2_rx_kernel_ready((*(*ring).rd.add(frame_num as usize)).iov_base, (*ring).version) != 0 {
            ppd.raw = (*(*ring).rd.add(frame_num as usize)).iov_base;

            match (*ring).version {
                TPACKET_V1 => {
                    test_payload(
                        (ppd.raw as *mut u8).add((*ppd.v1).tp_h.tp_mac as usize) as *mut c_void,
                        (*ppd.v1).tp_h.tp_snaplen as usize,
                    );
                    TOTAL_BYTES = TOTAL_BYTES.wrapping_add((*ppd.v1).tp_h.tp_snaplen);
                }
                TPACKET_V2 => {
                    test_payload(
                        (ppd.raw as *mut u8).add((*ppd.v2).tp_h.tp_mac as usize) as *mut c_void,
                        (*ppd.v2).tp_h.tp_snaplen as usize,
                    );
                    TOTAL_BYTES = TOTAL_BYTES.wrapping_add((*ppd.v2).tp_h.tp_snaplen);
                }
                _ => {}
            }

            status_bar_update();
            TOTAL_PACKETS = TOTAL_PACKETS.wrapping_add(1);

            __v1_v2_rx_user_ready(ppd.raw, (*ring).version);

            frame_num = (frame_num + 1) % (*ring).rd_num as c_uint;
        }

        poll(&mut pfd, 1, 1);
    }

    pair_udp_close(udp_sock.as_mut_ptr());

    if TOTAL_PACKETS != 2 * NUM_PACKETS {
        fprintf(
            stderr,
            c"walk_v%d_rx: received %u out of %u pkts\n".as_ptr(),
            (*ring).version,
            TOTAL_PACKETS,
            NUM_PACKETS,
        );
        exit(1);
    }

    fprintf(stderr, c" %u pkts (%u bytes)".as_ptr(), NUM_PACKETS, TOTAL_BYTES >> 1);
}

unsafe fn __v1_tx_kernel_ready(hdr: *mut tpacket_hdr) -> c_int {
    (!((*hdr).tp_status & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING) != 0)) as c_int
}

unsafe fn __v1_tx_user_ready(hdr: *mut tpacket_hdr) {
    (*hdr).tp_status = TP_STATUS_SEND_REQUEST;
    fence(Ordering::SeqCst);
}

unsafe fn __v2_tx_kernel_ready(hdr: *mut tpacket2_hdr) -> c_int {
    (!(((*hdr).tp_status as c_ulong) & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING) != 0)) as c_int
}

unsafe fn __v2_tx_user_ready(hdr: *mut tpacket2_hdr) {
    (*hdr).tp_status = TP_STATUS_SEND_REQUEST as c_uint;
    fence(Ordering::SeqCst);
}

unsafe fn __v3_tx_kernel_ready(hdr: *mut tpacket3_hdr) -> c_int {
    (!(((*hdr).tp_status as c_ulong) & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING) != 0)) as c_int
}

unsafe fn __v3_tx_user_ready(hdr: *mut tpacket3_hdr) {
    (*hdr).tp_status = TP_STATUS_SEND_REQUEST as c_uint;
    fence(Ordering::SeqCst);
}

unsafe fn __tx_kernel_ready(base: *mut c_void, version: c_int) -> c_int {
    match version {
        TPACKET_V1 => __v1_tx_kernel_ready(base as *mut tpacket_hdr),
        TPACKET_V2 => __v2_tx_kernel_ready(base as *mut tpacket2_hdr),
        TPACKET_V3 => __v3_tx_kernel_ready(base as *mut tpacket3_hdr),
        _ => {
            bug_on(true);
            0
        }
    }
}

unsafe fn __tx_user_ready(base: *mut c_void, version: c_int) {
    match version {
        TPACKET_V1 => __v1_tx_user_ready(base as *mut tpacket_hdr),
        TPACKET_V2 => __v2_tx_user_ready(base as *mut tpacket2_hdr),
        TPACKET_V3 => __v3_tx_user_ready(base as *mut tpacket3_hdr),
        _ => {}
    }
}

unsafe fn __v1_v2_set_packet_loss_discard(sock: c_int) {
    let mut discard: c_int = 1;

    let ret = setsockopt(
        sock,
        SOL_PACKET,
        PACKET_LOSS,
        &mut discard as *mut c_int as *mut c_void,
        size_of::<c_int>() as u32,
    );
    if ret == -1 {
        perror(c"setsockopt".as_ptr());
        exit(1);
    }
}

unsafe fn get_next_frame(ring: *mut ring, n: c_int) -> *mut c_void {
    let f0 = (*(*ring).rd.add(0)).iov_base as *mut u8;

    match (*ring).version {
        TPACKET_V1 | TPACKET_V2 => (*(*ring).rd.add(n as usize)).iov_base,
        TPACKET_V3 => f0.add((n as c_uint * (*(*ring).u.req3).tp_frame_size) as usize) as *mut c_void,
        _ => {
            bug_on(true);
            ptr::null_mut()
        }
    }
}

unsafe fn walk_tx(sock: c_int, ring: *mut ring) {
    let mut pfd: pollfd = core::mem::zeroed();
    let mut ret: c_int;
    let mut packet_len: usize = 0;
    let mut ppd: frame_map = core::mem::zeroed();
    let mut packet: [c_char; 1024] = [0; 1024];
    let mut frame_num: c_uint = 0;
    let mut got: c_uint = 0;
    let mut ll: sockaddr_ll = core::mem::zeroed();
    let nframes: c_int;

    ll.sll_family = PF_PACKET as u16;
    ll.sll_halen = ETH_ALEN as u8;

    /*
     * TPACKET_V{1,2} sets up the ring->rd* related variables based
     * on frames (e.g., rd_num is tp_frame_nr) whereas V3 sets these
     * up based on blocks (e.g, rd_num is  tp_block_nr)
     */
    if (*ring).version <= TPACKET_V2 {
        nframes = (*ring).rd_num;
    } else {
        nframes = (*(*ring).u.req3).tp_frame_nr as c_int;
    }

    bug_on((*ring).type_ != PACKET_TX_RING);
    bug_on(nframes < NUM_PACKETS as c_int);

    let rcv_sock = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL as u16) as c_int);
    if rcv_sock == -1 {
        perror(c"socket".as_ptr());
        exit(1);
    }

    pair_udp_setfilter(rcv_sock);

    ll.sll_ifindex = if_nametoindex(c"lo".as_ptr()) as c_int;
    ret = bind(
        rcv_sock,
        &ll as *const sockaddr_ll as *const sockaddr,
        size_of::<sockaddr_ll>() as u32,
    );
    if ret == -1 {
        perror(c"bind".as_ptr());
        exit(1);
    }

    pfd.fd = sock;
    pfd.events = POLLOUT | POLLERR;
    pfd.revents = 0;

    TOTAL_PACKETS = NUM_PACKETS;
    create_payload(packet.as_mut_ptr() as *mut c_void, &mut packet_len);

    while TOTAL_PACKETS > 0 {
        let next = get_next_frame(ring, frame_num as c_int);

        while __tx_kernel_ready(next, (*ring).version) != 0 && TOTAL_PACKETS > 0 {
            ppd.raw = next;

            match (*ring).version {
                TPACKET_V1 => {
                    (*ppd.v1).tp_h.tp_snaplen = packet_len as c_uint;
                    (*ppd.v1).tp_h.tp_len = packet_len as c_uint;

                    memcpy(
                        (ppd.raw as *mut u8).add(TPACKET_HDRLEN - size_of::<sockaddr_ll>()) as *mut c_void,
                        packet.as_ptr() as *const c_void,
                        packet_len,
                    );
                    TOTAL_BYTES = TOTAL_BYTES.wrapping_add((*ppd.v1).tp_h.tp_snaplen);
                }
                TPACKET_V2 => {
                    (*ppd.v2).tp_h.tp_snaplen = packet_len as c_uint;
                    (*ppd.v2).tp_h.tp_len = packet_len as c_uint;

                    memcpy(
                        (ppd.raw as *mut u8).add(TPACKET2_HDRLEN - size_of::<sockaddr_ll>()) as *mut c_void,
                        packet.as_ptr() as *const c_void,
                        packet_len,
                    );
                    TOTAL_BYTES = TOTAL_BYTES.wrapping_add((*ppd.v2).tp_h.tp_snaplen);
                }
                TPACKET_V3 => {
                    let tx = next as *mut tpacket3_hdr;

                    (*tx).tp_snaplen = packet_len as c_uint;
                    (*tx).tp_len = packet_len as c_uint;
                    (*tx).tp_next_offset = 0;

                    memcpy(
                        (tx as *mut u8).add(TPACKET3_HDRLEN - size_of::<sockaddr_ll>()) as *mut c_void,
                        packet.as_ptr() as *const c_void,
                        packet_len,
                    );
                    TOTAL_BYTES = TOTAL_BYTES.wrapping_add((*tx).tp_snaplen);
                }
                _ => {}
            }

            status_bar_update();
            TOTAL_PACKETS = TOTAL_PACKETS.wrapping_sub(1);

            __tx_user_ready(next, (*ring).version);

            frame_num = (frame_num + 1) % nframes as c_uint;
        }

        poll(&mut pfd, 1, 1);
    }

    bug_on(TOTAL_PACKETS != 0);

    ret = sendto(sock, ptr::null(), 0, 0, ptr::null(), 0) as c_int;
    if ret == -1 {
        perror(c"sendto".as_ptr());
        exit(1);
    }

    loop {
        let r = recvfrom(
            rcv_sock,
            packet.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 1024]>(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        ret = r as c_int;
        if !(ret > 0 && TOTAL_PACKETS < NUM_PACKETS) {
            break;
        }
        got = got.wrapping_add(ret as c_uint);
        test_payload(packet.as_mut_ptr() as *mut c_void, ret as usize);

        status_bar_update();
        TOTAL_PACKETS = TOTAL_PACKETS.wrapping_add(1);
    }

    close(rcv_sock);

    if TOTAL_PACKETS != NUM_PACKETS {
        fprintf(
            stderr,
            c"walk_v%d_rx: received %u out of %u pkts\n".as_ptr(),
            (*ring).version,
            TOTAL_PACKETS,
            NUM_PACKETS,
        );
        exit(1);
    }

    fprintf(stderr, c" %u pkts (%u bytes)".as_ptr(), NUM_PACKETS, got);
}

unsafe fn walk_v1_v2(sock: c_int, ring: *mut ring) {
    if (*ring).type_ == PACKET_RX_RING {
        walk_v1_v2_rx(sock, ring);
    } else {
        walk_tx(sock, ring);
    }
}

unsafe fn __v3_test_block_seq_num(pbd: *mut block_desc) {
    if __v3_prev_block_seq_num + 1 != (*pbd).h1.seq_num {
        fprintf(
            stderr,
            c"\nprev_block_seq_num:%lu, expected seq:%lu != actual seq:%lu\n".as_ptr(),
            __v3_prev_block_seq_num,
            __v3_prev_block_seq_num + 1,
            (*pbd).h1.seq_num,
        );
        exit(1);
    }

    __v3_prev_block_seq_num = (*pbd).h1.seq_num;
}

unsafe fn __v3_test_block_len(pbd: *mut block_desc, bytes: u32, block_num: c_int) {
    if (*pbd).h1.num_pkts != 0 && bytes != (*pbd).h1.blk_len {
        fprintf(
            stderr,
            c"\nblock:%u with %upackets, expected len:%u != actual len:%u\n".as_ptr(),
            block_num,
            (*pbd).h1.num_pkts,
            bytes,
            (*pbd).h1.blk_len,
        );
        exit(1);
    }
}

unsafe fn __v3_test_block_header(pbd: *mut block_desc, block_num: c_int) {
    if ((*pbd).h1.block_status as c_ulong & TP_STATUS_USER) == 0 {
        fprintf(stderr, c"\nblock %u: not in TP_STATUS_USER\n".as_ptr(), block_num);
        exit(1);
    }

    __v3_test_block_seq_num(pbd);
}

unsafe fn __v3_walk_block(pbd: *mut block_desc, block_num: c_int) {
    let num_pkts = (*pbd).h1.num_pkts as c_int;
    let mut i: c_int;
    let mut bytes: c_ulong = 0;
    let mut bytes_with_padding: c_ulong = align_8(size_of::<block_desc>()) as c_ulong;
    let mut ppd: *mut tpacket3_hdr;

    __v3_test_block_header(pbd, block_num);

    ppd = (pbd as *mut u8).add((*pbd).h1.offset_to_first_pkt as usize) as *mut tpacket3_hdr;

    i = 0;
    while i < num_pkts {
        bytes = bytes.wrapping_add((*ppd).tp_snaplen as c_ulong);

        if (*ppd).tp_next_offset != 0 {
            bytes_with_padding = bytes_with_padding.wrapping_add((*ppd).tp_next_offset as c_ulong);
        } else {
            bytes_with_padding = bytes_with_padding
                .wrapping_add(align_8((*ppd).tp_snaplen as usize + (*ppd).tp_mac as usize) as c_ulong);
        }

        test_payload(
            (ppd as *mut u8).add((*ppd).tp_mac as usize) as *mut c_void,
            (*ppd).tp_snaplen as usize,
        );

        status_bar_update();
        TOTAL_PACKETS = TOTAL_PACKETS.wrapping_add(1);

        ppd = (ppd as *mut u8).add((*ppd).tp_next_offset as usize) as *mut tpacket3_hdr;
        fence(Ordering::SeqCst);
        i += 1;
    }

    __v3_test_block_len(pbd, bytes_with_padding as u32, block_num);
    TOTAL_BYTES = TOTAL_BYTES.wrapping_add(bytes as c_uint);
}

unsafe fn __v3_flush_block(pbd: *mut block_desc) {
    (*pbd).h1.block_status = TP_STATUS_KERNEL as c_uint;
    fence(Ordering::SeqCst);
}

unsafe fn walk_v3_rx(sock: c_int, ring: *mut ring) {
    let mut block_num: c_uint = 0;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut pbd: *mut block_desc;
    let mut udp_sock: [c_int; 2] = [0; 2];

    bug_on((*ring).type_ != PACKET_RX_RING);

    pair_udp_open(udp_sock.as_mut_ptr(), PORT_BASE);

    pfd.fd = sock;
    pfd.events = POLLIN | POLLERR;
    pfd.revents = 0;

    pair_udp_send(udp_sock.as_mut_ptr(), NUM_PACKETS);

    while TOTAL_PACKETS < NUM_PACKETS * 2 {
        pbd = (*(*ring).rd.add(block_num as usize)).iov_base as *mut block_desc;

        while ((*pbd).h1.block_status as c_ulong & TP_STATUS_USER) == 0 {
            poll(&mut pfd, 1, 1);
        }

        __v3_walk_block(pbd, block_num as c_int);
        __v3_flush_block(pbd);

        block_num = (block_num + 1) % (*ring).rd_num as c_uint;
    }

    pair_udp_close(udp_sock.as_mut_ptr());

    if TOTAL_PACKETS != 2 * NUM_PACKETS {
        fprintf(
            stderr,
            c"walk_v3_rx: received %u out of %u pkts\n".as_ptr(),
            TOTAL_PACKETS,
            NUM_PACKETS,
        );
        exit(1);
    }

    fprintf(stderr, c" %u pkts (%u bytes)".as_ptr(), NUM_PACKETS, TOTAL_BYTES >> 1);
}

unsafe fn walk_v3(sock: c_int, ring: *mut ring) {
    if (*ring).type_ == PACKET_RX_RING {
        walk_v3_rx(sock, ring);
    } else {
        walk_tx(sock, ring);
    }
}

unsafe fn __v1_v2_fill(ring: *mut ring, blocks: c_uint) {
    (*(*ring).u.req).tp_block_size = (getpagesize() << 2) as c_uint;
    (*(*ring).u.req).tp_frame_size = TPACKET_ALIGNMENT << 7;
    (*(*ring).u.req).tp_block_nr = blocks;

    (*(*ring).u.req).tp_frame_nr =
        (*(*ring).u.req).tp_block_size / (*(*ring).u.req).tp_frame_size * (*(*ring).u.req).tp_block_nr;

    (*ring).mm_len = ((*(*ring).u.req).tp_block_size * (*(*ring).u.req).tp_block_nr) as usize;
    (*ring).walk = Some(walk_v1_v2);
    (*ring).rd_num = (*(*ring).u.req).tp_frame_nr as c_int;
    (*ring).flen = (*(*ring).u.req).tp_frame_size as c_int;
}

unsafe fn __v3_fill(ring: *mut ring, blocks: c_uint, type_: c_int) {
    if type_ == PACKET_RX_RING {
        (*(*ring).u.req3).tp_retire_blk_tov = 64;
        (*(*ring).u.req3).tp_sizeof_priv = 0;
        (*(*ring).u.req3).tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;
    }
    (*(*ring).u.req3).tp_block_size = (getpagesize() << 2) as c_uint;
    (*(*ring).u.req3).tp_frame_size = TPACKET_ALIGNMENT << 7;
    (*(*ring).u.req3).tp_block_nr = blocks;

    (*(*ring).u.req3).tp_frame_nr =
        (*(*ring).u.req3).tp_block_size / (*(*ring).u.req3).tp_frame_size * (*(*ring).u.req3).tp_block_nr;

    (*ring).mm_len = ((*(*ring).u.req3).tp_block_size * (*(*ring).u.req3).tp_block_nr) as usize;
    (*ring).walk = Some(walk_v3);
    (*ring).rd_num = (*(*ring).u.req3).tp_block_nr as c_int;
    (*ring).flen = (*(*ring).u.req3).tp_block_size as c_int;
}

unsafe fn setup_ring(sock: c_int, ring: *mut ring, version: c_int, type_: c_int) {
    let mut ret: c_int = 0;
    let blocks: c_uint = 256;

    (*ring).type_ = type_;
    (*ring).version = version;

    match version {
        TPACKET_V1 | TPACKET_V2 => {
            if type_ == PACKET_TX_RING {
                __v1_v2_set_packet_loss_discard(sock);
            }
            __v1_v2_fill(ring, blocks);
            ret = setsockopt(
                sock,
                SOL_PACKET,
                type_,
                &mut (*(*ring).u.req) as *mut tpacket_req as *mut c_void,
                size_of::<tpacket_req>() as u32,
            );
        }
        TPACKET_V3 => {
            __v3_fill(ring, blocks, type_);
            ret = setsockopt(
                sock,
                SOL_PACKET,
                type_,
                &mut (*(*ring).u.req3) as *mut tpacket_req3 as *mut c_void,
                size_of::<tpacket_req3>() as u32,
            );
        }
        _ => {}
    }

    if ret == -1 {
        perror(c"setsockopt".as_ptr());
        exit(1);
    }

    (*ring).rd_len = (*ring).rd_num as usize * size_of::<iovec>();
    (*ring).rd = malloc((*ring).rd_len) as *mut iovec;
    if (*ring).rd.is_null() {
        perror(c"malloc".as_ptr());
        exit(1);
    }

    TOTAL_PACKETS = 0;
    TOTAL_BYTES = 0;
}

unsafe fn mmap_ring(sock: c_int, ring: *mut ring) {
    let mut i: c_int;

    (*ring).mm_space = mmap(
        ptr::null_mut(),
        (*ring).mm_len,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_LOCKED | MAP_POPULATE,
        sock,
        0,
    ) as *mut u8;
    if (*ring).mm_space == (-1isize as *mut u8) {
        perror(c"mmap".as_ptr());
        exit(1);
    }

    memset((*ring).rd as *mut c_void, 0, (*ring).rd_len);
    i = 0;
    while i < (*ring).rd_num {
        (*(*ring).rd.add(i as usize)).iov_base = (*ring).mm_space.add((i * (*ring).flen) as usize) as *mut c_void;
        (*(*ring).rd.add(i as usize)).iov_len = (*ring).flen as usize;
        i += 1;
    }
}

unsafe fn bind_ring(sock: c_int, ring: *mut ring) {
    let ret: c_int;

    pair_udp_setfilter(sock);

    (*ring).ll.sll_family = PF_PACKET as u16;
    (*ring).ll.sll_protocol = htons(ETH_P_ALL as u16);
    (*ring).ll.sll_ifindex = if_nametoindex(c"lo".as_ptr()) as c_int;
    (*ring).ll.sll_hatype = 0;
    (*ring).ll.sll_pkttype = 0;
    (*ring).ll.sll_halen = 0;

    ret = bind(
        sock,
        &(*ring).ll as *const sockaddr_ll as *const sockaddr,
        size_of::<sockaddr_ll>() as u32,
    );
    if ret == -1 {
        perror(c"bind".as_ptr());
        exit(1);
    }
}

unsafe fn walk_ring(sock: c_int, ring: *mut ring) {
    ((*ring).walk.unwrap())(sock, ring);
}

unsafe fn unmap_ring(_sock: c_int, ring: *mut ring) {
    munmap((*ring).mm_space as *mut c_void, (*ring).mm_len);
    free((*ring).rd as *mut c_void);
}

unsafe fn test_kernel_bit_width() -> c_int {
    let mut input: [c_char; 512] = [0; 512];
    let mut ptr_: *mut c_char;
    let mut num: c_int = 0;
    let fd: c_int;
    let ret: isize;

    fd = open(c"/proc/kallsyms".as_ptr(), O_RDONLY);
    if fd == -1 {
        perror(c"open".as_ptr());
        exit(1);
    }

    ret = read(fd, input.as_mut_ptr() as *mut c_void, size_of::<[c_char; 512]>());
    if ret <= 0 {
        perror(c"read".as_ptr());
        exit(1);
    }

    close(fd);

    ptr_ = input.as_mut_ptr();
    while isspace(*ptr_ as c_int) == 0 {
        num += 1;
        ptr_ = ptr_.add(1);
    }

    num * 4
}

unsafe fn test_user_bit_width() -> c_int {
    size_of::<c_long>() as c_int * 8
}

static TPACKET_STR: [*const c_char; 3] = [
    c"TPACKET_V1".as_ptr(),
    c"TPACKET_V2".as_ptr(),
    c"TPACKET_V3".as_ptr(),
];

static TYPE_STR: [*const c_char; 14] = [
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    c"PACKET_RX_RING".as_ptr(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    ptr::null(),
    c"PACKET_TX_RING".as_ptr(),
];

unsafe fn test_tpacket(version: c_int, type_: c_int) -> c_int {
    let sock: c_int;
    let mut ring: ring = core::mem::zeroed();

    fprintf(
        stderr,
        c"test: %s with %s ".as_ptr(),
        TPACKET_STR[version as usize],
        TYPE_STR[type_ as usize],
    );
    fflush(stderr);

    if version == TPACKET_V1 && test_kernel_bit_width() != test_user_bit_width() {
        fprintf(
            stderr,
            c"test: skip %s %s since user and kernel space have different bit width\n".as_ptr(),
            TPACKET_STR[version as usize],
            TYPE_STR[type_ as usize],
        );
        return KSFT_SKIP;
    }

    sock = pfsocket(version);
    memset(&mut ring as *mut ring as *mut c_void, 0, size_of::<ring>());
    setup_ring(sock, &mut ring, version, type_);
    mmap_ring(sock, &mut ring);
    bind_ring(sock, &mut ring);
    walk_ring(sock, &mut ring);
    unmap_ring(sock, &mut ring);
    close(sock);

    fprintf(stderr, c"\n".as_ptr());
    0
}

unsafe fn main_0() -> c_int {
    let mut ret: c_int = 0;

    ret |= test_tpacket(TPACKET_V1, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V1, PACKET_TX_RING);

    ret |= test_tpacket(TPACKET_V2, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V2, PACKET_TX_RING);

    ret |= test_tpacket(TPACKET_V3, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V3, PACKET_TX_RING);

    if ret != 0 {
        return 1;
    }

    printf(c"OK. All tests passed\n".as_ptr());
    0
}

fn main() {
    unsafe {
        std::process::exit(main_0());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
