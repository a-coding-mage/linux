// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/xdp_metadata.c.
// Dependencies originally supplied by:
// test_progs.h, network_helpers.h, xdp_metadata*.skel.h, xdp_metadata.h, xsk.h,
// bpf/btf.h, linux/errqueue.h, linux/if_link.h, linux/net_tstamp.h,
// netinet/udp.h, sys/mman.h, net/if.h, poll.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const TX_NAME: &[u8] = b"veTX\0";
const RX_NAME: &[u8] = b"veRX\0";

const UDP_PAYLOAD_BYTES: usize = 4;

const UDP_SOURCE_PORT: u16 = 1234;
const AF_XDP_CONSUMER_PORT: u16 = 8080;

const UMEM_NUM: u64 = 16;
const UMEM_FRAME_SIZE: u64 = XSK_UMEM__DEFAULT_FRAME_SIZE as u64;
const UMEM_SIZE: usize = (UMEM_FRAME_SIZE * UMEM_NUM) as usize;
const XDP_FLAGS: u32 = XDP_FLAGS_DRV_MODE;
const QUEUE_ID: u32 = 0;

const TX_ADDR: &[u8] = b"10.0.0.1\0";
const RX_ADDR: &[u8] = b"10.0.0.2\0";
const PREFIX_LEN: &str = "8";
const FAMILY: c_int = AF_INET;
const TX_NETNS_NAME: &[u8] = b"xdp_metadata_tx\0";
const RX_NETNS_NAME: &[u8] = b"xdp_metadata_rx\0";
const TX_MAC: &str = "00:00:00:00:00:01";
const RX_MAC: &str = "00:00:00:00:00:02";

const VLAN_ID: u32 = 59;
const VLAN_PROTO: &str = "802.1Q";
fn vlan_pid() -> u16 {
    htons(ETH_P_8021Q as u16)
}
const TX_NAME_VLAN: &str = "veTX.59";

const XDP_RSS_TYPE_L4: u32 = 1 << 3;
const VLAN_VID_MASK: u32 = 0xfff;

const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_NORESERVE: c_int = 0x4000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MSG_DONTWAIT: c_int = 0x40;
const POLLIN: i16 = 0x001;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_UDP: c_int = 17;
const AF_INET: c_int = 2;
const ETH_ALEN: usize = 6;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_8021Q: u16 = 0x8100;
const XDP_COPY: u32 = 2;
const XDP_FLAGS_DRV_MODE: u32 = 4;
const XSK_RING_PROD__DEFAULT_NUM_DESCS: u32 = 2048;
const XSK_RING_CONS__DEFAULT_NUM_DESCS: u32 = 2048;
const XSK_UMEM__DEFAULT_FRAME_SIZE: u32 = 4096;
const XDP_UMEM_UNALIGNED_CHUNK_FLAG: u32 = 1;
const XDP_UMEM_TX_SW_CSUM: u32 = 1 << 1;
const XDP_UMEM_TX_METADATA_LEN: u32 = 1 << 2;
const XDP_TXMD_FLAGS_TIMESTAMP: u64 = 1;
const XDP_TXMD_FLAGS_CHECKSUM: u64 = 2;
const XDP_TX_METADATA: u32 = 1;
const BPF_F_XDP_DEV_BOUND_ONLY: u32 = 1 << 6;
const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct xsk {
    umem_area: *mut c_void,
    umem: *mut xsk_umem,
    fill: xsk_ring_prod,
    comp: xsk_ring_cons,
    tx: xsk_ring_prod,
    rx: xsk_ring_cons,
    socket: *mut xsk_socket,
}

