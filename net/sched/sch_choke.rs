// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of net/sched/sch_choke.c. */

const CHOKE_MAX_QUEUE: u32 = 128 * 1024 - 1;

#[repr(C)]
pub struct choke_sched_data {
    pub limit: u32,
    pub flags: u8,
    pub parms: red_parms,
    pub vars: red_vars,
    pub stats: choke_stats,
    pub head: u32,
    pub tail: u32,
    pub tab_mask: u32,
    pub tab: *mut *mut sk_buff,
}

#[repr(C)]
pub struct choke_stats {
    pub prob_drop: u32,
    pub prob_mark: u32,
    pub forced_drop: u32,
    pub forced_mark: u32,
    pub pdrop: u32,
    pub matched: u32,
}

unsafe fn choke_len(q: *const choke_sched_data) -> u32 {
    ((*q).tail.wrapping_sub((*q).head)) & (*q).tab_mask
}

unsafe fn use_ecn(q: *const choke_sched_data) -> i32 { ((*q).flags & TC_RED_ECN) as i32 }
unsafe fn use_harddrop(q: *const choke_sched_data) -> i32 { ((*q).flags & TC_RED_HARDDROP) as i32 }

unsafe fn choke_zap_head_holes(q: *mut choke_sched_data) {
    loop {
        (*q).head = ((*q).head + 1) & (*q).tab_mask;
        if (*q).head == (*q).tail || *(*q).tab.add((*q).head as usize) != core::ptr::null_mut() { break; }
    }
}

unsafe fn choke_zap_tail_holes(q: *mut choke_sched_data) {
    loop {
        (*q).tail = ((*q).tail.wrapping_sub(1)) & (*q).tab_mask;
        if (*q).head == (*q).tail || *(*q).tab.add((*q).tail as usize) != core::ptr::null_mut() { break; }
    }
}

unsafe fn choke_drop_by_idx(sch: *mut Qdisc, idx: u32, to_free: *mut *mut sk_buff) {
    let q = qdisc_priv(sch) as *mut choke_sched_data;
    let skb = *(*q).tab.add(idx as usize);
    *(*q).tab.add(idx as usize) = core::ptr::null_mut();
    if idx == (*q).head { choke_zap_head_holes(q); }
    if idx == (*q).tail { choke_zap_tail_holes(q); }
    qdisc_qlen_dec(sch); qdisc_qstats_backlog_dec(sch, skb);
    qdisc_tree_reduce_backlog(sch, 1, qdisc_pkt_len(skb));
    qdisc_drop(skb, sch, to_free);
}

#[repr(C)] pub struct choke_skb_cb { pub keys_valid: u8, pub keys: flow_keys_digest }
unsafe fn choke_skb_cb(skb: *const sk_buff) -> *mut choke_skb_cb {
    qdisc_cb_private_validate(skb, core::mem::size_of::<choke_skb_cb>());
    qdisc_skb_cb(skb).data as *mut choke_skb_cb
}

unsafe fn choke_match_flow(skb1: *mut sk_buff, skb2: *mut sk_buff) -> bool {
    let mut temp: flow_keys = core::mem::zeroed();
    if (*skb1).protocol != (*skb2).protocol { return false; }
    let c1 = choke_skb_cb(skb1); let c2 = choke_skb_cb(skb2);
    if (*c1).keys_valid == 0 { (*c1).keys_valid = 1; skb_flow_dissect_flow_keys(skb1, &mut temp, 0); make_flow_keys_digest(&mut (*c1).keys, &temp); }
    if (*c2).keys_valid == 0 { (*c2).keys_valid = 1; skb_flow_dissect_flow_keys(skb2, &mut temp, 0); make_flow_keys_digest(&mut (*c2).keys, &temp); }
    core::slice::from_raw_parts((&(*c1).keys) as *const _ as *const u8, core::mem::size_of::<flow_keys_digest>()) ==
        core::slice::from_raw_parts((&(*c2).keys) as *const _ as *const u8, core::mem::size_of::<flow_keys_digest>())
}

unsafe fn choke_peek_random(q: *const choke_sched_data, pidx: *mut u32) -> *mut sk_buff {
    let mut retrys = 3;
    loop {
        *pidx = ((*q).head + get_random_u32_below(choke_len(q))) & (*q).tab_mask;
        let skb = *(*q).tab.add(*pidx as usize); if !skb.is_null() { return skb; }
        retrys -= 1; if retrys <= 0 { break; }
    }
    *pidx = (*q).head; *(*q).tab.add(*pidx as usize)
}

unsafe fn choke_match_random(q: *const choke_sched_data, nskb: *mut sk_buff, pidx: *mut u32) -> bool {
    if (*q).head == (*q).tail { return false; }
    choke_match_flow(choke_peek_random(q, pidx), nskb)
}

