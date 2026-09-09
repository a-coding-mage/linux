/*
 * Copyright (c) 2006, 2019 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, available from the file COPYING in the main directory of
 * this source tree, or the OpenIB.org BSD license.
 */

// Kernel and rds.h declarations are supplied by the surrounding translation.

static mut bind_hash_table: rhashtable = unsafe { core::mem::zeroed() };

static ht_parms: rhashtable_params = rhashtable_params {
    nelem_hint: 768,
    key_len: RDS_BOUND_KEY_LEN,
    key_offset: core::mem::offset_of!(rds_sock, rs_bound_key),
    head_offset: core::mem::offset_of!(rds_sock, rs_bound_node),
    max_size: 16384,
    min_size: 1024,
};

/* Create a key for the bind hash table manipulation.  Port is in network byte
 * order.
 */
#[inline]
unsafe fn __rds_create_bind_key(
    mut key: *mut u8,
    addr: *const in6_addr,
    port: __be16,
    scope_id: __u32,
) {
    core::ptr::copy_nonoverlapping(
        addr as *const u8,
        key,
        core::mem::size_of::<in6_addr>(),
    );
    key = key.add(core::mem::size_of::<in6_addr>());
    core::ptr::copy_nonoverlapping(
        &port as *const __be16 as *const u8,
        key,
        core::mem::size_of::<__be16>(),
    );
    key = key.add(core::mem::size_of::<__be16>());
    core::ptr::copy_nonoverlapping(
        &scope_id as *const __u32 as *const u8,
        key,
        core::mem::size_of::<__u32>(),
    );
}

pub unsafe fn rds_find_bound(
    addr: *const in6_addr,
    port: __be16,
    scope_id: __u32,
) -> *mut rds_sock {
    let mut key = [0u8; RDS_BOUND_KEY_LEN];
    __rds_create_bind_key(key.as_mut_ptr(), addr, port, scope_id);
    rcu_read_lock();
    let mut rs = rhashtable_lookup(&bind_hash_table, key.as_ptr(), ht_parms);
    if !rs.is_null()
        && (sock_flag(rds_rs_to_sk(rs), SOCK_DEAD)
            || !refcount_inc_not_zero(&mut (*rds_rs_to_sk(rs)).sk_refcnt))
    {
        rs = core::ptr::null_mut();
    }
    rcu_read_unlock();
    rdsdebug!("returning rs %p for %pI6c:%u\n", rs, addr, ntohs(port));
    rs
}

/* returns -ve errno or +ve port */
unsafe fn rds_add_bound(
    rs: *mut rds_sock,
    addr: *const in6_addr,
    port: *mut __be16,
    scope_id: __u32,
) -> i32 {
    let mut ret = -EADDRINUSE;
    let (mut rover, last): (u16, u16);
    let mut key = [0u8; RDS_BOUND_KEY_LEN];
    if *port != 0 {
        rover = be16_to_cpu(*port);
        if rover == RDS_FLAG_PROBE_PORT { return -EINVAL; }
        last = rover;
    } else {
        rover = core::cmp::max(get_random_u16(), 2);
        last = rover.wrapping_sub(1);
    }
    loop {
        if rover == 0 { rover = rover.wrapping_add(1); }
        if rover == RDS_FLAG_PROBE_PORT { rover = rover.wrapping_add(1); continue; }
        __rds_create_bind_key(key.as_mut_ptr(), addr, cpu_to_be16(rover), scope_id);
        if !rhashtable_lookup_fast(&bind_hash_table, key.as_ptr(), ht_parms).is_null() {
            rover = rover.wrapping_add(1); continue;
        }
        core::ptr::copy_nonoverlapping(key.as_ptr(), (*rs).rs_bound_key.as_mut_ptr(), core::mem::size_of_val(&(*rs).rs_bound_key));
        (*rs).rs_bound_addr = *addr;
        net_get_random_once(&mut (*rs).rs_hash_initval as *mut _ as *mut u8, core::mem::size_of_val(&(*rs).rs_hash_initval));
        (*rs).rs_bound_port = cpu_to_be16(rover);
        (*rs).rs_bound_node.next = core::ptr::null_mut();
        rds_sock_addref(rs);
        if rhashtable_insert_fast(&mut bind_hash_table, &mut (*rs).rs_bound_node, ht_parms) == 0 {
            *port = (*rs).rs_bound_port;
            (*rs).rs_bound_scope_id = scope_id;
            rdsdebug!("rs %p binding to %pI6c:%d\n", rs, addr, ntohs(*port) as i32);
            ret = 0;
            break;
        } else {
            (*rs).rs_bound_addr = in6addr_any;
            rds_sock_put(rs);
            ret = -ENOMEM;
            break;
        }
    }
    ret
}

