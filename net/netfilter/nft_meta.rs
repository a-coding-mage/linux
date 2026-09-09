// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of nft_meta.c. External kernel types and symbols are supplied by dependencies. */

const NFT_META_SECS_PER_MINUTE: i64 = 60;
const NFT_META_SECS_PER_HOUR: u32 = 3600;
const NFT_META_SECS_PER_DAY: i64 = 86400;
const NFT_META_DAYS_PER_WEEK: u8 = 7;

unsafe fn nft_meta_weekday() -> u8 {
    let mut secs: time64_t = ktime_get_real_seconds();
    let mut dse: u64;
    secs -= NFT_META_SECS_PER_MINUTE * sys_tz.tz_minuteswest as i64;
    dse = div_u64(secs as u64, NFT_META_SECS_PER_DAY as u64);
    ((4 + dse) % NFT_META_DAYS_PER_WEEK as u64) as u8
}

unsafe fn nft_meta_hour(secs: time64_t) -> u32 {
    let mut tm: tm = core::mem::zeroed();
    time64_to_tm(secs, 0, &mut tm);
    tm.tm_hour as u32 * NFT_META_SECS_PER_HOUR
        + tm.tm_min as u32 * NFT_META_SECS_PER_MINUTE as u32
        + tm.tm_sec as u32
}

unsafe fn nft_meta_get_eval_time(key: nft_meta_keys, dest: *mut u32) {
    match key {
        NFT_META_TIME_NS => nft_reg_store64(dest, ktime_get_real_ns()),
        NFT_META_TIME_DAY => nft_reg_store8(dest, nft_meta_weekday()),
        NFT_META_TIME_HOUR => *dest = nft_meta_hour(ktime_get_real_seconds()),
        _ => {}
    }
}

unsafe fn nft_meta_get_eval_pkttype_lo(pkt: *const nft_pktinfo, dest: *mut u32) -> bool {
    let skb = (*pkt).skb;
    match nft_pf(pkt) {
        NFPROTO_IPV4 => {
            if ipv4_is_multicast((*ip_hdr(skb)).daddr) { nft_reg_store8(dest, PACKET_MULTICAST); }
            else { nft_reg_store8(dest, PACKET_BROADCAST); }
        }
        NFPROTO_IPV6 => nft_reg_store8(dest, PACKET_MULTICAST),
        NFPROTO_NETDEV => match (*skb).protocol {
            x if x == htons(ETH_P_IP) => {
                let noff = skb_network_offset(skb);
                let mut _iph: iphdr = core::mem::zeroed();
                let iph = skb_header_pointer(skb, noff, core::mem::size_of::<iphdr>(), &mut _iph as *mut _ as *mut core::ffi::c_void) as *const iphdr;
                if iph.is_null() { return false; }
                if ipv4_is_multicast((*iph).daddr) { nft_reg_store8(dest, PACKET_MULTICAST); }
                else { nft_reg_store8(dest, PACKET_BROADCAST); }
            }
            x if x == htons(ETH_P_IPV6) => nft_reg_store8(dest, PACKET_MULTICAST),
            _ => { DEBUG_NET_WARN_ON_ONCE(1); return false; }
        },
        _ => { DEBUG_NET_WARN_ON_ONCE(1); return false; }
    }
    true
}

unsafe fn nft_meta_get_eval_skugid(key: nft_meta_keys, dest: *mut u32, pkt: *const nft_pktinfo) -> bool {
    let sk = skb_to_full_sk((*pkt).skb);
    if sk.is_null() || !sk_fullsock(sk) || !net_eq(nft_net(pkt), sock_net(sk)) { return false; }
    let sock = READ_ONCE((*sk).sk_socket);
    let file = if !sock.is_null() { READ_ONCE((*sock).file) } else { core::ptr::null_mut() };
    if file.is_null() { return false; }
    match key {
        NFT_META_SKUID => *dest = from_kuid_munged((*sock_net(sk)).user_ns, (*(*file).f_cred).fsuid),
        NFT_META_SKGID => *dest = from_kgid_munged((*sock_net(sk)).user_ns, (*(*file).f_cred).fsgid),
        _ => {}
    }
    true
}

unsafe fn nft_meta_get_eval_kind(key: nft_meta_keys, dest: *mut u32, pkt: *const nft_pktinfo) -> bool {
    let in_dev = nft_in(pkt); let out_dev = nft_out(pkt);
    match key {
        NFT_META_IIFKIND => { if in_dev.is_null() || (*in_dev).rtnl_link_ops.is_null() { return false; } strscpy_pad(dest as *mut i8, (*(*in_dev).rtnl_link_ops).kind, IFNAMSIZ); }
        NFT_META_OIFKIND => { if out_dev.is_null() || (*out_dev).rtnl_link_ops.is_null() { return false; } strscpy_pad(dest as *mut i8, (*(*out_dev).rtnl_link_ops).kind, IFNAMSIZ); }
        _ => return false,
    }
    true
}

