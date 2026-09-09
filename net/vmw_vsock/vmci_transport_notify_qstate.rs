// SPDX-License-Identifier: GPL-2.0-only
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2009-2013 VMware, Inc. All rights reserved.
 */

// C dependencies supplied by the surrounding translation unit.

macro_rules! pkt_field {
    ($vsk:expr, $field:ident) => {
        unsafe { (*vmci_trans($vsk)).notify.pkt_q_state.$field }
    };
}

unsafe fn vmci_transport_notify_waiting_write(vsk: *mut vsock_sock) -> bool {
    let mut notify_limit: u64;

    if !pkt_field!(vsk, peer_waiting_write) {
        return false;
    }

    /* When the sender blocks, we take that as a sign that the sender is
     * faster than the receiver. To reduce the transmit rate of the sender,
     * we delay the sending of the read notification by decreasing the
     * write_notify_window. The notification is delayed until the number of
     * bytes used in the queue drops below the write_notify_window.
     */
    if !pkt_field!(vsk, peer_waiting_write_detected) {
        pkt_field!(vsk, peer_waiting_write_detected) = true;
        if pkt_field!(vsk, write_notify_window) < PAGE_SIZE {
            pkt_field!(vsk, write_notify_window) = pkt_field!(vsk, write_notify_min_window);
        } else {
            pkt_field!(vsk, write_notify_window) -= PAGE_SIZE;
            if pkt_field!(vsk, write_notify_window) < pkt_field!(vsk, write_notify_min_window) {
                pkt_field!(vsk, write_notify_window) = pkt_field!(vsk, write_notify_min_window);
            }
        }
    }
    notify_limit = (*vmci_trans(vsk)).consume_size - pkt_field!(vsk, write_notify_window);

    /* The notify_limit is used to delay notifications in the case where flow
     * control is enabled. Below the test is expressed in terms of free space
     * in the queue: if free_space > ConsumeSize - write_notify_window then
     * notify. An alternate way of expressing this is to rewrite the expression
     * to use the data ready in the receive queue.
     */
    let retval = vmci_qpair_consume_free_space((*vmci_trans(vsk)).qpair) > notify_limit;
    if retval {
        /* Once we notify the peer, we reset the detected flag so the next wait
         * will again cause a decrease in the window size.
         */
        pkt_field!(vsk, peer_waiting_write_detected) = false;
    }
    retval
}

unsafe fn vmci_transport_handle_read(sk: *mut sock, _pkt: *mut vmci_transport_packet, _bottom_half: bool, _dst: *mut sockaddr_vm, _src: *mut sockaddr_vm) {
    ((*sk).sk_write_space)(sk);
}

unsafe fn vmci_transport_handle_wrote(sk: *mut sock, _pkt: *mut vmci_transport_packet, _bottom_half: bool, _dst: *mut sockaddr_vm, _src: *mut sockaddr_vm) {
    vsock_data_ready(sk);
}

unsafe fn vsock_block_update_write_window(sk: *mut sock) {
    let vsk = vsock_sk(sk);
    if pkt_field!(vsk, write_notify_window) < (*vmci_trans(vsk)).consume_size {
        pkt_field!(vsk, write_notify_window) = core::cmp::min(pkt_field!(vsk, write_notify_window) + PAGE_SIZE, (*vmci_trans(vsk)).consume_size);
    }
}

unsafe fn vmci_transport_send_read_notification(sk: *mut sock) -> i32 {
    let vsk = vsock_sk(sk);
    let mut sent_read = false;
    let mut retries: u32 = 0;
    let mut err = 0;
    if vmci_transport_notify_waiting_write(vsk) {
        while ((*vsk).peer_shutdown & RCV_SHUTDOWN) == 0 && !sent_read && retries < VMCI_TRANSPORT_MAX_DGRAM_RESENDS {
            err = vmci_transport_send_read(sk);
            if err >= 0 { sent_read = true; }
            retries += 1;
        }
        if retries >= VMCI_TRANSPORT_MAX_DGRAM_RESENDS && !sent_read {
            pr_err!("%p unable to send read notification to peer\n", sk);
        } else {
            pkt_field!(vsk, peer_waiting_write) = false;
        }
    }
    err
}

