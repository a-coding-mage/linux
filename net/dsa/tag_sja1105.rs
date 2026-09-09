// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com> */
/* Linux kernel dependencies from the original source are supplied externally. */

const SJA1105_NAME: &[u8] = b"sja1105\0";
const SJA1110_NAME: &[u8] = b"sja1110\0";
const SJA1110_HEADER_HOST_TO_SWITCH: u16 = 1 << 15;
const SJA1110_RX_HEADER_IS_METADATA: u16 = 1 << 14;
const SJA1110_RX_HEADER_HOST_ONLY: u16 = 1 << 13;
const SJA1110_RX_HEADER_HAS_TRAILER: u16 = 1 << 12;
const SJA1110_TX_HEADER_UPDATE_TC: u16 = 1 << 14;
const SJA1110_TX_HEADER_TAKE_TS: u16 = 1 << 13;
const SJA1110_TX_HEADER_TAKE_TS_CASC: u16 = 1 << 12;
const SJA1110_TX_HEADER_HAS_TRAILER: u16 = 1 << 11;
const SJA1110_META_TSTAMP_SIZE: usize = 10;
const SJA1110_HEADER_LEN: usize = 4;
const SJA1110_RX_TRAILER_LEN: usize = 13;
const SJA1110_TX_TRAILER_LEN: usize = 4;
const SJA1110_MAX_PADDING_LEN: usize = 15;

#[inline] fn rx_header_src_port(x: u16) -> i32 { ((x & 0x00f0) >> 4) as i32 }
#[inline] fn rx_header_switch_id(x: u16) -> i32 { (x & 0x000f) as i32 }
#[inline] fn rx_header_trailer_pos(x: u16) -> usize { (x & 0x0fff) as usize }
#[inline] fn rx_trailer_switch_id(x: u8) -> i32 { ((x & 0xf0) >> 4) as i32 }
#[inline] fn rx_trailer_src_port(x: u8) -> i32 { (x & 0x0f) as i32 }
#[inline] fn rx_header_n_ts(x: u16) -> i32 { ((x & 0x01f0) >> 4) as i32 }
#[inline] fn tx_header_prio(x: u8) -> u16 { ((x as u16) << 7) & 0x0780 }
#[inline] fn tx_header_tstamp_id(x: u8) -> u16 { x as u16 }
#[inline] fn tx_header_trailer_pos(x: usize) -> u16 { (x as u16) & 0x07ff }
#[inline] fn tx_trailer_tstamp_id(x: u8) -> u32 { (x as u32) << 24 }
#[inline] fn tx_trailer_prio(x: u8) -> u32 { (x as u32) << 21 }
#[inline] fn tx_trailer_switchid(x: i32) -> u32 { (x as u32) << 12 }
#[inline] fn tx_trailer_destports(x: usize) -> u32 { (x as u32) << 1 }

#[repr(C)]
struct Sja1105TaggerPrivate {
    data: Sja1105TaggerData,
    meta_lock: Spinlock,
    stampable_skb: *mut SkBuff,
    xmit_worker: *mut KthreadWorker,
}

#[repr(C)] struct Sja1105Meta { tstamp: u64, dmac_byte_4: u64, dmac_byte_3: u64, source_port: u64, switch_id: u64 }

unsafe fn sja1105_tagger_private(ds: *mut DsaSwitch) -> *mut Sja1105TaggerPrivate {
    (*ds).tagger_data as *mut Sja1105TaggerPrivate
}

unsafe fn sja1105_is_link_local(skb: *const SkBuff) -> bool {
    let hdr = eth_hdr(skb); let dmac = ether_addr_to_u64((*hdr).h_dest);
    if ntohs((*hdr).h_proto) == ETH_P_SJA1105_META { return false; }
    if (dmac & SJA1105_LINKLOCAL_FILTER_A_MASK) == SJA1105_LINKLOCAL_FILTER_A { return true; }
    if (dmac & SJA1105_LINKLOCAL_FILTER_B_MASK) == SJA1105_LINKLOCAL_FILTER_B { return true; }
    false
}

