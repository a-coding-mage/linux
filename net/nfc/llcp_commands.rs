// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011  Intel Corporation. All rights reserved.
 */

// Translated from llcp_commands.c. Kernel and project symbols are supplied by
// the surrounding translation unit.

static LLCP_TLV_LENGTH: [u8; LLCP_TLV_MAX as usize] = [
    0, 1, 2, 2, 1, 1, 0, 1, 0, 2,
];

unsafe fn llcp_tlv8(tlv: *const u8, ty: u8) -> u8 {
    if *tlv != ty || *tlv.add(1) != LLCP_TLV_LENGTH[*tlv as usize] { return 0; }
    *tlv.add(2)
}

unsafe fn llcp_tlv16(tlv: *const u8, ty: u8) -> u16 {
    if *tlv != ty || *tlv.add(1) != LLCP_TLV_LENGTH[*tlv as usize] { return 0; }
    u16::from_be(*(tlv.add(2) as *const u16))
}

unsafe fn llcp_tlv_version(tlv: *const u8) -> u8 { llcp_tlv8(tlv, LLCP_TLV_VERSION) }
unsafe fn llcp_tlv_miux(tlv: *const u8) -> u16 { llcp_tlv16(tlv, LLCP_TLV_MIUX) & 0x7ff }
unsafe fn llcp_tlv_wks(tlv: *const u8) -> u16 { llcp_tlv16(tlv, LLCP_TLV_WKS) }
unsafe fn llcp_tlv_lto(tlv: *const u8) -> u16 { llcp_tlv8(tlv, LLCP_TLV_LTO) as u16 }
unsafe fn llcp_tlv_opt(tlv: *const u8) -> u8 { llcp_tlv8(tlv, LLCP_TLV_OPT) }
unsafe fn llcp_tlv_rw(tlv: *const u8) -> u8 { llcp_tlv8(tlv, LLCP_TLV_RW) & 0xf }

pub unsafe fn nfc_llcp_build_tlv(ty: u8, value: *const u8, value_length: u8,
                                 tlv_length: *mut u8) -> *mut u8 {
    if ty >= LLCP_TLV_MAX { return core::ptr::null_mut(); }
    let mut length = LLCP_TLV_LENGTH[ty as usize];
    if length == 0 && value_length == 0 { return core::ptr::null_mut(); }
    if length == 0 { length = value_length; }
    *tlv_length = 2 + length;
    let tlv = kzalloc((2 + length) as usize, GFP_KERNEL);
    if tlv.is_null() { return tlv; }
    *tlv = ty; *tlv.add(1) = length;
    memcpy(tlv.add(2), value, length as usize);
    tlv
}

pub unsafe fn nfc_llcp_build_sdres_tlv(tid: u8, sap: u8) -> *mut nfc_llcp_sdp_tlv {
    let sdres = kzalloc_obj::<nfc_llcp_sdp_tlv>();
    if sdres.is_null() { return core::ptr::null_mut(); }
    let value = [tid, sap];
    (*sdres).tlv = nfc_llcp_build_tlv(LLCP_TLV_SDRES, value.as_ptr(), 2,
                                       &mut (*sdres).tlv_len);
    if (*sdres).tlv.is_null() { kfree(sdres); return core::ptr::null_mut(); }
    (*sdres).tid = tid; (*sdres).sap = sap;
    INIT_HLIST_NODE(&mut (*sdres).node);
    sdres
}

pub unsafe fn nfc_llcp_build_sdreq_tlv(tid: u8, uri: *const i8, uri_len: usize)
    -> *mut nfc_llcp_sdp_tlv {
    if uri_len > (u8::MAX as usize) - 4 { return core::ptr::null_mut(); }
    let sdreq = kzalloc_obj::<nfc_llcp_sdp_tlv>();
    if sdreq.is_null() { return core::ptr::null_mut(); }
    (*sdreq).tlv_len = (uri_len + 3) as u8;
    if *uri.add(uri_len - 1) == 0 { (*sdreq).tlv_len -= 1; }
    (*sdreq).tlv = kzalloc((*sdreq).tlv_len as usize + 1, GFP_KERNEL);
    if (*sdreq).tlv.is_null() { kfree(sdreq); return core::ptr::null_mut(); }
    *(*sdreq).tlv = LLCP_TLV_SDREQ;
    *(*sdreq).tlv.add(1) = (*sdreq).tlv_len - 2;
    *(*sdreq).tlv.add(2) = tid;
    (*sdreq).tid = tid;
    (*sdreq).uri = (*sdreq).tlv.add(3) as *mut i8;
    memcpy((*sdreq).uri as *mut u8, uri as *const u8, uri_len);
    (*sdreq).time = jiffies;
    INIT_HLIST_NODE(&mut (*sdreq).node);
    sdreq
}

