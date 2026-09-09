// SPDX-License-Identifier: GPL-2.0-or-later
/* DataCenter TCP (DCTCP) congestion control. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// Rust kernel environment.

const DCTCP_MAX_ALPHA: u32 = 1024;

#[repr(C)]
struct dctcp {
    old_delivered: u32,
    old_delivered_ce: u32,
    prior_rcv_nxt: u32,
    dctcp_alpha: u32,
    next_seq: u32,
    ce_state: u32,
    loss_cwnd: u32,
    plb: tcp_plb_state,
}

static mut dctcp_shift_g: u32 = 4; /* g = 1/2^4 */

unsafe extern "C" fn dctcp_shift_g_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    param_set_uint_minmax(val, kp, 0, 10)
}

static dctcp_shift_g_ops: kernel_param_ops = kernel_param_ops {
    set: Some(dctcp_shift_g_set),
    get: Some(param_get_uint),
};

// module_param_cb(dctcp_shift_g, &dctcp_shift_g_ops, &dctcp_shift_g, 0644);
// MODULE_PARM_DESC(dctcp_shift_g, "parameter g for updating dctcp_alpha");

static mut dctcp_alpha_on_init: u32 = DCTCP_MAX_ALPHA;
// module_param(dctcp_alpha_on_init, uint, 0644);
// MODULE_PARM_DESC(dctcp_alpha_on_init, "parameter for initial alpha value");

unsafe fn dctcp_reset(tp: *const tcp_sock, ca: *mut dctcp) {
    (*ca).next_seq = (*tp).snd_nxt;
    (*ca).old_delivered = (*tp).delivered;
    (*ca).old_delivered_ce = (*tp).delivered_ce;
}

unsafe fn dctcp_init(sk: *mut sock) {
    let tp = tcp_sk(sk);
    if tcp_ecn_mode_any(tp) || ((*sk).sk_state == TCP_LISTEN || (*sk).sk_state == TCP_CLOSE) {
        let ca = inet_csk_ca(sk);
        (*ca).prior_rcv_nxt = (*tp).rcv_nxt;
        (*ca).dctcp_alpha = core::cmp::min(dctcp_alpha_on_init, DCTCP_MAX_ALPHA);
        (*ca).loss_cwnd = 0;
        (*ca).ce_state = 0;
        dctcp_reset(tp, ca);
        tcp_plb_init(sk, &mut (*ca).plb);
        return;
    }
    (*inet_csk(sk)).icsk_ca_ops = &raw mut dctcp_reno;
    INET_ECN_dontxmit(sk);
}

unsafe fn dctcp_ssthresh(sk: *mut sock) -> u32 {
    let ca = inet_csk_ca(sk);
    let tp = tcp_sk(sk);
    (*ca).loss_cwnd = tcp_snd_cwnd(tp);
    core::cmp::max(tcp_snd_cwnd(tp) - ((tcp_snd_cwnd(tp) * (*ca).dctcp_alpha) >> 11), 2)
}

unsafe fn dctcp_update_alpha(sk: *mut sock, _flags: u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    if !before((*tp).snd_una, (*ca).next_seq) {
        let delivered = (*tp).delivered.wrapping_sub((*ca).old_delivered);
        let delivered_ce = (*tp).delivered_ce.wrapping_sub((*ca).old_delivered_ce);
        let mut alpha = (*ca).dctcp_alpha;
        let mut ce_ratio = 0;
        if delivered > 0 {
            if delivered_ce > 0 { ce_ratio = (delivered_ce << TCP_PLB_SCALE) / delivered; }
            tcp_plb_update_state(sk, &mut (*ca).plb, ce_ratio as c_int);
            tcp_plb_check_rehash(sk, &mut (*ca).plb);
        }
        alpha -= core::cmp::min(if alpha != 0 { alpha } else { 1 }, alpha >> dctcp_shift_g);
        if delivered_ce != 0 {
            let mut marked = delivered_ce << (10 - dctcp_shift_g);
            marked /= core::cmp::max(1, delivered);
            alpha = core::cmp::min(alpha + marked, DCTCP_MAX_ALPHA);
        }
        WRITE_ONCE(&mut (*ca).dctcp_alpha, alpha);
        dctcp_reset(tp, ca);
    }
}

unsafe fn dctcp_react_to_loss(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let tp = tcp_sk(sk);
    (*ca).loss_cwnd = tcp_snd_cwnd(tp);
    WRITE_ONCE(&mut (*tp).snd_ssthresh, core::cmp::max(tcp_snd_cwnd(tp) >> 1, 2));
}

