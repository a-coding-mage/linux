// SPDX-License-Identifier: GPL-2.0
// Translated from tcp_fastopen.c; kernel dependencies are supplied externally.

pub unsafe fn reqsk_fastopen_remove(sk: *mut sock, req: *mut request_sock, reset: bool) {
    let lsk = (*req).rsk_listener;
    let fastopenq = &mut (*inet_csk(lsk)).icsk_accept_queue.fastopenq;

    rcu_assign_pointer((*tcp_sk(sk)).fastopen_rsk, core::ptr::null_mut());
    spin_lock_bh(&mut fastopenq.lock);
    fastopenq.qlen -= 1;
    (*tcp_rsk(req)).tfo_listener = false;
    if !(*req).sk.is_null() { }
    else {
        if !reset || (*lsk).sk_state != TCP_LISTEN {
            spin_unlock_bh(&mut fastopenq.lock);
            reqsk_put(req);
            return;
        }
        (*req).rsk_timer.expires = jiffies + 60 * HZ;
        if fastopenq.rskq_rst_head.is_null() {
            fastopenq.rskq_rst_head = req;
        } else {
            (*fastopenq.rskq_rst_tail).dl_next = req;
        }
        (*req).dl_next = core::ptr::null_mut();
        fastopenq.rskq_rst_tail = req;
        fastopenq.qlen += 1;
    }
    spin_unlock_bh(&mut fastopenq.lock);
}

pub unsafe fn tcp_fastopen_init_key_once(net: *mut net) {
    let mut key = [0u8; TCP_FASTOPEN_KEY_LENGTH];
    let ctxt;
    rcu_read_lock();
    ctxt = rcu_dereference((*net).ipv4.tcp_fastopen_ctx);
    if !ctxt.is_null() { rcu_read_unlock(); return; }
    rcu_read_unlock();
    get_random_bytes(key.as_mut_ptr() as *mut _, core::mem::size_of_val(&key));
    tcp_fastopen_reset_cipher(net, core::ptr::null_mut(), key.as_mut_ptr() as *mut _, core::ptr::null_mut());
}

unsafe fn tcp_fastopen_ctx_free(head: *mut rcu_head) {
    let ctx = container_of!(head, tcp_fastopen_context, rcu);
    kfree_sensitive(ctx);
}

pub unsafe fn tcp_fastopen_destroy_cipher(sk: *mut sock) {
    let ctx = rcu_dereference_protected((*inet_csk(sk)).icsk_accept_queue.fastopenq.ctx, 1);
    if !ctx.is_null() { call_rcu(&mut (*ctx).rcu, tcp_fastopen_ctx_free); }
}

pub unsafe fn tcp_fastopen_ctx_destroy(net: *mut net) {
    let ctxt = unrcu_pointer(xchg(&mut (*net).ipv4.tcp_fastopen_ctx, RCU_INITIALIZER(core::ptr::null_mut())));
    if !ctxt.is_null() { call_rcu(&mut (*ctxt).rcu, tcp_fastopen_ctx_free); }
}

pub unsafe fn tcp_fastopen_reset_cipher(net: *mut net, sk: *mut sock, primary_key: *mut core::ffi::c_void, backup_key: *mut core::ffi::c_void) -> i32 {
    let ctx = kmalloc_obj::<tcp_fastopen_context>();
    if ctx.is_null() { return -ENOMEM; }
    (*ctx).key[0].key[0] = get_unaligned_le64(primary_key);
    (*ctx).key[0].key[1] = get_unaligned_le64(primary_key.add(8));
    if !backup_key.is_null() {
        (*ctx).key[1].key[0] = get_unaligned_le64(backup_key);
        (*ctx).key[1].key[1] = get_unaligned_le64(backup_key.add(8));
        (*ctx).num = 2;
    } else { (*ctx).num = 1; }
    let octx;
    if !sk.is_null() {
        let q = &mut (*inet_csk(sk)).icsk_accept_queue.fastopenq;
        octx = unrcu_pointer(xchg(&mut q.ctx, RCU_INITIALIZER(ctx)));
    } else { octx = unrcu_pointer(xchg(&mut (*net).ipv4.tcp_fastopen_ctx, RCU_INITIALIZER(ctx))); }
    if !octx.is_null() { call_rcu(&mut (*octx).rcu, tcp_fastopen_ctx_free); }
    0
}

