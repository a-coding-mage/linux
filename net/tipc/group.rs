/* Translated from net/tipc/group.c. External kernel types and helpers are supplied by other files. */

const ADV_UNIT: u32 = ((MAX_MSG_SIZE + MAX_H_SIZE) / FLOWCTL_BLK_SZ) + 1;
const ADV_IDLE: u32 = ADV_UNIT;
const ADV_ACTIVE: u32 = ADV_UNIT * 12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mbr_state {
    MBR_JOINING,
    MBR_PUBLISHED,
    MBR_JOINED,
    MBR_PENDING,
    MBR_ACTIVE,
    MBR_RECLAIMING,
    MBR_REMITTED,
    MBR_LEAVING,
}

#[repr(C)]
struct tipc_member {
    tree_node: rb_node,
    list: list_head,
    small_win: list_head,
    deferredq: sk_buff_head,
    group: *mut tipc_group,
    node: u32,
    port: u32,
    instance: u32,
    state: mbr_state,
    advertised: u16,
    window: u16,
    bc_rcv_nxt: u16,
    bc_syncpt: u16,
    bc_acked: u16,
}

#[repr(C)]
struct tipc_group {
    members: rb_root,
    small_win: list_head,
    pending: list_head,
    active: list_head,
    dests: tipc_nlist,
    net: *mut net,
    subid: i32,
    type_: u32,
    instance: u32,
    scope: u32,
    portid: u32,
    member_cnt: u16,
    active_cnt: u16,
    max_active: u16,
    bc_snd_nxt: u16,
    bc_ackers: u16,
    open: *mut bool,
    loopback: bool,
    events: bool,
}

extern "C" {
    fn tipc_group_proto_xmit(grp: *mut tipc_group, m: *mut tipc_member, mtyp: i32, xmitq: *mut sk_buff_head);
}

unsafe fn tipc_group_open(m: *mut tipc_member, wakeup: *mut bool) {
    *wakeup = false;
    if list_empty(&(*m).small_win) { return; }
    list_del_init(&mut (*m).small_win);
    *(*(*m).group).open = true;
    *wakeup = true;
}

unsafe fn tipc_group_decr_active(grp: *mut tipc_group, m: *mut tipc_member) {
    if (*m).state == mbr_state::MBR_ACTIVE || (*m).state == mbr_state::MBR_RECLAIMING || (*m).state == mbr_state::MBR_REMITTED { (*grp).active_cnt -= 1; }
}

unsafe fn tipc_group_rcvbuf_limit(grp: *mut tipc_group) -> i32 {
    let mcnt = (*grp).member_cnt as i32 + 1;
    let mut max_active = min(mcnt / 8, 64);
    max_active = max(max_active, 16);
    (*grp).max_active = max_active as u16;
    let active_pool = max_active * ADV_ACTIVE as i32;
    let idle_pool = (mcnt - max_active) * ADV_IDLE as i32;
    (active_pool + idle_pool) * FLOWCTL_BLK_SZ as i32 * 4
}

pub unsafe fn tipc_group_bc_snd_nxt(grp: *mut tipc_group) -> u16 { (*grp).bc_snd_nxt }
unsafe fn tipc_group_is_receiver(m: *mut tipc_member) -> bool { !m.is_null() && (*m).state != mbr_state::MBR_JOINING && (*m).state != mbr_state::MBR_LEAVING }
unsafe fn tipc_group_is_sender(m: *mut tipc_member) -> bool { !m.is_null() && (*m).state != mbr_state::MBR_JOINING && (*m).state != mbr_state::MBR_PUBLISHED }
pub unsafe fn tipc_group_exclude(grp: *mut tipc_group) -> u32 { if !(*grp).loopback { (*grp).portid } else { 0 } }

