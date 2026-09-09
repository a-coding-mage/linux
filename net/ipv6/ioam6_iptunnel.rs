// SPDX-License-Identifier: GPL-2.0+
/* IPv6 IOAM Lightweight Tunnel implementation */

#[repr(C, packed)]
pub struct ioam6_lwt_encap {
    pub eh: ipv6_hopopt_hdr,
    pub pad: [u8; 2],
    pub ioamh: ioam6_hdr,
    pub traceh: ioam6_trace_hdr,
}

#[repr(C)]
pub struct ioam6_lwt_freq { pub k: u32, pub n: u32 }

#[repr(C)]
pub struct ioam6_lwt {
    pub null_rt: rt6_info,
    pub cache: dst_cache,
    pub freq: ioam6_lwt_freq,
    pub pkt_cnt: atomic_t,
    pub mode: u8,
    pub has_tunsrc: bool,
    pub tunsrc: in6_addr,
    pub tundst: in6_addr,
    pub tuninfo: ioam6_lwt_encap,
}

static freq_range: netlink_range_validation = netlink_range_validation {
    min: IOAM6_IPTUNNEL_FREQ_MIN, max: IOAM6_IPTUNNEL_FREQ_MAX,
};

unsafe fn ioam6_lwt_state(lwt: *mut lwtunnel_state) -> *mut ioam6_lwt {
    (*lwt).data as *mut ioam6_lwt
}
unsafe fn ioam6_lwt_info(lwt: *mut lwtunnel_state) -> *mut ioam6_lwt_encap {
    &mut (*ioam6_lwt_state(lwt)).tuninfo
}
unsafe fn ioam6_lwt_trace(lwt: *mut lwtunnel_state) -> *mut ioam6_trace_hdr {
    &mut (*ioam6_lwt_info(lwt)).traceh
}

/* NLA policy is supplied by the kernel dependency layer. */
static ioam6_iptunnel_policy: [nla_policy; IOAM6_IPTUNNEL_MAX as usize + 1] = [nla_policy::default(); IOAM6_IPTUNNEL_MAX as usize + 1];

unsafe fn ioam6_validate_trace_hdr(trace: *mut ioam6_trace_hdr) -> bool {
    let fields: u32;
    if (*trace).type_be32 == 0 || (*trace).remlen == 0 ||
       (*trace).remlen > IOAM6_TRACE_DATA_SIZE_MAX / 4 ||
       (*trace).type_.bit12 != 0 || (*trace).type_.bit13 != 0 ||
       (*trace).type_.bit14 != 0 || (*trace).type_.bit15 != 0 ||
       (*trace).type_.bit16 != 0 || (*trace).type_.bit17 != 0 ||
       (*trace).type_.bit18 != 0 || (*trace).type_.bit19 != 0 ||
       (*trace).type_.bit20 != 0 || (*trace).type_.bit21 != 0 ||
       (*trace).type_.bit23 != 0 { return false; }
    fields = be32_to_cpu((*trace).type_be32);
    (*trace).nodelen = ioam6_trace_compute_nodelen(fields);
    true
}

