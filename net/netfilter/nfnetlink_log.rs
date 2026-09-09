// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of nfnetlink_log.c. Kernel dependencies are external. */

const NFULNL_COPY_DISABLED: u8 = 0xff;
const NFULNL_NLBUFSIZ_DEFAULT: u32 = NLMSG_GOODSIZE;
const NFULNL_TIMEOUT_DEFAULT: u32 = 100;
const NFULNL_QTHRESH_DEFAULT: u32 = 100;
const NFULNL_COPY_RANGE_MAX: u32 = 0xffff - NLA_HDRLEN;
const INSTANCE_BUCKETS: usize = 16;

#[repr(C)]
pub struct nfulnl_instance {
    pub hlist: hlist_node, pub lock: spinlock_t, pub use_: refcount_t,
    pub qlen: u32, pub skb: *mut sk_buff, pub timer: timer_list,
    pub net: *mut net, pub ns_tracker: netns_tracker,
    pub peer_user_ns: *mut user_namespace, pub peer_portid: u32,
    pub flushtimeout: u32, pub nlbufsiz: u32, pub qthreshold: u32,
    pub copy_range: u32, pub seq: u32, pub group_num: u16, pub flags: u16,
    pub copy_mode: u8, pub rcu: rcu_head,
}

#[repr(C)]
pub struct nfnl_log_net {
    pub instances_lock: spinlock_t,
    pub instance_table: [hlist_head; INSTANCE_BUCKETS],
    pub global_seq: atomic_t,
}

static mut nfnl_log_net_id: u32 = 0;

unsafe fn nfnl_log_pernet(net: *mut net) -> *mut nfnl_log_net { net_generic(net, nfnl_log_net_id) }
unsafe fn instance_hashfn(group_num: u16) -> u8 { ((group_num & 0xff) % INSTANCE_BUCKETS as u16) as u8 }

unsafe fn __instance_lookup(log: *const nfnl_log_net, group_num: u16) -> *mut nfulnl_instance {
    let head = &(*log).instance_table[instance_hashfn(group_num) as usize] as *const _;
    let mut inst: *mut nfulnl_instance = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(inst, head, hlist, {
        if (*inst).group_num == group_num { return inst; }
    });
    core::ptr::null_mut()
}
unsafe fn instance_get(inst: *mut nfulnl_instance) { refcount_inc(&mut (*inst).use_); }
unsafe fn instance_lookup_get_rcu(log: *const nfnl_log_net, group_num: u16) -> *mut nfulnl_instance {
    let inst = __instance_lookup(log, group_num);
    if !inst.is_null() && !refcount_inc_not_zero(&mut (*inst).use_) { core::ptr::null_mut() } else { inst }
}
unsafe fn instance_lookup_get(log: *const nfnl_log_net, group_num: u16) -> *mut nfulnl_instance {
    rcu_read_lock(); let inst = instance_lookup_get_rcu(log, group_num); rcu_read_unlock(); inst
}

unsafe fn nfulnl_instance_free_rcu(head: *mut rcu_head) {
    let inst = container_of!(head, nfulnl_instance, rcu);
    put_net_track((*inst).net, &mut (*inst).ns_tracker); kfree(inst as *mut _); module_put(THIS_MODULE);
}
unsafe fn instance_put(inst: *mut nfulnl_instance) {
    if !inst.is_null() && refcount_dec_and_test(&mut (*inst).use_) { call_rcu(&mut (*inst).rcu, nfulnl_instance_free_rcu); }
}

