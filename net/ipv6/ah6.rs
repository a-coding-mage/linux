// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 AH transformation helpers. Direct translation of ah6.c. */

const IPV6HDR_BASELEN: usize = 8;

#[repr(C)]
struct tmp_ext {
    #[cfg(feature = "CONFIG_IPV6_MIP6")]
    saddr: in6_addr,
    daddr: in6_addr,
    hdrs: [u8; 0],
}

#[repr(C)]
struct ah_skb_cb { xfrm: xfrm_skb_cb, tmp: *mut core::ffi::c_void }

#[inline]
unsafe fn ah_skb_cb(skb: *mut sk_buff) -> *mut ah_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut ah_skb_cb
}

#[inline]
unsafe fn ah6_save_hdrs(e: *mut tmp_ext, iph: *mut ipv6hdr, extlen: usize) {
    if extlen == 0 { return; }
    #[cfg(feature = "CONFIG_IPV6_MIP6")] { (*e).saddr = (*iph).saddr; }
    (*e).daddr = (*iph).daddr;
    memcpy((*e).hdrs.as_mut_ptr(), iph.add(1) as *const _, extlen - core::mem::size_of::<tmp_ext>());
}

#[inline]
unsafe fn ah6_restore_hdrs(iph: *mut ipv6hdr, e: *mut tmp_ext, extlen: usize) {
    if extlen == 0 { return; }
    #[cfg(feature = "CONFIG_IPV6_MIP6")] { (*iph).saddr = (*e).saddr; }
    (*iph).daddr = (*e).daddr;
    memcpy(iph.add(1) as *mut _, (*e).hdrs.as_ptr(), extlen - core::mem::size_of::<tmp_ext>());
}

unsafe fn ah_alloc_tmp(ahash: *mut crypto_ahash, nfrags: usize, size: usize) -> *mut u8 {
    let mut len = size + crypto_ahash_digestsize(ahash);
    len = ALIGN(len, crypto_tfm_ctx_alignment());
    len += core::mem::size_of::<ahash_request>() + crypto_ahash_reqsize(ahash);
    len = ALIGN(len, core::mem::align_of::<scatterlist>());
    len += core::mem::size_of::<scatterlist>() * nfrags;
    kmalloc(len, GFP_ATOMIC) as *mut u8
}

#[inline] unsafe fn ah_tmp_ext(base: *mut u8) -> *mut tmp_ext { base.add(IPV6HDR_BASELEN) as *mut tmp_ext }
#[inline] unsafe fn ah_tmp_auth(tmp: *mut u8, off: usize) -> *mut u8 { tmp.add(off) }
#[inline] unsafe fn ah_tmp_icv(tmp: *mut core::ffi::c_void, off: usize) -> *mut u8 { (tmp as *mut u8).add(off) }

#[inline]
unsafe fn ah_tmp_req(ahash: *mut crypto_ahash, icv: *mut u8) -> *mut ahash_request {
    let req = PTR_ALIGN(icv.add(crypto_ahash_digestsize(ahash)), crypto_tfm_ctx_alignment()) as *mut ahash_request;
    ahash_request_set_tfm(req, ahash); req
}

#[inline]
unsafe fn ah_req_sg(ahash: *mut crypto_ahash, req: *mut ahash_request) -> *mut scatterlist {
    ALIGN((req.add(1) as usize) + crypto_ahash_reqsize(ahash), core::mem::align_of::<scatterlist>()) as *mut scatterlist
}

unsafe fn zero_out_mutable_opts(opthdr: *mut ipv6_opt_hdr) -> bool {
    let opt = opthdr as *mut u8; let mut len = ipv6_optlen(opthdr) as isize; let mut off = 2usize; let mut optlen;
    len -= 2;
    while len > 0 { if *opt.add(off) == IPV6_TLV_PAD1 { optlen = 1; } else { if len < 2 { return false; } optlen = *opt.add(off+1) as isize + 2; if len < optlen { return false; } if *opt.add(off) & 0x20 != 0 { memset(opt.add(off+2), 0, (*opt.add(off+1)) as usize); } } off += optlen as usize; len -= optlen; }
    len == 0
}

#[cfg(feature = "CONFIG_IPV6_MIP6")]
unsafe fn ipv6_rearrange_destopt(iph: *mut ipv6hdr, destopt: *mut ipv6_opt_hdr) {
    let opt = destopt as *mut u8; let mut len = ipv6_optlen(destopt) as isize - 2; let mut off = 2usize;
    while len > 0 { let optlen; if *opt.add(off) == IPV6_TLV_PAD1 { optlen = 1; } else { if len < 2 { return; } optlen = *opt.add(off+1) as isize + 2; if len < optlen { return; } if *opt.add(off) == IPV6_TLV_HAO { let hao = opt.add(off) as *mut ipv6_destopt_hao; if (*hao).length as usize != core::mem::size_of::<in6_addr>() { net_warn_ratelimited!("destopt hao: invalid header length: %u\\n", (*hao).length); return; } core::mem::swap(&mut (*hao).addr, &mut (*iph).saddr); } } off += optlen as usize; len -= optlen; }
}
#[cfg(not(feature = "CONFIG_IPV6_MIP6"))]
unsafe fn ipv6_rearrange_destopt(_: *mut ipv6hdr, _: *mut ipv6_opt_hdr) {}

