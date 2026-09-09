/* Faithful low-level translation of connection.c. Kernel and RDS symbols are
 * supplied by the surrounding translation unit. */

const RDS_CONNECTION_HASH_BITS: usize = 12;
const RDS_CONNECTION_HASH_ENTRIES: usize = 1 << RDS_CONNECTION_HASH_BITS;
const RDS_CONNECTION_HASH_MASK: usize = RDS_CONNECTION_HASH_ENTRIES - 1;

static mut rds_conn_lock: spinlock_t = spinlock_t::new();
static mut rds_conn_count: c_ulong = 0;
static mut rds_conn_hash: [hlist_head; RDS_CONNECTION_HASH_ENTRIES] = [hlist_head::new(); RDS_CONNECTION_HASH_ENTRIES];
static mut rds_conn_slab: *mut kmem_cache = core::ptr::null_mut();

unsafe fn rds_conn_bucket(laddr: *const in6_addr, faddr: *const in6_addr) -> *mut hlist_head {
    static mut rds6_hash_secret: u32 = 0;
    static mut rds_hash_secret: u32 = 0;
    let lhash: __be32 = (*laddr).s6_addr32[3];
    let fhash: __be32 = __ipv6_addr_jhash(faddr, rds6_hash_secret);
    let hash: u32 = __inet_ehashfn(lhash, 0, fhash, 0, rds_hash_secret);
    &mut rds_conn_hash[(hash as usize) & RDS_CONNECTION_HASH_MASK]
}

unsafe fn rds_conn_lookup(net: *mut net, head: *mut hlist_head,
    laddr: *const in6_addr, faddr: *const in6_addr,
    trans: *mut rds_transport, tos: u8, dev_if: c_int) -> *mut rds_connection {
    let mut conn: *mut rds_connection;
    let mut ret: *mut rds_connection = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(conn, head, c_hash_node, {
        if ipv6_addr_equal(&(*conn).c_faddr, faddr) &&
           ipv6_addr_equal(&(*conn).c_laddr, laddr) && (*conn).c_trans == trans &&
           (*conn).c_tos == tos && net == rds_conn_net(conn) && (*conn).c_dev_if == dev_if {
            ret = conn;
            break;
        }
    });
    rdsdebug!("returning conn %p for %pI6c -> %pI6c\n", ret, laddr, faddr);
    ret
}

unsafe fn rds_conn_path_reset(cp: *mut rds_conn_path) {
    let conn = (*cp).cp_conn;
    rdsdebug!("connection %pI6c to %pI6c reset\n", &(*conn).c_laddr, &(*conn).c_faddr);
    rds_stats_inc!(s_conn_reset);
    rds_send_path_reset(cp);
    (*cp).cp_flags = 0;
}

unsafe fn __rds_conn_path_init(conn: *mut rds_connection, cp: *mut rds_conn_path, _is_outgoing: bool) {
    spin_lock_init(&mut (*cp).cp_lock);
    (*cp).cp_next_tx_seq = 1;
    init_waitqueue_head(&mut (*cp).cp_waitq);
    INIT_LIST_HEAD!(&mut (*cp).cp_send_queue);
    INIT_LIST_HEAD!(&mut (*cp).cp_retrans);
    (*cp).cp_conn = conn;
    atomic_set(&mut (*cp).cp_state, RDS_CONN_DOWN);
    (*cp).cp_send_gen = 0;
    (*cp).cp_reconnect_jiffies = 0;
    (*conn).c_proposed_version = RDS_PROTOCOL_VERSION;
    INIT_DELAYED_WORK!(&mut (*cp).cp_send_w, rds_send_worker);
    INIT_DELAYED_WORK!(&mut (*cp).cp_recv_w, rds_recv_worker);
    INIT_DELAYED_WORK!(&mut (*cp).cp_conn_w, rds_connect_worker);
    INIT_WORK!(&mut (*cp).cp_down_w, rds_shutdown_worker);
    mutex_init(&mut (*cp).cp_cm_lock);
    (*cp).cp_flags = 0;
}

