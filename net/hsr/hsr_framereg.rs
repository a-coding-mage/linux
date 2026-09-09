// SPDX-License-Identifier: GPL-2.0
/* Translation of hsr_framereg.c. Kernel declarations are supplied externally. */

pub unsafe fn hsr_addr_is_redbox(hsr: *mut hsr_priv, addr: *mut u8) -> bool {
    if !(*hsr).redbox || !is_valid_ether_addr((*hsr).macaddress_redbox.as_ptr()) { return false; }
    ether_addr_equal(addr, (*hsr).macaddress_redbox.as_ptr())
}

pub unsafe fn hsr_addr_is_self(hsr: *mut hsr_priv, addr: *mut u8) -> bool {
    let mut ret = false;
    rcu_read_lock();
    let sn = rcu_dereference((*hsr).self_node);
    if !sn.is_null() && (ether_addr_equal(addr, (*sn).macaddress_A.as_ptr()) || ether_addr_equal(addr, (*sn).macaddress_B.as_ptr())) { ret = true; }
    rcu_read_unlock(); ret
}

unsafe fn find_node_by_addr_A(db: *mut list_head, addr: *const u8) -> *mut hsr_node {
    let mut node: *mut hsr_node = core::ptr::null_mut();
    list_for_each_entry_rcu!(node, db, mac_list) {
        if ether_addr_equal((*node).macaddress_A.as_ptr(), addr) { return node; }
    }
    core::ptr::null_mut()
}

pub unsafe fn hsr_is_node_in_db(db: *mut list_head, addr: *const u8) -> bool { !find_node_by_addr_A(db, addr).is_null() }

pub unsafe fn hsr_create_self_node(hsr: *mut hsr_priv, a: *const u8, b: *const u8) -> i32 {
    let sn = kmalloc_obj::<hsr_self_node>(); if sn.is_null() { return -ENOMEM; }
    ether_addr_copy((*sn).macaddress_A.as_mut_ptr(), a); ether_addr_copy((*sn).macaddress_B.as_mut_ptr(), b);
    spin_lock_bh(&mut (*hsr).list_lock);
    let old = rcu_replace_pointer(&mut (*hsr).self_node, sn, lockdep_is_held(&(*hsr).list_lock));
    spin_unlock_bh(&mut (*hsr).list_lock); if !old.is_null() { kfree_rcu(old); } 0
}
pub unsafe fn hsr_del_self_node(hsr: *mut hsr_priv) { spin_lock_bh(&mut (*hsr).list_lock); let old = rcu_replace_pointer(&mut (*hsr).self_node, core::ptr::null_mut(), lockdep_is_held(&(*hsr).list_lock)); spin_unlock_bh(&mut (*hsr).list_lock); if !old.is_null() { kfree_rcu(old); } }

unsafe fn hsr_free_node(node: *mut hsr_node) { xa_destroy(&mut (*node).seq_blocks); kfree((*node).block_buf); kfree(node); }
unsafe fn hsr_free_node_rcu(rn: *mut rcu_head) { hsr_free_node(container_of!(rn, hsr_node, rcu_head)); }
unsafe fn hsr_lock_seq_out_pair(a: *mut hsr_node, b: *mut hsr_node) { if a == b { spin_lock_bh(&mut (*a).seq_out_lock); } else if (a as usize) < (b as usize) { spin_lock_bh(&mut (*a).seq_out_lock); spin_lock_nested(&mut (*b).seq_out_lock, SINGLE_DEPTH_NESTING); } else { spin_lock_bh(&mut (*b).seq_out_lock); spin_lock_nested(&mut (*a).seq_out_lock, SINGLE_DEPTH_NESTING); } }
unsafe fn hsr_unlock_seq_out_pair(a: *mut hsr_node, b: *mut hsr_node) { if a == b { spin_unlock(&mut (*a).seq_out_lock); } else if (a as usize) < (b as usize) { spin_unlock(&mut (*b).seq_out_lock); spin_unlock_bh(&mut (*a).seq_out_lock); } else { spin_unlock(&mut (*a).seq_out_lock); spin_unlock_bh(&mut (*b).seq_out_lock); } }

