// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP association implementation, translated from associola.c. */

/* C kernel headers and build-time configuration are supplied by other
 * translation units.  Their symbols remain external dependencies here. */

unsafe extern "C" {
    fn sctp_select_active_and_retran_path(asoc: *mut sctp_association);
    fn sctp_assoc_bh_rcv(work: *mut work_struct);
    fn sctp_assoc_free_asconf_acks(asoc: *mut sctp_association);
    fn sctp_assoc_free_asconf_queue(asoc: *mut sctp_association);
}

unsafe fn sctp_association_init(asoc: *mut sctp_association, ep: *const sctp_endpoint,
    sk: *const sock, scope: sctp_scope, gfp: gfp_t) -> *mut sctp_association {
    let sp = sctp_sk(sk as *mut sock);
    (*asoc).ep = ep as *mut sctp_endpoint;
    (*asoc).base.sk = sk as *mut sock;
    (*asoc).base.net = sock_net(sk);
    sctp_endpoint_hold((*asoc).ep); sock_hold((*asoc).base.sk);
    (*asoc).base.r#type = SCTP_EP_TYPE_ASSOCIATION;
    refcount_set(&mut (*asoc).base.refcnt, 1);
    sctp_bind_addr_init(&mut (*asoc).base.bind_addr, (*ep).base.bind_addr.port);
    (*asoc).state = SCTP_STATE_CLOSED;
    (*asoc).cookie_life = ms_to_ktime((*sp).assocparams.sasoc_cookie_life);
    (*asoc).user_frag = (*sp).user_frag;
    (*asoc).max_retrans = (*sp).assocparams.sasoc_asocmaxrxt;
    (*asoc).pf_retrans = (*sp).pf_retrans; (*asoc).ps_retrans = (*sp).ps_retrans;
    (*asoc).pf_expose = (*sp).pf_expose;
    (*asoc).rto_initial = msecs_to_jiffies((*sp).rtoinfo.srto_initial);
    (*asoc).rto_max = msecs_to_jiffies((*sp).rtoinfo.srto_max);
    (*asoc).rto_min = msecs_to_jiffies((*sp).rtoinfo.srto_min);
    (*asoc).hbinterval = msecs_to_jiffies((*sp).hbinterval);
    (*asoc).probe_interval = msecs_to_jiffies((*sp).probe_interval);
    (*asoc).encap_port = (*sp).encap_port; (*asoc).pathmaxrxt = (*sp).pathmaxrxt;
    (*asoc).flowlabel = (*sp).flowlabel; (*asoc).dscp = (*sp).dscp;
    (*asoc).sackdelay = msecs_to_jiffies((*sp).sackdelay); (*asoc).sackfreq = (*sp).sackfreq;
    (*asoc).param_flags = (*sp).param_flags; (*asoc).max_burst = (*sp).max_burst;
    (*asoc).subscribe = (*sp).subscribe;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_T1_COOKIE] = (*asoc).rto_initial;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_T1_INIT] = (*asoc).rto_initial;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_T2_SHUTDOWN] = (*asoc).rto_initial;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_T5_SHUTDOWN_GUARD] = 5 * (*asoc).rto_max;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_SACK] = (*asoc).sackdelay;
    (*asoc).timeouts[SCTP_EVENT_TIMEOUT_AUTOCLOSE] = (*sp).autoclose as u64 * HZ;
    for i in SCTP_EVENT_TIMEOUT_NONE..SCTP_NUM_TIMEOUT_TYPES {
        timer_setup(&mut (*asoc).timers[i], sctp_timer_events[i], 0);
    }
    (*asoc).c.sinit_max_instreams = (*sp).initmsg.sinit_max_instreams;
    (*asoc).c.sinit_num_ostreams = (*sp).initmsg.sinit_num_ostreams;
    (*asoc).max_init_attempts = (*sp).initmsg.sinit_max_attempts;
    (*asoc).max_init_timeo = msecs_to_jiffies((*sp).initmsg.sinit_max_init_timeo);
    (*asoc).rwnd = if (*sk).sk_rcvbuf / 2 < SCTP_DEFAULT_MINWINDOW { SCTP_DEFAULT_MINWINDOW } else { (*sk).sk_rcvbuf / 2 };
    (*asoc).a_rwnd = (*asoc).rwnd; (*asoc).peer.rwnd = SCTP_DEFAULT_MAXWINDOW;
    atomic_set(&mut (*asoc).rmem_alloc, 0); init_waitqueue_head(&mut (*asoc).wait);
    (*asoc).c.my_vtag = sctp_generate_tag(ep); (*asoc).c.my_port = (*ep).base.bind_addr.port;
    (*asoc).c.initial_tsn = sctp_generate_tsn(ep); (*asoc).next_tsn = (*asoc).c.initial_tsn;
    (*asoc).ctsn_ack_point = (*asoc).next_tsn - 1; (*asoc).adv_peer_ack_point = (*asoc).ctsn_ack_point;
    (*asoc).highest_sacked = (*asoc).ctsn_ack_point; (*asoc).last_cwr_tsn = (*asoc).ctsn_ack_point;
    (*asoc).addip_serial = (*asoc).c.initial_tsn; (*asoc).strreset_outseq = (*asoc).c.initial_tsn;
    INIT_LIST_HEAD(&mut (*asoc).addip_chunk_list); INIT_LIST_HEAD(&mut (*asoc).asconf_ack_list);
    INIT_LIST_HEAD(&mut (*asoc).peer.transport_addr_list); (*asoc).peer.sack_needed = 1; (*asoc).peer.sack_generation = 1;
    sctp_inq_init(&mut (*asoc).base.inqueue); sctp_inq_set_th_handler(&mut (*asoc).base.inqueue, sctp_assoc_bh_rcv);
    sctp_outq_init(asoc, &mut (*asoc).outqueue); sctp_ulpq_init(&mut (*asoc).ulpq, asoc);
    if sctp_stream_init(&mut (*asoc).stream, (*asoc).c.sinit_num_ostreams, 0, gfp) != 0 { goto stream_free; }
    (*asoc).pathmtu = (*sp).pathmtu; sctp_assoc_update_frag_point(asoc);
    (*asoc).peer.ipv4_address = 1; if (*asoc).base.sk.sk_family == PF_INET6 { (*asoc).peer.ipv6_address = 1; }
    INIT_LIST_HEAD(&mut (*asoc).asocs);
    (*asoc).default_stream = (*sp).default_stream; (*asoc).default_ppid = (*sp).default_ppid;
    (*asoc).default_flags = (*sp).default_flags; (*asoc).default_context = (*sp).default_context;
    (*asoc).default_timetolive = (*sp).default_timetolive; (*asoc).default_rcv_context = (*sp).default_rcv_context;
    INIT_LIST_HEAD(&mut (*asoc).endpoint_shared_keys);
    if sctp_auth_asoc_copy_shkeys(ep, asoc, gfp) != 0 { goto stream_free; }
    (*asoc).active_key_id = (*ep).active_key_id; (*asoc).strreset_enable = (*ep).strreset_enable;
    if !(*ep).auth_hmacs_list.is_null() { memcpy((*asoc).c.auth_hmacs.as_mut_ptr() as *mut _, (*ep).auth_hmacs_list as *const _, ntohs((*(*ep).auth_hmacs_list).param_hdr.length) as usize); }
    if !(*ep).auth_chunk_list.is_null() { memcpy((*asoc).c.auth_chunks.as_mut_ptr() as *mut _, (*ep).auth_chunk_list as *const _, ntohs((*(*ep).auth_chunk_list).param_hdr.length) as usize); }
    let p = (*asoc).c.auth_random.as_mut_ptr() as *mut sctp_paramhdr;
    (*p).r#type = SCTP_PARAM_RANDOM; (*p).length = htons(size_of::<sctp_paramhdr>() as u16 + SCTP_AUTH_RANDOM_LENGTH);
    get_random_bytes(p.add(1) as *mut _, SCTP_AUTH_RANDOM_LENGTH);
    return asoc;
