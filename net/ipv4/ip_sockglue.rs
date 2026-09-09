// SPDX-License-Identifier: GPL-2.0
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		The IP to API glue.
 *
 * Authors:	see ip.c
 *
 * Fixes:
 *		Many		:	Split from ip.c , see ip.c for history.
 *		Martin Mares	:	TOS setting fixed.
 *		Alan Cox	:	Fixed a couple of oopses in Martin's
 *					TOS tweaks.
 *		Mike McLagan	:	Routing by source
 */

/* Kernel dependencies are supplied by other translated units. */

/*
 *	SOL_IP control messages.
 */

unsafe fn ip_cmsg_recv_pktinfo(msg: *mut msghdr, skb: *mut sk_buff) {
	let mut info: in_pktinfo = *PKTINFO_SKB_CB(skb);

	info.ipi_addr.s_addr = (*ip_hdr(skb)).daddr;

	put_cmsg(msg, SOL_IP, IP_PKTINFO, core::mem::size_of::<in_pktinfo>(), &info);
}

unsafe fn ip_cmsg_recv_ttl(msg: *mut msghdr, skb: *mut sk_buff) {
	let ttl: i32 = (*ip_hdr(skb)).ttl as i32;
	put_cmsg(msg, SOL_IP, IP_TTL, core::mem::size_of::<i32>(), &ttl);
}

unsafe fn ip_cmsg_recv_tos(msg: *mut msghdr, skb: *mut sk_buff) {
	put_cmsg(msg, SOL_IP, IP_TOS, 1, &(*ip_hdr(skb)).tos);
}

unsafe fn ip_cmsg_recv_opts(msg: *mut msghdr, skb: *mut sk_buff) {
	if (*IPCB(skb)).opt.optlen == 0 {
		return;
	}

	put_cmsg(
		msg,
		SOL_IP,
		IP_RECVOPTS,
		(*IPCB(skb)).opt.optlen,
		ip_hdr(skb).add(1),
	);
}

unsafe fn ip_cmsg_recv_retopts(
	net: *mut net,
	msg: *mut msghdr,
	skb: *mut sk_buff,
) {
	let mut optbuf: [u8; core::mem::size_of::<ip_options>() + 40] =
		[0; core::mem::size_of::<ip_options>() + 40];
	let opt: *mut ip_options = optbuf.as_mut_ptr() as *mut ip_options;

	if (*IPCB(skb)).opt.optlen == 0 {
		return;
	}

	if ip_options_echo(net, opt, skb) != 0 {
		(*msg).msg_flags |= MSG_CTRUNC;
		return;
	}
	ip_options_undo(opt);

	put_cmsg(msg, SOL_IP, IP_RETOPTS, (*opt).optlen, (*opt).__data.as_ptr());
}

unsafe fn ip_cmsg_recv_fragsize(msg: *mut msghdr, skb: *mut sk_buff) {
	let val: i32;

	if (*IPCB(skb)).frag_max_size == 0 {
		return;
	}

	val = (*IPCB(skb)).frag_max_size;
	put_cmsg(msg, SOL_IP, IP_RECVFRAGSIZE, core::mem::size_of::<i32>(), &val);
}

unsafe fn ip_cmsg_recv_checksum(
	msg: *mut msghdr,
	skb: *mut sk_buff,
	tlen: i32,
	offset: i32,
) {
	let mut csum: __wsum = (*skb).csum;

	if (*skb).ip_summed != CHECKSUM_COMPLETE {
		return;
	}

	if offset != 0 {
		let tend_off = skb_transport_offset(skb) + tlen;
		csum = csum_sub(csum, skb_checksum(skb, tend_off, offset, 0));
	}

	put_cmsg(msg, SOL_IP, IP_CHECKSUM, core::mem::size_of::<__wsum>(), &csum);
}

unsafe fn ip_cmsg_recv_security(msg: *mut msghdr, skb: *mut sk_buff) {
	let mut ctx: lsm_context = core::mem::zeroed();
	let mut secid: u32 = 0;
	let mut err: i32;

	err = security_socket_getpeersec_dgram(core::ptr::null_mut(), skb, &mut secid);
	if err != 0 {
		return;
	}

	err = security_secid_to_secctx(secid, &mut ctx);
	if err < 0 {
		return;
	}

	put_cmsg(msg, SOL_IP, SCM_SECURITY, ctx.len, ctx.context);
	security_release_secctx(&mut ctx);
}

unsafe fn ip_cmsg_recv_dstaddr(msg: *mut msghdr, skb: *mut sk_buff) {
	let mut _ports: [__be16; 2] = [0; 2];
	let ports: *mut __be16;
	let mut sin: sockaddr_in = core::mem::zeroed();

	/* All current transport protocols have the port numbers in the
	 * first four bytes of the transport header and this function is
	 * written with this assumption in mind.
	 */
	ports = skb_header_pointer(
		skb,
		skb_transport_offset(skb),
		core::mem::size_of::<[__be16; 2]>(),
		_ports.as_mut_ptr(),
	);
	if ports.is_null() {
		return;
	}

	sin.sin_family = AF_INET;
	sin.sin_addr.s_addr = (*ip_hdr(skb)).daddr;
	sin.sin_port = *ports.add(1);
	core::ptr::write_bytes(sin.sin_zero.as_mut_ptr(), 0, sin.sin_zero.len());

	put_cmsg(msg, SOL_IP, IP_ORIGDSTADDR, core::mem::size_of::<sockaddr_in>(), &sin);
}

pub unsafe fn ip_cmsg_recv_offset(
	msg: *mut msghdr,
	sk: *mut sock,
	skb: *mut sk_buff,
	tlen: i32,
	offset: i32,
) {
	let mut flags: c_ulong = inet_cmsg_flags(inet_sk(sk));

	if flags == 0 {
		return;
	}

	/* Ordered by supposed usage frequency */
	if flags & IP_CMSG_PKTINFO != 0 {
		ip_cmsg_recv_pktinfo(msg, skb);

		flags &= !IP_CMSG_PKTINFO;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_TTL != 0 {
		ip_cmsg_recv_ttl(msg, skb);

		flags &= !IP_CMSG_TTL;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_TOS != 0 {
		ip_cmsg_recv_tos(msg, skb);

		flags &= !IP_CMSG_TOS;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_RECVOPTS != 0 {
		ip_cmsg_recv_opts(msg, skb);

		flags &= !IP_CMSG_RECVOPTS;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_RETOPTS != 0 {
		ip_cmsg_recv_retopts(sock_net(sk), msg, skb);

		flags &= !IP_CMSG_RETOPTS;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_PASSSEC != 0 {
		ip_cmsg_recv_security(msg, skb);

		flags &= !IP_CMSG_PASSSEC;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_ORIGDSTADDR != 0 {
		ip_cmsg_recv_dstaddr(msg, skb);

		flags &= !IP_CMSG_ORIGDSTADDR;
		if flags == 0 {
			return;
		}
	}

	if flags & IP_CMSG_CHECKSUM != 0 {
		ip_cmsg_recv_checksum(msg, skb, tlen, offset);
	}

	if flags & IP_CMSG_RECVFRAGSIZE != 0 {
		ip_cmsg_recv_fragsize(msg, skb);
	}
}

// EXPORT_SYMBOL(ip_cmsg_recv_offset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
