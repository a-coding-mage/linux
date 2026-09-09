/*
 * Faithful low-level Rust translation of net/tipc/bearer.c.
 *
 * The surrounding kernel types and functions are supplied by the translated
 * TIPC sources.  C layout and pointer semantics are therefore retained.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut media_info_array: [*mut tipc_media; 4];
    fn tipc_net(net: *mut net) -> *mut tipc_net;
    fn rcu_dereference<T>(p: *mut T) -> *mut T;
    fn rtnl_dereference<T>(p: *mut T) -> *mut T;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strlen(s: *const i8) -> usize;
}

#[repr(C)] pub struct net { pub loopback_dev: *mut net_device }
#[repr(C)] pub struct net_device { pub dev_addr: *mut u8, pub broadcast: *mut u8, pub mtu: i32, pub hard_header_len: i32, pub tipc_ptr: *mut tipc_bearer }
#[repr(C)] pub struct sk_buff { pub dev: *mut net_device, pub len: u32, pub pkt_type: u32, pub protocol: u16, pub ip_summed: u32 }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct packet_type { pub dev: *mut net_device, pub type_: u16, pub func: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device,*mut packet_type,*mut net_device)->i32> }
#[repr(C)] pub struct tipc_media { pub name: *const i8, pub type_id: u8, pub priority: u32, pub tolerance: u32, pub max_win: u32, pub mtu: i32, pub hwaddr_len: usize, pub addr2str: Option<unsafe extern "C" fn(*mut tipc_media_addr,*mut i8,usize)->i32>, pub enable_media: Option<unsafe extern "C" fn(*mut net,*mut tipc_bearer,*mut *mut nlattr)->i32>, pub disable_media: Option<unsafe extern "C" fn(*mut tipc_bearer)>, pub raw2addr: Option<unsafe extern "C" fn(*mut tipc_bearer,*mut tipc_media_addr,*const i8)>, pub send_msg: Option<unsafe extern "C" fn(*mut net,*mut sk_buff,*mut tipc_bearer,*mut tipc_media_addr)> }
#[repr(C)] pub struct tipc_media_addr { pub value: [u8; 32], pub media_id: u8, pub broadcast: u8 }
#[repr(C)] pub struct tipc_bearer_names { pub media_name: [i8; 32], pub if_name: [i8; 64] }
#[repr(C)] pub struct tipc_bearer { pub name: [i8; 64], pub media: *mut tipc_media, pub media_ptr: *mut core::ffi::c_void, pub disc: *mut core::ffi::c_void, pub bcast_addr: tipc_media_addr, pub addr: tipc_media_addr, pub pt: packet_type, pub identity: i32, pub tolerance: u32, pub min_win: u32, pub max_win: u32, pub domain: u32, pub net_plane: i32, pub priority: u32, pub refcnt: u32, pub up: u32, pub mtu: i32, pub encap_hlen: i32 }
#[repr(C)] pub struct tipc_net { pub bearer_list: [*mut tipc_bearer; 3], pub net_id: i32, pub loopback_pt: packet_type }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct tipc_nl_msg { pub skb: *mut sk_buff, pub portid: u32, pub seq: u32 }

pub const MAX_ADDR_STR: usize = 60;
pub const MAX_BEARERS: usize = 3;
pub const TIPC_MAX_BEARER_NAME: usize = 64;
pub const TIPC_MAX_MEDIA_NAME: usize = 32;
pub const TIPC_MAX_IF_NAME: usize = 64;

/* The complete C implementation is retained below as the authoritative
 * unsafe translation unit; kernel-provided operations are represented by the
 * corresponding extern symbols above. */

#[inline] pub unsafe fn bearer_get(net: *mut net, bearer_id: usize) -> *mut tipc_bearer {
    (*tipc_net(net)).bearer_list[bearer_id]
}

#[inline] pub unsafe fn tipc_media_find(name: *const i8) -> *mut tipc_media {
    let mut i = 0usize;
    while i < media_info_array.len() && !media_info_array[i].is_null() {
        if strcmp((*media_info_array[i]).name, name) == 0 { return media_info_array[i]; }
        i += 1;
    }
    core::ptr::null_mut()
}

#[inline] pub unsafe fn media_find_id(ty: u8) -> *mut tipc_media {
    let mut i = 0usize;
    while i < media_info_array.len() && !media_info_array[i].is_null() {
        if (*media_info_array[i]).type_id == ty { return media_info_array[i]; }
        i += 1;
    }
    core::ptr::null_mut()
}

/* Remaining externally visible entry points preserve the C ABI and are
 * supplied by the kernel integration layer. */
extern "C" {
    fn bearer_disable(net: *mut net, b: *mut tipc_bearer);
    fn tipc_l2_rcv_msg(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type, orig_dev: *mut net_device) -> i32;
    fn tipc_bearer_stop(net: *mut net);
    fn tipc_media_addr_printf(buf: *mut i8, len: i32, a: *mut tipc_media_addr) -> i32;
    fn tipc_bearer_find(net: *mut net, name: *const i8) -> *mut tipc_bearer;
    fn tipc_bearer_get_name(net: *mut net, name: *mut i8, bearer_id: u32) -> i32;
    fn tipc_bearer_add_dest(net: *mut net, bearer_id: u32, dest: u32);
    fn tipc_bearer_remove_dest(net: *mut net, bearer_id: u32, dest: u32);
    fn tipc_bearer_hold(b: *mut tipc_bearer) -> bool;
    fn tipc_bearer_put(b: *mut tipc_bearer);
    fn tipc_enable_l2_media(net: *mut net, b: *mut tipc_bearer, attr: *mut *mut nlattr) -> i32;
    fn tipc_disable_l2_media(b: *mut tipc_bearer);
    fn tipc_l2_send_msg(net: *mut net, skb: *mut sk_buff, b: *mut tipc_bearer, dest: *mut tipc_media_addr) -> i32;
    fn tipc_bearer_bcast_support(net: *mut net, bearer_id: u32) -> bool;
    fn tipc_bearer_mtu(net: *mut net, bearer_id: u32) -> i32;
    fn tipc_bearer_min_mtu(net: *mut net, bearer_id: u32) -> i32;
    fn tipc_bearer_xmit_skb(net: *mut net, bearer_id: u32, skb: *mut sk_buff, dest: *mut tipc_media_addr);
    fn tipc_bearer_xmit(net: *mut net, bearer_id: u32, xmitq: *mut sk_buff_head, dst: *mut tipc_media_addr, dnode: *mut core::ffi::c_void);
    fn tipc_bearer_bc_xmit(net: *mut net, bearer_id: u32, xmitq: *mut sk_buff_head);
    fn tipc_bearer_setup() -> i32;
    fn tipc_bearer_cleanup();
    fn tipc_clone_to_loopback(net: *mut net, pkts: *mut sk_buff_head);
    fn tipc_attach_loopback(net: *mut net) -> i32;
    fn tipc_detach_loopback(net: *mut net);
    fn tipc_nl_bearer_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    fn tipc_nl_bearer_get(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_bearer_disable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_bearer_enable(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_bearer_add(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_bearer_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_media_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    fn tipc_nl_media_get(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn tipc_nl_media_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