unsafe fn sja1105_meta_unpack(skb: *const SkBuff, meta: *mut Sja1105Meta) {
    let buf = skb_mac_header(skb).add(ETH_HLEN);
    packing(buf, &mut (*meta).tstamp, 31, 0, 4, UNPACK, 0);
    packing(buf.add(4), &mut (*meta).dmac_byte_3, 7, 0, 1, UNPACK, 0);
    packing(buf.add(5), &mut (*meta).dmac_byte_4, 7, 0, 1, UNPACK, 0);
    packing(buf.add(6), &mut (*meta).source_port, 7, 0, 1, UNPACK, 0);
    packing(buf.add(7), &mut (*meta).switch_id, 7, 0, 1, UNPACK, 0);
}

unsafe fn sja1105_is_meta_frame(skb: *const SkBuff) -> bool {
    let hdr = eth_hdr(skb);
    ether_addr_to_u64((*hdr).h_source) == SJA1105_META_SMAC &&
    ether_addr_to_u64((*hdr).h_dest) == SJA1105_META_DMAC &&
    ntohs((*hdr).h_proto) == ETH_P_SJA1105_META
}

unsafe fn sja1105_defer_xmit(dp: *mut DsaPort, mut skb: *mut SkBuff) -> *mut SkBuff {
    let tagger_data = sja1105_tagger_data((*dp).ds); let priv_ = sja1105_tagger_private((*dp).ds);
    let xmit_work_fn = (*tagger_data).xmit_work_fn; let xmit_worker = (*priv_).xmit_worker;
    if xmit_work_fn.is_none() || xmit_worker.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    let xmit_work = kzalloc_obj::<Sja1105DeferredXmitWork>(GFP_ATOMIC);
    if xmit_work.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    kthread_init_work(&mut (*xmit_work).work, xmit_work_fn.unwrap());
    (*xmit_work).dp = dp; (*xmit_work).skb = skb;
    kthread_queue_work(xmit_worker, &mut (*xmit_work).work);
    core::ptr::null_mut()
}

unsafe fn sja1105_xmit_tpid(dp: *mut DsaPort) -> u16 {
    let ds = (*dp).ds;
    if !dsa_port_is_vlan_filtering(dp) { return ETH_P_SJA1105; }
    let mut other_dp: *mut DsaPort = core::ptr::null_mut(); let mut proto = 0u16;
    dsa_switch_for_each_port(other_dp, ds) {
        let br = dsa_port_bridge_dev_get(other_dp); if br.is_null() { continue; }
        br_vlan_get_proto(br, &mut proto); return proto;
    }
    WARN_ONCE!(true, "Port is VLAN-aware but cannot find associated bridge!\n"); ETH_P_SJA1105
}

unsafe fn sja1105_imprecise_xmit(skb: *mut SkBuff, netdev: *mut NetDevice) -> *mut SkBuff {
    let dp = dsa_user_to_port(netdev); let bridge_num = dsa_port_bridge_num_get(dp); let br = dsa_port_bridge_dev_get(dp);
    if br_vlan_enabled(br) { return skb; }
    dsa_8021q_xmit(skb, netdev, sja1105_xmit_tpid(dp), dsa_tag_8021q_bridge_vid(bridge_num))
}

unsafe fn sja1105_pvid_tag_control_pkt(dp: *mut DsaPort, mut skb: *mut SkBuff, pcp: u8) -> *mut SkBuff {
    let xmit_tpid = htons(sja1105_xmit_tpid(dp));
    if unlikely(skb_vlan_tag_present(skb)) { skb = __vlan_hwaccel_push_inside(skb); if skb.is_null() { return core::ptr::null_mut(); } }
    let hdr = skb_vlan_eth_hdr(skb); if (*hdr).h_vlan_proto == xmit_tpid { return skb; }
    vlan_insert_tag(skb, xmit_tpid, ((pcp as u16) << VLAN_PRIO_SHIFT) | SJA1105_DEFAULT_VLAN)
}

