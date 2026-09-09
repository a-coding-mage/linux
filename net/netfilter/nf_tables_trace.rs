// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2015 Red Hat GmbH
 * Author: Florian Westphal <fw@strlen.de>
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

pub const NFT_TRACETYPE_LL_HSIZE: usize = 20;
pub const NFT_TRACETYPE_NETWORK_HSIZE: usize = 40;
pub const NFT_TRACETYPE_TRANSPORT_HSIZE: usize = 20;

// DEFINE_STATIC_KEY_FALSE(nft_trace_enabled);
// EXPORT_SYMBOL_GPL(nft_trace_enabled);

unsafe fn trace_fill_header(
    nlskb: *mut sk_buff,
    r#type: u16,
    skb: *const sk_buff,
    off: i32,
    len: u32,
) -> i32 {
    let nla: *mut nlattr;

    if len == 0 {
        return 0;
    }

    nla = nla_reserve(nlskb, r#type, len);
    if nla.is_null() || skb_copy_bits(skb, off, nla_data(nla), len) != 0 {
        return -1;
    }

    0
}

unsafe fn nf_trace_fill_ll_header(nlskb: *mut sk_buff, skb: *const sk_buff) -> i32 {
    let mut veth: vlan_ethhdr = core::mem::zeroed();
    let off: i32;

    // BUILD_BUG_ON(sizeof(veth) > NFT_TRACETYPE_LL_HSIZE);

    off = skb_mac_header(skb) - (*skb).data;
    if off != -ETH_HLEN {
        return -1;
    }

    if skb_copy_bits(skb, off, &mut veth as *mut _ as *mut core::ffi::c_void, ETH_HLEN) != 0 {
        return -1;
    }

    veth.h_vlan_proto = htons((*skb).vlan_proto);
    veth.h_vlan_TCI = htons(skb_vlan_tag_get(skb));
    veth.h_vlan_encapsulated_proto = (*skb).protocol;

    nla_put(
        nlskb,
        NFTA_TRACE_LL_HEADER,
        core::mem::size_of::<vlan_ethhdr>() as u32,
        &veth as *const _ as *const core::ffi::c_void,
    )
}

unsafe fn nf_trace_fill_dev_info(
    nlskb: *mut sk_buff,
    indev: *const net_device,
    outdev: *const net_device,
) -> i32 {
    if !indev.is_null() {
        if nla_put_be32(nlskb, NFTA_TRACE_IIF, htonl((*indev).ifindex as u32)) != 0 {
            return -1;
        }
        if nla_put_be16(nlskb, NFTA_TRACE_IIFTYPE, htons((*indev).r#type)) != 0 {
            return -1;
        }
    }

    if !outdev.is_null() {
        if nla_put_be32(nlskb, NFTA_TRACE_OIF, htonl((*outdev).ifindex as u32)) != 0 {
            return -1;
        }
        if nla_put_be16(nlskb, NFTA_TRACE_OIFTYPE, htons((*outdev).r#type)) != 0 {
            return -1;
        }
    }

    0
}

unsafe fn nf_trace_fill_ct_info(nlskb: *mut sk_buff, skb: *const sk_buff) -> i32 {
    let ct_hook: *const nf_ct_hook;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct: *const nf_conn;
    let state: u32;

    ct_hook = rcu_dereference(nf_ct_hook);
    if ct_hook.is_null() {
        return 0;
    }

    ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() {
        if ctinfo != IP_CT_UNTRACKED {
            return 0;
        }
        state = NF_CT_STATE_UNTRACKED_BIT;
    } else {
        state = NF_CT_STATE_BIT(ctinfo);
    }

    if nla_put_be32(nlskb, NFTA_TRACE_CT_STATE, htonl(state)) != 0 {
        return -1;
    }

    if !ct.is_null() {
        let id: u32 = ((*ct_hook).get_id)(&(*ct).ct_general);
        let mut status: u32 = READ_ONCE((*ct).status);
        let dir: u8 = CTINFO2DIR(ctinfo);

        if nla_put_u8(nlskb, NFTA_TRACE_CT_DIRECTION, dir) != 0 {
            return -1;
        }
        if nla_put_be32(nlskb, NFTA_TRACE_CT_ID, id as __be32) != 0 {
            return -1;
        }

        /* Kernel implementation detail, withhold this from userspace for now */
        status &= !IPS_NAT_CLASH;
        if status != 0 && nla_put_be32(nlskb, NFTA_TRACE_CT_STATUS, htonl(status)) != 0 {
            return -1;
        }
    }

    0
}

unsafe fn nf_trace_fill_pkt_info(nlskb: *mut sk_buff, pkt: *const nft_pktinfo) -> i32 {
    let skb = (*pkt).skb;
    let mut off = skb_network_offset(skb);
    let len: u32;
    let nh_end: u32;

    nh_end = if (*pkt).flags & NFT_PKTINFO_L4PROTO != 0 { nft_thoff(pkt) } else { (*skb).len };
    len = core::cmp::min(nh_end - skb_network_offset(skb), NFT_TRACETYPE_NETWORK_HSIZE as u32);
    if trace_fill_header(nlskb, NFTA_TRACE_NETWORK_HEADER, skb, off, len) != 0 {
        return -1;
    }

    if (*pkt).flags & NFT_PKTINFO_L4PROTO != 0 {
        let transport_len = core::cmp::min((*skb).len - nft_thoff(pkt), NFT_TRACETYPE_TRANSPORT_HSIZE as u32);
        if trace_fill_header(nlskb, NFTA_TRACE_TRANSPORT_HEADER, skb, nft_thoff(pkt) as i32, transport_len) != 0 {
            return -1;
        }
    }

    if !skb_mac_header_was_set(skb) {
        return 0;
    }
    if skb_vlan_tag_get(skb) != 0 {
        return nf_trace_fill_ll_header(nlskb, skb);
    }

    off = skb_mac_header(skb) - (*skb).data;
    let ll_len = core::cmp::min((-off) as u32, NFT_TRACETYPE_LL_HSIZE as u32);
    trace_fill_header(nlskb, NFTA_TRACE_LL_HEADER, skb, off, ll_len)
}

unsafe fn nf_trace_fill_rule_info(
    nlskb: *mut sk_buff,
    verdict: *const nft_verdict,
    rule: *const nft_rule_dp,
    info: *const nft_traceinfo,
) -> i32 {
    if rule.is_null() || (*rule).is_last {
        return 0;
    }
    /* a continue verdict with ->type == RETURN means that this is
     * an implicit return (end of chain reached).
     *
     * Since no rule matched, the ->rule pointer is invalid.
     */
    if (*info).r#type == NFT_TRACETYPE_RETURN && (*verdict).code == NFT_CONTINUE {
        return 0;
    }
    nla_put_be64(nlskb, NFTA_TRACE_RULE_HANDLE, cpu_to_be64((*rule).handle), NFTA_TRACE_PAD)
}

unsafe fn nft_trace_have_verdict_chain(verdict: *const nft_verdict, info: *mut nft_traceinfo) -> bool {
    match (*info).r#type {
        NFT_TRACETYPE_RETURN | NFT_TRACETYPE_RULE => {}
        _ => return false,
    }
    match (*verdict).code {
        NFT_JUMP | NFT_GOTO => {}
        _ => return false,
    }
    true
}

unsafe fn nft_trace_get_chain(rule: *const nft_rule_dp, info: *const nft_traceinfo) -> *const nft_chain {
    if rule.is_null() {
        return &(*(*info).basechain).chain;
    }
    let mut current = rule;
    while !(*current).is_last {
        current = nft_rule_next(current);
    }
    let last = current as *const nft_rule_dp_last;
    if (*last).chain.is_null() {
        DEBUG_NET_WARN_ON_ONCE(1);
        return &(*(*info).basechain).chain;
    }
    (*last).chain
}

pub unsafe fn nft_trace_notify(pkt: *const nft_pktinfo, verdict: *const nft_verdict, rule: *const nft_rule_dp, info: *mut nft_traceinfo) {
    let chain: *const nft_chain;
    let nlh: *mut nlmsghdr;
    let skb: *mut sk_buff;
    let mut mark: u32 = 0;
    let event: u16;

    if !nfnetlink_has_listeners(nft_net(pkt), NFNLGRP_NFTRACE) {
        return;
    }
    chain = nft_trace_get_chain(rule, info);
    let mut size = nlmsg_total_size(core::mem::size_of::<nfgenmsg>())
        + nla_total_size(strlen((*(*chain).table).name)) + nla_total_size(strlen((*chain).name))
        + nla_total_size_64bit(core::mem::size_of::<__be64>()) + nla_total_size(core::mem::size_of::<__be32>())
        + nla_total_size(0) + nla_total_size(core::mem::size_of::<u32>()) * 8
        + nla_total_size(core::mem::size_of::<u8>()) + nla_total_size(NFT_TRACETYPE_LL_HSIZE)
        + nla_total_size(NFT_TRACETYPE_NETWORK_HSIZE) + nla_total_size(NFT_TRACETYPE_TRANSPORT_HSIZE);
    if nft_trace_have_verdict_chain(verdict, info) {
        size += nla_total_size(strlen((*(*verdict).chain).name));
    }

    skb = nlmsg_new(size, GFP_ATOMIC);
    if skb.is_null() { return; }
    event = nfnl_msg_type(NFNL_SUBSYS_NFTABLES, NFT_MSG_TRACE);
    nlh = nfnl_msg_put(skb, 0, 0, event, 0, (*(*info).basechain).r#type.family, NFNETLINK_V0, 0);
    if nlh.is_null() { goto_nla_put_failure(skb); return; }
    if nla_put_be32(skb, NFTA_TRACE_NFPROTO, htonl(nft_pf(pkt))) != 0
        || nla_put_be32(skb, NFTA_TRACE_TYPE, htonl((*info).r#type)) != 0
        || nla_put_u32(skb, NFTA_TRACE_ID, (*info).skbid) != 0
        || nla_put_string(skb, NFTA_TRACE_CHAIN, (*chain).name) != 0
        || nla_put_string(skb, NFTA_TRACE_TABLE, (*(*chain).table).name) != 0
        || nf_trace_fill_rule_info(skb, verdict, rule, info) != 0 { goto_nla_put_failure(skb); return; }

    match (*info).r#type {
        NFT_TRACETYPE_RETURN | NFT_TRACETYPE_RULE => {
            if nft_verdict_dump(skb, NFTA_TRACE_VERDICT, verdict) != 0 { goto_nla_put_failure(skb); return; }
            let v = (*verdict).code & NF_VERDICT_MASK;
            if v == NF_STOLEN { (*info).packet_dumped = true; } else { mark = (*(*pkt).skb).mark; }
        }
        NFT_TRACETYPE_POLICY => {
            mark = (*(*pkt).skb).mark;
            if nla_put_be32(skb, NFTA_TRACE_POLICY, htonl((*(*info).basechain).policy)) != 0 { goto_nla_put_failure(skb); return; }
        }
        _ => {}
    }
    if mark != 0 && nla_put_be32(skb, NFTA_TRACE_MARK, htonl(mark)) != 0 { goto_nla_put_failure(skb); return; }
    if !(*info).packet_dumped {
        if nf_trace_fill_dev_info(skb, nft_in(pkt), nft_out(pkt)) != 0
            || nf_trace_fill_pkt_info(skb, pkt) != 0
            || nf_trace_fill_ct_info(skb, (*pkt).skb) != 0 { goto_nla_put_failure(skb); return; }
        (*info).packet_dumped = true;
    }
    nlmsg_end(skb, nlh);
    nfnetlink_send(skb, nft_net(pkt), 0, NFNLGRP_NFTRACE, 0, GFP_ATOMIC);
}

unsafe fn goto_nla_put_failure(skb: *mut sk_buff) {
    DEBUG_NET_WARN_ON_ONCE(1);
    kfree_skb(skb);
}

pub unsafe fn nft_trace_init(info: *mut nft_traceinfo, pkt: *const nft_pktinfo, chain: *const nft_chain) {
    static mut trace_key: siphash_key_t = core::mem::zeroed();
    let skb = (*pkt).skb;
    (*info).basechain = nft_base_chain(chain);
    (*info).trace = true;
    (*info).nf_trace = (*skb).nf_trace;
    (*info).packet_dumped = false;
    net_get_random_once(&mut trace_key as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<siphash_key_t>());
    (*info).skbid = siphash_3u32(hash32_ptr(skb), skb_get_hash_net(nft_net(pkt), skb), (*skb).skb_iif, &trace_key) as u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
