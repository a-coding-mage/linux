/*
 * net/tipc/discover.c
 *
 * Copyright (c) 2003-2006, 2014-2018, Ericsson AB
 * Copyright (c) 2005-2006, 2010-2011, Wind River Systems
 * All rights reserved.
 */

/* Dependencies are supplied by the surrounding TIPC translation. */

const TIPC_DISC_INIT: ::core::ffi::c_ulong = msecs_to_jiffies(125);
const TIPC_DISC_FAST: ::core::ffi::c_ulong = msecs_to_jiffies(1000);
const TIPC_DISC_SLOW: ::core::ffi::c_ulong = msecs_to_jiffies(60000);
const TIPC_DISC_INACTIVE: ::core::ffi::c_ulong = 0xffff_ffff;

#[repr(C)]
pub struct tipc_discoverer {
    pub bearer_id: u32,
    pub dest: tipc_media_addr,
    pub net: *mut net,
    pub domain: u32,
    pub num_nodes: ::core::ffi::c_int,
    pub lock: spinlock_t,
    pub skb: *mut sk_buff,
    pub timer: timer_list,
    pub timer_intv: ::core::ffi::c_ulong,
    pub rcu: rcu_head,
}

unsafe fn tipc_disc_init_msg(
    net: *mut net,
    skb: *mut sk_buff,
    mtyp: u32,
    b: *mut tipc_bearer,
) {
    let tn = tipc_net(net);
    let dest_domain = (*b).domain;
    let hdr = buf_msg(skb);

    tipc_msg_init((*tn).trial_addr, hdr, LINK_CONFIG, mtyp, MAX_H_SIZE, dest_domain);
    msg_set_size(hdr, MAX_H_SIZE + NODE_ID_LEN);
    msg_set_non_seq(hdr, 1);
    msg_set_node_sig(hdr, (*tn).random);
    msg_set_node_capabilities(hdr, TIPC_NODE_CAPABILITIES);
    msg_set_dest_domain(hdr, dest_domain);
    msg_set_bc_netid(hdr, (*tn).net_id);
    ((*(*b).media).addr2msg)(msg_media_addr(hdr), &(*b).addr);
    msg_set_peer_net_hash(hdr, tipc_net_hash_mixes(net, (*tn).random));
    msg_set_node_id(hdr, tipc_own_id(net));
}

unsafe fn tipc_disc_msg_xmit(
    net: *mut net,
    mtyp: u32,
    dst: u32,
    src: u32,
    sugg_addr: u32,
    maddr: *mut tipc_media_addr,
    b: *mut tipc_bearer,
) {
    let skb = tipc_buf_acquire(MAX_H_SIZE + NODE_ID_LEN, GFP_ATOMIC);
    if skb.is_null() {
        return;
    }
    let hdr = buf_msg(skb);
    tipc_disc_init_msg(net, skb, mtyp, b);
    msg_set_sugg_node_addr(hdr, sugg_addr);
    msg_set_dest_domain(hdr, dst);
    tipc_bearer_xmit_skb(net, (*b).identity, skb, maddr);
}

unsafe fn disc_dupl_alert(
    b: *mut tipc_bearer,
    node_addr: u32,
    media_addr: *mut tipc_media_addr,
) {
    let mut media_addr_str = [0i8; 64];
    tipc_media_addr_printf(
        media_addr_str.as_mut_ptr(),
        media_addr_str.len(),
        media_addr,
    );
    pr_warn(
        "Duplicate %x using %s seen on <%s>\n",
        node_addr,
        media_addr_str.as_ptr(),
        (*b).name,
    );
}

unsafe fn tipc_disc_addr_trial_msg(
    d: *mut tipc_discoverer,
    maddr: *mut tipc_media_addr,
    b: *mut tipc_bearer,
    dst: u32,
    src: u32,
    mut sugg_addr: u32,
    peer_id: *mut u8,
    mtyp: ::core::ffi::c_int,
) -> bool {
    let net = (*d).net;
    let tn = tipc_net(net);
    let self_addr = tipc_own_addr(net);
    let trial = time_before(jiffies, (*tn).addr_trial_end) && self_addr == 0;

    if mtyp == DSC_TRIAL_FAIL_MSG {
        if !trial { return true; }
        if dst != (*tn).trial_addr { return true; }
        (*tn).trial_addr = sugg_addr;
        msg_set_prevnode(buf_msg((*d).skb), sugg_addr);
        (*tn).addr_trial_end = jiffies + msecs_to_jiffies(1000);
        return true;
    }

    if !trial && self_addr == 0 {
        schedule_work(&mut (*tn).work);
        msg_set_prevnode(buf_msg((*d).skb), (*tn).trial_addr);
        msg_set_type(buf_msg((*d).skb), DSC_REQ_MSG);
    }
    if mtyp != DSC_TRIAL_MSG { return trial; }

    sugg_addr = tipc_node_try_addr(net, peer_id, src);
    if sugg_addr != 0 {
        tipc_disc_msg_xmit(net, DSC_TRIAL_FAIL_MSG, src, self_addr, sugg_addr, maddr, b);
    }
    true
}

