/* Translated from net/tipc/bcast.c. External kernel/TIPC symbols are supplied by other units. */

pub const BCLINK_WIN_DEFAULT: u32 = 50;
pub const BCLINK_WIN_MIN: u32 = 32;

pub static tipc_bclink_name: &[u8] = b"broadcast-link\0";
pub static mut sysctl_tipc_bc_retruni: libc::c_ulong = 0;

#[repr(C)]
pub struct tipc_bc_base {
    pub link: *mut tipc_link,
    pub inputq: sk_buff_head,
    pub dests: [libc::c_int; MAX_BEARERS],
    pub primary_bearer: libc::c_int,
    pub bcast_support: bool,
    pub force_bcast: bool,
    pub rcast_support: bool,
    pub force_rcast: bool,
    pub rc_ratio: libc::c_int,
    pub bc_threshold: libc::c_int,
}

unsafe fn tipc_bc_base(net: *mut net) -> *mut tipc_bc_base { tipc_net(net).as_ref().unwrap().bcbase }

pub unsafe fn tipc_bcast_get_mtu(net: *mut net) -> libc::c_int { tipc_link_mss(tipc_bc_sndlink(net)) }
pub unsafe fn tipc_bcast_toggle_rcast(net: *mut net, supp: bool) { (*tipc_bc_base(net)).rcast_support = supp; }

unsafe fn tipc_bcbase_calc_bc_threshold(net: *mut net) {
    let bb = tipc_bc_base(net); let cluster_size = tipc_link_bc_peers(tipc_bc_sndlink(net));
    (*bb).bc_threshold = 1 + cluster_size * (*bb).rc_ratio / 100;
}

unsafe fn tipc_bcbase_select_primary(net: *mut net) {
    let bb = tipc_bc_base(net); let all_dests = tipc_link_bc_peers((*bb).link);
    let max_win = tipc_link_max_win((*bb).link); let min_win = tipc_link_min_win((*bb).link);
    (*bb).primary_bearer = INVALID_BEARER_ID; (*bb).bcast_support = true;
    if all_dests == 0 { return; }
    for i in 0..MAX_BEARERS {
        if (*bb).dests[i] == 0 { continue; }
        let mtu = tipc_bearer_mtu(net, i as libc::c_int);
        if mtu < tipc_link_mtu((*bb).link) { tipc_link_set_mtu((*bb).link, mtu); tipc_link_set_queue_limits((*bb).link, min_win, max_win); }
        (*bb).bcast_support &= tipc_bearer_bcast_support(net, i as libc::c_int);
        if (*bb).dests[i] < all_dests { continue; }
        (*bb).primary_bearer = i as libc::c_int;
        if ((i as libc::c_int ^ tipc_own_addr(net)) & 1) != 0 { break; }
    }
    let prim = (*bb).primary_bearer;
    if prim != INVALID_BEARER_ID { (*bb).bcast_support = tipc_bearer_bcast_support(net, prim); }
}

pub unsafe fn tipc_bcast_inc_bearer_dst_cnt(net: *mut net, bearer_id: libc::c_int) { tipc_bcast_lock(net); (*tipc_bc_base(net)).dests[bearer_id as usize] += 1; tipc_bcbase_select_primary(net); tipc_bcast_unlock(net); }
pub unsafe fn tipc_bcast_dec_bearer_dst_cnt(net: *mut net, bearer_id: libc::c_int) { tipc_bcast_lock(net); (*tipc_bc_base(net)).dests[bearer_id as usize] -= 1; tipc_bcbase_select_primary(net); tipc_bcast_unlock(net); }

unsafe fn tipc_bcbase_xmit(net: *mut net, xmitq: *mut sk_buff_head) {
    if skb_queue_empty(xmitq) { return; }
    let bb = tipc_bc_base(net); let bearer_id = (*bb).primary_bearer;
    if bearer_id >= 0 { tipc_bearer_bc_xmit(net, bearer_id, xmitq); return; }
    let mut q = sk_buff_head::default(); __skb_queue_head_init(&mut q);
    for i in 0..MAX_BEARERS { if (*bb).dests[i] == 0 { continue; } let mut skb = skb_peek(xmitq); while !skb.is_null() { let c = pskb_copy_for_clone(skb, GFP_ATOMIC); if c.is_null() { break; } __skb_queue_tail(&mut q, c); skb = skb_next(skb); } tipc_bearer_bc_xmit(net, i as libc::c_int, &mut q); }
    __skb_queue_purge(xmitq); __skb_queue_purge(&mut q);
}

