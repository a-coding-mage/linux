// SPDX-License-Identifier: GPL-2.0-only
/*
 * CAIA Delay-Gradient (CDG) congestion control
 *
 * This implementation is based on the paper:
 *   D.A. Hayes and G. Armitage. "Revisiting TCP congestion control using
 *   delay gradients." In IFIP Networking, pages 328-341. Springer, 2011.
 *
 * Scavenger traffic (Less-than-Best-Effort) should disable coexistence
 * heuristics using parameters use_shadow=0 and use_ineff=0.
 *
 * Parameters window, backoff_beta, and backoff_factor are crucial for
 * throughput and delay. Future work is needed to determine better defaults,
 * and to provide guidelines for use in different environments/contexts.
 *
 * Except for window, knobs are configured via /sys/module/tcp_cdg/parameters/.
 * Parameter window is only configurable when loading tcp_cdg as a module.
 *
 * Notable differences from paper/FreeBSD:
 *   o Using Hybrid Slow start and Proportional Rate Reduction.
 *   o Add toggle for shadow window mechanism. Suggested by David Hayes.
 *   o Add toggle for non-congestion loss tolerance.
 *   o Scaling parameter G is changed to a backoff factor;
 *     conversion is given by: backoff_factor = 1000/(G * window).
 *   o Limit shadow window to 2 * cwnd, or to cwnd when application limited.
 *   o More accurate e^-x.
 */

// Linux kernel and TCP declarations are supplied by the surrounding kernel bindings.

const HYSTART_ACK_TRAIN: u32 = 1;
const HYSTART_DELAY: u32 = 2;

static mut window: i32 = 8;
static mut backoff_beta: u32 = 0.7071_f64 as u32 * 1024;
static mut backoff_factor: u32 = 42;
static mut hystart_detect: u32 = 3;
static mut use_ineff: u32 = 5;
static mut use_shadow: bool = true;
static mut use_tolerance: bool = false;

#[repr(C)]
pub union cdg_minmax {
    pub fields: CdgMinMaxFields,
    pub v64: u64,
}
#[repr(C)]
pub struct CdgMinMaxFields { pub min: i32, pub max: i32 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdg_state { CDG_UNKNOWN = 0, CDG_NONFULL = 1, CDG_FULL = 2, CDG_BACKOFF = 3 }

#[repr(C)]
pub struct cdg {
    pub rtt: cdg_minmax,
    pub rtt_prev: cdg_minmax,
    pub gradients: *mut cdg_minmax,
    pub gsum: cdg_minmax,
    pub gfilled: bool,
    pub tail: u8,
    pub state: u8,
    pub delack: u8,
    pub rtt_seq: u32,
    pub shadow_wnd: u32,
    pub backoff_cnt: u16,
    pub sample_cnt: u16,
    pub delay_min: i32,
    pub last_ack: u32,
    pub round_start: u32,
}

/// nexp_u32 - negative base-e exponential
/// @ux: x in units of micro
///
/// Returns exp(ux * -1e-6) * U32_MAX.
unsafe fn nexp_u32(ux: u32) -> u32 {
    static V: [u16; 16] = [65535, 65518, 65501, 65468, 65401, 65267, 65001, 64470,
        63422, 61378, 57484, 50423, 38795, 22965, 8047, 987];
    let mut msb = ux >> 8;
    if msb > u16::MAX as u32 { return 0; }
    let mut res = u32::MAX.wrapping_sub((ux & 0xff).wrapping_mul(u32::MAX / 1_000_000));
    let mut i = 1usize;
    while msb != 0 {
        let y = V[i & (-(msb & 1) as usize)] as u32 + 1;
        res = (((res as u64) * y as u64) >> 16) as u32;
        i += 1;
        msb >>= 1;
    }
    res
}

// The following kernel types and functions are external declarations supplied by dependencies.
extern "C" {
    fn inet_csk_ca(sk: *mut sock) -> *mut cdg;
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn min_not_zero(a: i32, b: i32) -> i32;
    fn max(a: i32, b: i32) -> i32;
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_snd_cwnd(tp: *mut tcp_sock) -> u32;
    fn tcp_enter_cwr(sk: *mut sock);
    fn tcp_reno_cong_avoid(sk: *mut sock, ack: u32, acked: u32);
    fn tcp_reno_undo_cwnd(sk: *mut sock) -> u32;
    fn get_random_u32() -> u32;
    fn tcp_in_slow_start(tp: *mut tcp_sock) -> bool;
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops);
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn kfree(p: *mut cdg_minmax);
}

#[repr(C)] pub struct sock;
#[repr(C)] pub struct tcp_sock { pub tcp_mstamp: u32, pub sacked_out: u32, pub snd_nxt: u32, pub snd_ssthresh: u32 }
#[repr(C)] pub struct ack_sample { pub rtt_us: i32, pub pkts_acked: u32 }
#[repr(C)] pub struct tcp_congestion_ops;
#[repr(C)] pub enum tcp_ca_event { CA_EVENT_CWND_RESTART, CA_EVENT_COMPLETE_CWR }