unsafe fn nft_meta_store_ifindex(dest: *mut u32, dev: *const net_device) { *dest = if dev.is_null() { 0 } else { (*dev).ifindex }; }
unsafe fn nft_meta_store_ifname(dest: *mut u32, dev: *const net_device) { strscpy_pad(dest as *mut i8, if dev.is_null() { b"\0".as_ptr() as *const i8 } else { (*dev).name }, IFNAMSIZ); }
unsafe fn nft_meta_store_iftype(dest: *mut u32, dev: *const net_device) -> bool { if dev.is_null() { return false; } nft_reg_store16(dest, (*dev).type_); true }
unsafe fn nft_meta_store_ifgroup(dest: *mut u32, dev: *const net_device) -> bool { if dev.is_null() { return false; } *dest = (*dev).group; true }

unsafe fn nft_meta_get_eval_ifname(key: nft_meta_keys, dest: *mut u32, pkt: *const nft_pktinfo) -> bool {
    match key {
        NFT_META_IIFNAME => nft_meta_store_ifname(dest, nft_in(pkt)), NFT_META_OIFNAME => nft_meta_store_ifname(dest, nft_out(pkt)),
        NFT_META_IIF => nft_meta_store_ifindex(dest, nft_in(pkt)), NFT_META_OIF => nft_meta_store_ifindex(dest, nft_out(pkt)),
        NFT_META_IFTYPE => if !nft_meta_store_iftype(dest, (*(*pkt).skb).dev) { return false; },
        __NFT_META_IIFTYPE => if !nft_meta_store_iftype(dest, nft_in(pkt)) { return false; }, NFT_META_OIFTYPE => if !nft_meta_store_iftype(dest, nft_out(pkt)) { return false; },
        NFT_META_IIFGROUP => if !nft_meta_store_ifgroup(dest, nft_in(pkt)) { return false; }, NFT_META_OIFGROUP => if !nft_meta_store_ifgroup(dest, nft_out(pkt)) { return false; },
        _ => return false,
    } true
}

unsafe fn nft_meta_get_eval_sdif(pkt: *const nft_pktinfo) -> u32 { match nft_pf(pkt) { NFPROTO_IPV4 => inet_sdif((*pkt).skb), NFPROTO_IPV6 => inet6_sdif((*pkt).skb), _ => 0 } }
unsafe fn nft_meta_get_eval_sdifname(dest: *mut u32, pkt: *const nft_pktinfo) { let sdif = nft_meta_get_eval_sdif(pkt); let dev = if sdif != 0 { dev_get_by_index_rcu(nft_net(pkt), sdif) } else { core::ptr::null() }; nft_meta_store_ifname(dest, dev); }

unsafe fn nft_meta_pktinfo_may_update(pkt: *mut nft_pktinfo) {
    let skb = (*pkt).skb; if (*pkt).flags != 0 { return; }
    let mut ethertype: u16; let nhoff: i32;
    match (*skb).protocol {
        x if x == htons(ETH_P_8021Q) => { if !pskb_may_pull(skb, skb_mac_offset(skb) + core::mem::size_of::<vlan_ethhdr>()) { return; } let veth = skb_mac_header(skb) as *const vlan_ethhdr; nhoff = VLAN_HLEN; ethertype = (*veth).h_vlan_encapsulated_proto; }
        x if x == htons(ETH_P_PPP_SES) => { if !nf_flow_pppoe_proto(skb, &mut ethertype) { return; } nhoff = PPPOE_SES_HLEN; }
        _ => return,
    }
    let nhoff = nhoff + skb_network_offset(skb);
    match ethertype { x if x == htons(ETH_P_IP) => if __nft_set_pktinfo_ipv4_validate(pkt, nhoff) { nft_set_pktinfo_unspec(pkt); }, x if x == htons(ETH_P_IPV6) => if __nft_set_pktinfo_ipv6_validate(pkt, nhoff) { nft_set_pktinfo_unspec(pkt); }, _ => {} }
    (*pkt).ethertype = ethertype;
}