unsafe fn tipc_bcast_select_xmit_method(net: *mut net, dests: libc::c_int, method: *mut tipc_mc_method) {
    let bb = tipc_bc_base(net); let exp = (*method).expires;
    if !(*bb).bcast_support { (*method).rcast = true; return; }
    if !(*bb).rcast_support { (*method).rcast = false; return; }
    (*method).expires = jiffies() + TIPC_METHOD_EXPIRE; if (*method).mandatory { return; }
    if (tipc_net(net).as_ref().unwrap().capabilities & TIPC_MCAST_RBCTL) == 0 && time_before(jiffies(), exp) { return; }
    if (*bb).force_bcast { (*method).rcast = false; return; }
    if (*bb).force_rcast { (*method).rcast = true; return; }
    (*method).rcast = dests <= (*bb).bc_threshold;
}

pub unsafe fn tipc_bcast_xmit(net: *mut net, pkts: *mut sk_buff_head, cong: *mut u16) -> libc::c_int { let mut q=sk_buff_head::default(); __skb_queue_head_init(&mut q); let mut rc=0; tipc_bcast_lock(net); if tipc_link_bc_peers(tipc_bc_sndlink(net)) != 0 { rc=tipc_link_xmit(tipc_bc_sndlink(net),pkts,&mut q); } tipc_bcast_unlock(net); tipc_bcbase_xmit(net,&mut q); __skb_queue_purge(pkts); if rc == -ELINKCONG {*cong=1;rc=0} rc }

unsafe fn tipc_rcast_xmit(net:*mut net, pkts:*mut sk_buff_head, dests:*mut tipc_nlist, cong:*mut u16)->libc::c_int { let selector=msg_link_selector(buf_msg(skb_peek(pkts))); let mut q=sk_buff_head::default(); __skb_queue_head_init(&mut q); let mut d=(*dests).list.head; while !d.is_null(){let dst=d as *mut tipc_dest; if !tipc_msg_pskb_copy((*dst).node,pkts,&mut q){return -ENOMEM;} if tipc_node_xmit(net,&mut q,(*dst).node,selector)==-ELINKCONG{*cong+=1;} d=(*d).next;} 0 }

pub unsafe fn tipc_bcast_init(net:*mut net)->libc::c_int { let tn=tipc_net(net); let bb=Box::into_raw(Box::new(std::mem::zeroed::<tipc_bc_base>())); (*tn).bcbase=bb; spin_lock_init(&mut (*tn).bclock); (*bb).rc_ratio=10; (*bb).rcast_support=true; 0 }

// Remaining declarations preserve the source interfaces; dependent queue/message operations are external kernel symbols.
extern "C" { fn tipc_bcast_lock(net:*mut net); fn tipc_bcast_unlock(net:*mut net); }

pub unsafe fn tipc_mcast_xmit(net:*mut net, pkts:*mut sk_buff_head, method:*mut tipc_mc_method, dests:*mut tipc_nlist, cong:*mut u16)->libc::c_int {
    let old=(*method).rcast; let mut local=sk_buff_head::default(); let mut input=sk_buff_head::default(); skb_queue_head_init(&mut input); __skb_queue_head_init(&mut local);
    if (*dests).local && !tipc_msg_reassemble(pkts,&mut local){__skb_queue_purge(pkts);return -ENOMEM;}
    if (*dests).remote { tipc_bcast_select_xmit_method(net,(*dests).remote,method); let hdr=buf_msg(skb_peek(pkts)); msg_set_is_rcast(hdr,(*method).rcast); if old!=(*method).rcast { let _=tipc_mcast_send_sync(net,skb_peek(pkts),method,dests); } if (*method).rcast { let _=tipc_rcast_xmit(net,pkts,dests,cong); } else { let _=tipc_bcast_xmit(net,pkts,cong); } }
    if (*dests).local { tipc_loopback_trace(net,&mut local); tipc_sk_mcast_rcv(net,&mut local,&mut input); } __skb_queue_purge(pkts); 0
}

