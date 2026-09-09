// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// C dependencies supplied by the surrounding kernel/project translation.

unsafe fn batadv_tvlv_handler_release(ref_: *mut kref) {
    let tvlv_handler = container_of!(ref_, batadv_tvlv_handler, refcount);
    kfree_rcu!(tvlv_handler, rcu);
}

unsafe fn batadv_tvlv_handler_put(tvlv_handler: *mut batadv_tvlv_handler) {
    if tvlv_handler.is_null() { return; }
    kref_put!(&mut (*tvlv_handler).refcount, batadv_tvlv_handler_release);
}

unsafe fn batadv_tvlv_handler_get(
    bat_priv: *mut batadv_priv, type_: u8, version: u8,
) -> *mut batadv_tvlv_handler {
    let mut tvlv_handler: *mut batadv_tvlv_handler = core::ptr::null_mut();
    rcu_read_lock!();
    hlist_for_each_entry_rcu!(tvlv_handler_tmp, (*bat_priv).tvlv.handler_list, {
        if (*tvlv_handler_tmp).type_ != type_ { continue; }
        if (*tvlv_handler_tmp).version != version { continue; }
        if !kref_get_unless_zero!(&mut (*tvlv_handler_tmp).refcount) { continue; }
        tvlv_handler = tvlv_handler_tmp;
        break;
    });
    rcu_read_unlock!();
    tvlv_handler
}

unsafe fn batadv_tvlv_container_release(ref_: *mut kref) {
    let tvlv = container_of!(ref_, batadv_tvlv_container, refcount);
    kfree!(tvlv);
}

unsafe fn batadv_tvlv_container_put(tvlv: *mut batadv_tvlv_container) {
    if tvlv.is_null() { return; }
    kref_put!(&mut (*tvlv).refcount, batadv_tvlv_container_release);
}

unsafe fn batadv_tvlv_container_get(
    bat_priv: *mut batadv_priv, type_: u8, version: u8,
) -> *mut batadv_tvlv_container {
    lockdep_assert_held!(&(*bat_priv).tvlv.container_list_lock);
    let mut tvlv: *mut batadv_tvlv_container = core::ptr::null_mut();
    hlist_for_each_entry!(tvlv_tmp, (*bat_priv).tvlv.container_list, {
        if (*tvlv_tmp).tvlv_hdr.type_ != type_ { continue; }
        if (*tvlv_tmp).tvlv_hdr.version != version { continue; }
        kref_get!(&mut (*tvlv_tmp).refcount);
        tvlv = tvlv_tmp;
        break;
    });
    tvlv
}

unsafe fn batadv_tvlv_container_list_size(bat_priv: *mut batadv_priv) -> usize {
    lockdep_assert_held!(&(*bat_priv).tvlv.container_list_lock);
    let mut tvlv_len = 0usize;
    hlist_for_each_entry!(tvlv, (*bat_priv).tvlv.container_list, {
        tvlv_len += core::mem::size_of::<batadv_tvlv_hdr>();
        tvlv_len += ntohs!((*tvlv).tvlv_hdr.len) as usize;
    });
    tvlv_len
}

unsafe fn batadv_tvlv_container_remove(
    bat_priv: *mut batadv_priv, tvlv: *mut batadv_tvlv_container,
) {
    lockdep_assert_held!(&(*bat_priv).tvlv.container_list_lock);
    if tvlv.is_null() { return; }
    hlist_del!(&mut (*tvlv).list);
    batadv_tvlv_container_put(tvlv);
    batadv_tvlv_container_put(tvlv);
}

pub unsafe fn batadv_tvlv_container_unregister(
    bat_priv: *mut batadv_priv, type_: u8, version: u8,
) {
    spin_lock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
    let tvlv = batadv_tvlv_container_get(bat_priv, type_, version);
    batadv_tvlv_container_remove(bat_priv, tvlv);
    spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
}