#[repr(C)]
pub struct xsk_umem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xsk_socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_ring_prod {
    _opaque: [usize; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_ring_cons {
    _opaque: [usize; 4],
}

#[repr(C)]
pub struct xsk_socket_config {
    rx_size: u32,
    tx_size: u32,
    libbpf_flags: u32,
    xdp_flags: u32,
    bind_flags: u16,
}

#[repr(C)]
pub struct xsk_umem_config {
    fill_size: u32,
    comp_size: u32,
    frame_size: u32,
    frame_headroom: u32,
    flags: u32,
    tx_metadata_len: u32,
}

#[repr(C)]
pub struct xsk_tx_metadata_request {
    csum_start: u16,
    csum_offset: u16,
}

#[repr(C)]
pub struct xsk_tx_metadata_completion {
    tx_timestamp: u64,
}

#[repr(C)]
pub struct xsk_tx_metadata {
    flags: u64,
    request: xsk_tx_metadata_request,
    completion: xsk_tx_metadata_completion,
}

#[repr(C)]
pub struct xdp_desc {
    addr: u64,
    len: u32,
    options: u32,
}

#[repr(C)]
pub struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
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
    unsafe fn set_version(&mut self, version: u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | (version << 4);
    }

    unsafe fn version(&self) -> u8 {
        self.ihl_version >> 4
    }

    unsafe fn set_ihl(&mut self, ihl: u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (ihl & 0x0f);
    }
}

#[repr(C)]
pub struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
pub struct xdp_meta {
    rx_timestamp: u64,
    rx_hash: u32,
    rx_hash_type: u32,
    rx_vlan_tci: u32,
    rx_vlan_proto: u16,
}

#[repr(C)]
pub struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in_addr {
    s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
pub struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
pub union bpf_devmap_val_bpf_prog {
    fd: c_int,
    id: u32,
}

#[repr(C)]
pub struct bpf_devmap_val {
    ifindex: u32,
    bpf_prog: bpf_devmap_val_bpf_prog,
}

#[repr(C)]
pub struct xdp_metadata {
    obj: *mut bpf_object,
    progs: *mut xdp_metadata_progs,
    maps: *mut xdp_metadata_maps,
}

#[repr(C)]
pub struct xdp_metadata_progs {
    rx: *mut bpf_program,
}

#[repr(C)]
pub struct xdp_metadata_maps {
    xsk: *mut bpf_map,
}

#[repr(C)]
pub struct xdp_metadata2 {
    obj: *mut bpf_object,
    bss: *mut xdp_metadata2_bss,
}

#[repr(C)]
pub struct xdp_metadata2_bss {
    called: c_int,
}

unsafe extern "C" {
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
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
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_addr(cp: *const c_char) -> u32;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;

    fn xsk_umem__create(
        umem: *mut *mut xsk_umem,
        umem_area: *mut c_void,
        size: u64,
        fill: *mut xsk_ring_prod,
        comp: *mut xsk_ring_cons,
        config: *const xsk_umem_config,
    ) -> c_int;
    fn xsk_umem__delete(umem: *mut xsk_umem);
    fn xsk_socket__create(
        xsk: *mut *mut xsk_socket,
        ifindex: c_uint,
        queue_id: u32,
        umem: *mut xsk_umem,
        rx: *mut xsk_ring_cons,
        tx: *mut xsk_ring_prod,
        config: *const xsk_socket_config,
    ) -> c_int;
    fn xsk_socket__delete(xsk: *mut xsk_socket);
    fn xsk_socket__fd(xsk: *mut xsk_socket) -> c_int;
    fn xsk_ring_prod__reserve(prod: *mut xsk_ring_prod, nb: u32, idx: *mut u32) -> c_int;
    fn xsk_ring_prod__submit(prod: *mut xsk_ring_prod, nb: u32);
    fn xsk_ring_prod__fill_addr(prod: *mut xsk_ring_prod, idx: u32) -> *mut u64;
    fn xsk_ring_prod__tx_desc(prod: *mut xsk_ring_prod, idx: u32) -> *mut xdp_desc;
    fn xsk_ring_cons__peek(cons: *mut xsk_ring_cons, nb: u32, idx: *mut u32) -> c_int;
    fn xsk_ring_cons__release(cons: *mut xsk_ring_cons, nb: u32);
    fn xsk_ring_cons__comp_addr(cons: *mut xsk_ring_cons, idx: u32) -> *mut u64;
    fn xsk_ring_cons__rx_desc(cons: *mut xsk_ring_cons, idx: u32) -> *const xdp_desc;
    fn xsk_umem__get_data(umem_area: *mut c_void, addr: u64) -> *mut c_void;
    fn xsk_umem__extract_addr(addr: u64) -> u64;
    fn xsk_umem__add_offset_to_addr(addr: u64) -> u64;

    fn build_ip_csum(iph: *mut iphdr) -> u16;
    fn csum_tcpudp_magic(saddr: u32, daddr: u32, len: u32, proto: u8, sum: u32) -> u16;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn SYS(label: *const c_char, cmd: *const c_char);
    fn SYS_NOFAIL(cmd: *const c_char);

    fn ASSERT_NEQ<T: Copy + PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: Copy + PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_ERR(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE<T: Copy + PartialOrd>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GT<T: Copy + PartialOrd>(actual: T, expected: T, name: *const c_char) -> bool;

    fn xdp_metadata__open() -> *mut xdp_metadata;
    fn xdp_metadata__load(obj: *mut xdp_metadata) -> c_int;
    fn xdp_metadata__destroy(obj: *mut xdp_metadata);
    fn xdp_metadata2__open() -> *mut xdp_metadata2;
    fn xdp_metadata2__load(obj: *mut xdp_metadata2) -> c_int;
    fn xdp_metadata2__attach(obj: *mut xdp_metadata2) -> c_int;
    fn xdp_metadata2__destroy(obj: *mut xdp_metadata2);
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__set_ifindex(prog: *mut bpf_program, ifindex: c_uint);
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: u32);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        target_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *const c_void,
        value_sz: usize,
        flags: u64,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_xdp_attach(ifindex: c_uint, prog_fd: c_int, flags: u32, opts: *const c_void) -> c_int;
}

unsafe fn open_xsk(ifindex: c_int, xsk: *mut xsk) -> c_int {
    let mmap_flags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;
    let socket_config = xsk_socket_config {
        rx_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        tx_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        libbpf_flags: 0,
        xdp_flags: 0,
        bind_flags: XDP_COPY as u16,
    };
    let umem_config = xsk_umem_config {
        fill_size: XSK_RING_PROD__DEFAULT_NUM_DESCS,
        comp_size: XSK_RING_CONS__DEFAULT_NUM_DESCS,
        frame_size: XSK_UMEM__DEFAULT_FRAME_SIZE,
        frame_headroom: 0,
        flags: XDP_UMEM_UNALIGNED_CHUNK_FLAG | XDP_UMEM_TX_SW_CSUM | XDP_UMEM_TX_METADATA_LEN,
        tx_metadata_len: size_of::<xsk_tx_metadata>() as u32,
    };
    let mut idx: u32 = 0;
    let mut addr: u64;
    let mut ret: c_int;
    let mut i: c_int;

    (*xsk).umem_area = mmap(ptr::null_mut(), UMEM_SIZE, PROT_READ | PROT_WRITE, mmap_flags, -1, 0);
    if !ASSERT_NEQ((*xsk).umem_area, (-1isize) as *mut c_void, c"mmap".as_ptr()) {
        return -1;
    }

    ret = xsk_umem__create(
        &mut (*xsk).umem,
        (*xsk).umem_area,
        UMEM_SIZE as u64,
        &mut (*xsk).fill,
        &mut (*xsk).comp,
        &umem_config,
    );
    if !ASSERT_OK(ret, c"xsk_umem__create".as_ptr()) {
        return ret;
    }

    ret = xsk_socket__create(
        &mut (*xsk).socket,
        ifindex as c_uint,
        QUEUE_ID,
        (*xsk).umem,
        &mut (*xsk).rx,
        &mut (*xsk).tx,
        &socket_config,
    );
    if !ASSERT_OK(ret, c"xsk_socket__create".as_ptr()) {
        return ret;
    }

    /* First half of umem is for TX. This way address matches 1-to-1
     * to the completion queue index.
     */
    i = 0;
    while i < (UMEM_NUM / 2) as c_int {
        addr = i as u64 * UMEM_FRAME_SIZE;
        printf(c"%p: tx_desc[%d] -> %lx\n".as_ptr(), xsk, i, addr);
        i += 1;
    }

    /* Second half of umem is for RX. */
    ret = xsk_ring_prod__reserve(&mut (*xsk).fill, (UMEM_NUM / 2) as u32, &mut idx);
    if !ASSERT_EQ((UMEM_NUM / 2) as c_int, ret, c"xsk_ring_prod__reserve".as_ptr()) {
        return ret;
    }
    if !ASSERT_EQ(idx, 0, c"fill idx != 0".as_ptr()) {
        return -1;
    }

    i = 0;
    while i < (UMEM_NUM / 2) as c_int {
        addr = (UMEM_NUM / 2 + i as u64) * UMEM_FRAME_SIZE;
        printf(c"%p: rx_desc[%d] -> %lx\n".as_ptr(), xsk, i, addr);
        *xsk_ring_prod__fill_addr(&mut (*xsk).fill, i as u32) = addr;
        i += 1;
    }
    xsk_ring_prod__submit(&mut (*xsk).fill, ret as u32);

    0
}

unsafe fn close_xsk(xsk: *mut xsk) {
    if !(*xsk).socket.is_null() {
        xsk_socket__delete((*xsk).socket);
    }
    if !(*xsk).umem.is_null() {
        xsk_umem__delete((*xsk).umem);
    }
    munmap((*xsk).umem_area, UMEM_SIZE);
}

unsafe fn generate_packet(xsk: *mut xsk, dst_port: u16) -> c_int {
    let mut meta: *mut xsk_tx_metadata;
    let tx_desc: *mut xdp_desc;
    let mut udph: *mut udphdr;
    let mut eth: *mut ethhdr;
    let mut iph: *mut iphdr;
    let data: *mut c_void;
    let mut idx: u32 = 0;
    let mut ret: c_int;

    ret = xsk_ring_prod__reserve(&mut (*xsk).tx, 1, &mut idx);
    if !ASSERT_EQ(ret, 1, c"xsk_ring_prod__reserve".as_ptr()) {
        return -1;
    }

    tx_desc = xsk_ring_prod__tx_desc(&mut (*xsk).tx, idx);
    (*tx_desc).addr =
        (idx as u64 % (UMEM_NUM / 2)) * UMEM_FRAME_SIZE + size_of::<xsk_tx_metadata>() as u64;
    printf(c"%p: tx_desc[%u]->addr=%llx\n".as_ptr(), xsk, idx, (*tx_desc).addr);
    data = xsk_umem__get_data((*xsk).umem_area, (*tx_desc).addr);

    meta = (data as *mut u8).sub(size_of::<xsk_tx_metadata>()) as *mut xsk_tx_metadata;
    memset(meta as *mut c_void, 0, size_of::<xsk_tx_metadata>());
    (*meta).flags = XDP_TXMD_FLAGS_TIMESTAMP;

    eth = data as *mut ethhdr;
    iph = eth.add(1) as *mut iphdr;
    udph = iph.add(1) as *mut udphdr;

    memcpy((*eth).h_dest.as_mut_ptr() as *mut c_void, b"\x00\x00\x00\x00\x00\x02".as_ptr() as *const c_void, ETH_ALEN);
    memcpy((*eth).h_source.as_mut_ptr() as *mut c_void, b"\x00\x00\x00\x00\x00\x01".as_ptr() as *const c_void, ETH_ALEN);
    (*eth).h_proto = htons(ETH_P_IP);

    (*iph).set_version(0x4);
    (*iph).set_ihl(0x5);
    (*iph).tos = 0x9;
    (*iph).tot_len = htons((size_of::<iphdr>() + size_of::<udphdr>() + UDP_PAYLOAD_BYTES) as u16);
    (*iph).id = 0;
    (*iph).frag_off = 0;
    (*iph).ttl = 0;
    (*iph).protocol = IPPROTO_UDP as u8;
    ASSERT_EQ(inet_pton(FAMILY, TX_ADDR.as_ptr() as *const c_char, &mut (*iph).saddr as *mut _ as *mut c_void), 1, c"inet_pton(TX_ADDR)".as_ptr());
    ASSERT_EQ(inet_pton(FAMILY, RX_ADDR.as_ptr() as *const c_char, &mut (*iph).daddr as *mut _ as *mut c_void), 1, c"inet_pton(RX_ADDR)".as_ptr());
    (*iph).check = build_ip_csum(iph);

    (*udph).source = htons(UDP_SOURCE_PORT);
    (*udph).dest = htons(dst_port);
    (*udph).len = htons((size_of::<udphdr>() + UDP_PAYLOAD_BYTES) as u16);
    (*udph).check = !csum_tcpudp_magic((*iph).saddr, (*iph).daddr, ntohs((*udph).len) as u32, IPPROTO_UDP as u8, 0);

    memset(udph.add(1) as *mut c_void, 0xAA, UDP_PAYLOAD_BYTES);

    (*meta).flags |= XDP_TXMD_FLAGS_CHECKSUM;
    (*meta).request.csum_start = (size_of::<ethhdr>() + size_of::<iphdr>()) as u16;
    (*meta).request.csum_offset = offset_of!(udphdr, check) as u16;

    (*tx_desc).len = (size_of::<ethhdr>() + size_of::<iphdr>() + size_of::<udphdr>() + UDP_PAYLOAD_BYTES) as u32;
    (*tx_desc).options |= XDP_TX_METADATA;
    xsk_ring_prod__submit(&mut (*xsk).tx, 1);

    ret = sendto(xsk_socket__fd((*xsk).socket), ptr::null(), 0, MSG_DONTWAIT, ptr::null(), 0) as c_int;
    if !ASSERT_GE(ret, 0, c"sendto".as_ptr()) {
        return ret;
    }

    0
}

unsafe fn generate_packet_inet() -> c_int {
    let mut udp_payload = [0i8; UDP_PAYLOAD_BYTES];
    let mut rx_addr: sockaddr_in = core::mem::zeroed();
    let sock_fd: c_int;
    let mut err: c_int = 0;

    /* Build a packet */
    memset(udp_payload.as_mut_ptr() as *mut c_void, 0xAA, UDP_PAYLOAD_BYTES);
    rx_addr.sin_addr.s_addr = inet_addr(RX_ADDR.as_ptr() as *const c_char);
    rx_addr.sin_family = AF_INET as u16;
    rx_addr.sin_port = htons(AF_XDP_CONSUMER_PORT);

    sock_fd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if !ASSERT_GE(sock_fd, 0, c"socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP)".as_ptr()) {
        return sock_fd;
    }

    err = sendto(
        sock_fd,
        udp_payload.as_ptr() as *const c_void,
        UDP_PAYLOAD_BYTES,
        MSG_DONTWAIT,
        &rx_addr as *const _ as *const sockaddr,
        size_of::<sockaddr_in>() as u32,
    ) as c_int;
    ASSERT_GE(err, 0, c"sendto".as_ptr());

    close(sock_fd);
    err
}

unsafe fn complete_tx(xsk: *mut xsk) {
    let mut meta: *mut xsk_tx_metadata;
    let addr: u64;
    let data: *mut c_void;
    let mut idx: u32 = 0;

    if ASSERT_EQ(xsk_ring_cons__peek(&mut (*xsk).comp, 1, &mut idx), 1, c"xsk_ring_cons__peek".as_ptr()) {
        addr = *xsk_ring_cons__comp_addr(&mut (*xsk).comp, idx);

        printf(c"%p: complete tx idx=%u addr=%llx\n".as_ptr(), xsk, idx, addr);

        data = xsk_umem__get_data((*xsk).umem_area, addr);
        meta = (data as *mut u8).sub(size_of::<xsk_tx_metadata>()) as *mut xsk_tx_metadata;

        ASSERT_NEQ((*meta).completion.tx_timestamp, 0, c"tx_timestamp".as_ptr());

        xsk_ring_cons__release(&mut (*xsk).comp, 1);
    }
}

unsafe fn refill_rx(xsk: *mut xsk, addr: u64) {
    let mut idx: u32 = 0;

    if ASSERT_EQ(xsk_ring_prod__reserve(&mut (*xsk).fill, 1, &mut idx), 1, c"xsk_ring_prod__reserve".as_ptr()) {
        printf(c"%p: complete idx=%u addr=%llx\n".as_ptr(), xsk, idx, addr);
        *xsk_ring_prod__fill_addr(&mut (*xsk).fill, idx) = addr;
        xsk_ring_prod__submit(&mut (*xsk).fill, 1);
    }
}

unsafe fn verify_xsk_metadata(xsk: *mut xsk, sent_from_af_xdp: bool) -> c_int {
    let rx_desc: *const xdp_desc;
    let mut fds: pollfd = core::mem::zeroed();
    let meta: *mut xdp_meta;
    let udph: *mut udphdr;
    let eth: *mut ethhdr;
    let iph: *mut iphdr;
    let comp_addr: u64;
    let data: *mut c_void;
    let addr: u64;
    let mut idx: u32 = 0;
    let mut ret: c_int;

    ret = recvfrom(xsk_socket__fd((*xsk).socket), ptr::null_mut(), 0, MSG_DONTWAIT, ptr::null_mut(), ptr::null_mut()) as c_int;
    if !ASSERT_EQ(ret, 0, c"recvfrom".as_ptr()) {
        return -1;
    }

    fds.fd = xsk_socket__fd((*xsk).socket);
    fds.events = POLLIN;

    ret = poll(&mut fds, 1, 1000);
    if !ASSERT_GT(ret, 0, c"poll".as_ptr()) {
        return -1;
    }

    ret = xsk_ring_cons__peek(&mut (*xsk).rx, 1, &mut idx);
    if !ASSERT_EQ(ret, 1, c"xsk_ring_cons__peek".as_ptr()) {
        return -2;
    }

    rx_desc = xsk_ring_cons__rx_desc(&mut (*xsk).rx, idx);
    comp_addr = xsk_umem__extract_addr((*rx_desc).addr);
    addr = xsk_umem__add_offset_to_addr((*rx_desc).addr);
    printf(c"%p: rx_desc[%u]->addr=%llx addr=%llx comp_addr=%llx\n".as_ptr(), xsk, idx, (*rx_desc).addr, addr, comp_addr);
    data = xsk_umem__get_data((*xsk).umem_area, addr);

    /* Make sure we got the packet offset correctly. */
    eth = data as *mut ethhdr;
    ASSERT_EQ((*eth).h_proto, htons(ETH_P_IP), c"eth->h_proto".as_ptr());
    iph = eth.add(1) as *mut iphdr;
    ASSERT_EQ((*iph).version() as c_int, 4, c"iph->version".as_ptr());
    udph = iph.add(1) as *mut udphdr;

    /* custom metadata */
    meta = (data as *mut u8).sub(size_of::<xdp_meta>()) as *mut xdp_meta;

    if !ASSERT_NEQ((*meta).rx_timestamp, 0, c"rx_timestamp".as_ptr()) {
        return -1;
    }

    if !ASSERT_NEQ((*meta).rx_hash, 0, c"rx_hash".as_ptr()) {
        return -1;
    }

    if !sent_from_af_xdp {
        if !ASSERT_NEQ((*meta).rx_hash_type & XDP_RSS_TYPE_L4, 0, c"rx_hash_type".as_ptr()) {
            return -1;
        }

        if !ASSERT_EQ((*meta).rx_vlan_tci & VLAN_VID_MASK, VLAN_ID, c"rx_vlan_tci".as_ptr()) {
            return -1;
        }

        if !ASSERT_EQ((*meta).rx_vlan_proto, vlan_pid(), c"rx_vlan_proto".as_ptr()) {
            return -1;
        }
    } else {
        ASSERT_EQ((*meta).rx_hash_type, 0, c"rx_hash_type".as_ptr());

        /* checksum offload */
        ASSERT_EQ((*udph).check, htons(0x721c), c"csum".as_ptr());
    }

    xsk_ring_cons__release(&mut (*xsk).rx, 1);
    refill_rx(xsk, comp_addr);

    0
}

unsafe fn switch_ns_to_rx(tok: *mut *mut nstoken) {
    close_netns(*tok);
    *tok = open_netns(RX_NETNS_NAME.as_ptr() as *const c_char);
}

unsafe fn switch_ns_to_tx(tok: *mut *mut nstoken) {
    close_netns(*tok);
    *tok = open_netns(TX_NETNS_NAME.as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp_metadata() {
    let mut bpf_obj2: *mut xdp_metadata2 = ptr::null_mut();
    let mut bpf_obj: *mut xdp_metadata = ptr::null_mut();
    let mut new_prog: *mut bpf_program;
    let prog: *mut bpf_program;
    let mut devmap_e: bpf_devmap_val = core::mem::zeroed();
    let mut prog_arr: *mut bpf_map;
    let mut devmap: *mut bpf_map;
    let mut tok: *mut nstoken = ptr::null_mut();
    let queue_id: u32 = QUEUE_ID;
    let mut tx_xsk: xsk = core::mem::zeroed();
    let mut rx_xsk: xsk = core::mem::zeroed();
    let mut val: u32;
    let key: u32 = 0;
    let mut retries: c_int = 10;
    let rx_ifindex: c_uint;
    let tx_ifindex: c_uint;
    let sock_fd: c_int;
    let mut ret: c_int;

    /* Setup new networking namespaces, with a veth pair. */
    SYS(c"out".as_ptr(), c"ip netns add xdp_metadata_tx".as_ptr());
    SYS(c"out".as_ptr(), c"ip netns add xdp_metadata_rx".as_ptr());

    tok = open_netns(TX_NETNS_NAME.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(tok, c"setns".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }
    SYS(c"out".as_ptr(), c"ip link add numtxqueues 1 numrxqueues 1 veTX type veth peer veRX numtxqueues 1 numrxqueues 1".as_ptr());
    SYS(c"out".as_ptr(), c"ip link set veRX netns xdp_metadata_rx".as_ptr());

    SYS(c"out".as_ptr(), c"ip link set dev veTX address 00:00:00:00:00:01".as_ptr());
    SYS(c"out".as_ptr(), c"ip link set dev veTX up".as_ptr());

    SYS(c"out".as_ptr(), c"ip link add link veTX veTX.59 type vlan proto 802.1Q id 59".as_ptr());
    SYS(c"out".as_ptr(), c"ip link set dev veTX.59 up".as_ptr());
    SYS(c"out".as_ptr(), c"ip addr add 10.0.0.1/8 dev veTX.59".as_ptr());

    /* Avoid ARP calls */
    SYS(c"out".as_ptr(), c"ip -4 neigh add 10.0.0.2 lladdr 00:00:00:00:00:02 dev veTX.59".as_ptr());

    switch_ns_to_rx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns rx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    SYS(c"out".as_ptr(), c"ip link set dev veRX address 00:00:00:00:00:02".as_ptr());
    SYS(c"out".as_ptr(), c"ip link set dev veRX up".as_ptr());
    SYS(c"out".as_ptr(), c"ip addr add 10.0.0.2/8 dev veRX".as_ptr());

    rx_ifindex = if_nametoindex(RX_NAME.as_ptr() as *const c_char);

    /* Setup separate AF_XDP for RX interface. */
    ret = open_xsk(rx_ifindex as c_int, &mut rx_xsk);
    if !ASSERT_OK(ret, c"open_xsk(RX_NAME)".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    bpf_obj = xdp_metadata__open();
    if !ASSERT_OK_PTR(bpf_obj, c"open skeleton".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    prog = bpf_object__find_program_by_name((*bpf_obj).obj, c"rx".as_ptr());
    bpf_program__set_ifindex(prog, rx_ifindex);
    bpf_program__set_flags(prog, BPF_F_XDP_DEV_BOUND_ONLY);

    /* Make sure we can load a dev-bound program that performs
     * XDP_REDIRECT into a devmap.
     */
    new_prog = bpf_object__find_program_by_name((*bpf_obj).obj, c"redirect".as_ptr());
    bpf_program__set_ifindex(new_prog, rx_ifindex);
    bpf_program__set_flags(new_prog, BPF_F_XDP_DEV_BOUND_ONLY);

    if !ASSERT_OK(xdp_metadata__load(bpf_obj), c"load skeleton".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Make sure we can't add dev-bound programs to prog maps. */
    prog_arr = bpf_object__find_map_by_name((*bpf_obj).obj, c"prog_arr".as_ptr());
    if !ASSERT_OK_PTR(prog_arr, c"no prog_arr map".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    val = bpf_program__fd(prog) as u32;
    if !ASSERT_ERR(
        bpf_map__update_elem(
            prog_arr,
            &key as *const _ as *const c_void,
            size_of::<u32>(),
            &val as *const _ as *const c_void,
            size_of::<u32>(),
            BPF_ANY,
        ),
        c"update prog_arr".as_ptr(),
    ) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Make sure we can't add dev-bound programs to devmaps. */
    devmap = bpf_object__find_map_by_name((*bpf_obj).obj, c"dev_map".as_ptr());
    if !ASSERT_OK_PTR(devmap, c"no dev_map found".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    devmap_e.bpf_prog.fd = val as c_int;
    if !ASSERT_ERR(
        bpf_map__update_elem(
            devmap,
            &key as *const _ as *const c_void,
            size_of::<u32>(),
            &devmap_e as *const _ as *const c_void,
            size_of::<bpf_devmap_val>(),
            BPF_ANY,
        ),
        c"update dev_map".as_ptr(),
    ) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Attach BPF program to RX interface. */
    ret = bpf_xdp_attach(rx_ifindex, bpf_program__fd((*(*bpf_obj).progs).rx), XDP_FLAGS, ptr::null());
    if !ASSERT_GE(ret, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    sock_fd = xsk_socket__fd(rx_xsk.socket);
    ret = bpf_map_update_elem(
        bpf_map__fd((*(*bpf_obj).maps).xsk),
        &queue_id as *const _ as *const c_void,
        &sock_fd as *const _ as *const c_void,
        0,
    );
    if !ASSERT_GE(ret, 0, c"bpf_map_update_elem".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_tx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns tx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Setup separate AF_XDP for TX interface nad send packet to the RX socket. */
    tx_ifindex = if_nametoindex(TX_NAME.as_ptr() as *const c_char);
    ret = open_xsk(tx_ifindex as c_int, &mut tx_xsk);
    if !ASSERT_OK(ret, c"open_xsk(TX_NAME)".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    if !ASSERT_GE(generate_packet(&mut tx_xsk, AF_XDP_CONSUMER_PORT), 0, c"generate AF_XDP_CONSUMER_PORT".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_rx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns rx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Verify packet sent from AF_XDP has proper metadata. */
    if !ASSERT_GE(verify_xsk_metadata(&mut rx_xsk, true), 0, c"verify_xsk_metadata".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_tx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns tx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }
    complete_tx(&mut tx_xsk);

    /* Now check metadata of packet, generated with network stack */
    if !ASSERT_GE(generate_packet_inet(), 0, c"generate UDP packet".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_rx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns rx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    if !ASSERT_GE(verify_xsk_metadata(&mut rx_xsk, false), 0, c"verify_xsk_metadata".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Make sure freplace correctly picks up original bound device
     * and doesn't crash.
     */
    bpf_obj2 = xdp_metadata2__open();
    if !ASSERT_OK_PTR(bpf_obj2, c"open skeleton".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    new_prog = bpf_object__find_program_by_name((*bpf_obj2).obj, c"freplace_rx".as_ptr());
    bpf_program__set_attach_target(new_prog, bpf_program__fd(prog), c"rx".as_ptr());

    if !ASSERT_OK(xdp_metadata2__load(bpf_obj2), c"load freplace skeleton".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    if !ASSERT_OK(xdp_metadata2__attach(bpf_obj2), c"attach freplace".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_tx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns tx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    /* Send packet to trigger . */
    if !ASSERT_GE(generate_packet(&mut tx_xsk, AF_XDP_CONSUMER_PORT), 0, c"generate freplace packet".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    switch_ns_to_rx(&mut tok);
    if !ASSERT_OK_PTR(tok, c"setns rx".as_ptr()) {
        goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
        return;
    }

    loop {
        let cond = retries == 0;
        retries -= 1;
        if !cond {
            break;
        }
        if (*(*bpf_obj2).bss).called != 0 {
            break;
        }
        usleep(10);
    }
    ASSERT_GT((*(*bpf_obj2).bss).called, 0, c"not called".as_ptr());

    goto_out(bpf_obj2, bpf_obj, tok, &mut rx_xsk, &mut tx_xsk);
}

unsafe fn goto_out(
    bpf_obj2: *mut xdp_metadata2,
    bpf_obj: *mut xdp_metadata,
    tok: *mut nstoken,
    rx_xsk: *mut xsk,
    tx_xsk: *mut xsk,
) {
    close_xsk(rx_xsk);
    close_xsk(tx_xsk);
    xdp_metadata2__destroy(bpf_obj2);
    xdp_metadata__destroy(bpf_obj);
    if !tok.is_null() {
        close_netns(tok);
    }
    SYS_NOFAIL(c"ip netns del xdp_metadata_rx".as_ptr());
    SYS_NOFAIL(c"ip netns del xdp_metadata_tx".as_ptr());
}