unsafe fn ioam6_build_state(net: *mut net, nla: *mut nlattr, family: c_uint,
    cfg: *const c_void, ts: *mut *mut lwtunnel_state,
    extack: *mut netlink_ext_ack) -> c_int {
    let mut tb: [*mut nlattr; IOAM6_IPTUNNEL_MAX as usize + 1] = [core::ptr::null_mut(); IOAM6_IPTUNNEL_MAX as usize + 1];
    let mut freq_k: u32; let mut freq_n: u32; let mode: u8;
    if family != AF_INET6 { return -EINVAL; }
    let mut err = nla_parse_nested(tb.as_mut_ptr(), IOAM6_IPTUNNEL_MAX, nla, ioam6_iptunnel_policy.as_ptr(), extack);
    if err < 0 { return err; }
    let k = tb[IOAM6_IPTUNNEL_FREQ_K as usize]; let n = tb[IOAM6_IPTUNNEL_FREQ_N as usize];
    if (k.is_null() && !n.is_null()) || (!k.is_null() && n.is_null()) { NL_SET_ERR_MSG(extack, "freq: missing parameter"); return -EINVAL; }
    if k.is_null() { freq_k = IOAM6_IPTUNNEL_FREQ_MIN; freq_n = IOAM6_IPTUNNEL_FREQ_MIN; }
    else { freq_k = nla_get_u32(k); freq_n = nla_get_u32(n); if freq_k > freq_n { NL_SET_ERR_MSG(extack, "freq: k > n is forbidden"); return -EINVAL; } }
    mode = nla_get_u8_default(tb[IOAM6_IPTUNNEL_MODE as usize], IOAM6_IPTUNNEL_MODE_INLINE);
    if !tb[IOAM6_IPTUNNEL_SRC as usize].is_null() && mode == IOAM6_IPTUNNEL_MODE_INLINE { NL_SET_ERR_MSG(extack, "no tunnel src expected with this mode"); return -EINVAL; }
    if tb[IOAM6_IPTUNNEL_DST as usize].is_null() && mode != IOAM6_IPTUNNEL_MODE_INLINE { NL_SET_ERR_MSG(extack, "this mode needs a tunnel destination"); return -EINVAL; }
    if tb[IOAM6_IPTUNNEL_TRACE as usize].is_null() { NL_SET_ERR_MSG(extack, "missing trace"); return -EINVAL; }
    let trace = nla_data(tb[IOAM6_IPTUNNEL_TRACE as usize]) as *mut ioam6_trace_hdr;
    if !ioam6_validate_trace_hdr(trace) { NL_SET_ERR_MSG_ATTR(extack, tb[IOAM6_IPTUNNEL_TRACE as usize], "invalid trace validation"); return -EINVAL; }
    let len_aligned = ALIGN((*trace).remlen * 4, 8);
    let lwt = lwtunnel_state_alloc(core::mem::size_of::<ioam6_lwt>() + len_aligned);
    if lwt.is_null() { return -ENOMEM; }
    let ilwt = ioam6_lwt_state(lwt);
    err = dst_cache_init(&mut (*ilwt).cache, GFP_ATOMIC); if err != 0 { kfree(lwt as *mut c_void); return err; }
    dst_init(&mut (*ilwt).null_rt.dst, core::ptr::null_mut(), core::ptr::null_mut(), DST_OBSOLETE_NONE, DST_NOCOUNT);
    atomic_set(&mut (*ilwt).pkt_cnt, 0); (*ilwt).freq = ioam6_lwt_freq { k: freq_k, n: freq_n }; (*ilwt).mode = mode;
    (*ilwt).has_tunsrc = !tb[IOAM6_IPTUNNEL_SRC as usize].is_null();
    if (*ilwt).has_tunsrc { (*ilwt).tunsrc = nla_get_in6_addr(tb[IOAM6_IPTUNNEL_SRC as usize]); if ipv6_addr_any(&(*ilwt).tunsrc) { dst_cache_destroy(&mut (*ilwt).cache); kfree(lwt as *mut c_void); return -EINVAL; } }
    if !tb[IOAM6_IPTUNNEL_DST as usize].is_null() { (*ilwt).tundst = nla_get_in6_addr(tb[IOAM6_IPTUNNEL_DST as usize]); if ipv6_addr_any(&(*ilwt).tundst) { dst_cache_destroy(&mut (*ilwt).cache); kfree(lwt as *mut c_void); return -EINVAL; } }
    let tuninfo = ioam6_lwt_info(lwt); (*tuninfo).eh.hdrlen = ((core::mem::size_of::<ioam6_lwt_encap>() + len_aligned) >> 3) - 1; (*tuninfo).pad[0] = IPV6_TLV_PADN; (*tuninfo).ioamh.type_ = IOAM6_TYPE_PREALLOC; (*tuninfo).ioamh.opt_type = IPV6_TLV_IOAM; (*tuninfo).ioamh.opt_len = core::mem::size_of::<ioam6_hdr>() - 2 + core::mem::size_of::<ioam6_trace_hdr>() + (*trace).remlen * 4; core::ptr::copy_nonoverlapping(trace, &mut (*tuninfo).traceh, 1);
    (*lwt).type_ = LWTUNNEL_ENCAP_IOAM6; (*lwt).flags |= LWTUNNEL_STATE_OUTPUT_REDIRECT; *ts = lwt; 0
}

unsafe fn ioam6_do_fill(net: *mut net, skb: *mut sk_buff) -> c_int {
    let trace = (skb_transport_header(skb) as *mut u8).add(core::mem::size_of::<ipv6_hopopt_hdr>() + 2 + core::mem::size_of::<ioam6_hdr>()) as *mut ioam6_trace_hdr;
    let ns = ioam6_namespace(net, (*trace).namespace_id);
    if !ns.is_null() { ioam6_fill_trace_data(skb, ns, trace, false); }
    0
}

