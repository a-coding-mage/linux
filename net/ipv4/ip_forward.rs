// SPDX-License-Identifier: GPL-2.0
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		The IP forwarding functionality.
 *
 * Authors:	see ip.c
 *
 * Fixes:
 *		Many		:	Split from ip.c , see ip_input.c for
 *					history.
 *		Dave Gregorich	:	NULL ip_rt_put fix for multicast
 *					routing.
 *		Jos Vos		:	Add call_out_firewall before sending,
 *					use output device for accounting.
 *		Jos Vos		:	Call forward firewall after routing
 *					(always use output device).
 *		Mike McLagan	:	Routing by source
 */

unsafe fn ip_exceeds_mtu(skb: *const sk_buff, mtu: u32) -> bool {
	if (*skb).len <= mtu {
		return false;
	}

	if unlikely(((*ip_hdr(skb)).frag_off & htons(IP_DF)) == 0) {
		return false;
	}

	/* original fragment exceeds mtu and DF is set */
	if unlikely((*IPCB(skb)).frag_max_size > mtu) {
		return true;
	}

	if (*skb).ignore_df {
		return false;
	}

	if skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu) {
		return false;
	}

	true
}

unsafe fn ip_forward_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32 {
	let opt: *mut ip_options = &mut (*IPCB(skb)).opt;

	// #ifdef CONFIG_NET_SWITCHDEV
	if (*skb).offload_l3_fwd_mark {
		consume_skb(skb);
		return 0;
	}
	// #endif

	if unlikely((*opt).optlen != 0) {
		ip_forward_options(skb);
	}

	skb_clear_tstamp(skb);
	dst_output(net, sk, skb)
}

unsafe fn ip_forward(skb: *mut sk_buff) -> i32 {
	let mut mtu: u32;
	let mut iph: *mut iphdr;
	let mut rt: *mut rtable;
	let opt: *mut ip_options = &mut (*IPCB(skb)).opt;
	let mut net: *mut net;
	let mut reason: u32 = 0;

	/* that should never happen */
	if (*skb).pkt_type != PACKET_HOST {
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	if unlikely(!(*skb).sk.is_null()) {
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	if skb_warn_if_lro(skb) {
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	if !xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_FWD, skb) {
		reason = XFRM_POLICY;
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	if (*IPCB(skb)).opt.router_alert && ip_call_ra_chain(skb) {
		return NET_RX_SUCCESS;
	}

	skb_forward_csum(skb);
	net = dev_net((*skb).dev);

	/*
	 *	According to the RFC, we must first decrease the TTL field. If
	 *	that reaches zero, we must reply an ICMP control message telling
	 *	that the packet's lifetime expired.
	 */
	if (*ip_hdr(skb)).ttl <= 1 {
		__IP_INC_STATS(net, IPSTATS_MIB_INHDRERRORS);
		icmp_send(skb, ICMP_TIME_EXCEEDED, ICMP_EXC_TTL, 0);
		reason = IP_INHDR;
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	if !xfrm4_route_forward(skb) {
		reason = XFRM_POLICY;
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	rt = skb_rtable(skb);

	if (*opt).is_strictroute && (*rt).rt_uses_gateway {
		icmp_send(skb, ICMP_DEST_UNREACH, ICMP_SR_FAILED, 0);
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	__IP_INC_STATS(net, IPSTATS_MIB_OUTFORWDATAGRAMS);

	(*IPCB(skb)).flags |= IPSKB_FORWARDED;
	mtu = ip_dst_mtu_maybe_forward(&mut (*rt).dst, true);
	if ip_exceeds_mtu(skb, mtu) {
		IP_INC_STATS(net, IPSTATS_MIB_FRAGFAILS);
		icmp_send(skb, ICMP_DEST_UNREACH, ICMP_FRAG_NEEDED, htonl(mtu));
		reason = PKT_TOO_BIG;
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}

	/* We are about to mangle packet. Copy it! */
	if skb_cow(skb, LL_RESERVED_SPACE((*rt).dst.dev) + (*rt).dst.header_len) != 0 {
		goto_drop(skb, reason);
		return NET_RX_DROP;
	}
	iph = ip_hdr(skb);

	/* Decrease ttl after skb cow done */
	ip_decrease_ttl(iph);

	/*
	 *	We now generate an ICMP HOST REDIRECT giving the route
	 *	we calculated.
	 */
	if (*IPCB(skb)).flags & IPSKB_DOREDIRECT != 0 && !(*opt).srr && skb_sec_path(skb).is_null() {
		ip_rt_send_redirect(skb);
	}

	if READ_ONCE((*net).ipv4.sysctl_ip_fwd_update_priority) {
		(*skb).priority = rt_tos2priority((*iph).tos);
	}

	NF_HOOK(NFPROTO_IPV4, NF_INET_FORWARD, net, core::ptr::null_mut(), skb,
		(*skb).dev, (*rt).dst.dev, ip_forward_finish)
}

unsafe fn goto_drop(skb: *mut sk_buff, reason: u32) {
		kfree_skb_reason(skb, reason);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
