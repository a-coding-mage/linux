/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Faithful Rust declaration translation of net/sctp/structs.h.
 * Kernel-provided types and constants are intentionally external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* External kernel types supplied by the surrounding translation unit. */
extern "C" {
    pub static mut sctp_globals: sctp_globals_t;
}

#[repr(C)]
pub union sctp_addr {
    pub sa: sockaddr_inet,
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}

/* Forward declarations. */
pub enum sctp_globals {}
pub enum sctp_endpoint {}
pub enum sctp_association {}
pub enum sctp_transport {}
pub enum sctp_packet {}
pub enum sctp_chunk {}
pub enum sctp_inq {}
pub enum sctp_outq {}
pub enum sctp_bind_addr {}
pub enum sctp_ulpq {}
pub enum sctp_ep_common {}
pub enum sctp_stream {}

/* C kernel types referenced by this header. */
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be16 = u16;
pub type kuid_t = u32;
pub type ktime_t = i64;
pub type gfp_t = c_uint;
pub type sa_family_t = u16;

#[repr(C)]
pub struct sctp_globals_t {
    pub address_families: list_head,
    pub ep_hashtable: *mut sctp_hashbucket,
    pub port_hashtable: *mut sctp_bind_hashbucket,
    pub transport_hashtable: rhltable,
    pub ep_hashsize: c_int,
    pub port_hashsize: c_int,
    pub max_instreams: __u16,
    pub max_outstreams: __u16,
    pub checksum_disable: bool,
}

#[repr(C)]
pub struct sctp_bind_bucket {
    pub port: u16,
    pub fastreuse: i8,
    pub fastreuseport: i8,
    pub fastuid: kuid_t,
    pub node: hlist_node,
    pub owner: hlist_head,
    pub net: *mut net,
}
#[repr(C)] pub struct sctp_bind_hashbucket { pub lock: spinlock_t, pub chain: hlist_head }
#[repr(C, align(8))] pub struct sctp_hashbucket { pub lock: rwlock_t, pub chain: hlist_head }

#[repr(C)]
pub struct sctp_sock {
    pub inet: inet_sock,
    pub r#type: sctp_socket_type,
    pub pf: *mut sctp_pf,
    pub ep: *mut sctp_endpoint,
    pub bind_hash: *mut sctp_bind_bucket,
    pub default_stream: __u16,
    pub default_ppid: __u32,
    pub default_flags: __u16,
    pub default_context: __u32,
    pub default_timetolive: __u32,
    pub default_rcv_context: __u32,
    pub max_burst: c_int,
    pub hbinterval: __u32,
    pub probe_interval: __u32,
    pub udp_port: __be16,
    pub encap_port: __be16,
    pub pathmaxrxt: __u16,
    pub flowlabel: __u32,
    pub dscp: __u8,
    pub pf_retrans: __u16,
    pub ps_retrans: __u16,
    pub pathmtu: __u32,
    pub sackdelay: __u32,
    pub sackfreq: __u32,
    pub param_flags: __u32,
    pub default_ss: __u32,
    pub subscribe: __u16,
    pub user_frag: c_int,
    pub autoclose: __u32,
    pub adaptation_ind: __u32,
    pub pd_point: __u32,
    pub pd_mode: atomic_t,
    pub pd_lobby: sk_buff_head,
    pub auto_asconf_list: list_head,
    pub do_auto_asconf: c_int,
}

#[repr(C)] pub enum sctp_socket_type { SCTP_SOCKET_UDP=0, SCTP_SOCKET_UDP_HIGH_BANDWIDTH, SCTP_SOCKET_TCP }

pub const SCTP_CAN_FRTX: u16 = 0;
pub const SCTP_NEED_FRTX: u16 = 1;
pub const SCTP_DONT_FRTX: u16 = 2;
pub const SCTP_ADDRESS_TICK_DELAY: c_int = 500;
pub const SCTP_STREAM_CLOSED: u8 = 0;
pub const SCTP_STREAM_OPEN: u8 = 1;

#[repr(C)] pub union sctp_addr_param { pub p: sctp_paramhdr, pub v4: sctp_ipv4addr_param, pub v6: sctp_ipv6addr_param }
#[repr(C)] pub union sctp_params {
    pub v: *mut c_void, pub p: *mut sctp_paramhdr, pub life: *mut sctp_cookie_preserve_param,
    pub dns: *mut sctp_hostname_param, pub cookie: *mut sctp_cookie_param,
    pub v4: *mut sctp_ipv4addr_param, pub v6: *mut sctp_ipv6addr_param,
    pub addr: *mut sctp_addr_param,
}

#[repr(C)] pub struct sctp_cookie { pub my_vtag: __u32, pub peer_vtag: __u32, pub my_ttag: __u32, pub peer_ttag: __u32, pub expiration: ktime_t, pub sinit_num_ostreams: __u16, pub sinit_max_instreams: __u16, pub initial_tsn: __u32, pub peer_addr: sctp_addr, pub my_port: __u16, pub prsctp_capable: __u8, pub padding: __u8, pub adaptation_ind: __u32, pub raw_addr_list_len: __u32 }
#[repr(C, packed)] pub struct sctp_signed_cookie { pub mac: [__u8; 64], pub __pad: __u32, pub c: sctp_cookie }

/* The remaining kernel declarations retain their exact external ABI. */
extern "C" {
    pub fn sctp_stream_init(stream: *mut sctp_stream, outcnt: __u16, incnt: __u16, gfp: gfp_t) -> c_int;
    pub fn sctp_stream_init_ext(stream: *mut sctp_stream, sid: __u16) -> c_int;
    pub fn sctp_stream_free(stream: *mut sctp_stream);
    pub fn sctp_stream_clear(stream: *mut sctp_stream);
    pub fn sctp_stream_update(stream: *mut sctp_stream, new: *mut sctp_stream);
    pub fn sctp_get_af_specific(family: sa_family_t) -> *mut sctp_af;
    pub fn sctp_register_af(af: *mut sctp_af) -> c_int;
}

/* Types supplied by included kernel headers. */
pub enum sockaddr_inet {} pub enum sockaddr_in {} pub enum sockaddr_in6 {}
pub enum inet_sock {} pub enum list_head {} pub enum hlist_node {} pub enum hlist_head {}
pub enum spinlock_t {} pub enum rwlock_t {} pub enum rhltable {} pub enum net {}
pub enum atomic_t {} pub enum sk_buff_head {} pub enum sctp_paramhdr {}
pub enum sctp_ipv4addr_param {} pub enum sctp_ipv6addr_param {}
pub enum sctp_cookie_preserve_param {} pub enum sctp_hostname_param {} pub enum sctp_cookie_param {}
pub enum sctp_af {} pub enum sctp_pf {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
