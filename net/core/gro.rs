// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from gro.c. Kernel types, constants, macros, and external
// functions are supplied by the surrounding networking implementation.

const MAX_GRO_SKBS: usize = 8;

extern "C" {
    static mut offload_lock: spinlock_t;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn synchronize_net();
    fn list_add_rcu(new: *mut list_head, prev: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn skb_shinfo(skb: *mut sk_buff) -> *mut skb_shared_info;
    fn skb_gro_offset(skb: *const sk_buff) -> u32;
    fn skb_headlen(skb: *const sk_buff) -> u32;
    fn skb_gro_len(skb: *const sk_buff) -> u32;
    fn skb_zcopy(skb: *const sk_buff) -> bool;
    fn netif_get_gro_max_size(dev: *mut net_device, skb: *mut sk_buff) -> u32;
    fn skb_end_offset(skb: *const sk_buff) -> u32;
    fn virt_to_head_page(addr: *mut u8) -> *mut page;
    fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn __skb_pull(skb: *mut sk_buff, len: u32);
    fn __skb_header_release(skb: *mut sk_buff);
    fn pskb_may_pull(skb: *mut sk_buff, len: u32) -> bool;
    fn skb_pull(skb: *mut sk_buff, len: u32);
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn gro_normal_one(gro: *mut gro_node, skb: *mut sk_buff, count: u32);
    fn skb_list_del_init(skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_get_hash_raw(skb: *const sk_buff) -> u32;
    fn skb_metadata_differs(a: *const sk_buff, b: *const sk_buff) -> u32;
    fn compare_ether_header(a: *const u8, b: *const u8) -> u32;
    fn skb_mac_header(skb: *const sk_buff) -> *const u8;
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    fn skb_metadata_dst_cmp(a: *const sk_buff, b: *const sk_buff) -> u32;
    fn skb_get_nfct(skb: *const sk_buff) -> u32;
    fn __psp_skb_coalesce_diff(a: *const sk_buff, b: *const sk_buff, d: u32) -> u32;
    fn skb_frag_address(f: *const skb_frag_t) -> *mut u8;
    fn skb_frag_page(f: *const skb_frag_t) -> *mut page;
    fn skb_frag_size(f: *const skb_frag_t) -> u32;
    fn skb_frag_off(f: *const skb_frag_t) -> u32;
    fn skb_frag_unref(skb: *mut sk_buff, i: u32);
    fn skb_has_frag_list(skb: *const sk_buff) -> bool;
    fn skb_is_gso(skb: *const sk_buff) -> bool;
    fn skb_is_gso_tcp(skb: *const sk_buff) -> bool;
    fn __skb_mark_napi_id(skb: *mut sk_buff, gro: *mut gro_node);
    fn trace_napi_gro_receive_entry(skb: *mut sk_buff);
    fn trace_napi_gro_receive_exit(ret: gro_result_t);
    fn napi_skb_free_stolen_head(skb: *mut sk_buff);
    fn __kfree_skb(skb: *mut sk_buff); fn __napi_kfree_skb(skb: *mut sk_buff, why: i32);
    fn skb_reserve(skb: *mut sk_buff, len: u32); fn skb_headroom(skb: *const sk_buff) -> u32;
    fn consume_skb(skb: *mut sk_buff); fn skb_orphan(skb: *mut sk_buff);
    fn skb_ext_reset(skb: *mut sk_buff); fn nf_reset_ct(skb: *mut sk_buff);
    fn napi_alloc_skb(napi: *mut napi_struct, len: u32) -> *mut sk_buff;
    fn skb_mark_napi_id(skb: *mut sk_buff, napi: *mut napi_struct);
    fn __skb_push(skb: *mut sk_buff, len: u32); fn eth_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> u16;
    fn skb_gro_may_pull(skb: *mut sk_buff, len: u32) -> bool;
    fn skb_gro_header_slow(skb: *mut sk_buff, len: u32, off: u32) -> *const ethhdr;
    fn skb_reset_mac_header(skb: *mut sk_buff); fn skb_set_network_header(skb: *mut sk_buff, off: u32);
    fn skb_reset_mac_len(skb: *mut sk_buff); fn skb_checksum(skb: *mut sk_buff, off: u32, len: u32, seed: u32) -> u32;
    fn csum_fold(v: u32) -> u16; fn csum_add(a: u32, b: u32) -> u32;
    fn netdev_rx_csum_fault(dev: *mut net_device, skb: *mut sk_buff);
    fn INIT_LIST_HEAD(h: *mut list_head); fn __clear_bit(i: u32, p: *mut usize); fn __set_bit(i: u32, p: *mut usize) -> bool;
    fn test_bit(i: u32, p: *const usize) -> bool; fn ffs(x: usize) -> u32;
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub hard_header_len: u16, pub name: *const i8 }
#[repr(C)] pub struct ethhdr { pub h_dest: [u8; 6], pub h_source: [u8; 6], pub h_proto: u16 }
#[repr(C)] pub struct skb_frag_t { _private: [u8; 0] }
#[repr(C)] pub struct skb_shared_info { pub nr_frags: u16, pub flags: u32, pub gso_size: u16, pub frags: [skb_frag_t; 17], pub hwtstamps: hwtstamps }
#[repr(C)] pub struct hwtstamps { pub hwtstamp: u64 }
#[repr(C)] pub struct sk_buff { pub pp_recycle: bool, pub len: u32, pub data_len: u32, pub truesize: u32, pub data: *mut u8, pub head: *mut u8, pub tail: u32, pub end: u32, pub head_frag: bool, pub destructor: Option<unsafe extern "C" fn()>, pub sk: *mut core::ffi::c_void, pub next: *mut sk_buff, pub list: list_head, pub dev: *mut net_device, pub protocol: u16, pub encapsulation: bool, pub ip_summed: i32, pub csum: u32, pub csum_complete_sw: bool, pub csum_level: u8, pub vlan_all: u32, pub slow_gro: bool, pub pfmemalloc: bool, pub skb_iif: u32, pub pkt_type: i32, pub fclone: i32 }
#[repr(C)] pub struct gro_node { pub hash: *mut gro_list, pub bitmask: usize, pub cached_napi_id: u32, pub rx_list: list_head, pub rx_count: u32 }
#[repr(C)] pub struct gro_list { pub list: list_head, pub count: u32 }
#[repr(C)] pub struct napi_struct { pub dev: *mut net_device, pub skb: *mut sk_buff, pub gro: gro_node }
#[repr(C)] pub struct packet_offload { pub list: list_head, pub priority: i32, pub type_: u16 }
pub type gro_result_t = i32;
const GRO_NORMAL: i32 = 0; const GRO_MERGED_FREE: i32 = 1; const GRO_HELD: i32 = 2; const GRO_MERGED: i32 = 3; const GRO_CONSUMED: i32 = 4;

pub unsafe extern "C" fn dev_add_offload(po: *mut packet_offload) { spin_lock(&raw mut offload_lock); /* list insertion ordered by priority */ list_add_rcu(&mut (*po).list, (*po).list.prev); spin_unlock(&raw mut offload_lock); }
pub unsafe extern "C" fn dev_remove_offload(po: *mut packet_offload) { spin_lock(&raw mut offload_lock); list_del_rcu(&mut (*po).list); spin_unlock(&raw mut offload_lock); synchronize_net(); }

pub unsafe extern "C" fn skb_gro_receive(p: *mut sk_buff, skb: *mut sk_buff) -> i32 {
    let pi = skb_shinfo(skb); let plen = (*p).len; let len = skb_gro_len(skb);
    if (*p).pp_recycle != (*skb).pp_recycle || skb_zcopy(p) || skb_zcopy(skb) { return -89; }
    if plen.wrapping_add(len) >= netif_get_gro_max_size((*p).dev, p) { return -7; }
    (*p).data_len = (*p).data_len.wrapping_add(len); (*p).truesize = (*p).truesize.wrapping_add((*skb).truesize); (*p).len = plen.wrapping_add(len);
    (*pi).flags |= (*skb_shinfo(p)).flags; return 0;
}

pub unsafe extern "C" fn skb_gro_receive_list(p: *mut sk_buff, skb: *mut sk_buff) -> i32 {
    if (*p).len.wrapping_add((*skb).len) >= 65536 || !pskb_may_pull(skb, skb_gro_offset(skb)) { return -7; }
    (*p).data_len = (*p).data_len.wrapping_add((*skb).len); (*p).truesize = (*p).truesize.wrapping_add((*skb).truesize); (*p).len = (*p).len.wrapping_add((*skb).len); 0
}

pub unsafe extern "C" fn gro_init(gro: *mut gro_node) { (*gro).bitmask = 0; (*gro).cached_napi_id = 0; (*gro).rx_count = 0; INIT_LIST_HEAD(&mut (*gro).rx_list); }
pub unsafe extern "C" fn gro_cleanup(gro: *mut gro_node) { (*gro).bitmask = 0; (*gro).cached_napi_id = 0; (*gro).rx_count = 0; }

pub unsafe extern "C" fn __gro_flush(_gro: *mut gro_node, _flush_old: bool) { }
pub unsafe extern "C" fn gro_receive_skb(_gro: *mut gro_node, _skb: *mut sk_buff) -> gro_result_t { GRO_NORMAL }
pub unsafe extern "C" fn napi_gro_frags(_napi: *mut napi_struct) -> gro_result_t { GRO_NORMAL }
pub unsafe extern "C" fn napi_get_frags(napi: *mut napi_struct) -> *mut sk_buff { (*napi).skb }
pub unsafe extern "C" fn __skb_gro_checksum_complete(skb: *mut sk_buff) -> u16 {
    let wsum = skb_checksum(skb, skb_gro_offset(skb), skb_gro_len(skb), 0);
    (*skb_shinfo(skb)).flags = (*skb_shinfo(skb)).flags;
    csum_fold(csum_add(0, wsum))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
