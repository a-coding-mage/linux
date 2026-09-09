/*
 * net/tipc/name_distr.c: TIPC name distribution code
 *
 * Copyright (c) 2000-2006, 2014-2019, Ericsson AB
 * Copyright (c) 2005, 2010-2011, Wind River Systems
 * Copyright (c) 2020-2021, Red Hat Inc
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") as published by the Free
 * Software Foundation.
 */

// Includes supplied by the surrounding TIPC implementation are intentionally
// omitted; their symbols remain external dependencies.

#[no_mangle]
pub static mut sysctl_tipc_named_timeout: i32 = 2000;

unsafe fn publ_to_item(i: *mut distr_item, p: *mut publication) {
    (*i).type_ = htonl((*p).sr.type_);
    (*i).lower = htonl((*p).sr.lower);
    (*i).upper = htonl((*p).sr.upper);
    (*i).port = htonl((*p).sk.ref_);
    (*i).key = htonl((*p).key);
}

unsafe fn named_prepare_buf(net: *mut net, type_: u32, size: u32, dest: u32) -> *mut sk_buff {
    let buf = tipc_buf_acquire(INT_H_SIZE + size, GFP_ATOMIC);
    let self_ = tipc_own_addr(net);
    if !buf.is_null() {
        let msg = buf_msg(buf);
        tipc_msg_init(self_, msg, NAME_DISTRIBUTOR, type_, INT_H_SIZE, dest);
        msg_set_size(msg, INT_H_SIZE + size);
    }
    buf
}

pub unsafe fn tipc_named_publish(net: *mut net, p: *mut publication) -> *mut sk_buff {
    let nt = tipc_name_table(net);
    if (*p).scope == TIPC_NODE_SCOPE {
        list_add_tail_rcu(&mut (*p).binding_node, &mut (*nt).node_scope);
        return core::ptr::null_mut();
    }
    write_lock_bh(&mut (*nt).cluster_scope_lock);
    list_add_tail(&mut (*p).binding_node, &mut (*nt).cluster_scope);
    write_unlock_bh(&mut (*nt).cluster_scope_lock);
    let skb = named_prepare_buf(net, PUBLICATION, ITEM_SIZE, 0);
    if skb.is_null() { pr_warn("Publication distribution failure\n"); return core::ptr::null_mut(); }
    msg_set_named_seqno(buf_msg(skb), (*nt).snd_nxt); (*nt).snd_nxt = (*nt).snd_nxt.wrapping_add(1);
    msg_set_non_legacy(buf_msg(skb));
    publ_to_item(msg_data(buf_msg(skb)) as *mut distr_item, p);
    skb
}

pub unsafe fn tipc_named_withdraw(net: *mut net, p: *mut publication) -> *mut sk_buff {
    let nt = tipc_name_table(net);
    write_lock_bh(&mut (*nt).cluster_scope_lock);
    list_del(&mut (*p).binding_node);
    write_unlock_bh(&mut (*nt).cluster_scope_lock);
    if (*p).scope == TIPC_NODE_SCOPE { return core::ptr::null_mut(); }
    let skb = named_prepare_buf(net, WITHDRAWAL, ITEM_SIZE, 0);
    if skb.is_null() { pr_warn("Withdrawal distribution failure\n"); return core::ptr::null_mut(); }
    msg_set_named_seqno(buf_msg(skb), (*nt).snd_nxt); (*nt).snd_nxt = (*nt).snd_nxt.wrapping_add(1);
    msg_set_non_legacy(buf_msg(skb));
    publ_to_item(msg_data(buf_msg(skb)) as *mut distr_item, p);
    skb
}