pub unsafe fn rds_remove_bound(rs: *mut rds_sock) {
    if ipv6_addr_any(&(*rs).rs_bound_addr) { return; }
    rdsdebug!("rs %p unbinding from %pI6c:%d\n", rs, &(*rs).rs_bound_addr, ntohs((*rs).rs_bound_port));
    rhashtable_remove_fast(&mut bind_hash_table, &mut (*rs).rs_bound_node, ht_parms);
    rds_sock_put(rs);
    (*rs).rs_bound_addr = in6addr_any;
}

pub unsafe fn rds_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    let sk = (*sock).sk;
    let rs = rds_sk_to_rs(sk);
    let mut v6addr: in6_addr = core::mem::zeroed();
    let mut binding_addr: *const in6_addr;
    let trans: *mut rds_transport;
    let mut scope_id: __u32 = 0;
    let mut ret = 0;
    let mut port: __be16;
    if addr_len < offsetofend!(sockaddr, sa_family) { return -EINVAL; }
    if (*uaddr).sa_family == AF_INET {
        let sin = uaddr as *mut sockaddr_in;
        if addr_len < core::mem::size_of::<sockaddr_in>() as i32 || (*sin).sin_addr.s_addr == htonl(INADDR_ANY) || (*sin).sin_addr.s_addr == htonl(INADDR_BROADCAST) || ipv4_is_multicast((*sin).sin_addr.s_addr) { return -EINVAL; }
        ipv6_addr_set_v4mapped((*sin).sin_addr.s_addr, &mut v6addr);
        binding_addr = &v6addr;
        port = (*sin).sin_port;
    } else if (*uaddr).sa_family == AF_INET6 {
        let sin6 = uaddr as *mut sockaddr_in6;
        if addr_len < core::mem::size_of::<sockaddr_in6>() as i32 { return -EINVAL; }
        let addr_type = ipv6_addr_type(&(*sin6).sin6_addr);
        if addr_type & IPV6_ADDR_UNICAST == 0 {
            let addr4 = (*sin6).sin6_addr.s6_addr32[3];
            if addr_type & IPV6_ADDR_MAPPED == 0 || addr4 == htonl(INADDR_ANY) || addr4 == htonl(INADDR_BROADCAST) || ipv4_is_multicast(addr4) { return -EINVAL; }
        }
        if addr_type & IPV6_ADDR_LINKLOCAL != 0 {
            if (*sin6).sin6_scope_id == 0 { return -EINVAL; }
            scope_id = (*sin6).sin6_scope_id;
        }
        binding_addr = &(*sin6).sin6_addr;
        port = (*sin6).sin6_port;
    } else { return -EINVAL; }
    lock_sock(sk);
    if !ipv6_addr_any(&(*rs).rs_bound_addr) { ret = -EINVAL; goto out; }
    if !ipv6_addr_any(&(*rs).rs_conn_addr) && scope_id != 0 && (*rs).rs_bound_scope_id != 0 && scope_id != (*rs).rs_bound_scope_id { ret = -EINVAL; goto out; }
    if !(*rs).rs_transport.is_null() {
        trans = (*rs).rs_transport;
        if (*trans).laddr_check.is_none() || ((*trans).laddr_check.unwrap())(sock_net((*sock).sk), binding_addr, scope_id) != 0 { ret = -ENOPROTOOPT; goto out; }
    } else {
        trans = rds_trans_get_preferred(sock_net((*sock).sk), binding_addr, scope_id);
        if trans.is_null() { ret = -EADDRNOTAVAIL; pr_info_ratelimited!("RDS: %s could not find a transport for %pI6c, load rds_tcp or rds_rdma?\n", __func__, binding_addr); goto out; }
        (*rs).rs_transport = trans;
    }
    sock_set_flag(sk, SOCK_RCU_FREE);
    ret = rds_add_bound(rs, binding_addr, &mut port, scope_id);
    if ret != 0 { (*rs).rs_transport = core::ptr::null_mut(); }
out:
    release_sock(sk);
    ret
}

pub unsafe fn rds_bind_lock_destroy() { rhashtable_destroy(&mut bind_hash_table); }
pub unsafe fn rds_bind_lock_init() -> i32 { rhashtable_init(&mut bind_hash_table, &ht_parms) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
