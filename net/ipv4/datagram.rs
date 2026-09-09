// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * common UDP/RAW code
 * Linux INET implementation
 *
 * Authors:
 * Hideaki YOSHIFUJI <yoshfuji@linux-ipv6.org>
 */

// C dependencies supplied by the surrounding kernel translation.

pub unsafe fn __ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr_unsized,
    addr_len: i32,
) -> i32 {
    let inet: *mut inet_sock = inet_sk(sk);
    let usin: *mut sockaddr_in = uaddr as *mut sockaddr_in;
    let fl4: *mut flowi4;
    let mut rt: *mut rtable;
    let mut saddr: __be32;
    let mut oif: i32;
    let mut err: i32;

    if addr_len < core::mem::size_of::<sockaddr_in>() as i32 {
        return -EINVAL;
    }

    if (*usin).sin_family != AF_INET {
        return -EAFNOSUPPORT;
    }

    sk_dst_reset(sk);

    oif = (*sk).sk_bound_dev_if;
    saddr = (*inet).inet_saddr;
    if ipv4_is_multicast((*usin).sin_addr.s_addr) {
        if oif == 0 || netif_index_is_l3_master(sock_net(sk), oif) {
            oif = core::ptr::read_volatile(&(*inet).mc_index);
        }
        if saddr == 0 {
            saddr = core::ptr::read_volatile(&(*inet).mc_addr);
        }
    } else if oif == 0 {
        oif = core::ptr::read_volatile(&(*inet).uc_index);
    }
    fl4 = &mut (*inet).cork.fl.u.ip4;
    rt = ip_route_connect(
        fl4,
        (*usin).sin_addr.s_addr,
        saddr,
        oif,
        (*sk).sk_protocol,
        (*inet).inet_sport,
        (*usin).sin_port,
        sk,
    );
    if IS_ERR(rt) {
        err = PTR_ERR(rt);
        if err == -ENETUNREACH {
            IP_INC_STATS(sock_net(sk), IPSTATS_MIB_OUTNOROUTES);
        }
        return err;
    }

    if ((*rt).rt_flags & RTCF_BROADCAST) != 0 && !sock_flag(sk, SOCK_BROADCAST) {
        ip_rt_put(rt);
        err = -EACCES;
        return err;
    }

    /* Update addresses before rehashing */
    core::ptr::write_volatile(&mut (*inet).inet_daddr, (*fl4).daddr);
    (*inet).inet_dport = (*usin).sin_port;
    if (*inet).inet_saddr == 0 {
        (*inet).inet_saddr = (*fl4).saddr;
    }
    if (*inet).inet_rcv_saddr == 0 {
        core::ptr::write_volatile(&mut (*inet).inet_rcv_saddr, (*fl4).saddr);
        if let Some(rehash) = (*(*sk).sk_prot).rehash {
            rehash(sk);
        }
    }
    reuseport_has_conns_set(sk);
    (*sk).sk_state = TCP_ESTABLISHED;
    sk_set_txhash(sk);
    atomic_set(&mut (*inet).inet_id, get_random_u16());

    sk_dst_set(sk, &mut (*rt).dst);
    err = 0;
    err
}

pub unsafe fn ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr_unsized,
    addr_len: i32,
) -> i32 {
    let res: i32;

    lock_sock(sk);
    res = __ip4_datagram_connect(sk, uaddr, addr_len);
    release_sock(sk);
    res
}

/* Because UDP xmit path can manipulate sk_dst_cache without holding
 * socket lock, we need to use sk_dst_set() here,
 * even if we own the socket lock.
 */
pub unsafe fn ip4_datagram_release_cb(sk: *mut sock) {
    let inet: *const inet_sock = inet_sk(sk);
    let mut dst: *mut dst_entry;
    let mut fl4: flowi4 = core::mem::zeroed();
    let mut rt: *mut rtable;

    rcu_read_lock();

    dst = __sk_dst_get(sk);
    if dst.is_null() || core::ptr::read_volatile(&(*dst).obsolete) == 0 || ((*dst).ops).check(dst, 0) != 0 {
        rcu_read_unlock();
        return;
    }

    inet_sk_init_flowi4(inet, &mut fl4);
    rt = ip_route_output_flow(sock_net(sk), &mut fl4, sk);
    dst = if !IS_ERR(rt) { &mut (*rt).dst } else { core::ptr::null_mut() };
    sk_dst_set(sk, dst);

    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
