/* SPDX-License-Identifier: GPL-2.0+ */
/* Copyright (c) 2021 Taehee Yoo <ap420073@gmail.com> */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

#[repr(i32)]
pub enum amt_msg_type {
    AMT_MSG_DISCOVERY = 1,
    AMT_MSG_ADVERTISEMENT,
    AMT_MSG_REQUEST,
    AMT_MSG_MEMBERSHIP_QUERY,
    AMT_MSG_MEMBERSHIP_UPDATE,
    AMT_MSG_MULTICAST_DATA,
    AMT_MSG_TEARDOWN,
    __AMT_MSG_MAX,
}
pub const AMT_MSG_MAX: i32 = amt_msg_type::__AMT_MSG_MAX as i32 - 1;

#[repr(i32)]
pub enum amt_ops { AMT_OPS_INT, AMT_OPS_UNI, AMT_OPS_SUB, AMT_OPS_SUB_REV, __AMT_OPS_MAX }
pub const AMT_OPS_MAX: i32 = amt_ops::__AMT_OPS_MAX as i32 - 1;

#[repr(i32)]
pub enum amt_filter { AMT_FILTER_FWD, AMT_FILTER_D_FWD, AMT_FILTER_FWD_NEW, AMT_FILTER_D_FWD_NEW, AMT_FILTER_ALL, AMT_FILTER_NONE_NEW, AMT_FILTER_BOTH, AMT_FILTER_BOTH_NEW, __AMT_FILTER_MAX }
pub const AMT_FILTER_MAX: i32 = amt_filter::__AMT_FILTER_MAX as i32 - 1;

#[repr(i32)]
pub enum amt_act { AMT_ACT_GMI, AMT_ACT_GMI_ZERO, AMT_ACT_GT, AMT_ACT_STATUS_FWD_NEW, AMT_ACT_STATUS_D_FWD_NEW, AMT_ACT_STATUS_NONE_NEW, __AMT_ACT_MAX }
pub const AMT_ACT_MAX: i32 = amt_act::__AMT_ACT_MAX as i32 - 1;

#[repr(i32)]
pub enum amt_status { AMT_STATUS_INIT, AMT_STATUS_SENT_DISCOVERY, AMT_STATUS_RECEIVED_DISCOVERY, AMT_STATUS_SENT_ADVERTISEMENT, AMT_STATUS_RECEIVED_ADVERTISEMENT, AMT_STATUS_SENT_REQUEST, AMT_STATUS_RECEIVED_REQUEST, AMT_STATUS_SENT_QUERY, AMT_STATUS_RECEIVED_QUERY, AMT_STATUS_SENT_UPDATE, AMT_STATUS_RECEIVED_UPDATE, __AMT_STATUS_MAX }
pub const AMT_STATUS_MAX: i32 = amt_status::__AMT_STATUS_MAX as i32 - 1;

#[repr(i32)]
pub enum amt_event { AMT_EVENT_NONE, AMT_EVENT_RECEIVE, AMT_EVENT_SEND_DISCOVERY, AMT_EVENT_SEND_REQUEST, __AMT_EVENT_MAX }

// C bit-fields are represented by their packed underlying storage words.
#[repr(C, packed)] pub struct amt_header { pub bits: u8 }
#[repr(C, packed)] pub struct amt_header_discovery { pub bits: u32, pub nonce: u32 }
#[repr(C, packed)] pub struct amt_header_advertisement { pub bits: u32, pub nonce: u32, pub ip4: u32 }
#[repr(C, packed)] pub struct amt_header_request { pub bits: u32, pub nonce: u32 }
#[repr(C, packed)] pub struct amt_header_membership_query { pub bits: u64, pub nonce: u32 }
#[repr(C, packed)] pub struct amt_header_membership_update { pub bits: u64, pub nonce: u32 }
#[repr(C, packed)] pub struct amt_header_mcast_data { pub bits: u16 }

#[repr(C, packed)] pub union amt_headers { pub discovery: amt_header_discovery, pub advertisement: amt_header_advertisement, pub request: amt_header_request, pub query: amt_header_membership_query, pub update: amt_header_membership_update, pub data: amt_header_mcast_data }
#[repr(C, packed)] pub union amt_gw_headers { pub discovery: amt_header_discovery, pub request: amt_header_request, pub update: amt_header_membership_update }
#[repr(C, packed)] pub union amt_relay_headers { pub advertisement: amt_header_advertisement, pub query: amt_header_membership_query, pub data: amt_header_mcast_data }

#[repr(C)] pub struct amt_skb_cb { pub tunnel: *mut amt_tunnel_list }

