/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for the ARP (RFC 826) protocol.
 *
 * Version:	@(#)if_arp.h	1.0.1	04/16/93
 *
 * Authors:	Original taken from Berkeley UNIX 4.3, (c) UCB 1986-1988
 *		Portions taken from the KA9Q/NOS (v2.00m PA0GRI) source.
 *		Ross Biro
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *		Florian La Roche,
 *		Jonathan Layes <layes@loran.com>
 *		Arnaldo Carvalho de Melo <acme@conectiva.com.br> ARPHRD_HWX25
 */

// Dependencies supplied by the surrounding Linux translation.

#[inline]
pub unsafe fn arp_hdr(skb: *const sk_buff) -> *mut arphdr {
	 skb_network_header(skb) as *mut arphdr
}

#[inline]
pub unsafe fn arp_hdr_len(dev: *const net_device) -> ::core::primitive::u32 {
	match (*dev).r#type {
		// Preserved conditional: CONFIG_FIREWIRE_NET.
		// ARP header, device address and 2 IP addresses.
		// case ARPHRD_IEEE1394:
		//     return sizeof(struct arphdr) + dev->addr_len + sizeof(u32) * 2;
		_ => {
			// ARP header, plus 2 device addresses, plus 2 IP addresses.
			(core::mem::size_of::<arphdr>()
				+ ((*dev).addr_len as usize + core::mem::size_of::<u32>()) * 2) as u32
		}
	}
}

#[inline]
pub unsafe fn dev_is_mac_header_xmit(dev: *const net_device) -> bool {
	match (*dev).r#type {
		ARPHRD_TUNNEL
		| ARPHRD_TUNNEL6
		| ARPHRD_SIT
		| ARPHRD_IPGRE
		| ARPHRD_IP6GRE
		| ARPHRD_VOID
		| ARPHRD_NONE
		| ARPHRD_RAWIP
		| ARPHRD_PIMREG
		// PPP adds its l2 header automatically in ppp_start_xmit().
		// This makes it look like an l3 device to __bpf_redirect() and tcf_mirred_init().
		| ARPHRD_PPP => false,
		_ => true,
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
