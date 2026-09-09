// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_app.c: Application module support for IPVS
 *
 * Rust translation of the implementation source. Kernel and IPVS types,
 * constants, functions, and macros referenced below are supplied externally.
 */

// The original source exports register_ip_vs_app, unregister_ip_vs_app, and
// register_ip_vs_app_inc, and conditionally builds the procfs sequence code.

static mut __IP_VS_APP_MUTEX: Mutex = Mutex::new();

/* Get an ip_vs_app object. */
#[inline]
unsafe fn ip_vs_app_get(app: *mut ip_vs_app) -> c_int {
    try_module_get((*app).module)
}

#[inline]
unsafe fn ip_vs_app_put(app: *mut ip_vs_app) {
    module_put((*app).module);
}

unsafe fn ip_vs_app_inc_destroy(inc: *mut ip_vs_app) {
    kfree((*inc).timeout_table);
    kfree(inc);
}

unsafe fn ip_vs_app_inc_rcu_free(head: *mut rcu_head) {
    let inc = container_of!(head, ip_vs_app, rcu_head);
    ip_vs_app_inc_destroy(inc);
}

/* Allocate/initialize app incarnation and register it in proto apps. */
unsafe fn ip_vs_app_inc_new(ipvs: *mut netns_ipvs, app: *mut ip_vs_app,
                            proto: u16, port: u16) -> c_int {
    let pp = ip_vs_proto_get(proto);
    if pp.is_null() { return -EPROTONOSUPPORT; }
    if (*pp).unregister_app.is_none() { return -EOPNOTSUPP; }

    let inc = kmemdup(app, core::mem::size_of::<ip_vs_app>(), GFP_KERNEL);
    if inc.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*inc).p_list);
    INIT_LIST_HEAD!(&mut (*inc).incs_list);
    (*inc).app = app;
    (*inc).port = htons(port);
    atomic_set!(&mut (*inc).usecnt, 0);

    if !(*app).timeouts.is_null() {
        (*inc).timeout_table = ip_vs_create_timeout_table((*app).timeouts,
                                                           (*app).timeouts_size);
        if (*inc).timeout_table.is_null() {
            ip_vs_app_inc_destroy(inc);
            return -ENOMEM;
        }
    }

    let ret = ((*pp).register_app.unwrap())(ipvs, inc);
    if ret != 0 {
        ip_vs_app_inc_destroy(inc);
        return ret;
    }
    list_add!(&mut (*inc).a_list, &mut (*app).incs_list);
    IP_VS_DBG!(9, "%s App %s:%u registered\n", (*pp).name, (*inc).name,
               ntohs((*inc).port));
    0
}

/* Release app incarnation. */
unsafe fn ip_vs_app_inc_release(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) {
    let pp = ip_vs_proto_get((*inc).protocol);
    if pp.is_null() { return; }
    if let Some(f) = (*pp).unregister_app { f(ipvs, inc); }
    IP_VS_DBG!(9, "%s App %s:%u unregistered\n", (*pp).name, (*inc).name,
               ntohs((*inc).port));
    list_del!(&mut (*inc).a_list);
    call_rcu!(&mut (*inc).rcu_head, ip_vs_app_inc_rcu_free);
}

pub unsafe fn ip_vs_app_inc_get(inc: *mut ip_vs_app) -> c_int {
    let result = ip_vs_app_get((*inc).app);
    if result != 0 { atomic_inc!(&mut (*inc).usecnt); }
    result
}

pub unsafe fn ip_vs_app_inc_put(inc: *mut ip_vs_app) {
    atomic_dec!(&mut (*inc).usecnt);
    ip_vs_app_put((*inc).app);
}

pub unsafe fn register_ip_vs_app_inc(ipvs: *mut netns_ipvs, app: *mut ip_vs_app,
                                     proto: u16, port: u16) -> c_int {
    mutex_lock!(&mut __IP_VS_APP_MUTEX);
    let result = ip_vs_app_inc_new(ipvs, app, proto, port);
    mutex_unlock!(&mut __IP_VS_APP_MUTEX);
    result
}

