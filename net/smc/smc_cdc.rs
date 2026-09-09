// SPDX-License-Identifier: GPL-2.0
/* Shared Memory Communications over RDMA (SMC-R) and RoCE */

use core::ffi::c_void;

/* External kernel/project types and functions are supplied by the surrounding translation. */

unsafe fn smc_cdc_tx_handler(pnd_snd: *mut smc_wr_tx_pend_priv, _link: *mut smc_link, wc_status: ib_wc_status) {
    let cdcpend = pnd_snd as *mut smc_cdc_tx_pend;
    let conn = (*cdcpend).conn;
    let sndbuf_desc = (*conn).sndbuf_desc;
    let smc = container_of(conn, smc_sock_conn, conn);
    bh_lock_sock(&mut (*smc).sk);
    if wc_status == 0 && !sndbuf_desc.is_null() {
        let diff = smc_curs_diff((*sndbuf_desc).len, &(*conn).tx_curs_fin, &(*cdcpend).cursor);
        smp_mb__before_atomic();
        atomic_add(diff, &mut (*conn).sndbuf_space);
        smp_mb__after_atomic();
        smc_curs_copy(&mut (*conn).tx_curs_fin, &(*cdcpend).cursor, conn);
        smc_curs_copy(&mut (*conn).local_tx_ctrl_fin, &(*cdcpend).p_cursor, conn);
        (*conn).tx_cdc_seq_fin = (*cdcpend).ctrl_seq;
    }
    if atomic_dec_and_test(&mut (*conn).cdc_pend_tx_wr) {
        if sock_owned_by_user(&(*smc).sk) { (*conn).tx_in_release_sock = true; } else { smc_tx_pending(conn); }
        if wq_has_sleeper(&(*conn).cdc_pend_tx_wq) { wake_up(&mut (*conn).cdc_pend_tx_wq); }
    }
    warn_on(atomic_read(&(*conn).cdc_pend_tx_wr) < 0);
    smc_tx_sndbuf_nonfull(smc);
    bh_unlock_sock(&mut (*smc).sk);
}

pub unsafe fn smc_cdc_get_free_slot(conn: *mut smc_connection, link: *mut smc_link, wr_buf: *mut *mut smc_wr_buf, wr_rdma_buf: *mut *mut smc_rdma_wr, pend: *mut *mut smc_cdc_tx_pend) -> i32 {
    let mut rc = smc_wr_tx_get_free_slot(link, smc_cdc_tx_handler, wr_buf, wr_rdma_buf, pend as *mut *mut smc_wr_tx_pend_priv);
    if (*conn).killed { if rc == 0 { smc_wr_tx_put_slot(link, *pend as *mut smc_wr_tx_pend_priv); } rc = -EPIPE; }
    rc
}

unsafe fn smc_cdc_add_pending_send(conn: *mut smc_connection, pend: *mut smc_cdc_tx_pend) {
    (*pend).conn = conn;
    (*pend).cursor = (*conn).tx_curs_sent;
    (*pend).p_cursor = (*conn).local_tx_ctrl.prod;
    (*pend).ctrl_seq = (*conn).tx_cdc_seq;
}

pub unsafe fn smc_cdc_msg_send(conn: *mut smc_connection, wr_buf: *mut smc_wr_buf, pend: *mut smc_cdc_tx_pend) -> i32 {
    let link = (*conn).lnk;
    let mut cfed: smc_host_cursor = core::mem::zeroed();
    smc_cdc_add_pending_send(conn, pend);
    (*conn).tx_cdc_seq += 1;
    (*conn).local_tx_ctrl.seqno = (*conn).tx_cdc_seq;
    smc_host_msg_to_cdc(wr_buf as *mut smc_cdc_msg, conn, &mut cfed);
    atomic_inc(&mut (*conn).cdc_pend_tx_wr); smp_mb__after_atomic();
    let rc = smc_wr_tx_send(link, pend as *mut smc_wr_tx_pend_priv);
    if rc == 0 { smc_curs_copy(&mut (*conn).rx_curs_confirmed, &cfed, conn); (*conn).local_rx_ctrl.prod_flags.cons_curs_upd_req = 0; }
    else { (*conn).tx_cdc_seq -= 1; (*conn).local_tx_ctrl.seqno = (*conn).tx_cdc_seq; atomic_dec(&mut (*conn).cdc_pend_tx_wr); }
    rc
}

