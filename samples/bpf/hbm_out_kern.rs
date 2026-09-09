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
 *
 * This implementation uses 3 thresholds, one to start marking packets and
 * the other two to drop packets:
 *                                  CREDIT
 *        - <--------------------------|------------------------> +
 *              |    |          |      0
 *              |  Large pkt    |
 *              |  drop thresh  |
 *   Small pkt drop             Mark threshold
 *       thresh
 *
 * The effect of marking depends on the type of packet:
 * a) If the packet is ECN enabled and it is a TCP packet, then the packet
 *    is ECN marked.
 * b) If the packet is a TCP packet, then we probabilistically call tcp_cwr
 *    to reduce the congestion window. The current implementation uses a linear
 *    distribution (0% probability at marking threshold, 100% probability
 *    at drop threshold).
 * c) If the packet is not a TCP packet, then it is dropped.
 *
 * If the credit is below the drop threshold, the packet is dropped. If it
 * is a TCP packet, then it also calls tcp_cwr since packets dropped by
 * by a cgroup skb BPF program do not automatically trigger a call to
 * tcp_cwr in the current kernel code.
 *
 * This BPF program actually uses 2 drop thresholds, one threshold
 * for larger packets (>= 120 bytes) and another for smaller packets. This
 * protects smaller packets such as SYNs, ACKs, etc.
 *
 * The default bandwidth limit is set at 1Gbps but this can be changed by
 * a user program through a shared BPF map. In addition, by default this BPF
 * program does not limit connections using loopback. This behavior can be
 * overwritten by the user program. There is also an option to calculate
 * some statistics, such as percent of packets marked or dropped, which the
 * user program can access.
 *
 * A latter patch provides such a program (hbm.c)
 */

// Dependency declarations and SEC("cgroup_skb/egress") are supplied by the BPF environment.

extern "C" {
    static mut queue_stats: QueueStatsMap;
    static mut queue_state: QueueStateMap;

    fn bpf_map_lookup_elem(map: *mut QueueStatsMap, key: *const u32) -> *mut hbm_queue_stats;
    fn hbm_get_pkt_info(skb: *mut __sk_buff, pkti: *mut hbm_pkt_info);
    fn bpf_get_local_storage(map: *mut QueueStateMap, flags: u64) -> *mut hbm_vqueue;
    fn hbm_init_vqueue(qdp: *mut hbm_vqueue, rate: u64);
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_skb_ecn_set_ce(skb: *mut __sk_buff) -> i32;
    fn bpf_get_prandom_u32() -> u32;
    fn hbm_update_stats(qsp: *mut hbm_queue_stats, len: i32, curtime: u64,
                        congestion_flag: bool, drop_flag: bool, cwr_flag: bool,
                        ecn_ce_flag: bool, pkti: *mut hbm_pkt_info, credit: i64);
}

#[allow(non_snake_case)]
pub unsafe fn _hbm_out_cg(skb: *mut __sk_buff) -> i32 {
    let mut pkti: hbm_pkt_info = core::mem::zeroed();
    let len: i32 = (*skb).len as i32;
    let queue_index: u32 = 0;
    let mut curtime: u64;
    let mut credit: i32;
    let mut delta: i64 = 0;
    let mut new_credit: i64;
    let max_credit: i32 = MAX_CREDIT;
    let mut congestion_flag = false;
    let mut drop_flag = false;
    let mut cwr_flag = false;
    let mut ecn_ce_flag = false;
    let mut qdp: *mut hbm_vqueue;
    let mut qsp: *mut hbm_queue_stats = core::ptr::null_mut();
    let mut rv: i32 = ALLOW_PKT;

    qsp = bpf_map_lookup_elem(&mut queue_stats, &queue_index);
    if !qsp.is_null() && !(*qsp).loopback && (*skb).ifindex == 1 {
        return ALLOW_PKT;
    }

    hbm_get_pkt_info(skb, &mut pkti);

    // We may want to account for the length of headers in len
    // calculation, like ETH header + overhead, specially if it
    // is a gso packet. But I am not doing it right now.

    qdp = bpf_get_local_storage(&mut queue_state, 0);
    if qdp.is_null() {
        return ALLOW_PKT;
    } else if (*qdp).lasttime == 0 {
        hbm_init_vqueue(qdp, 1024);
    }

    curtime = bpf_ktime_get_ns();

    // Begin critical section
    bpf_spin_lock(&mut (*qdp).lock);
    credit = (*qdp).credit;
    delta = curtime as i64 - (*qdp).lasttime as i64;
    /* delta < 0 implies that another process with a curtime greater
     * than ours beat us to the critical section and already added
     * the new credit, so we should not add it ourselves
     */
    if delta > 0 {
        (*qdp).lasttime = curtime;
        new_credit = credit as i64 + CREDIT_PER_NS(delta, (*qdp).rate);
        if new_credit > max_credit as i64 {
            credit = max_credit;
        } else {
            credit = new_credit as i32;
        }
    }
    credit -= len;
    (*qdp).credit = credit;
    bpf_spin_unlock(&mut (*qdp).lock);
    // End critical section

    // Check if we should update rate
    if !qsp.is_null() && (*qsp).rate * 128 != (*qdp).rate {
        (*qdp).rate = (*qsp).rate * 128;
        bpf_printk!("Updating rate: %d (1sec:%llu bits)\n",
                    (*qdp).rate as i32,
                    CREDIT_PER_NS(1_000_000_000, (*qdp).rate) * 8);
    }

    // Set flags (drop, congestion, cwr)
    // Dropping => we are congested, so ignore congestion flag
    if credit < -DROP_THRESH || (len > LARGE_PKT_THRESH && credit < -LARGE_PKT_DROP_THRESH) {
        // Very congested, set drop packet
        drop_flag = true;
        if pkti.ecn {
            congestion_flag = true;
        } else if pkti.is_tcp {
            cwr_flag = true;
        }
    } else if credit < 0 {
        // Congested, set congestion flag
        if pkti.ecn || pkti.is_tcp {
            if credit < -MARK_THRESH {
                congestion_flag = true;
            } else {
                congestion_flag = false;
            }
        } else {
            congestion_flag = true;
        }
    }

    if congestion_flag {
        if bpf_skb_ecn_set_ce(skb) != 0 {
            ecn_ce_flag = true;
        } else if pkti.is_tcp {
            let rand = bpf_get_prandom_u32();
            if -credit >= MARK_THRESH + (rand % MARK_REGION_SIZE) as i32 {
                // Do congestion control
                cwr_flag = true;
            }
        } else if len > LARGE_PKT_THRESH {
            // Problem if too many small packets?
            drop_flag = true;
        }
    }

    if !qsp.is_null() && (*qsp).no_cn {
        cwr_flag = false;
    }

    hbm_update_stats(qsp, len, curtime, congestion_flag, drop_flag,
                     cwr_flag, ecn_ce_flag, &mut pkti, credit as i64);

    if drop_flag {
        core::sync::atomic::AtomicI32::from_ptr(&mut (*qdp).credit)
            .fetch_add(len, core::sync::atomic::Ordering::SeqCst);
        rv = DROP_PKT;
    }

    if cwr_flag {
        rv |= 2;
    }
    rv
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