pub unsafe fn tcp_fastopen_get_cipher(net: *mut net, icsk: *mut inet_connection_sock, key: *mut u64) -> i32 {
    let mut n_keys = 0;
    rcu_read_lock();
    let ctx = if !icsk.is_null() { rcu_dereference((*icsk).icsk_accept_queue.fastopenq.ctx) } else { rcu_dereference((*net).ipv4.tcp_fastopen_ctx) };
    if !ctx.is_null() {
        n_keys = tcp_fastopen_context_len(ctx);
        for i in 0..n_keys {
            put_unaligned_le64((*ctx).key[i as usize].key[0], key.add((i * 2) as usize));
            put_unaligned_le64((*ctx).key[i as usize].key[1], key.add((i * 2 + 1) as usize));
        }
    }
    rcu_read_unlock(); n_keys
}

unsafe fn __tcp_fastopen_cookie_gen_cipher(req: *mut request_sock, syn: *mut sk_buff, key: *const siphash_key_t, foc: *mut tcp_fastopen_cookie) -> bool {
    BUILD_BUG_ON!(TCP_FASTOPEN_COOKIE_SIZE != core::mem::size_of::<u64>());
    if (*(*req).rsk_ops).family == AF_INET {
        let iph = ip_hdr(syn);
        (*foc).val[0] = cpu_to_le64(siphash(&(*iph).saddr as *const _ as *const _, core::mem::size_of_val(&(*iph).saddr) + core::mem::size_of_val(&(*iph).daddr), key));
        (*foc).len = TCP_FASTOPEN_COOKIE_SIZE; return true;
    }
    #[cfg(CONFIG_IPV6)]
    if (*(*req).rsk_ops).family == AF_INET6 {
        let ip6h = ipv6_hdr(syn);
        (*foc).val[0] = cpu_to_le64(siphash(&(*ip6h).saddr as *const _ as *const _, core::mem::size_of_val(&(*ip6h).saddr) + core::mem::size_of_val(&(*ip6h).daddr), key));
        (*foc).len = TCP_FASTOPEN_COOKIE_SIZE; return true;
    }
    false
}

unsafe fn tcp_fastopen_cookie_gen(sk: *mut sock, req: *mut request_sock, syn: *mut sk_buff, foc: *mut tcp_fastopen_cookie) {
    rcu_read_lock(); let ctx = tcp_fastopen_get_ctx(sk);
    if !ctx.is_null() { __tcp_fastopen_cookie_gen_cipher(req, syn, &(*ctx).key[0], foc); }
    rcu_read_unlock();
}

pub unsafe fn tcp_fastopen_add_skb(sk: *mut sock, mut skb: *mut sk_buff) {
    let tp = tcp_sk(sk);
    if TCP_SKB_CB(skb).end_seq == (*tp).rcv_nxt { return; }
    skb = skb_clone(skb, GFP_ATOMIC); if skb.is_null() { return; }
    tcp_cleanup_skb(skb); (*tp).segs_in = 0; tcp_segs_in(tp, skb); __skb_pull(skb, tcp_hdrlen(skb));
    sk_forced_mem_schedule(sk, (*skb).truesize); skb_set_owner_r(skb, sk);
    TCP_SKB_CB(skb).seq += 1; TCP_SKB_CB(skb).tcp_flags &= !TCPHDR_SYN;
    (*tp).rcv_nxt = TCP_SKB_CB(skb).end_seq; tcp_add_receive_queue(sk, skb); (*tp).syn_data_acked = 1; (*tp).bytes_received = (*skb).len;
    if TCP_SKB_CB(skb).tcp_flags & TCPHDR_FIN != 0 { tcp_fin(sk); }
}

// The remaining functions preserve the source implementation's externally supplied kernel operations.
pub unsafe fn tcp_fastopen_cookie_gen_check(sk:*mut sock, req:*mut request_sock, syn:*mut sk_buff, orig:*mut tcp_fastopen_cookie, valid_foc:*mut tcp_fastopen_cookie)->i32 { let mut search_foc=tcp_fastopen_cookie{len:-1}; let mut foc=valid_foc; rcu_read_lock(); let ctx=tcp_fastopen_get_ctx(sk); if ctx.is_null(){rcu_read_unlock();return 0;} for i in 0..tcp_fastopen_context_len(ctx){__tcp_fastopen_cookie_gen_cipher(req,syn,&(*ctx).key[i as usize],foc);if tcp_fastopen_cookie_match(foc,orig){rcu_read_unlock();return i+1;}foc=&mut search_foc;} rcu_read_unlock();0 }

