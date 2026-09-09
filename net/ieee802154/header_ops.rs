// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Fraunhofer ITWM
 *
 * Written by:
 * Phoebe Buckheister <phoebe.buckheister@itwm.fraunhofer.de>
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_reserve(skb: *mut sk_buff, len: usize);
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn skb_put_data(skb: *mut sk_buff, data: *const c_void, len: usize) -> *mut u8;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_pull(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_mac_header(skb: *const sk_buff) -> *const u8;
    fn skb_tail_pointer(skb: *const sk_buff) -> *const u8;
    fn ieee802154_sechdr_authtag_len(hdr: *const ieee802154_sechdr) -> i32;
}

unsafe fn ieee802154_hdr_push_addr(
    buf: *mut u8,
    addr: *const ieee802154_addr,
    omit_pan: bool,
) -> i32 {
    let mut pos: i32 = 0;

    if (*addr).mode == IEEE802154_ADDR_NONE {
        return 0;
    }

    if !omit_pan {
        memcpy(buf.add(pos as usize) as *mut c_void, &(*addr).pan_id as *const _ as *const c_void, 2);
        pos += 2;
    }

    match (*addr).mode {
        IEEE802154_ADDR_SHORT => {
            memcpy(buf.add(pos as usize) as *mut c_void, &(*addr).short_addr as *const _ as *const c_void, 2);
            pos += 2;
        }
        IEEE802154_ADDR_LONG => {
            memcpy(buf.add(pos as usize) as *mut c_void, &(*addr).extended_addr as *const _ as *const c_void, IEEE802154_ADDR_LEN as usize);
            pos += IEEE802154_ADDR_LEN;
        }
        _ => return -EINVAL,
    }

    pos
}

unsafe fn ieee802154_hdr_push_sechdr(buf: *mut u8, hdr: *const ieee802154_sechdr) -> i32 {
    let mut pos: i32 = 5;

    memcpy(buf as *mut c_void, hdr as *const c_void, 1);
    memcpy(buf.add(1) as *mut c_void, &(*hdr).frame_counter as *const _ as *const c_void, 4);

    match (*hdr).key_id_mode {
        IEEE802154_SCF_KEY_IMPLICIT => return pos,
        IEEE802154_SCF_KEY_INDEX => {}
        IEEE802154_SCF_KEY_SHORT_INDEX => {
            memcpy(buf.add(pos as usize) as *mut c_void, &(*hdr).short_src as *const _ as *const c_void, 4);
            pos += 4;
        }
        IEEE802154_SCF_KEY_HW_INDEX => {
            memcpy(buf.add(pos as usize) as *mut c_void, &(*hdr).extended_src as *const _ as *const c_void, IEEE802154_ADDR_LEN as usize);
            pos += IEEE802154_ADDR_LEN;
        }
        _ => {}
    }

    *buf.add(pos as usize) = (*hdr).key_id;
    pos += 1;
    pos
}

