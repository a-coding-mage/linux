/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET An implementation of the TCP/IP protocol suite for the LINUX
 * operating system. Definitions for the UDP protocol.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[inline]
pub unsafe fn udp_hdr(skb: *const sk_buff) -> *mut udphdr {
    skb_transport_header(skb) as *mut udphdr
}

#[inline]
pub unsafe fn udp_get_len(skb: *const sk_buff, uh: *const udphdr, dataoff: u32) -> u32 {
    if (*uh).len != 0 {
        ntohs((*uh).len) as u32
    } else if skb_is_gso(skb) {
        (*skb).len.wrapping_sub(dataoff)
    } else {
        0
    }
}

#[inline]
pub unsafe fn udp_get_len_short(uh: *const udphdr) -> u32 {
    ntohs((*uh).len) as u32
}

#[inline]
pub unsafe fn udp_set_len(uh: *mut udphdr, len: u32) {
    (*uh).len = if len < GRO_LEGACY_MAX_SIZE { htons(len as u16) } else { 0 };
}

#[inline]
pub unsafe fn udp_set_len_short(uh: *mut udphdr, len: u32) {
    DEBUG_NET_WARN_ON_ONCE(len >= GRO_LEGACY_MAX_SIZE);
    (*uh).len = htons(len as u16);
}

pub const UDP_HTABLE_SIZE_MIN_PERNET: u32 = 128;
// IS_ENABLED(CONFIG_BASE_SMALL) is a build-time configuration condition.
pub const UDP_HTABLE_SIZE_MIN: u32 = if IS_ENABLED_CONFIG_BASE_SMALL { 128 } else { 256 };
pub const UDP_HTABLE_SIZE_MAX: u32 = 65536;

#[inline]
pub unsafe fn udp_hashfn(net: *const net, num: u32, mask: u32) -> u32 {
    num.wrapping_add(net_hash_mix(net)) & mask
}

pub const UDP_FLAGS_CORK: u32 = 0;
pub const UDP_FLAGS_NO_CHECK6_TX: u32 = 1;
pub const UDP_FLAGS_NO_CHECK6_RX: u32 = 2;
pub const UDP_FLAGS_GRO_ENABLED: u32 = 3;
pub const UDP_FLAGS_ACCEPT_FRAGLIST: u32 = 4;
pub const UDP_FLAGS_ACCEPT_L4: u32 = 5;
pub const UDP_FLAGS_ENCAP_ENABLED: u32 = 6;

#[repr(C)]
pub struct udp_prod_queue {
    pub ll_root: llist_head,
    pub rmem_alloc: atomic_t,
}

#[repr(C)]
pub struct udp_sock {
    pub inet: inet_sock,
    pub udp_flags: c_ulong,
    pub pending: c_int,
    pub encap_type: u8,
    // Present only when !IS_ENABLED(CONFIG_BASE_SMALL).
    pub udp_lrpa_hash: u16,
    pub udp_lrpa_node: hlist_nulls_node,
    pub len: u16,
    pub gso_size: u16,
    pub encap_rcv: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int>,
    pub encap_err_rcv: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff, c_int, __be16, u32, *mut u8)>,
    pub encap_err_lookup: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int>,
    pub encap_destroy: Option<unsafe extern "C" fn(*mut sock)>,
    pub gro_receive: Option<unsafe extern "C" fn(*mut sock, *mut list_head, *mut sk_buff) -> *mut sk_buff>,
    pub gro_complete: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff, c_int) -> c_int>,
    pub udp_prod_queue: *mut udp_prod_queue,
    pub reader_queue: sk_buff_head,
    pub forward_deficit: c_int,
    pub forward_threshold: c_int,
    pub peeking_with_offset: bool,
    pub tunnel_list: hlist_node,
    pub drop_counters: numa_drop_counters,
}