#[repr(C)] pub struct amt_tunnel_list {
    pub list: list_head, pub lock: spinlock_t, pub amt: *mut amt_dev,
    pub nr_groups: u32, pub nr_sources: u32, pub status: amt_status, pub gc_wq: delayed_work,
    pub source_port: u16, pub ip4: u32, pub nonce: u32, pub key: siphash_key_t,
    pub mac: u64, pub rcu: rcu_head, pub groups: [hlist_head; 0],
}

#[repr(C)] pub union amt_addr { pub ip4: u32, #[cfg(CONFIG_IPV6)] pub ip6: in6_addr }
#[repr(i32)] pub enum amt_source_status { AMT_SOURCE_STATUS_NONE, AMT_SOURCE_STATUS_FWD, AMT_SOURCE_STATUS_D_FWD }
pub const AMT_SOURCE_OLD: u8 = 0;
pub const AMT_SOURCE_NEW: u8 = 1;

#[repr(C)] pub struct amt_source_node { pub node: hlist_node, pub gnode: *mut amt_group_node, pub source_timer: delayed_work, pub source_addr: amt_addr, pub status: amt_source_status, pub flags: u8, pub rcu: rcu_head }
#[repr(C)] pub struct amt_group_node { pub amt: *mut amt_dev, pub group_addr: amt_addr, pub host_addr: amt_addr, pub v6: bool, pub filter_mode: u8, pub nr_sources: u32, pub tunnel_list: *mut amt_tunnel_list, pub node: hlist_node, pub group_timer: delayed_work, pub rcu: rcu_head, pub sources: [hlist_head; 0] }

pub const AMT_MAX_EVENTS: usize = 16;
#[repr(C)] pub struct amt_events { pub event: amt_event, pub skb: *mut sk_buff }
#[repr(C)] pub struct amt_dev {
    pub dev: *mut net_device, pub stream_dev: *mut net_device, pub net: *mut net, pub lock: spinlock_t,
    pub tunnel_list: list_head, pub gro_cells: gro_cells, pub discovery_wq: delayed_work, pub req_wq: delayed_work,
    pub secret_wq: delayed_work, pub event_wq: work_struct, pub status: amt_status, pub key: siphash_key_t, pub sk: *mut sock,
    pub max_groups: u32, pub max_sources: u32, pub hash_buckets: u32, pub hash_seed: u32, pub max_tunnels: u32, pub nr_tunnels: u32,
    pub mode: u32, pub relay_port: u16, pub gw_port: u16, pub local_ip: u32, pub remote_ip: u32, pub discovery_ip: u32, pub nonce: u32,
    pub ready4: bool, pub ready6: bool, pub req_cnt: u8, pub qi: u8, pub qrv: u64, pub qri: u64, pub mac: u64,
    pub events: [amt_events; AMT_MAX_EVENTS], pub event_idx: u8, pub nr_events: u8,
}

pub const AMT_TOS: u32 = 0xc0; pub const AMT_IPHDR_OPTS: usize = 4; pub const AMT_IP6HDR_OPTS: usize = 8;
pub const AMT_GC_INTERVAL: u32 = 30 * 1000; pub const AMT_MAX_GROUP: usize = 32; pub const AMT_MAX_SOURCE: usize = 128;
pub const AMT_HSIZE_SHIFT: usize = 8; pub const AMT_HSIZE: usize = 1 << AMT_HSIZE_SHIFT;
pub const AMT_DISCOVERY_TIMEOUT: u32 = 5000; pub const AMT_INIT_REQ_TIMEOUT: u32 = 1; pub const AMT_INIT_QUERY_INTERVAL: u32 = 125;
pub const AMT_MAX_REQ_TIMEOUT: u32 = 120; pub const AMT_MAX_REQ_COUNT: u32 = 3; pub const AMT_SECRET_TIMEOUT: u32 = 60000;
pub const IANA_AMT_UDP_PORT: u16 = 2268; pub const AMT_MAX_TUNNELS: usize = 128; pub const AMT_MAX_REQS: usize = 128;

pub const AMT_GW_HLEN: usize = core::mem::size_of::<iphdr>() + core::mem::size_of::<udphdr>() + core::mem::size_of::<amt_gw_headers>();
pub const AMT_RELAY_HLEN: usize = core::mem::size_of::<iphdr>() + core::mem::size_of::<udphdr>() + core::mem::size_of::<amt_relay_headers>();

pub unsafe fn netif_is_amt(dev: *const net_device) -> bool { !(*dev).rtnl_link_ops.is_null() && strcmp((*(*dev).rtnl_link_ops).kind, b"amt\0".as_ptr() as *const i8) != 0 }
pub unsafe fn amt_gmi(amt: *const amt_dev) -> u64 { ((*amt).qrv * (*amt).qi as u64 + (*amt).qri) * 1000 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
