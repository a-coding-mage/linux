// SPDX-License-Identifier: GPL-2.0-only
/* IEEE 802.15.4 scanning management */

const IEEE802154_BEACON_MHR_SZ: usize = 13;
const IEEE802154_BEACON_PL_SZ: usize = 4;
const IEEE802154_MAC_CMD_MHR_SZ: usize = 23;
const IEEE802154_MAC_CMD_PL_SZ: usize = 1;
const IEEE802154_BEACON_SKB_SZ: usize = IEEE802154_BEACON_MHR_SZ + IEEE802154_BEACON_PL_SZ;
const IEEE802154_MAC_CMD_SKB_SZ: usize = IEEE802154_MAC_CMD_MHR_SZ + IEEE802154_MAC_CMD_PL_SZ;

unsafe fn mac802154_scan_cleanup_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, aborted: bool) -> i32 {
    let wpan_dev = &mut (*sdata).wpan_dev;
    let wpan_phy = (*local).phy;
    clear_bit(IEEE802154_IS_SCANNING, &mut (*local).ongoing);
    cancel_delayed_work(&mut (*local).scan_work);
    let request = rcu_replace_pointer(&mut (*local).scan_req, core::ptr::null_mut(), 1);
    if request.is_null() { return 0; }
    kvfree_rcu_mightsleep(request);
    let arg = if aborted { NL802154_SCAN_DONE_REASON_ABORTED } else { NL802154_SCAN_DONE_REASON_FINISHED };
    nl802154_scan_done(wpan_phy, wpan_dev, arg);
    ieee802154_mlme_op_post(local);
    drv_set_channel(local, (*wpan_phy).current_page, (*wpan_phy).current_channel);
    ieee802154_configure_durations(wpan_phy, (*wpan_phy).current_page, (*wpan_phy).current_channel);
    drv_stop(local);
    synchronize_net();
    (*sdata).required_filtering = (*sdata).iface_default_filtering;
    drv_start(local, (*sdata).required_filtering, &mut (*local).addr_filt);
    0
}

pub unsafe fn mac802154_abort_scan_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data) -> i32 {
    ASSERT_RTNL();
    if !mac802154_is_scanning(local) { return -ESRCH; }
    mac802154_scan_cleanup_locked(local, sdata, true)
}

unsafe fn mac802154_scan_get_channel_time(duration_order: u8, symbol_duration: u8) -> u32 {
    let base = (symbol_duration as u64) * IEEE802154_SUPERFRAME_PERIOD * IEEE802154_SLOT_PERIOD;
    usecs_to_jiffies(base * ((1u64 << duration_order) + 1))
}

unsafe fn mac802154_flush_queued_beacons(local: *mut ieee802154_local) {
    let mut pos = (*local).rx_beacon_list.next;
    while pos != &mut (*local).rx_beacon_list as *mut _ {
        let mac_pkt = container_of(pos, cfg802154_mac_pkt, node);
        pos = (*pos).next;
        list_del(&mut (*mac_pkt).node);
        kfree_skb((*mac_pkt).skb);
        kfree(mac_pkt as *mut _);
    }
}

unsafe fn mac802154_scan_get_next_channel(_local: *mut ieee802154_local, req: *mut cfg802154_scan_request, channel: *mut u8) {
    *channel = (*channel).wrapping_add(1);
    *channel = find_next_bit(&(*req).channels as *const _ as *const usize, IEEE802154_MAX_CHANNEL + 1, *channel as usize) as u8;
}

unsafe fn mac802154_scan_find_next_chan(local: *mut ieee802154_local, req: *mut cfg802154_scan_request, _page: u8, channel: *mut u8) -> i32 {
    mac802154_scan_get_next_channel(local, req, channel);
    if *channel > IEEE802154_MAX_CHANNEL { return -EINVAL; }
    0
}

unsafe fn mac802154_scan_prepare_beacon_req(local: *mut ieee802154_local) -> i32 {
    memset(&mut (*local).scan_beacon_req as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*local).scan_beacon_req));
    (*local).scan_beacon_req.mhr.fc.type_ = IEEE802154_FC_TYPE_MAC_CMD;
    (*local).scan_beacon_req.mhr.fc.dest_addr_mode = IEEE802154_SHORT_ADDRESSING;
    (*local).scan_beacon_req.mhr.fc.version = IEEE802154_2003_STD;
    (*local).scan_beacon_req.mhr.fc.source_addr_mode = IEEE802154_NO_ADDRESSING;
    (*local).scan_beacon_req.mhr.dest.mode = IEEE802154_ADDR_SHORT;
    (*local).scan_beacon_req.mhr.dest.pan_id = cpu_to_le16(IEEE802154_PANID_BROADCAST);
    (*local).scan_beacon_req.mhr.dest.short_addr = cpu_to_le16(IEEE802154_ADDR_BROADCAST);
    (*local).scan_beacon_req.mac_pl.cmd_id = IEEE802154_CMD_BEACON_REQ;
    0
}