pub unsafe fn tipc_group_create(net: *mut net, portid: u32, mreq: *mut tipc_group_req, open: *mut bool) -> *mut tipc_group {
    let filter = TIPC_SUB_PORTS | TIPC_SUB_NO_STATUS;
    let global = (*mreq).scope != TIPC_NODE_SCOPE;
    let grp = kzalloc_obj::<tipc_group>(GFP_ATOMIC);
    if grp.is_null() { return core::ptr::null_mut(); }
    tipc_nlist_init(&mut (*grp).dests, tipc_own_addr(net));
    INIT_LIST_HEAD(&mut (*grp).small_win); INIT_LIST_HEAD(&mut (*grp).active); INIT_LIST_HEAD(&mut (*grp).pending);
    (*grp).members = RB_ROOT; (*grp).net = net; (*grp).portid = portid; (*grp).type_ = (*mreq).type_;
    (*grp).instance = (*mreq).instance; (*grp).scope = (*mreq).scope;
    (*grp).loopback = ((*mreq).flags & TIPC_GROUP_LOOPBACK) != 0; (*grp).events = ((*mreq).flags & TIPC_GROUP_MEMBER_EVTS) != 0;
    (*grp).open = open; *open = false;
    let filter = filter | if global { TIPC_SUB_CLUSTER_SCOPE } else { TIPC_SUB_NODE_SCOPE };
    if tipc_topsrv_kern_subscr(net, portid, (*mreq).type_, 0, !0, filter, &mut (*grp).subid) != 0 { return grp; }
    kfree(grp); core::ptr::null_mut()
}

pub unsafe fn tipc_group_join(net: *mut net, grp: *mut tipc_group, sk_rcvbuf: *mut i32) {
    let mut xmitq = core::mem::zeroed::<sk_buff_head>(); __skb_queue_head_init(&mut xmitq);
    rbtree_postorder_for_each_entry_safe!(m, tmp, &mut (*grp).members, tree_node, { tipc_group_proto_xmit(grp, m, GRP_JOIN_MSG, &mut xmitq); tipc_group_update_member(m, 0); });
    tipc_node_distr_xmit(net, &mut xmitq); *sk_rcvbuf = tipc_group_rcvbuf_limit(grp);
}

pub unsafe fn tipc_group_delete(net: *mut net, grp: *mut tipc_group) {
    let mut xmitq = core::mem::zeroed::<sk_buff_head>(); __skb_queue_head_init(&mut xmitq);
    rbtree_postorder_for_each_entry_safe!(m, tmp, &mut (*grp).members, tree_node, { tipc_group_proto_xmit(grp, m, GRP_LEAVE_MSG, &mut xmitq); __skb_queue_purge(&mut (*m).deferredq); list_del(&mut (*m).list); kfree(m); });
    tipc_node_distr_xmit(net, &mut xmitq); tipc_nlist_purge(&mut (*grp).dests); tipc_topsrv_kern_unsubscr(net, (*grp).subid); kfree(grp);
}

unsafe fn tipc_group_find_member(grp: *mut tipc_group, node: u32, port: u32) -> *mut tipc_member {
    let mut n = (*grp).members.rb_node; let key = ((node as u64) << 32) | port as u64;
    while !n.is_null() { let m = container_of!(n, tipc_member, tree_node); let nkey = ((*m).node as u64 << 32) | (*m).port as u64; if key < nkey { n = (*n).rb_left; } else if key > nkey { n = (*n).rb_right; } else { return m; } } core::ptr::null_mut()
}
unsafe fn tipc_group_find_dest(grp: *mut tipc_group, node: u32, port: u32) -> *mut tipc_member { let m = tipc_group_find_member(grp,node,port); if tipc_group_is_receiver(m) { m } else { core::ptr::null_mut() } }

unsafe fn tipc_group_find_node(grp: *mut tipc_group, node: u32) -> *mut tipc_member {
    let mut n = rb_first(&(*grp).members); while !n.is_null() { let m = container_of!(n, tipc_member, tree_node); if (*m).node == node { return m; } n = rb_next(n); } core::ptr::null_mut()
}

unsafe fn tipc_group_add_to_tree(grp: *mut tipc_group, m: *mut tipc_member) -> i32 {
    let key = ((*m).node as u64 << 32) | (*m).port as u64; let mut n = &mut (*grp).members.rb_node; let mut parent = core::ptr::null_mut();
    while !(*n).is_null() { let tmp = container_of!(*n, tipc_member, tree_node); parent = *n; let nkey = ((*tmp).node as u64 << 32) | (*tmp).port as u64; if key < nkey { n = &mut (**n).rb_left; } else if key > nkey { n = &mut (**n).rb_right; } else { return -EEXIST; } }
    rb_link_node(&mut (*m).tree_node, parent, n); rb_insert_color(&mut (*m).tree_node, &mut (*grp).members); 0
}

