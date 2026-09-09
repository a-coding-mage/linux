// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel transport implementation; direct low-level translation of transport.c. */

/* Kernel headers provide all referenced types, constants, and functions. */

unsafe fn sctp_transport_init(net: *mut net, peer: *mut sctp_transport,
                              addr: *const sctp_addr, _gfp: gfp_t) {
    (*peer).af_specific = sctp_get_af_specific((*addr).sa.sa_family);
    memcpy(&mut (*peer).ipaddr as *mut _, addr as *const _, (*peer).af_specific.sockaddr_len);
    memset(&mut (*peer).saddr as *mut _, 0, core::mem::size_of::<sctp_addr>());
    (*peer).sack_generation = 0;
    (*peer).rto = msecs_to_jiffies((*net).sctp.rto_initial);
    (*peer).last_time_heard = 0;
    (*peer).last_time_ecne_reduced = jiffies;
    (*peer).param_flags = SPP_HB_DISABLE | SPP_PMTUD_ENABLE | SPP_SACKDELAY_ENABLE;
    (*peer).pathmaxrxt = (*net).sctp.max_retrans_path;
    (*peer).pf_retrans = (*net).sctp.pf_retrans;
    INIT_LIST_HEAD(&mut (*peer).transmitted);
    INIT_LIST_HEAD(&mut (*peer).send_ready);
    INIT_LIST_HEAD(&mut (*peer).transports);
    timer_setup(&mut (*peer).T3_rtx_timer, sctp_generate_t3_rtx_event, 0);
    timer_setup(&mut (*peer).hb_timer, sctp_generate_heartbeat_event, 0);
    timer_setup(&mut (*peer).reconf_timer, sctp_generate_reconf_event, 0);
    timer_setup(&mut (*peer).probe_timer, sctp_generate_probe_event, 0);
    timer_setup(&mut (*peer).proto_unreach_timer, sctp_generate_proto_unreach_event, 0);
    get_random_bytes(&mut (*peer).hb_nonce as *mut _, core::mem::size_of_val(&(*peer).hb_nonce));
    refcount_set(&mut (*peer).refcnt, 1);
}

pub unsafe fn sctp_transport_new(net: *mut net, addr: *const sctp_addr, gfp: gfp_t) -> *mut sctp_transport {
    let transport = kzalloc_obj::<sctp_transport>(gfp);
    if transport.is_null() { return core::ptr::null_mut(); }
    sctp_transport_init(net, transport, addr, gfp);
    SCTP_DBG_OBJCNT_INC(transport);
    transport
}

pub unsafe fn sctp_transport_free(transport: *mut sctp_transport) {
    (*transport).dead = 1;
    if timer_delete(&mut (*transport).hb_timer) != 0 { sctp_transport_put(transport); }
    if timer_delete(&mut (*transport).T3_rtx_timer) != 0 { sctp_transport_put(transport); }
    if timer_delete(&mut (*transport).reconf_timer) != 0 { sctp_transport_put(transport); }
    if timer_delete(&mut (*transport).probe_timer) != 0 { sctp_transport_put(transport); }
    if timer_delete(&mut (*transport).proto_unreach_timer) != 0 { sctp_transport_put(transport); }
    sctp_transport_put(transport);
}

unsafe fn sctp_transport_destroy_rcu(head: *mut rcu_head) {
    let transport = container_of!(head, sctp_transport, rcu);
    dst_release((*transport).dst);
    kfree(transport);
    SCTP_DBG_OBJCNT_DEC(transport);
}

unsafe fn sctp_transport_destroy(transport: *mut sctp_transport) {
    if unlikely(refcount_read(&(*transport).refcnt) != 0) {
        WARN!(1, "Attempt to destroy undead transport %p!\n", transport); return;
    }
    sctp_packet_free(&mut (*transport).packet);
    if !(*transport).asoc.is_null() { sctp_association_put((*transport).asoc); }
    call_rcu(&mut (*transport).rcu, sctp_transport_destroy_rcu);
}

pub unsafe fn sctp_transport_reset_t3_rtx(t: *mut sctp_transport) {
    if !timer_pending(&(*t).T3_rtx_timer) && mod_timer(&mut (*t).T3_rtx_timer, jiffies + (*t).rto) == 0 { sctp_transport_hold(t); }
}
pub unsafe fn sctp_transport_reset_hb_timer(t: *mut sctp_transport) {
    let expires = jiffies + sctp_transport_timeout(t);
    if mod_timer(&mut (*t).hb_timer, expires + get_random_u32_below((*t).rto)) == 0 { sctp_transport_hold(t); }
}
pub unsafe fn sctp_transport_reset_reconf_timer(t: *mut sctp_transport) {
    if !timer_pending(&(*t).reconf_timer) && mod_timer(&mut (*t).reconf_timer, jiffies + (*t).rto) == 0 { sctp_transport_hold(t); }
}
pub unsafe fn sctp_transport_reset_probe_timer(t: *mut sctp_transport) {
    if mod_timer(&mut (*t).probe_timer, jiffies + (*t).probe_interval) == 0 { sctp_transport_hold(t); }
}
pub unsafe fn sctp_transport_reset_raise_timer(t: *mut sctp_transport) {
    if mod_timer(&mut (*t).probe_timer, jiffies + (*t).probe_interval * 30) == 0 { sctp_transport_hold(t); }
}