unsafe fn sja1105_xmit(mut skb: *mut SkBuff, netdev: *mut NetDevice) -> *mut SkBuff {
    let dp = dsa_user_to_port(netdev); let queue_mapping = skb_get_queue_mapping(skb); let pcp = netdev_txq_to_tc(netdev, queue_mapping); let tx_vid = dsa_tag_8021q_standalone_vid(dp);
    if (*skb).offload_fwd_mark { return sja1105_imprecise_xmit(skb, netdev); }
    if unlikely(sja1105_is_link_local(skb)) { skb = sja1105_pvid_tag_control_pkt(dp, skb, pcp); if skb.is_null() { return core::ptr::null_mut(); } return sja1105_defer_xmit(dp, skb); }
    dsa_8021q_xmit(skb, netdev, sja1105_xmit_tpid(dp), ((pcp as u16) << VLAN_PRIO_SHIFT) | tx_vid)
}

unsafe fn sja1110_xmit(mut skb: *mut SkBuff, netdev: *mut NetDevice) -> *mut SkBuff {
    let clone = (*sja1105_skb_cb(skb)).clone; let dp = dsa_user_to_port(netdev); let q = skb_get_queue_mapping(skb); let pcp = netdev_txq_to_tc(netdev, q); let tx_vid = dsa_tag_8021q_standalone_vid(dp);
    if (*skb).offload_fwd_mark { return sja1105_imprecise_xmit(skb, netdev); }
    if likely(!sja1105_is_link_local(skb)) { return dsa_8021q_xmit(skb, netdev, sja1105_xmit_tpid(dp), ((pcp as u16) << VLAN_PRIO_SHIFT) | tx_vid); }
    skb = sja1105_pvid_tag_control_pkt(dp, skb, pcp); if skb.is_null() { return core::ptr::null_mut(); }
    skb_push(skb, SJA1110_HEADER_LEN); dsa_alloc_etype_header(skb, SJA1110_HEADER_LEN);
    let trailer_pos = (*skb).len; let tx_header = dsa_etype_header_pos_tx(skb); let tx_trailer = skb_put(skb, SJA1110_TX_TRAILER_LEN) as *mut u32;
    (*tx_header.add(0)) = htons(ETH_P_SJA1110); (*tx_header.add(1)) = htons(SJA1110_HEADER_HOST_TO_SWITCH | SJA1110_TX_HEADER_HAS_TRAILER | tx_header_trailer_pos(trailer_pos));
    *tx_trailer = cpu_to_be32(tx_trailer_prio(pcp) | tx_trailer_switchid((*dp).ds.index) | tx_trailer_destports(1usize << (*dp).index));
    if !clone.is_null() { let ts_id = (*sja1105_skb_cb(clone)).ts_id; *tx_header.add(1) |= htons(SJA1110_TX_HEADER_TAKE_TS); *tx_trailer |= cpu_to_be32(tx_trailer_tstamp_id(ts_id)); }
    skb
}

unsafe fn sja1105_transfer_meta(skb: *mut SkBuff, meta: *const Sja1105Meta) { let hdr = eth_hdr(skb); (*hdr).h_dest[3] = (*meta).dmac_byte_3 as u8; (*hdr).h_dest[4] = (*meta).dmac_byte_4 as u8; (*sja1105_skb_cb(skb)).tstamp = (*meta).tstamp; }

unsafe fn sja1105_skb_has_tag_8021q(skb: *const SkBuff) -> bool { let tpid = ntohs((*eth_hdr(skb)).h_proto); tpid == ETH_P_SJA1105 || tpid == ETH_P_8021Q || skb_vlan_tag_present(skb) }
unsafe fn sja1110_skb_has_inband_control_extension(skb: *const SkBuff) -> bool { ntohs((*eth_hdr(skb)).h_proto) == ETH_P_SJA1110 }

