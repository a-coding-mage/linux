/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies:
 * arpa/inet.h, sys/socket.h, sys/types.h, linux/types.h,
 * linux/if_ether.h, linux/if_packet.h, linux/if_tun.h, linux/ip.h,
 * linux/ipv6.h, linux/ethtool.h, linux/sockios.h, linux/err.h,
 * netinet/tcp.h, netinet/udp.h, bpf/bpf_endian.h, net/if.h, stdio.h.
 */

pub const MAGIC_VAL: i32 = 0x1234;
pub const NUM_ITER: i32 = 100000;
pub const VIP_NUM: i32 = 5;
pub const MAGIC_BYTES: i32 = 123;

/* include/linux/net.h */
/* C fallback when SOCK_TYPE_MASK is not already defined. */
pub const SOCK_TYPE_MASK: i32 = 0xf;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be32 = __u32;
pub type __wsum = __u32;
pub type __sum16 = __u16;
pub type size_t = usize;
pub type socklen_t = u32;
pub type va_list = *mut core::ffi::c_void;

#[repr(C)]
pub struct network_helper_opts {
    pub timeout_ms: core::ffi::c_int,
    pub proto: core::ffi::c_int,
    /* +ve: Passed to listen() as-is.
     *   0: Default when the test does not set
     *      a particular value during the struct init.
     *      It is changed to 1 before passing to listen().
     *      Most tests only have one on-going connection.
     * -ve: It is changed to 0 before passing to listen().
     *      It is useful to force syncookie without
     *      changing the "tcp_syncookies" sysctl from 1 to 2.
     */
    pub backlog: core::ffi::c_int,
    pub post_socket_cb: Option<
        unsafe extern "C" fn(fd: core::ffi::c_int, opts: *mut core::ffi::c_void) -> core::ffi::c_int,
    >,
    pub cb_opts: *mut core::ffi::c_void,
}

/* ipv4 test vector */
#[repr(C, packed)]
pub struct ipv4_packet {
    pub eth: ethhdr,
    pub iph: iphdr,
    pub tcp: tcphdr,
}

/* ipv6 test vector */
#[repr(C, packed)]
pub struct ipv6_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub tcp: tcphdr,
}