unsafe fn __rds_conn_create(net: *mut net, laddr: *const in6_addr, faddr: *const in6_addr,
    mut trans: *mut rds_transport, gfp: gfp_t, tos: u8, is_outgoing: c_int, dev_if: c_int) -> *mut rds_connection {
    let head = rds_conn_bucket(laddr, faddr);
    let mut conn: *mut rds_connection;
    let mut parent: *mut rds_connection = core::ptr::null_mut();
    let mut free_cp: *mut rds_conn_path = core::ptr::null_mut();
    let npaths = if (*trans).t_mp_capable { RDS_MPATH_WORKERS } else { 1 };
    rcu_read_lock();
    conn = rds_conn_lookup(net, head, laddr, faddr, trans, tos, dev_if);
    if !conn.is_null() && (*conn).c_loopback && (*conn).c_trans != &mut rds_loop_transport &&
       ipv6_addr_equal(laddr, faddr) && is_outgoing == 0 { parent = conn; conn = (*parent).c_passive; }
    rcu_read_unlock();
    if !conn.is_null() { return conn; }
    conn = kmem_cache_zalloc(rds_conn_slab, gfp);
    if conn.is_null() { return ERR_PTR(-ENOMEM); }
    (*conn).c_path = kzalloc_objs::<rds_conn_path>(npaths, gfp);
    if (*conn).c_path.is_null() { kmem_cache_free(rds_conn_slab, conn); return ERR_PTR(-ENOMEM); }
    INIT_HLIST_NODE!(&mut (*conn).c_hash_node);
    (*conn).c_laddr = *laddr; (*conn).c_isv6 = !ipv6_addr_v4mapped(laddr);
    (*conn).c_faddr = *faddr; (*conn).c_dev_if = dev_if; (*conn).c_tos = tos;
    (*conn).c_bound_if = if ipv6_addr_type(laddr) & IPV6_ADDR_LINKLOCAL != 0 { dev_if } else { 0 };
    rds_conn_net_set(conn, net);
    let ret = rds_cong_get_maps(conn);
    if ret != 0 { kfree((*conn).c_path); kmem_cache_free(rds_conn_slab, conn); return ERR_PTR(ret); }
    let loop_trans = rds_trans_get_preferred(net, faddr, (*conn).c_dev_if);
    if !loop_trans.is_null() { rds_trans_put(loop_trans); (*conn).c_loopback = 1;
        if (*trans).t_prefer_loopback { if is_outgoing != 0 { trans = &mut rds_loop_transport; }
            else { kfree((*conn).c_path); kmem_cache_free(rds_conn_slab, conn); return ERR_PTR(-EOPNOTSUPP); } } }
    (*conn).c_trans = trans; init_waitqueue_head(&mut (*conn).c_hs_waitq);
    for i in 0..npaths { let cp = &mut (*conn).c_path.add(i); __rds_conn_path_init(conn, cp, is_outgoing != 0); (*cp).cp_index = i; (*cp).cp_wq = alloc_ordered_workqueue("krds_cp_wq#%lu/%d", 0, rds_conn_count, i); if (*cp).cp_wq.is_null() { (*cp).cp_wq = rds_wq; } }
    rcu_read_lock();
    let ret = if rds_destroy_pending(conn) { -ENETDOWN } else { (*trans).conn_alloc(conn, GFP_ATOMIC) };
    if ret != 0 { rcu_read_unlock(); free_cp = (*conn).c_path; kmem_cache_free(rds_conn_slab, conn); conn = ERR_PTR(ret); }
    else { spin_lock_irqsave(&mut rds_conn_lock, &mut 0); if !parent.is_null() { if !(*parent).c_passive.is_null() { (*trans).conn_free((*conn).c_path[0].cp_transport_data); free_cp = (*conn).c_path; kmem_cache_free(rds_conn_slab, conn); conn = (*parent).c_passive; } else { (*parent).c_passive = conn; rds_cong_add_conn(conn); rds_conn_count += 1; } } else { let found = rds_conn_lookup(net, head, laddr, faddr, trans, tos, dev_if); if !found.is_null() { for i in 0..npaths { let cp = &mut (*conn).c_path.add(i); if !cp.cp_transport_data.is_null() { (*trans).conn_free(cp.cp_transport_data); } } free_cp = (*conn).c_path; kmem_cache_free(rds_conn_slab, conn); conn = found; } else { (*conn).c_my_gen_num = rds_gen_num; (*conn).c_peer_gen_num = 0; hlist_add_head_rcu!(&mut (*conn).c_hash_node, head); rds_cong_add_conn(conn); rds_conn_count += 1; } } spin_unlock_irqrestore(&mut rds_conn_lock, &mut 0); rcu_read_unlock(); }
    if !free_cp.is_null() { for i in 0..npaths { if (*free_cp.add(i)).cp_wq != rds_wq { destroy_workqueue((*free_cp.add(i)).cp_wq); } } kfree(free_cp); }
    conn
}