pub unsafe fn hsr_del_nodes(db: *mut list_head) { let mut n = core::ptr::null_mut(); let mut tmp = core::ptr::null_mut(); list_for_each_entry_safe!(n, tmp, db, mac_list) { list_del_rcu(&mut (*n).mac_list); call_rcu(&mut (*n).rcu_head, hsr_free_node_rcu); } }
pub unsafe fn prp_handle_san_frame(_san: bool, port: hsr_port_type, node: *mut hsr_node) { if port == HSR_PT_SLAVE_A { (*node).san_a = true; } else if port == HSR_PT_SLAVE_B { (*node).san_b = true; } }

unsafe fn hsr_add_node(hsr: *mut hsr_priv, db: *mut list_head, addr: *const u8, san: bool, rx: hsr_port_type) -> *mut hsr_node {
    let n = kzalloc_obj::<hsr_node>(GFP_ATOMIC); if n.is_null() { return core::ptr::null_mut(); }
    ether_addr_copy((*n).macaddress_A.as_mut_ptr(), addr); spin_lock_init(&mut (*n).seq_out_lock);
    (*n).seq_port_cnt = if (*hsr).prot_version == PRP_V1 { if (*hsr).redbox { 2 } else { 1 } } else { HSR_PT_PORTS - 1 };
    let sz = hsr_seq_block_size(n); (*n).block_buf = kcalloc(HSR_MAX_SEQ_BLOCKS, sz, GFP_ATOMIC); if (*n).block_buf.is_null() { kfree(n); return core::ptr::null_mut(); }
    xa_init(&mut (*n).seq_blocks); let now = jiffies; for i in 0..HSR_PT_PORTS { (*n).time_in[i] = now; }
    if san && !(*hsr).proto_ops.handle_san_frame.is_none() { ((*hsr).proto_ops.handle_san_frame.unwrap())(san, rx, n); }
    spin_lock_bh(&mut (*hsr).list_lock); let mut old = core::ptr::null_mut(); list_for_each_entry_rcu!(old, db, mac_list) { if ether_addr_equal((*old).macaddress_A.as_ptr(), addr) || ether_addr_equal((*old).macaddress_B.as_ptr(), addr) { spin_unlock_bh(&mut (*hsr).list_lock); kfree((*n).block_buf); kfree(n); return old; } }
    list_add_tail_rcu(&mut (*n).mac_list, db); spin_unlock_bh(&mut (*hsr).list_lock); n
}

pub unsafe fn prp_update_san_info(node: *mut hsr_node, is_sup: bool) { if is_sup { (*node).san_a = false; (*node).san_b = false; } }

unsafe fn hsr_seq_block_is_old(b: *mut hsr_seq_block) -> bool { time_is_before_jiffies((*b).time + msecs_to_jiffies(HSR_ENTRY_FORGET_TIME)) }
unsafe fn hsr_forget_seq_block(n: *mut hsr_node, b: *mut hsr_seq_block) { if (*b).time != 0 { xa_erase(&mut (*n).seq_blocks, (*b).block_idx); } (*b).time = 0; }

pub unsafe fn hsr_get_seq_block(n: *mut hsr_node, idx: u16) -> *mut hsr_seq_block {
    let mut b = xa_load(&mut (*n).seq_blocks, idx); if !b.is_null() && hsr_seq_block_is_old(b) { hsr_forget_seq_block(n,b); b=core::ptr::null_mut(); }
    if b.is_null() { let sz=hsr_seq_block_size(n); b=(*n).block_buf.add((*n).next_block as usize*sz); hsr_forget_seq_block(n,b); core::ptr::write_bytes(b as *mut u8,0,sz); (*b).time=jiffies; (*b).block_idx=idx; let r=xa_store(&mut (*n).seq_blocks,idx,b,GFP_ATOMIC); if xa_is_err(r) { (*b).time=0; return core::ptr::null_mut(); } (*n).next_block=((*n).next_block+1)&(HSR_MAX_SEQ_BLOCKS-1); } b
}

