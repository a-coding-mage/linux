// SPDX-License-Identifier: GPL-2.0-or-later
/* Utility routines
 *
 * Copyright (C) 2015 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel/RXRPC translation. */

/*
 * Fill out a peer address from a socket buffer containing a packet.
 */
pub unsafe fn rxrpc_extract_addr_from_skb(
    srx: *mut sockaddr_rxrpc,
    skb: *mut sk_buff,
) -> i32 {
    core::ptr::write_bytes(srx as *mut u8, 0, core::mem::size_of::<sockaddr_rxrpc>());

    match ntohs((*skb).protocol) {
        ETH_P_IP => {
            (*srx).transport_type = SOCK_DGRAM;
            (*srx).transport_len = core::mem::size_of_val(&(*srx).transport.sin);
            (*srx).transport.sin.sin_family = AF_INET;
            (*srx).transport.sin.sin_port = (*udp_hdr(skb)).source;
            (*srx).transport.sin.sin_addr.s_addr = (*ip_hdr(skb)).saddr;
            0
        }

        // CONFIG_AF_RXRPC_IPV6
        #[cfg(CONFIG_AF_RXRPC_IPV6)]
        ETH_P_IPV6 => {
            (*srx).transport_type = SOCK_DGRAM;
            (*srx).transport_len = core::mem::size_of_val(&(*srx).transport.sin6);
            (*srx).transport.sin6.sin6_family = AF_INET6;
            (*srx).transport.sin6.sin6_port = (*udp_hdr(skb)).source;
            (*srx).transport.sin6.sin6_addr = (*ipv6_hdr(skb)).saddr;
            0
        }

        _ => {
            pr_warn_ratelimited(
                "AF_RXRPC: Unknown eth protocol %u\n",
                ntohs((*skb).protocol),
            );
            -EAFNOSUPPORT
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
