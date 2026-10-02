// SPDX-License-Identifier: GPL-2.0-or-later
/* NEW RECONSTRUCTION from the unchanged net/ipv6/ah6.c.
 * This is not a byte-identical recovery of the unavailable historical provider.
 * Kernel layouts and declarations are generated from the selected C headers.
 * C helpers contain only macro, field-layout and header-inline boundaries. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ah6_generated.rs"));
}
use bindings::*;
use core::mem::{align_of, size_of};
use core::ptr::{addr_of, addr_of_mut, copy, copy_nonoverlapping, null_mut, write_bytes};
#[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
use core::ptr::{read_unaligned, write_unaligned};
use kernel::ffi::{c_int, c_void};

const IPV6HDR_BASELEN: u32 = 8;

#[inline]
fn neg(errno: u32) -> c_int { -(errno as c_int) }
#[inline]
fn align(value: usize, alignment: usize) -> usize {
    value.wrapping_add(alignment.wrapping_sub(1)) & !alignment.wrapping_sub(1)
}
#[inline]
fn align8(value: usize) -> usize { align(value, 8) }
#[inline]
unsafe fn ah_skb_cb(skb: *mut sk_buff) -> *mut ah6_skb_cb {
    addr_of_mut!((*skb).cb).cast()
}
#[inline]
unsafe fn ah6_save_hdrs(ext: *mut ah6_tmp_ext, iph: *mut ipv6hdr, extlen: c_int) {
    if extlen == 0 { return; }
    #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
    { (*ext).saddr = *rust_ah6_saddr(iph); }
    (*ext).daddr = *rust_ah6_daddr(iph);
    copy_nonoverlapping(iph.add(1).cast::<u8>(), addr_of_mut!((*ext).hdrs).cast(),
                       (extlen as usize).wrapping_sub(size_of::<ah6_tmp_ext>()));
}
#[inline]
unsafe fn ah6_restore_hdrs(iph: *mut ipv6hdr, ext: *mut ah6_tmp_ext, extlen: c_int) {
    if extlen == 0 { return; }
    #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
    { *rust_ah6_saddr(iph) = (*ext).saddr; }
    *rust_ah6_daddr(iph) = (*ext).daddr;
    copy_nonoverlapping(addr_of!((*ext).hdrs).cast::<u8>(), iph.add(1).cast(),
                       (extlen as usize).wrapping_sub(size_of::<ah6_tmp_ext>()));
}

unsafe fn ah_alloc_tmp(ahash: *mut crypto_ahash, nfrags: c_int, size: u32) -> *mut u8 {
    // The original accumulator is unsigned int, including its conversions.
    let mut len = size.wrapping_add(rust_ah6_digestsize(ahash));
    len = align(len as usize, rust_ah6_ctx_alignment() as usize) as u32;
    len = len.wrapping_add(size_of::<ahash_request>() as u32)
             .wrapping_add(rust_ah6_reqsize(ahash));
    len = align(len as usize, align_of::<scatterlist>()) as u32;
    len = len.wrapping_add((size_of::<scatterlist>() as u32).wrapping_mul(nfrags as u32));
    rust_ah6_kmalloc_atomic(len).cast()
}
#[inline]
unsafe fn ah_tmp_ext(base: *mut u8) -> *mut ah6_tmp_ext {
    base.add(IPV6HDR_BASELEN as usize).cast()
}
#[inline]
unsafe fn ah_tmp_auth(base: *mut u8, offset: u32) -> *mut u8 { base.add(offset as usize) }
#[inline]
unsafe fn ah_tmp_icv(base: *mut u8, offset: u32) -> *mut u8 { base.add(offset as usize) }
#[inline]
unsafe fn ah_tmp_req(ahash: *mut crypto_ahash, icv: *mut u8) -> *mut ahash_request {
    let req = align(icv.add(rust_ah6_digestsize(ahash) as usize) as usize,
                    rust_ah6_ctx_alignment() as usize) as *mut ahash_request;
    rust_ah6_request_set_tfm(req, ahash);
    req
}
#[inline]
unsafe fn ah_req_sg(ahash: *mut crypto_ahash, req: *mut ahash_request) -> *mut scatterlist {
    align((req.add(1) as usize).wrapping_add(rust_ah6_reqsize(ahash) as usize),
          align_of::<scatterlist>()) as *mut scatterlist
}
#[inline]
unsafe fn ipv6_optlen(hdr: *const ipv6_opt_hdr) -> c_int { ((*hdr).hdrlen as c_int + 1) << 3 }
#[inline]
unsafe fn ipv6_authlen(hdr: *const ip_auth_hdr) -> c_int { ((*hdr).hdrlen as c_int + 2) << 2 }

unsafe fn zero_out_mutable_opts(opthdr: *mut ipv6_opt_hdr) -> bool {
    let opt = opthdr.cast::<u8>();
    let mut len = ipv6_optlen(opthdr) - 2;
    let mut off = 2usize;
    while len > 0 {
        let optlen;
        if *opt.add(off) as u32 == IPV6_TLV_PAD1 {
            optlen = 1;
        } else {
            if len < 2 { return false; }
            optlen = *opt.add(off.wrapping_add(1)) as c_int + 2;
            if len < optlen { return false; }
            if *opt.add(off) & 0x20 != 0 {
                write_bytes(opt.add(off.wrapping_add(2)), 0, *opt.add(off.wrapping_add(1)) as usize);
            }
        }
        off = off.wrapping_add(optlen as usize);
        len -= optlen;
    }
    len == 0
}

#[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
unsafe fn ipv6_rearrange_destopt(iph: *mut ipv6hdr, destopt: *mut ipv6_opt_hdr) {
    let opt = destopt.cast::<u8>();
    let mut len = ipv6_optlen(destopt) - 2;
    let mut off = 2usize;
    while len > 0 {
        let optlen;
        if *opt.add(off) as u32 == IPV6_TLV_PAD1 {
            optlen = 1;
        } else {
            if len < 2 { return; }
            optlen = *opt.add(off.wrapping_add(1)) as c_int + 2;
            if len < optlen { return; }
            if *opt.add(off) as u32 == IPV6_TLV_HAO {
                let hao = opt.add(off).cast::<ipv6_destopt_hao>();
                if (*hao).length as usize != size_of::<in6_addr>() {
                    rust_ah6_warn_hao((*hao).length);
                    return;
                }
                // HAO is packed; never form a Rust reference to its address.
                let saved = read_unaligned(addr_of!((*hao).addr));
                write_unaligned(addr_of_mut!((*hao).addr), *rust_ah6_saddr(iph));
                *rust_ah6_saddr(iph) = saved;
            }
        }
        off = off.wrapping_add(optlen as usize);
        len -= optlen;
    }
}
#[cfg(not(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE)))]
unsafe fn ipv6_rearrange_destopt(_: *mut ipv6hdr, _: *mut ipv6_opt_hdr) {}

unsafe fn ipv6_rearrange_rthdr(iph: *mut ipv6hdr, rthdr: *mut ipv6_rt_hdr) -> c_int {
    let left = (*rthdr).segments_left as usize;
    if left == 0 { return 0; }
    let segments = ((*rthdr).hdrlen >> 1) as usize;
    // Preserve the C guard for malformed locally generated routing headers.
    if left > segments { return neg(EINVAL); }
    (*rthdr).segments_left = 0;
    let addrs = addr_of_mut!((*rthdr.cast::<rt0_hdr>()).addr).cast::<in6_addr>();
    let final_addr = *addrs.add(segments - 1);
    let addrs = addrs.add(segments - left);
    copy(addrs, addrs.add(1), left - 1);
    *addrs = *rust_ah6_daddr(iph);
    *rust_ah6_daddr(iph) = final_addr;
    0
}

unsafe fn ipv6_clear_mutable_options(iph: *mut ipv6hdr, len: c_int, dir: u32) -> c_int {
    let mut raw = iph.add(1).cast::<u8>();
    let end = iph.cast::<u8>().offset(len as isize);
    let mut nexthdr = (*iph).nexthdr as u32;
    while raw < end {
        match nexthdr {
            NEXTHDR_DEST | NEXTHDR_HOP => {
                if nexthdr == NEXTHDR_DEST && dir == XFRM_POLICY_OUT {
                    ipv6_rearrange_destopt(iph, raw.cast());
                }
                if !zero_out_mutable_opts(raw.cast()) {
                    rust_ah6_debug_overrun(nexthdr == NEXTHDR_HOP);
                    return neg(EINVAL);
                }
            }
            NEXTHDR_ROUTING => {
                let err = ipv6_rearrange_rthdr(iph, raw.cast());
                if err != 0 { return err; }
            }
            _ => return 0,
        }
        let hdr = raw.cast::<ipv6_opt_hdr>();
        nexthdr = (*hdr).nexthdr as u32;
        raw = raw.add(ipv6_optlen(hdr) as usize);
    }
    0
}

#[inline]
unsafe fn output_extlen(skb: *mut sk_buff) -> c_int {
    let mut extlen = rust_ah6_network_header_len(skb).wrapping_sub(size_of::<ipv6hdr>() as u32) as c_int;
    if extlen != 0 { extlen = extlen.wrapping_add(size_of::<ah6_tmp_ext>() as c_int); }
    extlen
}
#[inline]
unsafe fn has_esn(x: *mut xfrm_state) -> bool { (*x).props.flags as u32 & XFRM_STATE_ESN != 0 }
#[inline]
unsafe fn clear_base_mutable(iph: *mut ipv6hdr) {
    rust_ah6_set_priority(iph, 0);
    (*iph).flow_lbl = [0; 3];
    (*iph).hop_limit = 0;
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_output_done(data: *mut c_void, err: c_int) {
    let skb = data.cast::<sk_buff>();
    let x = (*rust_ah6_skb_dst(skb)).xfrm;
    let ahp = (*x).data.cast::<ah_data>();
    let top_iph = rust_ah6_ipv6_hdr(skb);
    let ah = rust_ah6_auth_hdr(skb);
    let extlen = output_extlen(skb);
    let seqhi_len = if has_esn(x) { size_of::<u32>() as u32 } else { 0 };
    let base = (*ah_skb_cb(skb)).tmp.cast::<u8>();
    let ext = ah_tmp_ext(base);
    let seqhi = ext.cast::<u8>().offset(extlen as isize);
    let icv = ah_tmp_icv(seqhi, seqhi_len);
    // Original C restores/copies even on an asynchronous crypto error, then
    // transfers the error to xfrm_output_resume. Preserve that ordering.
    copy_nonoverlapping(icv, addr_of_mut!((*ah).auth_data).cast(), (*ahp).icv_trunc_len as usize);
    copy_nonoverlapping(base, top_iph.cast(), IPV6HDR_BASELEN as usize);
    ah6_restore_hdrs(top_iph, ext, extlen);
    kfree((*ah_skb_cb(skb)).tmp);
    xfrm_output_resume(rust_ah6_skb_to_full_sk(skb), skb, err);
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let ahp = (*x).data.cast::<ah_data>();
    let ahash = (*ahp).ahash;
    let mut trailer = null_mut();
    let nfrags = skb_cow_data(skb, 0, addr_of_mut!(trailer));
    if nfrags < 0 { return nfrags; }
    skb_push(skb, rust_ah6_network_offset(skb).wrapping_neg() as u32);
    let extlen = output_extlen(skb);
    let esn = has_esn(x);
    let sglists = if esn { 1 } else { 0 };
    let seqhi_len = if esn { size_of::<u32>() as u32 } else { 0 };
    let base = ah_alloc_tmp(ahash, nfrags.wrapping_add(sglists),
        IPV6HDR_BASELEN.wrapping_add(extlen as u32).wrapping_add(seqhi_len));
    if base.is_null() { return neg(ENOMEM); }
    let ext = ah_tmp_ext(base);
    let seqhi = ext.cast::<u8>().offset(extlen as isize).cast::<u32>();
    let icv = ah_tmp_icv(seqhi.cast(), seqhi_len);
    let req = ah_tmp_req(ahash, icv);
    let sg = ah_req_sg(ahash, req);
    let seqhisg = sg.add(nfrags as usize);
    let ah = rust_ah6_auth_hdr(skb);
    write_bytes(addr_of_mut!((*ah).auth_data).cast::<u8>(), 0, (*ahp).icv_trunc_len as usize);
    let top_iph = rust_ah6_ipv6_hdr(skb);
    (*top_iph).payload_len = ((*skb).len.wrapping_sub(size_of::<ipv6hdr>() as u32) as u16).to_be();
    let nexthdr = *rust_ah6_mac_header(skb);
    *rust_ah6_mac_header(skb) = IPPROTO_AH as u8;
    copy_nonoverlapping(top_iph.cast::<u8>(), base, IPV6HDR_BASELEN as usize);
    ah6_save_hdrs(ext, top_iph, extlen);

    let err = 'out_free: {
        if extlen != 0 {
            let err = ipv6_clear_mutable_options(top_iph,
                extlen.wrapping_sub(size_of::<ah6_tmp_ext>() as c_int)
                      .wrapping_add(size_of::<ipv6hdr>() as c_int), XFRM_POLICY_OUT);
            if err != 0 { break 'out_free err; }
        }
        (*ah).nexthdr = nexthdr;
        clear_base_mutable(top_iph);
        (*ah).hdrlen = (align8(size_of::<ip_auth_hdr>().wrapping_add((*ahp).icv_trunc_len as usize)) >> 2).wrapping_sub(2) as u8;
        (*ah).reserved = 0;
        (*ah).spi = (*x).id.spi;
        (*ah).seq_no = (*ah_skb_cb(skb)).xfrm.seq.output.low.to_be();
        sg_init_table(sg, nfrags.wrapping_add(sglists) as u32);
        let err = skb_to_sgvec_nomark(skb, sg, 0, (*skb).len as c_int);
        if err < 0 { break 'out_free err; }
        if has_esn(x) {
            *seqhi = (*ah_skb_cb(skb)).xfrm.seq.output.hi.to_be();
            rust_ah6_sg_set_buf(seqhisg, seqhi.cast(), seqhi_len);
        }
        rust_ah6_request_set_crypt(req, sg, icv, (*skb).len.wrapping_add(seqhi_len));
        rust_ah6_request_set_callback(req, 0, Some(rust_ah6_output_done), skb.cast());
        (*ah_skb_cb(skb)).tmp = base.cast();
        let err = crypto_ahash_digest(req);
        if err != 0 {
            if err == neg(EINPROGRESS) {
                // The completion callback now owns the allocation and skb.
                return err;
            }
            break 'out_free if err == neg(ENOSPC) { NET_XMIT_DROP as c_int } else { err };
        }
        copy_nonoverlapping(icv, addr_of_mut!((*ah).auth_data).cast(), (*ahp).icv_trunc_len as usize);
        copy_nonoverlapping(base, top_iph.cast(), IPV6HDR_BASELEN as usize);
        ah6_restore_hdrs(top_iph, ext, extlen);
        0
    };
    kfree(base.cast());
    err
}

#[inline]
unsafe fn finish_input(skb: *mut sk_buff, x: *mut xfrm_state, work: *mut u8,
                      ah_hlen: c_int, hdr_len: c_int) {
    let network_header = rust_ah6_network_header_slot(skb);
    *network_header = (*network_header).wrapping_add(ah_hlen as u16);
    copy_nonoverlapping(work, rust_ah6_network_header(skb), hdr_len as usize);
    rust_ah6_pull(skb, ah_hlen.wrapping_add(hdr_len) as u32);
    if (*x).props.mode as u32 == XFRM_MODE_TUNNEL {
        rust_ah6_reset_transport_header(skb);
    } else {
        rust_ah6_set_transport_header(skb, hdr_len.wrapping_neg());
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_input_done(data: *mut c_void, mut err: c_int) {
    let skb = data.cast::<sk_buff>();
    let x = rust_ah6_input_state(skb);
    let ahp = (*x).data.cast::<ah_data>();
    let ah = rust_ah6_auth_hdr(skb);
    let hdr_len = rust_ah6_network_header_len(skb) as c_int;
    let ah_hlen = ipv6_authlen(ah);
    if err == 0 {
        let seqhi_len = if has_esn(x) { size_of::<u32>() as u32 } else { 0 };
        let work = (*ah_skb_cb(skb)).tmp.cast::<u8>();
        let auth_data = ah_tmp_auth(work, hdr_len as u32);
        let seqhi = auth_data.add((*ahp).icv_trunc_len as usize);
        let icv = ah_tmp_icv(seqhi, seqhi_len);
        err = if rust_ah6_memneq(icv.cast(), auth_data.cast(), (*ahp).icv_trunc_len as usize) != 0 {
            neg(EBADMSG)
        } else { 0 };
        if err == 0 {
            err = (*ah).nexthdr as c_int;
            finish_input(skb, x, work, ah_hlen, hdr_len);
        }
    }
    kfree((*ah_skb_cb(skb)).tmp);
    xfrm_input_resume(skb, err);
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    // C's early validation paths deliberately return -ENOMEM.
    if !rust_ah6_may_pull(skb, size_of::<ip_auth_hdr>() as u32) { return neg(ENOMEM); }
    if rust_ah6_unclone(skb) != 0 { return neg(ENOMEM); }
    rust_ah6_set_checksum_none(skb);
    // The synchronous path has u16 locals in C; the callback has int locals.
    let hdr_len = rust_ah6_network_header_len(skb) as u16;
    let ah = (*skb).data.cast::<ip_auth_hdr>();
    let ahp = (*x).data.cast::<ah_data>();
    let ahash = (*ahp).ahash;
    let nexthdr = (*ah).nexthdr as c_int;
    let ah_hlen = ipv6_authlen(ah) as u16;
    if ah_hlen as usize != align8(size_of::<ip_auth_hdr>().wrapping_add((*ahp).icv_full_len as usize))
        && ah_hlen as usize != align8(size_of::<ip_auth_hdr>().wrapping_add((*ahp).icv_trunc_len as usize)) {
        return neg(ENOMEM);
    }
    if !rust_ah6_may_pull(skb, ah_hlen as u32) { return neg(ENOMEM); }
    let mut trailer = null_mut();
    let nfrags = skb_cow_data(skb, 0, addr_of_mut!(trailer));
    if nfrags < 0 { return nfrags; }
    // cow_data may reallocate: reacquire both pointers after it returns.
    let ah = (*skb).data.cast::<ip_auth_hdr>();
    let ip6h = rust_ah6_ipv6_hdr(skb);
    skb_push(skb, hdr_len as u32);
    let esn = has_esn(x);
    let sglists = if esn { 1 } else { 0 };
    let seqhi_len = if esn { size_of::<u32>() as u32 } else { 0 };
    let work = ah_alloc_tmp(ahash, nfrags.wrapping_add(sglists),
        (hdr_len as u32).wrapping_add((*ahp).icv_trunc_len as u32).wrapping_add(seqhi_len));
    if work.is_null() { return neg(ENOMEM); }
    let auth_data = ah_tmp_auth(work, hdr_len as u32);
    let seqhi = auth_data.add((*ahp).icv_trunc_len as usize).cast::<u32>();
    let icv = ah_tmp_icv(seqhi.cast(), seqhi_len);
    let req = ah_tmp_req(ahash, icv);
    let sg = ah_req_sg(ahash, req);
    let seqhisg = sg.add(nfrags as usize);
    copy_nonoverlapping(ip6h.cast::<u8>(), work, hdr_len as usize);
    copy_nonoverlapping(addr_of!((*ah).auth_data).cast::<u8>(), auth_data, (*ahp).icv_trunc_len as usize);
    write_bytes(addr_of_mut!((*ah).auth_data).cast::<u8>(), 0, (*ahp).icv_trunc_len as usize);

    let err = 'out_free: {
        let err = ipv6_clear_mutable_options(ip6h, hdr_len as c_int, XFRM_POLICY_IN);
        if err != 0 { break 'out_free err; }
        clear_base_mutable(ip6h);
        sg_init_table(sg, nfrags.wrapping_add(sglists) as u32);
        let err = skb_to_sgvec_nomark(skb, sg, 0, (*skb).len as c_int);
        if err < 0 { break 'out_free err; }
        if has_esn(x) {
            // Input ESN high word is already network endian in XFRM_SKB_CB.
            *seqhi = (*ah_skb_cb(skb)).xfrm.seq.input.hi;
            rust_ah6_sg_set_buf(seqhisg, seqhi.cast(), seqhi_len);
        }
        rust_ah6_request_set_crypt(req, sg, icv, (*skb).len.wrapping_add(seqhi_len));
        rust_ah6_request_set_callback(req, 0, Some(rust_ah6_input_done), skb.cast());
        (*ah_skb_cb(skb)).tmp = work.cast();
        let err = crypto_ahash_digest(req);
        if err != 0 {
            if err == neg(EINPROGRESS) { return err; }
            break 'out_free err;
        }
        if rust_ah6_memneq(icv.cast(), auth_data.cast(), (*ahp).icv_trunc_len as usize) != 0 {
            break 'out_free neg(EBADMSG);
        }
        finish_input(skb, x, work, ah_hlen as c_int, hdr_len as c_int);
        nexthdr
    };
    kfree(work.cast());
    err
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_err(skb: *mut sk_buff, _opt: *mut inet6_skb_parm,
        type_: u8, _code: u8, offset: c_int, info: u32) -> c_int {
    let dev = rust_ah6_skb_dev(skb);
    let net = rust_ah6_dev_net(dev);
    let iph = (*skb).data.cast::<ipv6hdr>();
    let ah = (*skb).data.offset(offset as isize).cast::<ip_auth_hdr>();
    if type_ as u32 != ICMPV6_PKT_TOOBIG && type_ as u32 != NDISC_REDIRECT { return 0; }
    let x = xfrm_state_lookup(net, rust_ah6_skb_mark(skb), rust_ah6_daddr(iph).cast(),
                              (*ah).spi, IPPROTO_AH as u8, AF_INET6 as u16);
    if x.is_null() { return 0; }
    if type_ as u32 == NDISC_REDIRECT {
        ip6_redirect(skb, net, (*dev).ifindex, 0, rust_ah6_sock_net_uid(net, null_mut()));
    } else {
        ip6_update_pmtu(skb, net, info, 0, 0, rust_ah6_sock_net_uid(net, null_mut()));
    }
    rust_ah6_state_put(x);
    0
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int {
    if (*x).aalg.is_null() { rust_ah6_extack_auth(extack); return neg(EINVAL); }
    if !(*x).encap.is_null() { rust_ah6_extack_encap(extack); return neg(EINVAL); }
    let ahp = rust_ah6_alloc_data();
    if ahp.is_null() { return neg(ENOMEM); }
    let result = 'error: {
        let aalg = (*x).aalg;
        let ahash = crypto_alloc_ahash(addr_of!((*aalg).alg_name).cast(), 0, 0);
        if rust_ah6_is_err(ahash.cast()) {
            rust_ah6_extack_crypto(extack);
            break 'error neg(EINVAL);
        }
        (*ahp).ahash = ahash;
        if crypto_ahash_setkey(ahash, addr_of!((*aalg).alg_key).cast(),
                (*aalg).alg_key_len.wrapping_add(7) / 8) != 0 {
            rust_ah6_extack_crypto(extack);
            break 'error neg(EINVAL);
        }
        let desc = xfrm_aalg_get_byname(addr_of!((*aalg).alg_name).cast(), 0);
        rust_ah6_bug_on(desc.is_null());
        let full_len = (*desc).uinfo.auth.icv_fullbits / 8;
        if full_len as u32 != rust_ah6_digestsize(ahash) {
            rust_ah6_extack_crypto(extack);
            break 'error neg(EINVAL);
        }
        (*ahp).icv_full_len = full_len as c_int;
        (*ahp).icv_trunc_len = ((*aalg).alg_trunc_len / 8) as c_int;
        (*x).props.header_len = align8(size_of::<ip_auth_hdr>().wrapping_add((*ahp).icv_trunc_len as usize)) as c_int;
        match (*x).props.mode as u32 {
            XFRM_MODE_BEET | XFRM_MODE_TRANSPORT => {}
            XFRM_MODE_TUNNEL => {
                (*x).props.header_len = (*x).props.header_len.wrapping_add(size_of::<ipv6hdr>() as c_int);
            }
            _ => { rust_ah6_extack_mode(extack); break 'error neg(EINVAL); }
        }
        (*x).data = ahp.cast();
        return 0;
    };
    // Zero allocation means ahash remains NULL when crypto_alloc_ahash fails.
    rust_ah6_free_ahash((*ahp).ahash);
    kfree(ahp.cast());
    result
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_destroy(x: *mut xfrm_state) {
    let ahp = (*x).data.cast::<ah_data>();
    if ahp.is_null() { return; }
    rust_ah6_free_ahash((*ahp).ahash);
    kfree(ahp.cast());
}

#[no_mangle]
pub unsafe extern "C" fn rust_ah6_rcv_cb(_skb: *mut sk_buff, _err: c_int) -> c_int {
    // The C receive callback is deliberately unconditional success.
    0
}

// SAFETY: This generated plain-data descriptor has no interior mutability or
// Rust-owned pointees. Its fields are scalar values, C function pointers and an
// opaque module pointer. Sharing the descriptor never dereferences that owner;
// XFRM only reads the descriptor through const pointers. The loader supplies
// the owner's lifetime, and the immutable static supplies stable table identity.
unsafe impl Sync for xfrm_type {}

#[no_mangle]
pub static rust_ah6_type: xfrm_type = xfrm_type {
    #[cfg(MODULE)]
    owner: addr_of_mut!(__this_module),
    #[cfg(not(MODULE))]
    owner: null_mut(),
    proto: IPPROTO_AH as u8,
    flags: XFRM_TYPE_REPLAY_PROT as u8,
    init_state: Some(rust_ah6_init_state),
    destructor: Some(rust_ah6_destroy),
    input: Some(rust_ah6_input),
    output: Some(rust_ah6_output),
    reject: None,
};

#[no_mangle]
pub static mut rust_ah6_protocol: xfrm6_protocol = xfrm6_protocol {
    handler: Some(xfrm6_rcv),
    input_handler: Some(xfrm_input),
    cb_handler: Some(rust_ah6_rcv_cb),
    err_handler: Some(rust_ah6_err),
    next: null_mut(),
    priority: 0,
};

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_ah6_init() -> c_int {
    if xfrm_register_type(addr_of!(rust_ah6_type), AF_INET6 as u16) < 0 {
        rust_ah6_log_add_type();
        return neg(EAGAIN);
    }
    if xfrm6_protocol_register(addr_of_mut!(rust_ah6_protocol), IPPROTO_AH as u8) < 0 {
        rust_ah6_log_add_protocol();
        xfrm_unregister_type(addr_of!(rust_ah6_type), AF_INET6 as u16);
        return neg(EAGAIN);
    }
    0
}

#[no_mangle]
#[link_section = ".exit.text"]
pub unsafe extern "C" fn rust_ah6_fini() {
    if xfrm6_protocol_deregister(addr_of_mut!(rust_ah6_protocol), IPPROTO_AH as u8) < 0 {
        rust_ah6_log_remove_protocol();
    }
    xfrm_unregister_type(addr_of!(rust_ah6_type), AF_INET6 as u16);
}

// ah6.c has no EXPORT_SYMBOL: no ffi_export entries are added by translation.