stream_free:
    sctp_stream_free(&mut (*asoc).stream); sock_put((*asoc).base.sk); sctp_endpoint_put((*asoc).ep); std::ptr::null_mut()
}

pub unsafe fn sctp_association_new(ep: *const sctp_endpoint, sk: *const sock, scope: sctp_scope, gfp: gfp_t) -> *mut sctp_association {
    let asoc = kzalloc_obj::<sctp_association>(gfp); if asoc.is_null() { return std::ptr::null_mut(); }
    if sctp_association_init(asoc, ep, sk, scope, gfp).is_null() { kfree(asoc as *mut _); return std::ptr::null_mut(); }
    SCTP_DBG_OBJCNT_INC(assoc); pr_debug!("Created asoc %p\n", asoc); asoc
}

pub unsafe fn sctp_association_free(asoc: *mut sctp_association) {
    let sk = (*asoc).base.sk; if !list_empty(&(*asoc).asocs) { list_del(&mut (*asoc).asocs); if sctp_style(sk, TCP) && sctp_sstate(sk, LISTENING) { sk_acceptq_removed(sk); } }
    (*asoc).base.dead = true; sctp_outq_free(&mut (*asoc).outqueue); sctp_ulpq_free(&mut (*asoc).ulpq); sctp_inq_free(&mut (*asoc).base.inqueue); sctp_tsnmap_free(&mut (*asoc).peer.tsn_map); sctp_stream_free(&mut (*asoc).stream);
    if !(*asoc).strreset_chunk.is_null() { sctp_chunk_free((*asoc).strreset_chunk); }
    sctp_bind_addr_free(&mut (*asoc).base.bind_addr);
    for i in SCTP_EVENT_TIMEOUT_NONE..SCTP_NUM_TIMEOUT_TYPES { if timer_delete(&mut (*asoc).timers[i]) != 0 { sctp_association_put(asoc); } }
    kfree((*asoc).peer.cookie); kfree((*asoc).peer.peer_random); kfree((*asoc).peer.peer_chunks); kfree((*asoc).peer.peer_hmacs);
    let mut pos = (*asoc).peer.transport_addr_list.next; while pos != &mut (*asoc).peer.transport_addr_list { let next = (*pos).next; let t = list_entry(pos, sctp_transport, transports); list_del_rcu(pos); sctp_unhash_transport(t); sctp_transport_free(t); pos = next; }
    (*asoc).peer.transport_count = 0; sctp_asconf_queue_teardown(asoc); kfree((*asoc).asconf_addr_del_pending); sctp_auth_destroy_keys(&mut (*asoc).endpoint_shared_keys); sctp_auth_key_put((*asoc).asoc_shared_key); sctp_association_put(asoc);
}

