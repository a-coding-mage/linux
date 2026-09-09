/*
 * Copyright (c) 2007, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, available from the file COPYING in the main directory of
 * this source tree, or the OpenIB.org BSD license.
 */

// Linux kernel dependencies and rds.h are supplied by the surrounding crate.

/*
 * This file implements the receive side of the unconventional congestion
 * management in RDS.
 */

static mut RDS_CONG_GENERATION: atomic_t = ATOMIC_INIT(0);
static mut RDS_CONG_MONITOR: list_head = LIST_HEAD_INIT();
static mut RDS_CONG_MONITOR_LOCK: rwlock_t = DEFINE_RWLOCK();
static mut RDS_CONG_LOCK: spinlock_t = DEFINE_SPINLOCK();
static mut RDS_CONG_TREE: rb_root = RB_ROOT;

unsafe fn rds_cong_tree_walk(addr: *const in6_addr,
                              insert: *mut rds_cong_map) -> *mut rds_cong_map {
    let mut p: *mut *mut rb_node = &mut RDS_CONG_TREE.rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();

    while !(*p).is_null() {
        parent = *p;
        let map = rb_entry(parent, rds_cong_map, m_rb_node);
        let diff = rds_addr_cmp(addr, &(*map).m_addr);
        if diff < 0 {
            p = &mut (*(*p)).rb_left;
        } else if diff > 0 {
            p = &mut (*(*p)).rb_right;
        } else {
            return map;
        }
    }

    if !insert.is_null() {
        rb_link_node(&mut (*insert).m_rb_node, parent, p);
        rb_insert_color(&mut (*insert).m_rb_node, &mut RDS_CONG_TREE);
    }
    core::ptr::null_mut()
}

unsafe fn rds_cong_from_addr(addr: *const in6_addr) -> *mut rds_cong_map {
    let mut map = kzalloc_obj::<rds_cong_map>();
    if map.is_null() {
        return core::ptr::null_mut();
    }

    (*map).m_addr = *addr;
    init_waitqueue_head(&mut (*map).m_waitq);
    INIT_LIST_HEAD(&mut (*map).m_conn_list);

    let mut i: usize = 0;
    while i < RDS_CONG_MAP_PAGES {
        let zp = get_zeroed_page(GFP_KERNEL);
        if zp == 0 {
            break;
        }
        (*map).m_page_addrs[i] = zp;
        i += 1;
    }

    if i != RDS_CONG_MAP_PAGES {
        let mut j = 0;
        while j < RDS_CONG_MAP_PAGES && (*map).m_page_addrs[j] != 0 {
            free_page((*map).m_page_addrs[j]);
            j += 1;
        }
        kfree(map);
        return core::ptr::null_mut();
    }

    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut RDS_CONG_LOCK, &mut flags);
    let ret = rds_cong_tree_walk(addr, map);
    spin_unlock_irqrestore(&mut RDS_CONG_LOCK, flags);

    if ret.is_null() {
        map
    } else {
        kfree(map);
        ret
    }
}

pub unsafe fn rds_cong_add_conn(conn: *mut rds_connection) {
    let mut flags: unsigned_long = 0;
    rdsdebug!("conn %p now on map %p\n", conn, (*conn).c_lcong);
    spin_lock_irqsave(&mut RDS_CONG_LOCK, &mut flags);
    list_add_tail(&mut (*conn).c_map_item, &mut (*(*conn).c_lcong).m_conn_list);
    spin_unlock_irqrestore(&mut RDS_CONG_LOCK, flags);
}

pub unsafe fn rds_cong_remove_conn(conn: *mut rds_connection) {
    let mut flags: unsigned_long = 0;
    rdsdebug!("removing conn %p from map %p\n", conn, (*conn).c_lcong);
    spin_lock_irqsave(&mut RDS_CONG_LOCK, &mut flags);
    list_del_init(&mut (*conn).c_map_item);
    spin_unlock_irqrestore(&mut RDS_CONG_LOCK, flags);
}

pub unsafe fn rds_cong_get_maps(conn: *mut rds_connection) -> i32 {
    (*conn).c_lcong = rds_cong_from_addr(&(*conn).c_laddr);
    (*conn).c_fcong = rds_cong_from_addr(&(*conn).c_faddr);
    if (*conn).c_lcong.is_null() || (*conn).c_fcong.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn rds_cong_queue_updates(map: *mut rds_cong_map) {
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut RDS_CONG_LOCK, &mut flags);
    list_for_each_entry!(conn, (*map).m_conn_list, c_map_item, rds_connection, {
        let cp = &mut (*conn).c_path[0];
        rcu_read_lock();
        if !test_and_set_bit(0, &mut (*conn).c_map_queued) && !rds_destroy_pending((*cp).cp_conn) {
            rds_stats_inc(s_cong_update_queued);
            queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_send_w, 0);
        }
        rcu_read_unlock();
    });
    spin_unlock_irqrestore(&mut RDS_CONG_LOCK, flags);
}

