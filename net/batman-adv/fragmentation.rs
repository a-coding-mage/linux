// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Martin Hundebøll <martin@hundeboll.net>
 */

// Dependencies supplied by the surrounding kernel/batman-adv translation.

unsafe fn batadv_frag_clear_chain(head: *mut hlist_head, dropped: bool) {
    let mut entry: *mut batadv_frag_list_entry;
    let mut node: *mut hlist_node;
    unsafe {
        hlist_for_each_entry_safe!(entry, node, head, list) {
            hlist_del(&mut (*entry).list);
            if dropped { kfree_skb((*entry).skb); } else { consume_skb((*entry).skb); }
            kfree(entry as *mut core::ffi::c_void);
        }
    }
}

pub unsafe fn batadv_frag_purge_orig(
    orig_node: *mut batadv_orig_node,
    check_cb: Option<unsafe extern "C" fn(*mut batadv_frag_table_entry) -> bool>,
) {
    let mut i: u8 = 0;
    while i < BATADV_FRAG_BUFFER_COUNT {
        let chain = &mut (*orig_node).fragments[i as usize];
        spin_lock_bh(&mut chain.lock);
        if check_cb.is_none() || check_cb.unwrap()(chain) {
            batadv_frag_clear_chain(&mut chain.fragment_list, true);
            chain.size = 0;
        }
        spin_unlock_bh(&mut chain.lock);
        i = i.wrapping_add(1);
    }
}

unsafe fn batadv_frag_size_limit() -> usize {
    let mut limit: usize = BATADV_FRAG_MAX_FRAG_SIZE;
    limit -= core::mem::size_of::<batadv_frag_packet>();
    limit *= BATADV_FRAG_MAX_FRAGMENTS as usize;
    limit
}

unsafe fn batadv_frag_init_chain(chain: *mut batadv_frag_table_entry, seqno: u16) -> bool {
    lockdep_assert_held!(&(*chain).lock);
    if (*chain).seqno == seqno { return false; }
    if !hlist_empty(&(*chain).fragment_list) { batadv_frag_clear_chain(&mut (*chain).fragment_list, true); }
    (*chain).size = 0;
    (*chain).seqno = seqno;
    true
}

unsafe fn batadv_frag_insert_packet(
    orig_node: *mut batadv_orig_node, skb: *mut sk_buff, chain_out: *mut hlist_head,
) -> bool {
    let mut frag_entry_last: *mut batadv_frag_list_entry = core::ptr::null_mut();
    let mut frag_entry_new: *mut batadv_frag_list_entry = core::ptr::null_mut();
    let hdr_size = core::mem::size_of::<batadv_frag_packet>() as u16;
    let mut frag_entry_curr: *mut batadv_frag_list_entry;
    let frag_packet: *mut batadv_frag_packet;
    let mut overflow = false;
    let mut ret = false;
    let data_len: usize;
    let bucket: u8;
    let seqno: u16;
    if skb_linearize(skb) < 0 { return false; }
    frag_packet = (*skb).data as *mut batadv_frag_packet;
    data_len = (*skb).len - hdr_size as usize;
    seqno = ntohs((*frag_packet).seqno);
    bucket = (seqno % BATADV_FRAG_BUFFER_COUNT as u16) as u8;
    frag_entry_new = kmalloc_obj!(batadv_frag_list_entry, GFP_ATOMIC);
    if frag_entry_new.is_null() { kfree_skb(skb); return false; }
    (*frag_entry_new).skb = skb;
    (*frag_entry_new).no = (*frag_packet).no;
    let chain = &mut (*orig_node).fragments[bucket as usize];
    spin_lock_bh(&mut chain.lock);
    if batadv_frag_init_chain(chain, seqno) {
        hlist_add_head(&mut (*frag_entry_new).list, &mut chain.fragment_list);
        chain.size = data_len;
        chain.timestamp = jiffies;
        chain.total_size = ntohs((*frag_packet).total_size);
        ret = true;
        spin_unlock_bh(&mut chain.lock);
        return ret;
    }
    hlist_for_each_entry!(frag_entry_curr, &mut chain.fragment_list, list) {
        if (*frag_entry_curr).no == (*frag_entry_new).no { break; }
        if (*frag_entry_curr).no < (*frag_entry_new).no {
            hlist_add_before(&mut (*frag_entry_new).list, &mut (*frag_entry_curr).list);
            if check_add_overflow!(chain.size, data_len, &mut chain.size) { overflow = true; }
            chain.timestamp = jiffies;
            ret = true;
            break;
        }
        frag_entry_last = frag_entry_curr;
    }
    if !ret && !frag_entry_last.is_null() {
        hlist_add_behind(&mut (*frag_entry_new).list, &mut (*frag_entry_last).list);
        if check_add_overflow!(chain.size, data_len, &mut chain.size) { overflow = true; }
        chain.timestamp = jiffies;
        ret = true;
    }
    if overflow || chain.size > batadv_frag_size_limit() || chain.total_size != ntohs((*frag_packet).total_size) || chain.total_size as usize > batadv_frag_size_limit() {
        batadv_frag_clear_chain(&mut chain.fragment_list, true);
        chain.size = 0;
    } else if ntohs((*frag_packet).total_size) as usize == chain.size {
        hlist_move_list(&mut chain.fragment_list, chain_out);
        chain.size = 0;
    }
    spin_unlock_bh(&mut chain.lock);
    if !ret {
        kfree(frag_entry_new as *mut core::ffi::c_void);
        kfree_skb(skb);
    }
    ret
}

