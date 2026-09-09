/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from tcp_ao.h. Required kernel types and constants are external. */

#[repr(C)]
pub union tcp_ao_addr {
    pub a4: in_addr,
    #[cfg(feature = "CONFIG_IPV6")]
    pub a6: in6_addr,
}

#[repr(C)]
pub struct tcp_ao_hdr { pub kind: u8, pub length: u8, pub keyid: u8, pub rnext_keyid: u8 }

#[inline]
pub unsafe fn tcp_ao_hdr_maclen(aoh: *const tcp_ao_hdr) -> u8 {
    (*aoh).length.wrapping_sub(core::mem::size_of::<tcp_ao_hdr>() as u8)
}

#[repr(C)]
pub struct tcp_ao_counters { pub pkt_good: atomic64_t, pub pkt_bad: atomic64_t, pub key_not_found: atomic64_t, pub ao_required: atomic64_t, pub dropped_icmp: atomic64_t }

#[repr(C)]
pub enum tcp_ao_algo_id { TCP_AO_ALGO_HMAC_SHA1 = 1, TCP_AO_ALGO_HMAC_SHA256, TCP_AO_ALGO_AES_128_CMAC }

pub const TCP_AO_MAX_MAC_LEN: usize = SHA256_DIGEST_SIZE;
pub const TCP_AO_MAX_TRAFFIC_KEY_LEN: usize = SHA256_DIGEST_SIZE;

pub enum tcp_ao_mac_ctx {}

#[repr(C)]
pub struct tcp_ao_key {
    pub node: hlist_node,
    pub addr: tcp_ao_addr,
    pub key: [u8; TCP_AO_MAXKEYLEN],
    pub algo: tcp_ao_algo_id,
    pub digest_size: core::ffi::c_uint,
    pub l3index: core::ffi::c_int,
    pub prefixlen: u8, pub family: u8, pub keylen: u8, pub keyflags: u8,
    pub sndid: u8, pub rcvid: u8, pub maclen: u8,
    pub rcu: rcu_head,
    pub pkt_good: atomic64_t, pub pkt_bad: atomic64_t,
    pub traffic_keys: [u8; 0],
}

#[inline] pub unsafe fn rcv_other_key(key: *mut tcp_ao_key) -> *mut u8 { (*key).traffic_keys.as_mut_ptr() }
#[inline] pub unsafe fn snd_other_key(key: *mut tcp_ao_key) -> *mut u8 { (*key).traffic_keys.as_mut_ptr().add((*key).digest_size as usize) }
#[inline] pub unsafe fn tcp_ao_maclen(key: *const tcp_ao_key) -> core::ffi::c_int { (*key).maclen as core::ffi::c_int }
#[inline] pub unsafe fn tcp_ao_len(key: *const tcp_ao_key) -> core::ffi::c_int { tcp_ao_maclen(key) + core::mem::size_of::<tcp_ao_hdr>() as i32 }
#[inline] pub unsafe fn tcp_ao_len_aligned(key: *const tcp_ao_key) -> core::ffi::c_int { (tcp_ao_len(key) + 3) & !3 }
#[inline] pub unsafe fn tcp_ao_digest_size(key: *mut tcp_ao_key) -> core::ffi::c_uint { (*key).digest_size }
#[inline] pub unsafe fn tcp_ao_sizeof_key(key: *const tcp_ao_key) -> usize { core::mem::size_of::<tcp_ao_key>() + ((*key).digest_size as usize).wrapping_shl(1) }

#[repr(C)]
pub struct tcp_ao_info {
    pub head: hlist_head, pub current_key: *mut tcp_ao_key, pub rnext_key: *mut tcp_ao_key,
    pub counters: tcp_ao_counters, pub ao_required: u32, pub accept_icmps: u32,
    pub lisn: __be32, pub risn: __be32, pub snd_sne: u32, pub rcv_sne: u32,
    pub refcnt: refcount_t, pub rcu: rcu_head,
}

