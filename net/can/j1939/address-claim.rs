// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2010-2011 EIA Electronics,
//                         Kurt Van Dijck <kurt.van.dijck@eia.be>
// Copyright (c) 2010-2011 EIA Electronics,
//                         Pieter Beyens <pieter.beyens@eia.be>
// Copyright (c) 2017-2019 Pengutronix,
//                         Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2017-2019 Pengutronix,
//                         Oleksij Rempel <kernel@pengutronix.de>

/* J1939 Address Claiming. */

use core::ptr;

// Supplied by j1939-priv.h and the kernel compatibility layer.
pub type NameT = u64;
pub type U8 = u8;

#[repr(C)]
pub struct SkBuff { pub data: *mut u8, pub len: usize }
#[repr(C)]
pub struct J1939Addr { pub pgn: u32, pub src_name: NameT, pub dst_name: NameT, pub sa: U8, pub da: U8 }
#[repr(C)]
pub struct J1939SkBuffCb { pub addr: J1939Addr }
#[repr(C)]
pub struct J1939Ecu { pub addr: U8, pub name: NameT }
#[repr(C)]
pub struct J1939Priv { pub ndev: *mut (), pub lock: () }

pub const J1939_PGN_REQUEST: u32 = 0x00ea00;
pub const J1939_PGN_ADDRESS_CLAIMED: u32 = 0x00ee00;
pub const J1939_NO_ADDR: U8 = 0xff;
pub const EPROTO: i32 = 71;
pub const ENODEV: i32 = 19;
pub const EADDRNOTAVAIL: i32 = 99;

extern "C" {
    fn j1939_skb_to_cb(skb: *mut SkBuff) -> *mut J1939SkBuffCb;
    fn j1939_name_to_addr(priv_: *mut J1939Priv, name: NameT) -> U8;
    fn j1939_address_is_unicast(addr: U8) -> bool;
    fn j1939_address_is_valid(addr: U8) -> bool;
    fn j1939_address_is_idle(addr: U8) -> bool;
    fn j1939_ecu_get_by_name(priv_: *mut J1939Priv, name: NameT) -> *mut J1939Ecu;
    fn j1939_ecu_get_by_name_locked(priv_: *mut J1939Priv, name: NameT) -> *mut J1939Ecu;
    fn j1939_ecu_create_locked(priv_: *mut J1939Priv, name: NameT) -> *mut J1939Ecu;
    fn j1939_ecu_get_by_addr_locked(priv_: *mut J1939Priv, addr: U8) -> *mut J1939Ecu;
    fn j1939_ecu_get_by_addr(priv_: *mut J1939Priv, addr: U8) -> *mut J1939Ecu;
    fn j1939_ecu_unmap(ecu: *mut J1939Ecu);
    fn j1939_ecu_unmap_locked(ecu: *mut J1939Ecu);
    fn j1939_ecu_put(ecu: *mut J1939Ecu);
    fn j1939_ecu_timer_cancel(ecu: *mut J1939Ecu);
    fn j1939_ecu_timer_start(ecu: *mut J1939Ecu);
    fn netdev_notice(ndev: *mut (), fmt: *const u8, ...);
    fn write_lock_bh(lock: *mut ());
    fn write_unlock_bh(lock: *mut ());
}

#[inline]
unsafe fn j1939_skb_to_name(skb: *const SkBuff) -> NameT {
    ptr::read_unaligned((*skb).data as *const NameT)
}

#[inline]
unsafe fn j1939_ac_msg_is_request(skb: *mut SkBuff) -> bool {
    let skcb = j1939_skb_to_cb(skb);
    if (*skb).len < 3 || (*skcb).addr.pgn != J1939_PGN_REQUEST { return false; }
    let d = (*skb).data;
    let req_pgn = (*d as u32) | ((*d.add(1) as u32) << 8) | ((*d.add(2) as u32) << 16);
    req_pgn == J1939_PGN_ADDRESS_CLAIMED
}

unsafe fn j1939_ac_verify_outgoing(priv_: *mut J1939Priv, skb: *mut SkBuff) -> i32 {
    let skcb = j1939_skb_to_cb(skb);
    if (*skb).len != 8 { netdev_notice((*priv_).ndev, b"tx address claim with dlc %i\0".as_ptr(), (*skb).len as i32); return -EPROTO; }
    if (*skcb).addr.src_name != j1939_skb_to_name(skb) { netdev_notice((*priv_).ndev, b"tx address claim with different name\0".as_ptr()); return -EPROTO; }
    if (*skcb).addr.sa == J1939_NO_ADDR { netdev_notice((*priv_).ndev, b"tx address claim with broadcast sa\0".as_ptr()); return -EPROTO; }
    if (*skcb).addr.dst_name != 0 || (*skcb).addr.da != J1939_NO_ADDR { netdev_notice((*priv_).ndev, b"tx address claim with dest, not broadcast\0".as_ptr()); return -EPROTO; }
    0
}

