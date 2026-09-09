// SPDX-License-Identifier: GPL-2.0
/* MPTCP Fast Open Mechanism
 *
 * Copyright (c) 2021-2022, Dmytro SHYTYI
 */

// Dependency declarations supplied by the protocol implementation.
use crate::protocol::*;

pub unsafe fn mptcp_fastopen_subflow_synack_set_params(
    subflow: *mut mptcp_subflow_context,
    req: *mut request_sock,
) {
    let sk: *mut sock;
    let ssk: *mut sock;
    let skb: *mut sk_buff;
    let tp: *mut tcp_sock;
    let has_rxtstamp: bool;

    /* on early fallback the subflow context is deleted by
     * subflow_syn_recv_sock()
     */
    if subflow.is_null() {
        return;
    }

    ssk = (*subflow).tcp_sock;
    sk = (*subflow).conn;
    tp = tcp_sk(ssk);

    /* A valid TFO cookie does not guarantee SYN data. */
    skb = skb_peek(&mut (*ssk).sk_receive_queue);
    if skb.is_null() {
        return;
    }

    (*subflow).is_mptfo = 1;

    /* dequeue the skb from sk receive queue */
    __skb_unlink(skb, &mut (*ssk).sk_receive_queue);
    skb_ext_reset(skb);

    mptcp_subflow_lend_fwdmem(subflow, skb);

    /* We copy the fastopen data, but that don't belong to the mptcp sequence
     * space, need to offset it in the subflow sequence, see mptcp_subflow_get_map_offset()
     */
    (*tp).copied_seq = (*tp).copied_seq.wrapping_add((*skb).len);
    (*subflow).ssn_offset = (*subflow).ssn_offset.wrapping_add((*skb).len);
    has_rxtstamp = (*TCP_SKB_CB(skb)).has_rxtstamp;

    /* Only the sequence delta is relevant */
    (*MPTCP_SKB_CB(skb)).map_seq = -((*skb).len as i64);
    (*MPTCP_SKB_CB(skb)).end_seq = 0;
    (*MPTCP_SKB_CB(skb)).offset = 0;
    (*MPTCP_SKB_CB(skb)).has_rxtstamp = has_rxtstamp;
    (*MPTCP_SKB_CB(skb)).cant_coalesce = 1;

    mptcp_data_lock(sk);
    DEBUG_NET_WARN_ON_ONCE(sock_owned_by_user_nocheck(sk));

    mptcp_borrow_fwdmem(sk, skb);
    skb_set_owner_r(skb, sk);
    __skb_queue_tail(&mut (*sk).sk_receive_queue, skb);
    (*mptcp_sk(sk)).bytes_received = (*mptcp_sk(sk))
        .bytes_received
        .wrapping_add((*skb).len);

    ((*sk).sk_data_ready)(sk);

    mptcp_data_unlock(sk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