pub unsafe fn smcr_cdc_msg_send_validation(conn: *mut smc_connection, pend: *mut smc_cdc_tx_pend, wr_buf: *mut smc_wr_buf) -> i32 {
    let local = &mut (*conn).local_tx_ctrl; let link = (*conn).lnk; let peer = wr_buf as *mut smc_cdc_msg;
    (*peer).common.type_ = local.common.type_; (*peer).len = local.len; (*peer).seqno = htons((*conn).tx_cdc_seq_fin); (*peer).token = htonl(local.token); (*peer).prod_flags.failover_validation = 1;
    smc_cdc_add_pending_send(conn, pend); atomic_inc(&mut (*conn).cdc_pend_tx_wr); smp_mb__after_atomic();
    let rc = smc_wr_tx_send(link, pend as *mut smc_wr_tx_pend_priv); if rc != 0 { atomic_dec(&mut (*conn).cdc_pend_tx_wr); } rc
}

unsafe fn smcr_cdc_get_slot_and_msg_send(conn: *mut smc_connection) -> i32 {
    let mut again = false;
    loop {
        let link = (*conn).lnk; if !smc_wr_tx_link_hold(link) { return -ENOLINK; }
        let mut wr_buf = core::ptr::null_mut(); let mut pend = core::ptr::null_mut();
        let rc = smc_cdc_get_free_slot(conn, link, &mut wr_buf, core::ptr::null_mut(), &mut pend);
        if rc != 0 { smc_wr_tx_link_put(link); return rc; }
        spin_lock_bh(&mut (*conn).send_lock);
        if link != (*conn).lnk { spin_unlock_bh(&mut (*conn).send_lock); smc_wr_tx_put_slot(link, pend as *mut smc_wr_tx_pend_priv); smc_wr_tx_link_put(link); if again { return -ENOLINK; } again = true; continue; }
        let rc = smc_cdc_msg_send(conn, wr_buf, pend); spin_unlock_bh(&mut (*conn).send_lock); smc_wr_tx_link_put(link); return rc;
    }
}

pub unsafe fn smc_cdc_get_slot_and_msg_send(conn: *mut smc_connection) -> i32 {
    if !smc_conn_lgr_valid(conn) || ((*conn).lgr.is_null() == false && (*(*conn).lgr).is_smcd && (*(*conn).lgr).peer_shutdown) { return -EPIPE; }
    if (*(*conn).lgr).is_smcd { spin_lock_bh(&mut (*conn).send_lock); let rc = smcd_cdc_msg_send(conn); spin_unlock_bh(&mut (*conn).send_lock); rc } else { smcr_cdc_get_slot_and_msg_send(conn) }
}

pub unsafe fn smc_cdc_wait_pend_tx_wr(conn: *mut smc_connection) { wait_event(&mut (*conn).cdc_pend_tx_wq, atomic_read(&(*conn).cdc_pend_tx_wr) == 0); }

