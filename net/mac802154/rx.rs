// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2012 Siemens AG
 *
 * Written by:
 * Pavel Smolenskiy <pavel.smolenskiy@gmail.com>
 * Maxim Gorbachyov <maxim.gorbachev@siemens.com>
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Kernel and ieee802154 dependencies are supplied by the surrounding crate.

unsafe fn ieee802154_deliver_skb(skb: *mut sk_buff) -> i32 {
    (*skb).ip_summed = CHECKSUM_UNNECESSARY;
    (*skb).protocol = htons(ETH_P_IEEE802154);
    netif_receive_skb(skb)
}

pub unsafe fn mac802154_rx_beacon_worker(work: *mut work_struct) {
    let local = container_of!(work, ieee802154_local, rx_beacon_work);
    let mac_pkt = list_first_entry_or_null!(&(*local).rx_beacon_list, cfg802154_mac_pkt, node);
    if mac_pkt.is_null() { return; }
    mac802154_process_beacon(local, (*mac_pkt).skb, (*mac_pkt).page, (*mac_pkt).channel);
    list_del(&mut (*mac_pkt).node);
    kfree_skb((*mac_pkt).skb);
    kfree(mac_pkt as *mut _);
}

unsafe fn mac802154_should_answer_beacon_req(local: *mut ieee802154_local) -> bool {
    rcu_read_lock();
    let beacon_req = rcu_dereference!((*local).beacon_req);
    if beacon_req.is_null() { rcu_read_unlock(); return false; }
    let interval = (*beacon_req).interval;
    rcu_read_unlock();
    if !mac802154_is_beaconing(local) { return false; }
    interval == IEEE802154_ACTIVE_SCAN_DURATION
}

pub unsafe fn mac802154_rx_mac_cmd_worker(work: *mut work_struct) {
    let local = container_of!(work, ieee802154_local, rx_mac_cmd_work);
    let mac_pkt = list_first_entry_or_null!(&(*local).rx_mac_cmd_list, cfg802154_mac_pkt, node);
    if mac_pkt.is_null() { return; }
    let mut mac_cmd: u8 = 0;
    let rc = ieee802154_get_mac_cmd((*mac_pkt).skb, &mut mac_cmd);
    if rc != 0 { goto_out!(mac_pkt); }
    match mac_cmd {
        IEEE802154_CMD_BEACON_REQ => {
            dev_dbg!(&(*(*mac_pkt).sdata).dev.dev, "processing BEACON REQ\n");
            if mac802154_should_answer_beacon_req(local) { queue_delayed_work((*local).mac_wq, &mut (*local).beacon_work, 0); }
        }
        IEEE802154_CMD_ASSOCIATION_RESP => {
            dev_dbg!(&(*(*mac_pkt).sdata).dev.dev, "processing ASSOC RESP\n");
            if mac802154_is_associating(local) { mac802154_process_association_resp((*mac_pkt).sdata, (*mac_pkt).skb); }
        }
        IEEE802154_CMD_ASSOCIATION_REQ => {
            dev_dbg!(&(*(*mac_pkt).sdata).dev.dev, "processing ASSOC REQ\n");
            if (*(*mac_pkt).sdata).wpan_dev.iftype == NL802154_IFTYPE_COORD { mac802154_process_association_req((*mac_pkt).sdata, (*mac_pkt).skb); }
        }
        IEEE802154_CMD_DISASSOCIATION_NOTIFY => {
            dev_dbg!(&(*(*mac_pkt).sdata).dev.dev, "processing DISASSOC NOTIF\n");
            if (*(*mac_pkt).sdata).wpan_dev.iftype == NL802154_IFTYPE_COORD { mac802154_process_disassociation_notif((*mac_pkt).sdata, (*mac_pkt).skb); }
        }
        _ => {}
    }
    list_del(&mut (*mac_pkt).node);
    kfree_skb((*mac_pkt).skb);
    kfree(mac_pkt as *mut _);
}