#[cfg(feature = "CONFIG_TCP_MD5SIG")]
extern "C" { pub static mut tcp_md5_needed: static_key_false_deferred; }
#[cfg(feature = "CONFIG_TCP_MD5SIG")]
#[inline] pub unsafe fn static_branch_tcp_md5() -> bool { static_branch_unlikely(&mut tcp_md5_needed.key) }
#[cfg(not(feature = "CONFIG_TCP_MD5SIG"))]
#[inline] pub const fn static_branch_tcp_md5() -> bool { false }

#[cfg(feature = "CONFIG_TCP_AO")]
extern "C" { pub static mut tcp_ao_needed: static_key_false_deferred; }
#[cfg(feature = "CONFIG_TCP_AO")]
#[inline] pub unsafe fn static_branch_tcp_ao() -> bool { static_branch_unlikely(&mut tcp_ao_needed.key) }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn static_branch_tcp_ao() -> bool { false }

#[cfg(feature = "CONFIG_TCP_AO")]
#[repr(C)] pub struct tcp4_ao_context { pub saddr: __be32, pub daddr: __be32, pub sport: __be16, pub dport: __be16, pub sisn: __be32, pub disn: __be32 }
#[cfg(feature = "CONFIG_TCP_AO")]
#[repr(C)] pub struct tcp6_ao_context { pub saddr: in6_addr, pub daddr: in6_addr, pub sport: __be16, pub dport: __be16, pub sisn: __be32, pub disn: __be32 }

#[cfg(feature = "CONFIG_TCP_AO")]
pub const TCP_AO_ESTABLISHED: u32 = TCPF_ESTABLISHED | TCPF_FIN_WAIT1 | TCPF_FIN_WAIT2 | TCPF_CLOSE_WAIT | TCPF_LAST_ACK | TCPF_CLOSING;

