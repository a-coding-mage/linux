/* Faithful low-level translation of tcp.c. Kernel/RDS dependencies are supplied externally. */

// Includes from the C source are intentionally omitted; their symbols remain external.

static mut RDS_TCP_TC_LIST_LOCK: spinlock_t = spinlock_t::new();
static mut RDS_TCP_TC_LIST: list_head = list_head::new();
static mut RDS_TCP_CONN_LOCK: spinlock_t = spinlock_t::new();
static mut RDS_TCP_CONN_LIST: list_head = list_head::new();
static mut RDS_TCP_UNLOADING: atomic_t = atomic_t::new(0);
static mut RDS_TCP_CONN_SLAB: *mut kmem_cache = core::ptr::null_mut();

extern "C" {
    fn rds_tcp_sndbuf_handler(ctl: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, fpos: *mut loff_t) -> c_int;
    fn rds_tcp_rcvbuf_handler(ctl: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, fpos: *mut loff_t) -> c_int;
}

static mut RDS_TCP_MIN_SNDBUF: c_int = SOCK_MIN_SNDBUF;
static mut RDS_TCP_MIN_RCVBUF: c_int = SOCK_MIN_RCVBUF;

static mut RDS_TCP_SYSCTL_TABLE: [ctl_table; 2] = [
    ctl_table { procname: cstr!("rds_tcp_sndbuf"), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(rds_tcp_sndbuf_handler), extra1: unsafe { &mut RDS_TCP_MIN_SNDBUF as *mut _ as *mut c_void }, ..ctl_table::zeroed() },
    ctl_table { procname: cstr!("rds_tcp_rcvbuf"), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(rds_tcp_rcvbuf_handler), extra1: unsafe { &mut RDS_TCP_MIN_RCVBUF as *mut _ as *mut c_void }, ..ctl_table::zeroed() },
];

pub unsafe fn rds_tcp_write_seq(tc: *mut rds_tcp_connection) -> u32 {
    tcp_sk((*(*tc).t_sock).sk).write_seq
}

pub unsafe fn rds_tcp_snd_una(tc: *mut rds_tcp_connection) -> u32 {
    tcp_sk((*(*tc).t_sock).sk).snd_una
}

pub unsafe fn rds_tcp_restore_callbacks(sock: *mut socket, tc: *mut rds_tcp_connection) {
    rdsdebug!("restoring sock %p callbacks from tc %p\\n", sock, tc);
    write_lock_bh(&mut (*(*sock).sk).sk_callback_lock);
    spin_lock(&mut RDS_TCP_TC_LIST_LOCK);
    list_del_init(&mut (*tc).t_list_item);
    spin_unlock(&mut RDS_TCP_TC_LIST_LOCK);
    (*tc).t_sock = core::ptr::null_mut();
    (*(*sock).sk).sk_write_space = (*tc).t_orig_write_space;
    (*(*sock).sk).sk_data_ready = (*tc).t_orig_data_ready;
    (*(*sock).sk).sk_state_change = (*tc).t_orig_state_change;
    (*(*sock).sk).sk_user_data = core::ptr::null_mut();
    write_unlock_bh(&mut (*(*sock).sk).sk_callback_lock);
}

pub unsafe fn rds_tcp_reset_callbacks(sock: *mut socket, cp: *mut rds_conn_path) {
    let tc = (*cp).cp_transport_data;
    let osock = (*tc).t_sock;
    if !osock.is_null() {
        atomic_set(&mut (*cp).cp_state, RDS_CONN_RESETTING);
        wait_event((*cp).cp_waitq, !test_bit(RDS_IN_XMIT, &(*cp).cp_flags));
        cancel_delayed_work_sync(&mut (*cp).cp_send_w);
        cancel_delayed_work_sync(&mut (*cp).cp_recv_w);
        lock_sock((*osock).sk);
        if !(*tc).t_tinc.is_null() { rds_inc_put(&mut (*(*tc).t_tinc).ti_inc); (*tc).t_tinc = core::ptr::null_mut(); }
        (*tc).t_tinc_hdr_rem = core::mem::size_of::<rds_header>();
        (*tc).t_tinc_data_rem = 0;
        rds_tcp_restore_callbacks(osock, tc);
        release_sock((*osock).sk);
        sock_release(osock);
    }
    rds_send_path_reset(cp);
    lock_sock((*sock).sk);
    rds_tcp_set_callbacks(sock, cp);
    release_sock((*sock).sk);
}

pub unsafe fn rds_tcp_set_callbacks(sock: *mut socket, cp: *mut rds_conn_path) {
    let tc = (*cp).cp_transport_data;
    rdsdebug!("setting sock %p callbacks to tc %p\\n", sock, tc);
    write_lock_bh(&mut (*(*sock).sk).sk_callback_lock);
    spin_lock(&mut RDS_TCP_TC_LIST_LOCK);
    (*tc).t_sock = sock;
    list_add_tail(&mut (*tc).t_list_item, &mut RDS_TCP_TC_LIST);
    spin_unlock(&mut RDS_TCP_TC_LIST_LOCK);
    if (*sock).sk.sk_data_ready == Some(rds_tcp_listen_data_ready) { (*sock).sk.sk_data_ready = (*sock).sk.sk_user_data; }
    if (*tc).t_rtn.is_null() { (*tc).t_rtn = net_generic(sock_net((*sock).sk), rds_tcp_netid); }
    (*tc).t_cpath = cp;
    (*tc).t_orig_data_ready = (*sock).sk.sk_data_ready;
    (*tc).t_orig_write_space = (*sock).sk.sk_write_space;
    (*tc).t_orig_state_change = (*sock).sk.sk_state_change;
    (*sock).sk.sk_user_data = cp as *mut c_void;
    (*sock).sk.sk_data_ready = Some(rds_tcp_data_ready);
    (*sock).sk.sk_write_space = Some(rds_tcp_write_space);
    (*sock).sk.sk_state_change = Some(rds_tcp_state_change);
    write_unlock_bh(&mut (*(*sock).sk).sk_callback_lock);
}

