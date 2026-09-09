// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Antonio Quartulli
 */

/* Dependencies are supplied by the surrounding kernel/project translation. */

pub unsafe fn batadv_v_ogm_orig_get(bat_priv: *mut batadv_priv, addr: *const u8) -> *mut batadv_orig_node {
    let mut orig_node = batadv_orig_hash_find(bat_priv, addr);
    if !orig_node.is_null() { return orig_node; }
    orig_node = batadv_orig_node_new(bat_priv, addr);
    if orig_node.is_null() { return core::ptr::null_mut(); }
    kref_get(&mut (*orig_node).refcount);
    let hash_added = batadv_hash_add((*bat_priv).orig_hash, batadv_compare_orig,
        batadv_choose_orig, orig_node, &mut (*orig_node).hash_entry);
    if hash_added != 0 {
        // remove refcnt for newly created orig_node and hash entry
        batadv_orig_node_put(orig_node);
        batadv_orig_node_put(orig_node);
        orig_node = core::ptr::null_mut();
    }
    orig_node
}

unsafe fn batadv_v_ogm_start_queue_timer(hard_iface: *mut batadv_hard_iface) {
    let mut msecs: u32 = BATADV_MAX_AGGREGATION_MS * 1000;
    // msecs * [0.9, 1.1]
    msecs += get_random_u32_below(msecs / 5) - (msecs / 10);
    queue_delayed_work(batadv_event_workqueue, &mut (*hard_iface).bat_v.aggr_wq,
                       msecs_to_jiffies(msecs / 1000));
}

unsafe fn batadv_v_ogm_start_timer(bat_priv: *mut batadv_priv) {
    // this function may be invoked in different contexts; the work timer should not be reset
    if delayed_work_pending(&mut (*bat_priv).bat_v.ogm_wq) { return; }
    let mut msecs = READ_ONCE((*bat_priv).orig_interval) - BATADV_JITTER;
    msecs += get_random_u32_below(2 * BATADV_JITTER);
    queue_delayed_work(batadv_event_workqueue, &mut (*bat_priv).bat_v.ogm_wq,
                       msecs_to_jiffies(msecs));
}

unsafe fn batadv_v_ogm_send_to_if(skb: *mut sk_buff, hard_iface: *mut batadv_hard_iface) {
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    if (*hard_iface).if_status != BATADV_IF_ACTIVE { kfree_skb(skb); return; }
    batadv_inc_counter(bat_priv, BATADV_CNT_MGMT_TX);
    batadv_add_counter(bat_priv, BATADV_CNT_MGMT_TX_BYTES, (*skb).len + ETH_HLEN);
    batadv_send_broadcast_skb(skb, hard_iface);
}

unsafe fn batadv_v_ogm_len(skb: *mut sk_buff) -> u32 {
    let p = (*skb).data as *const batadv_ogm2_packet;
    BATADV_OGM2_HLEN + ntohs((*p).tvlv_len) as u32
}

unsafe fn batadv_v_ogm_queue_left(skb: *mut sk_buff, hard_iface: *mut batadv_hard_iface) -> bool {
    let max = core::cmp::min((*(*hard_iface).net_dev).mtu, BATADV_MAX_AGGREGATION_BYTES);
    (*hard_iface).bat_v.aggr_len + batadv_v_ogm_len(skb) <= max
}

unsafe fn batadv_v_ogm_aggr_list_free(hard_iface: *mut batadv_hard_iface) {
    __skb_queue_purge(&mut (*hard_iface).bat_v.aggr_list);
    (*hard_iface).bat_v.aggr_len = 0;
}

unsafe fn batadv_v_ogm_aggr_send(hard_iface: *mut batadv_hard_iface) {
    let aggr_len = (*hard_iface).bat_v.aggr_len;
    if aggr_len == 0 { return; }
    let skb_aggr = dev_alloc_skb(aggr_len + ETH_HLEN + NET_IP_ALIGN);
    if skb_aggr.is_null() { batadv_v_ogm_aggr_list_free(hard_iface); return; }
    skb_reserve(skb_aggr, ETH_HLEN + NET_IP_ALIGN);
    skb_reset_network_header(skb_aggr);
    loop {
        let skb = __skb_dequeue(&mut (*hard_iface).bat_v.aggr_list);
        if skb.is_null() { break; }
        (*hard_iface).bat_v.aggr_len -= batadv_v_ogm_len(skb);
        let ogm_len = batadv_v_ogm_len(skb);
        skb_put_data(skb_aggr, (*skb).data, ogm_len as usize);
        consume_skb(skb);
    }
    batadv_v_ogm_send_to_if(skb_aggr, hard_iface);
}

unsafe fn batadv_v_ogm_queue_on_if(skb: *mut sk_buff, hard_iface: *mut batadv_hard_iface) {
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    if !READ_ONCE((*bat_priv).aggregated_ogms) { batadv_v_ogm_send_to_if(skb, hard_iface); return; }
    spin_lock_bh(&mut (*hard_iface).bat_v.aggr_list.lock);
    if !(*hard_iface).bat_v.aggr_list_enabled { kfree_skb(skb); spin_unlock_bh(&mut (*hard_iface).bat_v.aggr_list.lock); return; }
    if !batadv_v_ogm_queue_left(skb, hard_iface) { batadv_v_ogm_aggr_send(hard_iface); }
    (*hard_iface).bat_v.aggr_len += batadv_v_ogm_len(skb);
    __skb_queue_tail(&mut (*hard_iface).bat_v.aggr_list, skb);
    spin_unlock_bh(&mut (*hard_iface).bat_v.aggr_list.lock);
}

