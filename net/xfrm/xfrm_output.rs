// SPDX-License-Identifier: GPL-2.0-or-later
/* Common IPsec encapsulation code, translated from xfrm_output.c. */

// Kernel types, constants, macros, and functions referenced below are supplied
// by the surrounding translated networking sources.

unsafe fn xfrm_skb_check_space(skb: *mut sk_buff) -> c_int {
    let dst = skb_dst(skb);
    let nhead = (*dst).header_len + LL_RESERVED_SPACE((*dst).dev) - skb_headroom(skb);
    let ntail = (*(*dst).dev).needed_tailroom - skb_tailroom(skb);
    if nhead <= 0 { if ntail <= 0 { return 0; } return pskb_expand_head(skb, 0, ntail, GFP_ATOMIC); }
    pskb_expand_head(skb, nhead, if ntail < 0 { 0 } else { ntail }, GFP_ATOMIC)
}

unsafe fn skb_dst_pop(skb: *mut sk_buff) -> *mut dst_entry {
    let child = dst_clone(xfrm_dst_child(skb_dst(skb))); skb_dst_drop(skb); child
}

unsafe fn xfrm4_transport_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let iph = ip_hdr(skb); let ihl = (*iph).ihl as c_int * 4;
    if !(*skb).inner_protocol { skb_set_inner_transport_header(skb, skb_transport_offset(skb)); }
    skb_set_network_header(skb, -(*x).props.header_len);
    (*skb).mac_header = (*skb).network_header + core::mem::offset_of!(iphdr, protocol) as i32;
    (*skb).transport_header = (*skb).network_header + ihl;
    __skb_pull(skb, ihl); memmove(skb_network_header(skb), iph, ihl as usize); 0
}

unsafe fn xfrm6_transport_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    #[cfg(feature="CONFIG_IPV6")]
    { let iph = ipv6_hdr(skb); if !(*skb).inner_protocol { skb_set_inner_transport_header(skb, skb_transport_offset(skb)); }
      let mut prev: *mut u8 = core::ptr::null_mut(); let hdr_len = xfrm6_hdr_offset(x, skb, &mut prev); if hdr_len < 0 { return hdr_len; }
      skb_set_mac_header(skb, prev.offset(-(*x).props.header_len as isize).offset_from((*skb).data) as i32);
      skb_set_network_header(skb, -(*x).props.header_len); (*skb).transport_header = (*skb).network_header + hdr_len;
      __skb_pull(skb, hdr_len); memmove(ipv6_hdr(skb), iph, hdr_len as usize); 0 }
    #[cfg(not(feature="CONFIG_IPV6"))] { WARN_ON_ONCE(1); -EAFNOSUPPORT }
}

unsafe fn xfrm4_beet_encap_add(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let optlen = XFRM_MODE_SKB_CB(skb).optlen; let hdrlen = if optlen != 0 { IPV4_BEET_PHMAXLEN - (optlen & 4) } else { 0 };
    skb_set_network_header(skb, -(*x).props.header_len - hdrlen + (XFRM_MODE_SKB_CB(skb).ihl - core::mem::size_of::<iphdr>() as i32));
    if (*x).sel.family != AF_INET6 { (*skb).network_header += IPV4_BEET_PHMAXLEN; }
    (*skb).mac_header = (*skb).network_header + core::mem::offset_of!(iphdr, protocol) as i32;
    (*skb).transport_header = (*skb).network_header + core::mem::size_of::<iphdr>() as i32;
    xfrm4_beet_make_header(skb); let ph = __skb_pull(skb, XFRM_MODE_SKB_CB(skb).ihl - hdrlen); let top = ip_hdr(skb);
    if optlen != 0 { if WARN_ON(optlen < 0) { return -EINVAL; } (*ph).padlen = 4 - (optlen & 4); (*ph).hdrlen = optlen / 8; (*ph).nexthdr = (*top).protocol; if (*ph).padlen != 0 { memset(ph.add(1), IPOPT_NOP, (*ph).padlen as usize); } (*top).protocol = IPPROTO_BEETPH; (*top).ihl = (core::mem::size_of::<iphdr>() / 4) as u8; }
    (*top).saddr = (*x).props.saddr.a4; (*top).daddr = (*x).id.daddr.a4; 0
}

