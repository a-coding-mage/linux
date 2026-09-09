// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
 *
 * This module is not a complete tagger implementation. It only provides
 * primitives for taggers that rely on 802.1Q VLAN tags to use.
 */

// Linux dependencies supplied by the surrounding translation unit.

const DSA_8021Q_RSV_VAL: u16 = 3;
const DSA_8021Q_RSV_SHIFT: u32 = 10;
const DSA_8021Q_RSV_MASK: u16 = 0xc00;
const DSA_8021Q_RSV: u16 = (DSA_8021Q_RSV_VAL << DSA_8021Q_RSV_SHIFT) & DSA_8021Q_RSV_MASK;

const DSA_8021Q_SWITCH_ID_SHIFT: u32 = 6;
const DSA_8021Q_SWITCH_ID_MASK: u16 = 0x1c0;
const DSA_8021Q_VBID_HI_SHIFT: u32 = 9;
const DSA_8021Q_VBID_HI_MASK: u16 = 0x200;
const DSA_8021Q_VBID_LO_SHIFT: u32 = 4;
const DSA_8021Q_VBID_LO_MASK: u16 = 0x30;
const DSA_8021Q_PORT_SHIFT: u32 = 0;
const DSA_8021Q_PORT_MASK: u16 = 0x0f;

#[inline] fn dsa_8021q_switch_id(x: u16) -> u16 { (x << DSA_8021Q_SWITCH_ID_SHIFT) & DSA_8021Q_SWITCH_ID_MASK }
#[inline] fn dsa_8021q_vbid_hi(x: u16) -> u16 { (x & 0x4) >> 2 }
#[inline] fn dsa_8021q_vbid_lo(x: u16) -> u16 { x & 0x3 }
#[inline] fn dsa_8021q_vbid(x: u16) -> u16 {
    ((dsa_8021q_vbid_lo(x) << DSA_8021Q_VBID_LO_SHIFT) & DSA_8021Q_VBID_LO_MASK) |
    ((dsa_8021q_vbid_hi(x) << DSA_8021Q_VBID_HI_SHIFT) & DSA_8021Q_VBID_HI_MASK)
}
#[inline] fn dsa_8021q_port(x: u16) -> u16 { (x << DSA_8021Q_PORT_SHIFT) & DSA_8021Q_PORT_MASK }

#[repr(C)] pub struct DsaTag8021qVlan { pub list: ListHead, pub port: i32, pub vid: u16, pub refcount: RefcountT }
#[repr(C)] pub struct Dsa8021qContext { pub ds: *mut DsaSwitch, pub vlans: ListHead, pub proto: Be16 }

pub unsafe fn dsa_tag_8021q_bridge_vid(bridge_num: u32) -> u16 { DSA_8021Q_RSV | dsa_8021q_vbid(bridge_num as u16) }
pub unsafe fn dsa_tag_8021q_standalone_vid(dp: *const DsaPort) -> u16 {
    DSA_8021Q_RSV | dsa_8021q_switch_id((*(*dp).ds).index as u16) | dsa_8021q_port((*dp).index as u16)
}
pub fn dsa_8021q_rx_switch_id(vid: u16) -> i32 { ((vid & DSA_8021Q_SWITCH_ID_MASK) >> DSA_8021Q_SWITCH_ID_SHIFT) as i32 }
pub fn dsa_8021q_rx_source_port(vid: u16) -> i32 { ((vid & DSA_8021Q_PORT_MASK) >> DSA_8021Q_PORT_SHIFT) as i32 }
unsafe fn dsa_tag_8021q_rx_vbid(vid: u16) -> i32 {
    let hi = (vid & DSA_8021Q_VBID_HI_MASK) >> DSA_8021Q_VBID_HI_SHIFT;
    let lo = (vid & DSA_8021Q_VBID_LO_MASK) >> DSA_8021Q_VBID_LO_SHIFT;
    ((hi << 2) | lo) as i32
}
pub fn vid_is_dsa_8021q(vid: u16) -> bool { ((vid & DSA_8021Q_RSV_MASK) >> DSA_8021Q_RSV_SHIFT) == DSA_8021Q_RSV_VAL }

unsafe fn dsa_tag_8021q_vlan_find(ctx: *mut Dsa8021qContext, port: i32, vid: u16) -> *mut DsaTag8021qVlan {
    let mut v: *mut DsaTag8021qVlan = core::ptr::null_mut();
    list_for_each_entry!(v, &mut (*ctx).vlans, list) {
        if (*v).vid == vid && (*v).port == port { return v; }
    }
    core::ptr::null_mut()
}