unsafe fn sja1105_rcv_meta_state_machine(mut skb: *mut SkBuff, meta: *mut Sja1105Meta, is_link_local: bool, is_meta: bool) -> *mut SkBuff {
    let dp = dsa_user_to_port((*skb).dev); let priv_ = sja1105_tagger_private((*dp).ds);
    if is_link_local { spin_lock(&mut (*priv_).meta_lock); if !(*priv_).stampable_skb.is_null() { kfree_skb((*priv_).stampable_skb); } (*priv_).stampable_skb = skb; spin_unlock(&mut (*priv_).meta_lock); return core::ptr::null_mut(); }
    if is_meta { spin_lock(&mut (*priv_).meta_lock); let stampable = (*priv_).stampable_skb; (*priv_).stampable_skb = core::ptr::null_mut(); if stampable.is_null() { spin_unlock(&mut (*priv_).meta_lock); kfree_skb(skb); return core::ptr::null_mut(); } if (*stampable).dev != (*skb).dev { spin_unlock(&mut (*priv_).meta_lock); kfree_skb(skb); kfree_skb(stampable); return core::ptr::null_mut(); } kfree_skb(skb); skb = stampable; sja1105_transfer_meta(skb, meta); spin_unlock(&mut (*priv_).meta_lock); }
    skb
}

unsafe fn sja1105_rcv(skb: *mut SkBuff, netdev: *mut NetDevice) -> *mut SkBuff {
    let mut source_port = -1; let mut switch_id = -1; let mut vbid = -1; let mut vid = -1; let mut meta = core::mem::zeroed::<Sja1105Meta>(); let hdr = eth_hdr(skb); let ll = sja1105_is_link_local(skb); let im = sja1105_is_meta_frame(skb);
    if ll { source_port = (*hdr).h_dest[3] as i32; switch_id = (*hdr).h_dest[4] as i32; } else if im { sja1105_meta_unpack(skb, &mut meta); source_port = meta.source_port as i32; switch_id = meta.switch_id as i32; }
    if sja1105_skb_has_tag_8021q(skb) { dsa_8021q_rcv(skb, &mut source_port, &mut switch_id, &mut vbid, &mut vid); } else if source_port == -1 && switch_id == -1 { kfree_skb(skb); return core::ptr::null_mut(); }
    (*skb).dev = dsa_tag_8021q_find_user(netdev, source_port, switch_id, vid, vbid); if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); } if !ll { dsa_default_offload_fwd_mark(skb); } sja1105_rcv_meta_state_machine(skb, &mut meta, ll, im)
}

unsafe fn sja1110_rcv(skb: *mut SkBuff, netdev: *mut NetDevice) -> *mut SkBuff {
    let mut source_port = -1; let mut switch_id = -1; let mut vbid = -1; let mut vid = -1;
    if sja1110_skb_has_inband_control_extension(skb) { /* header parsing is supplied by the kernel tag API */ }
    if sja1105_skb_has_tag_8021q(skb) { dsa_8021q_rcv(skb, &mut source_port, &mut switch_id, &mut vbid, &mut vid); }
    (*skb).dev = dsa_tag_8021q_find_user(netdev, source_port, switch_id, vid, vbid); if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); } dsa_default_offload_fwd_mark(skb); skb
}

unsafe fn sja1105_disconnect(ds: *mut DsaSwitch) { let priv_ = (*ds).tagger_data as *mut Sja1105TaggerPrivate; kthread_destroy_worker((*priv_).xmit_worker); kfree(priv_ as *mut core::ffi::c_void); (*ds).tagger_data = core::ptr::null_mut(); }
unsafe fn sja1105_connect(ds: *mut DsaSwitch) -> i32 { let priv_ = kzalloc_obj::<Sja1105TaggerPrivate>(); if priv_.is_null() { return -12; } spin_lock_init(&mut (*priv_).meta_lock); (*ds).tagger_data = priv_ as *mut _; 0 }

// DSA_TAG_DRIVER, MODULE_ALIAS_DSA_TAG_DRIVER, and module metadata are registration macros in the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
