// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/* WARNING: This implementation is not necessarily the same
 * as the tcp_dctcp.c.  The purpose is mainly for testing
 * the kernel BPF logic.
 */

// C dependencies translated as external Rust dependencies:
// "bpf_tracing_net.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub const EBUSY: i32 = 16;
pub const DCTCP_MAX_ALPHA: u32 = 1024;

#[inline]
fn min_not_zero<T>(x: T, y: T) -> T
where
    T: Copy + Ord + From<u8> + PartialEq,
{
    let __x = x;
    let __y = y;

    if __x == T::from(0) {
        __y
    } else if __y == T::from(0) {
        __x
    } else {
        core::cmp::min(__x, __y)
    }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static fallback_cc: [core::ffi::c_char; TCP_CA_NAME_MAX as usize] =
    [0; TCP_CA_NAME_MAX as usize];
#[no_mangle]
pub static bpf_dctcp: [core::ffi::c_char; 10] =
    [b'b' as _, b'p' as _, b'f' as _, b'_' as _, b'd' as _, b'c' as _, b't' as _, b'c' as _, b'p' as _, 0];
#[no_mangle]
pub static tcp_cdg: [core::ffi::c_char; 4] = [b'c' as _, b'd' as _, b'g' as _, 0];
#[no_mangle]
pub static mut cc_res: [core::ffi::c_char; TCP_CA_NAME_MAX as usize] =
    [0; TCP_CA_NAME_MAX as usize];
#[no_mangle]
pub static mut tcp_cdg_res: i32 = 0;
#[no_mangle]
pub static mut stg_result: i32 = 0;
#[no_mangle]
pub static mut ebusy_cnt: i32 = 0;

// Original C used BPF map definition macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } sk_stg_map SEC(".maps");
#[repr(C)]
pub struct sk_stg_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: i32,
    pub value: i32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut sk_stg_map: sk_stg_map_def = sk_stg_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct bpf_dctcp_ca {
    pub old_delivered: u32,
    pub old_delivered_ce: u32,
    pub prior_rcv_nxt: u32,
    pub dctcp_alpha: u32,
    pub next_seq: u32,
    pub ce_state: u32,
    pub loss_cwnd: u32,
}

static mut dctcp_shift_g: u32 = 4; /* g = 1/2^4 */
static mut dctcp_alpha_on_init: u32 = DCTCP_MAX_ALPHA;

unsafe fn dctcp_reset(tp: *const tcp_sock, ca: *mut bpf_dctcp_ca) {
    (*ca).next_seq = (*tp).snd_nxt;

    (*ca).old_delivered = (*tp).delivered;
    (*ca).old_delivered_ce = (*tp).delivered_ce;
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_init(sk: *mut sock) {
    let tp: *const tcp_sock = tcp_sk(sk);
    let ca: *mut bpf_dctcp_ca = inet_csk_ca(sk);
    let mut stg: *mut i32;

    if ((*tp).ecn_flags & TCP_ECN_OK) == 0 && fallback_cc[0] != 0 {
        /* Switch to fallback */
        if bpf_setsockopt(
            sk,
            SOL_TCP,
            TCP_CONGESTION,
            fallback_cc.as_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&fallback_cc) as u32,
        ) == -EBUSY
        {
            ebusy_cnt += 1;
        }

        /* Switch back to myself and the recurred bpf_dctcp_init()
         * will get -EBUSY for all bpf_setsockopt(TCP_CONGESTION),
         * except the last "cdg" one.
         */
        if bpf_setsockopt(
            sk,
            SOL_TCP,
            TCP_CONGESTION,
            bpf_dctcp.as_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&bpf_dctcp) as u32,
        ) == -EBUSY
        {
            ebusy_cnt += 1;
        }

        /* Switch back to fallback */
        if bpf_setsockopt(
            sk,
            SOL_TCP,
            TCP_CONGESTION,
            fallback_cc.as_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&fallback_cc) as u32,
        ) == -EBUSY
        {
            ebusy_cnt += 1;
        }

        /* Expecting -ENOTSUPP for tcp_cdg_res */
        tcp_cdg_res = bpf_setsockopt(
            sk,
            SOL_TCP,
            TCP_CONGESTION,
            tcp_cdg.as_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&tcp_cdg) as u32,
        );
        bpf_getsockopt(
            sk,
            SOL_TCP,
            TCP_CONGESTION,
            cc_res.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&cc_res) as u32,
        );
        return;
    }

    (*ca).prior_rcv_nxt = (*tp).rcv_nxt;
    (*ca).dctcp_alpha = core::cmp::min(dctcp_alpha_on_init, DCTCP_MAX_ALPHA);
    (*ca).loss_cwnd = 0;
    (*ca).ce_state = 0;

    stg = bpf_sk_storage_get(
        &mut sk_stg_map as *mut sk_stg_map_def as *mut core::ffi::c_void,
        tp as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        0,
    ) as *mut i32;
    if !stg.is_null() {
        stg_result = *stg;
        bpf_sk_storage_delete(
            &mut sk_stg_map as *mut sk_stg_map_def as *mut core::ffi::c_void,
            tp as *mut core::ffi::c_void,
        );
    }
    dctcp_reset(tp, ca);
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_ssthresh(sk: *mut sock) -> u32 {
    let ca: *mut bpf_dctcp_ca = inet_csk_ca(sk);
    let tp: *mut tcp_sock = tcp_sk(sk);

    (*ca).loss_cwnd = (*tp).snd_cwnd;
    core::cmp::max(
        (*tp).snd_cwnd
            .wrapping_sub(((*tp).snd_cwnd.wrapping_mul((*ca).dctcp_alpha)) >> 11u32),
        2u32,
    )
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_update_alpha(sk: *mut sock, _flags: u32) {
    let tp: *const tcp_sock = tcp_sk(sk);
    let ca: *mut bpf_dctcp_ca = inet_csk_ca(sk);

    /* Expired RTT */
    if !before((*tp).snd_una, (*ca).next_seq) {
        let mut delivered_ce: u32 = (*tp).delivered_ce.wrapping_sub((*ca).old_delivered_ce);
        let mut alpha: u32 = (*ca).dctcp_alpha;

        /* alpha = (1 - g) * alpha + g * F */

        alpha = alpha.wrapping_sub(min_not_zero(alpha, alpha >> dctcp_shift_g));
        if delivered_ce != 0 {
            let delivered: u32 = (*tp).delivered.wrapping_sub((*ca).old_delivered);

            /* If dctcp_shift_g == 1, a 32bit value would overflow
             * after 8 M packets.
             */
            delivered_ce <<= 10 - dctcp_shift_g;
            delivered_ce /= core::cmp::max(1u32, delivered);

            alpha = core::cmp::min(alpha.wrapping_add(delivered_ce), DCTCP_MAX_ALPHA);
        }
        (*ca).dctcp_alpha = alpha;
        dctcp_reset(tp, ca);
    }
}

unsafe fn dctcp_react_to_loss(sk: *mut sock) {
    let ca: *mut bpf_dctcp_ca = inet_csk_ca(sk);
    let tp: *mut tcp_sock = tcp_sk(sk);

    (*ca).loss_cwnd = (*tp).snd_cwnd;
    (*tp).snd_ssthresh = core::cmp::max((*tp).snd_cwnd >> 1u32, 2u32);
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_state(sk: *mut sock, new_state: u8) {
    if new_state == TCP_CA_Recovery
        && new_state != BPF_CORE_READ_BITFIELD(inet_csk(sk), icsk_ca_state)
    {
        dctcp_react_to_loss(sk);
    }
    /* We handle RTO in bpf_dctcp_cwnd_event to ensure that we perform only
     * one loss-adjustment per RTT.
     */
}

unsafe fn dctcp_ece_ack_cwr(sk: *mut sock, ce_state: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);

    if ce_state == 1 {
        (*tp).ecn_flags |= TCP_ECN_DEMAND_CWR;
    } else {
        (*tp).ecn_flags &= !TCP_ECN_DEMAND_CWR;
    }
}

/* Minimal DCTP CE state machine:
 *
 * S:	0 <- last pkt was non-CE
 *	1 <- last pkt was CE
 */
unsafe fn dctcp_ece_ack_update(
    sk: *mut sock,
    evt: tcp_ca_event,
    prior_rcv_nxt: *mut u32,
    ce_state: *mut u32,
) {
    let new_ce_state: u32 = if evt == CA_EVENT_ECN_IS_CE { 1 } else { 0 };

    if *ce_state != new_ce_state {
        /* CE state has changed, force an immediate ACK to
         * reflect the new CE state. If an ACK was delayed,
         * send that first to reflect the prior CE state.
         */
        if ((*inet_csk(sk)).icsk_ack.pending & ICSK_ACK_TIMER) != 0 {
            dctcp_ece_ack_cwr(sk, *ce_state);
            bpf_tcp_send_ack(sk, *prior_rcv_nxt);
        }
        (*inet_csk(sk)).icsk_ack.pending |= ICSK_ACK_NOW;
    }
    *prior_rcv_nxt = (*tcp_sk(sk)).rcv_nxt;
    *ce_state = new_ce_state;
    dctcp_ece_ack_cwr(sk, new_ce_state);
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_cwnd_event(sk: *mut sock, ev: tcp_ca_event) {
    let ca: *mut bpf_dctcp_ca = inet_csk_ca(sk);

    match ev {
        CA_EVENT_ECN_IS_CE | CA_EVENT_ECN_NO_CE => {
            dctcp_ece_ack_update(sk, ev, &mut (*ca).prior_rcv_nxt, &mut (*ca).ce_state);
        }
        CA_EVENT_LOSS => {
            dctcp_react_to_loss(sk);
        }
        _ => {
            /* Don't care for the rest. */
        }
    }
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_cwnd_undo(sk: *mut sock) -> u32 {
    let ca: *const bpf_dctcp_ca = inet_csk_ca(sk);

    core::cmp::max((*tcp_sk(sk)).snd_cwnd, (*ca).loss_cwnd)
}

extern "C" {
    pub fn tcp_reno_cong_avoid(sk: *mut sock, ack: u32, acked: u32);
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_dctcp_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    tcp_reno_cong_avoid(sk, ack, acked);
}

#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut dctcp_nouse: tcp_congestion_ops = tcp_congestion_ops {
    init: bpf_dctcp_init as *mut core::ffi::c_void,
    set_state: bpf_dctcp_state as *mut core::ffi::c_void,
    flags: TCP_CONG_NEEDS_ECN,
    name: [
        b'b' as _, b'p' as _, b'f' as _, b'_' as _, b'd' as _, b'c' as _, b't' as _, b'c' as _,
        b'p' as _, b'_' as _, b'n' as _, b'o' as _, b'u' as _, b's' as _, b'e' as _, 0,
    ],
    ..unsafe { core::mem::zeroed() }
};

#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut dctcp: tcp_congestion_ops = tcp_congestion_ops {
    init: bpf_dctcp_init as *mut core::ffi::c_void,
    in_ack_event: bpf_dctcp_update_alpha as *mut core::ffi::c_void,
    cwnd_event: bpf_dctcp_cwnd_event as *mut core::ffi::c_void,
    ssthresh: bpf_dctcp_ssthresh as *mut core::ffi::c_void,
    cong_avoid: bpf_dctcp_cong_avoid as *mut core::ffi::c_void,
    undo_cwnd: bpf_dctcp_cwnd_undo as *mut core::ffi::c_void,
    set_state: bpf_dctcp_state as *mut core::ffi::c_void,
    flags: TCP_CONG_NEEDS_ECN,
    name: [
        b'b' as _, b'p' as _, b'f' as _, b'_' as _, b'd' as _, b'c' as _, b't' as _, b'c' as _,
        b'p' as _, 0,
    ],
    ..unsafe { core::mem::zeroed() }
};