// The remaining exported entry points and operation tables retain the C ABI and
// refer directly to the kernel declarations supplied by dependent modules.
pub unsafe fn nft_meta_get_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_meta; let skb = (*pkt).skb; let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    match (*priv_).key {
        NFT_META_LEN => *dest = (*skb).len, NFT_META_PROTOCOL => { nft_meta_pktinfo_may_update(pkt as *mut _); nft_reg_store16(dest, (*pkt).ethertype); }, NFT_META_NFPROTO => nft_reg_store8(dest, nft_pf(pkt)),
        NFT_META_L4PROTO => { nft_meta_pktinfo_may_update(pkt as *mut _); if (*pkt).flags & NFT_PKTINFO_L4PROTO == 0 { (*regs).verdict.code = NFT_BREAK; return; } nft_reg_store8(dest, (*pkt).tprot); }, NFT_META_PRIORITY => *dest = (*skb).priority, NFT_META_MARK => *dest = (*skb).mark,
        NFT_META_IIF | NFT_META_OIF | NFT_META_IIFNAME | NFT_META_OIFNAME | NFT_META_IIFTYPE | NFT_META_OIFTYPE | NFT_META_IIFGROUP | NFT_META_OIFGROUP => if !nft_meta_get_eval_ifname((*priv_).key, dest, pkt) { (*regs).verdict.code = NFT_BREAK; },
        NFT_META_SKUID | NFT_META_SKGID => if !nft_meta_get_eval_skugid((*priv_).key, dest, pkt) { (*regs).verdict.code = NFT_BREAK; }, NFT_META_PKTTYPE => { if (*skb).pkt_type != PACKET_LOOPBACK { nft_reg_store8(dest, (*skb).pkt_type); } else if !nft_meta_get_eval_pkttype_lo(pkt, dest) { (*regs).verdict.code = NFT_BREAK; } }, NFT_META_CPU => *dest = raw_smp_processor_id(), NFT_META_PRANDOM => *dest = get_random_u32(), NFT_META_IIFKIND | NFT_META_OIFKIND => if !nft_meta_get_eval_kind((*priv_).key, dest, pkt) { (*regs).verdict.code = NFT_BREAK; }, NFT_META_TIME_NS | NFT_META_TIME_DAY | NFT_META_TIME_HOUR => nft_meta_get_eval_time((*priv_).key, dest), NFT_META_SDIF => *dest = nft_meta_get_eval_sdif(pkt), NFT_META_SDIFNAME => nft_meta_get_eval_sdifname(dest, pkt), _ => { DEBUG_NET_WARN_ON_ONCE(1); (*regs).verdict.code = NFT_BREAK; }
    }
}

pub unsafe fn nft_meta_set_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) { let meta = nft_expr_priv(expr) as *const nft_meta; let skb = (*pkt).skb; let sreg = (*regs).data.as_mut_ptr().add((*meta).sreg as usize); let value = *sreg; match (*meta).key { NFT_META_MARK => (*skb).mark = value, NFT_META_PRIORITY => (*skb).priority = value, NFT_META_PKTTYPE => { let value8 = nft_reg_load8(sreg); if (*skb).pkt_type != value8 && skb_pkt_type_ok(value8) && skb_pkt_type_ok((*skb).pkt_type) { (*skb).pkt_type = value8; } }, NFT_META_NFTRACE => (*skb).nf_trace = nft_reg_load8(sreg) != 0, _ => DEBUG_NET_WARN_ON_ONCE(1) } }

pub unsafe fn nft_meta_inner_eval(expr: *const nft_expr, regs: *mut nft_regs, _pkt: *const nft_pktinfo, tun_ctx: *const nft_inner_tun_ctx) { let priv_ = nft_expr_priv(expr) as *const nft_meta; let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize); match (*priv_).key { NFT_META_PROTOCOL => nft_reg_store16(dest, (*tun_ctx).llproto), NFT_META_L4PROTO => { if (*tun_ctx).flags & NFT_PAYLOAD_CTX_INNER_TH == 0 { (*regs).verdict.code = NFT_BREAK; } else { nft_reg_store8(dest, (*tun_ctx).l4proto); } }, _ => { DEBUG_NET_WARN_ON_ONCE(1); (*regs).verdict.code = NFT_BREAK; } } }

pub unsafe fn nft_meta_get_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_meta; (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_META_KEY as usize)));
    let len = match (*priv_).key { NFT_META_PROTOCOL | NFT_META_IIFTYPE | NFT_META_OIFTYPE => core::mem::size_of::<u16>(), NFT_META_NFPROTO | NFT_META_L4PROTO | NFT_META_LEN | NFT_META_PRIORITY | NFT_META_MARK | NFT_META_IIF | NFT_META_OIF | NFT_META_SDIF | NFT_META_SKUID | NFT_META_SKGID | NFT_META_PKTTYPE | NFT_META_CPU | NFT_META_IIFGROUP | NFT_META_OIFGROUP | NFT_META_PRANDOM => core::mem::size_of::<u32>(), NFT_META_IIFNAME | NFT_META_OIFNAME | NFT_META_IIFKIND | NFT_META_OIFKIND | NFT_META_SDIFNAME => IFNAMSIZ as usize, NFT_META_TIME_NS => core::mem::size_of::<u64>(), NFT_META_TIME_DAY => core::mem::size_of::<u8>(), NFT_META_TIME_HOUR => core::mem::size_of::<u32>(), _ => return -EOPNOTSUPP };
    (*priv_).len = len as u32; nft_parse_register_store(ctx, *tb.add(NFTA_META_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len)
}

