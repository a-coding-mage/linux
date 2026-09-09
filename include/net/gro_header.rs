/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from net/gro.h. Required kernel types and functions are external dependencies. */

pub const GRO_MAX_HEAD: usize = MAX_HEADER + 128;

#[repr(C)]
pub union napi_gro_cb_union {
    pub frag: napi_gro_cb_frag,
    pub slow: napi_gro_cb_slow,
}
#[repr(C)]
pub struct napi_gro_cb_frag { pub frag0: *mut core::ffi::c_void, pub frag0_len: u32 }
#[repr(C)]
pub struct napi_gro_cb_slow { pub last: *mut sk_buff, pub age: usize }

#[repr(C)]
pub struct napi_gro_cb {
    pub u: napi_gro_cb_union,
    pub data_offset: i32,
    pub flush: u16,
    pub count: u16,
    pub proto: u16,
    pub pad: u16,
    pub gro_remcsum_start: u16,
    pub same_flow: u8,
    pub encap_mark: u8,
    pub csum_valid: u8,
    pub csum_cnt: u8,
    pub free: u8,
    pub is_fou: u8,
    pub ip_fixedid: u8,
    pub recursion_counter: u8,
    pub is_flist: u8,
    pub csum: __wsum,
    pub network_offset: u16,
    pub inner_network_offset: u16,
}

pub const NAPI_GRO_FREE: u8 = 1;
pub const NAPI_GRO_FREE_STOLEN_HEAD: u8 = 2;
pub const GRO_RECURSION_LIMIT: u8 = 15;

#[inline]
pub unsafe fn napi_gro_cb(skb: *mut sk_buff) -> *mut napi_gro_cb { (*skb).cb.as_mut_ptr() as *mut napi_gro_cb }

#[inline]
pub unsafe fn gro_recursion_inc_test(skb: *mut sk_buff) -> bool {
    let cb = &mut *napi_gro_cb(skb);
    cb.recursion_counter = cb.recursion_counter.wrapping_add(1);
    cb.recursion_counter == GRO_RECURSION_LIMIT
}

pub type gro_receive_t = unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff;
#[inline]
pub unsafe fn call_gro_receive(cb: gro_receive_t, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    if gro_recursion_inc_test(skb) { (*napi_gro_cb(skb)).flush |= 1; core::ptr::null_mut() } else { cb(head, skb) }
}
pub type gro_receive_sk_t = unsafe extern "C" fn(*mut sock, *mut list_head, *mut sk_buff) -> *mut sk_buff;
#[inline]
pub unsafe fn call_gro_receive_sk(cb: gro_receive_sk_t, sk: *mut sock, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    if gro_recursion_inc_test(skb) { (*napi_gro_cb(skb)).flush |= 1; core::ptr::null_mut() } else { cb(sk, head, skb) }
}

#[inline] pub unsafe fn skb_gro_offset(skb: *const sk_buff) -> u32 { (*napi_gro_cb(skb as *mut sk_buff)).data_offset as u32 }
#[inline] pub unsafe fn skb_gro_len(skb: *const sk_buff) -> u32 { (*skb).len - skb_gro_offset(skb) }
#[inline] pub unsafe fn skb_gro_pull(skb: *mut sk_buff, len: u32) { (*napi_gro_cb(skb)).data_offset += len as i32; }
#[inline] pub unsafe fn skb_gro_header_fast(skb: *const sk_buff, offset: usize) -> *mut core::ffi::c_void { ((*napi_gro_cb(skb as *mut sk_buff)).u.frag.frag0 as *mut u8).add(offset) as *mut _ }
#[inline] pub unsafe fn skb_gro_may_pull(skb: *const sk_buff, hlen: u32) -> bool { hlen <= (*napi_gro_cb(skb as *mut sk_buff)).u.frag.frag0_len }
#[inline] pub unsafe fn skb_gro_header_slow(skb: *mut sk_buff, hlen: u32, offset: usize) -> *mut core::ffi::c_void { if !pskb_may_pull(skb, hlen) { core::ptr::null_mut() } else { (*skb).data.add(offset) as *mut _ } }
#[inline] pub unsafe fn skb_gro_header(skb: *mut sk_buff, hlen: u32, offset: usize) -> *mut core::ffi::c_void { let p=skb_gro_header_fast(skb,offset); if skb_gro_may_pull(skb,hlen) { p } else { skb_gro_header_slow(skb,hlen,offset) } }
#[inline] pub unsafe fn skb_gro_receive_network_offset(skb: *const sk_buff) -> i32 { let c=&*napi_gro_cb(skb as *mut _); c.network_offset as i32 }
#[inline] pub unsafe fn skb_gro_network_header(skb: *const sk_buff) -> *mut core::ffi::c_void { if skb_gro_may_pull(skb,skb_gro_offset(skb)) { skb_gro_header_fast(skb,skb_gro_receive_network_offset(skb) as usize) } else { (*skb).data.add(skb_gro_receive_network_offset(skb) as usize) as *mut _ } }

