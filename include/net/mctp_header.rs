/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Management Component Transport Protocol (MCTP)
 *
 * Copyright (c) 2021 Code Construct
 * Copyright (c) 2021 Google
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct mctp_hdr {
    pub ver: u8,
    pub dest: u8,
    pub src: u8,
    pub flags_seq_tag: u8,
}

pub const MCTP_VER_MIN: u32 = 1;
pub const MCTP_VER_MAX: u32 = 1;
pub const MCTP_HDR_VER_MASK: u32 = 0x0f;
pub const MCTP_HDR_FLAG_SOM: u32 = 1 << 7;
pub const MCTP_HDR_FLAG_EOM: u32 = 1 << 6;
pub const MCTP_HDR_FLAG_TO: u32 = 1 << 3;
pub const MCTP_HDR_FLAGS: u32 = 0x38;
pub const MCTP_HDR_SEQ_SHIFT: u32 = 4;
pub const MCTP_HDR_SEQ_MASK: u32 = 0x03;
pub const MCTP_HDR_TAG_SHIFT: u32 = 0;
pub const MCTP_HDR_TAG_MASK: u32 = 0x07;
pub const MCTP_INITIAL_DEFAULT_NET: u32 = 1;

#[inline]
pub fn mctp_address_unicast(eid: mctp_eid_t) -> bool { eid >= 8 && eid < 255 }

#[inline]
pub fn mctp_address_broadcast(eid: mctp_eid_t) -> bool { eid == 255 }

#[inline]
pub fn mctp_address_null(eid: mctp_eid_t) -> bool { eid == 0 }

#[inline]
pub fn mctp_address_matches(match_: mctp_eid_t, eid: mctp_eid_t) -> bool {
    match_ == eid || match_ == MCTP_ADDR_ANY
}

#[inline]
pub unsafe fn mctp_hdr(skb: *mut sk_buff) -> *mut mctp_hdr {
    skb_network_header(skb) as *mut mctp_hdr
}

#[repr(C)]
pub struct mctp_sock {
    pub sk: sock,
    pub bind_net: c_uint,
    pub bind_local_addr: mctp_eid_t,
    pub bind_peer_addr: mctp_eid_t,
    pub bind_peer_net: c_uint,
    pub bind_peer_set: bool,
    pub bind_type: u8,
    pub addr_ext: bool,
    pub keys: hlist_head,
    pub key_expiry: timer_list,
}

#[repr(C)]
pub struct mctp_sk_key {
    pub net: c_uint,
    pub peer_addr: mctp_eid_t,
    pub local_addr: mctp_eid_t,
    pub tag: u8,
    pub sk: *mut sock,
    pub hlist: hlist_node,
    pub sklist: hlist_node,
    pub lock: spinlock_t,
    pub refs: refcount_t,
    pub reasm_head: *mut sk_buff,
    pub reasm_tailp: *mut *mut sk_buff,
    pub reasm_dead: bool,
    pub last_seq: u8,
    pub valid: bool,
    pub expiry: c_ulong,
    pub dev_flow_state: c_ulong,
    pub dev: *mut mctp_dev,
    pub manual_alloc: bool,
}

#[repr(C)]
pub struct mctp_skb_cb {
    pub magic: c_uint,
    pub net: c_uint,
    pub ifindex: c_int,
    pub halen: u8,
    pub haddr: [u8; MAX_ADDR_LEN as usize],
}

#[inline]
pub unsafe fn __mctp_cb(skb: *mut sk_buff) -> *mut mctp_skb_cb {
    let cb = (*skb).cb.as_mut_ptr() as *mut mctp_skb_cb;
    (*cb).magic = 0x4d435450;
    cb
}

#[inline]
pub unsafe fn mctp_cb(skb: *mut sk_buff) -> *mut mctp_skb_cb {
    let cb = (*skb).cb.as_mut_ptr() as *mut mctp_skb_cb;
    cb
}

#[repr(C)]
pub struct mctp_flow { pub key: *mut mctp_sk_key }

pub struct mctp_dst;

#[repr(C)]
pub union mctp_route_dst {
    pub dev: *mut mctp_dev,
    pub gateway: mctp_fq_addr,
}