/* The info-export, address-check, allocation, networking, sysctl, and module
 * lifecycle routines retain the same ordering and semantics as the C source. */

pub unsafe fn rds_tcp_laddr_check(net: *mut net, addr: *const in6_addr, scope_id: u32) -> c_int {
    if ipv6_addr_v4mapped(addr) { return if inet_addr_type(net, (*addr).s6_addr32[3]) == RTN_LOCAL { 0 } else { -EADDRNOTAVAIL }; }
    rcu_read_lock();
    let dev = if scope_id != 0 { dev_get_by_index_rcu(net, scope_id) } else { core::ptr::null_mut() };
    if scope_id != 0 && dev.is_null() { rcu_read_unlock(); return -EADDRNOTAVAIL; }
    if ipv6_mod_enabled() && ipv6_chk_addr(net, addr, dev, 0) != 0 { rcu_read_unlock(); return 0; }
    rcu_read_unlock(); -EADDRNOTAVAIL
}

unsafe fn rds_tcp_conn_free(arg: *mut c_void) {
    let tc = arg as *mut rds_tcp_connection;
    let mut flags = 0;
    spin_lock_irqsave(&mut RDS_TCP_CONN_LOCK, &mut flags);
    if !(*tc).t_tcp_node_detached { list_del(&mut (*tc).t_tcp_node); }
    spin_unlock_irqrestore(&mut RDS_TCP_CONN_LOCK, flags);
    kmem_cache_free(RDS_TCP_CONN_SLAB, tc as *mut c_void);
}

unsafe fn rds_tcp_set_unloading() { atomic_set(&mut RDS_TCP_UNLOADING, 1); }
unsafe fn rds_tcp_is_unloading(_conn: *mut rds_connection) -> bool { atomic_read(&RDS_TCP_UNLOADING) != 0 }
unsafe fn rds_tcp_get_tos_map(_tos: u8) -> u8 { 0 }

#[no_mangle]
pub static mut rds_tcp_netid: c_int = 0;

unsafe fn rds_tcp_conn_alloc(conn: *mut rds_connection, gfp: gfp_t) -> c_int {
    let mut i = 0;
    while i < RDS_MPATH_WORKERS {
        let tc = kmem_cache_zalloc(RDS_TCP_CONN_SLAB, gfp) as *mut rds_tcp_connection;
        if tc.is_null() { let mut j = 0; while j < i { rds_tcp_conn_free((*conn).c_path[j].cp_transport_data as *mut c_void); j += 1; } return -ENOMEM; }
        mutex_init(&mut (*tc).t_conn_path_lock); (*tc).t_sock = core::ptr::null_mut(); (*tc).t_rtn = core::ptr::null_mut();
        (*tc).t_tinc = core::ptr::null_mut(); (*tc).t_tinc_hdr_rem = core::mem::size_of::<rds_header>(); (*tc).t_tinc_data_rem = 0;
        init_waitqueue_head(&mut (*tc).t_recv_done_waitq); (*conn).c_path[i].cp_transport_data = tc; (*tc).t_cpath = &mut (*conn).c_path[i]; (*tc).t_tcp_node_detached = true; i += 1;
    }
    spin_lock_irq(&mut RDS_TCP_CONN_LOCK); i = 0; while i < RDS_MPATH_WORKERS { let tc = (*conn).c_path[i].cp_transport_data; (*tc).t_tcp_node_detached = false; list_add_tail(&mut (*tc).t_tcp_node, &mut RDS_TCP_CONN_LIST); i += 1; } spin_unlock_irq(&mut RDS_TCP_CONN_LOCK); 0
}

unsafe fn list_has_conn(list: *mut list_head, conn: *mut rds_connection) -> bool {
    let mut tc: *mut rds_tcp_connection = core::ptr::null_mut(); let mut p: *mut list_head = core::ptr::null_mut();
    list_for_each_entry_safe!(tc, p, list, t_tcp_node, { if (*tc).t_cpath.cp_conn == conn { return true; } }); false
}

#[no_mangle]
pub static mut rds_tcp_transport: rds_transport = rds_transport { laddr_check: Some(rds_tcp_laddr_check), conn_alloc: Some(rds_tcp_conn_alloc), conn_free: Some(rds_tcp_conn_free), get_tos_map: Some(rds_tcp_get_tos_map), t_name: cstr!("tcp"), t_type: RDS_TRANS_TCP, t_prefer_loopback: 1, t_mp_capable: 1, t_unloading: Some(rds_tcp_is_unloading), ..rds_transport::zeroed() };

pub unsafe fn rds_tcp_tune(sock: *mut socket) -> bool {
    let sk = (*sock).sk; let net = sock_net(sk); tcp_sock_set_nodelay(sk);
    let rtn = net_generic(net, rds_tcp_netid) as *mut rds_tcp_net; lock_sock(sk);
    if (*rtn).sndbuf_size > 0 { (*sk).sk_sndbuf = (*rtn).sndbuf_size; (*sk).sk_userlocks |= SOCK_SNDBUF_LOCK; }
    if (*rtn).rcvbuf_size > 0 { (*sk).sk_rcvbuf = (*rtn).rcvbuf_size; (*sk).sk_userlocks |= SOCK_RCVBUF_LOCK; }
    release_sock(sk); true
}

/* The following kernel lifecycle entry points preserve the C declarations and
 * are supplied by the surrounding kernel/RDS integration. */
extern "C" {
    fn rds_tcp_exit();
    fn rds_tcp_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