extern "C" {
    pub fn __skb_gro_checksum_complete(skb: *mut sk_buff) -> __sum16;
    pub fn skb_gro_remcsum_process(skb: *mut sk_buff, ptr: *mut core::ffi::c_void, off: u32, hdrlen: usize, start: i32, offset: i32, grc: *mut gro_remcsum, nopartial: bool) -> *mut core::ffi::c_void;
    pub fn skb_gro_receive(p: *mut sk_buff, skb: *mut sk_buff) -> i32;
    pub fn skb_gro_receive_list(p: *mut sk_buff, skb: *mut sk_buff) -> i32;
    pub fn __gro_flush(gro: *mut gro_node, flush_old: bool);
    pub fn gro_init(gro: *mut gro_node);
    pub fn gro_cleanup(gro: *mut gro_node);
    pub fn udp6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn udp6_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32;
    pub fn udp_gro_receive(head: *mut list_head, skb: *mut sk_buff, uh: *mut udphdr, sk: *mut sock) -> *mut sk_buff;
    pub fn udp_gro_complete(skb: *mut sk_buff, nhoff: i32, lookup: udp_lookup_t) -> i32;
    pub fn gro_find_receive_by_type(t: __be16) -> *mut packet_offload;
    pub fn gro_find_complete_by_type(t: __be16) -> *mut packet_offload;
    pub fn ipv6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn ipv6_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32;
    pub fn inet_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn inet_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32;
    pub fn udp4_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn udp4_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32;
}

#[repr(C)] pub struct gro_remcsum { pub offset: i32, pub delta: __wsum }
#[inline] pub unsafe fn skb_gro_remcsum_init(g: *mut gro_remcsum) { (*g).offset=0; (*g).delta=0; }
#[inline] pub unsafe fn skb_gro_remcsum_cleanup(skb: *mut sk_buff, g: *mut gro_remcsum) { if (*g).delta != 0 { let p=skb_gro_header(skb, ((*g).offset as usize + core::mem::size_of::<u16>()) as u32, (*g).offset as usize); if !p.is_null() { remcsum_unadjust(p as *mut __sum16, (*g).delta); } } }

#[inline] pub unsafe fn gro_flush(g: *mut gro_node, flush_old: bool) { if (*g).bitmask != 0 { __gro_flush(g,flush_old); } }
#[inline] pub unsafe fn napi_gro_flush(napi: *mut napi_struct, flush_old: bool) { gro_flush(&mut (*napi).gro,flush_old); }
#[inline] pub unsafe fn gro_normal_list(g: *mut gro_node) { if (*g).rx_count != 0 { netif_receive_skb_list_internal(&mut (*g).rx_list); INIT_LIST_HEAD(&mut (*g).rx_list); (*g).rx_count=0; } }
#[inline] pub unsafe fn gro_flush_normal(g: *mut gro_node, f: bool) { gro_flush(g,f); gro_normal_list(g); }
#[inline] pub unsafe fn gro_normal_one(g: *mut gro_node, skb: *mut sk_buff, segs: i32) { list_add_tail(&mut (*skb).list,&mut (*g).rx_list); (*g).rx_count += segs; if (*g).rx_count >= READ_ONCE(net_hotdata.gro_normal_batch) { gro_normal_list(g); } }

#[inline] pub unsafe fn skb_at_gro_remcsum_start(skb: *mut sk_buff) -> bool { (*napi_gro_cb(skb)).gro_remcsum_start as u32 == skb_gro_offset(skb) }
#[inline] pub unsafe fn __skb_gro_checksum_convert_check(skb: *mut sk_buff) -> bool { (*napi_gro_cb(skb)).csum_cnt == 0 && (*napi_gro_cb(skb)).csum_valid == 0 }
#[inline] pub unsafe fn __skb_gro_checksum_convert(skb: *mut sk_buff, pseudo: __wsum) { (*napi_gro_cb(skb)).csum = !pseudo; (*napi_gro_cb(skb)).csum_valid=1; }
#[inline] pub unsafe fn skb_gro_incr_csum_unnecessary(skb: *mut sk_buff) { if (*napi_gro_cb(skb)).csum_cnt > 0 { (*napi_gro_cb(skb)).csum_cnt-=1; } else { __skb_incr_checksum_unnecessary(skb); } }
#[inline] pub unsafe fn skb_gro_checksum_try_convert(skb: *mut sk_buff, proto: i32, compute: unsafe fn(*const sk_buff,i32)->__wsum) { if __skb_gro_checksum_convert_check(skb) { __skb_gro_checksum_convert(skb,compute(skb,proto)); } }
#[inline] pub unsafe fn inet_gro_compute_pseudo(skb: *const sk_buff, proto: i32) -> __wsum { let iph=skb_gro_network_header(skb) as *const iphdr; csum_tcpudp_nofold((*iph).saddr,(*iph).daddr,skb_gro_len(skb),proto,0) }
#[inline] pub unsafe fn ip6_gro_compute_pseudo(skb: *const sk_buff, proto: i32) -> __wsum { let iph=skb_gro_network_header(skb) as *const ipv6hdr; !csum_unfold(csum_ipv6_magic(&(*iph).saddr,&(*iph).daddr,skb_gro_len(skb),proto,0)) }
#[inline] pub unsafe fn udp_gro_udphdr(skb: *mut sk_buff) -> *mut udphdr { let off=skb_gro_offset(skb); skb_gro_header(skb,off+core::mem::size_of::<udphdr>() as u32,off as usize) as *mut udphdr }
#[inline] pub unsafe fn skb_gro_flush_final(skb: *mut sk_buff, _pp: *mut sk_buff, flush: i32) { (*napi_gro_cb(skb)).flush |= flush as u16; }

#[inline] pub unsafe fn tcp_gro_pull_header(skb: *mut sk_buff) -> *mut tcphdr { let off=skb_gro_offset(skb); let th=skb_gro_header(skb,off+core::mem::size_of::<tcphdr>() as u32,off as usize) as *mut tcphdr; if th.is_null() || (*th).doff < 5 { core::ptr::null_mut() } else { let l=(*th).doff as u32*4; if !skb_gro_may_pull(skb,off+l) { let x=skb_gro_header_slow(skb,off+l,off as usize); if x.is_null(){return core::ptr::null_mut();} } skb_gro_pull(skb,l); th } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