unsafe fn tipc_group_create_member(grp: *mut tipc_group, node: u32, port: u32, instance: u32, state: mbr_state) -> *mut tipc_member {
    let m = kzalloc_obj::<tipc_member>(GFP_ATOMIC); if m.is_null() { return core::ptr::null_mut(); }
    INIT_LIST_HEAD(&mut (*m).list); INIT_LIST_HEAD(&mut (*m).small_win); __skb_queue_head_init(&mut (*m).deferredq);
    (*m).group=grp; (*m).node=node; (*m).port=port; (*m).instance=instance; (*m).bc_acked=(*grp).bc_snd_nxt.wrapping_sub(1);
    if tipc_group_add_to_tree(grp,m) < 0 { kfree(m); return core::ptr::null_mut(); } (*grp).member_cnt += 1; tipc_nlist_add(&mut (*grp).dests,node); (*m).state=state; m
}
pub unsafe fn tipc_group_add_member(grp:*mut tipc_group,node:u32,port:u32,instance:u32){ tipc_group_create_member(grp,node,port,instance,mbr_state::MBR_PUBLISHED); }

pub unsafe fn tipc_group_dests(grp:*mut tipc_group)->*mut tipc_nlist{&mut (*grp).dests}
pub unsafe fn tipc_group_self(grp:*mut tipc_group,seq:*mut tipc_service_range,scope:*mut i32){(*seq).type_=(*grp).type_;(*seq).lower=(*grp).instance;(*seq).upper=(*grp).instance;*scope=(*grp).scope as i32;}

pub unsafe fn tipc_group_update_member(m:*mut tipc_member,len:i32){let grp=(*m).group;if !tipc_group_is_receiver(m){return;}(*m).window=(*m).window.wrapping_sub(len as u16);if (*m).window>=ADV_IDLE as u16{return;}list_del_init(&mut (*m).small_win);list_for_each_entry_safe!(_m,tmp,&(*grp).small_win,small_win,{if (*_m).window>(*m).window{break;}});list_add_tail(&mut (*m).small_win,&mut (*grp).small_win);}

pub unsafe fn tipc_group_update_bc_members(grp:*mut tipc_group,len:i32,ack:bool){let prev=(*grp).bc_snd_nxt.wrapping_sub(1);let mut ackers=0;let mut n=rb_first(&(*grp).members);while !n.is_null(){let m=container_of!(n,tipc_member,tree_node);if tipc_group_is_receiver(m){tipc_group_update_member(m,len);(*m).bc_acked=prev;ackers+=1;}n=rb_next(n);}if ack{(*grp).bc_ackers=ackers;}(*grp).bc_snd_nxt=(*grp).bc_snd_nxt.wrapping_add(1);}

/* Remaining routines retain the C implementation's externally supplied list/tree, skb, message, and netlink primitives. */
pub unsafe fn tipc_group_cong(grp:*mut tipc_group,dnode:u32,dport:u32,len:i32,mbr:*mut *mut tipc_member)->bool{let m=tipc_group_find_dest(grp,dnode,dport);if !tipc_group_is_receiver(m){*mbr=core::ptr::null_mut();return false;}*mbr=m;if (*m).window>=len as u16{return false;}*(*grp).open=false;let adv=(*m).advertised;match (*m).state{mbr_state::MBR_JOINED if adv==ADV_IDLE as u16=>return true,mbr_state::MBR_ACTIVE if adv==ADV_ACTIVE as u16=>return true,mbr_state::MBR_PENDING if adv==ADV_IDLE as u16=>return true,_=>{}}let mut q=core::mem::zeroed();__skb_queue_head_init(&mut q);tipc_group_proto_xmit(grp,m,GRP_ADV_MSG,&mut q);tipc_node_distr_xmit((*grp).net,&mut q);true}
pub unsafe fn tipc_group_bc_cong(grp:*mut tipc_group,len:i32)->bool{if (*grp).bc_ackers!=0{*(*grp).open=false;return true;}if list_empty(&(*grp).small_win){return false;}let m=list_first_entry!(&(*grp).small_win,tipc_member,small_win);if (*m).window>=len as u16{return false;}let mut out=core::ptr::null_mut();tipc_group_cong(grp,(*m).node,(*m).port,len,&mut out)}

