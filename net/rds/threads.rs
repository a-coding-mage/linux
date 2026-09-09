/*
 * Copyright (c) 2006, 2018 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, available from the file COPYING in the main directory of
 * this source tree, or the OpenIB.org BSD license.
 */

/* Linux kernel headers and rds.h provide the types, constants, and external
 * symbols referenced below. */

pub static mut rds_wq: *mut workqueue_struct = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn rds_connect_path_complete(
    cp: *mut rds_conn_path,
    curr: i32,
) {
    if !rds_conn_path_transition(cp, curr, RDS_CONN_UP) {
        printk(
            KERN_WARNING,
            b"%s: Cannot transition to state UP, current state is %d\n\0".as_ptr(),
            b"rds_connect_path_complete\0".as_ptr(),
            (*cp).cp_state,
        );
        rds_conn_path_drop(cp, false);
        return;
    }

    rdsdebug(
        b"conn %p for %pI6c to %pI6c complete\n\0".as_ptr(),
        (*cp).cp_conn,
        &(*(*cp).cp_conn).c_laddr,
        &(*(*cp).cp_conn).c_faddr,
    );

    (*cp).cp_reconnect_jiffies = 0;
    set_bit(0, &mut (*(*cp).cp_conn).c_map_queued);
    rcu_read_lock();
    if !rds_destroy_pending((*cp).cp_conn) {
        queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_send_w, 0);
        queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_recv_w, 0);
    }
    rcu_read_unlock();
    (*cp).cp_conn.c_proposed_version = RDS_PROTOCOL_VERSION;
}

#[no_mangle]
pub unsafe extern "C" fn rds_connect_complete(conn: *mut rds_connection) {
    rds_connect_path_complete(&mut (*conn).c_path[0], RDS_CONN_CONNECTING);
}

#[no_mangle]
pub unsafe extern "C" fn rds_queue_reconnect(cp: *mut rds_conn_path) {
    let mut rand: c_ulong = 0;
    let conn = (*cp).cp_conn;

    rdsdebug(
        b"conn %p for %pI6c to %pI6c reconnect jiffies %lu\n\0".as_ptr(),
        conn, &(*conn).c_laddr, &(*conn).c_faddr, (*cp).cp_reconnect_jiffies,
    );

    if (*conn).c_trans.t_type == RDS_TRANS_TCP
        && rds_addr_cmp(&(*conn).c_laddr, &(*conn).c_faddr) >= 0
    {
        return;
    }

    set_bit(RDS_RECONNECT_PENDING, &mut (*cp).cp_flags);
    if (*cp).cp_reconnect_jiffies == 0 {
        (*cp).cp_reconnect_jiffies = rds_sysctl_reconnect_min_jiffies;
        rcu_read_lock();
        if !rds_destroy_pending((*cp).cp_conn) {
            queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_conn_w, 0);
        }
        rcu_read_unlock();
        return;
    }

    get_random_bytes(&mut rand as *mut _ as *mut c_void, core::mem::size_of::<c_ulong>());
    rcu_read_lock();
    if !rds_destroy_pending((*cp).cp_conn) {
        queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_conn_w,
            rand % (*cp).cp_reconnect_jiffies);
    }
    rcu_read_unlock();
    (*cp).cp_reconnect_jiffies = core::cmp::min(
        (*cp).cp_reconnect_jiffies * 2,
        rds_sysctl_reconnect_max_jiffies,
    );
}

#[no_mangle]
pub unsafe extern "C" fn rds_connect_worker(work: *mut work_struct) {
    let cp = container_of(work, core::mem::offset_of!(rds_conn_path, cp_conn_w.work));
    let conn = (*cp).cp_conn;
    let mut ret: i32;
    if (*cp).cp_index > 0 && rds_addr_cmp(&(*conn).c_laddr, &(*conn).c_faddr) >= 0 { return; }
    clear_bit(RDS_RECONNECT_PENDING, &mut (*cp).cp_flags);
    ret = rds_conn_path_transition(cp, RDS_CONN_DOWN, RDS_CONN_CONNECTING);
    if ret != 0 {
        ret = ((*conn).c_trans.conn_path_connect)(cp);
        if ret != 0 {
            if rds_conn_path_transition(cp, RDS_CONN_CONNECTING, RDS_CONN_DOWN) { rds_queue_reconnect(cp); }
            else { rds_conn_path_error(cp, b"connect failed\n\0".as_ptr()); }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_worker(work: *mut work_struct) {
    let cp = container_of(work, core::mem::offset_of!(rds_conn_path, cp_send_w.work));
    if rds_conn_path_state(cp) == RDS_CONN_UP {
        clear_bit(RDS_LL_SEND_FULL, &mut (*cp).cp_flags);
        match rds_send_xmit(cp) {
            -EAGAIN => { rds_stats_inc(s_send_immediate_retry); queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_send_w, 0); }
            -ENOMEM => { rds_stats_inc(s_send_delayed_retry); queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_send_w, 2); }
            _ => {}
        }
        cond_resched();
    }
}

#[no_mangle]
pub unsafe extern "C" fn rds_recv_worker(work: *mut work_struct) {
    let cp = container_of(work, core::mem::offset_of!(rds_conn_path, cp_recv_w.work));
    if rds_conn_path_state(cp) == RDS_CONN_UP {
        match ((*cp).cp_conn).c_trans.recv_path(cp) {
            -EAGAIN => { rds_stats_inc(s_recv_immediate_retry); queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_recv_w, 0); }
            -ENOMEM => { rds_stats_inc(s_recv_delayed_retry); queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_recv_w, 2); }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rds_shutdown_worker(work: *mut work_struct) {
    let cp = container_of(work, core::mem::offset_of!(rds_conn_path, cp_down_w));
    rds_conn_shutdown(cp);
}

#[no_mangle]
pub unsafe extern "C" fn rds_threads_exit() { destroy_workqueue(rds_wq); }

#[no_mangle]
pub unsafe extern "C" fn rds_threads_init() -> i32 {
    rds_wq = create_singlethread_workqueue(b"krdsd\0".as_ptr());
    if rds_wq.is_null() { -ENOMEM } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn rds_addr_cmp(addr1: *const in6_addr, addr2: *const in6_addr) -> i32 {
    for i in 0..4 {
        if (*addr1).s6_addr32[i] != (*addr2).s6_addr32[i] {
            let a = ntohl((*addr1).s6_addr32[i]);
            let b = ntohl((*addr2).s6_addr32[i]);
            return if a < b { -1 } else { 1 };
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