pub unsafe fn tipc_disc_rcv(net: *mut net, skb: *mut sk_buff, b: *mut tipc_bearer) {
    let tn = tipc_net(net);
    let mut hdr = buf_msg(skb);
    let pnet_hash = msg_peer_net_hash(hdr);
    let caps = msg_node_capabilities(hdr);
    let legacy = (*tn).legacy_addr_format;
    let sugg = msg_sugg_node_addr(hdr);
    let signature = msg_node_sig(hdr);
    let mut peer_id = [0u8; NODE_ID_LEN as usize];
    let dst = msg_dest_domain(hdr);
    let net_id = msg_bc_netid(hdr);
    let mut maddr: tipc_media_addr = ::core::mem::zeroed();
    let src = msg_prevnode(hdr);
    let mtyp = msg_type(hdr);
    let mut dupl_addr = false;
    let mut respond = false;

    if skb_linearize(skb) != 0 { kfree_skb(skb); return; }
    hdr = buf_msg(skb);
    if caps & TIPC_NODE_ID128 != 0 {
        memcpy(peer_id.as_mut_ptr(), msg_node_id(hdr), NODE_ID_LEN);
    } else {
        sprintf(peer_id.as_mut_ptr() as *mut i8, "%x", src);
    }
    let err = ((*(*b).media).msg2addr)(b, &mut maddr, msg_media_addr(hdr));
    kfree_skb(skb);
    if err != 0 || maddr.broadcast { pr_warn_ratelimited("Rcv corrupt discovery message\n"); return; }
    if !memcmp(&maddr, &(*b).addr, ::core::mem::size_of::<tipc_media_addr>()) { return; }
    if net_id != (*tn).net_id { return; }
    if tipc_disc_addr_trial_msg((*b).disc, &mut maddr, b, dst, src, sugg, peer_id.as_mut_ptr(), mtyp as i32) { return; }
    let self_addr = tipc_own_addr(net);
    if in_own_node(net, src) { disc_dupl_alert(b, self_addr, &mut maddr); return; }
    if !tipc_in_scope(legacy, dst, self_addr) || !tipc_in_scope(legacy, (*b).domain, src) { return; }
    tipc_node_check_dest(net, src, peer_id.as_mut_ptr(), b, caps, signature, pnet_hash, &mut maddr, &mut respond, &mut dupl_addr);
    if dupl_addr { disc_dupl_alert(b, src, &mut maddr); }
    if !respond || mtyp != DSC_REQ_MSG { return; }
    tipc_disc_msg_xmit(net, DSC_RESP_MSG, src, self_addr, 0, &mut maddr, b);
}

pub unsafe fn tipc_disc_add_dest(d: *mut tipc_discoverer) { spin_lock_bh(&mut (*d).lock); (*d).num_nodes += 1; spin_unlock_bh(&mut (*d).lock); }

pub unsafe fn tipc_disc_remove_dest(d: *mut tipc_discoverer) {
    spin_lock_bh(&mut (*d).lock);
    (*d).num_nodes -= 1;
    let num = (*d).num_nodes;
    let intv = (*d).timer_intv;
    if num == 0 && (intv == TIPC_DISC_INACTIVE || intv > TIPC_DISC_FAST) {
        (*d).timer_intv = TIPC_DISC_INIT;
        mod_timer(&mut (*d).timer, jiffies + (*d).timer_intv);
    }
    spin_unlock_bh(&mut (*d).lock);
}