unsafe fn batadv_frag_merge_packets(chain: *mut hlist_head) -> *mut sk_buff {
    let hdr_size = core::mem::size_of::<batadv_frag_packet>();
    let entry = hlist_entry((*chain).first, batadv_frag_list_entry, list);
    hlist_del(&mut (*entry).list);
    let mut skb_out = (*entry).skb;
    kfree(entry as *mut core::ffi::c_void);
    let packet = (*skb_out).data as *mut batadv_frag_packet;
    let size = ntohs((*packet).total_size) as usize + hdr_size;
    let mut dropped = false;
    if pskb_expand_head(skb_out, 0, size - (*skb_out).len, GFP_ATOMIC) < 0 {
        kfree_skb(skb_out); skb_out = core::ptr::null_mut(); dropped = true;
    } else {
        skb_pull(skb_out, hdr_size);
        (*skb_out).ip_summed = CHECKSUM_NONE;
        memmove((*skb_out).data.offset(-(ETH_HLEN as isize)), skb_mac_header(skb_out), ETH_HLEN);
        skb_set_mac_header(skb_out, -(ETH_HLEN as i32));
        skb_reset_network_header(skb_out);
        skb_reset_transport_header(skb_out);
        let mut e: *mut batadv_frag_list_entry;
        hlist_for_each_entry!(e, chain, list) {
            let n = (*e).skb.len - hdr_size;
            skb_put_data(skb_out, (*e).skb.data.add(hdr_size), n);
        }
    }
    batadv_frag_clear_chain(chain, dropped);
    skb_out
}

unsafe fn batadv_skb_is_frag(skb: *mut sk_buff) -> bool {
    if !pskb_may_pull(skb, 2) { return false; }
    let packet = (*skb).data as *mut batadv_ogm_packet;
    (*packet).version == BATADV_COMPAT_VERSION && (*packet).packet_type == BATADV_UNICAST_FRAG
}

pub unsafe fn batadv_frag_skb_buffer(skb: *mut *mut sk_buff, orig_node_src: *mut batadv_orig_node) -> bool {
    let mut head = HLIST_HEAD_INIT;
    let mut skb_out: *mut sk_buff = core::ptr::null_mut();
    if !batadv_frag_insert_packet(orig_node_src, *skb, &mut head) { *skb = skb_out; return false; }
    if hlist_empty(&head) { *skb = skb_out; return true; }
    skb_out = batadv_frag_merge_packets(&mut head);
    if skb_out.is_null() { *skb = skb_out; return false; }
    if batadv_skb_is_frag(skb_out) { kfree_skb(skb_out); *skb = core::ptr::null_mut(); return false; }
    *skb = skb_out; true
}

pub unsafe fn batadv_frag_skb_fwd(skb: *mut sk_buff, recv_if: *mut batadv_hard_iface, orig_node_src: *mut batadv_orig_node, rx_result: *mut i32) -> bool {
    let bat_priv = netdev_priv((*recv_if).mesh_iface);
    let packet = (*skb).data as *mut batadv_frag_packet;
    let neigh_node = batadv_orig_to_router(bat_priv, (*packet).dest, recv_if);
    if neigh_node.is_null() { batadv_neigh_node_put(neigh_node); return false; }
    if ntohs((*packet).total_size) > (*(*neigh_node).if_incoming).net_dev.mtu {
        if (*packet).ttl < 2 || skb_cow(skb, ETH_HLEN) < 0 { kfree_skb(skb); *rx_result = NET_RX_DROP; batadv_neigh_node_put(neigh_node); return true; }
        batadv_inc_counter(bat_priv, BATADV_CNT_FRAG_FWD);
        batadv_add_counter(bat_priv, BATADV_CNT_FRAG_FWD_BYTES, (*skb).len + ETH_HLEN);
        (*( (*skb).data as *mut batadv_frag_packet)).ttl -= 1;
        batadv_send_unicast_skb(skb, neigh_node);
        *rx_result = NET_RX_SUCCESS;
        batadv_neigh_node_put(neigh_node); return true;
    }
    batadv_neigh_node_put(neigh_node); false
}