unsafe fn xfrm_outer_mode_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    match (*x).props.mode { XFRM_MODE_BEET | XFRM_MODE_TUNNEL => if (*x).props.family == AF_INET { xfrm4_prepare_output(x, skb) } else if (*x).props.family == AF_INET6 { xfrm6_prepare_output(x, skb) } else { -EOPNOTSUPP },
      XFRM_MODE_TRANSPORT => if (*x).props.family == AF_INET { xfrm4_transport_output(x, skb) } else { xfrm6_transport_output(x, skb) },
      XFRM_MODE_ROUTEOPTIMIZATION => xfrm6_ro_output(x, skb), _ => { if !(*x).mode_cbs.is_null() && !(*(*x).mode_cbs).prepare_output.is_none() { (*(*x).mode_cbs).prepare_output.unwrap()(x, skb) } else { WARN_ON_ONCE(1); -EOPNOTSUPP } } }
}

unsafe fn xfrm_output_one(skb: *mut sk_buff, mut err: c_int) -> c_int {
    let mut dst = skb_dst(skb); let mut x = (*dst).xfrm; let net = xs_net(x);
    if err > 0 && (*x).xso.r#type != XFRM_DEV_OFFLOAD_PACKET { loop { err=xfrm_skb_check_space(skb); if err != 0 { XFRM_INC_STATS(net, LINUX_MIB_XFRMOUTERROR); break; } (*skb).mark=xfrm_smark_get((*skb).mark,x); err=xfrm_outer_mode_output(x,skb); if err!=0 { break; } spin_lock_bh(&mut (*x).lock); if (*x).km.state != XFRM_STATE_VALID { err=-EINVAL; } else { err=xfrm_state_check_expire(x); } if err==0 { err=xfrm_replay_overflow(x,skb); } if err!=0 { spin_unlock_bh(&mut (*x).lock); break; } (*x).curlft.bytes += (*skb).len as u64; (*x).curlft.packets += 1; spin_unlock_bh(&mut (*x).lock); skb_dst_force(skb); if skb_dst(skb).is_null() { return -EHOSTUNREACH; } err=(*x).r#type.output(x,skb); if err == -EINPROGRESS { return err; } if err!=0 { break; } dst=skb_dst_pop(skb); if dst.is_null() { return -EHOSTUNREACH; } skb_dst_set(skb,dst); x=(*dst).xfrm; if x.is_null() || (*x).outer_mode.flags & XFRM_MODE_FLAG_TUNNEL != 0 { return 0; } } }
    kfree_skb(skb); err
}

pub unsafe fn xfrm_output_resume(sk: *mut sock, skb: *mut sk_buff, mut err: c_int) -> c_int { let net=xs_net((*skb_dst(skb)).xfrm); while xfrm_output_one(skb,err)==0 { nf_reset_ct(skb); err=(*skb_dst(skb)).ops.local_out(net,sk,skb); if err!=1 { return err; } if (*skb_dst(skb)).xfrm.is_null() { return dst_output(net,sk,skb); } err=nf_hook((*skb_dst(skb)).ops.family,NF_INET_POST_ROUTING,net,sk,skb,core::ptr::null_mut(),(*skb_dst(skb)).dev,xfrm_output2); } if err == -EINPROGRESS { 0 } else { err } }

unsafe fn xfrm_output2(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int { xfrm_output_resume(sk,skb,1) }

unsafe fn xfrm_inner_extract_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { match (*skb).protocol { v if v==htons(ETH_P_IP) => xfrm4_extract_output(x,skb), v if v==htons(ETH_P_IPV6) => xfrm6_extract_output(x,skb), _ => -EAFNOSUPPORT } }

// Remaining exported helpers retain the kernel ABI and are supplied by the
// translated companion units; declarations are intentionally external.
extern "C" { fn xfrm4_prepare_output(x:*mut xfrm_state,skb:*mut sk_buff)->c_int; fn xfrm6_prepare_output(x:*mut xfrm_state,skb:*mut sk_buff)->c_int; fn xfrm6_ro_output(x:*mut xfrm_state,skb:*mut sk_buff)->c_int; fn xfrm4_extract_output(x:*mut xfrm_state,skb:*mut sk_buff)->c_int; fn xfrm6_extract_output(x:*mut xfrm_state,skb:*mut sk_buff)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
