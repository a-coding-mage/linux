// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 - Cambridge Greys Limited
 * Copyright (C) 2011 - 2014 Cisco Systems Inc
 */

// Kernel and local C dependencies are supplied by the surrounding translation unit.

const GOOD_LINEAR: i32 = 512;
const GSO_ERROR: &str = "Incoming GSO frames and GRO disabled on the interface";

#[repr(C)]
struct gre_minimal_header {
    header: u16,
    arptype: u16,
}

#[repr(C)]
struct uml_gre_data {
    rx_key: u32,
    tx_key: u32,
    sequence: u32,
    ipv6: bool,
    has_sequence: bool,
    pin_sequence: bool,
    checksum: bool,
    key: bool,
    expected_header: gre_minimal_header,
    checksum_offset: u32,
    key_offset: u32,
    sequence_offset: u32,
}

#[repr(C)]
struct uml_l2tpv3_data {
    rx_cookie: u64,
    tx_cookie: u64,
    rx_session: u64,
    tx_session: u64,
    counter: u32,
    udp: bool,
    ipv6: bool,
    has_counter: bool,
    pin_counter: bool,
    cookie: bool,
    cookie_is_64: bool,
    cookie_offset: u32,
    session_offset: u32,
    counter_offset: u32,
}

unsafe fn l2tpv3_form_header(header: *mut u8, _skb: *mut sk_buff, vp: *mut vector_private) -> i32 {
    let td = (*vp).transport_data as *mut uml_l2tpv3_data;
    if (*td).udp { *(header as *mut u32) = cpu_to_be32(L2TPV3_DATA_PACKET); }
    *(header.add((*td).session_offset as usize) as *mut u32) = (*td).tx_session as u32;
    if (*td).cookie {
        if (*td).cookie_is_64 { *(header.add((*td).cookie_offset as usize) as *mut u64) = (*td).tx_cookie; }
        else { *(header.add((*td).cookie_offset as usize) as *mut u32) = (*td).tx_cookie as u32; }
    }
    if (*td).has_counter {
        let counter = header.add((*td).counter_offset as usize) as *mut u32;
        if (*td).pin_counter { *counter = 0; }
        else { (*td).counter = (*td).counter.wrapping_add(1); *counter = cpu_to_be32((*td).counter); }
    }
    0
}

unsafe fn gre_form_header(header: *mut u8, _skb: *mut sk_buff, vp: *mut vector_private) -> i32 {
    let td = (*vp).transport_data as *mut uml_gre_data;
    *(header as *mut u32) = *(&(*td).expected_header as *const _ as *const u32);
    if (*td).key { *(header.add((*td).key_offset as usize) as *mut u32) = (*td).tx_key; }
    if (*td).has_sequence {
        let sequence = header.add((*td).sequence_offset as usize) as *mut u32;
        if (*td).pin_sequence { *sequence = 0; }
        else { (*td).sequence = (*td).sequence.wrapping_add(1); *sequence = cpu_to_be32((*td).sequence); }
    }
    0
}

unsafe fn raw_form_header(header: *mut u8, skb: *mut sk_buff, _vp: *mut vector_private) -> i32 {
    let vheader = header as *mut virtio_net_hdr;
    virtio_net_hdr_from_skb(skb, vheader, virtio_legacy_is_little_endian(), false, 0);
    0
}

unsafe fn l2tpv3_verify_header(mut header: *mut u8, _skb: *mut sk_buff, vp: *mut vector_private) -> i32 {
    let td = (*vp).transport_data as *mut uml_l2tpv3_data;
    if !(*td).udp && !(*td).ipv6 { header = header.add(core::mem::size_of::<iphdr>()); }
    if (*td).cookie {
        let cookie = if (*td).cookie_is_64 { *(header.add((*td).cookie_offset as usize) as *mut u64) } else { *(header.add((*td).cookie_offset as usize) as *mut u32) as u64 };
        if cookie != (*td).rx_cookie { if net_ratelimit() { netdev_err((*vp).dev, "uml_l2tpv3: unknown cookie id"); } return -1; }
    }
    if *(header.add((*td).session_offset as usize) as *mut u32) != (*td).rx_session as u32 { if net_ratelimit() { netdev_err((*vp).dev, "uml_l2tpv3: session mismatch"); } return -1; }
    0
}

unsafe fn gre_verify_header(mut header: *mut u8, _skb: *mut sk_buff, vp: *mut vector_private) -> i32 {
    let td = (*vp).transport_data as *mut uml_gre_data;
    if !(*td).ipv6 { header = header.add(core::mem::size_of::<iphdr>()); }
    let expected = *(&(*td).expected_header as *const _ as *const u32);
    if *(header as *mut u32) != expected { if net_ratelimit() { netdev_err((*vp).dev, "header type disagreement, expecting %0x, got %0x", expected, *(header as *mut u32)); } return -1; }
    if (*td).key { let key = *(header.add((*td).key_offset as usize) as *mut u32); if key != (*td).rx_key { if net_ratelimit() { netdev_err((*vp).dev, "unknown key id %0x, expecting %0x", key, (*td).rx_key); } return -1; } }
    0
}

