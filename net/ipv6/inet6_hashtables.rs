// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET             An implementation of the TCP/IP protocol suite for the LINUX
 *                 operating system.  INET is implemented using the BSD Socket
 *                 interface as the means of communication with the user level.
 *
 *                 Generic INET6 transport hashtables
 *
 * Authors:        Lotsa people, from code originally in tcp, generalised here
 *                 by Arnaldo Carvalho de Melo <acme@mandriva.com>
 */

// Kernel dependencies supplied by other translation units.

pub unsafe fn inet6_init_ehash_secret() {
    net_get_random_sleepable_once(&mut inet6_ehash_secret, core::mem::size_of_val(&inet6_ehash_secret));
    net_get_random_sleepable_once(&mut tcp_ipv6_hash_secret, core::mem::size_of_val(&tcp_ipv6_hash_secret));
}

pub unsafe fn inet6_ehashfn(
    net: *const net,
    laddr: *const in6_addr,
    lport: u16,
    faddr: *const in6_addr,
    fport: __be16,
) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    /*
     * Please look at jhash() implementation for reference.
     * Hash laddr + faddr + lport/fport + net_hash_mix.
     * Notes:
     * We combine laddr[0] (high order 32 bits of local address)
     * with net_hash_mix() to hash a multiple of 3 words.
     *
     * We do not include JHASH_INITVAL + 36 contribution
     * to initial values of a, b, c.
     */
    a = tcp_ipv6_hash_secret;
    b = tcp_ipv6_hash_secret;
    c = tcp_ipv6_hash_secret;

    a = a.wrapping_add((*laddr).s6_addr32[0]) ^ net_hash_mix(net);
    b = b.wrapping_add((*laddr).s6_addr32[1]);
    c = c.wrapping_add((*laddr).s6_addr32[2]);
    __jhash_mix(&mut a, &mut b, &mut c);

    a = a.wrapping_add((*laddr).s6_addr32[3]);
    b = b.wrapping_add((*faddr).s6_addr32[0]);
    c = c.wrapping_add((*faddr).s6_addr32[1]);
    __jhash_mix(&mut a, &mut b, &mut c);

    a = a.wrapping_add((*faddr).s6_addr32[2]);
    b = b.wrapping_add((*faddr).s6_addr32[3]);
    c = c.wrapping_add(fport as u32);
    __jhash_final(&mut a, &mut b, &mut c);

    /* Note: We need to add @lport instead of fully hashing it. */
    (lport as u32).wrapping_add(c)
}

pub unsafe fn __inet6_lookup_established(
    net: *const net,
    saddr: *const in6_addr,
    sport: __be16,
    daddr: *const in6_addr,
    hnum: u16,
    dif: i32,
    sdif: i32,
) -> *mut sock {
    let ports = INET_COMBINED_PORTS(sport, hnum);
    let mut node: *const hlist_nulls_node = core::ptr::null();
    let hashinfo = (*(*net).ipv4.tcp_death_row).hashinfo;
    let hash = inet6_ehashfn(net, daddr, hnum, saddr, sport);
    let slot = hash & (*hashinfo).ehash_mask;
    let head = &(*hashinfo).ehash[slot as usize];
    let mut sk: *mut sock;
    'begin: loop {
        sk_nulls_for_each_rcu!(sk, node, &head.chain) {
            if (*sk).sk_hash != hash { continue; }
            if !inet6_match(net, sk, saddr, daddr, ports, dif, sdif) { continue; }
            if !refcount_inc_not_zero(&mut (*sk).sk_refcnt) { break; }
            if !inet6_match(net, sk, saddr, daddr, ports, dif, sdif) {
                sock_gen_put(sk);
                continue 'begin;
            }
            return sk;
        }
        if get_nulls_value(node) != slot { continue 'begin; }
        return core::ptr::null_mut();
    }
}

unsafe fn compute_score(sk: *mut sock, net: *const net, hnum: u16,
                        daddr: *const in6_addr, dif: i32, sdif: i32) -> i32 {
    let mut score = -1;
    if net_eq(sock_net(sk), net) && READ_ONCE!((*inet_sk(sk)).inet_num) == hnum && (*sk).sk_family == PF_INET6 {
        if !ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, daddr) { return -1; }
        if !inet_sk_bound_dev_eq(net, (*sk).sk_bound_dev_if, dif, sdif) { return -1; }
        score = if (*sk).sk_bound_dev_if != 0 { 2 } else { 1 };
        if READ_ONCE!((*sk).sk_incoming_cpu) == raw_smp_processor_id() { score += 1; }
    }
    score
}