pub unsafe fn batadv_tvlv_container_register(
    bat_priv: *mut batadv_priv, type_: u8, version: u8,
    tvlv_value: *mut core::ffi::c_void, mut tvlv_value_len: u16,
) {
    if tvlv_value.is_null() { tvlv_value_len = 0; }
    let tvlv_new = kzalloc!(core::mem::size_of::<batadv_tvlv_container>() + tvlv_value_len as usize, GFP_ATOMIC);
    if tvlv_new.is_null() { return; }
    (*tvlv_new).tvlv_hdr.version = version;
    (*tvlv_new).tvlv_hdr.type_ = type_;
    (*tvlv_new).tvlv_hdr.len = htons!(tvlv_value_len);
    memcpy!((tvlv_new as *mut u8).add(1), tvlv_value, ntohs!((*tvlv_new).tvlv_hdr.len));
    INIT_HLIST_NODE!(&mut (*tvlv_new).list);
    kref_init!(&mut (*tvlv_new).refcount);
    spin_lock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
    let tvlv_old = batadv_tvlv_container_get(bat_priv, type_, version);
    batadv_tvlv_container_remove(bat_priv, tvlv_old);
    kref_get!(&mut (*tvlv_new).refcount);
    hlist_add_head!(&mut (*tvlv_new).list, &mut (*bat_priv).tvlv.container_list);
    spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
    batadv_tvlv_container_put(tvlv_new);
}

unsafe fn batadv_tvlv_realloc_packet_buff(
    ogm_buff: *mut batadv_ogm_buf, additional_packet_len: usize,
) -> bool {
    let newlen = (*ogm_buff).header_length + additional_packet_len;
    let newcapacity = roundup_pow_of_two!(newlen);
    if newcapacity == (*ogm_buff).capacity { (*ogm_buff).len = newlen; return true; }
    let new_buff = kmalloc!(newcapacity, GFP_ATOMIC);
    if new_buff.is_null() {
        if newlen <= (*ogm_buff).capacity { (*ogm_buff).len = newlen; return true; }
        return false;
    }
    memcpy!(new_buff, (*ogm_buff).buf, (*ogm_buff).header_length);
    kfree!((*ogm_buff).buf);
    (*ogm_buff).buf = new_buff;
    (*ogm_buff).len = newlen;
    (*ogm_buff).capacity = newcapacity;
    true
}

pub unsafe fn batadv_tvlv_container_ogm_append(
    bat_priv: *mut batadv_priv, ogm_buff: *mut batadv_ogm_buf,
) -> i32 {
    spin_lock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
    let tvlv_value_len = batadv_tvlv_container_list_size(bat_priv);
    if tvlv_value_len > U16_MAX as usize { spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock); return -E2BIG; }
    if !batadv_tvlv_realloc_packet_buff(ogm_buff, tvlv_value_len) { spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock); return -ENOMEM; }
    if tvlv_value_len == 0 { spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock); return 0; }
    let mut tvlv_value = (*ogm_buff).buf.add((*ogm_buff).header_length);
    hlist_for_each_entry!(tvlv, (*bat_priv).tvlv.container_list, {
        let tvlv_hdr = tvlv_value as *mut batadv_tvlv_hdr;
        (*tvlv_hdr).type_ = (*tvlv).tvlv_hdr.type_;
        (*tvlv_hdr).version = (*tvlv).tvlv_hdr.version;
        (*tvlv_hdr).len = (*tvlv).tvlv_hdr.len;
        tvlv_value = (tvlv_hdr.add(1) as *mut u8).add(ntohs!((*tvlv).tvlv_hdr.len) as usize);
        memcpy!(tvlv_hdr.add(1), (tvlv as *mut u8).add(1), ntohs!((*tvlv).tvlv_hdr.len));
    });
    spin_unlock_bh!(&mut (*bat_priv).tvlv.container_list_lock);
    tvlv_value_len as i32
}