pub unsafe fn rds_conn_create(net: *mut net, laddr: *const in6_addr, faddr: *const in6_addr, trans: *mut rds_transport, tos: u8, gfp: gfp_t, dev_if: c_int) -> *mut rds_connection { __rds_conn_create(net,laddr,faddr,trans,gfp,tos,0,dev_if) }
pub unsafe fn rds_conn_create_outgoing(net: *mut net, laddr: *const in6_addr, faddr: *const in6_addr, trans: *mut rds_transport, tos: u8, gfp: gfp_t, dev_if: c_int) -> *mut rds_connection { __rds_conn_create(net,laddr,faddr,trans,gfp,tos,1,dev_if) }

pub unsafe fn rds_conn_path_drop(cp: *mut rds_conn_path, destroy: bool) { atomic_set(&mut (*cp).cp_state, RDS_CONN_ERROR); rcu_read_lock(); if !destroy && rds_destroy_pending((*cp).cp_conn) { rcu_read_unlock(); return; } queue_work((*cp).cp_wq, &mut (*cp).cp_down_w); rcu_read_unlock(); }
pub unsafe fn rds_conn_drop(conn: *mut rds_connection) { WARN_ON!((*conn).c_trans.t_mp_capable); rds_conn_path_drop((*conn).c_path, false); }
pub unsafe fn rds_conn_path_connect_if_down(cp: *mut rds_conn_path) { rcu_read_lock(); if rds_destroy_pending((*cp).cp_conn) { rcu_read_unlock(); return; } if rds_conn_path_state(cp)==RDS_CONN_DOWN && !test_and_set_bit(RDS_RECONNECT_PENDING,&mut (*cp).cp_flags) { queue_delayed_work((*cp).cp_wq,&mut (*cp).cp_conn_w,0); } rcu_read_unlock(); }
pub unsafe fn rds_check_all_paths(conn: *mut rds_connection) { let mut i=0; loop { rds_conn_path_connect_if_down((*conn).c_path.add(i)); i+=1; if i>=(*conn).c_npaths { break; } } }
pub unsafe fn rds_conn_connect_if_down(conn: *mut rds_connection) { WARN_ON!((*conn).c_trans.t_mp_capable); rds_conn_path_connect_if_down((*conn).c_path); }
pub unsafe fn __rds_conn_path_error(cp: *mut rds_conn_path, fmt: *const c_char, mut ap: ...) { vprintk(fmt, ap); rds_conn_path_drop(cp,false); }