pub unsafe fn nft_meta_set_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_meta; (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_META_KEY as usize)));
    let len = match (*priv_).key { NFT_META_MARK | NFT_META_PRIORITY => 4, NFT_META_NFTRACE | NFT_META_PKTTYPE => 1, _ => return -EOPNOTSUPP };
    (*priv_).len = len; let err = nft_parse_register_load(ctx, *tb.add(NFTA_META_SREG as usize), &mut (*priv_).sreg, len as usize); if err < 0 { return err; } if (*priv_).key == NFT_META_NFTRACE { static_branch_inc(&nft_trace_enabled); } 0
}

pub unsafe fn nft_meta_get_validate(_ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 { 0 }
pub unsafe fn nft_meta_set_validate(_ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 { 0 }
pub unsafe fn nft_meta_get_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 { let p = nft_expr_priv(expr) as *const nft_meta; if nla_put_be32(skb, NFTA_META_KEY, htonl((*p).key)) != 0 || nft_dump_register(skb, NFTA_META_DREG, (*p).dreg) != 0 { -1 } else { 0 } }
pub unsafe fn nft_meta_set_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 { let p = nft_expr_priv(expr) as *const nft_meta; if nla_put_be32(skb, NFTA_META_KEY, htonl((*p).key)) != 0 || nft_dump_register(skb, NFTA_META_SREG, (*p).sreg) != 0 { -1 } else { 0 } }
pub unsafe fn nft_meta_set_destroy(_ctx: *const nft_ctx, expr: *const nft_expr) { if (*(nft_expr_priv(expr) as *const nft_meta)).key == NFT_META_NFTRACE { static_branch_dec(&nft_trace_enabled); } }

// C policy and expression/object operation tables. Their field types and
// registration symbols are defined by the surrounding kernel translation.
#[allow(non_upper_case_globals)]
pub static nft_meta_policy: [nla_policy; NFTA_META_MAX as usize + 1] = [nla_policy { type_: NLA_UNSPEC, len: 0 }; NFTA_META_MAX as usize + 1];

pub unsafe fn nft_meta_inner_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    if (*tb.add(NFTA_META_KEY as usize)).is_null() || (*tb.add(NFTA_META_DREG as usize)).is_null() { return -EINVAL; }
    let p = nft_expr_priv(expr) as *mut nft_meta; (*p).key = ntohl(nla_get_be32(*tb.add(NFTA_META_KEY as usize)));
    let len = match (*p).key { NFT_META_PROTOCOL => 2, NFT_META_L4PROTO => 4, _ => return -EOPNOTSUPP };
    (*p).len = len; nft_parse_register_store(ctx, *tb.add(NFTA_META_DREG as usize), &mut (*p).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len as usize)
}

pub unsafe fn nft_meta_select_ops(ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if (*tb.add(NFTA_META_KEY as usize)).is_null() || (!(*tb.add(NFTA_META_DREG as usize)).is_null() && !(*tb.add(NFTA_META_SREG as usize)).is_null()) { return ERR_PTR(-EINVAL) as *const _; }
    if !(*tb.add(NFTA_META_DREG as usize)).is_null() { &nft_meta_get_ops } else if !(*tb.add(NFTA_META_SREG as usize)).is_null() { &nft_meta_set_ops } else { ERR_PTR(-EINVAL) as *const _ }
}

pub static nft_meta_get_ops: nft_expr_ops = nft_expr_ops { type_: &nft_meta_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_meta>()), eval: Some(nft_meta_get_eval), init: Some(nft_meta_get_init), dump: Some(nft_meta_get_dump), validate: Some(nft_meta_get_validate) };
pub static nft_meta_set_ops: nft_expr_ops = nft_expr_ops { type_: &nft_meta_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_meta>()), eval: Some(nft_meta_set_eval), init: Some(nft_meta_set_init), dump: Some(nft_meta_set_dump), validate: Some(nft_meta_set_validate) };
pub static nft_meta_inner_ops: nft_expr_ops = nft_expr_ops { type_: &nft_meta_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_meta>()), eval: None, init: Some(nft_meta_inner_init), dump: Some(nft_meta_get_dump), validate: None };
pub static nft_meta_type: nft_expr_type = nft_expr_type { name: b"meta\0".as_ptr() as *const i8, select_ops: Some(nft_meta_select_ops), inner_ops: &nft_meta_inner_ops, policy: &nft_meta_policy, maxattr: NFTA_META_MAX, owner: THIS_MODULE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