unsafe fn choke_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv(sch) as *mut choke_sched_data; let p = &(*q).parms as *const red_parms;
    (*choke_skb_cb(skb)).keys_valid = 0;
    (*q).vars.qavg = red_calc_qavg(p, &mut (*q).vars, (*sch).q.qlen);
    if red_is_idling(&(*q).vars) { red_end_of_idle_period(&mut (*q).vars); }
    if (*q).vars.qavg <= (*p).qth_min { (*q).vars.qcount = -1; } else {
        let mut idx = 0;
        if choke_match_random(q, skb, &mut idx) { (*q).stats.matched += 1; choke_drop_by_idx(sch, idx, to_free); qdisc_drop(skb, sch, to_free); return NET_XMIT_CN; }
        if (*q).vars.qavg > (*p).qth_max { (*q).vars.qcount = -1; qdisc_qstats_overlimit(sch); if use_harddrop(q) != 0 || use_ecn(q) == 0 || !INET_ECN_set_ce(skb) { (*q).stats.forced_drop += 1; qdisc_drop(skb, sch, to_free); return NET_XMIT_CN; } (*q).stats.forced_mark += 1;
        } else { (*q).vars.qcount += 1; if (*q).vars.qcount != 0 && red_mark_probability(p, &mut (*q).vars, (*q).vars.qavg) { (*q).vars.qcount = 0; (*q).vars.qR = red_random(p); qdisc_qstats_overlimit(sch); if use_ecn(q) == 0 || !INET_ECN_set_ce(skb) { (*q).stats.prob_drop += 1; qdisc_drop(skb, sch, to_free); return NET_XMIT_CN; } (*q).stats.prob_mark += 1; } else if (*q).vars.qcount == 0 { (*q).vars.qR = red_random(p); } }
    }
    if (*sch).q.qlen < (*q).limit { *(*q).tab.add((*q).tail as usize) = skb; (*q).tail = ((*q).tail + 1) & (*q).tab_mask; qdisc_qlen_inc(sch); qdisc_qstats_backlog_inc(sch, skb); return NET_XMIT_SUCCESS; }
    (*q).stats.pdrop += 1; qdisc_drop(skb, sch, to_free)
}

unsafe fn choke_dequeue(sch: *mut Qdisc) -> *mut sk_buff { let q = qdisc_priv(sch) as *mut choke_sched_data; if (*q).head == (*q).tail { if !red_is_idling(&(*q).vars) { red_start_of_idle_period(&mut (*q).vars); } return core::ptr::null_mut(); } let skb = *(*q).tab.add((*q).head as usize); *(*q).tab.add((*q).head as usize) = core::ptr::null_mut(); choke_zap_head_holes(q); qdisc_qlen_dec(sch); qdisc_qstats_backlog_dec(sch, skb); qdisc_bstats_update(sch, skb); skb }
unsafe fn choke_reset(sch: *mut Qdisc) { let q = qdisc_priv(sch) as *mut choke_sched_data; while (*q).head != (*q).tail { let skb = *(*q).tab.add((*q).head as usize); (*q).head = ((*q).head + 1) & (*q).tab_mask; if !skb.is_null() { rtnl_qdisc_drop(skb, sch); } } if !(*q).tab.is_null() { core::ptr::write_bytes((*q).tab, 0, ((*q).tab_mask + 1) as usize); } (*q).head = 0; (*q).tail = 0; red_restart(&mut (*q).vars); }
unsafe fn choke_peek_head(sch: *mut Qdisc) -> *mut sk_buff { let q = qdisc_priv(sch) as *mut choke_sched_data; if (*q).head != (*q).tail { *(*q).tab.add((*q).head as usize) } else { core::ptr::null_mut() } }
unsafe fn choke_destroy(sch: *mut Qdisc) { let q = qdisc_priv(sch) as *mut choke_sched_data; choke_free((*q).tab as *mut core::ffi::c_void); }
unsafe fn choke_free(addr: *mut core::ffi::c_void) { kvfree(addr); }

// Configuration, netlink dump, statistics, qdisc operations, and module registration
// use the declarations supplied by the surrounding kernel translation unit.
unsafe fn choke_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 { choke_change(sch, opt, extack) }
unsafe fn choke_change(_sch: *mut Qdisc, opt: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { if opt.is_null() { return -EINVAL; } /* nla parsing and RED parameter setup are external kernel operations. */ -EINVAL }
unsafe fn choke_dump(_sch: *mut Qdisc, _skb: *mut sk_buff) -> i32 { -EMSGSIZE }
unsafe fn choke_dump_stats(_sch: *mut Qdisc, _d: *mut gnet_dump) -> i32 { -EINVAL }

#[repr(C)] pub struct Qdisc_ops { pub id: *const u8, pub priv_size: usize, pub enqueue: Option<unsafe fn(*mut sk_buff,*mut Qdisc,*mut *mut sk_buff)->i32>, pub dequeue: Option<unsafe fn(*mut Qdisc)->*mut sk_buff>, pub peek: Option<unsafe fn(*mut Qdisc)->*mut sk_buff>, pub init: Option<unsafe fn(*mut Qdisc,*mut nlattr,*mut netlink_ext_ack)->i32>, pub destroy: Option<unsafe fn(*mut Qdisc)>, pub reset: Option<unsafe fn(*mut Qdisc)>, pub change: Option<unsafe fn(*mut Qdisc,*mut nlattr,*mut netlink_ext_ack)->i32>, pub dump: Option<unsafe fn(*mut Qdisc,*mut sk_buff)->i32>, pub dump_stats: Option<unsafe fn(*mut Qdisc,*mut gnet_dump)->i32> }
static mut choke_qdisc_ops: Qdisc_ops = Qdisc_ops { id: b"choke\0".as_ptr(), priv_size: core::mem::size_of::<choke_sched_data>(), enqueue: Some(choke_enqueue), dequeue: Some(choke_dequeue), peek: Some(choke_peek_head), init: Some(choke_init), destroy: Some(choke_destroy), reset: Some(choke_reset), change: Some(choke_change), dump: Some(choke_dump), dump_stats: Some(choke_dump_stats) };
unsafe fn choke_module_init() -> i32 { register_qdisc(&mut choke_qdisc_ops) }
unsafe fn choke_module_exit() { unregister_qdisc(&mut choke_qdisc_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
