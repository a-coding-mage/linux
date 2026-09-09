// SPDX-License-Identifier: GPL-2.0-only
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2009-2013 VMware, Inc. All rights reserved.
 */

// C dependencies supplied by the surrounding translation unit.

macro_rules! pkt_field {
    ($vsk:expr, $field:ident) => {
        unsafe { (*vmci_trans($vsk)).notify.pkt.$field }
    };
}

unsafe fn vmci_transport_notify_waiting_write(vsk: *mut vsock_sock) -> bool {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    {
        let mut notify_limit: u64;
        if !pkt_field!(vsk, peer_waiting_write) { return false; }

        #[cfg(feature = "VSOCK_OPTIMIZATION_FLOW_CONTROL")]
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
        #[cfg(feature = "VSOCK_OPTIMIZATION_FLOW_CONTROL")]
        { notify_limit = (*vmci_trans(vsk)).consume_size - pkt_field!(vsk, write_notify_window); }
        #[cfg(not(feature = "VSOCK_OPTIMIZATION_FLOW_CONTROL"))]
        { notify_limit = 0; }
        let retval = vmci_qpair_consume_free_space((*vmci_trans(vsk)).qpair) > notify_limit;
        #[cfg(feature = "VSOCK_OPTIMIZATION_FLOW_CONTROL")]
        if retval { pkt_field!(vsk, peer_waiting_write_detected) = false; }
        retval
    }
    #[cfg(not(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY"))]
    { true }
}

unsafe fn vmci_transport_notify_waiting_read(vsk: *mut vsock_sock) -> bool {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { if !pkt_field!(vsk, peer_waiting_read) { return false; } vmci_qpair_produce_buf_ready((*vmci_trans(vsk)).qpair) > 0 }
    #[cfg(not(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY"))]
    { true }
}

unsafe fn vmci_transport_handle_waiting_read(sk: *mut sock, pkt: *mut vmci_transport_packet, bottom_half: bool, dst: *mut sockaddr_vm, src: *mut sockaddr_vm) {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { let vsk = vsock_sk(sk); pkt_field!(vsk, peer_waiting_read) = true; core::ptr::copy_nonoverlapping(&(*pkt).u.wait, &mut pkt_field!(vsk, peer_waiting_read_info), 1); if vmci_transport_notify_waiting_read(vsk) { let sent = if bottom_half { vmci_transport_send_wrote_bh(dst, src) > 0 } else { vmci_transport_send_wrote(sk) > 0 }; if sent { pkt_field!(vsk, peer_waiting_read) = false; } } }
}

unsafe fn vmci_transport_handle_waiting_write(sk: *mut sock, pkt: *mut vmci_transport_packet, bottom_half: bool, dst: *mut sockaddr_vm, src: *mut sockaddr_vm) {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { let vsk = vsock_sk(sk); pkt_field!(vsk, peer_waiting_write) = true; core::ptr::copy_nonoverlapping(&(*pkt).u.wait, &mut pkt_field!(vsk, peer_waiting_write_info), 1); if vmci_transport_notify_waiting_write(vsk) { let sent = if bottom_half { vmci_transport_send_read_bh(dst, src) > 0 } else { vmci_transport_send_read(sk) > 0 }; if sent { pkt_field!(vsk, peer_waiting_write) = false; } } }
}

unsafe fn vmci_transport_handle_read(sk: *mut sock, _pkt: *mut vmci_transport_packet, _bottom_half: bool, _dst: *mut sockaddr_vm, _src: *mut sockaddr_vm) {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { pkt_field!(vsock_sk(sk), sent_waiting_write) = false; }
    ((*sk).sk_write_space)(sk);
}

unsafe fn send_waiting_read(sk: *mut sock, room_needed: u64) -> bool {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { let vsk = vsock_sk(sk); if pkt_field!(vsk, sent_waiting_read) { return true; } if pkt_field!(vsk, write_notify_window) < (*vmci_trans(vsk)).consume_size { pkt_field!(vsk, write_notify_window) = core::cmp::min(pkt_field!(vsk, write_notify_window).wrapping_add(PAGE_SIZE), (*vmci_trans(vsk)).consume_size); } let (mut tail, mut head) = (0, 0); vmci_qpair_get_consume_indexes((*vmci_trans(vsk)).qpair, &mut tail, &mut head); let room_left = (*vmci_trans(vsk)).consume_size - head; let mut info = vmci_transport_waiting_info { offset: 0, generation: 0 }; if room_needed >= room_left { info.offset = room_needed - room_left; info.generation = pkt_field!(vsk, consume_q_generation).wrapping_add(1); } else { info.offset = head + room_needed; info.generation = pkt_field!(vsk, consume_q_generation); } let ret = vmci_transport_send_waiting_read(sk, &mut info) > 0; if ret { pkt_field!(vsk, sent_waiting_read) = true; } ret }
    #[cfg(not(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY"))]
    { true }
}