pub unsafe fn sctp_transport_set_owner(t: *mut sctp_transport, a: *mut sctp_association) { (*t).asoc = a; sctp_association_hold(a); }

pub unsafe fn sctp_transport_hold(t: *mut sctp_transport) -> i32 { refcount_inc_not_zero(&mut (*t).refcnt) }
pub unsafe fn sctp_transport_put(t: *mut sctp_transport) { if refcount_dec_and_test(&mut (*t).refcnt) { sctp_transport_destroy(t); } }

pub unsafe fn sctp_transport_burst_limited(t: *mut sctp_transport) {
    let a = (*t).asoc; let old = (*t).cwnd;
    if (*t).burst_limited != 0 || (*a).max_burst == 0 { return; }
    let max_bytes = (*t).flight_size + (*a).max_burst * (*a).pathmtu;
    if max_bytes < old { (*t).cwnd = max_bytes; (*t).burst_limited = old; }
}
pub unsafe fn sctp_transport_burst_reset(t: *mut sctp_transport) { if (*t).burst_limited != 0 { (*t).cwnd = (*t).burst_limited; (*t).burst_limited = 0; } }

pub unsafe fn sctp_transport_timeout(t: *mut sctp_transport) -> ulong {
    let mut timeout = (*t).rto >> 1;
    if (*t).state != SCTP_UNCONFIRMED && (*t).state != SCTP_PF { timeout += (*t).hbinterval; }
    max_t!(ulong, timeout, HZ / 5)
}

pub unsafe fn sctp_transport_dst_release(t: *mut sctp_transport) { dst_release((*t).dst); (*t).dst = core::ptr::null_mut(); (*t).dst_pending_confirm = 0; }
pub unsafe fn sctp_transport_dst_confirm(t: *mut sctp_transport) { (*t).dst_pending_confirm = 1; }

pub unsafe fn sctp_transport_pmtu(t: *mut sctp_transport, sk: *mut sock) {
    if (*t).dst.is_null() || READ_ONCE!((*(*t).dst).obsolete) { sctp_transport_dst_release(t); (*t).af_specific.get_dst(t, &mut (*t).saddr, &mut (*t).fl, sk); }
    if (*t).param_flags & SPP_PMTUD_DISABLE != 0 { let a = (*t).asoc; if (*t).pathmtu == 0 && !a.is_null() { (*t).pathmtu = (*a).pathmtu; } if (*t).pathmtu != 0 { return; } }
    (*t).pathmtu = if !(*t).dst.is_null() { sctp_dst_mtu((*t).dst) } else { SCTP_DEFAULT_MAXSEGMENT };
    sctp_transport_pl_update(t);
}

pub unsafe fn sctp_transport_reset(t: *mut sctp_transport) {
    let a = (*t).asoc;
    (*t).cwnd = min!(4 * (*a).pathmtu, max_t!(__u32, 2 * (*a).pathmtu, 4380));
    (*t).burst_limited = 0; (*t).ssthresh = (*a).peer.i.a_rwnd; (*t).rto = (*a).rto_initial; sctp_max_rto(a, t);
    (*t).rtt = 0; (*t).srtt = 0; (*t).rttvar = 0; (*t).partial_bytes_acked = 0; (*t).flight_size = 0;
    (*t).error_count = 0; (*t).rto_pending = 0; (*t).hb_sent = 0;
    (*t).cacc.changeover_active = 0; (*t).cacc.cycling_changeover = 0; (*t).cacc.next_tsn_at_change = 0; (*t).cacc.cacc_saw_newack = 0;
}

pub unsafe fn sctp_transport_immediate_rtx(t: *mut sctp_transport) {
    if timer_delete(&mut (*t).T3_rtx_timer) != 0 { sctp_transport_put(t); }
    sctp_retransmit(&mut (*(*t).asoc).outqueue, t, SCTP_RTXR_T3_RTX);
    if !timer_pending(&(*t).T3_rtx_timer) && mod_timer(&mut (*t).T3_rtx_timer, jiffies + (*t).rto) == 0 { sctp_transport_hold(t); }
}

pub unsafe fn sctp_transport_route(t: *mut sctp_transport, saddr: *mut sctp_addr, opt: *mut sctp_sock) {
    let a = (*t).asoc; let af = (*t).af_specific;
    sctp_transport_dst_release(t); af.get_dst(t, saddr, &mut (*t).fl, sctp_opt2sk(opt));
    if !saddr.is_null() { memcpy(&mut (*t).saddr as *mut _, saddr as *const _, core::mem::size_of::<sctp_addr>()); } else { af.get_saddr(opt, t, &mut (*t).fl); }
    sctp_transport_pmtu(t, sctp_opt2sk(opt));
    if !(*t).dst.is_null() && !a.is_null() && ((*a).peer.primary_path.is_null() || t == (*a).peer.active_path) { opt.pf.to_sk_saddr(&(*t).saddr, (*a).base.sk); }
}

