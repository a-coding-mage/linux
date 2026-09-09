// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xfrm4_output.c - Common IPsec encapsulation code for IPv4.
 * Copyright (c) 2004 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn __xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    // CONFIG_NETFILTER conditional compilation from the original source.
    #[cfg(CONFIG_NETFILTER)]
    {
        let x: *mut xfrm_state = (*skb_dst(skb)).xfrm;

        if x.is_null() {
            (*IPCB(skb)).flags |= IPSKB_REROUTED;
            return dst_output(net, sk, skb);
        }
    }

    xfrm_output(sk, skb)
}

pub unsafe fn xfrm4_output(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    NF_HOOK_COND(
        NFPROTO_IPV4,
        NF_INET_POST_ROUTING,
        net,
        sk,
        skb,
        (*skb).dev,
        skb_dst_dev(skb),
        __xfrm4_output,
        !((*IPCB(skb)).flags & IPSKB_REROUTED != 0),
    )
}

pub unsafe fn xfrm4_local_error(skb: *mut sk_buff, mtu: u32) {
    let hdr: *mut iphdr;

    hdr = if (*skb).encapsulation {
        inner_ip_hdr(skb)
    } else {
        ip_hdr(skb)
    };
    ip_local_error(
        (*skb).sk,
        EMSGSIZE,
        (*hdr).daddr,
        (*inet_sk((*skb).sk)).inet_dport,
        mtu,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