pub unsafe fn smcd_cdc_msg_send(conn: *mut smc_connection) -> i32 {
    let smc = container_of(conn, smc_sock_conn, conn); let mut curs: smc_host_cursor = core::mem::zeroed(); let mut cdc: smcd_cdc_msg = core::mem::zeroed();
    cdc.common.type_ = SMC_CDC_MSG_TYPE; curs.acurs.counter = atomic64_read(&(*conn).local_tx_ctrl.prod.acurs); cdc.prod.wrap = curs.wrap; cdc.prod.count = curs.count; curs.acurs.counter = atomic64_read(&(*conn).local_tx_ctrl.cons.acurs); cdc.cons.wrap = curs.wrap; cdc.cons.count = curs.count; cdc.cons.prod_flags = (*conn).local_tx_ctrl.prod_flags; cdc.cons.conn_state_flags = (*conn).local_tx_ctrl.conn_state_flags;
    let rc = smcd_tx_ism_write(conn, &cdc, core::mem::size_of::<smcd_cdc_msg>(), 0, 1); if rc != 0 { return rc; }
    smc_curs_copy(&mut (*conn).rx_curs_confirmed, &curs, conn); (*conn).local_rx_ctrl.prod_flags.cons_curs_upd_req = 0;
    if smc_ism_support_dmb_nocopy((*(*conn).lgr).smcd) { return 0; }
    let diff = smc_curs_diff((*(*conn).sndbuf_desc).len, &(*conn).tx_curs_fin, &(*conn).tx_curs_sent); smp_mb__before_atomic(); atomic_add(diff, &mut (*conn).sndbuf_space); smp_mb__after_atomic(); smc_curs_copy(&mut (*conn).tx_curs_fin, &(*conn).tx_curs_sent, conn); smc_tx_sndbuf_nonfull(smc); 0
}

unsafe fn smc_cdc_before(seq1: u16, seq2: u16) -> bool { (seq1.wrapping_sub(seq2) as i16) < 0 }

unsafe fn smc_cdc_handle_urg_data_arrival(smc: *mut smc_sock, diff_prod: *mut i32) {
    let conn = &mut (*smc).conn; smc_curs_copy(&mut conn.urg_curs, &conn.local_rx_ctrl.prod, conn); conn.urg_state = SMC_URG_VALID;
    if !sock_flag(&(*smc).sk, SOCK_URGINLINE) { *diff_prod -= 1; }
    let base = ((*conn).rmb_desc.cpu_addr as *mut u8).add(conn.rx_off);
    conn.urg_rx_byte = if conn.urg_curs.count != 0 { *base.add(conn.urg_curs.count as usize - 1) } else { *base.add(conn.rmb_desc.len as usize - 1) }; sk_send_sigurg(&mut (*smc).sk);
}

unsafe fn smc_cdc_msg_validate(smc: *mut smc_sock, cdc: *mut smc_cdc_msg, link: *mut smc_link) {
    let conn = &mut (*smc).conn; let diff = conn.local_rx_ctrl.seqno - ntohs((*cdc).seqno) as i16;
    if diff < 0 { conn.out_of_sync = 1; spin_lock_bh(&mut conn.send_lock); conn.local_tx_ctrl.conn_state_flags.peer_conn_abort = 1; conn.lnk = link; spin_unlock_bh(&mut conn.send_lock); sock_hold(&mut (*smc).sk); if !queue_work(smc_close_wq, &mut conn.abort_work) { sock_put(&mut (*smc).sk); } }
}

