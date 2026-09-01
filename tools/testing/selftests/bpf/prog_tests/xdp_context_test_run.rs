// SPDX-License-Identifier: GPL-2.0
// Translated from C source. Dependencies from test_progs.h,
// network_helpers.h, linux/ipv6.h, arpa/inet.h, and skeleton headers are
// expected to be supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const RX_NAME: &[u8] = b"veth0\0";
const TX_NAME: &[u8] = b"veth1\0";
const TX_NETNS: &[u8] = b"xdp_context_tx\0";
const RX_NETNS: &[u8] = b"xdp_context_rx\0";
const RX_MAC: &[u8] = b"02:00:00:00:00:01\0";
const TX_MAC: &[u8] = b"02:00:00:00:00:02\0";
const TAP_NAME: &[u8] = b"tap0\0";
const DUMMY_NAME: &[u8] = b"dum0\0";
const TAP_NETNS: &[u8] = b"xdp_context_tuntap\0";
const LWT_NETNS: &[u8] = b"xdp_context_lwt\0";

const TEST_PAYLOAD_LEN: usize = 32;
static TEST_PAYLOAD: [u8; TEST_PAYLOAD_LEN] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
];

const ETH_ALEN: usize = 6;
const AF_PACKET: c_int = 17;
const PF_PACKET: c_int = AF_PACKET;
const SOCK_RAW: c_int = 3;
const IPPROTO_RAW: c_int = 255;
const IPPROTO_UDP: c_int = 17;
const AF_INET6: c_int = 10;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const XDP_PASS: u32 = 2;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const BPF_TC_INGRESS: u32 = 1;
const BPF_TC_EGRESS: u32 = 2;
const BPF_STREAM_STDERR: u32 = 2;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub ctx_in: *mut c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: u32,
    pub ctx_size_out: u32,
    pub retval: u32,
    pub repeat: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: c_int,
    pub attach_point: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tc_opts {
    pub sz: usize,
    pub prog_fd: c_int,
    pub handle: u32,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ll {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: c_int,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

impl ipv6hdr {
    unsafe fn set_version(&mut self, version: u8) {
        self.priority_version = (self.priority_version & 0x0f) | ((version & 0x0f) << 4);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_packet {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct test_xdp_context_test_run_progs {
    pub xdp_context: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_context_test_run {
    pub progs: test_xdp_context_test_run_progs,
}

#[repr(C)]
pub struct test_xdp_meta_progs {
    pub ing_cls: *mut bpf_program,
    pub ing_xdp: *mut bpf_program,
    pub ing_cls_dynptr_read: *mut bpf_program,
    pub ing_cls_dynptr_slice: *mut bpf_program,
    pub ing_xdp_zalloc_meta: *mut bpf_program,
    pub ing_cls_dynptr_write: *mut bpf_program,
    pub ing_cls_dynptr_slice_rdwr: *mut bpf_program,
    pub ing_cls_dynptr_offset_wr: *mut bpf_program,
    pub ing_cls_dynptr_offset_rd: *mut bpf_program,
    pub ing_cls_dynptr_offset_oob: *mut bpf_program,
    pub clone_data_meta_survives_data_write: *mut bpf_program,
    pub clone_data_meta_survives_meta_write: *mut bpf_program,
    pub clone_meta_dynptr_survives_data_slice_write: *mut bpf_program,
    pub clone_meta_dynptr_survives_meta_slice_write: *mut bpf_program,
    pub clone_meta_dynptr_rw_before_data_dynptr_write: *mut bpf_program,
    pub clone_meta_dynptr_rw_before_meta_dynptr_write: *mut bpf_program,
    pub helper_skb_vlan_push_pop: *mut bpf_program,
    pub helper_skb_adjust_room: *mut bpf_program,
    pub helper_skb_change_head_tail: *mut bpf_program,
    pub helper_skb_change_proto: *mut bpf_program,
    pub dummy_lwt_xmit: *mut bpf_program,
    pub tc_is_meta_empty: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_meta_bss {
    pub test_pass: bool,
}

#[repr(C)]
pub struct test_xdp_meta {
    pub obj: *mut bpf_object,
    pub progs: test_xdp_meta_progs,
    pub bss: *mut test_xdp_meta_bss,
}

unsafe extern "C" {
    static pkt_v4: ipv4_packet;
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn test_xdp_context_test_run__open_and_load() -> *mut test_xdp_context_test_run;
    fn test_xdp_context_test_run__destroy(skel: *mut test_xdp_context_test_run);
    fn test_xdp_meta__open_and_load() -> *mut test_xdp_meta;
    fn test_xdp_meta__destroy(skel: *mut test_xdp_meta);
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: u32, opts: *const c_void) -> c_int;
    fn bpf_prog_stream_read(prog_fd: c_int, stream: u32, buf: *mut c_void, size: usize, flags: *mut c_void) -> c_int;
    fn bpf_program__pin(prog: *mut bpf_program, path: *const c_char) -> c_int;

    fn netns_new(name: *const c_char, open: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn open_tuntap(name: *const c_char, is_tap: bool) -> c_int;
    fn if_nametoindex(name: *const c_char) -> c_uint;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn sendto(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int, dest_addr: *const sockaddr, addrlen: u32) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;

    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
}

unsafe fn new_bpf_test_run_opts() -> bpf_test_run_opts {
    zeroed()
}

unsafe fn new_bpf_tc_hook(attach_point: u32) -> bpf_tc_hook {
    let mut hook: bpf_tc_hook = zeroed();
    hook.sz = size_of::<bpf_tc_hook>();
    hook.attach_point = attach_point;
    hook
}

unsafe fn new_bpf_tc_opts(handle: u32, priority: u32) -> bpf_tc_opts {
    let mut opts: bpf_tc_opts = zeroed();
    opts.sz = size_of::<bpf_tc_opts>();
    opts.handle = handle;
    opts.priority = priority;
    opts
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_context_error(
    prog_fd: c_int,
    mut opts: bpf_test_run_opts,
    data_meta: u32,
    data: u32,
    data_end: u32,
    ingress_ifindex: u32,
    rx_queue_index: u32,
    egress_ifindex: u32,
) {
    let mut ctx = xdp_md {
        data,
        data_end,
        data_meta,
        ingress_ifindex,
        rx_queue_index,
        egress_ifindex,
    };
    let err: c_int;

    opts.ctx_in = &mut ctx as *mut _ as *mut c_void;
    opts.ctx_size_in = size_of::<xdp_md>() as u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    ASSERT_EQ(errno as isize, EINVAL as isize, c"errno-EINVAL".as_ptr());
    ASSERT_ERR(err, c"bpf_prog_test_run".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_context_test_run() {
    let mut skel: *mut test_xdp_context_test_run = null_mut();
    let mut data = [0u8; size_of::<ipv4_packet>() + size_of::<u32>()];
    let mut bad_ctx = [0u8; size_of::<xdp_md>() + 1];
    let mut large_data = [0u8; 256];
    let mut ctx_in: xdp_md = zeroed();
    let mut ctx_out: xdp_md = zeroed();
    let mut opts = new_bpf_test_run_opts();
    opts.data_in = data.as_mut_ptr() as *mut c_void;
    opts.data_size_in = size_of_val(&data) as u32;
    opts.ctx_out = &mut ctx_out as *mut _ as *mut c_void;
    opts.ctx_size_out = size_of::<xdp_md>() as u32;
    opts.repeat = 1;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = test_xdp_context_test_run__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel".as_ptr()) {
        return;
    }
    prog_fd = bpf_program__fd((*skel).progs.xdp_context);

    /* Data past the end of the kernel's struct xdp_md must be 0 */
    bad_ctx[size_of_val(&bad_ctx) - 1] = 1;
    opts.ctx_in = bad_ctx.as_mut_ptr() as *mut c_void;
    opts.ctx_size_in = size_of_val(&bad_ctx) as u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    ASSERT_EQ(errno as isize, E2BIG as isize, c"extradata-errno".as_ptr());
    ASSERT_ERR(err, c"bpf_prog_test_run(extradata)".as_ptr());

    *(data.as_mut_ptr() as *mut u32) = XDP_PASS;
    *(data.as_mut_ptr().add(size_of::<u32>()) as *mut ipv4_packet) = pkt_v4;
    opts.ctx_in = &mut ctx_in as *mut _ as *mut c_void;
    opts.ctx_size_in = size_of::<xdp_md>() as u32;
    memset(&mut ctx_in as *mut _ as *mut c_void, 0, size_of::<xdp_md>());
    ctx_in.data_meta = 0;
    ctx_in.data = size_of::<u32>() as u32;
    ctx_in.data_end = ctx_in.data + size_of::<ipv4_packet>() as u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    ASSERT_OK(err, c"bpf_prog_test_run(valid)".as_ptr());
    ASSERT_EQ(opts.retval as isize, XDP_PASS as isize, c"valid-retval".as_ptr());
    ASSERT_EQ(opts.data_size_out as isize, size_of::<ipv4_packet>() as isize, c"valid-datasize".as_ptr());
    ASSERT_EQ(opts.ctx_size_out as isize, opts.ctx_size_in as isize, c"valid-ctxsize".as_ptr());
    ASSERT_EQ(ctx_out.data_meta as isize, 0, c"valid-datameta".as_ptr());
    ASSERT_EQ(ctx_out.data as isize, 0, c"valid-data".as_ptr());
    ASSERT_EQ(ctx_out.data_end as isize, size_of::<ipv4_packet>() as isize, c"valid-dataend".as_ptr());

    /* Meta data's size must be a multiple of 4 */
    test_xdp_context_error(prog_fd, opts, 0, 1, size_of_val(&data) as u32, 0, 0, 0);

    /* data_meta must reference the start of data */
    test_xdp_context_error(prog_fd, opts, 4, size_of::<u32>() as u32, size_of_val(&data) as u32, 0, 0, 0);

    /* Total size of data must be data_end - data_meta or larger */
    test_xdp_context_error(prog_fd, opts, 0, size_of::<u32>() as u32, size_of_val(&data) as u32 + 1, 0, 0, 0);

    /* RX queue cannot be specified without specifying an ingress */
    test_xdp_context_error(prog_fd, opts, 0, size_of::<u32>() as u32, size_of_val(&data) as u32, 0, 1, 0);

    /* Interface 1 is always the loopback interface which always has only
     * one RX queue (index 0). This makes index 1 an invalid rx queue index
     * for interface 1.
     */
    test_xdp_context_error(prog_fd, opts, 0, size_of::<u32>() as u32, size_of_val(&data) as u32, 1, 1, 0);

    /* The egress cannot be specified */
    test_xdp_context_error(prog_fd, opts, 0, size_of::<u32>() as u32, size_of_val(&data) as u32, 0, 0, 1);

    /* Meta data must be 216 bytes or smaller (256 - sizeof(struct
     * xdp_frame)). Test both nearest invalid size and nearest invalid
     * 4-byte-aligned size, and make sure data_in is large enough that we
     * actually hit the check on metadata length
     */
    opts.data_in = large_data.as_mut_ptr() as *mut c_void;
    opts.data_size_in = size_of_val(&large_data) as u32;
    test_xdp_context_error(prog_fd, opts, 0, 217, size_of_val(&large_data) as u32, 0, 0, 0);
    test_xdp_context_error(prog_fd, opts, 0, 220, size_of_val(&large_data) as u32, 0, 0, 0);

    test_xdp_context_test_run__destroy(skel);
}

unsafe fn size_of_val<T>(v: &T) -> usize {
    core::mem::size_of_val(v)
}

unsafe fn send_test_packet(ifindex: c_int) -> c_int {
    let mut n: isize;
    let mut sock: c_int = -1;
    let mut packet = [0u8; size_of::<ethhdr>() + TEST_PAYLOAD_LEN];

    /* We use the Ethernet header only to identify the test packet */
    let eth = ethhdr {
        h_dest: [0; ETH_ALEN],
        h_source: [0x12, 0x34, 0xDE, 0xAD, 0xBE, 0xEF],
        h_proto: 0,
    };

    memcpy(packet.as_mut_ptr() as *mut c_void, &eth as *const _ as *const c_void, size_of::<ethhdr>());
    memcpy(packet.as_mut_ptr().add(size_of::<ethhdr>()) as *mut c_void, TEST_PAYLOAD.as_ptr() as *const c_void, TEST_PAYLOAD_LEN);

    sock = socket(AF_PACKET, SOCK_RAW, IPPROTO_RAW);
    if !ASSERT_GE(sock as isize, 0, c"socket".as_ptr()) {
        goto_send_test_packet_err(sock);
        return -1;
    }

    let saddr = sockaddr_ll {
        sll_family: PF_PACKET as u16,
        sll_protocol: 0,
        sll_ifindex: ifindex,
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: ETH_ALEN as u8,
        sll_addr: [0; 8],
    };
    n = sendto(sock, packet.as_ptr() as *const c_void, size_of_val(&packet), 0, &saddr as *const _ as *const sockaddr, size_of::<sockaddr_ll>() as u32);
    if !ASSERT_EQ(n, size_of_val(&packet) as isize, c"sendto".as_ptr()) {
        goto_send_test_packet_err(sock);
        return -1;
    }

    close(sock);
    0
}

unsafe fn goto_send_test_packet_err(sock: c_int) {
    if sock >= 0 {
        close(sock);
    }
}

unsafe fn write_test_packet(tap_fd: c_int) -> c_int {
    let mut packet = [0u8; size_of::<ethhdr>() + TEST_PAYLOAD_LEN];
    let n: isize;

    /* The Ethernet header is mostly not relevant. We use it to identify the
     * test packet and some BPF helpers we exercise expect to operate on
     * Ethernet frames carrying IP packets. Pretend that's the case.
     */
    let eth = ethhdr {
        h_dest: [0; ETH_ALEN],
        h_source: [0x12, 0x34, 0xDE, 0xAD, 0xBE, 0xEF],
        h_proto: htons(ETH_P_IP),
    };

    memcpy(packet.as_mut_ptr() as *mut c_void, &eth as *const _ as *const c_void, size_of::<ethhdr>());
    memcpy(packet.as_mut_ptr().add(size_of::<ethhdr>()) as *mut c_void, TEST_PAYLOAD.as_ptr() as *const c_void, TEST_PAYLOAD_LEN);

    n = write(tap_fd, packet.as_ptr() as *const c_void, size_of_val(&packet));
    if !ASSERT_EQ(n, size_of_val(&packet) as isize, c"write packet".as_ptr()) {
        return -1;
    }

    0
}

/* Inject Ethernet+IPv6+UDP frame into TAP */
unsafe fn write_test_packet_udp(tap_fd: c_int) -> c_int {
    let mut pkt = [0u8; size_of::<ethhdr>() + size_of::<ipv6hdr>() + size_of::<udphdr>() + TEST_PAYLOAD_LEN];
    let eth = pkt.as_mut_ptr() as *mut ethhdr;
    let ip6 = eth.add(1) as *mut ipv6hdr;
    let udp = ip6.add(1) as *mut udphdr;
    let payload = udp.add(1) as *mut u8;
    let tap_mac: [u8; ETH_ALEN] = [0x02, 0, 0, 0, 0, 0x01];
    let n: isize;

    memcpy((*eth).h_dest.as_mut_ptr() as *mut c_void, tap_mac.as_ptr() as *const c_void, ETH_ALEN);
    (*eth).h_proto = htons(ETH_P_IPV6);

    (*ip6).set_version(6);
    (*ip6).hop_limit = 64;
    (*ip6).nexthdr = IPPROTO_UDP as u8;
    (*ip6).payload_len = htons((size_of::<udphdr>() + TEST_PAYLOAD_LEN) as u16);
    inet_pton(AF_INET6, c"fd00::2".as_ptr(), &mut (*ip6).saddr as *mut _ as *mut c_void);
    inet_pton(AF_INET6, c"fd00:1::1".as_ptr(), &mut (*ip6).daddr as *mut _ as *mut c_void);

    (*udp).source = htons(42);
    (*udp).dest = htons(42);
    (*udp).len = htons((size_of::<udphdr>() + TEST_PAYLOAD_LEN) as u16);
    /* UDP checksum is not validated on the forwarding path. */

    memcpy(payload as *mut c_void, TEST_PAYLOAD.as_ptr() as *const c_void, TEST_PAYLOAD_LEN);

    n = write(tap_fd, pkt.as_ptr() as *const c_void, size_of_val(&pkt));
    if !ASSERT_EQ(n, size_of_val(&pkt) as isize, c"write frame".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn dump_err_stream(prog: *const bpf_program) {
    let mut buf = [0u8; 512];
    let mut ret: c_int;

    ret = 0;
    loop {
        ret = bpf_prog_stream_read(bpf_program__fd(prog), BPF_STREAM_STDERR, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf), null_mut());
        if ret > 0 {
            fwrite(buf.as_ptr() as *const c_void, size_of::<u8>(), ret as usize, stderr);
        }
        if ret <= 0 {
            break;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_context_veth() {
    let mut tc_hook = new_bpf_tc_hook(BPF_TC_INGRESS);
    let mut tc_opts = new_bpf_tc_opts(1, 1);
    let mut rx_ns: *mut netns_obj = null_mut();
    let mut tx_ns: *mut netns_obj = null_mut();
    let mut tc_prog: *mut bpf_program;
    let mut xdp_prog: *mut bpf_program;
    let mut skel: *mut test_xdp_meta = null_mut();
    let mut nstoken: *mut nstoken = null_mut();
    let rx_ifindex: c_int;
    let tx_ifindex: c_int;
    let mut ret: c_int;

    tx_ns = netns_new(TX_NETNS.as_ptr() as *const c_char, false);
    if !ASSERT_OK_PTR(tx_ns, c"create tx_ns".as_ptr()) {
        return;
    }

    rx_ns = netns_new(RX_NETNS.as_ptr() as *const c_char, false);
    if !ASSERT_OK_PTR(rx_ns, c"create rx_ns".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link add veth0 netns xdp_context_rx type veth peer name veth1 netns xdp_context_tx".as_ptr());

    nstoken = open_netns(RX_NETNS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns rx_ns".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link set dev veth0 up".as_ptr());

    skel = test_xdp_meta__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open and load skeleton".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    rx_ifindex = if_nametoindex(RX_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(rx_ifindex as isize, 0, c"if_nametoindex rx".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    tc_hook.ifindex = rx_ifindex;
    ret = bpf_tc_hook_create(&mut tc_hook);
    if !ASSERT_OK(ret, c"bpf_tc_hook_create".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    tc_prog = bpf_object__find_program_by_name((*skel).obj, c"ing_cls".as_ptr());
    if !ASSERT_OK_PTR(tc_prog, c"open ing_cls prog".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    tc_opts.prog_fd = bpf_program__fd(tc_prog);
    ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    xdp_prog = bpf_object__find_program_by_name((*skel).obj, c"ing_xdp".as_ptr());
    if !ASSERT_OK_PTR(xdp_prog, c"open ing_xdp prog".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    ret = bpf_xdp_attach(rx_ifindex, bpf_program__fd(xdp_prog), 0, null());
    if !ASSERT_GE(ret as isize, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    close_netns(nstoken);

    nstoken = open_netns(TX_NETNS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns tx_ns".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link set dev veth1 up".as_ptr());

    tx_ifindex = if_nametoindex(TX_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(tx_ifindex as isize, 0, c"if_nametoindex tx".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    (*(*skel).bss).test_pass = false;

    ret = send_test_packet(tx_ifindex);
    if !ASSERT_OK(ret, c"send_test_packet".as_ptr()) {
        goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
        return;
    }

    if !ASSERT_TRUE((*(*skel).bss).test_pass, c"test_pass".as_ptr()) {
        dump_err_stream(tc_prog);
    }

    goto_test_xdp_context_veth_close(nstoken, skel, rx_ns, tx_ns);
}

unsafe fn goto_test_xdp_context_veth_close(nstoken_p: *mut nstoken, skel: *mut test_xdp_meta, rx_ns: *mut netns_obj, tx_ns: *mut netns_obj) {
    close_netns(nstoken_p);
    test_xdp_meta__destroy(skel);
    netns_free(rx_ns);
    netns_free(tx_ns);
}

unsafe fn test_tuntap(
    xdp_prog: *mut bpf_program,
    tc_prio_1_prog: *mut bpf_program,
    tc_prio_2_prog: *mut bpf_program,
    test_pass: *mut bool,
) {
    let mut tc_hook = new_bpf_tc_hook(BPF_TC_INGRESS);
    let mut tc_opts = new_bpf_tc_opts(1, 1);
    let mut ns: *mut netns_obj = null_mut();
    let mut tap_fd: c_int = -1;
    let tap_ifindex: c_int;
    let mut ret: c_int;

    *test_pass = false;

    ns = netns_new(TAP_NETNS.as_ptr() as *const c_char, true);
    if !ASSERT_OK_PTR(ns, c"create and open ns".as_ptr()) {
        return;
    }

    tap_fd = open_tuntap(TAP_NAME.as_ptr() as *const c_char, true);
    if !ASSERT_GE(tap_fd as isize, 0, c"open_tuntap".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link set dev tap0 up".as_ptr());

    tap_ifindex = if_nametoindex(TAP_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(tap_ifindex as isize, 0, c"if_nametoindex".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    tc_hook.ifindex = tap_ifindex;
    ret = bpf_tc_hook_create(&mut tc_hook);
    if !ASSERT_OK(ret, c"bpf_tc_hook_create".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    tc_opts.prog_fd = bpf_program__fd(tc_prio_1_prog);
    ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    if !tc_prio_2_prog.is_null() {
        let mut tc_opts = new_bpf_tc_opts(1, 2);
        tc_opts.prog_fd = bpf_program__fd(tc_prio_2_prog);

        ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
        if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
            goto_test_tuntap_close(tap_fd, ns);
            return;
        }
    }

    ret = bpf_xdp_attach(tap_ifindex, bpf_program__fd(xdp_prog), 0, null());
    if !ASSERT_GE(ret as isize, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    ret = write_test_packet(tap_fd);
    if !ASSERT_OK(ret, c"write_test_packet".as_ptr()) {
        goto_test_tuntap_close(tap_fd, ns);
        return;
    }

    if !ASSERT_TRUE(*test_pass, c"test_pass".as_ptr()) {
        dump_err_stream(if !tc_prio_2_prog.is_null() { tc_prio_2_prog } else { tc_prio_1_prog });
    }

    goto_test_tuntap_close(tap_fd, ns);
}

unsafe fn goto_test_tuntap_close(tap_fd: c_int, ns: *mut netns_obj) {
    if tap_fd >= 0 {
        close(tap_fd);
    }
    netns_free(ns);
}

/* Write a packet to a tap dev and copy it to ingress of a dummy dev */
unsafe fn test_tuntap_mirred(
    xdp_prog: *mut bpf_program,
    tc_prog: *mut bpf_program,
    test_pass: *mut bool,
) {
    let mut tc_hook = new_bpf_tc_hook(BPF_TC_INGRESS);
    let mut tc_opts = new_bpf_tc_opts(1, 1);
    let mut ns: *mut netns_obj = null_mut();
    let dummy_ifindex: c_int;
    let mut tap_fd: c_int = -1;
    let tap_ifindex: c_int;
    let mut ret: c_int;

    *test_pass = false;

    ns = netns_new(TAP_NETNS.as_ptr() as *const c_char, true);
    if !ASSERT_OK_PTR(ns, c"netns_new".as_ptr()) {
        return;
    }

    /* Setup dummy interface */
    SYS(c"close".as_ptr(), c"ip link add name dum0 type dummy".as_ptr());
    SYS(c"close".as_ptr(), c"ip link set dev dum0 up".as_ptr());

    dummy_ifindex = if_nametoindex(DUMMY_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(dummy_ifindex as isize, 0, c"if_nametoindex".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    tc_hook.ifindex = dummy_ifindex;
    ret = bpf_tc_hook_create(&mut tc_hook);
    if !ASSERT_OK(ret, c"bpf_tc_hook_create".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    tc_opts.prog_fd = bpf_program__fd(tc_prog);
    ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    /* Setup TAP interface */
    tap_fd = open_tuntap(TAP_NAME.as_ptr() as *const c_char, true);
    if !ASSERT_GE(tap_fd as isize, 0, c"open_tuntap".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link set dev tap0 up".as_ptr());

    tap_ifindex = if_nametoindex(TAP_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(tap_ifindex as isize, 0, c"if_nametoindex".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    ret = bpf_xdp_attach(tap_ifindex, bpf_program__fd(xdp_prog), 0, null());
    if !ASSERT_GE(ret as isize, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    /* Copy all packets received from TAP to dummy ingress */
    SYS(c"close".as_ptr(), c"tc qdisc add dev tap0 clsact".as_ptr());
    SYS(c"close".as_ptr(), c"tc filter add dev tap0 ingress protocol all matchall action mirred ingress mirror dev dum0".as_ptr());

    /* Receive a packet on TAP */
    ret = write_test_packet(tap_fd);
    if !ASSERT_OK(ret, c"write_test_packet".as_ptr()) {
        goto_test_tuntap_mirred_close(tap_fd, ns);
        return;
    }

    if !ASSERT_TRUE(*test_pass, c"test_pass".as_ptr()) {
        dump_err_stream(tc_prog);
    }

    goto_test_tuntap_mirred_close(tap_fd, ns);
}

unsafe fn goto_test_tuntap_mirred_close(tap_fd: c_int, ns: *mut netns_obj) {
    if tap_fd >= 0 {
        close(tap_fd);
    }
    netns_free(ns);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_context_tuntap() {
    let mut skel: *mut test_xdp_meta = null_mut();

    skel = test_xdp_meta__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open and load skeleton".as_ptr()) {
        return;
    }

    if test__start_subtest(c"data_meta".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.ing_cls, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_read".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.ing_cls_dynptr_read, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_slice".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.ing_cls_dynptr_slice, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_write".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp_zalloc_meta, (*skel).progs.ing_cls_dynptr_write, (*skel).progs.ing_cls_dynptr_read, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_slice_rdwr".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp_zalloc_meta, (*skel).progs.ing_cls_dynptr_slice_rdwr, (*skel).progs.ing_cls_dynptr_slice, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_offset".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp_zalloc_meta, (*skel).progs.ing_cls_dynptr_offset_wr, (*skel).progs.ing_cls_dynptr_offset_rd, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"dynptr_offset_oob".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.ing_cls_dynptr_offset_oob, (*skel).progs.ing_cls, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_data_meta_survives_data_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_data_meta_survives_data_write, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_data_meta_survives_meta_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_data_meta_survives_meta_write, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_meta_dynptr_survives_data_slice_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_meta_dynptr_survives_data_slice_write, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_meta_dynptr_survives_meta_slice_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_meta_dynptr_survives_meta_slice_write, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_meta_dynptr_rw_before_data_dynptr_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_meta_dynptr_rw_before_data_dynptr_write, &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"clone_meta_dynptr_rw_before_meta_dynptr_write".as_ptr()) {
        test_tuntap_mirred((*skel).progs.ing_xdp, (*skel).progs.clone_meta_dynptr_rw_before_meta_dynptr_write, &mut (*(*skel).bss).test_pass);
    }
    /* Tests for BPF helpers which touch headroom */
    if test__start_subtest(c"helper_skb_vlan_push_pop".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.helper_skb_vlan_push_pop, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"helper_skb_adjust_room".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.helper_skb_adjust_room, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"helper_skb_change_head_tail".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.helper_skb_change_head_tail, null_mut(), &mut (*(*skel).bss).test_pass);
    }
    if test__start_subtest(c"helper_skb_change_proto".as_ptr()) {
        test_tuntap((*skel).progs.ing_xdp, (*skel).progs.helper_skb_change_proto, null_mut(), &mut (*(*skel).bss).test_pass);
    }

    test_xdp_meta__destroy(skel);
}

/*
 * Test topology:
 *
 *      tap0 fd00::1
 *        RX:  injected IPv6 UDP frame, XDP ingress sets metadata
 *        fwd: encap route prepends outer header(s)
 *        TX:  TC egress validates metadata
 *
 * A routable IPv6 UDP frame is written into the tap fd, so it enters the RX
 * path where XDP stores metadata. Routing then forwards it back out the same
 * tap through an encapsulating route that prepends outer header(s). The TC
 * egress program checks that the pushed header did not silently corrupt
 * metadata.
 */
const LWT_PIN_PATH: &[u8] = b"/sys/fs/bpf/xdp_context_lwt_xmit\0";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum lwt_encap_type {
    LWT_ENCAP_BPF,
    LWT_ENCAP_MPLS,
    LWT_ENCAP_SEG6,
    LWT_ENCAP_IOAM6,
}

unsafe fn test_lwt_encap(skel: *mut test_xdp_meta, typ: lwt_encap_type) {
    let mut tc_hook = new_bpf_tc_hook(BPF_TC_EGRESS);
    let mut tc_opts = new_bpf_tc_opts(1, 1);
    let mut lwt_prog: *mut bpf_program = null_mut();
    let mut ns: *mut netns_obj = null_mut();
    let encap: *const c_char;
    let mut pinned = false;
    let tap_ifindex: c_int;
    let mut tap_fd: c_int = -1;
    let mut ret: c_int;

    (*(*skel).bss).test_pass = false;

    match typ {
        lwt_encap_type::LWT_ENCAP_BPF => {
            encap = c"encap bpf xmit pinned /sys/fs/bpf/xdp_context_lwt_xmit via fd00::2".as_ptr();
            lwt_prog = (*skel).progs.dummy_lwt_xmit;
        }
        lwt_encap_type::LWT_ENCAP_MPLS => {
            encap = c"encap mpls 100 via inet6 fd00::2".as_ptr();
        }
        lwt_encap_type::LWT_ENCAP_SEG6 => {
            encap = c"encap seg6 mode encap segs fd00::2".as_ptr();
        }
        lwt_encap_type::LWT_ENCAP_IOAM6 => {
            encap = c"encap ioam6 mode encap tundst fd00::2 trace prealloc type 0x800000 ns 0 size 4 via fd00::2".as_ptr();
        }
    }

    if !lwt_prog.is_null() {
        unlink(LWT_PIN_PATH.as_ptr() as *const c_char);
        ret = bpf_program__pin(lwt_prog, LWT_PIN_PATH.as_ptr() as *const c_char);
        if !ASSERT_OK(ret, c"pin lwt prog".as_ptr()) {
            return;
        }
        pinned = true;
    }

    ns = netns_new(LWT_NETNS.as_ptr() as *const c_char, true);
    if !ASSERT_OK_PTR(ns, c"netns_new".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    tap_fd = open_tuntap(TAP_NAME.as_ptr() as *const c_char, true);
    if !ASSERT_GE(tap_fd as isize, 0, c"open_tuntap".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    SYS(c"close".as_ptr(), c"ip link set dev tap0 address 02:00:00:00:00:01".as_ptr());
    SYS(c"close".as_ptr(), c"sysctl -wq net.ipv6.conf.all.forwarding=1".as_ptr());
    SYS(c"close".as_ptr(), c"ip addr add fd00::1/64 dev tap0 nodad".as_ptr());
    SYS(c"close".as_ptr(), c"ip link set dev tap0 up".as_ptr());
    SYS(c"close".as_ptr(), c"ip neigh add fd00::2 lladdr 02:00:00:00:00:02 nud permanent dev tap0".as_ptr());
    SYS(c"close".as_ptr(), c"ip -6 route add fd00:1::/64 %s dev %s".as_ptr(), encap, TAP_NAME.as_ptr() as *const c_char);

    tap_ifindex = if_nametoindex(TAP_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(tap_ifindex as isize, 0, c"if_nametoindex".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    ret = bpf_xdp_attach(tap_ifindex, bpf_program__fd((*skel).progs.ing_xdp), 0, null());
    if !ASSERT_GE(ret as isize, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    tc_hook.ifindex = tap_ifindex;
    ret = bpf_tc_hook_create(&mut tc_hook);
    if !ASSERT_OK(ret, c"bpf_tc_hook_create".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    tc_opts.prog_fd = bpf_program__fd((*skel).progs.tc_is_meta_empty);
    ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    ret = write_test_packet_udp(tap_fd);
    if !ASSERT_OK(ret, c"write_test_packet_udp".as_ptr()) {
        goto_test_lwt_encap_close(tap_fd, ns, pinned);
        return;
    }

    if !ASSERT_TRUE((*(*skel).bss).test_pass, c"test_pass".as_ptr()) {
        dump_err_stream((*skel).progs.tc_is_meta_empty);
    }

    goto_test_lwt_encap_close(tap_fd, ns, pinned);
}

unsafe fn goto_test_lwt_encap_close(tap_fd: c_int, ns: *mut netns_obj, pinned: bool) {
    if tap_fd >= 0 {
        close(tap_fd);
    }
    netns_free(ns);
    if pinned {
        unlink(LWT_PIN_PATH.as_ptr() as *const c_char);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_context_lwt_encap() {
    let skel: *mut test_xdp_meta;

    skel = test_xdp_meta__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open and load skeleton".as_ptr()) {
        return;
    }

    if test__start_subtest(c"bpf_encap".as_ptr()) {
        test_lwt_encap(skel, lwt_encap_type::LWT_ENCAP_BPF);
    }
    if test__start_subtest(c"mpls_encap".as_ptr()) {
        test_lwt_encap(skel, lwt_encap_type::LWT_ENCAP_MPLS);
    }
    if test__start_subtest(c"seg6_encap".as_ptr()) {
        test_lwt_encap(skel, lwt_encap_type::LWT_ENCAP_SEG6);
    }
    if test__start_subtest(c"ioam6_encap".as_ptr()) {
        test_lwt_encap(skel, lwt_encap_type::LWT_ENCAP_IOAM6);
    }

    test_xdp_meta__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
