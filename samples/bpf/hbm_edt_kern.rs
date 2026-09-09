// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * Sample Host Bandwidth Manager (HBM) BPF program.
 *
 * A cgroup skb BPF egress program to limit cgroup output bandwidth.
 * It uses a modified virtual token bucket queue to limit average
 * egress bandwidth. The implementation uses credits instead of tokens.
 * Negative credits imply that queueing would have happened (this is
 * a virtual queue, so no queueing is done by it. However, queueing may
 * occur at the actual qdisc (which is not used for rate limiting).
 */

// Dependency declarations and constants are supplied by hbm_kern.h.
// BPF section: "cgroup_skb/egress"
pub unsafe fn _hbm_out_cg(skb: *mut __sk_buff) -> i32 {
    let mut delta: i64 = 0;
    let mut delta_send: i64;
    let mut curtime: u64;
    let mut sendtime: u64;
    let mut qsp: *mut hbm_queue_stats = core::ptr::null_mut();
    let mut queue_index: u32 = 0;
    let mut congestion_flag = false;
    let mut ecn_ce_flag = false;
    let mut pkti: hbm_pkt_info = core::mem::zeroed();
    let qdp: *mut hbm_vqueue;
    let mut drop_flag = false;
    let mut cwr_flag = false;
    let len: i32 = (*skb).len;
    let mut rv: i32 = ALLOW_PKT;

    qsp = bpf_map_lookup_elem(&queue_stats, &mut queue_index);

    // Check if we should ignore loopback traffic
    if !qsp.is_null() && !(*qsp).loopback && (*skb).ifindex == 1 {
        return ALLOW_PKT;
    }

    hbm_get_pkt_info(skb, &mut pkti);

    // We may want to account for the length of headers in len
    // calculation, like ETH header + overhead, specially if it
    // is a gso packet. But I am not doing it right now.

    qdp = bpf_get_local_storage(&queue_state, 0);
    if qdp.is_null() {
        return ALLOW_PKT;
    }
    if (*qdp).lasttime == 0 {
        hbm_init_edt_vqueue(qdp, 1024);
    }

    curtime = bpf_ktime_get_ns();

    // Begin critical section
    bpf_spin_lock(&mut (*qdp).lock);
    delta = (*qdp).lasttime as i64 - curtime as i64;
    // bound bursts to 100us
    if delta < -(BURST_SIZE_NS as i64) {
        // negative delta is a credit that allows bursts
        (*qdp).lasttime = curtime - BURST_SIZE_NS;
        delta = -(BURST_SIZE_NS as i64);
    }
    sendtime = (*qdp).lasttime;
    delta_send = BYTES_TO_NS(len, (*qdp).rate);
    __sync_add_and_fetch(&mut (*qdp).lasttime, delta_send);
    bpf_spin_unlock(&mut (*qdp).lock);
    // End critical section

    // Set EDT of packet
    (*skb).tstamp = sendtime;

    // Check if we should update rate
    if !qsp.is_null() && (*qsp).rate * 128 != (*qdp).rate {
        (*qdp).rate = (*qsp).rate * 128;
    }

    // Set flags (drop, congestion, cwr)
    // last packet will be sent in the future, bound latency
    if delta > DROP_THRESH_NS as i64
        || (delta > LARGE_PKT_DROP_THRESH_NS as i64 && len > LARGE_PKT_THRESH)
    {
        drop_flag = true;
        if pkti.is_tcp && pkti.ecn == 0 {
            cwr_flag = true;
        }
    } else if delta > MARK_THRESH_NS as i64 {
        if pkti.is_tcp {
            congestion_flag = true;
        } else {
            drop_flag = true;
        }
    }

    if congestion_flag {
        if bpf_skb_ecn_set_ce(skb) != 0 {
            ecn_ce_flag = true;
        } else if pkti.is_tcp {
            let rand: u32 = bpf_get_prandom_u32();

            if delta >= MARK_THRESH_NS as i64 + (rand % MARK_REGION_SIZE_NS) as i64 {
                // Do congestion control
                cwr_flag = true;
            }
        } else if len > LARGE_PKT_THRESH {
            // Problem if too many small packets?
            drop_flag = true;
            congestion_flag = false;
        }
    }

    if pkti.is_tcp && drop_flag && pkti.packets_out <= 1 {
        drop_flag = false;
        cwr_flag = true;
        congestion_flag = false;
    }

    if !qsp.is_null() && (*qsp).no_cn {
        cwr_flag = false;
    }

    hbm_update_stats(
        qsp, len, curtime, congestion_flag, drop_flag, cwr_flag, ecn_ce_flag,
        &mut pkti, delta as i32,
    );

    if drop_flag {
        __sync_add_and_fetch(&mut (*qdp).lasttime, -delta_send);
        rv = DROP_PKT;
    }

    if cwr_flag {
        rv |= CWR;
    }
    rv
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