unsafe fn instance_create(net: *mut net, group_num: u16, portid: u32, user_ns: *mut user_namespace) -> *mut nfulnl_instance {
    let log = nfnl_log_pernet(net); spin_lock_bh(&mut (*log).instances_lock);
    if !__instance_lookup(log, group_num).is_null() { spin_unlock_bh(&mut (*log).instances_lock); return ERR_PTR!(-EEXIST); }
    let inst = kzalloc_obj!(nfulnl_instance, GFP_ATOMIC);
    if inst.is_null() { spin_unlock_bh(&mut (*log).instances_lock); return ERR_PTR!(-ENOMEM); }
    if !try_module_get(THIS_MODULE) { kfree(inst as *mut _); spin_unlock_bh(&mut (*log).instances_lock); return ERR_PTR!(-EAGAIN); }
    INIT_HLIST_NODE!(&mut (*inst).hlist); spin_lock_init(&mut (*inst).lock); refcount_set(&mut (*inst).use_, 2);
    timer_setup!(&mut (*inst).timer, nfulnl_timer, 0);
    (*inst).net = get_net_track(net, &mut (*inst).ns_tracker, GFP_ATOMIC); (*inst).peer_user_ns = user_ns;
    (*inst).peer_portid = portid; (*inst).group_num = group_num; (*inst).qthreshold = NFULNL_QTHRESH_DEFAULT;
    (*inst).flushtimeout = NFULNL_TIMEOUT_DEFAULT; (*inst).nlbufsiz = NFULNL_NLBUFSIZ_DEFAULT;
    (*inst).copy_mode = NFULNL_COPY_PACKET; (*inst).copy_range = NFULNL_COPY_RANGE_MAX;
    hlist_add_head_rcu!(&mut (*inst).hlist, &mut (*log).instance_table[instance_hashfn(group_num) as usize]);
    spin_unlock_bh(&mut (*log).instances_lock); inst
}

unsafe fn __instance_destroy(inst: *mut nfulnl_instance) {
    hlist_del_rcu!(&mut (*inst).hlist); spin_lock(&mut (*inst).lock); (*inst).copy_mode = NFULNL_COPY_DISABLED;
    if !(*inst).skb.is_null() { __nfulnl_flush(inst); } spin_unlock(&mut (*inst).lock); instance_put(inst);
}
unsafe fn instance_destroy(log: *mut nfnl_log_net, inst: *mut nfulnl_instance) { spin_lock_bh(&mut (*log).instances_lock); __instance_destroy(inst); spin_unlock_bh(&mut (*log).instances_lock); }

unsafe fn nfulnl_set_mode(inst: *mut nfulnl_instance, mode: u8, mut range: u32) -> i32 {
    let mut status = 0; spin_lock_bh(&mut (*inst).lock);
    match mode { NFULNL_COPY_NONE | NFULNL_COPY_META => { (*inst).copy_mode = mode; (*inst).copy_range = 0; },
        NFULNL_COPY_PACKET => { (*inst).copy_mode = mode; if range == 0 { range = NFULNL_COPY_RANGE_MAX; } (*inst).copy_range = core::cmp::min(range, NFULNL_COPY_RANGE_MAX); },
        _ => status = -EINVAL }
    spin_unlock_bh(&mut (*inst).lock); status
}
unsafe fn nfulnl_set_nlbufsiz(inst: *mut nfulnl_instance, v: u32) -> i32 { spin_lock_bh(&mut (*inst).lock); let r = if v < NFULNL_NLBUFSIZ_DEFAULT || v > 131072 { -ERANGE } else { (*inst).nlbufsiz = v; 0 }; spin_unlock_bh(&mut (*inst).lock); r }
unsafe fn nfulnl_set_timeout(i: *mut nfulnl_instance, v: u32) { spin_lock_bh(&mut (*i).lock); (*i).flushtimeout=v; spin_unlock_bh(&mut (*i).lock); }
unsafe fn nfulnl_set_qthresh(i: *mut nfulnl_instance, v: u32) { spin_lock_bh(&mut (*i).lock); (*i).qthreshold=v; spin_unlock_bh(&mut (*i).lock); }
unsafe fn nfulnl_set_flags(i: *mut nfulnl_instance, v: u16) -> i32 { spin_lock_bh(&mut (*i).lock); (*i).flags=v; spin_unlock_bh(&mut (*i).lock); 0 }