pub unsafe fn register_ip_vs_app(ipvs: *mut netns_ipvs, app: *mut ip_vs_app) -> *mut ip_vs_app {
    mutex_lock!(&mut __IP_VS_APP_MUTEX);
    if !ip_vs_use_count_inc() { mutex_unlock!(&mut __IP_VS_APP_MUTEX); return ERR_PTR!(-ENOENT); }
    let mut a: *mut ip_vs_app;
    list_for_each_entry!(a, (*ipvs).app_list, a_list) {
        if strcmp((*app).name, (*a).name) == 0 {
            ip_vs_use_count_dec(); mutex_unlock!(&mut __IP_VS_APP_MUTEX); return ERR_PTR!(-EEXIST);
        }
    }
    a = kmemdup(app, core::mem::size_of::<ip_vs_app>(), GFP_KERNEL);
    if a.is_null() { ip_vs_use_count_dec(); mutex_unlock!(&mut __IP_VS_APP_MUTEX); return ERR_PTR!(-ENOMEM); }
    INIT_LIST_HEAD!(&mut (*a).incs_list);
    list_add!(&mut (*a).a_list, &mut (*ipvs).app_list);
    mutex_unlock!(&mut __IP_VS_APP_MUTEX);
    a
}

pub unsafe fn unregister_ip_vs_app(ipvs: *mut netns_ipvs, app: *mut ip_vs_app) {
    mutex_lock!(&mut __IP_VS_APP_MUTEX);
    let mut a: *mut ip_vs_app; let mut anxt: *mut ip_vs_app;
    list_for_each_entry_safe!(a, anxt, (*ipvs).app_list, a_list) {
        if !app.is_null() && strcmp((*app).name, (*a).name) != 0 { continue; }
        let mut inc: *mut ip_vs_app; let mut nxt: *mut ip_vs_app;
        list_for_each_entry_safe!(inc, nxt, (*a).incs_list, a_list) { ip_vs_app_inc_release(ipvs, inc); }
        list_del!(&mut (*a).a_list); kfree(a); ip_vs_use_count_dec();
    }
    mutex_unlock!(&mut __IP_VS_APP_MUTEX);
}

pub unsafe fn ip_vs_bind_app(cp: *mut ip_vs_conn, pp: *mut ip_vs_protocol) -> c_int {
    ((*pp).app_conn_bind.unwrap())(cp)
}

pub unsafe fn ip_vs_unbind_app(cp: *mut ip_vs_conn) {
    let inc = (*cp).app;
    if inc.is_null() { return; }
    if let Some(f) = (*inc).unbind_conn { f(inc, cp); }
    if let Some(f) = (*inc).done_conn { f(inc, cp); }
    ip_vs_app_inc_put(inc); (*cp).app = core::ptr::null_mut();
}

#[inline] unsafe fn vs_fix_seq(vseq: *const ip_vs_seq, th: *mut tcphdr) {
    let seq = ntohl((*th).seq);
    if (*vseq).delta != 0 || (*vseq).previous_delta != 0 {
        if after(seq, (*vseq).init_seq) { (*th).seq = htonl(seq.wrapping_add((*vseq).delta as u32)); }
        else { (*th).seq = htonl(seq.wrapping_add((*vseq).previous_delta as u32)); }
    }
}

#[inline] unsafe fn vs_fix_ack_seq(vseq: *const ip_vs_seq, th: *mut tcphdr) {
    let ack_seq = ntohl((*th).ack_seq);
    if (*vseq).delta != 0 || (*vseq).previous_delta != 0 {
        if after(ack_seq, (*vseq).init_seq.wrapping_add((*vseq).delta as u32)) { (*th).ack_seq = htonl(ack_seq.wrapping_sub((*vseq).delta as u32)); }
        else { (*th).ack_seq = htonl(ack_seq.wrapping_sub((*vseq).previous_delta as u32)); }
    }
}

#[inline] unsafe fn vs_seq_update(cp: *mut ip_vs_conn, vseq: *mut ip_vs_seq, flag: u32, seq: u32, diff: c_int) {
    spin_lock_bh!(&mut (*cp).lock);
    if (*cp).flags & flag == 0 || after(seq, (*vseq).init_seq) {
        (*vseq).previous_delta = (*vseq).delta; (*vseq).delta += diff; (*vseq).init_seq = seq; (*cp).flags |= flag;
    }
    spin_unlock_bh!(&mut (*cp).lock);
}