unsafe fn dctcp_state(sk: *mut sock, new_state: u8) {
    if new_state == TCP_CA_Recovery && new_state != (*inet_csk(sk)).icsk_ca_state { dctcp_react_to_loss(sk); }
    /* RTO is handled in dctcp_cwnd_event so there is only one loss adjustment per RTT. */
}

unsafe fn dctcp_cwnd_event(sk: *mut sock, ev: tcp_ca_event) {
    let ca = inet_csk_ca(sk);
    match ev {
        CA_EVENT_ECN_IS_CE | CA_EVENT_ECN_NO_CE => dctcp_ece_ack_update(sk, ev, &mut (*ca).prior_rcv_nxt, &mut (*ca).ce_state),
        CA_EVENT_LOSS => { tcp_plb_update_state_upon_rto(sk, &mut (*ca).plb); dctcp_react_to_loss(sk); },
        _ => {},
    }
}

unsafe fn dctcp_cwnd_event_tx_start(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    tcp_plb_check_rehash(sk, &mut (*ca).plb);
}

unsafe fn dctcp_get_info(sk: *mut sock, ext: u32, attr: *mut c_int, info: *mut tcp_cc_info) -> usize {
    let ca = inet_csk_ca(sk);
    let tp = tcp_sk(sk);
    if ext & (1 << (INET_DIAG_DCTCPINFO - 1)) != 0 || ext & (1 << (INET_DIAG_VEGASINFO - 1)) != 0 {
        core::ptr::write_bytes(&mut (*info).dctcp, 0, 1);
        if (*inet_csk(sk)).icsk_ca_ops != &raw mut dctcp_reno {
            (*info).dctcp.dctcp_enabled = 1;
            (*info).dctcp.dctcp_ce_state = (*ca).ce_state as u16;
            (*info).dctcp.dctcp_alpha = (*ca).dctcp_alpha;
            (*info).dctcp.dctcp_ab_ecn = (*tp).mss_cache * ((*tp).delivered_ce - (*ca).old_delivered_ce);
            (*info).dctcp.dctcp_ab_tot = (*tp).mss_cache * ((*tp).delivered - (*ca).old_delivered);
        }
        *attr = INET_DIAG_DCTCPINFO;
        return core::mem::size_of::<tcp_dctcp_info>();
    }
    0
}

unsafe fn dctcp_cwnd_undo(sk: *mut sock) -> u32 {
    let ca = inet_csk_ca(sk);
    core::cmp::max(tcp_snd_cwnd(tcp_sk(sk)), (*ca).loss_cwnd)
}

// The following congestion-operation tables, BTF registration, module
// registration, and metadata correspond directly to the C initializers/macros.
static mut dctcp: tcp_congestion_ops = tcp_congestion_ops { init: Some(dctcp_init), in_ack_event: Some(dctcp_update_alpha), cwnd_event: Some(dctcp_cwnd_event), cwnd_event_tx_start: Some(dctcp_cwnd_event_tx_start), ssthresh: Some(dctcp_ssthresh), cong_avoid: Some(tcp_reno_cong_avoid), undo_cwnd: Some(dctcp_cwnd_undo), set_state: Some(dctcp_state), get_info: Some(dctcp_get_info), flags: TCP_CONG_NEEDS_ECN, owner: THIS_MODULE, name: *b"dctcp\0" };
static mut dctcp_reno: tcp_congestion_ops = tcp_congestion_ops { ssthresh: Some(tcp_reno_ssthresh), cong_avoid: Some(tcp_reno_cong_avoid), undo_cwnd: Some(tcp_reno_undo_cwnd), get_info: Some(dctcp_get_info), owner: THIS_MODULE, name: *b"dctcp-reno\0" };

unsafe fn dctcp_register() -> c_int {
    // BUILD_BUG_ON(sizeof(struct dctcp) > ICSK_CA_PRIV_SIZE);
    let ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &tcp_dctcp_kfunc_set);
    if ret < 0 { return ret; }
    tcp_register_congestion_control(&raw mut dctcp)
}

unsafe fn dctcp_unregister() {
    tcp_unregister_congestion_control(&raw mut dctcp);
}

// BTF_KFUNCS_START/END and module_init/module_exit registrations are retained
// as build-system metadata in the surrounding kernel environment.
// module_init(dctcp_register);
// module_exit(dctcp_unregister);
// MODULE_AUTHOR("Daniel Borkmann <dborkman@redhat.com>");
// MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_AUTHOR("Glenn Judd <glenn.judd@morganstanley.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("DataCenter TCP (DCTCP)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