unsafe fn ieee802154_subif_frame(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff, hdr: *const ieee802154_hdr) -> i32 {
    let wpan_phy = (*(*sdata).local).hw.phy;
    let wpan_dev = &mut (*sdata).wpan_dev;
    let cb = mac_cb(skb);
    let span = wpan_dev.pan_id;
    let sshort = wpan_dev.short_addr;
    if (*sdata).required_filtering == IEEE802154_FILTERING_3_SCAN && (*sdata).required_filtering > (*wpan_phy).filtering && (*cb).type_ != IEEE802154_FC_TYPE_BEACON { goto_fail!(skb); }
    match (*cb).dest.mode {
        IEEE802154_ADDR_NONE => { (*skb).pkt_type = if (*hdr).source.mode == IEEE802154_ADDR_NONE || wpan_dev.parent.is_null() { PACKET_HOST } else { PACKET_OTHERHOST }; }
        IEEE802154_ADDR_LONG => { (*skb).pkt_type = if ((*cb).dest.pan_id != span && (*cb).dest.pan_id != cpu_to_le16(IEEE802154_PANID_BROADCAST)) || (*cb).dest.extended_addr != wpan_dev.extended_addr { PACKET_OTHERHOST } else { PACKET_HOST }; }
        IEEE802154_ADDR_SHORT => { (*skb).pkt_type = if (*cb).dest.pan_id != span && (*cb).dest.pan_id != cpu_to_le16(IEEE802154_PANID_BROADCAST) { PACKET_OTHERHOST } else if (*cb).dest.short_addr == sshort { PACKET_HOST } else if (*cb).dest.short_addr == cpu_to_le16(IEEE802154_ADDR_BROADCAST) { PACKET_BROADCAST } else { PACKET_OTHERHOST }; }
        _ => goto_fail!(skb),
    }
    (*skb).dev = (*sdata).dev;
    if mac802154_llsec_decrypt(&mut (*sdata).sec, skb) != 0 { goto_fail!(skb); }
    (*sdata).dev.stats.rx_packets += 1;
    (*sdata).dev.stats.rx_bytes += (*skb).len;
    match (*mac_cb(skb)).type_ {
        IEEE802154_FC_TYPE_BEACON => {
            if !mac802154_is_scanning((*sdata).local) { goto_fail!(skb); }
            let mac_pkt = kzalloc_obj!(cfg802154_mac_pkt, GFP_ATOMIC);
            if mac_pkt.is_null() { goto_fail!(skb); }
            (*mac_pkt).skb = skb_get(skb); (*mac_pkt).sdata = sdata; (*mac_pkt).page = (*(*sdata).local).scan_page; (*mac_pkt).channel = (*(*sdata).local).scan_channel;
            list_add_tail(&mut (*mac_pkt).node, &mut (*(*sdata).local).rx_beacon_list); queue_work((*(*sdata).local).mac_wq, &mut (*(*sdata).local).rx_beacon_work); NET_RX_SUCCESS
        }
        IEEE802154_FC_TYPE_MAC_CMD => {
            let mac_pkt = kzalloc_obj!(cfg802154_mac_pkt, GFP_ATOMIC); if mac_pkt.is_null() { goto_fail!(skb); }
            (*mac_pkt).skb = skb_get(skb); (*mac_pkt).sdata = sdata; list_add_tail(&mut (*mac_pkt).node, &mut (*(*sdata).local).rx_mac_cmd_list); queue_work((*(*sdata).local).mac_wq, &mut (*(*sdata).local).rx_mac_cmd_work); NET_RX_SUCCESS
        }
        IEEE802154_FC_TYPE_ACK => goto_fail!(skb),
        IEEE802154_FC_TYPE_DATA => ieee802154_deliver_skb(skb),
        _ => goto_fail!(skb),
    }
}

unsafe fn ieee802154_print_addr(name: *const i8, addr: *const ieee802154_addr) {
    if (*addr).mode == IEEE802154_ADDR_NONE { pr_debug!("%s not present\n", name); return; }
    pr_debug!("%s PAN ID: %04x\n", name, le16_to_cpu((*addr).pan_id));
    if (*addr).mode == IEEE802154_ADDR_SHORT { pr_debug!("%s is short: %04x\n", name, le16_to_cpu((*addr).short_addr)); }
    else { let hw = swab64((*addr).extended_addr); pr_debug!("%s is hardware: %8phC\n", name, &hw); }
}