#[macro_export]
macro_rules! udp_test_bit { ($nr:ident, $sk:expr) => { test_bit(concat_idents!(UDP_FLAGS_, $nr), &mut udp_sk($sk).udp_flags) }; }
#[macro_export]
macro_rules! udp_set_bit { ($nr:ident, $sk:expr) => { set_bit(concat_idents!(UDP_FLAGS_, $nr), &mut udp_sk($sk).udp_flags) }; }
#[macro_export]
macro_rules! udp_test_and_set_bit { ($nr:ident, $sk:expr) => { test_and_set_bit(concat_idents!(UDP_FLAGS_, $nr), &mut udp_sk($sk).udp_flags) }; }
#[macro_export]
macro_rules! udp_clear_bit { ($nr:ident, $sk:expr) => { clear_bit(concat_idents!(UDP_FLAGS_, $nr), &mut udp_sk($sk).udp_flags) }; }
#[macro_export]
macro_rules! udp_assign_bit { ($nr:ident, $sk:expr, $val:expr) => { assign_bit(concat_idents!(UDP_FLAGS_, $nr), &mut udp_sk($sk).udp_flags, $val) }; }

pub const UDP_MAX_SEGMENTS: usize = 1usize << 7;

#[inline]
pub unsafe fn udp_sk(ptr: *mut sock) -> *mut udp_sock { container_of_const(ptr, udp_sock, inet.sk) }

#[inline]
pub unsafe fn udp_set_peek_off(sk: *mut sock, val: c_int) -> c_int {
    sk_set_peek_off(sk, val);
    WRITE_ONCE((*udp_sk(sk)).peeking_with_offset, val >= 0);
    0
}

#[inline]
pub unsafe fn udp_set_no_check6_tx(sk: *mut sock, val: bool) { udp_assign_bit!(NO_CHECK6_TX, sk, val); }
#[inline]
pub unsafe fn udp_set_no_check6_rx(sk: *mut sock, val: bool) { udp_assign_bit!(NO_CHECK6_RX, sk, val); }
#[inline]
pub unsafe fn udp_get_no_check6_tx(sk: *const sock) -> bool { udp_test_bit!(NO_CHECK6_TX, sk) }
#[inline]
pub unsafe fn udp_get_no_check6_rx(sk: *const sock) -> bool { udp_test_bit!(NO_CHECK6_RX, sk) }

#[inline]
pub unsafe fn udp_cmsg_recv(msg: *mut msghdr, sk: *mut sock, skb: *mut sk_buff) {
    let mut gso_size: c_int;
    if skb_shinfo(skb).gso_type & SKB_GSO_UDP_L4 != 0 {
        gso_size = skb_shinfo(skb).gso_size as c_int;
        put_cmsg(msg, SOL_UDP, UDP_GRO, core::mem::size_of_val(&gso_size), &mut gso_size as *mut _ as *mut _);
    }
}

// DECLARE_STATIC_KEY_FALSE(udp_encap_needed_key);
// DECLARE_STATIC_KEY_FALSE(udpv6_encap_needed_key) when CONFIG_IPV6 is enabled.

#[inline]
pub unsafe fn udp_encap_needed() -> bool {
    if static_branch_unlikely(&udp_encap_needed_key) { return true; }
    // CONFIG_IPV6 conditional retained from the source header.
    if IS_ENABLED_CONFIG_IPV6 && static_branch_unlikely(&udpv6_encap_needed_key) { return true; }
    false
}

#[inline]
pub unsafe fn udp_unexpected_gso(sk: *mut sock, skb: *mut sk_buff) -> bool {
    if !skb_is_gso(skb) { return false; }
    if skb_shinfo(skb).gso_type & SKB_GSO_UDP_L4 != 0 && !udp_test_bit!(ACCEPT_L4, sk) { return true; }
    if skb_shinfo(skb).gso_type & SKB_GSO_FRAGLIST != 0 && !udp_test_bit!(ACCEPT_FRAGLIST, sk) { return true; }
    if udp_encap_needed() && READ_ONCE((*udp_sk(sk)).encap_rcv).is_some()
        && skb_shinfo(skb).gso_type & (SKB_GSO_UDP_TUNNEL | SKB_GSO_UDP_TUNNEL_CSUM) == 0 { return true; }
    false
}

#[inline]
pub unsafe fn udp_allow_gso(sk: *mut sock) {
    udp_set_bit!(ACCEPT_L4, sk);
    udp_set_bit!(ACCEPT_FRAGLIST, sk);
}

// hlist iteration macros from the source are provided by the surrounding kernel translation.

#[inline]
pub unsafe fn udp_tunnel_sk(net: *const net, is_ipv6: bool) -> *mut sock {
    if IS_ENABLED_CONFIG_NET_UDP_TUNNEL { rcu_dereference((*net).ipv4.udp_tunnel_gro[is_ipv6 as usize].sk) } else { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