unsafe fn dsa_port_do_tag_8021q_vlan_add(dp: *mut DsaPort, vid: u16, flags: u16) -> i32 {
    let ctx = (*(*dp).ds).tag_8021q_ctx;
    let ds = (*dp).ds;
    let port = (*dp).index;
    if !(dsa_port_is_cpu(dp) || dsa_port_is_dsa(dp)) { return ((*(*ds).ops).tag_8021q_vlan_add)(ds, port, vid, flags); }
    let v = dsa_tag_8021q_vlan_find(ctx, port, vid);
    if !v.is_null() { refcount_inc!(&mut (*v).refcount); return 0; }
    let v = kzalloc_obj!(DsaTag8021qVlan);
    if v.is_null() { return -12; }
    let err = ((*(*ds).ops).tag_8021q_vlan_add)(ds, port, vid, flags);
    if err != 0 { kfree!(v); return err; }
    (*v).vid = vid; (*v).port = port; refcount_set!(&mut (*v).refcount, 1); list_add_tail!(&mut (*v).list, &mut (*ctx).vlans); 0
}

unsafe fn dsa_port_do_tag_8021q_vlan_del(dp: *mut DsaPort, vid: u16) -> i32 {
    let ctx = (*(*dp).ds).tag_8021q_ctx; let ds = (*dp).ds; let port = (*dp).index;
    if !(dsa_port_is_cpu(dp) || dsa_port_is_dsa(dp)) { return ((*(*ds).ops).tag_8021q_vlan_del)(ds, port, vid); }
    let v = dsa_tag_8021q_vlan_find(ctx, port, vid); if v.is_null() { return -2; }
    if !refcount_dec_and_test!(&mut (*v).refcount) { return 0; }
    let err = ((*(*ds).ops).tag_8021q_vlan_del)(ds, port, vid); if err != 0 { refcount_set!(&mut (*v).refcount, 1); return err; }
    list_del!(&mut (*v).list); kfree!(v); 0
}

unsafe fn dsa_port_tag_8021q_vlan_match(dp: *mut DsaPort, info: *mut DsaNotifierTag8021qVlanInfo) -> bool { dsa_port_is_dsa(dp) || dsa_port_is_cpu(dp) || dp == (*info).dp }

pub unsafe fn dsa_switch_tag_8021q_vlan_add(ds: *mut DsaSwitch, info: *mut DsaNotifierTag8021qVlanInfo) -> i32 {
    if (*(*ds).ops).tag_8021q_vlan_add.is_none() || (*ds).tag_8021q_ctx.is_null() { return 0; }
    let mut dp: *mut DsaPort = core::ptr::null_mut();
    dsa_switch_for_each_port!(dp, ds) { if dsa_port_tag_8021q_vlan_match(dp, info) { let mut flags = 0; if dsa_port_is_user(dp) { flags = BRIDGE_VLAN_INFO_UNTAGGED | BRIDGE_VLAN_INFO_PVID; } let err = dsa_port_do_tag_8021q_vlan_add(dp, (*info).vid, flags); if err != 0 { return err; } } } 0
}
pub unsafe fn dsa_switch_tag_8021q_vlan_del(ds: *mut DsaSwitch, info: *mut DsaNotifierTag8021qVlanInfo) -> i32 {
    if (*(*ds).ops).tag_8021q_vlan_del.is_none() || (*ds).tag_8021q_ctx.is_null() { return 0; }
    let mut dp: *mut DsaPort = core::ptr::null_mut(); dsa_switch_for_each_port!(dp, ds) { if dsa_port_tag_8021q_vlan_match(dp, info) { let err = dsa_port_do_tag_8021q_vlan_del(dp, (*info).vid); if err != 0 { return err; } } } 0
}

pub unsafe fn dsa_tag_8021q_bridge_join(ds: *mut DsaSwitch, port: i32, bridge: DsaBridge, tx_fwd_offload: *mut bool, _extack: *mut NetlinkExtAck) -> i32 {
    let dp = dsa_to_port(ds, port); let standalone_vid = dsa_tag_8021q_standalone_vid(dp); let bridge_vid = dsa_tag_8021q_bridge_vid(bridge.num);
    let err = dsa_port_tag_8021q_vlan_add(dp, bridge_vid, true); if err != 0 { return err; } dsa_port_tag_8021q_vlan_del(dp, standalone_vid, false); *tx_fwd_offload = true; 0
}
pub unsafe fn dsa_tag_8021q_bridge_leave(ds: *mut DsaSwitch, port: i32, bridge: DsaBridge) { let dp = dsa_to_port(ds, port); let standalone_vid = dsa_tag_8021q_standalone_vid(dp); let bridge_vid = dsa_tag_8021q_bridge_vid(bridge.num); let err = dsa_port_tag_8021q_vlan_add(dp, standalone_vid, false); if err != 0 { dev_err!((*ds).dev, "Failed to delete tag_8021q standalone VLAN %d from port %d: %pe\n", standalone_vid, port, err); } dsa_port_tag_8021q_vlan_del(dp, bridge_vid, true); }