unsafe fn tipc_disc_timeout(t: *mut timer_list) {
    let d = timer_container_of(t);
    let tn = tipc_net((*d).net);
    let mut maddr: tipc_media_addr = ::core::mem::zeroed();
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let net = (*d).net;
    let mut bearer_id: u32;
    spin_lock_bh(&mut (*d).lock);
    if tipc_node((*d).domain) != core::ptr::null_mut() && (*d).num_nodes != 0 {
        (*d).timer_intv = TIPC_DISC_INACTIVE;
        spin_unlock_bh(&mut (*d).lock);
        return;
    }
    if !time_before(jiffies, (*tn).addr_trial_end) && tipc_own_addr(net) == 0 {
        mod_timer(&mut (*d).timer, jiffies + TIPC_DISC_INIT);
        spin_unlock_bh(&mut (*d).lock);
        schedule_work(&mut (*tn).work);
        return;
    }
    if time_before(jiffies, (*tn).addr_trial_end) { (*d).timer_intv = TIPC_DISC_INIT; }
    else {
        (*d).timer_intv = (*d).timer_intv.wrapping_mul(2);
        if (*d).num_nodes != 0 && (*d).timer_intv > TIPC_DISC_SLOW { (*d).timer_intv = TIPC_DISC_SLOW; }
        else if (*d).num_nodes == 0 && (*d).timer_intv > TIPC_DISC_FAST { (*d).timer_intv = TIPC_DISC_FAST; }
        msg_set_type(buf_msg((*d).skb), DSC_REQ_MSG);
        msg_set_prevnode(buf_msg((*d).skb), (*tn).trial_addr);
    }
    mod_timer(&mut (*d).timer, jiffies + (*d).timer_intv);
    memcpy(&mut maddr, &(*d).dest, ::core::mem::size_of::<tipc_media_addr>());
    skb = skb_clone((*d).skb, GFP_ATOMIC);
    bearer_id = (*d).bearer_id;
    spin_unlock_bh(&mut (*d).lock);
    if !skb.is_null() { tipc_bearer_xmit_skb(net, bearer_id, skb, &mut maddr); }
}

pub unsafe fn tipc_disc_create(
    net: *mut net,
    b: *mut tipc_bearer,
    dest: *mut tipc_media_addr,
    skb: *mut *mut sk_buff,
) -> ::core::ffi::c_int {
    let tn = tipc_net(net);
    let d = kmalloc_obj::<tipc_discoverer>(GFP_ATOMIC);
    if d.is_null() { return -ENOMEM; }
    (*d).skb = tipc_buf_acquire(MAX_H_SIZE + NODE_ID_LEN, GFP_ATOMIC);
    if (*d).skb.is_null() { kfree(d as *mut ::core::ffi::c_void); return -ENOMEM; }
    tipc_disc_init_msg(net, (*d).skb, DSC_REQ_MSG, b);
    if tipc_own_addr(net) == 0 {
        (*tn).addr_trial_end = jiffies + msecs_to_jiffies(1000);
        msg_set_type(buf_msg((*d).skb), DSC_TRIAL_MSG);
    }
    memcpy(&mut (*d).dest, dest, ::core::mem::size_of::<tipc_media_addr>());
    (*d).net = net;
    (*d).bearer_id = (*b).identity;
    (*d).domain = (*b).domain;
    (*d).num_nodes = 0;
    (*d).timer_intv = TIPC_DISC_INIT;
    spin_lock_init(&mut (*d).lock);
    timer_setup(&mut (*d).timer, tipc_disc_timeout, 0);
    mod_timer(&mut (*d).timer, jiffies + (*d).timer_intv);
    (*b).disc = d;
    *skb = skb_clone((*d).skb, GFP_ATOMIC);
    0
}

unsafe fn tipc_disc_free_rcu(rp: *mut rcu_head) {
    let d = container_of::<tipc_discoverer>(rp, "rcu");
    kfree_skb((*d).skb);
    kfree(d as *mut ::core::ffi::c_void);
}

pub unsafe fn tipc_disc_delete(d: *mut tipc_discoverer) {
    timer_shutdown_sync(&mut (*d).timer);
    call_rcu(&mut (*d).rcu, tipc_disc_free_rcu);
}

pub unsafe fn tipc_disc_reset(net: *mut net, b: *mut tipc_bearer) {
    let d = (*b).disc;
    let mut maddr: tipc_media_addr = ::core::mem::zeroed();
    spin_lock_bh(&mut (*d).lock);
    tipc_disc_init_msg(net, (*d).skb, DSC_REQ_MSG, b);
    (*d).net = net;
    (*d).bearer_id = (*b).identity;
    (*d).domain = (*b).domain;
    (*d).num_nodes = 0;
    (*d).timer_intv = TIPC_DISC_INIT;
    memcpy(&mut maddr, &(*d).dest, ::core::mem::size_of::<tipc_media_addr>());
    mod_timer(&mut (*d).timer, jiffies + (*d).timer_intv);
    let skb = skb_clone((*d).skb, GFP_ATOMIC);
    spin_unlock_bh(&mut (*d).lock);
    if !skb.is_null() { tipc_bearer_xmit_skb(net, (*b).identity, skb, &mut maddr); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