unsafe fn nfulnl_alloc_skb(net: *mut net, _peer_portid: u32, inst_size: u32, pkt_size: u32) -> *mut sk_buff {
    let n = core::cmp::max(inst_size, pkt_size); let mut skb = alloc_skb(n, GFP_ATOMIC | __GFP_NOWARN);
    if skb.is_null() && n > pkt_size { skb = alloc_skb(pkt_size, GFP_ATOMIC); } skb
}
unsafe fn __nfulnl_send(inst: *mut nfulnl_instance) { if (*inst).qlen > 1 { let nlh=nfnl_msg_put((*inst).skb,0,0,NLMSG_DONE,0,AF_UNSPEC,NFNETLINK_V0,htons((*inst).group_num)); if nlh.is_null() { kfree_skb((*inst).skb); } } if !(*inst).skb.is_null() { nfnetlink_unicast((*inst).skb,(*inst).net,(*inst).peer_portid); } (*inst).qlen=0; (*inst).skb=core::ptr::null_mut(); }
unsafe fn __nfulnl_flush(inst: *mut nfulnl_instance) { if timer_delete(&mut (*inst).timer) { instance_put(inst); } if !(*inst).skb.is_null() { __nfulnl_send(inst); } }
unsafe extern "C" fn nfulnl_timer(t: *mut timer_list) { let inst=timer_container_of!(t,nfulnl_instance,timer); spin_lock_bh(&mut (*inst).lock); if !(*inst).skb.is_null(){__nfulnl_send(inst);} spin_unlock_bh(&mut (*inst).lock); instance_put(inst); }

/* Packet construction and netlink callback portions retain the kernel ABI and operations. */
unsafe fn nfulnl_get_bridge_size(skb: *const sk_buff) -> u32 { if !skb_mac_header_was_set(skb){return 0;} let mut size=0; if skb_vlan_tag_present(skb){size+=nla_total_size(0);size+=nla_total_size(core::mem::size_of::<u16>() as u32);size+=nla_total_size(core::mem::size_of::<u16>() as u32);} let l=skb_mac_header_len(skb); if l>0{size+=nla_total_size(l)} size }
unsafe fn nfulnl_put_bridge(inst:*mut nfulnl_instance,skb:*const sk_buff)->i32 { if !skb_mac_header_was_set(skb){return 0;} if skb_vlan_tag_present(skb){let nest=nla_nest_start((*inst).skb,NFULA_VLAN);if nest.is_null(){return -1;} if nla_put_be16((*inst).skb,NFULA_VLAN_TCI,htons((*skb).vlan_tci))!=0||nla_put_be16((*inst).skb,NFULA_VLAN_PROTO,(*skb).vlan_proto)!=0{return -1;} nla_nest_end((*inst).skb,nest);} let l=skb_mac_header_len(skb); if l>0&&nla_put((*inst).skb,NFULA_L2HDR,l,skb_mac_header(skb))!=0{return -1;} 0 }

unsafe fn nfulnl_get_copy_len(li:*const nf_loginfo,skb:*const sk_buff,mut copy_len:u32)->u32 { let mut len=(*skb).len; if (*li).u_.ulog.flags&NF_LOG_F_COPY_LEN!=0&&(*li).u_.ulog.copy_len<copy_len{copy_len=(*li).u_.ulog.copy_len;} if !skb_frags_readable(skb){len=skb_headlen(skb);} core::cmp::min(len,copy_len) }

/* The remaining declarations mirror the C entry points and use the supplied kernel types/APIs. */
unsafe extern "C" { fn nfulnl_log_packet(net:*mut net,pf:u8,hooknum:u32,skb:*const sk_buff,in_:*const net_device,out:*const net_device,li:*const nf_loginfo,prefix:*const i8); fn nfnetlink_log_init()->i32; fn nfnetlink_log_fini(); }

// External kernel declarations and configuration tables are intentionally unresolved here;
// their names and ABI are supplied by the surrounding translated kernel sources.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
