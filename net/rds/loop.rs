/*
 * Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available to you under a choice of one of two
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

// Dependencies supplied by the Linux kernel and RDS headers.

static mut loop_conns_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut loop_conns: list_head = LIST_HEAD!();
static mut rds_loop_unloading: atomic_t = ATOMIC_INIT!(0);

unsafe fn rds_loop_set_unloading() {
    atomic_set(&raw mut rds_loop_unloading, 1);
}

unsafe fn rds_loop_is_unloading(_conn: *mut rds_connection) -> bool {
    atomic_read(&raw mut rds_loop_unloading) != 0
}

/*
 * This 'loopback' transport is a special case for flows that originate
 * and terminate on the same machine.
 *
 * Connection build-up notices if the destination address is thought of
 * as a local address by a transport.  At that time it decides to use the
 * loopback transport instead of the bound transport of the sending socket.
 *
 * The loopback transport's sending path just hands the sent rds_message
 * straight to the receiving path via an embedded rds_incoming.
 */

/*
 * Usually a message transits both the sender and receiver's conns as it
 * flows to the receiver.  In the loopback case, though, the receive path
 * is handed the sending conn so the sense of the addresses is reversed.
 */
unsafe fn rds_loop_xmit(
    conn: *mut rds_connection,
    rm: *mut rds_message,
    hdr_off: c_uint,
    sg: c_uint,
    off: c_uint,
) -> c_int {
    let sgp = &mut (*rm).data.op_sg[sg as usize];
    let mut ret = core::mem::size_of::<rds_header>() as c_int
        + be32_to_cpu((*rm).m_inc.i_hdr.h_len) as c_int;

    /* Do not send cong updates to loopback */
    if (*rm).m_inc.i_hdr.h_flags & RDS_FLAG_CONG_BITMAP != 0 {
        rds_cong_map_updated((*conn).c_fcong, !0u64);
        ret = core::cmp::min(ret, sgp.length as c_int - (*conn).c_xmit_data_off);
        return ret;
    }

    BUG_ON(hdr_off != 0 || sg != 0 || off != 0);

    /* rds_send_queue_rm() stored the connection path in this embedded
     * inc; use the path init so the re-initialization keeps the field
     * valid instead of discarding it.
     */
    rds_inc_path_init(&mut (*rm).m_inc, &mut (*conn).c_path[0], &(*conn).c_laddr);
    /* For the embedded inc. Matching put is in loop_inc_free() */
    rds_message_addref(rm);

    rds_recv_incoming(
        conn,
        &(*conn).c_laddr,
        &(*conn).c_faddr,
        &mut (*rm).m_inc,
        GFP_KERNEL,
    );

    rds_send_drop_acked(conn, be64_to_cpu((*rm).m_inc.i_hdr.h_sequence), core::ptr::null_mut());

    rds_inc_put(&mut (*rm).m_inc);
    ret
}

/*
 * See rds_loop_xmit(). Since our inc is embedded in the rm, we
 * make sure the rm lives at least until the inc is done.
 */
unsafe fn rds_loop_inc_free(inc: *mut rds_incoming) {
    let rm = container_of!(inc, rds_message, m_inc);
    rds_message_put(rm);
}

/* we need to at least give the thread something to succeed */
unsafe fn rds_loop_recv_path(_cp: *mut rds_conn_path) -> c_int {
    0
}

#[repr(C)]
struct rds_loop_connection {
    loop_node: list_head,
    conn: *mut rds_connection,
}

/*
 * Even the loopback transport needs to keep track of its connections,
 * so it can call rds_conn_destroy() on them on exit. N.B. there are
 * 1+ loopback addresses (127.*.*.*) so it's not a bug to have
 * multiple loopback conns allocated, although rather useless.
 */
unsafe fn rds_loop_conn_alloc(conn: *mut rds_connection, gfp: gfp_t) -> c_int {
    let lc = kzalloc_obj::<rds_loop_connection>(gfp);
    if lc.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*lc).loop_node);
    (*lc).conn = conn;
    (*conn).c_transport_data = lc as *mut c_void;

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut loop_conns_lock, &mut flags);
    list_add_tail(&mut (*lc).loop_node, &raw mut loop_conns);
    spin_unlock_irqrestore(&raw mut loop_conns_lock, flags);

    0
}

