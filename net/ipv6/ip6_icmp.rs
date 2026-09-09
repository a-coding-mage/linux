// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the corresponding Linux networking headers are
// intentionally not reproduced here.

// #if IS_ENABLED(CONFIG_IPV6) && IS_ENABLED(CONFIG_NF_NAT)

// #include <net/netfilter/nf_conntrack.h>

pub unsafe fn icmpv6_ndo_send(
    mut skb_in: *mut sk_buff,
    type_: u8,
    code: u8,
    info: u32,
) {
    let mut parm: inet6_skb_parm = core::mem::zeroed();
    let mut cloned_skb: *mut sk_buff = core::ptr::null_mut();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let mut dir: ip_conntrack_dir;
    let mut orig_ip: in6_addr = core::mem::zeroed();
    let ct: *mut nf_conn;

    ct = nf_ct_get(skb_in, &mut ctinfo);
    if ct.is_null() || (core::ptr::read_volatile(&(*ct).status) & IPS_NAT_MASK) == 0 {
        icmp6_send(skb_in, type_, code, info, core::ptr::null_mut(), &mut parm);
        return;
    }

    if skb_shared(skb_in) {
        cloned_skb = skb_clone(skb_in, GFP_ATOMIC);
        skb_in = cloned_skb;
    }

    if skb_in.is_null()
        || skb_network_header(skb_in) < (*skb_in).head
        || skb_network_header(skb_in).add(core::mem::size_of::<ipv6hdr>())
            > skb_tail_pointer(skb_in)
        || skb_ensure_writable(
            skb_in,
            skb_network_offset(skb_in) + core::mem::size_of::<ipv6hdr>(),
        ) != 0
    {
        consume_skb(cloned_skb);
        return;
    }

    orig_ip = (*ipv6_hdr(skb_in)).saddr;
    dir = CTINFO2DIR(ctinfo);
    (*ipv6_hdr(skb_in)).saddr = (*ct).tuplehash[dir as usize].tuple.src.u3.in6;
    icmp6_send(skb_in, type_, code, info, core::ptr::null_mut(), &mut parm);
    (*ipv6_hdr(skb_in)).saddr = orig_ip;
    consume_skb(cloned_skb);
}

// EXPORT_SYMBOL(icmpv6_ndo_send);
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
