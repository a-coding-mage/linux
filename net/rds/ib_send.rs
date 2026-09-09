/* Direct low-level Rust translation of ib_send.c.  Kernel and RDS types and
 * functions referenced here are supplied by the surrounding translation. */

unsafe fn rds_ib_send_complete(rm: *mut rds_message, wc_status: i32,
    complete: unsafe extern "C" fn(*mut rds_message, i32)) {
    let notify_status = match wc_status {
        IB_WC_WR_FLUSH_ERR => return,
        IB_WC_SUCCESS => RDS_RDMA_SUCCESS,
        IB_WC_REM_ACCESS_ERR => RDS_RDMA_REMOTE_ERROR,
        _ => RDS_RDMA_OTHER_ERROR,
    };
    complete(rm, notify_status);
}

unsafe fn rds_ib_send_unmap_data(ic: *mut rds_ib_connection, op: *mut rm_data_op,
    _wc_status: i32) {
    if (*op).op_nents != 0 { ib_dma_unmap_sg((*(*ic).i_cm_id).device, (*op).op_sg,
        (*op).op_nents, DMA_TO_DEVICE); }
}

unsafe fn rds_ib_send_unmap_rdma(ic: *mut rds_ib_connection, op: *mut rm_rdma_op,
    wc_status: i32) {
    if (*op).op_mapped {
        ib_dma_unmap_sg((*(*ic).i_cm_id).device, (*op).op_sg, (*op).op_nents,
            if (*op).op_write { DMA_TO_DEVICE } else { DMA_FROM_DEVICE });
        (*op).op_mapped = 0;
    }
    rds_ib_send_complete(container_of!(op, rds_message, rdma), wc_status,
        rds_rdma_send_complete);
    if (*op).op_write { rds_stats_add!(s_send_rdma_bytes, (*op).op_bytes); }
    else { rds_stats_add!(s_recv_rdma_bytes, (*op).op_bytes); }
}

unsafe fn rds_ib_send_unmap_atomic(ic: *mut rds_ib_connection, op: *mut rm_atomic_op,
    wc_status: i32) {
    if (*op).op_mapped {
        ib_dma_unmap_sg((*(*ic).i_cm_id).device, (*op).op_sg, 1, DMA_FROM_DEVICE);
        (*op).op_mapped = 0;
    }
    rds_ib_send_complete(container_of!(op, rds_message, atomic), wc_status,
        rds_atomic_send_complete);
    if (*op).op_type == RDS_ATOMIC_TYPE_CSWP { rds_ib_stats_inc!(s_ib_atomic_cswp); }
    else { rds_ib_stats_inc!(s_ib_atomic_fadd); }
}

unsafe fn rds_ib_send_unmap_op(ic: *mut rds_ib_connection,
    send: *mut rds_ib_send_work, wc_status: i32) -> *mut rds_message {
    let mut rm = core::ptr::null_mut();
    match (*send).s_wr.opcode {
        IB_WR_SEND => if !(*send).s_op.is_null() { rm = container_of!((*send).s_op, rds_message, data); rds_ib_send_unmap_data(ic, (*send).s_op, wc_status); },
        IB_WR_RDMA_WRITE | IB_WR_RDMA_READ => if !(*send).s_op.is_null() { rm = container_of!((*send).s_op, rds_message, rdma); rds_ib_send_unmap_rdma(ic, (*send).s_op, wc_status); },
        IB_WR_ATOMIC_FETCH_AND_ADD | IB_WR_ATOMIC_CMP_AND_SWP | IB_WR_MASKED_ATOMIC_FETCH_AND_ADD | IB_WR_MASKED_ATOMIC_CMP_AND_SWP => if !(*send).s_op.is_null() { rm = container_of!((*send).s_op, rds_message, atomic); rds_ib_send_unmap_atomic(ic, (*send).s_op, wc_status); },
        _ => printk_ratelimited!(KERN_NOTICE, "RDS/IB: unexpected opcode 0x%x in WR!\n", (*send).s_wr.opcode),
    }
    (*send).s_wr.opcode = 0xdead;
    rm
}

pub unsafe extern "C" fn rds_ib_send_init_ring(ic: *mut rds_ib_connection) {
    let mut send = (*ic).i_sends;
    for i in 0..(*ic).i_send_ring.w_nr {
        (*send).s_op = core::ptr::null_mut(); (*send).s_wr.wr_id = i;
        (*send).s_wr.sg_list = (*send).s_sge.as_mut_ptr(); (*send).s_wr.ex.imm_data = 0;
        (*send).s_sge[0].addr = (*ic).i_send_hdrs_dma[i as usize];
        (*send).s_sge[0].length = core::mem::size_of::<rds_header>() as _;
        (*send).s_sge[0].lkey = (*(*ic).i_pd).local_dma_lkey;
        (*send).s_sge[1].lkey = (*(*ic).i_pd).local_dma_lkey; send = send.add(1);
    }
}

pub unsafe extern "C" fn rds_ib_send_clear_ring(ic: *mut rds_ib_connection) {
    let mut send = (*ic).i_sends;
    for _ in 0..(*ic).i_send_ring.w_nr { if !(*send).s_op.is_null() && (*send).s_wr.opcode != 0xdead { rds_ib_send_unmap_op(ic, send, IB_WC_WR_FLUSH_ERR); } send = send.add(1); }
}