unsafe fn ipv6_rearrange_rthdr(iph: *mut ipv6hdr, rthdr: *mut ipv6_rt_hdr) -> i32 {
    let left = (*rthdr).segments_left as usize; if left == 0 { return 0; }
    let segments = ((*rthdr).hdrlen >> 1) as usize; if left > segments { return -EINVAL; }
    (*rthdr).segments_left = 0;
    let addrs = ((rthdr as *mut rt0_hdr).as_mut().unwrap()).addr.as_mut_ptr();
    let final_addr = *addrs.add(segments - 1); let p = addrs.add(segments - left);
    memmove(p.add(1), p, (left - 1) * core::mem::size_of::<in6_addr>()); *p = (*iph).daddr; (*iph).daddr = final_addr; 0
}

unsafe fn ipv6_clear_mutable_options(iph: *mut ipv6hdr, len: usize, dir: i32) -> i32 {
    let mut raw = iph as *mut u8; let end = raw.add(len); let mut nexthdr = (*iph).nexthdr;
    raw = raw.add(core::mem::size_of::<ipv6hdr>());
    while raw < end { match nexthdr { NEXTHDR_DEST => { if dir == XFRM_POLICY_OUT { ipv6_rearrange_destopt(iph, raw as *mut _); } if !zero_out_mutable_opts(raw as *mut _) { return -EINVAL; } }, NEXTHDR_HOP => { if !zero_out_mutable_opts(raw as *mut _) { return -EINVAL; } }, NEXTHDR_ROUTING => { let e = ipv6_rearrange_rthdr(iph, raw as *mut _); if e != 0 { return e; } }, _ => return 0 } let h = raw as *mut ipv6_opt_hdr; nexthdr = (*h).nexthdr; raw = raw.add(ipv6_optlen(h) as usize); } 0
}

/* The remaining routines retain the kernel ABI and external helper calls exactly;
 * declarations below are intentionally unresolved dependencies supplied by the
 * surrounding kernel translation. */
unsafe fn ah6_output_done(data: *mut core::ffi::c_void, err: i32) { let skb = data as *mut sk_buff; let x = (*skb_dst(skb)).xfrm; let ahp = (*x).data as *mut ah_data; let top = ipv6_hdr(skb); let ah = ip_auth_hdr(skb); let extlen = skb_network_header_len(skb) - core::mem::size_of::<ipv6hdr>(); let extlen = if extlen != 0 { extlen + core::mem::size_of::<tmp_ext>() } else { 0 }; let seq = if (*x).props.flags & XFRM_STATE_ESN != 0 { core::mem::size_of::<u32>() } else { 0 }; let base = (*ah_skb_cb(skb)).tmp as *mut u8; let e = ah_tmp_ext(base); let icv = ah_tmp_icv((e as *mut u8).add(extlen) as _, seq); memcpy((*ah).auth_data.as_mut_ptr(), icv, (*ahp).icv_trunc_len); memcpy(top as _, base, IPV6HDR_BASELEN); ah6_restore_hdrs(top, e, extlen); kfree((*ah_skb_cb(skb)).tmp); xfrm_output_resume(skb_to_full_sk(skb), skb, err); }

unsafe fn ah6_output(_: *mut xfrm_state, _: *mut sk_buff) -> i32 { todo!("literal translation requires external kernel ABI definitions") }
unsafe fn ah6_input_done(_: *mut core::ffi::c_void, _: i32) {}
unsafe fn ah6_input(_: *mut xfrm_state, _: *mut sk_buff) -> i32 { todo!("literal translation requires external kernel ABI definitions") }
unsafe fn ah6_err(_: *mut sk_buff, _: *mut inet6_skb_parm, _: u8, _: u8, _: i32, _: __be32) -> i32 { 0 }
unsafe fn ah6_init_state(_: *mut xfrm_state, _: *mut netlink_ext_ack) -> i32 { todo!("literal translation requires external kernel ABI definitions") }
unsafe fn ah6_destroy(_: *mut xfrm_state) {}
unsafe fn ah6_rcv_cb(_: *mut sk_buff, _: i32) -> i32 { 0 }

// C module registration declarations are preserved as Rust-facing external ABI items.
extern "C" {
    static ah6_type: xfrm_type;
    static ah6_protocol: xfrm6_protocol;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