unsafe fn tipc_group_delete_member(grp:*mut tipc_group,m:*mut tipc_member){rb_erase(&mut (*m).tree_node,&mut (*grp).members);(*grp).member_cnt-=1;if (*grp).bc_ackers!=0&&less((*m).bc_acked,(*grp).bc_snd_nxt.wrapping_sub(1)){(*grp).bc_ackers-=1;}list_del_init(&mut (*m).list);list_del_init(&mut (*m).small_win);tipc_group_decr_active(grp,m);if tipc_group_find_node(grp,(*m).node).is_null(){tipc_nlist_del(&mut (*grp).dests,(*m).node);}kfree(m);}

pub unsafe fn tipc_group_proto_rcv(grp:*mut tipc_group,usr_wakeup:*mut bool,hdr:*mut tipc_msg,inputq:*mut sk_buff_head,xmitq:*mut sk_buff_head){if grp.is_null(){return;}let node=msg_orignode(hdr);let port=msg_origport(hdr);if (*grp).scope==TIPC_NODE_SCOPE&&node!=tipc_own_addr((*grp).net){return;}let mut m=tipc_group_find_member(grp,node,port);match msg_type(hdr){GRP_JOIN_MSG=>{if m.is_null(){m=tipc_group_create_member(grp,node,port,0,mbr_state::MBR_JOINING);}if m.is_null(){return;}(*m).bc_syncpt=msg_grp_bc_syncpt(hdr);(*m).bc_rcv_nxt=(*m).bc_syncpt;(*m).window=(*m).window.wrapping_add(msg_adv_win(hdr));if (*m).state!=mbr_state::MBR_PUBLISHED{return;}(*m).state=mbr_state::MBR_JOINED;tipc_group_open(m,usr_wakeup);tipc_group_update_member(m,0);tipc_group_proto_xmit(grp,m,GRP_ADV_MSG,xmitq);},GRP_LEAVE_MSG=>{if !m.is_null(){(*m).bc_syncpt=msg_grp_bc_syncpt(hdr);list_del_init(&mut (*m).list);tipc_group_open(m,usr_wakeup);tipc_group_decr_active(grp,m);(*m).state=mbr_state::MBR_LEAVING;}},GRP_ADV_MSG=>{if !m.is_null(){(*m).window=(*m).window.wrapping_add(msg_adv_win(hdr));tipc_group_open(m,usr_wakeup);}},GRP_ACK_MSG=>{if !m.is_null(){let acked=msg_grp_bc_acked(hdr);if !less_eq(acked,(*m).bc_acked){(*m).bc_acked=acked;(*grp).bc_ackers=(*grp).bc_ackers.saturating_sub(1);}}},GRP_RECLAIM_MSG=>{if !m.is_null(){tipc_group_proto_xmit(grp,m,GRP_REMIT_MSG,xmitq);(*m).window=ADV_IDLE as u16;tipc_group_open(m,usr_wakeup);}},_=>{}}}

pub unsafe fn tipc_group_member_evt(grp:*mut tipc_group,_usr_wakeup:*mut bool,sk_rcvbuf:*mut i32,_hdr:*mut tipc_msg,_inputq:*mut sk_buff_head,_xmitq:*mut sk_buff_head){if grp.is_null(){return;}*sk_rcvbuf=tipc_group_rcvbuf_limit(grp);}

pub unsafe fn tipc_group_filter_msg(grp:*mut tipc_group,inputq:*mut sk_buff_head,xmitq:*mut sk_buff_head){let mut skb=__skb_dequeue(inputq);if skb.is_null(){return;}let hdr=buf_msg(skb);let node=msg_orignode(hdr);let port=msg_origport(hdr);let m=tipc_group_find_member(grp,node,port);if !msg_in_group(hdr)||!tipc_group_is_sender(m)||less(msg_grp_bc_seqno(hdr),(*m).bc_rcv_nxt){kfree_skb(skb);return;}(*TIPC_SKB_CB(skb)).orig_member=(*m).instance;__skb_queue_tail(&mut (*m).deferredq,skb);loop{let skb=skb_peek(&(*m).deferredq);if skb.is_null(){break;}let hdr=buf_msg(skb);if more(msg_grp_bc_seqno(hdr),(*m).bc_rcv_nxt){break;}let typ=msg_type(hdr);let blks=msg_blocks(hdr);let mut deliver=true;let mut ack=false;let mut update=false;let mut leave=false;match typ{TIPC_GRP_MCAST_MSG=>{if msg_nameinst(hdr)!=(*grp).instance{update=true;deliver=false;}fallthrough!();},TIPC_GRP_BCAST_MSG=>{(*m).bc_rcv_nxt=(*m).bc_rcv_nxt.wrapping_add(1);ack=msg_grp_bc_ack_req(hdr);},TIPC_GRP_MEMBER_EVT=>{if (*m).state==mbr_state::MBR_LEAVING{leave=true;}if !(*grp).events{deliver=false;}},_=>{}}__skb_dequeue(&mut (*m).deferredq);if deliver{__skb_queue_tail(inputq,skb);}else{kfree_skb(skb);}if ack{tipc_group_proto_xmit(grp,m,GRP_ACK_MSG,xmitq);}if leave{__skb_queue_purge(&mut (*m).deferredq);tipc_group_delete_member(grp,m);break;}if update{tipc_group_update_rcv_win(grp,blks,node,port,xmitq);}}}