pub unsafe fn nfc_llcp_free_sdp_tlv(sdp: *mut nfc_llcp_sdp_tlv) { kfree((*sdp).tlv); kfree(sdp); }

pub unsafe fn nfc_llcp_free_sdp_tlv_list(head: *mut hlist_head) {
    let mut sdp: *mut nfc_llcp_sdp_tlv = core::ptr::null_mut();
    let mut n: *mut hlist_node = core::ptr::null_mut();
    hlist_for_each_entry_safe!(sdp, n, head, node) {
        hlist_del(&mut (*sdp).node);
        nfc_llcp_free_sdp_tlv(sdp);
    }
}

pub unsafe fn nfc_llcp_parse_gb_tlv(local: *mut nfc_llcp_local, tlv_array: *const u8,
                                    tlv_array_len: u16) -> i32 {
    if local.is_null() { return -ENODEV; }
    let mut tlv = tlv_array; let mut offset: u16 = 0;
    while offset < tlv_array_len {
        if offset + 2 > tlv_array_len { return -EINVAL; }
        let ty = *tlv; let length = *tlv.add(1) as u16;
        if offset + 2 + length > tlv_array_len { return -EINVAL; }
        match ty {
            LLCP_TLV_VERSION => (*local).remote_version = llcp_tlv_version(tlv),
            LLCP_TLV_MIUX => (*local).remote_miu = llcp_tlv_miux(tlv) + 128,
            LLCP_TLV_WKS => (*local).remote_wks = llcp_tlv_wks(tlv),
            LLCP_TLV_LTO => (*local).remote_lto = llcp_tlv_lto(tlv) * 10,
            LLCP_TLV_OPT => (*local).remote_opt = llcp_tlv_opt(tlv),
            _ => (),
        }
        offset += length + 2; tlv = tlv.add(length as usize + 2);
    }
    0
}

pub unsafe fn nfc_llcp_parse_connection_tlv(sock: *mut nfc_llcp_sock, tlv_array: *const u8,
                                             tlv_array_len: u16) -> i32 {
    if sock.is_null() { return -ENOTCONN; }
    let mut tlv = tlv_array; let mut offset: u16 = 0;
    while offset < tlv_array_len {
        if offset + 2 > tlv_array_len { return -EINVAL; }
        let ty = *tlv; let length = *tlv.add(1) as u16;
        if offset + 2 + length > tlv_array_len { return -EINVAL; }
        match ty {
            LLCP_TLV_MIUX => (*sock).remote_miu = llcp_tlv_miux(tlv) + 128,
            LLCP_TLV_RW => (*sock).remote_rw = llcp_tlv_rw(tlv),
            LLCP_TLV_SN => (),
            _ => (),
        }
        offset += length + 2; tlv = tlv.add(length as usize + 2);
    }
    0
}

unsafe fn llcp_add_header(pdu: *mut sk_buff, dsap: u8, ssap: u8, ptype: u8) -> *mut sk_buff {
    let header = [(dsap << 2) | (ptype >> 2), (ptype << 6) | ssap];
    skb_put_data(pdu, header.as_ptr(), LLCP_HEADER_SIZE); pdu
}
unsafe fn llcp_add_tlv(pdu: *mut sk_buff, tlv: *const u8, len: u8) -> *mut sk_buff {
    if tlv.is_null() { return core::ptr::null_mut(); }
    skb_put_data(pdu, tlv, len as usize); pdu
}