unsafe fn sctp_association_destroy(asoc: *mut sctp_association) { if !(*asoc).base.dead { WARN!(1, "Attempt to destroy undead association %p!\n", asoc); return; } sctp_endpoint_put((*asoc).ep); sock_put((*asoc).base.sk); if (*asoc).assoc_id != 0 { spin_lock_bh(&mut sctp_assocs_id_lock); idr_remove(&mut sctp_assocs_id, (*asoc).assoc_id); spin_unlock_bh(&mut sctp_assocs_id_lock); } WARN_ON(atomic_read(&(*asoc).rmem_alloc)); kfree_rcu(asoc, rcu); SCTP_DBG_OBJCNT_DEC(assoc); }
pub unsafe fn sctp_association_hold(a: *mut sctp_association) { refcount_inc(&mut (*a).base.refcnt); }
pub unsafe fn sctp_association_put(a: *mut sctp_association) { if refcount_dec_and_test(&mut (*a).base.refcnt) { sctp_association_destroy(a); } }

pub unsafe fn sctp_assoc_set_primary(a: *mut sctp_association, t: *mut sctp_transport) { let changeover = (!(*a).peer.primary_path.is_null() && (*a).peer.primary_path != t) as i32; (*a).peer.primary_path=t; sctp_ulpevent_notify_peer_addr_change(t,SCTP_ADDR_MADE_PRIM,0); memcpy(&mut (*a).peer.primary_addr as *mut _ as *mut _, &(*t).ipaddr as *const _ as *const _, size_of::<sctp_addr>()); if (*t).state==SCTP_ACTIVE || (*t).state==SCTP_UNKNOWN { (*a).peer.active_path=t; } if (*a).outqueue.outstanding_bytes==0 && (*a).outqueue.out_qlen==0{return;} if (*t).cacc.changeover_active { (*t).cacc.cycling_changeover=changeover; } (*t).cacc.changeover_active=changeover; (*t).cacc.next_tsn_at_change=(*a).next_tsn; }

pub unsafe fn sctp_association_get_next_tsn(a: *mut sctp_association) -> u32 { let r=(*a).next_tsn; (*a).next_tsn=(*a).next_tsn.wrapping_add(1); (*a).unack_data=(*a).unack_data.wrapping_add(1); r }
pub unsafe fn sctp_cmp_addr_exact(a:*const sctp_addr,b:*const sctp_addr)->i32 { let af=sctp_get_af_specific((*a).sa.sa_family); if af.is_null(){0}else{((*af).cmp_addr)(a,b)} }
pub unsafe fn sctp_get_ecne_prepend(a:*mut sctp_association)->*mut sctp_chunk { if !(*a).need_ecne {std::ptr::null_mut()} else {sctp_make_ecne(a,(*a).last_ecne_tsn)} }

pub unsafe fn sctp_assoc_lookup_paddr(a:*const sctp_association,address:*const sctp_addr)->*mut sctp_transport { let mut p=(*a).peer.transport_addr_list.next; while p != &(*a).peer.transport_addr_list as *const _ as *mut _ { let t=list_entry(p,sctp_transport,transports); if sctp_cmp_addr_exact(address,&t.ipaddr)!=0{return t;} p=(*p).next;} std::ptr::null_mut() }