/* External function declarations from CONFIG_TCP_AO. */
#[cfg(feature = "CONFIG_TCP_AO")]
extern "C" {
    pub fn tcp_ao_transmit_skb(sk: *mut sock, skb: *mut sk_buff, key: *mut tcp_ao_key, th: *mut tcphdr, hash_location: *mut u8);
    pub fn tcp_ao_mac_update(mac_ctx: *mut tcp_ao_mac_ctx, data: *const core::ffi::c_void, data_len: usize);
    pub fn tcp_ao_hash_skb(family: u16, ao_hash: *mut core::ffi::c_char, key: *mut tcp_ao_key, sk: *const sock, skb: *const sk_buff, tkey: *const u8, hash_offset: i32, sne: u32) -> i32;
    pub fn tcp_parse_ao(sk: *mut sock, cmd: i32, family: u16, optval: sockptr_t, optlen: i32) -> i32;
    pub fn tcp_ao_established_key(sk: *const sock, ao: *mut tcp_ao_info, sndid: i32, rcvid: i32) -> *mut tcp_ao_key;
    pub fn tcp_ao_copy_all_matching(sk: *const sock, newsk: *mut sock, req: *mut request_sock, skb: *mut sk_buff, family: i32) -> i32;
    pub fn tcp_ao_calc_traffic_key(mkt: *const tcp_ao_key, traffic_key: *mut u8, input: *const core::ffi::c_void, input_len: u32);
    pub fn tcp_ao_time_wait(tcptw: *mut tcp_timewait_sock, tp: *mut tcp_sock);
    pub fn tcp_ao_ignore_icmp(sk: *const sock, family: i32, type_: i32, code: i32) -> bool;
    pub fn tcp_ao_get_mkts(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> i32;
    pub fn tcp_ao_get_sock_info(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> i32;
    pub fn tcp_ao_get_repair(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> i32;
    pub fn tcp_ao_set_repair(sk: *mut sock, optval: sockptr_t, optlen: u32) -> i32;
    pub fn tcp_inbound_ao_hash(sk: *mut sock, skb: *const sk_buff, family: u16, req: *const request_sock, l3index: i32, aoh: *const tcp_ao_hdr) -> skb_drop_reason;
    pub fn tcp_ao_compute_sne(next_sne: u32, next_seq: u32, seq: u32) -> u32;
    pub fn tcp_ao_do_lookup(sk: *const sock, l3index: i32, addr: *const tcp_ao_addr, family: i32, sndid: i32, rcvid: i32) -> *mut tcp_ao_key;
    pub fn tcp_ao_hash_hdr(family: u16, ao_hash: *mut core::ffi::c_char, key: *mut tcp_ao_key, tkey: *const u8, daddr: *const tcp_ao_addr, saddr: *const tcp_ao_addr, th: *const tcphdr, sne: u32) -> i32;
    pub fn tcp_ao_destroy_sock(sk: *mut sock, twsk: bool);
    pub fn tcp_ao_established(sk: *mut sock); pub fn tcp_ao_finish_connect(sk: *mut sock, skb: *mut sk_buff); pub fn tcp_ao_connect_init(sk: *mut sock);
    pub fn tcp_ao_syncookie(sk: *mut sock, skb: *const sk_buff, req: *mut request_sock, family: u16);
    pub fn tcp_v4_parse_ao(sk: *mut sock, cmd: i32, optval: sockptr_t, optlen: i32) -> i32;
    pub fn tcp_v4_ao_lookup(sk: *const sock, addr_sk: *mut sock, sndid: i32, rcvid: i32) -> *mut tcp_ao_key;
    pub fn tcp_v4_ao_hash_skb(ao_hash: *mut core::ffi::c_char, key: *mut tcp_ao_key, sk: *const sock, skb: *const sk_buff, tkey: *const u8, hash_offset: i32, sne: u32) -> i32;
    pub fn tcp_v6_ao_hash_pseudoheader(mac_ctx: *mut tcp_ao_mac_ctx, daddr: *const in6_addr, saddr: *const in6_addr, nbytes: i32);
    pub fn tcp_v6_ao_lookup(sk: *const sock, addr_sk: *mut sock, sndid: i32, rcvid: i32) -> *mut tcp_ao_key;
    pub fn tcp_v6_ao_hash_skb(ao_hash: *mut core::ffi::c_char, key: *mut tcp_ao_key, sk: *const sock, skb: *const sk_buff, tkey: *const u8, hash_offset: i32, sne: u32) -> i32;
    pub fn tcp_v6_parse_ao(sk: *mut sock, cmd: i32, optval: sockptr_t, optlen: i32) -> i32;
}

#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub unsafe fn tcp_ao_transmit_skb(_: *mut sock, _: *mut sk_buff, _: *mut tcp_ao_key, _: *mut tcphdr, _: *mut u8) {}
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn tcp_ao_ignore_icmp(_: *const sock, _: i32, _: i32, _: i32) -> bool { false }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub fn tcp_ao_do_lookup(_: *const sock, _: i32, _: *const tcp_ao_addr, _: i32, _: i32, _: i32) -> *mut tcp_ao_key { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn tcp_ao_get_mkts(_: *mut sock, _: sockptr_t, _: sockptr_t) -> i32 { -ENOPROTOOPT }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn tcp_ao_get_sock_info(_: *mut sock, _: sockptr_t, _: sockptr_t) -> i32 { -ENOPROTOOPT }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn tcp_ao_get_repair(_: *mut sock, _: sockptr_t, _: sockptr_t) -> i32 { -ENOPROTOOPT }
#[cfg(not(feature = "CONFIG_TCP_AO"))]
#[inline] pub const fn tcp_ao_set_repair(_: *mut sock, _: sockptr_t, _: u32) -> i32 { -ENOPROTOOPT }

#[cfg(any(feature = "CONFIG_TCP_MD5SIG", feature = "CONFIG_TCP_AO"))]
extern "C" { pub fn tcp_do_parse_auth_options(th: *const tcphdr, md5_hash: *mut *const u8, ao_hash: *mut *const u8) -> i32; }
#[cfg(not(any(feature = "CONFIG_TCP_MD5SIG", feature = "CONFIG_TCP_AO")))]
pub unsafe fn tcp_do_parse_auth_options(_: *const tcphdr, md5_hash: *mut *const u8, ao_hash: *mut *const u8) -> i32 { *md5_hash = core::ptr::null(); *ao_hash = core::ptr::null(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