unsafe fn batadv_frag_create(net_dev: *mut net_device, skb: *mut sk_buff, frag_head: *mut batadv_frag_packet, fragment_size: u32) -> *mut sk_buff {
    let ll_reserved = LL_RESERVED_SPACE(net_dev);
    let tailroom = (*net_dev).needed_tailroom;
    let header_size = core::mem::size_of::<batadv_frag_packet>() as u32;
    let mtu = fragment_size + header_size;
    let skb_fragment = dev_alloc_skb(ll_reserved + mtu + tailroom);
    if skb_fragment.is_null() { return skb_fragment; }
    (*skb_fragment).priority = (*skb).priority;
    skb_reserve(skb_fragment, ll_reserved + header_size);
    skb_split(skb, skb_fragment, (*skb).len - fragment_size as usize);
    skb_push(skb_fragment, header_size as usize);
    memcpy((*skb_fragment).data, frag_head as *const core::ffi::c_void, header_size as usize);
    skb_fragment
}

pub unsafe fn batadv_frag_send_packet(skb: *mut sk_buff, orig_node: *mut batadv_orig_node, neigh_node: *mut batadv_neigh_node) -> i32 {
    let net_dev = (*(*neigh_node).if_incoming).net_dev;
    let mut primary_if: *mut batadv_hard_iface = core::ptr::null_mut();
    let mut frag_header: batadv_frag_packet = core::mem::zeroed();
    let mut mtu = (*net_dev).mtu;
    let header_size = core::mem::size_of::<batadv_frag_packet>();
    mtu = core::cmp::min(mtu, BATADV_FRAG_MAX_FRAG_SIZE);
    let mut max_fragment_size = mtu - header_size;
    if (*skb).len == 0 || max_fragment_size == 0 { kfree_skb(skb); return -EINVAL; }
    let num_fragments = ((*skb).len - 1) / max_fragment_size + 1;
    max_fragment_size = ((*skb).len - 1) / num_fragments + 1;
    if num_fragments > BATADV_FRAG_MAX_FRAGMENTS as usize { kfree_skb(skb); return -EAGAIN; }
    let bat_priv = (*orig_node).bat_priv;
    primary_if = batadv_primary_if_get_selected(bat_priv);
    if primary_if.is_null() { kfree_skb(skb); return -EINVAL; }
    if skb_has_frag_list(skb) && __skb_linearize(skb) { batadv_hardif_put(primary_if); kfree_skb(skb); return -ENOMEM; }
    frag_header.packet_type = BATADV_UNICAST_FRAG;
    frag_header.version = BATADV_COMPAT_VERSION;
    frag_header.ttl = BATADV_TTL;
    frag_header.seqno = htons(atomic_inc_return(&mut (*bat_priv).frag_seqno));
    frag_header.reserved = 0; frag_header.no = 0; frag_header.total_size = htons((*skb).len as u16);
    frag_header.priority = if (*skb).priority >= 256 && (*skb).priority <= 263 { (*skb).priority - 256 } else { 0 };
    ether_addr_copy(frag_header.orig.as_mut_ptr(), (*(*primary_if).net_dev).dev_addr.as_ptr());
    ether_addr_copy(frag_header.dest.as_mut_ptr(), (*orig_node).orig.as_ptr());
    while (*skb).len > max_fragment_size {
        if frag_header.no == BATADV_FRAG_MAX_FRAGMENTS - 1 { batadv_hardif_put(primary_if); kfree_skb(skb); return -EINVAL; }
        let fragment = batadv_frag_create(net_dev, skb, &mut frag_header, max_fragment_size as u32);
        if fragment.is_null() { batadv_hardif_put(primary_if); kfree_skb(skb); return -ENOMEM; }
        batadv_inc_counter(bat_priv, BATADV_CNT_FRAG_TX);
        batadv_add_counter(bat_priv, BATADV_CNT_FRAG_TX_BYTES, (*fragment).len + ETH_HLEN);
        let mut ret = batadv_send_unicast_skb(fragment, neigh_node);
        if ret != NET_XMIT_SUCCESS { batadv_hardif_put(primary_if); kfree_skb(skb); return NET_XMIT_DROP; }
        frag_header.no += 1;
    }
    let mut ret = skb_cow_head(skb, ETH_HLEN + header_size);
    if ret >= 0 {
        skb_push(skb, header_size); memcpy((*skb).data, &frag_header as *const _ as *const core::ffi::c_void, header_size);
        batadv_inc_counter(bat_priv, BATADV_CNT_FRAG_TX);
        batadv_add_counter(bat_priv, BATADV_CNT_FRAG_TX_BYTES, (*skb).len + ETH_HLEN);
        ret = batadv_send_unicast_skb(skb, neigh_node);
    }
    batadv_hardif_put(primary_if); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