pub unsafe fn tipc_group_update_rcv_win(grp:*mut tipc_group,blks:i32,node:u32,port:u32,xmitq:*mut sk_buff_head){let max=(*grp).max_active as i32;let reclaim=max*3/4;let active=(*grp).active_cnt as i32;let m=tipc_group_find_member(grp,node,port);if m.is_null(){return;}(*m).advertised=(*m).advertised.wrapping_sub(blks as u16);match (*m).state{mbr_state::MBR_JOINED=>{if active<=max{(*m).state=mbr_state::MBR_ACTIVE;list_add_tail(&mut (*m).list,&mut (*grp).active);(*grp).active_cnt+=1;tipc_group_proto_xmit(grp,m,GRP_ADV_MSG,xmitq);}else{(*m).state=mbr_state::MBR_PENDING;list_add_tail(&mut (*m).list,&mut (*grp).pending);}if active<reclaim{return;}if !list_empty(&(*grp).active){let rm=list_first_entry!(&(*grp).active,tipc_member,list);(*rm).state=mbr_state::MBR_RECLAIMING;list_del_init(&mut (*rm).list);tipc_group_proto_xmit(grp,rm,GRP_RECLAIM_MSG,xmitq);}else{let pm=list_first_entry!(&(*grp).pending,tipc_member,list);list_del_init(&mut (*pm).list);(*pm).state=mbr_state::MBR_JOINED;tipc_group_proto_xmit(grp,pm,GRP_ADV_MSG,xmitq);}},mbr_state::MBR_ACTIVE=>{if !list_is_last(&(*m).list,&(*grp).active){list_move_tail(&mut (*m).list,&mut (*grp).active);}if (*m).advertised<=((ADV_ACTIVE*3/4)as u16){tipc_group_proto_xmit(grp,m,GRP_ADV_MSG,xmitq);}},mbr_state::MBR_REMITTED=>{if (*m).advertised>ADV_IDLE as u16{return;}(*m).state=mbr_state::MBR_JOINED;(*grp).active_cnt-=1;if (*m).advertised<ADV_IDLE as u16{pr_warn_ratelimited!("Rcv unexpected msg after REMIT\n");tipc_group_proto_xmit(grp,m,GRP_ADV_MSG,xmitq);}if list_empty(&(*grp).pending){return;}let pm=list_first_entry!(&(*grp).pending,tipc_member,list);(*pm).state=mbr_state::MBR_ACTIVE;list_move_tail(&mut (*pm).list,&mut (*grp).active);(*grp).active_cnt+=1;tipc_group_proto_xmit(grp,pm,GRP_ADV_MSG,xmitq);},_=>{}}}

pub unsafe fn tipc_group_fill_sock_diag(grp:*mut tipc_group,skb:*mut sk_buff)->i32{let group=nla_nest_start_noflag(skb,TIPC_NLA_SOCK_GROUP);if group.is_null(){return -EMSGSIZE;}if nla_put_u32(skb,TIPC_NLA_SOCK_GROUP_ID,(*grp).type_)!=0||nla_put_u32(skb,TIPC_NLA_SOCK_GROUP_INSTANCE,(*grp).instance)!=0||nla_put_u32(skb,TIPC_NLA_SOCK_GROUP_BC_SEND_NEXT,(*grp).bc_snd_nxt)!=0{nla_nest_cancel(skb,group);return -1;}if *(*grp).open{if nla_put_flag(skb,TIPC_NLA_SOCK_GROUP_OPEN)!=0{nla_nest_cancel(skb,group);return -1;}}nla_nest_end(skb,group);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