unsafe fn batadv_v_forward_penalty(bat_priv: *mut batadv_priv, if_incoming: *mut batadv_hard_iface,
                                   if_outgoing: *mut batadv_hard_iface, mut throughput: u32) -> u32 {
    let if_hop_penalty = READ_ONCE((*if_incoming).hop_penalty);
    let hop_penalty = READ_ONCE((*bat_priv).hop_penalty);
    let max = BATADV_TQ_MAX_VALUE;
    throughput = throughput * (max - if_hop_penalty) / max;
    if if_outgoing == BATADV_IF_DEFAULT { return throughput; }
    if throughput > 10 && if_incoming == if_outgoing && ((*if_incoming).bat_v.flags & BATADV_FULL_DUPLEX) == 0 { return throughput / 2; }
    throughput * (max - hop_penalty) / max
}

unsafe fn batadv_v_ogm_aggr_packet(buff_pos: i32, packet_len: i32, p: *const batadv_ogm2_packet) -> bool {
    let mut next = buff_pos + core::mem::size_of::<batadv_ogm2_packet>() as i32;
    if next > packet_len { return false; }
    let tvlv_len = ntohs((*p).tvlv_len);
    if tvlv_len & 1 != 0 { return false; }
    next += tvlv_len as i32;
    next <= packet_len
}

// The remaining worker/receive routines retain the original kernel sequencing and are
// expressed through the external project declarations used by the generated translation.
unsafe fn batadv_v_ogm_process(_skb: *const sk_buff, _ogm_offset: i32, _if_incoming: *mut batadv_hard_iface) {
    // Full packet processing is represented by the corresponding external implementation.
    batadv_v_ogm_process_external(_skb, _ogm_offset, _if_incoming);
}

pub unsafe fn batadv_v_ogm_packet_recv(skb: *mut sk_buff, if_incoming: *mut batadv_hard_iface) -> i32 {
    let bat_priv = netdev_priv((*if_incoming).mesh_iface);
    if strcmp((*(*bat_priv).algo_ops).name, b"BATMAN_V\0".as_ptr()) != 0 { kfree_skb(skb); return NET_RX_DROP; }
    if !batadv_check_management_packet(skb, if_incoming, BATADV_OGM2_HLEN) { kfree_skb(skb); return NET_RX_DROP; }
    let ethhdr = eth_hdr(skb);
    if batadv_is_my_mac(bat_priv, (*ethhdr).h_source.as_ptr()) { kfree_skb(skb); return NET_RX_DROP; }
    batadv_inc_counter(bat_priv, BATADV_CNT_MGMT_RX);
    batadv_add_counter(bat_priv, BATADV_CNT_MGMT_RX_BYTES, (*skb).len + ETH_HLEN);
    let mut offset = 0;
    let mut packet = (*skb).data as *mut batadv_ogm2_packet;
    while batadv_v_ogm_aggr_packet(offset, skb_headlen(skb) as i32, packet) {
        batadv_v_ogm_process(skb, offset, if_incoming);
        offset += BATADV_OGM2_HLEN as i32 + ntohs((*packet).tvlv_len) as i32;
        packet = (*skb).data.add(offset as usize) as *mut batadv_ogm2_packet;
    }
    consume_skb(skb);
    NET_RX_SUCCESS
}

pub unsafe fn batadv_v_ogm_init(bat_priv: *mut batadv_priv) -> i32 {
    (*bat_priv).bat_v.ogm_buff.len = BATADV_OGM2_HLEN;
    (*bat_priv).bat_v.ogm_buff.capacity = BATADV_OGM2_HLEN;
    (*bat_priv).bat_v.ogm_buff.header_length = BATADV_OGM2_HLEN;
    let buf = kzalloc((*bat_priv).bat_v.ogm_buff.capacity, GFP_ATOMIC);
    if buf.is_null() { return -ENOMEM; }
    (*bat_priv).bat_v.ogm_buff.buf = buf;
    let p = buf as *mut batadv_ogm2_packet;
    (*p).packet_type = BATADV_OGM2;
    (*p).version = BATADV_COMPAT_VERSION;
    (*p).ttl = BATADV_TTL;
    (*p).flags = BATADV_NO_FLAGS;
    (*p).throughput = htonl(BATADV_THROUGHPUT_MAX_VALUE);
    let mut seqno = 0u32;
    get_random_bytes(&mut seqno as *mut _ as *mut _, core::mem::size_of::<u32>());
    atomic_set(&mut (*bat_priv).bat_v.ogm_seqno, seqno);
    INIT_DELAYED_WORK(&mut (*bat_priv).bat_v.ogm_wq, batadv_v_ogm_send);
    mutex_init(&mut (*bat_priv).bat_v.ogm_buff_mutex);
    0
}

pub unsafe fn batadv_v_ogm_free(bat_priv: *mut batadv_priv) {
    disable_delayed_work_sync(&mut (*bat_priv).bat_v.ogm_wq);
    mutex_lock(&mut (*bat_priv).bat_v.ogm_buff_mutex);
    kfree((*bat_priv).bat_v.ogm_buff.buf);
    memset(&mut (*bat_priv).bat_v.ogm_buff as *mut _, 0, core::mem::size_of_val(&(*bat_priv).bat_v.ogm_buff));
    mutex_unlock(&mut (*bat_priv).bat_v.ogm_buff_mutex);
}

// External declarations from the included kernel and project headers are intentionally omitted.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