pub unsafe fn sctp_assoc_choose_alter_transport(a:*mut sctp_association,last:*mut sctp_transport)->*mut sctp_transport { if last.is_null(){(*a).peer.active_path}else{if last==(*a).peer.retran_path{sctp_assoc_update_retran_path(a);}(*a).peer.retran_path} }
pub unsafe fn sctp_assoc_update_frag_point(a:*mut sctp_association){let mut f=sctp_mtu_payload(sctp_sk((*a).base.sk),(*a).pathmtu,sctp_datachk_len(&(*a).stream));if (*a).user_frag!=0{f=min_t(f,(*a).user_frag);}f=min_t(f,SCTP_MAX_CHUNK_LEN-sctp_datachk_len(&(*a).stream));(*a).frag_point=SCTP_TRUNC4(f);}
pub unsafe fn sctp_assoc_set_pmtu(a:*mut sctp_association,p:u32){if (*a).pathmtu!=p{(*a).pathmtu=p;sctp_assoc_update_frag_point(a);}pr_debug!("%s: asoc:%p, pmtu:%d, frag_point:%d\n",__func__,a,(*a).pathmtu,(*a).frag_point);}

pub unsafe fn sctp_assoc_rwnd_increase(a:*mut sctp_association,len:u32){if (*a).rwnd_over>=len{(*a).rwnd_over-=len;}else{(*a).rwnd+=len-(*a).rwnd_over;(*a).rwnd_over=0;}if (*a).rwnd_press!=0{let c=min_t((*a).pathmtu,(*a).rwnd_press);(*a).rwnd+=c;(*a).rwnd_press-=c;}if sctp_peer_needs_update(a){(*a).a_rwnd=(*a).rwnd;let sack=sctp_make_sack(a);if sack.is_null(){return;}(*a).peer.sack_needed=0;sctp_outq_tail(&mut (*a).outqueue,sack,GFP_ATOMIC);if timer_delete(&mut (*a).timers[SCTP_EVENT_TIMEOUT_SACK])!=0{sctp_association_put(a);}}}
unsafe fn sctp_peer_needs_update(a:*mut sctp_association)->bool{match (*a).state{SCTP_STATE_ESTABLISHED|SCTP_STATE_SHUTDOWN_PENDING|SCTP_STATE_SHUTDOWN_RECEIVED|SCTP_STATE_SHUTDOWN_SENT=>(*a).rwnd>(*a).a_rwnd&&(*a).rwnd-(*a).a_rwnd>=max_t((*a).base.sk.sk_rcvbuf>>(*(*a).base.net).sctp.rwnd_upd_shift,(*a).pathmtu),_= >false}}
pub unsafe fn sctp_assoc_rwnd_decrease(a:*mut sctp_association,len:u32){let rx=if (*a).ep.rcvbuf_policy{atomic_read(&(*a).rmem_alloc)}else{atomic_read(&(*a).base.sk.sk_rmem_alloc)};let over=rx>=(*a).base.sk.sk_rcvbuf;if (*a).rwnd>=len{(*a).rwnd-=len;if over{(*a).rwnd_press+=(*a).rwnd;(*a).rwnd=0;}}else{(*a).rwnd_over+=len-(*a).rwnd;(*a).rwnd=0;}}

pub unsafe fn sctp_assoc_clean_asconf_ack_cache(a:*const sctp_association){let mut p=(*a).asconf_ack_list.next;while p!=&(*a).asconf_ack_list as *const _ as *mut _{let n=(*p).next;let x=list_entry(p,sctp_chunk,transmitted_list);if (*x).subh.addip_hdr.serial==htonl((*a).peer.addip_serial){break;}list_del_init(&mut (*x).transmitted_list);sctp_chunk_free(x);p=n;}}
pub unsafe fn sctp_assoc_lookup_asconf_ack(a:*const sctp_association,serial:be32)->*mut sctp_chunk{let mut p=(*a).asconf_ack_list.next;while p!=&(*a).asconf_ack_list as *const _ as *mut _{let x=list_entry(p,sctp_chunk,transmitted_list);if !sctp_chunk_pending(x)&&(*x).subh.addip_hdr.serial==serial{sctp_chunk_hold(x);return x;}p=(*p).next;}std::ptr::null_mut()}
pub unsafe fn sctp_asconf_queue_teardown(a:*mut sctp_association){sctp_assoc_free_asconf_acks(a);sctp_assoc_free_asconf_queue(a);if !(*a).addip_last_asconf.is_null(){sctp_chunk_free((*a).addip_last_asconf);(*a).addip_last_asconf=std::ptr::null_mut();}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