unsafe fn ioam6_do_inline(net: *mut net, skb: *mut sk_buff, tuninfo: *mut ioam6_lwt_encap, cache_dst: *mut dst_entry) -> c_int {
    let hdrlen = ((*tuninfo).eh.hdrlen as c_int + 1) << 3;
    let mut err = skb_cow_head(skb, hdrlen + dst_dev_overhead(cache_dst, skb)); if err != 0 { return err; }
    let oldhdr = ipv6_hdr(skb); skb_pull(skb, core::mem::size_of::<ipv6hdr>()); skb_postpull_rcsum(skb, skb_network_header(skb), core::mem::size_of::<ipv6hdr>());
    skb_push(skb, core::mem::size_of::<ipv6hdr>() + hdrlen); skb_reset_network_header(skb); skb_mac_header_rebuild(skb);
    let hdr = ipv6_hdr(skb); core::ptr::copy(hdr, oldhdr, 1); (*tuninfo).eh.nexthdr = (*hdr).nexthdr; skb_set_transport_header(skb, core::mem::size_of::<ipv6hdr>()); skb_postpush_rcsum(skb, hdr as *mut c_void, core::mem::size_of::<ipv6hdr>() + hdrlen as usize); core::ptr::copy_nonoverlapping(tuninfo as *const u8, skb_transport_header(skb) as *mut u8, hdrlen as usize); (*hdr).nexthdr = NEXTHDR_HOP; (*hdr).payload_len = cpu_to_be16((*skb).len - core::mem::size_of::<ipv6hdr>() as u32); err = ioam6_do_fill(net, skb); err
}

unsafe fn ioam6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let orig_dst = skb_dst(skb); let ilwt = ioam6_lwt_state((*orig_dst).lwtstate); let cnt = atomic_fetch_inc(&mut (*ilwt).pkt_cnt);
    if (*skb).protocol != htons(ETH_P_IPV6) || cnt % (*ilwt).freq.n >= (*ilwt).freq.k { if (*skb).protocol != htons(ETH_P_IPV6) { kfree_skb(skb); return -EINVAL; } return (*orig_dst).lwtstate.orig_output(net, sk, skb); }
    let dst = dst_cache_get(&mut (*ilwt).cache); let target = if dst.is_null() { orig_dst } else { dst };
    let err = match (*ilwt).mode { IOAM6_IPTUNNEL_MODE_INLINE => { if (*ipv6_hdr(skb)).nexthdr == NEXTHDR_HOP { 0 } else { ioam6_do_inline(net, skb, &mut (*ilwt).tuninfo, target) } }, _ => 0 };
    if err != 0 { if !dst.is_null() { dst_release(dst); } kfree_skb(skb); return err; }
    if !dst.is_null() { dst_release(dst); } (*orig_dst).lwtstate.orig_output(net, sk, skb)
}

unsafe fn ioam6_destroy_state(lwt: *mut lwtunnel_state) { dst_cache_destroy(&mut (*ioam6_lwt_state(lwt)).cache); }
unsafe fn ioam6_fill_encap_info(_skb: *mut sk_buff, _lwtstate: *mut lwtunnel_state) -> c_int { 0 }
unsafe fn ioam6_encap_nlsize(_lwtstate: *mut lwtunnel_state) -> c_int { 0 }
unsafe fn ioam6_encap_cmp(_a: *mut lwtunnel_state, _b: *mut lwtunnel_state) -> c_int { 0 }

static ioam6_iptun_ops: lwtunnel_encap_ops = lwtunnel_encap_ops { build_state: ioam6_build_state, destroy_state: ioam6_destroy_state, output: ioam6_output, fill_encap: ioam6_fill_encap_info, get_encap_size: ioam6_encap_nlsize, cmp_encap: ioam6_encap_cmp, owner: THIS_MODULE };

pub unsafe extern "C" fn ioam6_iptunnel_init() -> c_int { lwtunnel_encap_add_ops(&ioam6_iptun_ops, LWTUNNEL_ENCAP_IOAM6) }
pub unsafe extern "C" fn ioam6_iptunnel_exit() { lwtunnel_encap_del_ops(&ioam6_iptun_ops, LWTUNNEL_ENCAP_IOAM6); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