pub unsafe fn sctp_transport_update_pmtu(t: *mut sctp_transport, mut pmtu: u32) -> bool {
    if unlikely(pmtu < SCTP_DEFAULT_MINSEGMENT) { pr_warn_ratelimited!("Reported pmtu too low\n"); pmtu = SCTP_DEFAULT_MINSEGMENT; }
    pmtu = SCTP_TRUNC4!(pmtu);
    if sctp_transport_pl_enabled(t) { return sctp_transport_pl_toobig(t, pmtu - sctp_transport_pl_hlen(t)); }
    let mut dst = sctp_transport_dst_check(t);
    if !dst.is_null() { (*dst).ops.update_pmtu(dst, (*t).asoc.base.sk, core::ptr::null_mut(), pmtu, true); dst = sctp_transport_dst_check(t); }
    if dst.is_null() { (*t).af_specific.get_dst(t, &mut (*t).saddr, &mut (*t).fl, (*t).asoc.base.sk); dst = (*t).dst; }
    let change = if !dst.is_null() { pmtu = sctp_dst_mtu(dst); (*t).pathmtu != pmtu } else { true }; (*t).pathmtu = pmtu; change
}

pub unsafe fn sctp_transport_pl_send(t: *mut sctp_transport) {
    if (*t).pl.probe_count < SCTP_MAX_PROBES { (*t).pl.probe_count += 1; return; }
    (*t).pl.probe_count = 0;
    if ((*t).pl.state == SCTP_PL_BASE && (*t).pl.probe_size == SCTP_BASE_PLPMTU) ||
       ((*t).pl.state == SCTP_PL_SEARCH && (*t).pl.pmtu == (*t).pl.probe_size) ||
       ((*t).pl.state == SCTP_PL_COMPLETE && (*t).pl.pmtu == (*t).pl.probe_size) {
        (*t).pl.state = if (*t).pl.state == SCTP_PL_SEARCH { SCTP_PL_BASE } else { SCTP_PL_ERROR };
        (*t).pl.probe_size = SCTP_BASE_PLPMTU; (*t).pl.probe_high = 0; (*t).pl.pmtu = SCTP_BASE_PLPMTU;
        (*t).pathmtu = (*t).pl.pmtu + sctp_transport_pl_hlen(t); sctp_assoc_sync_pmtu((*t).asoc);
    }
}
pub unsafe fn sctp_transport_pl_recv(t: *mut sctp_transport) -> bool {
    (*t).pl.pmtu = (*t).pl.probe_size; (*t).pl.probe_count = 0;
    if (*t).pl.state == SCTP_PL_BASE || (*t).pl.state == SCTP_PL_ERROR { (*t).pl.state = SCTP_PL_SEARCH; (*t).pl.probe_size += SCTP_PL_BIG_STEP; }
    else if (*t).pl.state == SCTP_PL_SEARCH { (*t).pl.probe_size += SCTP_PL_MIN_STEP; if (*t).pl.probe_high != 0 && (*t).pl.probe_size >= (*t).pl.probe_high { (*t).pl.state = SCTP_PL_COMPLETE; (*t).pl.probe_size = (*t).pl.pmtu; sctp_transport_reset_raise_timer(t); } }
    else { (*t).pl.state = SCTP_PL_SEARCH; (*t).pl.probe_size = min!((*t).pl.probe_size + SCTP_PL_MIN_STEP, SCTP_MAX_PLPMTU); }
    (*t).pl.state == SCTP_PL_COMPLETE
}

pub unsafe fn sctp_transport_update_rto(t: *mut sctp_transport, rtt: u32) {
    if (*t).rttvar != 0 || (*t).srtt != 0 { let n = (*t).asoc.base.net; let b = READ_ONCE!((*n).sctp.rto_beta); let a = READ_ONCE!((*n).sctp.rto_alpha); if b < 32 { (*t).rttvar = (*t).rttvar - ((*t).rttvar >> b) + (abs!((*t).srtt as i64 - rtt as i64) as u32 >> b); } if a < 32 { (*t).srtt = (*t).srtt - ((*t).srtt >> a) + (rtt >> a); } } else { (*t).srtt = rtt; (*t).rttvar = rtt >> 1; }
    if (*t).rttvar == 0 { (*t).rttvar = SCTP_CLOCK_GRANULARITY; } (*t).rto = (*t).srtt + ((*t).rttvar << 2); if (*t).rto < (*t).asoc.rto_min { (*t).rto = (*t).asoc.rto_min; } if (*t).rto > (*t).asoc.rto_max { (*t).rto = (*t).asoc.rto_max; } sctp_max_rto((*t).asoc, t); (*t).rtt = rtt; (*t).rto_pending = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