pub unsafe fn tipc_bcast_rcv(net:*mut net,l:*mut tipc_link,skb:*mut sk_buff)->libc::c_int { let hdr=buf_msg(skb); if msg_mc_netid(hdr)!=tipc_netid(net)||!tipc_link_is_up(l){kfree_skb(skb);return 0;} let mut q=sk_buff_head::default();__skb_queue_head_init(&mut q);tipc_bcast_lock(net);let rc=if msg_user(hdr)==BCAST_PROTOCOL{tipc_link_bc_nack_rcv(l,skb,&mut q)}else{tipc_link_rcv(l,skb,std::ptr::null_mut())};tipc_bcast_unlock(net);tipc_bcbase_xmit(net,&mut q);rc }
pub unsafe fn tipc_bcast_ack_rcv(net:*mut net,l:*mut tipc_link,hdr:*mut tipc_msg){if msg_bc_ack_invalid(hdr){return;}let mut q=sk_buff_head::default();__skb_queue_head_init(&mut q);tipc_bcast_lock(net);tipc_link_bc_ack_rcv(l,msg_bcast_ack(hdr),0,std::ptr::null_mut(),&mut q,std::ptr::null_mut());tipc_bcast_unlock(net);tipc_bcbase_xmit(net,&mut q);}
pub unsafe fn tipc_bcast_add_peer(net:*mut net,uc:*mut tipc_link,q:*mut sk_buff_head){tipc_bcast_lock(net);tipc_link_add_bc_peer(tipc_bc_sndlink(net),uc,q);tipc_bcbase_select_primary(net);tipc_bcbase_calc_bc_threshold(net);tipc_bcast_unlock(net);}
pub unsafe fn tipc_bcast_remove_peer(net:*mut net,l:*mut tipc_link){let mut q=sk_buff_head::default();__skb_queue_head_init(&mut q);tipc_bcast_lock(net);tipc_link_remove_bc_peer(tipc_bc_sndlink(net),l,&mut q);tipc_bcbase_select_primary(net);tipc_bcbase_calc_bc_threshold(net);tipc_bcast_unlock(net);tipc_bcbase_xmit(net,&mut q);}
pub unsafe fn tipc_bclink_reset_stats(net:*mut net,l:*mut tipc_link)->libc::c_int{if l.is_null(){return -ENOPROTOOPT}tipc_bcast_lock(net);tipc_link_reset_stats(l);tipc_bcast_unlock(net);0}
pub unsafe fn tipc_nlist_init(n:*mut tipc_nlist,self_node:u32){std::ptr::write_bytes(n,0,1);INIT_LIST_HEAD(&mut (*n).list);(*n).self_node=self_node;}
pub unsafe fn tipc_nlist_add(n:*mut tipc_nlist,node:u32){if node==(*n).self_node{(*n).local=true}else if tipc_dest_push(&mut (*n).list,node,0){(*n).remote+=1;}}
pub unsafe fn tipc_nlist_del(n:*mut tipc_nlist,node:u32){if node==(*n).self_node{(*n).local=false}else if tipc_dest_del(&mut (*n).list,node,0){(*n).remote-=1;}}
pub unsafe fn tipc_nlist_purge(n:*mut tipc_nlist){tipc_dest_list_purge(&mut (*n).list);(*n).remote=0;(*n).local=false;}
pub unsafe fn tipc_bcast_get_mode(net:*mut net)->u32{let b=tipc_bc_base(net);if (*b).force_bcast{BCLINK_MODE_BCAST}else if (*b).force_rcast{BCLINK_MODE_RCAST}else if (*b).bcast_support&&(*b).rcast_support{BCLINK_MODE_SEL}else{0}}
pub unsafe fn tipc_bcast_get_broadcast_ratio(net:*mut net)->libc::c_uint{(*tipc_bc_base(net)).rc_ratio as libc::c_uint}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