unsafe fn ieee802154_parse_frame_start(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> i32 {
    let cb = mac_cb(skb); skb_reset_mac_header(skb); let hlen = ieee802154_hdr_pull(skb, hdr); if hlen < 0 { return -EINVAL; }
    (*skb).mac_len = hlen as u32; pr_debug!("fc: %04x dsn: %02x\n", le16_to_cpup(&(*hdr).fc), (*hdr).seq);
    (*cb).type_ = (*hdr).fc.type_; (*cb).ackreq = (*hdr).fc.ack_request; (*cb).secen = (*hdr).fc.security_enabled;
    ieee802154_print_addr(c_str!("destination"), &(*hdr).dest); ieee802154_print_addr(c_str!("source"), &(*hdr).source); (*cb).source = (*hdr).source; (*cb).dest = (*hdr).dest; 0
}

unsafe fn __ieee802154_rx_handle_packet(local: *mut ieee802154_local, skb: *mut sk_buff) {
    let mut hdr = ieee802154_hdr::default(); if ieee802154_parse_frame_start(skb, &mut hdr) != 0 { return; }
    list_for_each_entry_rcu!(sdata, &(*local).interfaces, list, {
        if (*sdata).wpan_dev.iftype == NL802154_IFTYPE_MONITOR || !ieee802154_sdata_running(sdata) { continue; }
        if (*local).hw.phy.filtering < IEEE802154_FILTERING_4_FRAME_FIELDS && (*sdata).required_filtering == IEEE802154_FILTERING_4_FRAME_FIELDS { continue; }
        let skb2 = skb_clone(skb, GFP_ATOMIC); if !skb2.is_null() { (*skb2).dev = (*sdata).dev; ieee802154_subif_frame(sdata, skb2, &hdr); }
    });
}

unsafe fn ieee802154_monitors_rx(local: *mut ieee802154_local, skb: *mut sk_buff) {
    skb_reset_mac_header(skb); (*skb).ip_summed = CHECKSUM_UNNECESSARY; (*skb).pkt_type = PACKET_OTHERHOST; (*skb).protocol = htons(ETH_P_IEEE802154);
    list_for_each_entry_rcu!(sdata, &(*local).interfaces, list, {
        if (*sdata).wpan_dev.iftype != NL802154_IFTYPE_MONITOR || !ieee802154_sdata_running(sdata) { continue; }
        let skb2 = skb_clone(skb, GFP_ATOMIC); if !skb2.is_null() { (*skb2).dev = (*sdata).dev; ieee802154_deliver_skb(skb2); (*sdata).dev.stats.rx_packets += 1; (*sdata).dev.stats.rx_bytes += (*skb).len; }
    });
}

pub unsafe fn ieee802154_rx(local: *mut ieee802154_local, skb: *mut sk_buff) {
    let mut crc: u16; WARN_ON_ONCE!(softirq_count() == 0); if (*local).suspended { kfree_skb(skb); return; }
    if (*local).hw.flags & IEEE802154_HW_RX_OMIT_CKSUM != 0 { crc = crc_ccitt(0, (*skb).data, (*skb).len); put_unaligned_le16(crc, skb_put(skb, 2)); }
    rcu_read_lock(); ieee802154_monitors_rx(local, skb);
    if (*local).hw.phy.filtering == IEEE802154_FILTERING_NONE { crc = crc_ccitt(0, (*skb).data, (*skb).len); if crc != 0 { rcu_read_unlock(); kfree_skb(skb); return; } }
    skb_trim(skb, (*skb).len - 2); __ieee802154_rx_handle_packet(local, skb); rcu_read_unlock(); kfree_skb(skb);
}

pub unsafe fn ieee802154_rx_irqsafe(hw: *mut ieee802154_hw, skb: *mut sk_buff, lqi: u8) {
    let local = hw_to_local(hw); let cb = mac_cb_init(skb); (*cb).lqi = lqi; (*skb).pkt_type = IEEE802154_RX_MSG; skb_queue_tail(&mut (*local).skb_queue, skb); tasklet_schedule(&mut (*local).tasklet);
}

// EXPORT_SYMBOL(ieee802154_rx_irqsafe)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