unsafe fn tcp_cdg_grad(ca: *mut cdg) -> i32 {
    let ca = &mut *ca;
    let mut gmin = ca.rtt.fields.min - ca.rtt_prev.fields.min;
    let mut gmax = ca.rtt.fields.max - ca.rtt_prev.fields.max;
    if !ca.gradients.is_null() {
        let p = ca.gradients.add(ca.tail as usize);
        ca.gsum.fields.min += gmin - (*p).fields.min;
        ca.gsum.fields.max += gmax - (*p).fields.max;
        (*p).fields.min = gmin; (*p).fields.max = gmax;
        ca.tail = (ca.tail + 1) & ((window as u8) - 1);
        gmin = ca.gsum.fields.min; gmax = ca.gsum.fields.max;
    }
    let mut grad = if gmin > 0 { gmin } else { gmax };
    if !ca.gfilled {
        if ca.gradients.is_null() && window > 1 { grad *= window; }
        else if ca.tail == 0 { ca.gfilled = true; }
        else { grad = (grad * window) / ca.tail as i32; }
    }
    if gmin <= -32 || gmax <= -32 { ca.backoff_cnt = 0; }
    if use_tolerance {
        gmin = (gmin + 32) / 64; gmax = (gmax + 32) / 64;
        if gmin > 0 && gmax <= 0 { ca.state = cdg_state::CDG_FULL as u8; }
        else if (gmin > 0 && gmax > 0) || gmax < 0 { ca.state = cdg_state::CDG_NONFULL as u8; }
    }
    grad
}

unsafe fn tcp_cdg_backoff(sk: *mut sock, grad: u32) -> bool {
    let ca = &mut *inet_csk_ca(sk); let tp = tcp_sk(sk);
    if get_random_u32() <= nexp_u32(grad.wrapping_mul(backoff_factor)) { return false; }
    if use_ineff != 0 { ca.backoff_cnt += 1; if ca.backoff_cnt > use_ineff as u16 { return false; } }
    ca.shadow_wnd = ca.shadow_wnd.max(tcp_snd_cwnd(tp)); ca.state = cdg_state::CDG_BACKOFF as u8;
    tcp_enter_cwr(sk); true
}

unsafe fn tcp_cdg_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let ca = &mut *inet_csk_ca(sk); let tp = tcp_sk(sk);
    if tcp_in_slow_start(tp) && hystart_detect != 0 { /* tcp_cdg_hystart_update(sk); */ }
    let _ = ca; let _ = tp; let _ = ack; let _ = acked;
}

unsafe fn tcp_cdg_acked(sk: *mut sock, sample: *const ack_sample) {
    let ca = &mut *inet_csk_ca(sk); let tp = tcp_sk(sk); let sample = &*sample;
    if sample.rtt_us <= 0 { return; }
    if (*tp).sacked_out == 0 {
        if sample.pkts_acked == 1 && ca.delack != 0 { ca.rtt.fields.min = ca.rtt.fields.min.min(sample.rtt_us); ca.delack -= 1; return; }
        else if sample.pkts_acked > 1 && ca.delack < 5 { ca.delack += 1; }
    }
    ca.rtt.fields.min = if ca.rtt.fields.min == 0 { sample.rtt_us } else { ca.rtt.fields.min.min(sample.rtt_us) };
    ca.rtt.fields.max = ca.rtt.fields.max.max(sample.rtt_us);
}

unsafe fn tcp_cdg_ssthresh(sk: *mut sock) -> u32 {
    let ca = &mut *inet_csk_ca(sk); let tp = tcp_sk(sk); let cwnd = tcp_snd_cwnd(tp);
    if ca.state == cdg_state::CDG_BACKOFF as u8 { return 2.max((cwnd * backoff_beta.min(1024)) >> 10); }
    if ca.state == cdg_state::CDG_NONFULL as u8 && use_tolerance { return cwnd; }
    ca.shadow_wnd = (ca.shadow_wnd >> 1).min(cwnd);
    if use_shadow { 2.max(ca.shadow_wnd.max(cwnd >> 1)) } else { 2.max(cwnd >> 1) }
}

unsafe fn tcp_cdg_cwnd_event(_sk: *mut sock, _ev: tcp_ca_event) {}
unsafe fn tcp_cdg_init(sk: *mut sock) { let ca = &mut *inet_csk_ca(sk); let tp = tcp_sk(sk); ca.gradients = core::ptr::null_mut(); ca.rtt_seq = (*tp).snd_nxt; ca.shadow_wnd = tcp_snd_cwnd(tp); }
unsafe fn tcp_cdg_release(sk: *mut sock) { let ca = &mut *inet_csk_ca(sk); kfree(ca.gradients); ca.gradients = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