#[inline] unsafe fn app_tcp_pkt_out(cp: *mut ip_vs_conn, skb: *mut sk_buff, app: *mut ip_vs_app, ipvsh: *mut ip_vs_iphdr) -> c_int {
    if skb_ensure_writable(skb, (*ipvsh).len + core::mem::size_of::<tcphdr>()) != 0 { return 0; }
    let th = ((*skb).data.add((*ipvsh).len)) as *mut tcphdr; let seq = ntohl((*th).seq);
    if (*cp).flags & IP_VS_CONN_F_OUT_SEQ != 0 { vs_fix_seq(&(*cp).out_seq, th); }
    if (*cp).flags & IP_VS_CONN_F_IN_SEQ != 0 { vs_fix_ack_seq(&(*cp).in_seq, th); }
    let f = match (*app).pkt_out { Some(f) => f, None => return 1 }; let mut diff = 0;
    if f(app, cp, skb, &mut diff, ipvsh) == 0 { return 0; }
    if diff != 0 { vs_seq_update(cp, &mut (*cp).out_seq, IP_VS_CONN_F_OUT_SEQ, seq, diff); } 1
}

pub unsafe fn ip_vs_app_pkt_out(cp: *mut ip_vs_conn, skb: *mut sk_buff, ipvsh: *mut ip_vs_iphdr) -> c_int {
    let app = (*cp).app; if app.is_null() { return 1; }
    if (*cp).protocol == IPPROTO_TCP { return app_tcp_pkt_out(cp, skb, app, ipvsh); }
    match (*app).pkt_out { Some(f) => f(app, cp, skb, core::ptr::null_mut(), ipvsh), None => 1 }
}

#[inline] unsafe fn app_tcp_pkt_in(cp: *mut ip_vs_conn, skb: *mut sk_buff, app: *mut ip_vs_app, ipvsh: *mut ip_vs_iphdr) -> c_int {
    if skb_ensure_writable(skb, (*ipvsh).len + core::mem::size_of::<tcphdr>()) != 0 { return 0; }
    let th = ((*skb).data.add((*ipvsh).len)) as *mut tcphdr; let seq = ntohl((*th).seq);
    if (*cp).flags & IP_VS_CONN_F_IN_SEQ != 0 { vs_fix_seq(&(*cp).in_seq, th); }
    if (*cp).flags & IP_VS_CONN_F_OUT_SEQ != 0 { vs_fix_ack_seq(&(*cp).out_seq, th); }
    let f = match (*app).pkt_in { Some(f) => f, None => return 1 }; let mut diff = 0;
    if f(app, cp, skb, &mut diff, ipvsh) == 0 { return 0; }
    if diff != 0 { vs_seq_update(cp, &mut (*cp).in_seq, IP_VS_CONN_F_IN_SEQ, seq, diff); } 1
}

pub unsafe fn ip_vs_app_pkt_in(cp: *mut ip_vs_conn, skb: *mut sk_buff, ipvsh: *mut ip_vs_iphdr) -> c_int {
    let app = (*cp).app; if app.is_null() { return 1; }
    if (*cp).protocol == IPPROTO_TCP { return app_tcp_pkt_in(cp, skb, app, ipvsh); }
    match (*app).pkt_in { Some(f) => f(app, cp, skb, core::ptr::null_mut(), ipvsh), None => 1 }
}

pub unsafe fn ip_vs_app_net_init(ipvs: *mut netns_ipvs) -> c_int {
    INIT_LIST_HEAD!(&mut (*ipvs).app_list);
    // CONFIG_PROC_FS: proc_create_net("ip_vs_app", ...) and sequence ops.
    0
}

pub unsafe fn ip_vs_app_net_cleanup(ipvs: *mut netns_ipvs) {
    unregister_ip_vs_app(ipvs, core::ptr::null_mut());
    // CONFIG_PROC_FS: remove_proc_entry("ip_vs_app", ...).
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