unsafe fn send_waiting_write(sk: *mut sock, room_needed: u64) -> bool {
    #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")]
    { let vsk = vsock_sk(sk); if pkt_field!(vsk, sent_waiting_write) { return true; } let (mut tail, mut head) = (0, 0); vmci_qpair_get_produce_indexes((*vmci_trans(vsk)).qpair, &mut tail, &mut head); let room_left = (*vmci_trans(vsk)).produce_size - tail; let mut info = vmci_transport_waiting_info { offset: 0, generation: 0 }; if room_needed + 1 >= room_left { info.offset = room_needed + 1 - room_left; info.generation = pkt_field!(vsk, produce_q_generation); } else { info.offset = tail + room_needed + 1; info.generation = pkt_field!(vsk, produce_q_generation).wrapping_sub(1); } let ret = vmci_transport_send_waiting_write(sk, &mut info) > 0; if ret { pkt_field!(vsk, sent_waiting_write) = true; } ret }
    #[cfg(not(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY"))]
    { true }
}

unsafe fn vmci_transport_send_read_notification(sk: *mut sock) -> i32 { let vsk = vsock_sk(sk); let mut sent_read = false; let mut retries = 0; let mut err = 0; if vmci_transport_notify_waiting_write(vsk) { while ((*vsk).peer_shutdown & RCV_SHUTDOWN) == 0 && !sent_read && retries < VMCI_TRANSPORT_MAX_DGRAM_RESENDS { err = vmci_transport_send_read(sk); if err >= 0 { sent_read = true; } retries += 1; } if retries >= VMCI_TRANSPORT_MAX_DGRAM_RESENDS { pr_err!("%p unable to send read notify to peer\n", sk); } else { #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")] { pkt_field!(vsk, peer_waiting_write) = false; } } } err }

unsafe fn vmci_transport_handle_wrote(sk: *mut sock, _pkt: *mut vmci_transport_packet, _bottom_half: bool, _dst: *mut sockaddr_vm, _src: *mut sockaddr_vm) { #[cfg(feature = "VSOCK_OPTIMIZATION_WAITING_NOTIFY")] { pkt_field!(vsock_sk(sk), sent_waiting_read) = false; } vsock_data_ready(sk); }

unsafe fn vmci_transport_notify_pkt_socket_init(sk: *mut sock) { let vsk = vsock_sk(sk); pkt_field!(vsk, write_notify_window) = PAGE_SIZE; pkt_field!(vsk, write_notify_min_window) = PAGE_SIZE; pkt_field!(vsk, peer_waiting_read) = false; pkt_field!(vsk, peer_waiting_write) = false; pkt_field!(vsk, peer_waiting_write_detected) = false; pkt_field!(vsk, sent_waiting_read) = false; pkt_field!(vsk, sent_waiting_write) = false; pkt_field!(vsk, produce_q_generation) = 0; pkt_field!(vsk, consume_q_generation) = 0; core::ptr::write_bytes(&mut pkt_field!(vsk, peer_waiting_read_info), 0, 1); core::ptr::write_bytes(&mut pkt_field!(vsk, peer_waiting_write_info), 0, 1); }
unsafe fn vmci_transport_notify_pkt_socket_destruct(_vsk: *mut vsock_sock) {}

unsafe fn vmci_transport_notify_pkt_poll_in(sk: *mut sock, target: usize, ready: *mut bool) -> i32 { let vsk=vsock_sk(sk); if vsock_stream_has_data(vsk)>=target { *ready=true; } else { if (*sk).sk_state==TCP_ESTABLISHED && !send_waiting_read(sk,1) { return -1; } *ready=false; } 0 }
unsafe fn vmci_transport_notify_pkt_poll_out(sk: *mut sock, _target: usize, avail: *mut bool) -> i32 { let n=vsock_stream_has_space(vsock_sk(sk)); if n>0 { *avail=true; 0 } else if n==0 { if !send_waiting_write(sk,1) { return -1; } *avail=false; 0 } else { 0 } }
unsafe fn vmci_transport_notify_pkt_recv_init(_sk:*mut sock, _target:usize, data:*mut vmci_transport_recv_notify_data)->i32 { #[cfg(feature="VSOCK_OPTIMIZATION_WAITING_NOTIFY")] { (*data).consume_head=0; (*data).produce_tail=0; } 0 }
unsafe fn vmci_transport_notify_pkt_recv_pre_block(sk:*mut sock,target:usize,_data:*mut vmci_transport_recv_notify_data)->i32 { if !send_waiting_read(sk,target) {-EHOSTUNREACH} else {0} }
unsafe fn vmci_transport_notify_pkt_recv_pre_dequeue(sk:*mut sock,_target:usize,data:*mut vmci_transport_recv_notify_data)->i32 { #[cfg(feature="VSOCK_OPTIMIZATION_WAITING_NOTIFY")] { vmci_qpair_get_consume_indexes((*vmci_trans(vsock_sk(sk))).qpair,&mut (*data).produce_tail,&mut (*data).consume_head); } 0 }
unsafe fn vmci_transport_notify_pkt_recv_post_dequeue(sk:*mut sock,_target:usize,copied:isize,data_read:bool,data:*mut vmci_transport_recv_notify_data)->i32 { if data_read { #[cfg(feature="VSOCK_OPTIMIZATION_WAITING_NOTIFY")] if copied as u64 >= (*vmci_trans(vsock_sk(sk))).consume_size-(*data).consume_head { pkt_field!(vsock_sk(sk),consume_q_generation)+=1; } let e=vmci_transport_send_read_notification(sk); if e<0 {e} else {0} } else {0} }
unsafe fn vmci_transport_notify_pkt_send_init(_sk:*mut sock,data:*mut vmci_transport_send_notify_data)->i32 { #[cfg(feature="VSOCK_OPTIMIZATION_WAITING_NOTIFY")] {(*data).consume_head=0;(*data).produce_tail=0;} 0 }
unsafe fn vmci_transport_notify_pkt_send_pre_block(sk:*mut sock,_data:*mut vmci_transport_send_notify_data)->i32 {if send_waiting_write(sk,1){0}else{-EHOSTUNREACH}}
unsafe fn vmci_transport_notify_pkt_send_pre_enqueue(sk:*mut sock,data:*mut vmci_transport_send_notify_data)->i32 {#[cfg(feature="VSOCK_OPTIMIZATION_WAITING_NOTIFY")] {vmci_qpair_get_produce_indexes((*vmci_trans(vsock_sk(sk))).qpair,&mut (*data).produce_tail,&mut (*data).consume_head);} 0}
unsafe fn vmci_transport_notify_pkt_send_post_enqueue(sk:*mut sock,_written:isize,_data:*mut vmci_transport_send_notify_data)->i32 {if vmci_transport_notify_waiting_read(vsock_sk(sk)){let e=vmci_transport_send_wrote(sk);if e<0{e}else{0}}else{0}}
unsafe fn vmci_transport_notify_pkt_handle_pkt(sk:*mut sock,pkt:*mut vmci_transport_packet,b:bool,d:*mut sockaddr_vm,s:*mut sockaddr_vm,done:*mut bool){*done=true;match (*pkt).type_{VMCI_TRANSPORT_PACKET_TYPE_WROTE=>vmci_transport_handle_wrote(sk,pkt,b,d,s),VMCI_TRANSPORT_PACKET_TYPE_READ=>vmci_transport_handle_read(sk,pkt,b,d,s),VMCI_TRANSPORT_PACKET_TYPE_WAITING_WRITE=>vmci_transport_handle_waiting_write(sk,pkt,b,d,s),VMCI_TRANSPORT_PACKET_TYPE_WAITING_READ=>vmci_transport_handle_waiting_read(sk,pkt,b,d,s),_=>*done=false}}
unsafe fn vmci_transport_notify_pkt_process_request(sk:*mut sock){pkt_field!(vsock_sk(sk),write_notify_window)=(*vmci_trans(vsock_sk(sk))).consume_size;}
unsafe fn vmci_transport_notify_pkt_process_negotiate(sk:*mut sock){vmci_transport_notify_pkt_process_request(sk)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