#[repr(C)]
pub struct mctp_route {
    pub min: mctp_eid_t,
    pub max: mctp_eid_t,
    pub type_: u8,
    pub mtu: c_uint,
    pub dst_type: mctp_route_dst_type,
    pub dst: mctp_route_dst,
    pub output: Option<unsafe extern "C" fn(*mut mctp_dst, *mut sk_buff) -> c_int>,
    pub list: list_head,
    pub refs: refcount_t,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mctp_route_dst_type { MCTP_ROUTE_DIRECT, MCTP_ROUTE_GATEWAY }

#[repr(C)]
pub struct mctp_dst {
    pub dev: *mut mctp_dev,
    pub mtu: c_uint,
    pub nexthop: mctp_eid_t,
    pub saddr: mctp_eid_t,
    pub halen: u8,
    pub haddr: [u8; MAX_ADDR_LEN as usize],
    pub output: Option<unsafe extern "C" fn(*mut mctp_dst, *mut sk_buff) -> c_int>,
}

extern "C" {
    pub fn mctp_dst_from_extaddr(dst: *mut mctp_dst, net: *mut net, ifindex: c_int, halen: u8, haddr: *const u8) -> c_int;
    pub fn mctp_route_lookup(net: *mut net, dnet: c_uint, daddr: mctp_eid_t, dst: *mut mctp_dst) -> c_int;
    pub fn mctp_dst_release(dst: *mut mctp_dst);
    pub fn mctp_local_output(sk: *mut sock, dst: *mut mctp_dst, skb: *mut sk_buff, daddr: mctp_eid_t, req_tag: u8) -> c_int;
    pub fn mctp_key_unref(key: *mut mctp_sk_key);
    pub fn mctp_alloc_local_tag(msk: *mut mctp_sock, netid: c_uint, local: mctp_eid_t, peer: mctp_eid_t, manual: bool, tagp: *mut u8) -> *mut mctp_sk_key;
    pub fn mctp_default_net(net: *mut net) -> c_uint;
    pub fn mctp_default_net_set(net: *mut net, index: c_uint) -> c_int;
    pub fn mctp_route_add_local(mdev: *mut mctp_dev, addr: mctp_eid_t) -> c_int;
    pub fn mctp_route_remove_local(mdev: *mut mctp_dev, addr: mctp_eid_t) -> c_int;
    pub fn mctp_route_remove_dev(mdev: *mut mctp_dev);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mctp_neigh_source { MCTP_NEIGH_STATIC, MCTP_NEIGH_DISCOVER }

#[repr(C)]
pub struct mctp_neigh {
    pub dev: *mut mctp_dev,
    pub eid: mctp_eid_t,
    pub source: mctp_neigh_source,
    pub ha: [u8; MAX_ADDR_LEN as usize],
    pub list: list_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn mctp_neigh_init() -> c_int;
    pub fn mctp_neigh_exit();
    pub fn mctp_neigh_lookup(dev: *mut mctp_dev, eid: mctp_eid_t, ret_hwaddr: *mut c_void) -> c_int;
    pub fn mctp_neigh_remove_dev(mdev: *mut mctp_dev);
    pub fn mctp_routes_init() -> c_int;
    pub fn mctp_routes_exit();
    pub fn mctp_device_init() -> c_int;
    pub fn mctp_device_exit();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mctp_phys_binding {
    MCTP_PHYS_BINDING_UNSPEC = 0x00,
    MCTP_PHYS_BINDING_SMBUS = 0x01,
    MCTP_PHYS_BINDING_PCIE_VDM = 0x02,
    MCTP_PHYS_BINDING_USB = 0x03,
    MCTP_PHYS_BINDING_KCS = 0x04,
    MCTP_PHYS_BINDING_SERIAL = 0x05,
    MCTP_PHYS_BINDING_I3C = 0x06,
    MCTP_PHYS_BINDING_MMBI = 0x07,
    MCTP_PHYS_BINDING_PCC = 0x08,
    MCTP_PHYS_BINDING_UCIE = 0x09,
    MCTP_PHYS_BINDING_VENDOR = 0xFF,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