unsafe fn dsa_tag_8021q_port_setup(ds: *mut DsaSwitch, port: i32) -> i32 { let ctx = (*ds).tag_8021q_ctx; let dp = dsa_to_port(ds, port); if !dsa_port_is_user(dp) { return 0; } let conduit = dsa_port_to_conduit(dp); let vid = dsa_tag_8021q_standalone_vid(dp); let err = dsa_port_tag_8021q_vlan_add(dp, vid, false); if err != 0 { return err; } vlan_vid_add!(conduit, (*ctx).proto, vid); err }
unsafe fn dsa_tag_8021q_port_teardown(ds: *mut DsaSwitch, port: i32) { let ctx = (*ds).tag_8021q_ctx; let dp = dsa_to_port(ds, port); if !dsa_port_is_user(dp) { return; } let conduit = dsa_port_to_conduit(dp); let vid = dsa_tag_8021q_standalone_vid(dp); dsa_port_tag_8021q_vlan_del(dp, vid, false); vlan_vid_del!(conduit, (*ctx).proto, vid); }
unsafe fn dsa_tag_8021q_setup(ds: *mut DsaSwitch) -> i32 { ASSERT_RTNL!(); for port in 0..(*ds).num_ports { let err = dsa_tag_8021q_port_setup(ds, port); if err < 0 { return err; } } 0 }
unsafe fn dsa_tag_8021q_teardown(ds: *mut DsaSwitch) { ASSERT_RTNL!(); for port in 0..(*ds).num_ports { dsa_tag_8021q_port_teardown(ds, port); } }

pub unsafe fn dsa_tag_8021q_register(ds: *mut DsaSwitch, proto: Be16) -> i32 { let ctx = kzalloc_obj!(Dsa8021qContext); if ctx.is_null() { return -12; } (*ctx).proto = proto; (*ctx).ds = ds; INIT_LIST_HEAD!(&mut (*ctx).vlans); (*ds).tag_8021q_ctx = ctx; let err = dsa_tag_8021q_setup(ds); if err != 0 { kfree!(ctx); return err; } 0 }
pub unsafe fn dsa_tag_8021q_unregister(ds: *mut DsaSwitch) { let ctx = (*ds).tag_8021q_ctx; dsa_tag_8021q_teardown(ds); let mut v: *mut DsaTag8021qVlan = core::ptr::null_mut(); let mut n: *mut DsaTag8021qVlan = core::ptr::null_mut(); list_for_each_entry_safe!(v, n, &mut (*ctx).vlans, list) { list_del!(&mut (*v).list); kfree!(v); } (*ds).tag_8021q_ctx = core::ptr::null_mut(); kfree!(ctx); }

pub unsafe fn dsa_8021q_xmit(skb: *mut SkBuff, _netdev: *mut NetDevice, tpid: u16, tci: u16) -> *mut SkBuff { vlan_insert_tag(skb, htons(tpid), tci) }
unsafe fn dsa_tag_8021q_find_port_by_vbid(conduit: *mut NetDevice, vbid: i32) -> *mut NetDevice { let cpu_dp = (*conduit).dsa_ptr; let dst = (*cpu_dp).dst; if vbid == 0 { return core::ptr::null_mut(); } let mut dp: *mut DsaPort = core::ptr::null_mut(); dsa_tree_for_each_user_port!(dp, dst) { if (*dp).bridge.is_null() || ((*dp).stp_state != BR_STATE_LEARNING && (*dp).stp_state != BR_STATE_FORWARDING) || (*dp).cpu_dp != cpu_dp { continue; } if dsa_port_bridge_num_get(dp) == vbid { return (*dp).user; } } core::ptr::null_mut() }
pub unsafe fn dsa_tag_8021q_find_user(conduit: *mut NetDevice, source_port: i32, switch_id: i32, vid: i32, vbid: i32) -> *mut NetDevice { if source_port != -1 && switch_id != -1 { dsa_conduit_find_user(conduit, switch_id, source_port) } else if vbid >= 1 { dsa_tag_8021q_find_port_by_vbid(conduit, vbid) } else { dsa_find_designated_bridge_port_by_vid(conduit, vid) } }

pub unsafe fn dsa_8021q_rcv(skb: *mut SkBuff, source_port: *mut i32, switch_id: *mut i32, vbid: *mut i32, vid: *mut i32) {
    let (vlan_proto, tci) = if skb_vlan_tag_present(skb) { let p = (*skb).vlan_proto; let t = skb_vlan_tag_get(skb); __vlan_hwaccel_clear_tag!(skb); (p, t) } else { let hdr = vlan_eth_hdr(skb); let p = (*hdr).h_vlan_proto; skb_push_rcsum(skb, ETH_HLEN); let mut t = 0; __skb_vlan_pop!(skb, &mut t); skb_pull_rcsum(skb, ETH_HLEN); (p, t) };
    let tmp_vid = tci & VLAN_VID_MASK; if !vid_is_dsa_8021q(tmp_vid) { if !vid.is_null() { *vid = tmp_vid as i32; } __vlan_hwaccel_put_tag!(skb, vlan_proto, tci); return; }
    let sp = dsa_8021q_rx_source_port(tmp_vid); let si = dsa_8021q_rx_switch_id(tmp_vid); let vb = dsa_tag_8021q_rx_vbid(tmp_vid);
    if vb == 0 && *source_port == -1 { *source_port = sp; } if vb == 0 && *switch_id == -1 { *switch_id = si; } if !vbid.is_null() { *vbid = vb; } (*skb).priority = ((tci & VLAN_PRIO_MASK) >> VLAN_PRIO_SHIFT) as u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