pub unsafe fn tcp_fastopen_create_child(sk:*mut sock, skb:*mut sk_buff, req:*mut request_sock)->*mut sock { let mut own=false; let child=(*inet_csk(sk)).icsk_af_ops.syn_recv_sock(sk,skb,req,core::ptr::null_mut(),core::ptr::null_mut(),&mut own,core::ptr::null_mut()); if child.is_null(){return core::ptr::null_mut();} let q=&mut (*inet_csk(sk)).icsk_accept_queue; spin_lock(&mut q.fastopenq.lock);q.fastopenq.qlen+=1;spin_unlock(&mut q.fastopenq.lock);let tp=tcp_sk(child);rcu_assign_pointer((*tp).fastopen_rsk,req);(*tcp_rsk(req)).tfo_listener=true;(*tp).snd_wnd=ntohs((*tcp_hdr(skb)).window);(*tp).max_window=(*tp).snd_wnd;(*req).timeout=tcp_timeout_init(child);tcp_reset_xmit_timer(child,ICSK_TIME_RETRANS,(*req).timeout,false);refcount_set(&mut (*req).rsk_refcnt,2);sk_mark_napi_id_set(child,skb);tcp_init_transfer(child,BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB,skb);(*tp).rcv_nxt=TCP_SKB_CB(skb).seq+1;tcp_fastopen_add_skb(child,skb);(*tcp_rsk(req)).rcv_nxt=(*tp).rcv_nxt;(*tp).rcv_wup=(*tp).rcv_nxt;(*tp).rcv_mwnd_seq=(*tp).rcv_wup+(*tp).rcv_wnd;child }

unsafe fn tcp_fastopen_queue_check(sk:*mut sock)->bool { let q=&mut (*inet_csk(sk)).icsk_accept_queue.fastopenq;let max=READ_ONCE(q.max_qlen);if max==0{return false;}if q.qlen>=max{spin_lock(&mut q.lock);let r=q.rskq_rst_head;if r.is_null()||time_after((*r).rsk_timer.expires,jiffies){spin_unlock(&mut q.lock);return false;}q.rskq_rst_head=(*r).dl_next;q.qlen-=1;spin_unlock(&mut q.lock);reqsk_put(r);}true }
unsafe fn tcp_fastopen_no_cookie(sk:*const sock,dst:*const dst_entry,flag:i32)->bool { (READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_fastopen)&flag)!=0||(*tcp_sk(sk as *mut sock)).fastopen_no_cookie||(!dst.is_null()&&dst_metric(dst,RTAX_FASTOPEN_NO_COOKIE)!=0) }

pub unsafe fn tcp_try_fastopen(sk:*mut sock,skb:*mut sk_buff,req:*mut request_sock,foc:*mut tcp_fastopen_cookie,dst:*const dst_entry)->*mut sock { let syn_data=TCP_SKB_CB(skb).end_seq!=TCP_SKB_CB(skb).seq+1;let tf=READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_fastopen);if (*foc).len==0{NET_INC_STATS(sock_net(sk),LINUX_MIB_TCPFASTOPENCOOKIEREQD);}if tf&TFO_SERVER_ENABLE==0||(!syn_data&&(*foc).len<0)||!tcp_fastopen_queue_check(sk){(*foc).len=-1;return core::ptr::null_mut();}if tcp_fastopen_no_cookie(sk,dst,TFO_SERVER_COOKIE_NOT_REQD){return tcp_fastopen_create_child(sk,skb,req);}let mut valid=tcp_fastopen_cookie{len:-1};if (*foc).len==0{tcp_fastopen_cookie_gen(sk,req,skb,&mut valid);}else if (*foc).len>0{let ret=tcp_fastopen_cookie_gen_check(sk,req,skb,foc,&mut valid);if ret!=0{let child=tcp_fastopen_create_child(sk,skb,req);if !child.is_null(){if ret==2{valid.exp=(*foc).exp;*foc=valid;}else{(*foc).len=-1;}(*tcp_sk(child)).syn_fastopen_child=1;return child;}}}valid.exp=(*foc).exp;*foc=valid;core::ptr::null_mut() }

