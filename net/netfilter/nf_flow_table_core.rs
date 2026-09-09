// SPDX-License-Identifier: GPL-2.0-only
// Kernel headers and build-time macros from the C source are supplied by other
// translated units; their names are intentionally preserved here.

static mut flowtable_lock: mutex = unsafe { core::mem::zeroed() };
static mut flowtables: list_head = unsafe { core::mem::zeroed() };
static mut flow_offload_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn flow_offload_fill_dir(flow: *mut flow_offload, dir: flow_offload_tuple_dir) {
    let ft = &mut (*flow).tuplehash[dir as usize].tuple;
    let ctt = &(*(*flow).ct).tuplehash[dir as usize].tuple;
    ft.dir = dir;
    match ctt.src.l3num {
        NFPROTO_IPV4 => { ft.src_v4 = ctt.src.u3.in_; ft.dst_v4 = ctt.dst.u3.in_; }
        NFPROTO_IPV6 => { ft.src_v6 = ctt.src.u3.in6; ft.dst_v6 = ctt.dst.u3.in6; }
        _ => {}
    }
    ft.l3proto = ctt.src.l3num;
    ft.l4proto = ctt.dst.protonum;
    match ctt.dst.protonum {
        IPPROTO_TCP | IPPROTO_UDP => { ft.src_port = ctt.src.u.tcp.port; ft.dst_port = ctt.dst.u.tcp.port; }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn flow_offload_alloc(ct: *mut nf_conn) -> *mut flow_offload {
    if nf_ct_is_dying(ct) { return core::ptr::null_mut(); }
    let flow = kmem_cache_zalloc(flow_offload_cachep, GFP_ATOMIC) as *mut flow_offload;
    if flow.is_null() { return core::ptr::null_mut(); }
    refcount_inc(&mut (*ct).ct_general.use_);
    (*flow).ct = ct;
    flow_offload_fill_dir(flow, FLOW_OFFLOAD_DIR_ORIGINAL);
    flow_offload_fill_dir(flow, FLOW_OFFLOAD_DIR_REPLY);
    if (*ct).status & IPS_SRC_NAT != 0 { __set_bit(NF_FLOW_SNAT, &mut (*flow).flags); }
    if (*ct).status & IPS_DST_NAT != 0 { __set_bit(NF_FLOW_DNAT, &mut (*flow).flags); }
    flow
}

unsafe fn flow_offload_dst_cookie(flow_tuple: *mut flow_offload_tuple) -> u32 {
    if (*flow_tuple).l3proto == NFPROTO_IPV6 { return rt6_get_cookie(dst_rt6_info((*flow_tuple).dst_cache)); }
    0
}

unsafe fn nft_route_dst_fetch(route: *mut nf_flow_route, dir: flow_offload_tuple_dir) -> *mut dst_entry {
    let dst = (*route).tuple[dir as usize].dst;
    (*route).tuple[dir as usize].dst = core::ptr::null_mut();
    dst
}

unsafe fn flow_offload_fill_route(flow: *mut flow_offload, route: *mut nf_flow_route, dir: flow_offload_tuple_dir) -> i32 {
    let ft = &mut (*flow).tuplehash[dir as usize].tuple;
    let dst = nft_route_dst_fetch(route, dir);
    match ft.l3proto {
        NFPROTO_IPV4 => ft.mtu = ip_dst_mtu_maybe_forward(dst, true),
        NFPROTO_IPV6 => ft.mtu = ip6_dst_mtu_maybe_forward(dst, true),
        _ => {}
    }
    ft.iifidx = (*route).tuple[dir as usize].in_.ifindex;
    let mut j = 0;
    let n = (*route).tuple[dir as usize].in_.num_encaps;
    let mut i = n as isize - 1;
    while i >= 0 {
        (*ft).encap[j as usize].id = (*route).tuple[dir as usize].in_.encap[i as usize].id;
        (*ft).encap[j as usize].proto = (*route).tuple[dir as usize].in_.encap[i as usize].proto;
        if (*route).tuple[dir as usize].in_.ingress_vlans & BIT(i as u32) != 0 { ft.in_vlan_ingress |= BIT(j as u32); }
        j += 1; i -= 1;
    }
    ft.tun = (*route).tuple[dir as usize].in_.tun;
    ft.encap_num = n;
    ft.needs_gso_segment = (*route).tuple[dir as usize].out.needs_gso_segment;
    ft.tun_num = (*route).tuple[dir as usize].in_.num_tuns;
    match (*route).tuple[dir as usize].xmit_type {
        FLOW_OFFLOAD_XMIT_DIRECT => {
            if (*route).tuple[(!dir) as usize].in_.num_tuns != 0 { ft.dst_cache = dst; ft.dst_cookie = flow_offload_dst_cookie(ft); } else { dst_release(dst); }
            core::ptr::copy_nonoverlapping((*route).tuple[dir as usize].out.h_dest.as_ptr(), ft.out.h_dest.as_mut_ptr(), ETH_ALEN as usize);
            core::ptr::copy_nonoverlapping((*route).tuple[dir as usize].out.h_source.as_ptr(), ft.out.h_source.as_mut_ptr(), ETH_ALEN as usize);
            ft.out.ifidx = (*route).tuple[dir as usize].out.ifindex;
        }
        FLOW_OFFLOAD_XMIT_XFRM | FLOW_OFFLOAD_XMIT_NEIGH => { ft.ifidx = (*route).tuple[dir as usize].out.ifindex; ft.dst_cache = dst; ft.dst_cookie = flow_offload_dst_cookie(ft); }
        _ => { WARN_ON_ONCE(1); }
    }
    ft.xmit_type = (*route).tuple[dir as usize].xmit_type;
    0
}

unsafe fn nft_flow_dst_release(flow: *mut flow_offload, dir: flow_offload_tuple_dir) { dst_release((*flow).tuplehash[dir as usize].tuple.dst_cache); }

#[no_mangle]
pub unsafe extern "C" fn flow_offload_route_init(flow: *mut flow_offload, route: *mut nf_flow_route) { flow_offload_fill_route(flow, route, FLOW_OFFLOAD_DIR_ORIGINAL); flow_offload_fill_route(flow, route, FLOW_OFFLOAD_DIR_REPLY); (*flow).type_ = NF_FLOW_OFFLOAD_ROUTE; }

#[inline]
unsafe fn nf_flow_has_expired(flow: *const flow_offload) -> bool { nf_flow_timeout_delta((*flow).timeout) <= 0 }

unsafe fn flow_offload_fixup_tcp(ct: *mut nf_conn, tcp_state: u8) {
    let tcp = &mut (*ct).proto.tcp;
    spin_lock_bh(&mut (*ct).lock);
    if tcp.state != tcp_state { tcp.state = tcp_state; }
    if tcp.state == TCP_CONNTRACK_CLOSE { tcp.seen[0].flags |= IP_CT_TCP_FLAG_CLOSE_INIT; }
    tcp.seen[0].td_maxwin = 0; tcp.seen[0].flags &= !IP_CT_TCP_FLAG_MAXACK_SET;
    tcp.seen[1].td_maxwin = 0; tcp.seen[1].flags &= !IP_CT_TCP_FLAG_MAXACK_SET;
    spin_unlock_bh(&mut (*ct).lock);
}

unsafe fn flow_offload_fixup_ct(flow: *mut flow_offload) {
    let ct = (*flow).ct; let net = nf_ct_net(ct); let l4num = nf_ct_protonum(ct); let mut expired; let mut closing = false; let mut offload_timeout = 0u32; let mut timeout: i32;
    if l4num == IPPROTO_TCP {
        let tn = nf_tcp_pernet(net); let tcp_state;
        closing = test_bit(NF_FLOW_CLOSING, &(*flow).flags);
        if closing { flow_offload_fixup_tcp(ct, TCP_CONNTRACK_CLOSE); timeout = READ_ONCE((*tn).timeouts[TCP_CONNTRACK_CLOSE as usize]); expired = false; }
        else { tcp_state = READ_ONCE((*ct).proto.tcp.state); flow_offload_fixup_tcp(ct, tcp_state); timeout = READ_ONCE((*tn).timeouts[tcp_state as usize]); expired = nf_flow_has_expired(flow); }
        offload_timeout = READ_ONCE((*tn).offload_timeout);
    } else if l4num == IPPROTO_UDP {
        let tn = nf_udp_pernet(net); let state = if test_bit(IPS_SEEN_REPLY_BIT, &(*ct).status) { UDP_CT_REPLIED } else { UDP_CT_UNREPLIED };
        timeout = READ_ONCE((*tn).timeouts[state as usize]); expired = nf_flow_has_expired(flow); offload_timeout = READ_ONCE((*tn).offload_timeout);
    } else { return; }
    if expired { timeout -= offload_timeout as i32; }
    if timeout < 0 { timeout = 0; }
    if closing || nf_flow_timeout_delta(READ_ONCE((*ct).timeout)) > timeout { nf_ct_refresh(ct, timeout as u32); }
}

unsafe fn flow_offload_route_release(flow: *mut flow_offload) { nft_flow_dst_release(flow, FLOW_OFFLOAD_DIR_ORIGINAL); nft_flow_dst_release(flow, FLOW_OFFLOAD_DIR_REPLY); }

#[no_mangle]
pub unsafe extern "C" fn flow_offload_free(flow: *mut flow_offload) { if (*flow).type_ == NF_FLOW_OFFLOAD_ROUTE { flow_offload_route_release(flow); } nf_ct_put((*flow).ct); kfree_rcu(flow, rcu_head); }

unsafe fn flow_offload_hash(data: *const core::ffi::c_void, _len: u32, seed: u32) -> u32 { jhash(data, offsetof::<flow_offload_tuple>("__hash"), seed) }
unsafe fn flow_offload_hash_obj(data: *const core::ffi::c_void, _len: u32, seed: u32) -> u32 { let t = data as *const flow_offload_tuple_rhash; jhash(&(*t).tuple as *const _ as *const _, offsetof::<flow_offload_tuple>("__hash"), seed) }
unsafe fn flow_offload_hash_cmp(arg: *mut rhashtable_compare_arg, ptr: *const core::ffi::c_void) -> i32 { let tuple = (*arg).key as *const flow_offload_tuple; let x = ptr as *const flow_offload_tuple_rhash; if memcmp(&(*x).tuple as *const _ as *const _, tuple as *const _ as *const _, offsetof::<flow_offload_tuple>("__hash")) != 0 { 1 } else { 0 } }

static nf_flow_offload_rhash_params: rhashtable_params = rhashtable_params { head_offset: offsetof::<flow_offload_tuple_rhash>("node"), hashfn: Some(flow_offload_hash), obj_hashfn: Some(flow_offload_hash_obj), obj_cmpfn: Some(flow_offload_hash_cmp), automatic_shrinking: true };

#[no_mangle]
pub unsafe extern "C" fn flow_offload_get_timeout(flow: *mut flow_offload) -> usize { let mut timeout = NF_FLOW_TIMEOUT as usize; let net = nf_ct_net((*flow).ct); match nf_ct_protonum((*flow).ct) { IPPROTO_TCP => timeout = (*nf_tcp_pernet(net)).offload_timeout as usize, IPPROTO_UDP => timeout = (*nf_udp_pernet(net)).offload_timeout as usize, _ => {} } timeout }

#[no_mangle]
pub unsafe extern "C" fn flow_offload_add(flow_table: *mut nf_flowtable, flow: *mut flow_offload) -> i32 {
    (*flow).timeout = nf_flowtable_time_stamp + flow_offload_get_timeout(flow);
    let mut err = rhashtable_insert_fast(&mut (*flow_table).rhashtable, &mut (*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY as usize].node, &nf_flow_offload_rhash_params);
    if err < 0 { return err; }
    err = rhashtable_insert_fast(&mut (*flow_table).rhashtable, &mut (*flow).tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL as usize].node, &nf_flow_offload_rhash_params);
    if err < 0 { rhashtable_remove_fast(&mut (*flow_table).rhashtable, &mut (*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY as usize].node, &nf_flow_offload_rhash_params); return err; }
    nf_ct_refresh((*flow).ct, NF_CT_DAY);
    if nf_flowtable_hw_offload(flow_table) { nf_flow_offload_add(flow_table, flow); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn flow_offload_refresh(flow_table: *mut nf_flowtable, flow: *mut flow_offload, force: bool) { let timeout = nf_flowtable_time_stamp + flow_offload_get_timeout(flow); if force || timeout - READ_ONCE((*flow).timeout) > HZ { WRITE_ONCE((*flow).timeout, timeout); } else { return; } if !nf_flowtable_hw_offload(flow_table) || test_bit(NF_FLOW_CLOSING, &(*flow).flags) { return; } if test_bit(NF_FLOW_HW, &(*flow).flags) { nf_flow_offload_refresh(flow_table, flow); } }

unsafe fn flow_offload_del(flow_table: *mut nf_flowtable, flow: *mut flow_offload) { rhashtable_remove_fast(&mut (*flow_table).rhashtable, &mut (*flow).tuplehash[0].node, &nf_flow_offload_rhash_params); rhashtable_remove_fast(&mut (*flow_table).rhashtable, &mut (*flow).tuplehash[1].node, &nf_flow_offload_rhash_params); flow_offload_free(flow); }

#[no_mangle]
pub unsafe extern "C" fn flow_offload_teardown(flow: *mut flow_offload) { clear_bit(IPS_OFFLOAD_BIT, &mut (*(*flow).ct).status); if !test_and_set_bit(NF_FLOW_TEARDOWN, &mut (*flow).flags) { flow_offload_fixup_ct(flow); } }

#[no_mangle]
pub unsafe extern "C" fn flow_offload_lookup(flow_table: *mut nf_flowtable, tuple: *mut flow_offload_tuple) -> *mut flow_offload_tuple_rhash { let tuplehash = rhashtable_lookup(&mut (*flow_table).rhashtable, tuple, &nf_flow_offload_rhash_params); if tuplehash.is_null() { return core::ptr::null_mut(); } let dir = (*tuplehash).tuple.dir; let flow = container_of_tuplehash(tuplehash, dir); if test_bit(NF_FLOW_TEARDOWN, &(*flow).flags) || nf_ct_is_dying((*flow).ct) { return core::ptr::null_mut(); } tuplehash }

// The remaining declarations retain the source's external kernel operations;
// their bodies are translated literally where this file defines behavior.
unsafe fn nf_flow_table_gc_run(flow_table: *mut nf_flowtable) { nf_flow_table_iterate(flow_table, nf_flow_offload_gc_step, core::ptr::null_mut()); }

#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_init(flowtable: *mut nf_flowtable) -> i32 { INIT_DELAYED_WORK(&mut (*flowtable).gc_work, nf_flow_offload_work_gc); flow_block_init(&mut (*flowtable).flow_block); init_rwsem(&mut (*flowtable).flow_block_lock); let err = rhashtable_init(&mut (*flowtable).rhashtable, &nf_flow_offload_rhash_params); if err < 0 { return err; } queue_delayed_work(system_power_efficient_wq, &mut (*flowtable).gc_work, HZ); mutex_lock(&mut flowtable_lock); list_add(&mut (*flowtable).list, &mut flowtables); mutex_unlock(&mut flowtable_lock); 0 }

#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_cleanup(dev: *mut net_device) { mutex_lock(&mut flowtable_lock); list_for_each_entry(flowtable, &mut flowtables, list) { nf_flow_table_gc_cleanup(flowtable, dev); } mutex_unlock(&mut flowtable_lock); }

#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_free(flow_table: *mut nf_flowtable) { mutex_lock(&mut flowtable_lock); list_del(&mut (*flow_table).list); mutex_unlock(&mut flowtable_lock); cancel_delayed_work_sync(&mut (*flow_table).gc_work); nf_flow_table_offload_flush(flow_table); nf_flow_table_iterate(flow_table, nf_flow_table_do_cleanup, core::ptr::null_mut()); nf_flow_table_gc_run(flow_table); nf_flow_table_offload_flush_cleanup(flow_table); rhashtable_destroy(&mut (*flow_table).rhashtable); }

unsafe fn nf_flow_nat_port_tcp(skb: *mut sk_buff, thoff: u32, port: __be16, new_port: __be16) { let tcph = (skb_network_header(skb).add(thoff as usize)) as *mut tcphdr; inet_proto_csum_replace2(&mut (*tcph).check, skb, port, new_port, false); }
unsafe fn nf_flow_nat_port_udp(skb: *mut sk_buff, thoff: u32, port: __be16, new_port: __be16) { let udph = skb_network_header(skb).add(thoff as usize) as *mut udphdr; if (*udph).check != 0 || (*skb).ip_summed == CHECKSUM_PARTIAL { inet_proto_csum_replace2(&mut (*udph).check, skb, port, new_port, false); if (*udph).check == 0 { (*udph).check = CSUM_MANGLED_0; } } }
unsafe fn nf_flow_nat_port(skb: *mut sk_buff, thoff: u32, protocol: u8, port: __be16, new_port: __be16) { match protocol { IPPROTO_TCP => nf_flow_nat_port_tcp(skb, thoff, port, new_port), IPPROTO_UDP => nf_flow_nat_port_udp(skb, thoff, port, new_port), _ => {} } }

#[no_mangle]
pub unsafe extern "C" fn nf_flow_snat_port(flow: *const flow_offload, skb: *mut sk_buff, thoff: u32, protocol: u8, dir: flow_offload_tuple_dir) { let hdr = skb_network_header(skb).add(thoff as usize) as *mut flow_ports; let (port, new_port) = match dir { FLOW_OFFLOAD_DIR_ORIGINAL => { let p=(*hdr).source; let n=(*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY as usize].tuple.dst_port; (*hdr).source=n; (p,n) }, FLOW_OFFLOAD_DIR_REPLY => { let p=(*hdr).dest; let n=(*flow).tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL as usize].tuple.src_port; (*hdr).dest=n; (p,n) }, _ => (0,0) }; nf_flow_nat_port(skb, thoff, protocol, port, new_port); }

#[no_mangle]
pub unsafe extern "C" fn nf_flow_dnat_port(flow: *const flow_offload, skb: *mut sk_buff, thoff: u32, protocol: u8, dir: flow_offload_tuple_dir) { let hdr = skb_network_header(skb).add(thoff as usize) as *mut flow_ports; let (port, new_port) = match dir { FLOW_OFFLOAD_DIR_ORIGINAL => { let p=(*hdr).dest; let n=(*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY as usize].tuple.src_port; (*hdr).dest=n; (p,n) }, FLOW_OFFLOAD_DIR_REPLY => { let p=(*hdr).source; let n=(*flow).tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL as usize].tuple.dst_port; (*hdr).source=n; (p,n) }, _ => (0,0) }; nf_flow_nat_port(skb, thoff, protocol, port, new_port); }

// The following kernel declarations are supplied by translated dependencies.
extern "C" { fn nf_flow_table_gc_cleanup(_: *mut nf_flowtable, _: *mut net_device); fn nf_flow_table_iterate(_: *mut nf_flowtable, _: unsafe fn(*mut nf_flowtable,*mut flow_offload,*mut core::ffi::c_void), _: *mut core::ffi::c_void); fn nf_flow_offload_gc_step(_: *mut nf_flowtable, _: *mut flow_offload, _: *mut core::ffi::c_void); fn nf_flow_offload_work_gc(_: *mut work_struct); fn nf_flow_table_do_cleanup(_: *mut nf_flowtable, _: *mut flow_offload, _: *mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