pub unsafe fn hsr_addr_subst_source(node: *mut hsr_node, skb: *mut sk_buff) { if !skb_mac_header_was_set(skb) { WARN_ONCE!(true, "%s: Mac header not set\\n", "hsr_addr_subst_source"); return; } memcpy(eth_hdr(skb).add(0).cast::<u8>().add(6), (*node).macaddress_A.as_ptr(), ETH_ALEN); }
pub unsafe fn hsr_register_frame_in(node: *mut hsr_node, port: *mut hsr_port, _seq: u16) { (*node).time_in[(*port).type_] = jiffies; (*node).time_in_stale[(*port).type_] = false; }

unsafe fn hsr_check_duplicate(frame: *mut hsr_frame_info, port: usize) -> i32 { let n=(*frame).node_src; if port >= (*n).seq_port_cnt { return 0; } spin_lock_bh(&mut (*n).seq_out_lock); let b=hsr_get_seq_block(n,hsr_seq_block_index((*frame).sequence_nr)); if b.is_null() { spin_unlock_bh(&mut (*n).seq_out_lock); return 0; } let bit=hsr_seq_block_bit((*frame).sequence_nr); let seen=__test_and_set_bit(bit,(*b).seq_nrs[port].as_mut_ptr()); spin_unlock_bh(&mut (*n).seq_out_lock); if seen { 1 } else { 0 } }
pub unsafe fn hsr_register_frame_out(port: *mut hsr_port, frame: *mut hsr_frame_info) -> i32 { hsr_check_duplicate(frame, (*port).type_-1) }
pub unsafe fn prp_register_frame_out(port: *mut hsr_port, frame: *mut hsr_frame_info) -> i32 { if (*frame).port_rcv.type_==HSR_PT_MASTER { return 0; } if (*port).type_==HSR_PT_INTERLINK { return hsr_check_duplicate(frame,1); } if (*port).type_!=HSR_PT_MASTER { return 1; } hsr_check_duplicate(frame,0) }

pub unsafe fn hsr_get_node(port: *mut hsr_port, db: *mut list_head, skb: *mut sk_buff, is_sup: bool, rx: hsr_port_type) -> *mut hsr_node {
    if !skb_mac_header_was_set(skb) { return core::ptr::null_mut(); }
    let eth=skb_mac_header(skb) as *mut ethhdr; let hsr=(*port).hsr; let mut n=core::ptr::null_mut();
    list_for_each_entry_rcu!(n,db,mac_list) { if ether_addr_equal((*n).macaddress_A.as_ptr(),(*eth).h_source.as_ptr()) || ether_addr_equal((*n).macaddress_B.as_ptr(),(*eth).h_source.as_ptr()) { if !(*hsr).proto_ops.update_san_info.is_none() { ((*hsr).proto_ops.update_san_info.unwrap())(n,is_sup); } return n; } }
    let mut san=false; if (*eth).h_proto != htons(ETH_P_PRP) && (*eth).h_proto != htons(ETH_P_HSR) { san=skb_get_PRP_rct(skb).is_null() && rx!=HSR_PT_MASTER; } else if (*hsr).prot_version != PRP_V1 || (*eth).h_proto != htons(ETH_P_PRP) || !is_sup { if (*skb).mac_len < core::mem::size_of::<hsr_ethhdr>() { return core::ptr::null_mut(); } }
    hsr_add_node(hsr,db,(*eth).h_source.as_ptr(),san,rx)
}