// The remaining send routines retain the kernel skb allocation and queueing
// operations; their declarations depend on the surrounding NFC translation.
pub unsafe fn llcp_allocate_pdu(sock: *mut nfc_llcp_sock, cmd: u8, size: u16) -> *mut sk_buff {
    if (*sock).ssap == 0 { return core::ptr::null_mut(); }
    let mut err = 0; let skb = nfc_alloc_send_skb((*sock).dev, &mut (*sock).sk, MSG_DONTWAIT,
                                                  size + LLCP_HEADER_SIZE, &mut err);
    if skb.is_null() { return core::ptr::null_mut(); }
    llcp_add_header(skb, (*sock).dsap, (*sock).ssap, cmd)
}

pub unsafe fn nfc_llcp_send_disconnect(sock: *mut nfc_llcp_sock) -> i32 {
    let local = (*sock).local; if local.is_null() { return -ENODEV; }
    if (*sock).dev.is_null() { return -ENODEV; }
    let skb = llcp_allocate_pdu(sock, LLCP_PDU_DISC, 0); if skb.is_null() { return -ENOMEM; }
    skb_queue_tail(&mut (*local).tx_queue, skb); 0
}

pub unsafe fn nfc_llcp_send_symm(dev: *mut nfc_dev) -> i32 {
    let local = nfc_llcp_find_local(dev); if local.is_null() { return -ENODEV; }
    let size = LLCP_HEADER_SIZE + (*dev).tx_headroom + (*dev).tx_tailroom + NFC_HEADER_SIZE;
    let skb = alloc_skb(size, GFP_KERNEL); if skb.is_null() { nfc_llcp_local_put(local); return -ENOMEM; }
    skb_reserve(skb, (*dev).tx_headroom + NFC_HEADER_SIZE);
    let skb = llcp_add_header(skb, 0, 0, LLCP_PDU_SYMM);
    __net_timestamp(skb); nfc_llcp_send_to_raw_sock(local, skb, NFC_DIRECTION_TX);
    let err = nfc_data_exchange(dev, (*local).target_idx, skb, nfc_llcp_recv, local);
    nfc_llcp_local_put(local); err
}

pub unsafe fn nfc_llcp_send_connect(sock: *mut nfc_llcp_sock) -> i32 {
    let local = (*sock).local; if local.is_null() { return -ENODEV; }
    let mut sn: *mut u8 = core::ptr::null_mut(); let mut miux: *mut u8 = core::ptr::null_mut();
    let mut rw_tlv: *mut u8 = core::ptr::null_mut(); let mut sn_len = 0; let mut miux_len = 0; let mut rw_len = 0;
    let mut size: u16 = 0;
    if !(*sock).service_name.is_null() {
        sn = nfc_llcp_build_tlv(LLCP_TLV_SN, (*sock).service_name, (*sock).service_name_len, &mut sn_len);
        if sn.is_null() { return -ENOMEM; } size += sn_len as u16;
    }
    let miux_value = if u16::from_be((*sock).miux) > LLCP_MAX_MIUX { (*local).miux } else { (*sock).miux };
    let rw = if (*sock).rw > LLCP_MAX_RW { (*local).rw } else { (*sock).rw };
    miux = nfc_llcp_build_tlv(LLCP_TLV_MIUX, &miux_value as *const _ as *const u8, 0, &mut miux_len);
    if miux.is_null() { kfree(sn); return -ENOMEM; } size += miux_len as u16;
    rw_tlv = nfc_llcp_build_tlv(LLCP_TLV_RW, &rw, 0, &mut rw_len);
    if rw_tlv.is_null() { kfree(sn); kfree(miux); return -ENOMEM; } size += rw_len as u16;
    let skb = llcp_allocate_pdu(sock, LLCP_PDU_CONNECT, size);
    if skb.is_null() { kfree(sn); kfree(miux); kfree(rw_tlv); return -ENOMEM; }
    llcp_add_tlv(skb, sn, sn_len); llcp_add_tlv(skb, miux, miux_len); llcp_add_tlv(skb, rw_tlv, rw_len);
    skb_queue_tail(&mut (*local).tx_queue, skb);
    kfree(sn); kfree(miux); kfree(rw_tlv); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