unsafe fn batadv_tvlv_call_handler(
    bat_priv: *mut batadv_priv, tvlv_handler: *mut batadv_tvlv_handler,
    packet_type: u8, orig_node: *mut batadv_orig_node, skb: *mut sk_buff,
    tvlv_value: *mut core::ffi::c_void, tvlv_value_len: u16,
) -> i32 {
    if tvlv_handler.is_null() { return NET_RX_SUCCESS; }
    match packet_type {
        BATADV_IV_OGM | BATADV_OGM2 => {
            if (*tvlv_handler).ogm_handler.is_none() || orig_node.is_null() { return NET_RX_SUCCESS; }
            ((*tvlv_handler).ogm_handler.unwrap())(bat_priv, orig_node, BATADV_NO_FLAGS, tvlv_value, tvlv_value_len);
        }
        BATADV_UNICAST_TVLV => {
            if skb.is_null() || (*tvlv_handler).unicast_handler.is_none() { return NET_RX_SUCCESS; }
            let packet = (*skb).data as *mut batadv_unicast_tvlv_packet;
            return ((*tvlv_handler).unicast_handler.unwrap())(bat_priv, (*packet).src.as_mut_ptr(), (*packet).dst.as_mut_ptr(), tvlv_value, tvlv_value_len);
        }
        BATADV_MCAST => {
            if skb.is_null() || (*tvlv_handler).mcast_handler.is_none() { return NET_RX_SUCCESS; }
            let offset = (tvlv_value as *mut u8).offset_from((*skb).data) as u32;
            if !skb_set_transport_header_careful!(skb, offset + tvlv_value_len as u32) { return -EINVAL; }
            skb_set_network_header!(skb, offset);
            return ((*tvlv_handler).mcast_handler.unwrap())(bat_priv, skb);
        }
        _ => {}
    }
    NET_RX_SUCCESS
}

unsafe fn batadv_tvlv_hdr_next(tvlv_value: &mut *mut core::ffi::c_void, tvlv_value_len: &mut u16) -> *mut batadv_tvlv_hdr {
    let mut hdr = *tvlv_value as *mut batadv_tvlv_hdr;
    let mut len = *tvlv_value_len;
    if len < core::mem::size_of::<batadv_tvlv_hdr>() as u16 { return core::ptr::null_mut(); }
    let content_len = ntohs!((*hdr).len);
    len -= core::mem::size_of::<batadv_tvlv_hdr>() as u16;
    if content_len > len || (content_len & 1) != 0 { return core::ptr::null_mut(); }
    *tvlv_value = (hdr.add(1) as *mut u8).add(content_len as usize) as *mut core::ffi::c_void;
    *tvlv_value_len = len - content_len;
    hdr
}

unsafe fn batadv_tvlv_containers_contain(mut value: *mut core::ffi::c_void, mut len: u16, type_: u8, version: u8) -> bool {
    loop {
        let hdr = batadv_tvlv_hdr_next(&mut value, &mut len);
        if hdr.is_null() { return false; }
        if (*hdr).type_ == type_ && (*hdr).version == version { return true; }
    }
}

pub unsafe fn batadv_tvlv_containers_process(
    bat_priv: *mut batadv_priv, packet_type: u8, orig_node: *mut batadv_orig_node,
    skb: *mut sk_buff, mut tvlv_value: *mut core::ffi::c_void, mut tvlv_value_len: u16,
) -> i32 {
    let start = tvlv_value;
    let start_len = tvlv_value_len;
    let mut ret = NET_RX_SUCCESS;
    loop {
        let hdr = batadv_tvlv_hdr_next(&mut tvlv_value, &mut tvlv_value_len);
        if hdr.is_null() { break; }
        let handler = batadv_tvlv_handler_get(bat_priv, (*hdr).type_, (*hdr).version);
        let res = batadv_tvlv_call_handler(bat_priv, handler, packet_type, orig_node, skb, hdr.add(1) as *mut _, ntohs!((*hdr).len));
        if ret == NET_RX_SUCCESS || res < 0 { ret = res; }
        batadv_tvlv_handler_put(handler);
    }
    if packet_type != BATADV_IV_OGM && packet_type != BATADV_OGM2 { return ret; }
    rcu_read_lock!();
    hlist_for_each_entry_rcu!(handler, (*bat_priv).tvlv.handler_list, {
        if (*handler).ogm_handler.is_none() || ((*handler).flags & BATADV_TVLV_HANDLER_OGM_CIFNOTFND) == 0 { continue; }
        if batadv_tvlv_containers_contain(start, start_len, (*handler).type_, (*handler).version) { continue; }
        ((*handler).ogm_handler.unwrap())(bat_priv, orig_node, BATADV_TVLV_HANDLER_OGM_CIFNOTFND, core::ptr::null_mut(), 0);
    });
    rcu_read_unlock!();
    NET_RX_SUCCESS
}

pub unsafe fn batadv_tvlv_ogm_receive(bat_priv: *mut batadv_priv, packet: *mut batadv_ogm_packet, orig_node: *mut batadv_orig_node) {
    if packet.is_null() { return; }
    let len = ntohs!((*packet).tvlv_len);
    if len == 0 { return; }
    batadv_tvlv_containers_process(bat_priv, BATADV_IV_OGM, orig_node, core::ptr::null_mut(), packet.add(1) as *mut _, len);
}

