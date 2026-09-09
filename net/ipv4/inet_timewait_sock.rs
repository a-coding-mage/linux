// SPDX-License-Identifier: GPL-2.0-only
/* Generic TIME_WAIT sockets functions. */

// Kernel and networking dependencies are supplied by other translation units.

pub unsafe fn inet_twsk_bind_unhash(
    tw: *mut inet_timewait_sock,
    hashinfo: *mut inet_hashinfo,
) {
    let tb2 = (*tw).tw_tb2;
    let tb = (*tw).tw_tb;

    if tb.is_null() {
        return;
    }

    __sk_del_bind_node(tw as *mut sock);
    (*tw).tw_tb = core::ptr::null_mut();
    (*tw).tw_tb2 = core::ptr::null_mut();
    inet_bind2_bucket_destroy((*hashinfo).bind2_bucket_cachep, tb2);
    inet_bind_bucket_destroy(tb);

    __sock_put(tw as *mut sock);
}

// Must be called with locally disabled BHs.
unsafe fn inet_twsk_kill(tw: *mut inet_timewait_sock) {
    let hashinfo = (*(*tw).tw_dr).hashinfo;
    let lock = inet_ehash_lockp(hashinfo, (*tw).tw_hash);
    let bhead: *mut inet_bind_hashbucket;
    let bhead2: *mut inet_bind_hashbucket;

    spin_lock(lock);
    sk_nulls_del_node_init_rcu(tw as *mut sock);
    spin_unlock(lock);

    bhead = &mut (*hashinfo).bhash[inet_bhashfn(
        twsk_net(tw), (*tw).tw_num, (*hashinfo).bhash_size,
    )] as *mut _;
    bhead2 = inet_bhashfn_portaddr(hashinfo, tw as *mut sock, twsk_net(tw), (*tw).tw_num);

    spin_lock(&mut (*bhead).lock);
    spin_lock(&mut (*bhead2).lock);
    inet_twsk_bind_unhash(tw, hashinfo);
    spin_unlock(&mut (*bhead2).lock);
    spin_unlock(&mut (*bhead).lock);

    refcount_dec(&mut (*(*tw).tw_dr).tw_refcount);
    inet_twsk_put(tw);
}

pub unsafe fn inet_twsk_free(tw: *mut inet_timewait_sock) {
    let owner = (*(*tw).tw_prot).owner;
    tcp_twsk_destructor(tw as *mut sock);
    kmem_cache_free((*(*(*tw).tw_prot).twsk_prot).twsk_slab, tw as *mut _);
    module_put(owner);
}

pub unsafe fn inet_twsk_put(tw: *mut inet_timewait_sock) {
    if refcount_dec_and_test(&mut (*tw).tw_refcnt) {
        inet_twsk_free(tw);
    }
}

unsafe fn inet_twsk_schedule(tw: *mut inet_timewait_sock, timeo: i32) {
    __inet_twsk_schedule(tw, timeo, false);
}

pub unsafe fn inet_twsk_hashdance_schedule(
    tw: *mut inet_timewait_sock,
    sk: *mut sock,
    hashinfo: *mut inet_hashinfo,
    timeo: i32,
) {
    let inet = inet_sk(sk);
    let icsk = inet_csk(sk);
    let lock = inet_ehash_lockp(hashinfo, (*sk).sk_hash);
    let bhead: *mut inet_bind_hashbucket;
    let bhead2: *mut inet_bind_hashbucket;

    bhead = &mut (*hashinfo).bhash[inet_bhashfn(
        twsk_net(tw), (*inet).inet_num, (*hashinfo).bhash_size,
    )] as *mut _;
    bhead2 = inet_bhashfn_portaddr(hashinfo, sk, twsk_net(tw), (*inet).inet_num);

    local_bh_disable();
    spin_lock(&mut (*bhead).lock);
    spin_lock(&mut (*bhead2).lock);

    (*tw).tw_tb = (*icsk).icsk_bind_hash;
    WARN_ON((*icsk).icsk_bind_hash.is_null());
    (*tw).tw_tb2 = (*icsk).icsk_bind2_hash;
    WARN_ON((*icsk).icsk_bind2_hash.is_null());
    sk_add_bind_node(tw as *mut sock, &mut (*(*tw).tw_tb2).owners);

    spin_unlock(&mut (*bhead2).lock);
    spin_unlock(&mut (*bhead).lock);
    spin_lock(lock);

    refcount_set(&mut (*tw).tw_refcnt, 3);
    smp_wmb();
    hlist_nulls_replace_init_rcu(&mut (*sk).sk_nulls_node, &mut (*tw).tw_node);
    sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1);
    inet_twsk_schedule(tw, timeo);

    spin_unlock(lock);
    local_bh_enable();
}

unsafe fn tw_timer_handler(t: *mut timer_list) {
    let tw = timer_container_of!(t, inet_timewait_sock, tw_timer);
    inet_twsk_kill(tw);
}