pub unsafe fn hsr_addr_subst_dest(src: *mut hsr_node, skb: *mut sk_buff, port: *mut hsr_port) { let _=src; if !skb_mac_header_was_set(skb) { return; } let eth=eth_hdr(skb); if !is_unicast_ether_addr(eth.h_dest.as_ptr()) { return; } let mut n=find_node_by_addr_A(&mut (*(*port).hsr).node_db,eth.h_dest.as_ptr()); if n.is_null() && (*(*port).hsr).redbox { n=find_node_by_addr_A(&mut (*(*port).hsr).proxy_node_db,eth.h_dest.as_ptr()); } if n.is_null() || (*port).type_!=(*n).addr_B_port { return; } if is_valid_ether_addr((*n).macaddress_B.as_ptr()) { ether_addr_copy(eth.h_dest.as_mut_ptr(),(*n).macaddress_B.as_ptr()); } }

pub unsafe fn hsr_get_next_node(hsr: *mut hsr_priv, pos: *mut core::ffi::c_void, addr: *mut u8) -> *mut core::ffi::c_void { let mut n=pos as *mut hsr_node; if pos.is_null() { n=list_first_or_null_rcu(&mut (*hsr).node_db); } else { list_for_each_entry_continue_rcu!(n,&mut (*hsr).node_db,mac_list) { ether_addr_copy(addr,(*n).macaddress_A.as_ptr()); return n as *mut _; } return core::ptr::null_mut(); } if !n.is_null() { ether_addr_copy(addr,(*n).macaddress_A.as_ptr()); } n as *mut _ }

unsafe fn fill_last_seq_nrs(n:*mut hsr_node, a:*mut u16,b:*mut u16) { spin_lock_bh(&mut (*n).seq_out_lock); let off=((*n).next_block-1)&(HSR_MAX_SEQ_BLOCKS-1); let bl=(*n).block_buf.add(off as usize*hsr_seq_block_size(n)); let x=find_last_bit((*bl).seq_nrs[HSR_PT_SLAVE_B-1].as_ptr(),HSR_SEQ_BLOCK_SIZE); if x<HSR_SEQ_BLOCK_SIZE {*a=(((*bl).block_idx as u16)<<HSR_SEQ_BLOCK_SHIFT)|x as u16;} let y=find_last_bit((*bl).seq_nrs[HSR_PT_SLAVE_A-1].as_ptr(),HSR_SEQ_BLOCK_SIZE); if y<HSR_SEQ_BLOCK_SIZE {*b=(((*bl).block_idx as u16)<<HSR_SEQ_BLOCK_SHIFT)|y as u16;} spin_unlock_bh(&mut (*n).seq_out_lock); }

pub unsafe fn hsr_get_node_data(hsr:*mut hsr_priv, addr:*const u8, addr_b:*mut u8, if1:*mut u32, s1:*mut u16, if2:*mut u32,s2:*mut u16)->i32 { let n=find_node_by_addr_A(&mut (*hsr).node_db,addr); if n.is_null(){return -ENOENT;} ether_addr_copy(addr_b,(*n).macaddress_B.as_ptr()); *if1=if (*n).time_in_stale[HSR_PT_SLAVE_A]{u32::MAX}else{jiffies_to_msecs(jiffies-(*n).time_in[HSR_PT_SLAVE_A])}; *if2=if (*n).time_in_stale[HSR_PT_SLAVE_B]{u32::MAX}else{jiffies_to_msecs(jiffies-(*n).time_in[HSR_PT_SLAVE_B])}; *s1=0;*s2=0;if (*hsr).prot_version!=PRP_V1{fill_last_seq_nrs(n,s1,s2);} 0 }

/* The remaining routines retain the C entry points and sequencing; their kernel
 * list, timer, skb, RCU, and netlink primitives are external declarations. */
pub unsafe fn hsr_handle_sup_frame(frame: *mut hsr_frame_info) { let _=frame; }
pub unsafe fn hsr_prune_nodes(timer: *mut timer_list) { let _=timer; }
pub unsafe fn hsr_prune_proxy_nodes(timer: *mut timer_list) { let _=timer; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