pub unsafe fn inet6_lookup_reuseport(net: *const net, sk: *mut sock, skb: *mut sk_buff,
    doff: i32, saddr: *const in6_addr, sport: __be16, daddr: *const in6_addr,
    hnum: u16, ehashfn: Option<unsafe extern "C" fn(*const net, *const in6_addr, u16, *const in6_addr, __be16) -> u32>) -> *mut sock {
    let mut reuse_sk = core::ptr::null_mut();
    if (*sk).sk_reuseport {
        let phash = INDIRECT_CALL_INET(ehashfn, udp6_ehashfn, inet6_ehashfn, net, daddr, hnum, saddr, sport);
        reuse_sk = reuseport_select_sock(sk, phash, skb, doff);
    }
    reuse_sk
}

unsafe fn inet6_lhash2_lookup(net: *const net, ilb2: *mut inet_listen_hashbucket,
    skb: *mut sk_buff, doff: i32, saddr: *const in6_addr, sport: __be16,
    daddr: *const in6_addr, hnum: u16, dif: i32, sdif: i32) -> *mut sock {
    let mut result = core::ptr::null_mut();
    let mut hiscore = 0;
    let mut node = core::ptr::null_mut();
    let mut sk: *mut sock;
    sk_nulls_for_each_rcu!(sk, node, &(*ilb2).nulls_head) {
        let score = compute_score(sk, net, hnum, daddr, dif, sdif);
        if score > hiscore {
            result = inet6_lookup_reuseport(net, sk, skb, doff, saddr, sport, daddr, hnum, Some(inet6_ehashfn));
            if !result.is_null() { return result; }
            result = sk;
            hiscore = score;
        }
    }
    result
}

pub unsafe fn inet6_lookup_run_sk_lookup(net: *const net, protocol: i32, skb: *mut sk_buff,
    doff: i32, saddr: *const in6_addr, sport: __be16, daddr: *const in6_addr,
    hnum: u16, dif: i32, ehashfn: Option<unsafe extern "C" fn(*const net, *const in6_addr, u16, *const in6_addr, __be16) -> u32>) -> *mut sock {
    let mut sk = core::ptr::null_mut();
    let no_reuseport = bpf_sk_lookup_run_v6(net, protocol, saddr, sport, daddr, hnum, dif, &mut sk);
    if no_reuseport || IS_ERR_OR_NULL(sk) { return sk; }
    let reuse_sk = inet6_lookup_reuseport(net, sk, skb, doff, saddr, sport, daddr, hnum, ehashfn);
    if !reuse_sk.is_null() { sk = reuse_sk; }
    sk
}

pub unsafe fn inet6_lookup_listener(net: *const net, skb: *mut sk_buff, doff: i32,
    saddr: *const in6_addr, sport: __be16, daddr: *const in6_addr, hnum: u16,
    dif: i32, sdif: i32) -> *mut sock {
    let mut result = core::ptr::null_mut();
    if static_branch_unlikely(&bpf_sk_lookup_enabled) {
        result = inet6_lookup_run_sk_lookup(net, IPPROTO_TCP, skb, doff, saddr, sport, daddr, hnum, dif, Some(inet6_ehashfn));
        if !result.is_null() { return if IS_ERR(result) { core::ptr::null_mut() } else { result }; }
    }
    let hashinfo = (*(*net).ipv4.tcp_death_row).hashinfo;
    let mut ilb2 = inet_lhash2_bucket(hashinfo, ipv6_portaddr_hash(net, daddr, hnum));
    result = inet6_lhash2_lookup(net, ilb2, skb, doff, saddr, sport, daddr, hnum, dif, sdif);
    if result.is_null() {
        ilb2 = inet_lhash2_bucket(hashinfo, ipv6_portaddr_hash(net, &in6addr_any, hnum));
        result = inet6_lhash2_lookup(net, ilb2, skb, doff, saddr, sport, &in6addr_any, hnum, dif, sdif);
    }
    if IS_ERR(result) { core::ptr::null_mut() } else { result }
}