unsafe fn rds_loop_conn_free(arg: *mut c_void) {
    let lc = arg as *mut rds_loop_connection;
    let mut flags: c_ulong = 0;

    rdsdebug!("lc %p\n", lc);
    spin_lock_irqsave(&raw mut loop_conns_lock, &mut flags);
    list_del(&mut (*lc).loop_node);
    spin_unlock_irqrestore(&raw mut loop_conns_lock, flags);
    kfree(lc as *mut c_void);
}

unsafe fn rds_loop_conn_path_connect(cp: *mut rds_conn_path) -> c_int {
    rds_connect_complete((*cp).cp_conn);
    0
}

unsafe fn rds_loop_conn_path_shutdown(_cp: *mut rds_conn_path) {}

unsafe fn rds_loop_exit() {
    let mut tmp_list: list_head = LIST_HEAD!();

    rds_loop_set_unloading();
    synchronize_rcu();
    /* avoid calling conn_destroy with irqs off */
    spin_lock_irq(&raw mut loop_conns_lock);
    list_splice(&raw mut loop_conns, &mut tmp_list);
    INIT_LIST_HEAD(&raw mut loop_conns);
    spin_unlock_irq(&raw mut loop_conns_lock);

    list_for_each_entry_safe!(lc, _lc, &mut tmp_list, loop_node, rds_loop_connection, {
        WARN_ON((*lc).conn.is_null() || (*(*lc).conn).c_passive);
        rds_conn_destroy((*lc).conn);
    });
}

unsafe fn rds_loop_kill_conns(net: *mut net) {
    let mut tmp_list: list_head = LIST_HEAD!();

    spin_lock_irq(&raw mut loop_conns_lock);
    list_for_each_entry_safe!(lc, _lc, &mut loop_conns, loop_node, rds_loop_connection, {
        let c_net = read_pnet(&(*(*lc).conn).c_net);
        if net != c_net {
            continue;
        }
        list_move_tail(&mut (*lc).loop_node, &mut tmp_list);
    });
    spin_unlock_irq(&raw mut loop_conns_lock);

    list_for_each_entry_safe!(lc, _lc, &mut tmp_list, loop_node, rds_loop_connection, {
        WARN_ON((*lc).conn.is_null() || (*(*lc).conn).c_passive);
        rds_conn_destroy((*lc).conn);
    });
}

unsafe fn rds_loop_exit_net(net: *mut net) {
    rds_loop_kill_conns(net);
}

static mut rds_loop_net_ops: pernet_operations = pernet_operations {
    exit: Some(rds_loop_exit_net),
};

unsafe fn rds_loop_net_init() -> c_int {
    register_pernet_device(&raw mut rds_loop_net_ops)
}

unsafe fn rds_loop_net_exit() {
    unregister_pernet_device(&raw mut rds_loop_net_ops);
}

/*
 * This is missing .xmit_* because loop doesn't go through generic
 * rds_send_xmit() and doesn't call rds_recv_incoming().  .listen_stop and
 * .laddr_check are missing because transport.c doesn't iterate over
 * rds_loop_transport.
 */
#[no_mangle]
pub static mut rds_loop_transport: rds_transport = rds_transport {
    xmit: Some(rds_loop_xmit),
    recv_path: Some(rds_loop_recv_path),
    conn_alloc: Some(rds_loop_conn_alloc),
    conn_free: Some(rds_loop_conn_free),
    conn_path_connect: Some(rds_loop_conn_path_connect),
    conn_path_shutdown: Some(rds_loop_conn_path_shutdown),
    inc_copy_to_user: Some(rds_message_inc_copy_to_user),
    inc_free: Some(rds_loop_inc_free),
    t_name: b"loopback\0".as_ptr() as *const c_char,
    t_type: RDS_TRANS_LOOP,
    t_unloading: Some(rds_loop_is_unloading),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
