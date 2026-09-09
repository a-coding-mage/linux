/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than redefined.
 */

/* IPv6 transport protocols */
unsafe extern "C" {
    pub static mut rawv6_prot: struct proto;
    pub static mut udpv6_prot: struct proto;
    pub static mut tcpv6_prot: struct proto;
    pub static mut pingv6_prot: struct proto;
}

/* extension headers */
unsafe extern "C" {
    pub fn ipv6_exthdrs_init() -> ::core::ffi::c_int;
    pub fn ipv6_exthdrs_exit();
    pub fn ipv6_frag_init() -> ::core::ffi::c_int;
    pub fn ipv6_frag_exit();
}

/* transport protocols */
unsafe extern "C" {
    pub fn pingv6_init() -> ::core::ffi::c_int;
    pub fn pingv6_exit();
    pub fn rawv6_init() -> ::core::ffi::c_int;
    pub fn rawv6_exit();
    pub fn udpv6_init() -> ::core::ffi::c_int;
    pub fn udpv6_exit();
    pub fn tcpv6_init() -> ::core::ffi::c_int;
    pub fn tcpv6_exit();
}

/* this does all the common and the specific ctl work */
unsafe extern "C" {
    pub fn ip6_datagram_recv_ctl(
        sk: *mut struct sock,
        msg: *mut struct msghdr,
        skb: *mut struct sk_buff,
    );
    pub fn ip6_datagram_recv_common_ctl(
        sk: *mut struct sock,
        msg: *mut struct msghdr,
        skb: *mut struct sk_buff,
    );
    pub fn ip6_datagram_recv_specific_ctl(
        sk: *mut struct sock,
        msg: *mut struct msghdr,
        skb: *mut struct sk_buff,
    );

    pub fn ip6_datagram_send_ctl(
        net: *mut struct net,
        sk: *mut struct sock,
        msg: *mut struct msghdr,
        fl6: *mut struct flowi6,
        ipc6: *mut struct ipcm6_cookie,
    ) -> ::core::ffi::c_int;

    pub fn __ip6_dgram_sock_seq_show(
        seq: *mut struct seq_file,
        sp: *mut struct sock,
        srcp: __u16,
        destp: __u16,
        rqueue: ::core::ffi::c_int,
        bucket: ::core::ffi::c_int,
    );
}

#[inline]
pub unsafe fn ip6_dgram_sock_seq_show(
    seq: *mut struct seq_file,
    sp: *mut struct sock,
    srcp: __u16,
    destp: __u16,
    bucket: ::core::ffi::c_int,
) {
    __ip6_dgram_sock_seq_show(seq, sp, srcp, destp, sk_rmem_alloc_get(sp), bucket);
}

/* Equivalent to LOOPBACK4_IPV6 cpu_to_be32(0x7f000006). */
pub const LOOPBACK4_IPV6: u32 = 0x7f000006u32.to_be();

pub const IPV6_SEQ_DGRAM_HEADER: &str =
    "  sl  "
    "local_address                         "
    "remote_address                        "
    "st tx_queue rx_queue tr tm->when retrnsmt"
    "   uid  timeout inode ref pointer drops\n";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