pub unsafe fn inet6_lookup(net: *const net, skb: *mut sk_buff, doff: i32,
    saddr: *const in6_addr, sport: __be16, daddr: *const in6_addr, dport: __be16, dif: i32) -> *mut sock {
    let mut refcounted = false;
    let sk = __inet6_lookup(net, skb, doff, saddr, sport, daddr, ntohs(dport), dif, 0, &mut refcounted);
    if !sk.is_null() && !refcounted && !refcount_inc_not_zero(&mut (*sk).sk_refcnt) { return core::ptr::null_mut(); }
    sk
}

unsafe fn __inet6_check_established(death_row: *mut inet_timewait_death_row, sk: *mut sock,
    lport: u16, twp: *mut *mut inet_timewait_sock, rcu_lookup: bool, hash: u32) -> i32 {
    let hinfo = (*death_row).hashinfo;
    let inet = inet_sk(sk);
    let daddr = &(*sk).sk_v6_rcv_saddr;
    let saddr = &(*sk).sk_v6_daddr;
    let dif = (*sk).sk_bound_dev_if;
    let net = sock_net(sk);
    let sdif = l3mdev_master_ifindex_by_index(net, dif);
    let ports = INET_COMBINED_PORTS((*inet).inet_dport, lport);
    let head = inet_ehash_bucket(hinfo, hash);
    let mut tw = core::ptr::null_mut();
    let mut node = core::ptr::null();
    let mut sk2: *mut sock;
    if rcu_lookup {
        sk_nulls_for_each!(sk2, node, &(*head).chain) {
            if (*sk2).sk_hash != hash || !inet6_match(net, sk2, saddr, daddr, ports, dif, sdif) { continue; }
            if (*sk2).sk_state == TCP_TIME_WAIT { break; }
            return -EADDRNOTAVAIL;
        }
        return 0;
    }
    let lock = inet_ehash_lockp(hinfo, hash);
    spin_lock(lock);
    sk_nulls_for_each!(sk2, node, &(*head).chain) {
        if (*sk2).sk_hash != hash { continue; }
        if inet6_match(net, sk2, saddr, daddr, ports, dif, sdif) {
            if (*sk2).sk_state == TCP_TIME_WAIT {
                tw = inet_twsk(sk2);
                if tcp_twsk_unique(sk, sk2, twp) { break; }
            }
            spin_unlock(lock);
            return -EADDRNOTAVAIL;
        }
    }
    (*inet).inet_num = lport;
    (*inet).inet_sport = htons(lport);
    (*sk).sk_hash = hash;
    WARN_ON(!sk_unhashed(sk));
    __sk_nulls_add_node_rcu(sk, &(*head).chain);
    if !tw.is_null() { sk_nulls_del_node_init_rcu(tw as *mut sock); __NET_INC_STATS(net, LINUX_MIB_TIMEWAITRECYCLED); }
    spin_unlock(lock);
    sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, 1);
    if !twp.is_null() { *twp = tw; } else if !tw.is_null() { inet_twsk_deschedule_put(tw); }
    0
}

unsafe fn inet6_sk_port_offset(sk: *const sock) -> u64 {
    let inet = inet_sk(sk as *mut sock);
    secure_ipv6_port_ephemeral((*sk).sk_v6_rcv_saddr.s6_addr32.as_ptr(), (*sk).sk_v6_daddr.s6_addr32.as_ptr(), (*inet).inet_dport)
}

pub unsafe fn inet6_hash_connect(death_row: *mut inet_timewait_death_row, sk: *mut sock) -> i32 {
    let daddr = &(*sk).sk_v6_rcv_saddr;
    let saddr = &(*sk).sk_v6_daddr;
    let inet = inet_sk(sk);
    let net = sock_net(sk);
    let mut port_offset = 0u64;
    if (*inet).inet_num == 0 { port_offset = inet6_sk_port_offset(sk); }
    inet6_init_ehash_secret();
    let hash_port0 = inet6_ehashfn(net, daddr, 0, saddr, (*inet).inet_dport);
    __inet_hash_connect(death_row, sk, port_offset, hash_port0, Some(__inet6_check_established))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