pub unsafe fn ieee802154_hdr_push(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> i32 {
    let mut buf = [0u8; IEEE802154_MAX_HEADER_LEN as usize];
    let mut pos: i32 = 2;
    let mut rc: i32;
    let fc = &mut (*hdr).fc;

    buf[pos as usize] = (*hdr).seq;
    pos += 1;
    fc.dest_addr_mode = (*hdr).dest.mode;
    rc = ieee802154_hdr_push_addr(buf.as_mut_ptr().add(pos as usize), &(*hdr).dest, false);
    if rc < 0 { return -EINVAL; }
    pos += rc;
    fc.source_addr_mode = (*hdr).source.mode;
    if (*hdr).source.pan_id == (*hdr).dest.pan_id && (*hdr).dest.mode != IEEE802154_ADDR_NONE {
        fc.intra_pan = true;
    }
    rc = ieee802154_hdr_push_addr(buf.as_mut_ptr().add(pos as usize), &(*hdr).source, fc.intra_pan);
    if rc < 0 { return -EINVAL; }
    pos += rc;
    if fc.security_enabled {
        fc.version = 1;
        rc = ieee802154_hdr_push_sechdr(buf.as_mut_ptr().add(pos as usize), &(*hdr).sec);
        if rc < 0 { return -EINVAL; }
        pos += rc;
    }
    memcpy(buf.as_mut_ptr() as *mut c_void, fc as *const _ as *const c_void, 2);
    memcpy(skb_push(skb, pos as usize) as *mut c_void, buf.as_ptr() as *const c_void, pos as usize);
    pos
}

pub unsafe fn ieee802154_mac_cmd_push(skb: *mut sk_buff, f: *mut c_void, pl: *const c_void, pl_len: u32) -> i32 {
    let frame = f as *mut ieee802154_mac_cmd_frame;
    let mac_pl = &(*frame).mac_pl;
    let mhr = &(*frame).mhr;
    skb_reserve(skb, core::mem::size_of_val(mhr));
    let ret = ieee802154_hdr_push(skb, mhr as *const _ as *mut _);
    if ret < 0 { return ret; }
    skb_reset_mac_header(skb);
    (*skb).mac_len = ret as _;
    skb_put_data(skb, mac_pl as *const _ as *const c_void, core::mem::size_of_val(mac_pl));
    skb_put_data(skb, pl, pl_len as usize);
    0
}

pub unsafe fn ieee802154_beacon_push(skb: *mut sk_buff, beacon: *mut ieee802154_beacon_frame) -> i32 {
    let mac_pl = &(*beacon).mac_pl;
    let mhr = &(*beacon).mhr;
    skb_reserve(skb, core::mem::size_of_val(mhr));
    let ret = ieee802154_hdr_push(skb, mhr as *const _ as *mut _);
    if ret < 0 { return ret; }
    skb_reset_mac_header(skb);
    (*skb).mac_len = ret as _;
    skb_put_data(skb, mac_pl as *const _ as *const c_void, core::mem::size_of_val(mac_pl));
    if mac_pl.pend_short_addr_count != 0 || mac_pl.pend_ext_addr_count != 0 { return -EOPNOTSUPP; }
    0
}

unsafe fn ieee802154_hdr_get_addr(buf: *const u8, mode: i32, omit_pan: bool, addr: *mut ieee802154_addr) -> i32 {
    let mut pos: i32 = 0;
    if mode == IEEE802154_ADDR_NONE {
        memset(addr as *mut c_void, 0, core::mem::size_of::<ieee802154_addr>());
        (*addr).mode = IEEE802154_ADDR_NONE;
        return 0;
    }
    (*addr).mode = mode;
    if !omit_pan { memcpy(&mut (*addr).pan_id as *mut _ as *mut c_void, buf.add(pos as usize) as *const c_void, 2); pos += 2; }
    if mode == IEEE802154_ADDR_SHORT {
        memcpy(&mut (*addr).short_addr as *mut _ as *mut c_void, buf.add(pos as usize) as *const c_void, 2);
        pos + 2
    } else {
        memcpy(&mut (*addr).extended_addr as *mut _ as *mut c_void, buf.add(pos as usize) as *const c_void, IEEE802154_ADDR_LEN as usize);
        pos + IEEE802154_ADDR_LEN
    }
}

unsafe fn ieee802154_hdr_addr_len(mode: i32, omit_pan: bool) -> i32 {
    let pan_len = if omit_pan { 0 } else { 2 };
    match mode { IEEE802154_ADDR_NONE => 0, IEEE802154_ADDR_SHORT => 2 + pan_len, IEEE802154_ADDR_LONG => IEEE802154_ADDR_LEN + pan_len, _ => -EINVAL }
}

unsafe fn ieee802154_hdr_get_sechdr(buf: *const u8, hdr: *mut ieee802154_sechdr) -> i32 {
    let mut pos: i32 = 5;
    memcpy(hdr as *mut c_void, buf as *const c_void, 1);
    memcpy(&mut (*hdr).frame_counter as *mut _ as *mut c_void, buf.add(1) as *const c_void, 4);
    match (*hdr).key_id_mode {
        IEEE802154_SCF_KEY_IMPLICIT => return pos,
        IEEE802154_SCF_KEY_INDEX => {}
        IEEE802154_SCF_KEY_SHORT_INDEX => { memcpy(&mut (*hdr).short_src as *mut _ as *mut c_void, buf.add(pos as usize) as *const c_void, 4); pos += 4; }
        IEEE802154_SCF_KEY_HW_INDEX => { memcpy(&mut (*hdr).extended_src as *mut _ as *mut c_void, buf.add(pos as usize) as *const c_void, IEEE802154_ADDR_LEN as usize); pos += IEEE802154_ADDR_LEN; }
        _ => {}
    }
    (*hdr).key_id = *buf.add(pos as usize); pos += 1; pos
}

static IEEE802154_SECHDR_LENGTHS: [i32; 4] = [5, 6, 10, 14];

unsafe fn ieee802154_hdr_sechdr_len(sc: u8) -> i32 { IEEE802154_SECHDR_LENGTHS[IEEE802154_SCF_KEY_ID_MODE(sc) as usize] }

unsafe fn ieee802154_hdr_minlen(hdr: *const ieee802154_hdr) -> i32 {
    let dlen = ieee802154_hdr_addr_len((*hdr).fc.dest_addr_mode, false);
    let slen = ieee802154_hdr_addr_len((*hdr).fc.source_addr_mode, (*hdr).fc.intra_pan);
    if slen < 0 || dlen < 0 { return -EINVAL; }
    3 + dlen + slen + (*hdr).fc.security_enabled as i32
}

unsafe fn ieee802154_hdr_get_addrs(buf: *const u8, hdr: *mut ieee802154_hdr) -> i32 {
    let mut pos = 0;
    pos += ieee802154_hdr_get_addr(buf.add(pos as usize), (*hdr).fc.dest_addr_mode, false, &mut (*hdr).dest);
    pos += ieee802154_hdr_get_addr(buf.add(pos as usize), (*hdr).fc.source_addr_mode, (*hdr).fc.intra_pan, &mut (*hdr).source);
    if (*hdr).fc.intra_pan { (*hdr).source.pan_id = (*hdr).dest.pan_id; }
    pos
}

pub unsafe fn ieee802154_hdr_pull(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> i32 {
    let mut pos: i32 = 3;
    if !pskb_may_pull(skb, 3) { return -EINVAL; }
    memcpy(hdr as *mut c_void, (*skb).data as *const c_void, 3);
    let rc = ieee802154_hdr_minlen(hdr);
    if rc < 0 || !pskb_may_pull(skb, rc as usize) { return -EINVAL; }
    pos += ieee802154_hdr_get_addrs((*skb).data.add(pos as usize), hdr);
    if (*hdr).fc.security_enabled {
        let want = pos + ieee802154_hdr_sechdr_len(*(*skb).data.add(pos as usize));
        if !pskb_may_pull(skb, want as usize) { return -EINVAL; }
        pos += ieee802154_hdr_get_sechdr((*skb).data.add(pos as usize), &mut (*hdr).sec);
    }
    skb_pull(skb, pos as usize); pos
}

pub unsafe fn ieee802154_mac_cmd_pl_pull(skb: *mut sk_buff, mac_pl: *mut ieee802154_mac_cmd_pl) -> i32 {
    let len = core::mem::size_of::<ieee802154_mac_cmd_pl>();
    if !pskb_may_pull(skb, len) { return -EINVAL; }
    memcpy(mac_pl as *mut c_void, (*skb).data as *const c_void, len);
    skb_pull(skb, len); 0
}

pub unsafe fn ieee802154_hdr_peek_addrs(skb: *const sk_buff, hdr: *mut ieee802154_hdr) -> i32 {
    let buf = skb_mac_header(skb); let mut pos: i32 = 3;
    if buf.add(3) > skb_tail_pointer(skb) { return -EINVAL; }
    memcpy(hdr as *mut c_void, buf as *const c_void, 3);
    let rc = ieee802154_hdr_minlen(hdr);
    if rc < 0 || buf.add(rc as usize) > skb_tail_pointer(skb) { return -EINVAL; }
    pos += ieee802154_hdr_get_addrs(buf.add(pos as usize), hdr); pos
}

pub unsafe fn ieee802154_hdr_peek(skb: *const sk_buff, hdr: *mut ieee802154_hdr) -> i32 {
    let buf = skb_mac_header(skb); let mut pos = ieee802154_hdr_peek_addrs(skb, hdr);
    if pos < 0 { return -EINVAL; }
    if (*hdr).fc.security_enabled {
        let key_id_mode = IEEE802154_SCF_KEY_ID_MODE(*buf.add(pos as usize));
        let want = pos + IEEE802154_SECHDR_LENGTHS[key_id_mode as usize];
        if buf.add(want as usize) > skb_tail_pointer(skb) { return -EINVAL; }
        pos += ieee802154_hdr_get_sechdr(buf.add(pos as usize), &mut (*hdr).sec);
    }
    pos
}

pub unsafe fn ieee802154_max_payload(hdr: *const ieee802154_hdr) -> i32 {
    let mut hlen = ieee802154_hdr_minlen(hdr);
    if (*hdr).fc.security_enabled {
        hlen += IEEE802154_SECHDR_LENGTHS[(*hdr).sec.key_id_mode as usize] - 1;
        hlen += ieee802154_sechdr_authtag_len(&(*hdr).sec);
    }
    IEEE802154_MTU - hlen - IEEE802154_MFR_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
