// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xfrm6_output.c - Common IPsec encapsulation code for IPv6.
 * Copyright (C) 2002 USAGI/WIDE Project
 * Copyright (c) 2004 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn xfrm6_local_rxpmtu(skb: *mut sk_buff, mtu: u32) {
    let mut fl6: flowi6 = core::mem::zeroed();
    let sk: *mut sock = (*skb).sk;

    fl6.flowi6_oif = (*sk).sk_bound_dev_if;
    fl6.daddr = (*ipv6_hdr(skb)).daddr;

    ipv6_local_rxpmtu(sk, &mut fl6, mtu);
}

pub unsafe fn xfrm6_local_error(skb: *mut sk_buff, mtu: u32) {
    let mut fl6: flowi6 = core::mem::zeroed();
    let hdr: *const ipv6hdr;
    let sk: *mut sock = (*skb).sk;

    hdr = if (*skb).encapsulation {
        inner_ipv6_hdr(skb)
    } else {
        ipv6_hdr(skb)
    };
    fl6.fl6_dport = (*inet_sk(sk)).inet_dport;
    fl6.daddr = (*hdr).daddr;

    ipv6_local_error(sk, EMSGSIZE, &mut fl6, mtu);
}

unsafe fn __xfrm6_output_finish(
    _net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    xfrm_output(sk, skb)
}

unsafe fn xfrm6_noneed_fragment(skb: *mut sk_buff) -> c_int {
    let fh: *mut frag_hdr;
    let prevhdr: u8 = (*ipv6_hdr(skb)).nexthdr;

    if prevhdr != NEXTHDR_FRAGMENT {
        return 0;
    }
    fh = ( (*skb).data.add(core::mem::size_of::<ipv6hdr>()) ) as *mut frag_hdr;
    if (*fh).nexthdr == NEXTHDR_ESP || (*fh).nexthdr == NEXTHDR_AUTH {
        return 1;
    }
    0
}

unsafe fn __xfrm6_output(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    let dst: *mut dst_entry = skb_dst(skb);
    let x: *mut xfrm_state = (*dst).xfrm;
    let mtu: u32;
    let toobig: bool;

    // CONFIG_NETFILTER controls this block at build time in the C source.
    #[cfg(CONFIG_NETFILTER)]
    if x.is_null() {
        (*IP6CB(skb)).flags |= IP6SKB_REROUTED;
        return dst_output(net, sk, skb);
    }

    if (*x).props.mode != XFRM_MODE_TUNNEL {
        return xfrm_output(sk, skb);
    }

    if (*skb).protocol == htons(ETH_P_IPV6) {
        mtu = ip6_skb_dst_mtu(skb);
    } else {
        mtu = dst_mtu(skb_dst(skb));
    }

    toobig = (*skb).len > mtu && !skb_is_gso(skb);

    if toobig && xfrm6_local_dontfrag(sk) {
        xfrm6_local_rxpmtu(skb, mtu);
        kfree_skb(skb);
        return -EMSGSIZE;
    } else if toobig && xfrm6_noneed_fragment(skb) != 0 {
        (*skb).ignore_df = 1;
        return xfrm_output(sk, skb);
    } else if !(*skb).ignore_df && toobig && !sk.is_null() {
        xfrm_local_error(skb, mtu);
        kfree_skb(skb);
        return -EMSGSIZE;
    }

    if toobig {
        return ip6_fragment(net, sk, skb, Some(__xfrm6_output_finish));
    }

    xfrm_output(sk, skb)
}

pub unsafe fn xfrm6_output(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    NF_HOOK_COND(
        NFPROTO_IPV6,
        NF_INET_POST_ROUTING,
        net,
        sk,
        skb,
        (*skb).dev,
        skb_dst_dev(skb),
        __xfrm6_output,
        ((*IP6CB(skb)).flags & IP6SKB_REROUTED) == 0,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