pub unsafe fn tcp_fastopen_cookie_check(sk:*mut sock,mss:*mut u16,cookie:*mut tcp_fastopen_cookie)->bool { tcp_fastopen_cache_get(sk,mss,cookie);if tcp_fastopen_active_should_disable(sk){(*cookie).len=-1;return false;}let dst=__sk_dst_get(sk);if tcp_fastopen_no_cookie(sk,dst,TFO_CLIENT_NO_COOKIE){(*cookie).len=-1;return true;}if (*cookie).len>0{return true;}(*tcp_sk(sk)).fastopen_client_fail=TFO_COOKIE_UNAVAILABLE;false }
pub unsafe fn tcp_fastopen_defer_connect(sk:*mut sock,err:*mut i32)->bool { let mut cookie=tcp_fastopen_cookie{len:0};let tp=tcp_sk(sk);let mut mss=0;if (*tp).fastopen_connect&&!(*tp).fastopen_req.is_null(){if tcp_fastopen_cookie_check(sk,&mut mss,&mut cookie){inet_set_bit(DEFER_CONNECT,sk);return true;}(*tp).fastopen_req=kzalloc_obj((*tp).fastopen_req,(*sk).sk_allocation);if !(*tp).fastopen_req.is_null(){(*(*tp).fastopen_req).cookie=cookie;}else{*err=-ENOBUFS;}}false }

pub unsafe fn tcp_fastopen_active_disable(sk:*mut sock){let net=sock_net(sk);if READ_ONCE((*net).ipv4.sysctl_tcp_fastopen_blackhole_timeout)==0{return;}WRITE_ONCE((*net).ipv4.tfo_active_disable_stamp,jiffies);smp_mb__before_atomic();atomic_inc(&mut (*net).ipv4.tfo_active_disable_times);NET_INC_STATS(net,LINUX_MIB_TCPFASTOPENBLACKHOLE);}
pub unsafe fn tcp_fastopen_active_should_disable(sk:*mut sock)->bool{let t=READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_fastopen_blackhole_timeout);if t==0{return false;}let n=atomic_read(&(*sock_net(sk)).ipv4.tfo_active_disable_times);if n==0{return false;}smp_rmb();let mult=1<<core::cmp::min(n-1,6);let timeout=READ_ONCE((*sock_net(sk)).ipv4.tfo_active_disable_stamp)+mult*t*HZ;if time_before(jiffies,timeout){true}else{(*tcp_sk(sk)).syn_fastopen_ch=1;false}}
pub unsafe fn tcp_fastopen_active_disable_ofo_check(sk:*mut sock){let tp=tcp_sk(sk);if !(*tp).syn_fastopen{return;}if (*tp).data_segs_in==0{let skb=skb_rb_first(&(*tp).out_of_order_queue);if !skb.is_null()&&skb_rb_next(skb).is_null()&&TCP_SKB_CB(skb).tcp_flags&TCPHDR_FIN!=0{tcp_fastopen_active_disable(sk);}}else if (*tp).syn_fastopen_ch&&atomic_read(&(*sock_net(sk)).ipv4.tfo_active_disable_times)!=0{let dst=__sk_dst_get(sk);let dev=if !dst.is_null(){dst_dev_rcu(dst)}else{core::ptr::null_mut()};if dev.is_null()||(*dev).flags&IFF_LOOPBACK==0{atomic_set(&mut (*sock_net(sk)).ipv4.tfo_active_disable_times,0);}}}
pub unsafe fn tcp_fastopen_active_detect_blackhole(sk:*mut sock,expired:bool){let timeouts=(*inet_csk(sk)).icsk_retransmits;let tp=tcp_sk(sk);if ((*tp).syn_fastopen||(*tp).syn_data||(*tp).syn_data_acked)&&(timeouts==2||(timeouts<2&&expired)){tcp_fastopen_active_disable(sk);NET_INC_STATS(sock_net(sk),LINUX_MIB_TCPFASTOPENACTIVEFAIL);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