unsafe fn mac802154_transmit_beacon_req(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data) -> i32 {
    let skb = alloc_skb(IEEE802154_MAC_CMD_SKB_SZ, GFP_KERNEL);
    if skb.is_null() { return -ENOBUFS; }
    (*skb).dev = (*sdata).dev;
    let ret = ieee802154_mac_cmd_push(skb, &(*local).scan_beacon_req, core::ptr::null_mut(), 0);
    if ret != 0 { kfree_skb(skb); return ret; }
    ieee802154_mlme_tx(local, sdata, skb)
}

pub unsafe fn mac802154_scan_worker(work: *mut work_struct) {
    let local = container_of(work, ieee802154_local, scan_work.work);
    drv_stop(local); synchronize_net(); mac802154_flush_queued_beacons(local);
    rcu_read_lock();
    let req = rcu_dereference((*local).scan_req);
    if req.is_null() { rcu_read_unlock(); return; }
    let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF((*req).wpan_dev);
    if (*local).suspended || !ieee802154_sdata_running(sdata) { rcu_read_unlock(); queue_delayed_work((*local).mac_wq, &mut (*local).scan_work, msecs_to_jiffies(1000)); return; }
    let dev = (*sdata).dev; netdev_hold(dev, core::ptr::null_mut(), GFP_ATOMIC);
    let phy = (*req).wpan_phy; let typ = (*req).type_; let duration = (*req).duration;
    let page = (*local).scan_page; let mut channel = (*local).scan_channel;
    loop { if mac802154_scan_find_next_chan(local, req, page, &mut channel) != 0 { rcu_read_unlock(); break; } if ieee802154_chan_is_valid(phy, page, channel) { rcu_read_unlock(); break; } }
    rtnl_lock(); let ret = drv_set_channel(local, page, channel); rtnl_unlock();
    if ret != 0 { goto_end_scan(local, sdata, dev); return; }
    (*local).scan_page = page; (*local).scan_channel = channel;
    rtnl_lock(); let ret = drv_start(local, IEEE802154_FILTERING_3_SCAN, &mut (*local).addr_filt); rtnl_unlock();
    if ret != 0 { goto_end_scan(local, sdata, dev); return; }
    if typ == NL802154_SCAN_ACTIVE { let _ = mac802154_transmit_beacon_req(local, sdata); }
    ieee802154_configure_durations(phy, page, channel);
    let scan_duration = mac802154_scan_get_channel_time(duration, (*phy).symbol_duration);
    queue_delayed_work((*local).mac_wq, &mut (*local).scan_work, scan_duration); netdev_put(dev, core::ptr::null_mut()); return;
}

unsafe fn goto_end_scan(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, dev: *mut net_device) { rtnl_lock(); mac802154_scan_cleanup_locked(local, sdata, false); rtnl_unlock(); netdev_put(dev, core::ptr::null_mut()); }

pub unsafe fn mac802154_trigger_scan_locked(sdata: *mut ieee802154_sub_if_data, request: *mut cfg802154_scan_request) -> i32 {
    let local = (*sdata).local; ASSERT_RTNL();
    if mac802154_is_scanning(local) { return -EBUSY; }
    if (*request).type_ != NL802154_SCAN_PASSIVE && (*request).type_ != NL802154_SCAN_ACTIVE { return -EOPNOTSUPP; }
    rcu_assign_pointer(&mut (*local).scan_req, request); ieee802154_mlme_op_pre(local);
    (*sdata).required_filtering = IEEE802154_FILTERING_3_SCAN; (*local).scan_page = (*request).page; (*local).scan_channel = -1i8 as u8;
    set_bit(IEEE802154_IS_SCANNING, &mut (*local).ongoing);
    if (*request).type_ == NL802154_SCAN_ACTIVE { mac802154_scan_prepare_beacon_req(local); }
    nl802154_scan_started((*request).wpan_phy, (*request).wpan_dev); queue_delayed_work((*local).mac_wq, &mut (*local).scan_work, 0); 0
}

// Remaining management routines retain the same C ABI and field-level behavior.
// Their declarations are intentionally kept external until repository dependencies are translated.
extern "C" {
    pub fn mac802154_process_beacon(local: *mut ieee802154_local, skb: *mut sk_buff, page: u8, channel: u8) -> i32;
    pub fn mac802154_transmit_beacon(local: *mut ieee802154_local, wpan_dev: *mut wpan_dev) -> i32;
    pub fn mac802154_beacon_worker(work: *mut work_struct);
    pub fn mac802154_stop_beacons_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data) -> i32;
    pub fn mac802154_send_beacons_locked(sdata: *mut ieee802154_sub_if_data, request: *mut cfg802154_beacon_request) -> i32;
    pub fn mac802154_perform_association(sdata: *mut ieee802154_sub_if_data, coord: *mut ieee802154_pan_device, short_addr: *mut __le16) -> i32;
    pub fn mac802154_process_association_resp(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn mac802154_send_disassociation_notif(sdata: *mut ieee802154_sub_if_data, target: *mut ieee802154_pan_device, reason: u8) -> i32;
    pub fn mac802154_process_association_req(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn mac802154_process_disassociation_notif(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