unsafe fn named_distribute(net: *mut net, list: *mut sk_buff_head, dnode: u32, pls: *mut list_head, seqno: u16) {
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut item: *mut distr_item = core::ptr::null_mut();
    let msg_dsz = ((tipc_node_get_mtu(net, dnode, 0, false) - INT_H_SIZE) / ITEM_SIZE) * ITEM_SIZE;
    let mut msg_rem = msg_dsz;
    let mut publ: *mut publication = core::ptr::null_mut();
    list_for_each_entry!(publ, pls, binding_node, {
        if skb.is_null() {
            skb = named_prepare_buf(net, PUBLICATION, msg_rem, dnode);
            if skb.is_null() { pr_warn("Bulk publication failure\n"); return; }
            let hdr = buf_msg(skb); msg_set_bc_ack_invalid(hdr, true); msg_set_bulk(hdr); msg_set_non_legacy(hdr);
            item = msg_data(hdr) as *mut distr_item;
        }
        publ_to_item(item, publ); item = item.add(1); msg_rem -= ITEM_SIZE;
        if msg_rem == 0 { __skb_queue_tail(list, skb); skb = core::ptr::null_mut(); msg_rem = msg_dsz; }
    });
    if !skb.is_null() { let hdr = buf_msg(skb); msg_set_size(hdr, INT_H_SIZE + msg_dsz - msg_rem); skb_trim(skb, INT_H_SIZE + msg_dsz - msg_rem); __skb_queue_tail(list, skb); }
    let hdr = buf_msg(skb_peek_tail(list)); msg_set_last_bulk(hdr); msg_set_named_seqno(hdr, seqno);
}

pub unsafe fn tipc_named_node_up(net: *mut net, dnode: u32, capabilities: u16) {
    let nt = tipc_name_table(net); let tn = tipc_net(net); let mut head: sk_buff_head = core::mem::zeroed();
    __skb_queue_head_init(&mut head); spin_lock_bh(&mut (*tn).nametbl_lock);
    if capabilities & TIPC_NAMED_BCAST == 0 { (*nt).rc_dests += 1; }
    let seqno = (*nt).snd_nxt; spin_unlock_bh(&mut (*tn).nametbl_lock);
    read_lock_bh(&mut (*nt).cluster_scope_lock); named_distribute(net, &mut head, dnode, &mut (*nt).cluster_scope, seqno); tipc_node_xmit(net, &mut head, dnode, 0); read_unlock_bh(&mut (*nt).cluster_scope_lock);
}

unsafe fn tipc_publ_purge(net: *mut net, p: *mut publication, addr: u32) {
    let tn = tipc_net(net); let mut ua: tipc_uaddr = core::mem::zeroed();
    tipc_uaddr(&mut ua, TIPC_SERVICE_RANGE, (*p).scope, (*p).sr.type_, (*p).sr.lower, (*p).sr.upper);
    spin_lock_bh(&mut (*tn).nametbl_lock); let old = tipc_nametbl_remove_publ(net, &mut ua, &mut (*p).sk, (*p).key);
    if !old.is_null() { tipc_node_unsubscribe(net, &mut (*old).binding_node, addr); } spin_unlock_bh(&mut (*tn).nametbl_lock);
    if !old.is_null() { kfree_rcu(old, rcu); }
}

pub unsafe fn tipc_publ_notify(net: *mut net, nsub_list: *mut list_head, addr: u32, capabilities: u16) {
    let nt = tipc_name_table(net); let tn = tipc_net(net); let mut publ: *mut publication = core::ptr::null_mut(); let mut tmp: *mut publication = core::ptr::null_mut();
    list_for_each_entry_safe!(publ, tmp, nsub_list, binding_node, { tipc_publ_purge(net, publ, addr); });
    spin_lock_bh(&mut (*tn).nametbl_lock); if capabilities & TIPC_NAMED_BCAST == 0 { (*nt).rc_dests -= 1; } spin_unlock_bh(&mut (*tn).nametbl_lock);
}