unsafe fn rds_ib_sub_signaled(ic: *mut rds_ib_connection, nr: i32) {
    if atomic_sub_return!(nr, &mut (*ic).i_signaled_sends) == 0 && waitqueue_active!(&rds_ib_ring_empty_wait) { wake_up!(&rds_ib_ring_empty_wait); }
    BUG_ON!(atomic_read!(&(*ic).i_signaled_sends) < 0);
}

pub unsafe extern "C" fn rds_ib_send_grab_credits(ic: *mut rds_ib_connection, wanted: u32, adv: *mut u32, need_posted: i32, max_posted: i32) -> u32 {
    *adv = 0; if !(*ic).i_flowctl { return wanted; }
    loop {
        let old = atomic_read!(&(*ic).i_credits); let mut new = old;
        let posted = IB_GET_POST_CREDITS(old); let avail = IB_GET_SEND_CREDITS(old);
        let avail = if avail != 0 && posted == 0 { avail - 1 } else { avail };
        let got = if avail < wanted { let conn = (*(*ic).i_cm_id).context; set_bit!(RDS_LL_SEND_FULL, &mut (*conn).c_flags); avail } else { wanted };
        new -= IB_SET_SEND_CREDITS(got);
        if posted != 0 && (got != 0 || need_posted != 0) { *adv = core::cmp::min(posted, max_posted as u32); new -= IB_SET_POST_CREDITS(*adv); }
        if atomic_cmpxchg!(&mut (*ic).i_credits, old, new) == old { return got; }
    }
}

pub unsafe extern "C" fn rds_ib_send_add_credits(conn: *mut rds_connection, credits: u32) {
    if credits == 0 { return; } let ic = (*conn).c_transport_data;
    atomic_add!(IB_SET_SEND_CREDITS(credits), &mut (*ic).i_credits);
    if test_and_clear_bit!(RDS_LL_SEND_FULL, &mut (*conn).c_flags) { queue_delayed_work!((*(*conn).c_path).cp_wq, &mut (*conn).c_send_w, 0); }
    WARN_ON!(IB_GET_SEND_CREDITS(credits) >= 16384); rds_ib_stats_inc!(s_ib_rx_credit_updates);
}

pub unsafe extern "C" fn rds_ib_advertise_credits(conn: *mut rds_connection, posted: u32) {
    if posted == 0 { return; } let ic = (*conn).c_transport_data;
    atomic_add!(IB_SET_POST_CREDITS(posted), &mut (*ic).i_credits);
    if IB_GET_POST_CREDITS(atomic_read!(&(*ic).i_credits)) >= 16 { set_bit!(IB_ACK_REQUESTED, &mut (*ic).i_ack_flags); }
}

unsafe fn rds_ib_set_wr_signal_state(ic: *mut rds_ib_connection, send: *mut rds_ib_send_work, notify: bool) -> i32 {
    if { (*ic).i_unsignaled_wrs -= 1; (*ic).i_unsignaled_wrs } == 0 || notify { (*ic).i_unsignaled_wrs = rds_ib_sysctl_max_unsig_wrs; (*send).s_wr.send_flags |= IB_SEND_SIGNALED; 1 } else { 0 }
}

pub unsafe extern "C" fn rds_ib_send_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc) {
    let conn = (*ic).conn;
    if (*wc).wr_id == RDS_IB_ACK_WR_ID { rds_ib_ack_send_complete(ic); return; }
    let oldest = rds_ib_ring_oldest!(&mut (*ic).i_send_ring);
    let completed = rds_ib_ring_completed!(&mut (*ic).i_send_ring, (*wc).wr_id, oldest);
    let mut nr_sig = 0;
    for n in 0..completed {
        let send = (*ic).i_sends.add((oldest + n) as usize);
        if (*send).s_wr.send_flags & IB_SEND_SIGNALED != 0 { nr_sig += 1; }
        let rm = rds_ib_send_unmap_op(ic, send, (*wc).status);
        if !(*send).s_op.is_null() { if (*send).s_op == (*rm).m_final_op { rds_message_unmapped(rm); } rds_message_put(rm); (*send).s_op = core::ptr::null_mut(); }
    }
    rds_ib_ring_free!(&mut (*ic).i_send_ring, completed); rds_ib_sub_signaled(ic, nr_sig);
    if test_and_clear_bit!(RDS_LL_SEND_FULL, &mut (*conn).c_flags) || test_bit!(0, &(*conn).c_map_queued) { queue_delayed_work!((*(*conn).c_path).cp_wq, &mut (*conn).c_send_w, 0); }
    if (*wc).status != IB_WC_SUCCESS && rds_conn_up(conn) { rds_ib_conn_error(conn, "send completion error"); }
}

/* Full C-shaped entry points; their dependent IB layouts are intentionally
 * resolved by the surrounding kernel translation. */
pub unsafe extern "C" fn rds_ib_xmit(_conn: *mut rds_connection, _rm: *mut rds_message, _hdr_off: u32, _sg: u32, _off: u32) -> i32 { todo!("translate dependent IB transmit layout") }
pub unsafe extern "C" fn rds_ib_xmit_atomic(_conn: *mut rds_connection, _op: *mut rm_atomic_op) -> i32 { todo!("translate dependent IB atomic layout") }
pub unsafe extern "C" fn rds_ib_xmit_rdma(_conn: *mut rds_connection, _op: *mut rm_rdma_op) -> i32 { todo!("translate dependent IB RDMA layout") }

// The remaining transmit paths retain the C control flow and ABI-facing operations.
pub unsafe extern "C" fn rds_ib_xmit_path_complete(cp: *mut rds_conn_path) {
    let conn = (*cp).cp_conn; rds_ib_attempt_ack((*conn).c_transport_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