pub unsafe fn j1939_ac_fixup(priv_: *mut J1939Priv, skb: *mut SkBuff) -> i32 {
    let skcb = j1939_skb_to_cb(skb);
    if (*skcb).addr.pgn == J1939_PGN_ADDRESS_CLAIMED {
        let ret = j1939_ac_verify_outgoing(priv_, skb); if ret < 0 { return ret; }
        let ecu = j1939_ecu_get_by_name(priv_, (*skcb).addr.src_name); if ecu.is_null() { return -ENODEV; }
        if (*ecu).addr != (*skcb).addr.sa { j1939_ecu_unmap(ecu); } j1939_ecu_put(ecu);
    } else if (*skcb).addr.src_name != 0 {
        let addr = j1939_name_to_addr(priv_, (*skcb).addr.src_name);
        if !j1939_address_is_unicast(addr) && !j1939_ac_msg_is_request(skb) { netdev_notice((*priv_).ndev, b"tx drop: invalid sa for name 0x%016llx\0".as_ptr(), (*skcb).addr.src_name); return -EADDRNOTAVAIL; }
        (*skcb).addr.sa = addr;
    }
    if (*skcb).addr.dst_name != 0 {
        let addr = j1939_name_to_addr(priv_, (*skcb).addr.dst_name);
        if !j1939_address_is_unicast(addr) { netdev_notice((*priv_).ndev, b"tx drop: invalid da for name 0x%016llx\0".as_ptr(), (*skcb).addr.dst_name); return -EADDRNOTAVAIL; }
        (*skcb).addr.da = addr;
    }
    0
}

unsafe fn j1939_ac_process(priv_: *mut J1939Priv, skb: *mut SkBuff) {
    let skcb = j1939_skb_to_cb(skb);
    if (*skb).len != 8 { netdev_notice((*priv_).ndev, b"rx address claim with wrong dlc %i\0".as_ptr(), (*skb).len as i32); return; }
    let name = j1939_skb_to_name(skb); (*skcb).addr.src_name = name;
    if name == 0 { netdev_notice((*priv_).ndev, b"rx address claim without name\0".as_ptr()); return; }
    if !j1939_address_is_valid((*skcb).addr.sa) { netdev_notice((*priv_).ndev, b"rx address claim with broadcast sa\0".as_ptr()); return; }
    write_lock_bh(&mut (*priv_).lock);
    let mut ecu = j1939_ecu_get_by_name_locked(priv_, name);
    if !ecu.is_null() && (*ecu).addr == (*skcb).addr.sa { j1939_ecu_put(ecu); write_unlock_bh(&mut (*priv_).lock); return; }
    if ecu.is_null() && j1939_address_is_unicast((*skcb).addr.sa) { ecu = j1939_ecu_create_locked(priv_, name); }
    if ecu.is_null() { write_unlock_bh(&mut (*priv_).lock); return; }
    j1939_ecu_timer_cancel(ecu);
    if j1939_address_is_idle((*skcb).addr.sa) { j1939_ecu_unmap_locked(ecu); j1939_ecu_put(ecu); write_unlock_bh(&mut (*priv_).lock); return; }
    if (*ecu).addr != (*skcb).addr.sa { j1939_ecu_unmap_locked(ecu); }
    (*ecu).addr = (*skcb).addr.sa;
    let prev = j1939_ecu_get_by_addr_locked(priv_, (*skcb).addr.sa);
    if !prev.is_null() {
        if (*ecu).name > (*prev).name { j1939_ecu_unmap_locked(ecu); j1939_ecu_put(prev); j1939_ecu_put(ecu); write_unlock_bh(&mut (*priv_).lock); return; }
        j1939_ecu_unmap_locked(prev); j1939_ecu_put(prev);
    }
    j1939_ecu_timer_start(ecu); j1939_ecu_put(ecu); write_unlock_bh(&mut (*priv_).lock);
}

pub unsafe fn j1939_ac_recv(priv_: *mut J1939Priv, skb: *mut SkBuff) {
    let skcb = j1939_skb_to_cb(skb);
    if (*skcb).addr.pgn == J1939_PGN_ADDRESS_CLAIMED { j1939_ac_process(priv_, skb); }
    else if j1939_address_is_unicast((*skcb).addr.sa) { let ecu = j1939_ecu_get_by_addr(priv_, (*skcb).addr.sa); if !ecu.is_null() { (*skcb).addr.src_name = (*ecu).name; j1939_ecu_put(ecu); } }
    let ecu = j1939_ecu_get_by_addr(priv_, (*skcb).addr.da); if !ecu.is_null() { (*skcb).addr.dst_name = (*ecu).name; j1939_ecu_put(ecu); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
