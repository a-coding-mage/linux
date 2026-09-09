/* SPDX-License-Identifier: GPL-2.0 */

pub const RDS_TCP_PORT: u32 = 16385;

/* per-network namespace private data for this module */
#[repr(C)]
pub struct rds_tcp_net {
    /* serialize "rds_tcp_accept_one" with "rds_tcp_accept_lock"
     * to protect "rds_tcp_accepted_sock"
     */
    pub rds_tcp_accept_lock: mutex,
    pub rds_tcp_listen_sock: *mut socket,
    pub rds_tcp_accepted_sock: *mut socket,
    pub rds_tcp_accept_w: work_struct,
    pub rds_tcp_sysctl: *mut ctl_table_header,
    pub ctl_table: *const ctl_table,
    pub sndbuf_size: core::ffi::c_int,
    pub rcvbuf_size: core::ffi::c_int,
}

#[repr(C)]
pub struct rds_tcp_incoming {
    pub ti_inc: rds_incoming,
    pub ti_skb_list: sk_buff_head,
}

#[repr(C)]
pub struct rds_tcp_connection {
    pub t_tcp_node: list_head,
    pub t_tcp_node_detached: bool,
    pub t_cpath: *mut rds_conn_path,
    /* t_conn_path_lock synchronizes the connection establishment between
     * rds_tcp_accept_one and rds_tcp_conn_path_connect
     */
    pub t_conn_path_lock: mutex,
    pub t_sock: *mut socket,
    pub t_client_port_group: u32,
    pub t_rtn: *mut rds_tcp_net,
    pub t_orig_write_space: *mut core::ffi::c_void,
    pub t_orig_data_ready: *mut core::ffi::c_void,
    pub t_orig_state_change: *mut core::ffi::c_void,

    pub t_tinc: *mut rds_tcp_incoming,
    pub t_tinc_hdr_rem: usize,
    pub t_tinc_data_rem: usize,

    /* XXX error report? */
    pub t_conn_w: work_struct,
    pub t_send_w: work_struct,
    pub t_down_w: work_struct,
    pub t_recv_w: work_struct,

    /* for info exporting only */
    pub t_list_item: list_head,
    pub t_last_sent_nxt: u32,
    pub t_last_expected_una: u32,
    pub t_last_seen_una: u32,

    /* for rds_tcp_conn_path_shutdown */
    pub t_recv_done_waitq: wait_queue_head_t,
}

#[repr(C)]
pub struct rds_tcp_statistics {
    pub s_tcp_data_ready_calls: u64,
    pub s_tcp_write_space_calls: u64,
    pub s_tcp_sndbuf_full: u64,
    pub s_tcp_connect_raced: u64,
    pub s_tcp_listen_closed_stale: u64,
}

/* tcp.c */
unsafe extern "C" {
    pub static mut rds_tcp_netid: core::ffi::c_int;
    pub fn rds_tcp_tune(sock: *mut socket) -> bool;
    pub fn rds_tcp_set_callbacks(sock: *mut socket, cp: *mut rds_conn_path);
    pub fn rds_tcp_reset_callbacks(sock: *mut socket, cp: *mut rds_conn_path);
    pub fn rds_tcp_restore_callbacks(sock: *mut socket, tc: *mut rds_tcp_connection);
    pub fn rds_tcp_write_seq(tc: *mut rds_tcp_connection) -> u32;
    pub fn rds_tcp_snd_una(tc: *mut rds_tcp_connection) -> u32;
    pub static mut rds_tcp_transport: rds_transport;
    pub fn rds_tcp_accept_work(rtn: *mut rds_tcp_net);
    pub fn rds_tcp_laddr_check(net: *mut net, addr: *const in6_addr, scope_id: u32) -> core::ffi::c_int;

    /* tcp_connect.c */
    pub fn rds_tcp_conn_path_connect(cp: *mut rds_conn_path) -> core::ffi::c_int;
    pub fn rds_tcp_conn_path_shutdown(conn: *mut rds_conn_path);
    pub fn rds_tcp_state_change(sk: *mut sock);

    /* tcp_listen.c */
    pub fn rds_tcp_listen_init(net: *mut net, isv6: bool) -> *mut socket;
    pub fn rds_tcp_listen_stop(sock: *mut socket, acceptor: *mut work_struct);
    pub fn rds_tcp_listen_data_ready(sk: *mut sock);
    pub fn rds_tcp_conn_slots_available(conn: *mut rds_connection, fan_out: bool);
    pub fn rds_tcp_accept_one(rtn: *mut rds_tcp_net) -> core::ffi::c_int;
    pub fn rds_tcp_keepalive(sock: *mut socket);
    pub fn rds_tcp_listen_sock_def_readable(net: *mut net) -> *mut core::ffi::c_void;

    /* tcp_recv.c */
    pub fn rds_tcp_recv_init() -> core::ffi::c_int;
    pub fn rds_tcp_recv_exit();
    pub fn rds_tcp_data_ready(sk: *mut sock);
    pub fn rds_tcp_recv_path(cp: *mut rds_conn_path) -> core::ffi::c_int;
    pub fn rds_tcp_inc_free(inc: *mut rds_incoming);
    pub fn rds_tcp_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> core::ffi::c_int;

    /* tcp_send.c */
    pub fn rds_tcp_xmit_path_prepare(cp: *mut rds_conn_path);
    pub fn rds_tcp_xmit_path_complete(cp: *mut rds_conn_path);
    pub fn rds_tcp_xmit(conn: *mut rds_connection, rm: *mut rds_message, hdr_off: u32, sg: u32, off: u32) -> core::ffi::c_int;
    pub fn rds_tcp_is_acked(rm: *mut rds_message, ack: u64) -> core::ffi::c_int;
    pub fn rds_tcp_write_space(sk: *mut sock);

    /* tcp_stats.c */
    pub static mut rds_tcp_stats: [rds_tcp_statistics; 0];
    pub fn rds_tcp_stats_info_copy(iter: *mut rds_info_iterator, avail: u32) -> u32;
}

macro_rules! rds_tcp_stats_inc {
    ($member:ident) => {
        rds_stats_inc_which!(rds_tcp_stats, $member)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