unsafe fn smc_cdc_msg_recv_action(smc: *mut smc_sock, cdc: *mut smc_cdc_msg) {
    let conn = &mut (*smc).conn; let mut prod_old = core::mem::zeroed(); let mut cons_old = core::mem::zeroed(); smc_curs_copy(&mut prod_old, &conn.local_rx_ctrl.prod, conn); smc_curs_copy(&mut cons_old, &conn.local_rx_ctrl.cons, conn); smc_cdc_msg_to_host(&mut conn.local_rx_ctrl, cdc, conn);
    let diff_cons = smc_curs_diff(conn.peer_rmbe_size, &cons_old, &conn.local_rx_ctrl.cons); if diff_cons != 0 { smp_mb__before_atomic(); atomic_add(diff_cons, &mut conn.peer_rmbe_space); smp_mb__after_atomic(); if (*conn.lgr).is_smcd && smc_ism_support_dmb_nocopy((*conn.lgr).smcd) { let d = smc_curs_diff((*conn.sndbuf_desc).len, &conn.tx_curs_fin, &conn.local_rx_ctrl.cons); smp_mb__before_atomic(); atomic_add(d, &mut conn.sndbuf_space); smp_mb__after_atomic(); smc_curs_copy(&mut conn.tx_curs_fin, &conn.local_rx_ctrl.cons, conn); smc_tx_sndbuf_nonfull(smc); } }
    let mut diff_prod = smc_curs_diff((*conn.rmb_desc).len, &prod_old, &conn.local_rx_ctrl.prod); if diff_prod != 0 { if conn.local_rx_ctrl.prod_flags.urg_data_present { smc_cdc_handle_urg_data_arrival(smc, &mut diff_prod); } smp_mb__before_atomic(); atomic_add(diff_prod, &mut conn.bytes_to_rcv); smp_mb__after_atomic(); ((*smc).sk.sk_data_ready)(&mut (*smc).sk); } else { if conn.local_rx_ctrl.prod_flags.write_blocked { ((*smc).sk.sk_data_ready)(&mut (*smc).sk); } if conn.local_rx_ctrl.prod_flags.urg_data_pending { conn.urg_state = SMC_URG_NOTYET; } }
    if (diff_cons != 0 && smc_tx_prepared_sends(conn)) || conn.local_rx_ctrl.prod_flags.cons_curs_upd_req || conn.local_rx_ctrl.prod_flags.urg_data_pending { if !sock_owned_by_user(&smc.sk) { smc_tx_pending(conn); } else { conn.tx_in_release_sock = true; } }
    if diff_cons != 0 && conn.urg_tx_pend && atomic_read(&conn.peer_rmbe_space) == conn.peer_rmbe_size { conn.urg_tx_pend = false; ((*smc).sk.sk_write_space)(&mut (*smc).sk); }
    if conn.local_rx_ctrl.conn_state_flags.peer_conn_abort { smc.sk.sk_err = ECONNRESET; conn.local_tx_ctrl.conn_state_flags.peer_conn_abort = 1; }
    if smc_cdc_rxed_any_close_or_senddone(conn) { smc.sk.sk_shutdown |= RCV_SHUTDOWN; smc_sock_set_flag(&mut smc.sk, SOCK_DONE); sock_hold(&mut smc.sk); if !queue_work(smc_close_wq, &mut conn.close_work) { sock_put(&mut smc.sk); } }
}

unsafe fn smc_cdc_msg_recv(smc: *mut smc_sock, cdc: *mut smc_cdc_msg) { sock_hold(&mut (*smc).sk); bh_lock_sock(&mut (*smc).sk); smc_cdc_msg_recv_action(smc, cdc); bh_unlock_sock(&mut (*smc).sk); sock_put(&mut (*smc).sk); }

/* Remaining receive-side helpers retain the C ABI-facing implementation through external project types. */
pub unsafe fn smcd_cdc_rx_init(conn: *mut smc_connection) { tasklet_setup(&mut (*conn).rx_tsklet, smcd_cdc_rx_tsklet); }

unsafe fn smcd_cdc_rx_tsklet(t: *mut tasklet_struct) { let conn = from_tasklet(t); if conn.is_null() || (*conn).killed { return; } let data = (*conn).rmb_desc as *mut smcd_cdc_msg; let mut cdc: smcd_cdc_msg = core::mem::zeroed(); smcd_curs_copy(&mut cdc.prod, &(*data).prod, conn); smcd_curs_copy(&mut cdc.cons, &(*data).cons, conn); let smc = container_of(conn, smc_sock_conn, conn); smc_cdc_msg_recv(smc, &mut cdc as *mut smcd_cdc_msg as *mut smc_cdc_msg); }

pub unsafe fn smc_cdc_init() -> i32 { let mut rc = 0; let mut handler = smc_cdc_rx_handlers.as_mut_ptr(); while !(*handler).handler.is_none() { init_hlist_node(&mut (*handler).list); rc = smc_wr_rx_register_handler(handler); if rc != 0 { break; } handler = handler.add(1); } rc }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