pub unsafe fn rds_cong_map_updated(map: *mut rds_cong_map, portmask: u64) {
    rdsdebug!("waking map %p for %pI4\n", map, &(*map).m_addr);
    rds_stats_inc(s_cong_update_received);
    atomic_inc(&mut RDS_CONG_GENERATION);
    if wq_has_sleeper(&(*map).m_waitq) { wake_up(&mut (*map).m_waitq); }
    if wq_has_sleeper(&rds_poll_waitq) { wake_up_all(&mut rds_poll_waitq); }
    if portmask != 0 && !list_empty(&RDS_CONG_MONITOR) {
        let mut flags: unsigned_long = 0;
        read_lock_irqsave(&mut RDS_CONG_MONITOR_LOCK, &mut flags);
        list_for_each_entry!(rs, RDS_CONG_MONITOR, rs_cong_list, rds_sock, {
            spin_lock(&mut (*rs).rs_lock);
            (*rs).rs_cong_notify |= (*rs).rs_cong_mask & portmask;
            (*rs).rs_cong_mask &= !portmask;
            spin_unlock(&mut (*rs).rs_lock);
            if (*rs).rs_cong_notify != 0 { rds_wake_sk_sleep(rs); }
        });
        read_unlock_irqrestore(&mut RDS_CONG_MONITOR_LOCK, flags);
    }
}

pub unsafe fn rds_cong_updated_since(recent: *mut unsigned_long) -> i32 {
    let gen = atomic_read(&RDS_CONG_GENERATION);
    if *recent == gen { 0 } else { *recent = gen; 1 }
}

pub unsafe fn rds_cong_set_bit(map: *mut rds_cong_map, port: __be16) {
    let i = be16_to_cpu(port) as usize / RDS_CONG_MAP_PAGE_BITS;
    let off = be16_to_cpu(port) as usize % RDS_CONG_MAP_PAGE_BITS;
    set_bit_le(off, (*map).m_page_addrs[i] as *mut core::ffi::c_void);
}

pub unsafe fn rds_cong_clear_bit(map: *mut rds_cong_map, port: __be16) {
    let i = be16_to_cpu(port) as usize / RDS_CONG_MAP_PAGE_BITS;
    let off = be16_to_cpu(port) as usize % RDS_CONG_MAP_PAGE_BITS;
    clear_bit_le(off, (*map).m_page_addrs[i] as *mut core::ffi::c_void);
}

unsafe fn rds_cong_test_bit(map: *mut rds_cong_map, port: __be16) -> i32 {
    let i = be16_to_cpu(port) as usize / RDS_CONG_MAP_PAGE_BITS;
    let off = be16_to_cpu(port) as usize % RDS_CONG_MAP_PAGE_BITS;
    test_bit_le(off, (*map).m_page_addrs[i] as *mut core::ffi::c_void)
}

pub unsafe fn rds_cong_add_socket(rs: *mut rds_sock) {
    let mut flags: unsigned_long = 0;
    write_lock_irqsave(&mut RDS_CONG_MONITOR_LOCK, &mut flags);
    if list_empty(&(*rs).rs_cong_list) { list_add(&mut (*rs).rs_cong_list, &mut RDS_CONG_MONITOR); }
    write_unlock_irqrestore(&mut RDS_CONG_MONITOR_LOCK, flags);
}

pub unsafe fn rds_cong_remove_socket(rs: *mut rds_sock) {
    let mut flags: unsigned_long = 0;
    write_lock_irqsave(&mut RDS_CONG_MONITOR_LOCK, &mut flags);
    list_del_init(&mut (*rs).rs_cong_list);
    write_unlock_irqrestore(&mut RDS_CONG_MONITOR_LOCK, flags);
    spin_lock_irqsave(&mut RDS_CONG_LOCK, &mut flags);
    let map = rds_cong_tree_walk(&(*rs).rs_bound_addr, core::ptr::null_mut());
    spin_unlock_irqrestore(&mut RDS_CONG_LOCK, flags);
    if !map.is_null() && rds_cong_test_bit(map, (*rs).rs_bound_port) != 0 {
        rds_cong_clear_bit(map, (*rs).rs_bound_port);
        rds_cong_queue_updates(map);
    }
}

pub unsafe fn rds_cong_wait(map: *mut rds_cong_map, port: __be16, nonblock: i32, rs: *mut rds_sock) -> i32 {
    if rds_cong_test_bit(map, port) == 0 { return 0; }
    if nonblock != 0 {
        if !rs.is_null() && (*rs).rs_cong_monitor {
            let mut flags: unsigned_long = 0;
            spin_lock_irqsave(&mut (*rs).rs_lock, &mut flags);
            (*rs).rs_cong_mask |= RDS_CONG_MONITOR_MASK(ntohs(port));
            spin_unlock_irqrestore(&mut (*rs).rs_lock, flags);
            if rds_cong_test_bit(map, port) == 0 { return 0; }
        }
        rds_stats_inc(s_cong_send_error);
        return -ENOBUFS;
    }
    rds_stats_inc(s_cong_send_blocked);
    wait_event_interruptible(&mut (*map).m_waitq, rds_cong_test_bit(map, port) == 0)
}

pub unsafe fn rds_cong_exit() {
    loop {
        let node = rb_first(&RDS_CONG_TREE);
        if node.is_null() { break; }
        let map = rb_entry(node, rds_cong_map, m_rb_node);
        rb_erase(&mut (*map).m_rb_node, &mut RDS_CONG_TREE);
        let mut i = 0;
        while i < RDS_CONG_MAP_PAGES && (*map).m_page_addrs[i] != 0 { free_page((*map).m_page_addrs[i]); i += 1; }
        kfree(map);
    }
}

pub unsafe fn rds_cong_update_alloc(conn: *mut rds_connection) -> *mut rds_message {
    let rm = rds_message_map_pages((*(*conn).c_lcong).m_page_addrs, RDS_CONG_MAP_BYTES);
    if !IS_ERR(rm) { (*rm).m_inc.i_hdr.h_flags = RDS_FLAG_CONG_BITMAP; }
    rm
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
