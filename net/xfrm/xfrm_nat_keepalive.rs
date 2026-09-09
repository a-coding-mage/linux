// SPDX-License-Identifier: GPL-2.0-only
/*
 * xfrm_nat_keepalive.c
 *
 * (c) 2024 Eyal Birger <eyal.birger@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut NAT_KEEPALIVE_SK_IPV4: PerCpu<sock_bh_locked> = PerCpu::new(sock_bh_locked {
    bh_lock: INIT_LOCAL_LOCK!(bh_lock),
});
// Preserved from: #if IS_ENABLED(CONFIG_IPV6)
#[cfg(feature = "CONFIG_IPV6")]
static mut NAT_KEEPALIVE_SK_IPV6: PerCpu<sock_bh_locked> = PerCpu::new(sock_bh_locked {
    bh_lock: INIT_LOCAL_LOCK!(bh_lock),
});

#[repr(C)]
struct nat_keepalive {
    net: *mut net,
    family: u16,
    saddr: xfrm_address_t,
    daddr: xfrm_address_t,
    encap_sport: __be16,
    encap_dport: __be16,
    smark: __u32,
}

unsafe fn nat_keepalive_init(ka: *mut nat_keepalive, x: *mut xfrm_state) {
    (*ka).net = xs_net(x);
    (*ka).family = (*x).props.family;
    (*ka).saddr = (*x).props.saddr;
    (*ka).daddr = (*x).id.daddr;
    (*ka).encap_sport = (*x).encap.encap_sport;
    (*ka).encap_dport = (*x).encap.encap_dport;
    (*ka).smark = xfrm_smark_get(0, x);
}

unsafe fn nat_keepalive_send_ipv4(mut skb: *mut sk_buff, ka: *mut nat_keepalive) -> c_int {
    let net = (*ka).net;
    let mut fl4: flowi4 = core::mem::zeroed();
    let rt: *mut rtable;
    let sk: *mut sock;
    let tos: __u8 = 0;
    let err: c_int;

    flowi4_init_output(&mut fl4, 0, (*skb).mark, tos, RT_SCOPE_UNIVERSE, IPPROTO_UDP, 0,
                       (*ka).daddr.a4, (*ka).saddr.a4, (*ka).encap_dport,
                       (*ka).encap_sport, sock_net_uid(net, core::ptr::null_mut()));
    rt = ip_route_output_key(net, &mut fl4);
    if IS_ERR(rt) {
        kfree_skb(skb);
        return PTR_ERR(rt);
    }
    skb_dst_set(skb, &mut (*rt).dst);
    local_lock_nested_bh(&mut NAT_KEEPALIVE_SK_IPV4.bh_lock);
    sk = this_cpu_read!(NAT_KEEPALIVE_SK_IPV4.sock);
    sock_net_set(sk, net);
    err = ip_build_and_send_pkt(skb, sk, fl4.saddr, fl4.daddr, core::ptr::null_mut(), tos);
    sock_net_set(sk, &init_net);
    local_unlock_nested_bh(&mut NAT_KEEPALIVE_SK_IPV4.bh_lock);
    err
}

// Preserved from: #if IS_ENABLED(CONFIG_IPV6)
#[cfg(feature = "CONFIG_IPV6")]
unsafe fn nat_keepalive_send_ipv6(mut skb: *mut sk_buff, ka: *mut nat_keepalive, uh: *mut udphdr) -> c_int {
    let net = (*ka).net;
    let dst: *mut dst_entry;
    let mut fl6: flowi6 = core::mem::zeroed();
    let sk: *mut sock;
    let csum: __wsum;
    let err: c_int;
    csum = skb_checksum(skb, 0, (*skb).len, 0);
    (*uh).check = csum_ipv6_magic(&(*ka).saddr.in6, &(*ka).daddr.in6, (*skb).len, IPPROTO_UDP, csum);
    if (*uh).check == 0 { (*uh).check = CSUM_MANGLED_0; }
    core::ptr::write_bytes(&mut fl6, 0, 1);
    fl6.flowi6_mark = (*skb).mark;
    fl6.saddr = (*ka).saddr.in6;
    fl6.daddr = (*ka).daddr.in6;
    fl6.flowi6_proto = IPPROTO_UDP;
    fl6.fl6_sport = (*ka).encap_sport;
    fl6.fl6_dport = (*ka).encap_dport;
    local_lock_nested_bh(&mut NAT_KEEPALIVE_SK_IPV6.bh_lock);
    sk = this_cpu_read!(NAT_KEEPALIVE_SK_IPV6.sock);
    sock_net_set(sk, net);
    dst = ip6_dst_lookup_flow(net, sk, &mut fl6, core::ptr::null_mut());
    if IS_ERR(dst) {
        local_unlock_nested_bh(&mut NAT_KEEPALIVE_SK_IPV6.bh_lock);
        kfree_skb(skb);
        return PTR_ERR(dst);
    }
    skb_dst_set(skb, dst);
    err = ip6_xmit(sk, skb, &mut fl6, (*skb).mark, core::ptr::null_mut(), 0, 0);
    sock_net_set(sk, &init_net);
    local_unlock_nested_bh(&mut NAT_KEEPALIVE_SK_IPV6.bh_lock);
    err
}

unsafe fn nat_keepalive_send(ka: *mut nat_keepalive) {
    const NAT_KA_HDRS_LEN: usize = core::mem::size_of::<iphdr>().max(core::mem::size_of::<ipv6hdr>()) + core::mem::size_of::<udphdr>();
    const NAT_KA_PAYLOAD: u8 = 0xff;
    let skb = alloc_skb(NAT_KA_HDRS_LEN + core::mem::size_of::<u8>(), GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, NAT_KA_HDRS_LEN);
    skb_put_u8(skb, NAT_KA_PAYLOAD);
    let uh = skb_push(skb, core::mem::size_of::<udphdr>()) as *mut udphdr;
    (*uh).source = (*ka).encap_sport;
    (*uh).dest = (*ka).encap_dport;
    udp_set_len_short(uh, (*skb).len);
    (*uh).check = 0;
    (*skb).mark = (*ka).smark;
    match (*ka).family as c_int {
        AF_INET => { nat_keepalive_send_ipv4(skb, ka); }
        #[cfg(feature = "CONFIG_IPV6")]
        AF_INET6 => { nat_keepalive_send_ipv6(skb, ka, uh); }
        _ => { kfree_skb(skb); }
    }
}

const NAT_KEEPALIVE_BATCH_SIZE: usize = 16;
const NAT_KEEPALIVE_BATCH_FULL: c_int = 1;

#[repr(C)]
struct nat_keepalive_work_ctx {
    batch: [*mut xfrm_state; NAT_KEEPALIVE_BATCH_SIZE],
    nr: c_uint,
    next_run: time64_t,
    now: time64_t,
}

unsafe fn nat_keepalive_work_collect(x: *mut xfrm_state, _count: c_int, ptr: *mut c_void) -> c_int {
    let ctx = &mut *(ptr as *mut nat_keepalive_work_ctx);
    if !READ_ONCE!((*x).nat_keepalive_interval) { return 0; }
    if ctx.nr as usize == ctx.batch.len() { return NAT_KEEPALIVE_BATCH_FULL; }
    xfrm_state_hold(x);
    ctx.batch[ctx.nr as usize] = x;
    ctx.nr += 1;
    0
}

unsafe fn nat_keepalive_work_single(x: *mut xfrm_state, ctx: *mut nat_keepalive_work_ctx) {
    let mut send_keepalive = false;
    let mut ka: nat_keepalive = core::mem::zeroed();
    let mut next_run: time64_t = 0;
    let interval: u32;
    let delta: c_int;
    spin_lock_bh(&mut (*x).lock);
    if (*x).km.state == XFRM_STATE_DEAD { spin_unlock_bh(&mut (*x).lock); return; }
    interval = (*x).nat_keepalive_interval;
    if interval == 0 { spin_unlock_bh(&mut (*x).lock); return; }
    delta = ((*ctx).now - (*x).lastused) as c_int;
    if delta < interval as c_int {
        (*x).nat_keepalive_expiration = (*ctx).now + interval as i64 - delta as i64;
        next_run = (*x).nat_keepalive_expiration;
    } else if (*x).nat_keepalive_expiration > (*ctx).now {
        next_run = (*x).nat_keepalive_expiration;
    } else {
        next_run = (*ctx).now + interval as i64;
        nat_keepalive_init(&mut ka, x);
        send_keepalive = true;
    }
    spin_unlock_bh(&mut (*x).lock);
    if send_keepalive { nat_keepalive_send(&mut ka); }
    if next_run != 0 && ((*ctx).next_run == 0 || next_run < (*ctx).next_run) { (*ctx).next_run = next_run; }
}

unsafe fn nat_keepalive_work(work: *mut work_struct) {
    let mut ctx: nat_keepalive_work_ctx = core::mem::zeroed();
    let mut walk: xfrm_state_walk = core::mem::zeroed();
    let net = container_of!(work, net, xfrm.nat_keepalive_work.work);
    ctx.next_run = 0;
    ctx.now = ktime_get_real_seconds();
    xfrm_state_walk_init(&mut walk, IPPROTO_ESP, core::ptr::null_mut());
    let mut err: c_int;
    loop {
        ctx.nr = 0;
        err = xfrm_state_walk(net, &mut walk, nat_keepalive_work_collect, &mut ctx as *mut _ as *mut c_void);
        local_bh_disable();
        for i in 0..ctx.nr as usize {
            nat_keepalive_work_single(ctx.batch[i], &mut ctx);
            xfrm_state_put(ctx.batch[i]);
        }
        local_bh_enable();
        if err != NAT_KEEPALIVE_BATCH_FULL { break; }
    }
    xfrm_state_walk_done(&mut walk, net);
    if ctx.next_run != 0 {
        schedule_delayed_work(&mut (*net).xfrm.nat_keepalive_work,
                              (ctx.next_run - ctx.now) * HZ as i64);
    }
}

unsafe fn nat_keepalive_sk_init(socks: *mut sock_bh_locked, family: c_ushort) -> c_int {
    let mut sk: *mut sock = core::ptr::null_mut();
    let mut err: c_int = 0;
    for_each_possible_cpu!(i, {
        err = inet_ctl_sock_create(&mut sk, family, SOCK_RAW, IPPROTO_UDP, &init_net);
        if err < 0 { break; }
        per_cpu_ptr!(socks, i).sock = sk;
    });
    if err == 0 { return 0; }
    for_each_possible_cpu!(i, { inet_ctl_sock_destroy(per_cpu_ptr!(socks, i).sock); });
    err
}

unsafe fn nat_keepalive_sk_fini(socks: *mut sock_bh_locked) {
    for_each_possible_cpu!(i, { inet_ctl_sock_destroy(per_cpu_ptr!(socks, i).sock); });
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_nat_keepalive_state_updated(x: *mut xfrm_state) {
    if (*x).nat_keepalive_interval == 0 { return; }
    let net = xs_net(x);
    schedule_delayed_work(&mut (*net).xfrm.nat_keepalive_work, 0);
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_nat_keepalive_net_init(net: *mut net) -> c_int {
    INIT_DELAYED_WORK!(&mut (*net).xfrm.nat_keepalive_work, nat_keepalive_work);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_nat_keepalive_net_fini(net: *mut net) -> c_int {
    disable_delayed_work_sync(&mut (*net).xfrm.nat_keepalive_work);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_nat_keepalive_init(family: c_ushort) -> c_int {
    let mut err = -EAFNOSUPPORT;
    match family as c_int {
        AF_INET => { err = nat_keepalive_sk_init(&mut NAT_KEEPALIVE_SK_IPV4, PF_INET as c_ushort); }
        #[cfg(feature = "CONFIG_IPV6")]
        AF_INET6 => { err = nat_keepalive_sk_init(&mut NAT_KEEPALIVE_SK_IPV6, PF_INET6 as c_ushort); }
        _ => {}
    }
    if err != 0 { pr_err!("xfrm nat keepalive init: failed to init err:%d\n", err); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_nat_keepalive_fini(family: c_ushort) {
    match family as c_int {
        AF_INET => nat_keepalive_sk_fini(&mut NAT_KEEPALIVE_SK_IPV4),
        #[cfg(feature = "CONFIG_IPV6")]
        AF_INET6 => nat_keepalive_sk_fini(&mut NAT_KEEPALIVE_SK_IPV6),
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
