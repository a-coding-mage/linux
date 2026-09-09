/*
 * Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Kernel/network types, constants, macros, and external functions are supplied by dependencies.

static mut rds_tcp_incoming_slab: *mut kmem_cache = core::ptr::null_mut();

unsafe fn rds_tcp_inc_purge(inc: *mut rds_incoming) {
    let tinc: *mut rds_tcp_incoming = container_of!(inc, rds_tcp_incoming, ti_inc);
    rdsdebug!("purging tinc %p inc %p\n", tinc, inc);
    skb_queue_purge(&mut (*tinc).ti_skb_list);
}

pub unsafe fn rds_tcp_inc_free(inc: *mut rds_incoming) {
    let tinc: *mut rds_tcp_incoming = container_of!(inc, rds_tcp_incoming, ti_inc);
    rds_tcp_inc_purge(inc);
    rdsdebug!("freeing tinc %p inc %p\n", tinc, inc);
    kmem_cache_free(rds_tcp_incoming_slab, tinc);
}

pub unsafe fn rds_tcp_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> i32 {
    let mut ret: i32 = 0;
    if iov_iter_count(to) == 0 { return ret; }

    let tinc: *mut rds_tcp_incoming = container_of!(inc, rds_tcp_incoming, ti_inc);
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    skb_queue_walk!(&(*tinc).ti_skb_list, skb, {
        let mut skb_off: usize = 0;
        while skb_off < (*skb).len {
            let mut to_copy = iov_iter_count(to);
            to_copy = core::cmp::min(to_copy, (*skb).len - skb_off);
            if skb_copy_datagram_iter(skb, skb_off, to, to_copy) != 0 { return -EFAULT; }
            rds_stats_add!(s_copy_to_user, to_copy);
            ret += to_copy as i32;
            if iov_iter_count(to) == 0 { return ret; }
            skb_off += to_copy;
        }
    });
    ret
}

unsafe fn rds_tcp_cong_recv(conn: *mut rds_connection, tinc: *mut rds_tcp_incoming) {
    if be32_to_cpu((*tinc).ti_inc.i_hdr.h_len) != RDS_CONG_MAP_BYTES { return; }
    let mut map_page: usize = 0;
    let mut map_off: usize = 0;
    let map: *mut rds_cong_map = (*conn).c_fcong;
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    skb_queue_walk!(&(*tinc).ti_skb_list, skb, {
        let mut skb_off: usize = 0;
        while skb_off < (*skb).len {
            let to_copy = core::cmp::min(PAGE_SIZE - map_off, (*skb).len - skb_off);
            BUG_ON!(map_page >= RDS_CONG_MAP_PAGES);
            let ret = skb_copy_bits(skb, skb_off,
                ((*map).m_page_addrs[map_page] as *mut u8).add(map_off), to_copy);
            BUG_ON!(ret != 0);
            skb_off += to_copy;
            map_off += to_copy;
            if map_off == PAGE_SIZE { map_off = 0; map_page += 1; }
        }
    });
    rds_cong_map_updated(map, !0u64);
}

pub struct rds_tcp_desc_arg { pub conn_path: *mut rds_conn_path, pub gfp: gfp_t }

unsafe fn rds_tcp_data_recv(desc: *mut read_descriptor_t, skb: *mut sk_buff,
                             mut offset: u32, len: usize) -> usize {
    let arg = (*desc).arg.data as *mut rds_tcp_desc_arg;
    let cp = (*arg).conn_path;
    let tc = (*cp).cp_transport_data as *mut rds_tcp_connection;
    let mut tinc = (*tc).t_tinc;
    let mut left = len;
    rdsdebug!("tcp data tc %p skb %p offset %u len %zu\n", tc, skb, offset, len);
    while left != 0 {
        if tinc.is_null() {
            tinc = kmem_cache_alloc(rds_tcp_incoming_slab, (*arg).gfp);
            if tinc.is_null() { (*desc).error = -ENOMEM; break; }
            (*tc).t_tinc = tinc;
            rds_inc_path_init!(&mut (*tinc).ti_inc, cp, &(*cp).cp_conn.as_ref().unwrap().c_faddr);
            (*tinc).ti_inc.i_rx_lat_trace[RDS_MSG_RX_HDR] = local_clock();
            skb_queue_head_init(&mut (*tinc).ti_skb_list);
        }
        if (*tc).t_tinc_hdr_rem != 0 {
            let to_copy = core::cmp::min((*tc).t_tinc_hdr_rem, left);
            skb_copy_bits(skb, offset, (&mut (*tinc).ti_inc.i_hdr as *mut _ as *mut u8)
                .add(core::mem::size_of::<rds_header>() - (*tc).t_tinc_hdr_rem), to_copy);
            (*tc).t_tinc_hdr_rem -= to_copy; left -= to_copy; offset += to_copy as u32;
            if (*tc).t_tinc_hdr_rem == 0 {
                (*tc).t_tinc_data_rem = be32_to_cpu((*tinc).ti_inc.i_hdr.h_len) as usize;
                (*tinc).ti_inc.i_rx_lat_trace[RDS_MSG_RX_START] = local_clock();
            }
        }
        if (*tc).t_tinc_data_rem != 0 {
            let to_copy = core::cmp::min((*tc).t_tinc_data_rem, left);
            let clone = pskb_extract(skb, offset, to_copy, (*arg).gfp);
            if clone.is_null() { (*desc).error = -ENOMEM; break; }
            skb_queue_tail(&mut (*tinc).ti_skb_list, clone);
            (*tc).t_tinc_data_rem -= to_copy; left -= to_copy; offset += to_copy as u32;
        }
        if (*tc).t_tinc_hdr_rem == 0 && (*tc).t_tinc_data_rem == 0 {
            let conn = (*cp).cp_conn;
            if (*tinc).ti_inc.i_hdr.h_flags == RDS_FLAG_CONG_BITMAP { rds_tcp_cong_recv(conn, tinc); }
            else { rds_recv_incoming(conn, &(*conn).c_faddr, &(*conn).c_laddr, &mut (*tinc).ti_inc, (*arg).gfp); }
            (*tc).t_tinc_hdr_rem = core::mem::size_of::<rds_header>();
            (*tc).t_tinc_data_rem = 0; (*tc).t_tinc = core::ptr::null_mut();
            rds_inc_put(&mut (*tinc).ti_inc); tinc = core::ptr::null_mut();
        }
    }
    len - left
}

unsafe fn rds_tcp_read_sock(cp: *mut rds_conn_path, gfp: gfp_t) -> i32 {
    let tc = (*cp).cp_transport_data as *mut rds_tcp_connection;
    let sock = (*tc).t_sock;
    let mut desc = read_descriptor_t { arg: read_descriptor_arg { data: &mut rds_tcp_desc_arg { conn_path: cp, gfp } }, error: 0, count: 1 };
    tcp_read_sock((*sock).sk, &mut desc, rds_tcp_data_recv);
    if skb_queue_empty_lockless(&(*(*sock).sk).sk_receive_queue) && wq_has_sleeper(&(*tc).t_recv_done_waitq) { wake_up(&(*tc).t_recv_done_waitq); }
    desc.error
}

pub unsafe fn rds_tcp_recv_path(cp: *mut rds_conn_path) -> i32 {
    let tc = (*cp).cp_transport_data as *mut rds_tcp_connection;
    let sock = (*tc).t_sock;
    lock_sock((*sock).sk); let ret = rds_tcp_read_sock(cp, GFP_KERNEL); release_sock((*sock).sk); ret
}

pub unsafe fn rds_tcp_data_ready(sk: *mut sock) {
    trace_sk_data_ready(sk); read_lock_bh(&mut (*sk).sk_callback_lock);
    let cp = (*sk).sk_user_data as *mut rds_conn_path;
    if cp.is_null() { read_unlock_bh(&mut (*sk).sk_callback_lock); ((*sk).sk_data_ready)(sk); return; }
    let tc = (*cp).cp_transport_data as *mut rds_tcp_connection;
    let ready = (*tc).t_orig_data_ready;
    rds_tcp_stats_inc!(s_tcp_data_ready_calls);
    if rds_tcp_read_sock(cp, GFP_ATOMIC) == -ENOMEM { rcu_read_lock(); if !rds_destroy_pending((*cp).cp_conn) { queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_recv_w, 0); } rcu_read_unlock(); }
    read_unlock_bh(&mut (*sk).sk_callback_lock); ready(sk);
}

pub unsafe fn rds_tcp_recv_init() -> i32 {
    rds_tcp_incoming_slab = KMEM_CACHE!(rds_tcp_incoming, 0);
    if rds_tcp_incoming_slab.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn rds_tcp_recv_exit() { kmem_cache_destroy(rds_tcp_incoming_slab); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
