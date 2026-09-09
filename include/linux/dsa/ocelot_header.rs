/* SPDX-License-Identifier: GPL-2.0
 * Copyright 2019-2021 NXP
 */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff { pub cb: [u8; 48] }
#[repr(C)] pub struct dsa_port { _private: [u8; 0] }
#[repr(C)] pub struct kthread_work { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct dsa_switch { _private: [u8; 0] }
#[repr(C)] pub struct ocelot_skb_cb {
    pub clone: *mut sk_buff,
    pub ptp_class: u32,
    pub ptp_tx_time: usize,
    pub tstamp_lo: u32,
    pub ptp_cmd: u8,
    pub ts_id: u8,
}
#[repr(C)] pub struct felix_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}
#[repr(C)] pub struct ocelot_8021q_tagger_data {
    pub xmit_work_fn: Option<unsafe extern "C" fn(*mut kthread_work)>,
}

#[macro_export]
macro_rules! OCELOT_SKB_CB { ($skb:expr) => { ($skb as *mut u8).cast::<ocelot_skb_cb>() } }

pub const IFH_TAG_TYPE_C: u32 = 0;
pub const IFH_TAG_TYPE_S: u32 = 1;
pub const IFH_REW_OP_NOOP: u32 = 0x0;
pub const IFH_REW_OP_DSCP: u32 = 0x1;
pub const IFH_REW_OP_ONE_STEP_PTP: u32 = 0x2;
pub const IFH_REW_OP_TWO_STEP_PTP: u32 = 0x3;
pub const IFH_REW_OP_ORIGIN_PTP: u32 = 0x5;
pub const OCELOT_TAG_LEN: u32 = 16;
pub const OCELOT_SHORT_PREFIX_LEN: u32 = 4;
pub const OCELOT_LONG_PREFIX_LEN: u32 = 16;
pub const OCELOT_TOTAL_TAG_LEN: u32 = OCELOT_SHORT_PREFIX_LEN + OCELOT_TAG_LEN;

extern "C" {
    pub fn packing(buf: *mut c_void, value: *mut u64, hi: u32, lo: u32, width: u32, op: u32, quirks: u32);
    pub fn ocelot_8021q_tagger_data(ds: *mut dsa_switch) -> *mut ocelot_8021q_tagger_data;
    pub fn ocelot_skb_mac_header(skb: *mut sk_buff) -> *mut c_void;
    pub fn ocelot_br_vlan_enabled(br: *mut net_device) -> bool;
    pub fn ocelot_br_vlan_get_proto(br: *mut net_device, proto: *mut u16);
    pub fn ocelot_vlan_remove_tag(skb: *mut sk_buff, tci: *mut u16);
    pub fn ocelot_br_vlan_get_pvid_rcu(br: *mut net_device, tci: *mut u16);
    pub fn ocelot_rcu_read_lock();
    pub fn ocelot_rcu_read_unlock();
}
pub const UNPACK: u32 = 0;
pub const PACK: u32 = 1;
pub const ETH_P_8021Q: u16 = 0x8100;

pub unsafe fn ocelot_xfh_get_rew_val(extraction: *mut c_void, rew_val: *mut u64) { packing(extraction, rew_val, 116, 85, OCELOT_TAG_LEN, UNPACK, 0); }
pub unsafe fn ocelot_xfh_get_len(extraction: *mut c_void, len: *mut u64) { let mut llen=0; let mut wlen=0; packing(extraction,&mut llen,84,79,OCELOT_TAG_LEN,UNPACK,0); packing(extraction,&mut wlen,78,71,OCELOT_TAG_LEN,UNPACK,0); *len=60*wlen+llen-80; }
pub unsafe fn ocelot_xfh_get_src_port(extraction:*mut c_void, v:*mut u64){packing(extraction,v,46,43,OCELOT_TAG_LEN,UNPACK,0)}
pub unsafe fn ocelot_xfh_get_qos_class(extraction:*mut c_void, v:*mut u64){packing(extraction,v,19,17,OCELOT_TAG_LEN,UNPACK,0)}
pub unsafe fn ocelot_xfh_get_tag_type(extraction:*mut c_void, v:*mut u64){packing(extraction,v,16,16,OCELOT_TAG_LEN,UNPACK,0)}
pub unsafe fn ocelot_xfh_get_vlan_tci(extraction:*mut c_void, v:*mut u64){packing(extraction,v,15,0,OCELOT_TAG_LEN,UNPACK,0)}
pub unsafe fn ocelot_ifh_set_bypass(p:*mut c_void, mut v:u64){packing(p,&mut v,127,127,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_rew_op(p:*mut c_void, mut v:u64){packing(p,&mut v,125,117,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_dest(p:*mut c_void, mut v:u64){packing(p,&mut v,67,56,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_qos_class(p:*mut c_void, mut v:u64){packing(p,&mut v,19,17,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn seville_ifh_set_dest(p:*mut c_void, mut v:u64){packing(p,&mut v,67,57,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_src(p:*mut c_void, mut v:u64){packing(p,&mut v,46,43,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_tag_type(p:*mut c_void, mut v:u64){packing(p,&mut v,16,16,OCELOT_TAG_LEN,PACK,0)}
pub unsafe fn ocelot_ifh_set_vlan_tci(p:*mut c_void, mut v:u64){packing(p,&mut v,15,0,OCELOT_TAG_LEN,PACK,0)}

pub unsafe fn ocelot_ptp_rew_op(skb:*mut sk_buff)->u32 { let cb=OCELOT_SKB_CB!(skb); let clone=(*cb).clone; let cmd=(*cb).ptp_cmd; let mut rew=0; if cmd==IFH_REW_OP_TWO_STEP_PTP as u8 && !clone.is_null(){rew=cmd as u32; rew|=((*OCELOT_SKB_CB!(clone)).ts_id as u32)<<3;} else if cmd==IFH_REW_OP_ORIGIN_PTP as u8 {rew=cmd as u32;} rew }

/* Determine the PTP REW_OP to use for injecting the given skb. */
/*
 * If the port is under a VLAN-aware bridge, remove the VLAN header from the
 * payload and move it into the DSA tag. Otherwise the classified VLAN is zero.
 */
pub unsafe fn ocelot_xmit_get_vlan_info(skb:*mut sk_buff, br:*mut net_device, vlan_tci:*mut u64, tag_type:*mut u64) {
    let mut proto=0u16; let mut tci=0u16;
    if br.is_null() || !ocelot_br_vlan_enabled(br) { *vlan_tci=0; *tag_type=IFH_TAG_TYPE_C as u64; return; }
    ocelot_br_vlan_get_proto(br,&mut proto);
    let hdr=ocelot_skb_mac_header(skb) as *mut u16;
    if u16::from_be((*hdr.add(6))) == proto { ocelot_vlan_remove_tag(skb,&mut tci); *vlan_tci=tci as u64; }
    else { ocelot_rcu_read_lock(); ocelot_br_vlan_get_pvid_rcu(br,&mut tci); ocelot_rcu_read_unlock(); *vlan_tci=tci as u64; }
    *tag_type=if proto!=ETH_P_8021Q {IFH_TAG_TYPE_S as u64} else {IFH_TAG_TYPE_C as u64};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
