/*
 * net/ife/ife.c - Inter-FE protocol based on ForCES WG InterFE LFB
 * Copyright (c) 2015 Jamal Hadi Salim <jhs@mojatatu.com>
 * Copyright (c) 2017 Yotam Gigi <yotamg@mellanox.com>
 *
 * Refer to: draft-ietf-forces-interfelfb-03 and netdev01 paper:
 * "Distributing Linux Traffic Control Classifier-Action Subsystem"
 * Authors: Jamal Hadi Salim and Damascene M. Joachimpillai
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

#[repr(C)]
pub struct ifeheadr {
    pub metalen: u16,
    pub tlv_data: [u8; 0],
}

#[repr(C)]
pub struct meta_tlvhdr {
    pub type_: u16,
    pub len: u16,
}

// External kernel types and helpers supplied by other translation units.
extern "C" {
    pub fn skb_cow_head(skb: *mut sk_buff, headroom: i32) -> i32;
    pub fn __skb_push(skb: *mut sk_buff, len: i32) -> *mut core::ffi::c_void;
    pub fn skb_reset_mac_header(skb: *mut sk_buff);
    pub fn pskb_may_pull(skb: *mut sk_buff, len: i32) -> bool;
    pub fn skb_set_mac_header(skb: *mut sk_buff, offset: i32);
    pub fn __skb_pull(skb: *mut sk_buff, len: i32) -> *mut core::ffi::c_void;
    pub fn htons(value: u16) -> u16;
    pub fn ntohs(value: u16) -> u16;
    pub fn htonl(value: u32) -> u32;
    pub fn nla_total_size(len: u16) -> u16;
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
}

extern "C" {
    pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
}

const ETH_HLEN: i32 = 14;
const IFE_METAHDRLEN: u16 = 2;
const NLA_HDRLEN: u16 = 4;

#[inline]
unsafe fn nla_align(len: u16) -> u16 {
    (len + 3) & !3
}

pub unsafe fn ife_encode(skb: *mut sk_buff, mut metalen: u16) -> *mut core::ffi::c_void {
    /* OUTERHDR:TOTMETALEN:{TLVHDR:Metadatum:TLVHDR..}:ORIGDATA
     * where ORIGDATA = original ethernet header ...
     */
    let hdrm = metalen as i32 + IFE_METAHDRLEN as i32;
    let total_push = hdrm + ETH_HLEN;
    let err = skb_cow_head(skb, total_push);
    if err != 0 {
        return core::ptr::null_mut();
    }

    let iethh = (*skb).data;
    __skb_push(skb, total_push);
    memcpy((*skb).data as *mut core::ffi::c_void, iethh as *const core::ffi::c_void, ETH_HLEN as usize);
    skb_reset_mac_header(skb);

    let ifehdr = ((*skb).data.add(ETH_HLEN as usize)) as *mut ifeheadr;
    metalen = metalen.wrapping_add(IFE_METAHDRLEN);
    (*ifehdr).metalen = htons(metalen);

    (*ifehdr).tlv_data.as_mut_ptr() as *mut core::ffi::c_void
}

pub unsafe fn ife_decode(skb: *mut sk_buff, metalen: *mut u16) -> *mut core::ffi::c_void {
    if !pskb_may_pull(skb, ETH_HLEN + IFE_METAHDRLEN as i32) {
        return core::ptr::null_mut();
    }

    let mut ifehdr = ((*skb).data.add(ETH_HLEN as usize)) as *mut ifeheadr;
    let ifehdrln = ntohs((*ifehdr).metalen);
    let total_pull = ETH_HLEN + ifehdrln as i32;
    if ifehdrln < 2 || !pskb_may_pull(skb, total_pull + ETH_HLEN) {
        return core::ptr::null_mut();
    }

    ifehdr = ((*skb).data.add(ETH_HLEN as usize)) as *mut ifeheadr;
    skb_set_mac_header(skb, total_pull);
    __skb_pull(skb, total_pull);
    *metalen = ifehdrln.wrapping_sub(IFE_METAHDRLEN);
    (*ifehdr).tlv_data.as_mut_ptr() as *mut core::ffi::c_void
}

unsafe fn __ife_tlv_meta_valid(skbdata: *const u8, ifehdr_end: *const u8) -> bool {
    if skbdata.add(core::mem::size_of::<meta_tlvhdr>()) > ifehdr_end {
        return false;
    }
    let tlv = skbdata as *const meta_tlvhdr;
    let tlvlen = ntohs((*tlv).len);
    if tlvlen < NLA_HDRLEN || nla_align(tlvlen) < tlvlen {
        return false;
    }
    skbdata.add(nla_align(tlvlen) as usize) <= ifehdr_end
}

pub unsafe fn ife_tlv_meta_decode(skbdata: *mut core::ffi::c_void, ifehdr_end: *const core::ffi::c_void, attrtype: *mut u16, dlen: *mut u16, totlen: *mut u16) -> *mut core::ffi::c_void {
    if !__ife_tlv_meta_valid(skbdata as *const u8, ifehdr_end as *const u8) {
        return core::ptr::null_mut();
    }
    let tlv = skbdata as *mut meta_tlvhdr;
    *dlen = ntohs((*tlv).len).wrapping_sub(NLA_HDRLEN);
    *attrtype = ntohs((*tlv).type_);
    if !totlen.is_null() {
        *totlen = nla_total_size(*dlen);
    }
    (skbdata as *mut u8).add(core::mem::size_of::<meta_tlvhdr>()) as *mut core::ffi::c_void
}

pub unsafe fn ife_tlv_meta_next(skbdata: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let tlv = skbdata as *mut meta_tlvhdr;
    (skbdata as *mut u8).add(nla_align(ntohs((*tlv).len)) as usize) as *mut core::ffi::c_void
}

pub unsafe fn ife_tlv_meta_encode(skbdata: *mut core::ffi::c_void, attrtype: u16, dlen: u16, dval: *const core::ffi::c_void) -> u16 {
    let tlv = skbdata as *mut u32;
    let totlen = nla_total_size(dlen);
    let dptr = (tlv as *mut u8).add(NLA_HDRLEN as usize);
    let htlv = ((attrtype as u32) << 16) | (dlen.wrapping_add(NLA_HDRLEN) as u32);
    *tlv = htonl(htlv);
    memset(dptr as *mut core::ffi::c_void, 0, totlen.wrapping_sub(NLA_HDRLEN) as usize);
    memcpy(dptr as *mut core::ffi::c_void, dval, dlen as usize);
    totlen
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