pub unsafe fn batadv_tvlv_handler_register(
    bat_priv: *mut batadv_priv, optr: Option<unsafe extern "C" fn(*mut batadv_priv,*mut batadv_orig_node,u8,*mut core::ffi::c_void,u16)>,
    uptr: Option<unsafe extern "C" fn(*mut batadv_priv,*mut u8,*mut u8,*mut core::ffi::c_void,u16)->i32>,
    mptr: Option<unsafe extern "C" fn(*mut batadv_priv,*mut sk_buff)->i32>, type_: u8, version: u8, flags: u8,
) {
    spin_lock_bh!(&mut (*bat_priv).tvlv.handler_list_lock);
    let old = batadv_tvlv_handler_get(bat_priv, type_, version);
    if !old.is_null() { spin_unlock_bh!(&mut (*bat_priv).tvlv.handler_list_lock); batadv_tvlv_handler_put(old); return; }
    let h = kzalloc_obj!(batadv_tvlv_handler, GFP_ATOMIC);
    if h.is_null() { spin_unlock_bh!(&mut (*bat_priv).tvlv.handler_list_lock); return; }
    (*h).ogm_handler = optr; (*h).unicast_handler = uptr; (*h).mcast_handler = mptr;
    (*h).type_ = type_; (*h).version = version; (*h).flags = flags;
    kref_init!(&mut (*h).refcount); INIT_HLIST_NODE!(&mut (*h).list);
    kref_get!(&mut (*h).refcount); hlist_add_head_rcu!(&mut (*h).list, &mut (*bat_priv).tvlv.handler_list);
    spin_unlock_bh!(&mut (*bat_priv).tvlv.handler_list_lock);
    batadv_tvlv_handler_put(h);
}

pub unsafe fn batadv_tvlv_handler_unregister(bat_priv: *mut batadv_priv, type_: u8, version: u8) {
    let h = batadv_tvlv_handler_get(bat_priv, type_, version);
    if h.is_null() { return; }
    batadv_tvlv_handler_put(h);
    spin_lock_bh!(&mut (*bat_priv).tvlv.handler_list_lock); hlist_del_rcu!(&mut (*h).list); spin_unlock_bh!(&mut (*bat_priv).tvlv.handler_list_lock);
    batadv_tvlv_handler_put(h);
}

pub unsafe fn batadv_tvlv_unicast_send(
    bat_priv: *mut batadv_priv, src: *const u8, dst: *const u8, type_: u8, version: u8,
    tvlv_value: *mut core::ffi::c_void, tvlv_value_len: u16,
) {
    let orig_node = batadv_orig_hash_find(bat_priv, dst);
    if orig_node.is_null() { return; }
    let tvlv_len = core::mem::size_of::<batadv_tvlv_hdr>() + tvlv_value_len as usize;
    let skb = netdev_alloc_skb_ip_align!(core::ptr::null_mut(), ETH_HLEN + core::mem::size_of::<batadv_unicast_tvlv_packet>() + tvlv_len);
    if !skb.is_null() {
        (*skb).priority = TC_PRIO_CONTROL; skb_reserve!(skb, ETH_HLEN);
        let packet = skb_put!(skb, core::mem::size_of::<batadv_unicast_tvlv_packet>() + tvlv_len) as *mut batadv_unicast_tvlv_packet;
        (*packet).packet_type = BATADV_UNICAST_TVLV; (*packet).version = BATADV_COMPAT_VERSION; (*packet).ttl = BATADV_TTL; (*packet).reserved = 0;
        (*packet).tvlv_len = htons!(tvlv_len as u16); (*packet).align = 0; ether_addr_copy!((*packet).src.as_mut_ptr(), src); ether_addr_copy!((*packet).dst.as_mut_ptr(), dst);
        let hdr = packet.add(1) as *mut batadv_tvlv_hdr; (*hdr).version = version; (*hdr).type_ = type_; (*hdr).len = htons!(tvlv_value_len);
        memcpy!(hdr.add(1), tvlv_value, tvlv_value_len as usize);
        batadv_send_skb_to_orig(skb, orig_node, core::ptr::null_mut());
    }
    batadv_orig_node_put(orig_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