pub unsafe fn rds_conn_shutdown(cp: *mut rds_conn_path) {
    let conn=(*cp).cp_conn;
    if !rds_conn_path_transition(cp,RDS_CONN_DOWN,RDS_CONN_DOWN) {
        mutex_lock(&mut (*cp).cp_cm_lock);
        if !rds_conn_path_transition(cp,RDS_CONN_UP,RDS_CONN_DISCONNECTING) &&
           !rds_conn_path_transition(cp,RDS_CONN_ERROR,RDS_CONN_DISCONNECTING) &&
           !rds_conn_path_transition(cp,RDS_CONN_RESETTING,RDS_CONN_DISCONNECTING) {
            rds_conn_path_error(cp,"shutdown called in state %d\n",atomic_read(&(*cp).cp_state)); mutex_unlock(&mut (*cp).cp_cm_lock); return;
        }
        mutex_unlock(&mut (*cp).cp_cm_lock);
        wait_event!((*cp).cp_waitq,!test_bit(RDS_IN_XMIT,&(*cp).cp_flags));
        wait_event!((*cp).cp_waitq,!test_bit(RDS_RECV_REFILL,&(*cp).cp_flags));
        ((*conn).c_trans).conn_path_shutdown(cp); rds_conn_path_reset(cp);
        if !rds_conn_path_transition(cp,RDS_CONN_DISCONNECTING,RDS_CONN_DOWN) && !rds_conn_path_transition(cp,RDS_CONN_ERROR,RDS_CONN_DOWN) { rds_conn_path_error(cp,"failed to transition to state DOWN, current state is %d\n",atomic_read(&(*cp).cp_state)); return; }
    }
    cancel_delayed_work_sync(&mut (*cp).cp_conn_w); clear_bit(RDS_RECONNECT_PENDING,&mut (*cp).cp_flags); rcu_read_lock();
    if !hlist_unhashed(&(*conn).c_hash_node) { rcu_read_unlock(); if (*conn).c_trans.t_mp_capable && (*cp).cp_index==0 { rds_send_ping(conn,0); } rds_queue_reconnect(cp); } else { rcu_read_unlock(); }
    if !(*conn).c_trans.conn_slots_available.is_none() { ((*conn).c_trans).conn_slots_available(conn,false); }
}

pub unsafe fn rds_conn_destroy(conn: *mut rds_connection) {
    let npaths=if (*conn).c_trans.t_mp_capable {RDS_MPATH_WORKERS} else {1}; spin_lock_irq(&mut rds_conn_lock); hlist_del_init_rcu(&mut (*conn).c_hash_node); spin_unlock_irq(&mut rds_conn_lock); synchronize_rcu();
    for i in 0..npaths { rds_conn_path_destroy((*conn).c_path.add(i)); BUG_ON!(!list_empty(&(*(*conn).c_path.add(i)).cp_retrans)); }
    rds_cong_remove_conn(conn); kfree((*conn).c_path); kmem_cache_free(rds_conn_slab,conn); spin_lock_irqsave(&mut rds_conn_lock,&mut 0); rds_conn_count-=1; spin_unlock_irqrestore(&mut rds_conn_lock,&mut 0);
}

unsafe fn rds_conn_path_destroy(cp:*mut rds_conn_path) { if (*cp).cp_transport_data.is_null(){return;} cancel_delayed_work_sync(&mut (*cp).cp_send_w); cancel_delayed_work_sync(&mut (*cp).cp_recv_w); rds_conn_path_drop(cp,true); flush_work(&mut (*cp).cp_down_w); while !list_empty(&(*cp).cp_send_queue){ let rm=list_first_entry!(&(*cp).cp_send_queue,rds_message,m_conn_item); list_del_init!(&mut (*rm).m_conn_item); rds_message_put(rm); } if !(*cp).cp_xmit_rm.is_null(){rds_message_put((*cp).cp_xmit_rm);} if (*cp).cp_wq!=rds_wq {destroy_workqueue((*cp).cp_wq);(*cp).cp_wq=core::ptr::null_mut();} ((*(*cp).cp_conn).c_trans).conn_free((*cp).cp_transport_data); }

// Information walkers and registration routines retain the same ABI and ordering.
pub unsafe fn rds_conn_init()->c_int { let ret=rds_loop_net_init(); if ret!=0{return ret;} rds_conn_slab=KMEM_CACHE!(rds_connection,0); if rds_conn_slab.is_null(){rds_loop_net_exit();return -ENOMEM;} rds_info_register_func(RDS_INFO_CONNECTIONS,rds_conn_info); rds_info_register_func(RDS_INFO_SEND_MESSAGES,rds_conn_message_info_send); rds_info_register_func(RDS_INFO_RETRANS_MESSAGES,rds_conn_message_info_retrans); 0 }
pub unsafe fn rds_conn_exit(){rds_loop_net_exit();rds_loop_exit();WARN_ON!(!hlist_empty(&rds_conn_hash[0]));kmem_cache_destroy(rds_conn_slab);rds_info_deregister_func(RDS_INFO_CONNECTIONS,rds_conn_info);rds_info_deregister_func(RDS_INFO_SEND_MESSAGES,rds_conn_message_info_send);rds_info_deregister_func(RDS_INFO_RETRANS_MESSAGES,rds_conn_message_info_retrans);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