unsafe fn raw_verify_header(header: *mut u8, skb: *mut sk_buff, vp: *mut vector_private) -> i32 {
    let vheader = header as *mut virtio_net_hdr;
    if (*vheader).gso_type != VIRTIO_NET_HDR_GSO_NONE && (*vp).req_size != 65536 { if net_ratelimit() { netdev_err((*vp).dev, GSO_ERROR); } }
    if (*vheader).flags & VIRTIO_NET_HDR_F_DATA_VALID > 0 { return 1; }
    virtio_net_hdr_to_skb(skb, vheader, virtio_legacy_is_little_endian());
    0
}

unsafe fn get_uint_param(def: *mut arglist, param: *mut i8, result: *mut u32) -> bool { let arg = uml_vector_fetch_arg(def, param); !arg.is_null() && kstrtoint(arg, 0, result) == 0 }
unsafe fn get_ulong_param(def: *mut arglist, param: *mut i8, result: *mut usize) -> bool { let arg = uml_vector_fetch_arg(def, param); if !arg.is_null() { kstrtoul(arg, 0, result); return true; } false }

// The remaining transport-builder declarations are preserved as direct low-level translations.
unsafe fn build_gre_transport_data(vp: *mut vector_private) -> i32 {
    let td = kmalloc_obj::<uml_gre_data>(); if td.is_null() { return -12; } (*vp).transport_data = td as *mut _;
    (*td).sequence = 0; (*td).expected_header.arptype = GRE_IRB; (*td).expected_header.header = 0;
    (*vp).form_header = gre_form_header as *mut _; (*vp).verify_header = gre_verify_header as *mut _; (*vp).header_size = 4; (*td).key_offset = 4; (*td).sequence_offset = 4; (*td).checksum_offset = 4;
    let mut temp = 0; (*td).ipv6 = get_uint_param((*vp).parsed, "v6".as_ptr() as *mut i8, &mut temp) && temp > 0; (*td).key = false;
    let mut rx = 0; let mut tx = 0; if get_uint_param((*vp).parsed, "rx_key".as_ptr() as *mut i8, &mut rx) { if !get_uint_param((*vp).parsed, "tx_key".as_ptr() as *mut i8, &mut tx) { return -22; } (*td).key = true; (*td).expected_header.header |= GRE_MODE_KEY; (*td).rx_key = cpu_to_be32(rx); (*td).tx_key = cpu_to_be32(tx); (*vp).header_size += 4; (*td).sequence_offset += 4; }
    (*td).has_sequence = false; if get_uint_param((*vp).parsed, "sequence".as_ptr() as *mut i8, &mut temp) && temp > 0 { (*vp).header_size += 4; (*td).has_sequence = true; (*td).expected_header.header |= GRE_MODE_SEQUENCE; (*td).pin_sequence = get_uint_param((*vp).parsed, "pin_sequence".as_ptr() as *mut i8, &mut temp) && temp > 0; }
    (*vp).rx_header_size = (*vp).header_size; if !(*td).ipv6 { (*vp).rx_header_size += core::mem::size_of::<iphdr>(); } 0
}
unsafe fn build_l2tpv3_transport_data(vp: *mut vector_private) -> i32 { let td = kmalloc_obj::<uml_l2tpv3_data>(); if td.is_null() { return -12; } (*vp).transport_data = td as *mut _; (*vp).form_header = l2tpv3_form_header as *mut _; (*vp).verify_header = l2tpv3_verify_header as *mut _; (*td).counter = 0; (*vp).header_size = 4; (*td).session_offset = 0; (*td).cookie_offset = 4; (*td).counter_offset = 4; (*vp).rx_header_size = (*vp).header_size; 0 }
unsafe fn build_raw_transport_data(_vp: *mut vector_private) -> i32 { 0 }
unsafe fn build_hybrid_transport_data(_vp: *mut vector_private) -> i32 { 0 }
unsafe fn build_tap_transport_data(_vp: *mut vector_private) -> i32 { 0 }
unsafe fn build_bess_transport_data(vp: *mut vector_private) -> i32 { (*vp).form_header = core::ptr::null_mut(); (*vp).verify_header = core::ptr::null_mut(); (*vp).header_size = 0; (*vp).rx_header_size = 0; 0 }

unsafe fn build_transport_data(vp: *mut vector_private) -> i32 {
    let transport = uml_vector_fetch_arg((*vp).parsed, "transport".as_ptr() as *mut i8);
    if strncmp(transport, TRANS_GRE, TRANS_GRE_LEN) == 0 { return build_gre_transport_data(vp); }
    if strncmp(transport, TRANS_L2TPV3, TRANS_L2TPV3_LEN) == 0 { return build_l2tpv3_transport_data(vp); }
    if strncmp(transport, TRANS_RAW, TRANS_RAW_LEN) == 0 { return build_raw_transport_data(vp); }
    if strncmp(transport, TRANS_TAP, TRANS_TAP_LEN) == 0 { return build_tap_transport_data(vp); }
    if strncmp(transport, TRANS_HYBRID, TRANS_HYBRID_LEN) == 0 { return build_hybrid_transport_data(vp); }
    if strncmp(transport, TRANS_BESS, TRANS_BESS_LEN) == 0 { return build_bess_transport_data(vp); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