unsafe fn vmci_transport_notify_pkt_socket_init(sk: *mut sock) {
    let vsk = vsock_sk(sk);
    pkt_field!(vsk, write_notify_window) = PAGE_SIZE;
    pkt_field!(vsk, write_notify_min_window) = PAGE_SIZE;
    pkt_field!(vsk, peer_waiting_write) = false;
    pkt_field!(vsk, peer_waiting_write_detected) = false;
}

unsafe fn vmci_transport_notify_pkt_socket_destruct(vsk: *mut vsock_sock) {
    pkt_field!(vsk, write_notify_window) = PAGE_SIZE;
    pkt_field!(vsk, write_notify_min_window) = PAGE_SIZE;
    pkt_field!(vsk, peer_waiting_write) = false;
    pkt_field!(vsk, peer_waiting_write_detected) = false;
}

unsafe fn vmci_transport_notify_pkt_poll_in(sk: *mut sock, target: usize, data_ready_now: *mut bool) -> i32 {
    let vsk = vsock_sk(sk);
    if vsock_stream_has_data(vsk) >= target as _ { *data_ready_now = true; }
    else { if (*sk).sk_state == TCP_ESTABLISHED { vsock_block_update_write_window(sk); } *data_ready_now = false; }
    0
}

unsafe fn vmci_transport_notify_pkt_poll_out(sk: *mut sock, _target: usize, space_avail_now: *mut bool) -> i32 {
    let free = vsock_stream_has_space(vsock_sk(sk));
    if free > 0 { *space_avail_now = true; return 0; }
    if free == 0 { *space_avail_now = false; }
    0
}

unsafe fn vmci_transport_notify_pkt_recv_init(sk: *mut sock, target: usize, data: *mut vmci_transport_recv_notify_data) -> i32 {
    let vsk = vsock_sk(sk);
    (*data).consume_head = 0; (*data).produce_tail = 0; (*data).notify_on_block = false;
    if pkt_field!(vsk, write_notify_min_window) < target + 1 {
        pkt_field!(vsk, write_notify_min_window) = target + 1;
        if pkt_field!(vsk, write_notify_window) < pkt_field!(vsk, write_notify_min_window) {
            pkt_field!(vsk, write_notify_window) = pkt_field!(vsk, write_notify_min_window);
            (*data).notify_on_block = true;
        }
    }
    0
}

unsafe fn vmci_transport_notify_pkt_recv_pre_block(sk: *mut sock, _target: usize, data: *mut vmci_transport_recv_notify_data) -> i32 {
    vsock_block_update_write_window(sk);
    if (*data).notify_on_block { let err = vmci_transport_send_read_notification(sk); if err < 0 { return err; } (*data).notify_on_block = false; }
    0
}

unsafe fn vmci_transport_notify_pkt_recv_post_dequeue(sk: *mut sock, _target: usize, copied: isize, data_read: bool, _data: *mut vmci_transport_recv_notify_data) -> i32 {
    if data_read {
        smp_mb();
        let vsk = vsock_sk(sk);
        let free_space = vmci_qpair_consume_free_space((*vmci_trans(vsk)).qpair);
        if free_space == copied as u64 { pkt_field!(vsk, peer_waiting_write) = true; }
        let err = vmci_transport_send_read_notification(sk); if err < 0 { return err; }
        vsock_data_ready(sk);
    }
    0
}

unsafe fn vmci_transport_notify_pkt_send_init(_sk: *mut sock, data: *mut vmci_transport_send_notify_data) -> i32 {
    (*data).consume_head = 0; (*data).produce_tail = 0; 0
}