#[repr(C)]
pub struct nstoken {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tmonitor_ctx {
    _unused: [u8; 0],
}

pub type tm_print_fn_t = Option<
    unsafe extern "C" fn(format: *const core::ffi::c_char, args: va_list) -> core::ffi::c_int,
>;

unsafe extern "C" {
    pub static mut pkt_v4: ipv4_packet;
    pub static mut pkt_v6: ipv6_packet;

    pub fn htons(hostshort: __u16) -> __u16;
    pub fn ntohs(netshort: __u16) -> __u16;

    pub fn settimeo(fd: core::ffi::c_int, timeout_ms: core::ffi::c_int) -> core::ffi::c_int;
    pub fn start_server_str(
        family: core::ffi::c_int,
        type_: core::ffi::c_int,
        addr_str: *const core::ffi::c_char,
        port: __u16,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn start_server(
        family: core::ffi::c_int,
        type_: core::ffi::c_int,
        addr: *const core::ffi::c_char,
        port: __u16,
        timeout_ms: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn start_reuseport_server(
        family: core::ffi::c_int,
        type_: core::ffi::c_int,
        addr_str: *const core::ffi::c_char,
        port: __u16,
        timeout_ms: core::ffi::c_int,
        nr_listens: core::ffi::c_uint,
    ) -> *mut core::ffi::c_int;
    pub fn start_server_addr(
        type_: core::ffi::c_int,
        addr: *const sockaddr_storage,
        len: socklen_t,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn free_fds(fds: *mut core::ffi::c_int, nr_close_fds: core::ffi::c_uint);
    pub fn client_socket(
        family: core::ffi::c_int,
        type_: core::ffi::c_int,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn connect_to_addr(
        type_: core::ffi::c_int,
        addr: *const sockaddr_storage,
        len: socklen_t,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn connect_to_addr_str(
        family: core::ffi::c_int,
        type_: core::ffi::c_int,
        addr_str: *const core::ffi::c_char,
        port: __u16,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn connect_to_fd(server_fd: core::ffi::c_int, timeout_ms: core::ffi::c_int)
        -> core::ffi::c_int;
    pub fn connect_to_fd_opts(
        server_fd: core::ffi::c_int,
        opts: *const network_helper_opts,
    ) -> core::ffi::c_int;
    pub fn connect_fd_to_fd(
        client_fd: core::ffi::c_int,
        server_fd: core::ffi::c_int,
        timeout_ms: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn fastopen_connect(
        server_fd: core::ffi::c_int,
        data: *const core::ffi::c_char,
        data_len: core::ffi::c_uint,
        timeout_ms: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn make_sockaddr(
        family: core::ffi::c_int,
        addr_str: *const core::ffi::c_char,
        port: __u16,
        addr: *mut sockaddr_storage,
        len: *mut socklen_t,
    ) -> core::ffi::c_int;
    pub fn ping_command(family: core::ffi::c_int) -> *mut core::ffi::c_char;
    pub fn get_socket_local_port(sock_fd: core::ffi::c_int) -> core::ffi::c_int;
    pub fn get_hw_ring_size(
        ifname: *mut core::ffi::c_char,
        ring_param: *mut ethtool_ringparam,
    ) -> core::ffi::c_int;
    pub fn set_hw_ring_size(
        ifname: *mut core::ffi::c_char,
        ring_param: *mut ethtool_ringparam,
    ) -> core::ffi::c_int;

    pub fn open_tuntap(dev_name: *const core::ffi::c_char, need_mac: bool) -> core::ffi::c_int;

    /**
     * open_netns() - Switch to specified network namespace by name.
     *
     * Returns token with which to restore the original namespace
     * using close_netns().
     */
    pub fn open_netns(name: *const core::ffi::c_char) -> *mut nstoken;
    pub fn close_netns(token: *mut nstoken);
    pub fn send_recv_data(
        lfd: core::ffi::c_int,
        fd: core::ffi::c_int,
        total_bytes: u32,
    ) -> core::ffi::c_int;
    pub fn make_netns(name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn remove_netns(name: *const core::ffi::c_char) -> core::ffi::c_int;

    /**
     * append_tid() - Append thread ID to the given string.
     *
     * @str: string to extend
     * @sz: string's size
     *
     * 8 characters are used to append the thread ID (7 digits + '\0')
     *
     * Returns -1 on errors, 0 otherwise
     */
    pub fn append_tid(str_: *mut core::ffi::c_char, sz: size_t) -> core::ffi::c_int;

    /**
     * tc_prog_attach - attach BPF program(s) to an interface
     *
     * Takes file descriptors pointing to at least one, at most two BPF
     * programs, and attach those programs to an interface ingress, egress or
     * both.
     *
     * @dev: string containing the interface name
     * @ingress_fd: file descriptor of the program to attach to interface ingress
     * @egress_fd: file descriptor of the program to attach to interface egress
     *
     * Returns 0 on success, -1 if no valid file descriptor has been found, if
     * the interface name is invalid or if an error ocurred during attach.
     */
    pub fn tc_prog_attach(
        dev: *const core::ffi::c_char,
        ingress_fd: core::ffi::c_int,
        egress_fd: core::ffi::c_int,
    ) -> core::ffi::c_int;

    /* Defined when the C build enables TRAFFIC_MONITOR. */
    pub fn traffic_monitor_start_enabled(
        netns: *const core::ffi::c_char,
        test_name: *const core::ffi::c_char,
        subtest_name: *const core::ffi::c_char,
    ) -> *mut tmonitor_ctx;
    pub fn traffic_monitor_stop_enabled(ctx: *mut tmonitor_ctx);
    pub fn traffic_monitor_set_print_enabled(fn_: tm_print_fn_t) -> tm_print_fn_t;
}

pub unsafe fn csum_fold(mut csum: __u32) -> __u16 {
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    csum = (csum & 0xffff).wrapping_add(csum >> 16);

    !(csum as __u16)
}

pub unsafe fn csum_partial(buf: *const core::ffi::c_void, len: core::ffi::c_int, mut sum: __wsum) -> __wsum {
    let p = buf as *const __u16;
    let num_u16 = len >> 1;
    let mut i: core::ffi::c_int = 0;

    while i < num_u16 {
        sum = sum.wrapping_add(*p.offset(i as isize) as __wsum);
        i += 1;
    }

    sum
}

pub unsafe fn build_ip_csum(iph: *mut iphdr) -> __sum16 {
    let mut sum: __u32 = 0;
    let p: *mut __u16;

    (*iph).check = 0;
    p = iph as *mut core::ffi::c_void as *mut __u16;
    sum = csum_partial(p as *const core::ffi::c_void, ((*iph).ihl as core::ffi::c_int) << 2, 0);

    csum_fold(sum)
}

/**
 * csum_tcpudp_magic - compute IP pseudo-header checksum
 *
 * Compute the IPv4 pseudo header checksum. The helper can take a
 * accumulated sum from the transport layer to accumulate it and directly
 * return the transport layer
 *
 * @saddr: IP source address
 * @daddr: IP dest address
 * @len: IP data size
 * @proto: transport layer protocol
 * @csum: The accumulated partial sum to add to the computation
 *
 * Returns the folded sum
 */
pub unsafe fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    csum: __wsum,
) -> __sum16 {
    let mut s: __u64 = csum as __u64;

    s = s.wrapping_add(saddr as __u32 as __u64);
    s = s.wrapping_add(daddr as __u32 as __u64);
    s = s.wrapping_add(htons((proto as __u32).wrapping_add(len) as __u16) as __u64);
    s = (s & 0xffffffff).wrapping_add(s >> 32);
    s = (s & 0xffffffff).wrapping_add(s >> 32);

    csum_fold(s as __u32)
}

/**
 * csum_ipv6_magic - compute IPv6 pseudo-header checksum
 *
 * Compute the ipv6 pseudo header checksum. The helper can take a
 * accumulated sum from the transport layer to accumulate it and directly
 * return the transport layer
 *
 * @saddr: IPv6 source address
 * @daddr: IPv6 dest address
 * @len: IPv6 data size
 * @proto: transport layer protocol
 * @csum: The accumulated partial sum to add to the computation
 *
 * Returns the folded sum
 */
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    csum: __wsum,
) -> __sum16 {
    let mut s: __u64 = csum as __u64;
    let mut i: core::ffi::c_int;

    i = 0;
    while i < 4 {
        s = s.wrapping_add((*saddr).s6_addr32[i as usize] as __u32 as __u64);
        i += 1;
    }
    i = 0;
    while i < 4 {
        s = s.wrapping_add((*daddr).s6_addr32[i as usize] as __u32 as __u64);
        i += 1;
    }
    s = s.wrapping_add(htons((proto as __u32).wrapping_add(len) as __u16) as __u64);
    s = (s & 0xffffffff).wrapping_add(s >> 32);
    s = (s & 0xffffffff).wrapping_add(s >> 32);

    csum_fold(s as __u32)
}

/**
 * build_udp_v4_csum - compute UDP checksum for UDP over IPv4
 *
 * Compute the checksum to embed in UDP header, composed of the sum of IP
 * pseudo-header checksum, UDP header checksum and UDP data checksum
 * @iph IP header
 * @udph UDP header, which must be immediately followed by UDP data
 *
 * Returns the total checksum
 */
pub unsafe fn build_udp_v4_csum(iph: *const iphdr, udph: *const udphdr) -> __sum16 {
    let mut sum: core::ffi::c_ulong;

    sum = csum_partial(
        udph as *const core::ffi::c_void,
        ntohs((*udph).len) as core::ffi::c_int,
        0,
    ) as core::ffi::c_ulong;
    csum_tcpudp_magic(
        (*iph).saddr,
        (*iph).daddr,
        ntohs((*udph).len) as __u32,
        IPPROTO_UDP as __u8,
        sum as __wsum,
    )
}

/**
 * build_udp_v6_csum - compute UDP checksum for UDP over IPv6
 *
 * Compute the checksum to embed in UDP header, composed of the sum of IPv6
 * pseudo-header checksum, UDP header checksum and UDP data checksum
 * @ip6h IPv6 header
 * @udph UDP header, which must be immediately followed by UDP data
 *
 * Returns the total checksum
 */
pub unsafe fn build_udp_v6_csum(ip6h: *const ipv6hdr, udph: *const udphdr) -> __sum16 {
    let mut sum: core::ffi::c_ulong;

    sum = csum_partial(
        udph as *const core::ffi::c_void,
        ntohs((*udph).len) as core::ffi::c_int,
        0,
    ) as core::ffi::c_ulong;
    csum_ipv6_magic(
        &(*ip6h).saddr,
        &(*ip6h).daddr,
        ntohs((*udph).len) as __u32,
        IPPROTO_UDP as __u8,
        sum as __wsum,
    )
}

/* C #ifdef TRAFFIC_MONITOR:
 * When enabled, traffic_monitor_start/stop/set_print are external functions.
 * Otherwise, the inline fallback implementations below return NULL/do nothing.
 */

pub unsafe fn traffic_monitor_start(
    _netns: *const core::ffi::c_char,
    _test_name: *const core::ffi::c_char,
    _subtest_name: *const core::ffi::c_char,
) -> *mut tmonitor_ctx {
    core::ptr::null_mut()
}

pub unsafe fn traffic_monitor_stop(_ctx: *mut tmonitor_ctx) {}

pub unsafe fn traffic_monitor_set_print(_fn: tm_print_fn_t) -> tm_print_fn_t {
    None
}