pub unsafe fn inet_twsk_alloc(
    sk: *const sock,
    dr: *mut inet_timewait_death_row,
    state: i32,
) -> *mut inet_timewait_sock {
    let mut tw: *mut inet_timewait_sock;

    if refcount_read(&(*dr).tw_refcount).wrapping_sub(1) >= READ_ONCE((*dr).sysctl_max_tw_buckets) {
        return core::ptr::null_mut();
    }

    tw = kmem_cache_alloc((*(*(*sk).sk_prot_creator).twsk_prot).twsk_slab, GFP_ATOMIC);
    if !tw.is_null() {
        let inet = inet_sk(sk as *mut sock);
        (*tw).tw_dr = dr;
        (*tw).tw_daddr = (*inet).inet_daddr;
        (*tw).tw_rcv_saddr = (*inet).inet_rcv_saddr;
        (*tw).tw_bound_dev_if = (*sk).sk_bound_dev_if;
        (*tw).tw_tos = (*inet).tos;
        (*tw).tw_num = (*inet).inet_num;
        (*tw).tw_state = TCP_TIME_WAIT;
        (*tw).tw_substate = state;
        (*tw).tw_sport = (*inet).inet_sport;
        (*tw).tw_dport = (*inet).inet_dport;
        (*tw).tw_family = (*sk).sk_family;
        (*tw).tw_reuse = (*sk).sk_reuse;
        (*tw).tw_reuseport = (*sk).sk_reuseport;
        (*tw).tw_hash = (*sk).sk_hash;
        (*tw).tw_ipv6only = 0;
        (*tw).tw_transparent = inet_test_bit(TRANSPARENT, sk);
        (*tw).tw_connect_bind = ((*sk).sk_userlocks & SOCK_CONNECT_BIND) != 0;
        (*tw).tw_prot = (*sk).sk_prot_creator;
        atomic64_set(&mut (*tw).tw_cookie, atomic64_read(&(*sk).sk_cookie));
        twsk_net_set(tw, sock_net(sk as *mut sock));
        timer_setup(&mut (*tw).tw_timer, tw_timer_handler, 0);
        (*tw).tw_refcnt = refcount_t::default();
        refcount_set(&mut (*tw).tw_refcnt, 0);
        __module_get((*(*tw).tw_prot).owner);
        psp_twsk_init(tw, sk);
    }
    tw
}

pub unsafe fn inet_twsk_deschedule_put(tw: *mut inet_timewait_sock) {
    let hashinfo = (*(*tw).tw_dr).hashinfo;
    let lock = inet_ehash_lockp(hashinfo, (*tw).tw_hash);
    spin_lock(lock);
    spin_unlock(lock);
    if timer_shutdown_sync(&mut (*tw).tw_timer) {
        inet_twsk_kill(tw);
    }
    inet_twsk_put(tw);
}

pub unsafe fn __inet_twsk_schedule(tw: *mut inet_timewait_sock, timeo: i32, rearm: bool) {
    if !rearm {
        let kill = timeo <= 4 * HZ;
        __NET_INC_STATS(twsk_net(tw), if kill { LINUX_MIB_TIMEWAITKILLED } else { LINUX_MIB_TIMEWAITED });
        BUG_ON(mod_timer(&mut (*tw).tw_timer, jiffies + timeo));
        refcount_inc(&mut (*(*tw).tw_dr).tw_refcount);
    } else {
        mod_timer_pending(&mut (*tw).tw_timer, jiffies + timeo);
    }
}

pub unsafe fn inet_twsk_purge(hashinfo: *mut inet_hashinfo) {
    let mut head = &mut (*hashinfo).ehash[0] as *mut inet_ehash_bucket;
    let ehash_mask = (*hashinfo).ehash_mask;
    let mut node: *mut hlist_nulls_node = core::ptr::null_mut();
    let mut slot = 0;
    let mut sk: *mut sock;

    while slot <= ehash_mask {
        if hlist_nulls_empty(&mut (*head).chain) {
            slot += 1;
            head = head.add(1);
            continue;
        }
restart_rcu:
        cond_resched();
        rcu_read_lock();
restart:
        sk_nulls_for_each_rcu!(sk, node, &mut (*head).chain, {
            let state = inet_sk_state_load(sk);
            if ((1 << state) & !(TCPF_TIME_WAIT | TCPF_NEW_SYN_RECV)) != 0 || check_net(sock_net(sk)) {
                continue;
            }
            if !refcount_inc_not_zero(&mut (*sk).sk_refcnt) {
                continue;
            }
            if check_net(sock_net(sk)) {
                sock_gen_put(sk);
                goto!(restart);
            }
            rcu_read_unlock();
            local_bh_disable();
            if state == TCP_TIME_WAIT {
                inet_twsk_deschedule_put(inet_twsk(sk));
            } else {
                let req = inet_reqsk(sk);
                inet_csk_reqsk_queue_drop_and_put((*req).rsk_listener, req);
            }
            local_bh_enable();
            goto!(restart_rcu);
        });
        if get_nulls_value(node) != slot {
            goto!(restart);
        }
        rcu_read_unlock();
        slot += 1;
        head = head.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