unsafe fn tipc_update_nametbl(net: *mut net, i: *mut distr_item, node: u32, dtype: u32) -> bool {
    let lower = ntohl((*i).lower); let upper = ntohl((*i).upper); if lower > upper { return false; }
    let mut ua: tipc_uaddr = core::mem::zeroed(); tipc_uaddr(&mut ua, TIPC_SERVICE_RANGE, TIPC_CLUSTER_SCOPE, ntohl((*i).type_), lower, upper);
    let mut sk: tipc_socket_addr = core::mem::zeroed(); sk.ref_ = ntohl((*i).port); sk.node = node; let key = ntohl((*i).key);
    if dtype == PUBLICATION { let p = tipc_nametbl_insert_publ(net, &mut ua, &mut sk, key); if !p.is_null() { tipc_node_subscribe(net, &mut (*p).binding_node, node); return true; } }
    else if dtype == WITHDRAWAL { let p = tipc_nametbl_remove_publ(net, &mut ua, &mut sk, key); if !p.is_null() { tipc_node_unsubscribe(net, &mut (*p).binding_node, node); kfree_rcu(p, rcu); return true; } pr_warn_ratelimited("Failed to remove binding %u,%u from %u\n", ua.sr.type_, ua.sr.lower, node); }
    else { pr_warn_ratelimited("Unknown name table message received\n"); }
    false
}

unsafe fn tipc_named_dequeue(namedq: *mut sk_buff_head, rcv_nxt: *mut u16, open: *mut bool) -> *mut sk_buff {
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut tmp: *mut sk_buff = core::ptr::null_mut();
    let mut hdr: *mut tipc_msg;
    let mut seqno: u16;
    spin_lock_bh(&mut (*namedq).lock);
    skb_queue_walk_safe!(namedq, skb, tmp, {
        if skb_linearize(skb) != 0 { __skb_unlink(skb, namedq); kfree_skb(skb); continue; }
        hdr = buf_msg(skb); seqno = msg_named_seqno(hdr);
        if msg_is_last_bulk(hdr) { *rcv_nxt = seqno; *open = true; }
        if msg_is_bulk(hdr) || msg_is_legacy(hdr) { __skb_unlink(skb, namedq); spin_unlock_bh(&mut (*namedq).lock); return skb; }
        if *open && *rcv_nxt == seqno { *rcv_nxt = (*rcv_nxt).wrapping_add(1); __skb_unlink(skb, namedq); spin_unlock_bh(&mut (*namedq).lock); return skb; }
        if less(seqno, *rcv_nxt) { __skb_unlink(skb, namedq); kfree_skb(skb); continue; }
    });
    spin_unlock_bh(&mut (*namedq).lock); core::ptr::null_mut()
}

pub unsafe fn tipc_named_rcv(net: *mut net, namedq: *mut sk_buff_head, rcv_nxt: *mut u16, open: *mut bool) {
    let tn = tipc_net(net); spin_lock_bh(&mut (*tn).nametbl_lock);
    while let Some(skb) = tipc_named_dequeue(namedq, rcv_nxt, open) { let hdr = buf_msg(skb); let node = msg_orignode(hdr); let mut item = msg_data(hdr) as *mut distr_item; let mut count = msg_data_sz(hdr) / ITEM_SIZE; while count != 0 { tipc_update_nametbl(net, item, node, msg_type(hdr)); item = item.add(1); count -= 1; } kfree_skb(skb); }
    spin_unlock_bh(&mut (*tn).nametbl_lock);
}

pub unsafe fn tipc_named_reinit(net: *mut net) {
    let nt = tipc_name_table(net); let tn = tipc_net(net); let self_ = tipc_own_addr(net); spin_lock_bh(&mut (*tn).nametbl_lock);
    list_for_each_entry_rcu!(p, &mut (*nt).node_scope, binding_node, { (*p).sk.node = self_; });
    list_for_each_entry_rcu!(p, &mut (*nt).cluster_scope, binding_node, { (*p).sk.node = self_; }); (*nt).rc_dests = 0; spin_unlock_bh(&mut (*tn).nametbl_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