unsafe fn vmci_transport_notify_pkt_send_post_enqueue(sk: *mut sock, written: isize, _data: *mut vmci_transport_send_notify_data) -> i32 {
    smp_mb();
    let vsk = vsock_sk(sk);
    let was_empty = vmci_qpair_produce_buf_ready((*vmci_trans(vsk)).qpair) == written as u64;
    let mut err = 0;
    let mut sent_wrote = false;
    let mut retries = 0;
    if was_empty { while ((*vsk).peer_shutdown & RCV_SHUTDOWN) == 0 && !sent_wrote && retries < VMCI_TRANSPORT_MAX_DGRAM_RESENDS { err = vmci_transport_send_wrote(sk); if err >= 0 { sent_wrote = true; } retries += 1; } }
    if retries >= VMCI_TRANSPORT_MAX_DGRAM_RESENDS && !sent_wrote { pr_err!("%p unable to send wrote notification to peer\n", sk); }
    err
}

unsafe fn vmci_transport_notify_pkt_handle_pkt(sk: *mut sock, pkt: *mut vmci_transport_packet, bottom_half: bool, dst: *mut sockaddr_vm, src: *mut sockaddr_vm, pkt_processed: *mut bool) {
    let mut processed = false;
    match (*pkt).type_ {
        VMCI_TRANSPORT_PACKET_TYPE_WROTE => { vmci_transport_handle_wrote(sk, pkt, bottom_half, dst, src); processed = true; }
        VMCI_TRANSPORT_PACKET_TYPE_READ => { vmci_transport_handle_read(sk, pkt, bottom_half, dst, src); processed = true; }
        _ => {}
    }
    if !pkt_processed.is_null() { *pkt_processed = processed; }
}

unsafe fn vmci_transport_notify_pkt_process_request(sk: *mut sock) { let vsk = vsock_sk(sk); pkt_field!(vsk, write_notify_window) = (*vmci_trans(vsk)).consume_size; if (*vmci_trans(vsk)).consume_size < pkt_field!(vsk, write_notify_min_window) { pkt_field!(vsk, write_notify_min_window) = (*vmci_trans(vsk)).consume_size; } }
unsafe fn vmci_transport_notify_pkt_process_negotiate(sk: *mut sock) { vmci_transport_notify_pkt_process_request(sk); }
unsafe fn vmci_transport_notify_pkt_recv_pre_dequeue(_sk: *mut sock, _target: usize, _data: *mut vmci_transport_recv_notify_data) -> i32 { 0 /* NOP for QState. */ }
unsafe fn vmci_transport_notify_pkt_send_pre_block(_sk: *mut sock, _data: *mut vmci_transport_send_notify_data) -> i32 { 0 /* NOP for QState. */ }
unsafe fn vmci_transport_notify_pkt_send_pre_enqueue(_sk: *mut sock, _data: *mut vmci_transport_send_notify_data) -> i32 { 0 /* NOP for QState. */ }

/* Socket always on control packet based operations. */
const VMCI_TRANSPORT_NOTIFY_PKT_Q_STATE_OPS: vmci_transport_notify_ops = vmci_transport_notify_ops {
    socket_init: Some(vmci_transport_notify_pkt_socket_init), socket_destruct: Some(vmci_transport_notify_pkt_socket_destruct),
    poll_in: Some(vmci_transport_notify_pkt_poll_in), poll_out: Some(vmci_transport_notify_pkt_poll_out),
    handle_notify_pkt: Some(vmci_transport_notify_pkt_handle_pkt), recv_init: Some(vmci_transport_notify_pkt_recv_init),
    recv_pre_block: Some(vmci_transport_notify_pkt_recv_pre_block), recv_pre_dequeue: Some(vmci_transport_notify_pkt_recv_pre_dequeue),
    recv_post_dequeue: Some(vmci_transport_notify_pkt_recv_post_dequeue), send_init: Some(vmci_transport_notify_pkt_send_init),
    send_pre_block: Some(vmci_transport_notify_pkt_send_pre_block), send_pre_enqueue: Some(vmci_transport_notify_pkt_send_pre_enqueue),
    send_post_enqueue: Some(vmci_transport_notify_pkt_send_post_enqueue), process_request: Some(vmci_transport_notify_pkt_process_request),
    process_negotiate: Some(vmci_transport_notify_pkt_process_negotiate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
