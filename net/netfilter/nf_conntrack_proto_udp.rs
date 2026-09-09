// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static UDP_TIMEOUTS: [u32; UDP_CT_MAX as usize] = [
    [UDP_CT_UNREPLIED as usize] = 30 * HZ,
    [UDP_CT_REPLIED as usize] = 120 * HZ,
];

unsafe fn udp_get_timeouts(net: *mut net) -> *mut u32 {
    nf_udp_pernet(net).timeouts.as_mut_ptr()
}

unsafe fn udp_error_log(skb: *const sk_buff, state: *const nf_hook_state, msg: *const c_char) {
    nf_l4proto_log_invalid(skb, state, IPPROTO_UDP, b"%s\0".as_ptr() as *const c_char, msg);
}

unsafe fn udp_validate_len(skb: *mut sk_buff, hdr: *const udphdr, dataoff: u32) -> bool {
    let udplen: u32 = udp_get_len(skb, hdr, dataoff);
    let skblen: u32 = (*skb).len - dataoff;
    if udplen > skblen || udplen < core::mem::size_of::<udphdr>() as u32 {
        return false;
    }
    true
}

unsafe fn udp_error(skb: *mut sk_buff, dataoff: u32, state: *const nf_hook_state) -> bool {
    let mut hdr_storage: udphdr = core::mem::zeroed();
    let hdr = skb_header_pointer(
        skb,
        dataoff,
        core::mem::size_of::<udphdr>() as u32,
        &mut hdr_storage as *mut udphdr as *mut c_void,
    );
    if hdr.is_null() {
        udp_error_log(skb, state, b"short packet\0".as_ptr() as *const c_char);
        return true;
    }

    if !udp_validate_len(skb, hdr, dataoff) {
        udp_error_log(skb, state, b"truncated/malformed packet\0".as_ptr() as *const c_char);
        return true;
    }

    if (*hdr).check == 0 {
        return false;
    }

    if (*state).hook == NF_INET_PRE_ROUTING
        && (*(*state).net).ct.sysctl_checksum
        && nf_checksum(skb, (*state).hook, dataoff, IPPROTO_UDP, (*state).pf) != 0
    {
        udp_error_log(skb, state, b"bad checksum\0".as_ptr() as *const c_char);
        return true;
    }
    false
}

/* Returns verdict for packet, and may modify conntracktype. */
pub unsafe fn nf_conntrack_udp_packet(
    ct: *mut nf_conn,
    skb: *mut sk_buff,
    dataoff: u32,
    ctinfo: ip_conntrack_info,
    state: *const nf_hook_state,
) -> i32 {
    if udp_error(skb, dataoff, state) {
        return -NF_ACCEPT;
    }

    let mut timeouts = nf_ct_timeout_lookup(ct);
    if timeouts.is_null() {
        timeouts = udp_get_timeouts(nf_ct_net(ct));
    }

    let status: c_ulong = READ_ONCE((*ct).status);
    if (status & IPS_CONFIRMED) == 0 {
        (*ct).proto.udp.stream_ts = 2 * HZ + jiffies;
    }

    if status & IPS_SEEN_REPLY != 0 {
        let mut extra = *timeouts.add(UDP_CT_UNREPLIED as usize);
        let mut stream = false;
        if time_after(jiffies, (*ct).proto.udp.stream_ts) {
            extra = *timeouts.add(UDP_CT_REPLIED as usize);
            stream = (status & IPS_ASSURED) == 0;
        }
        nf_ct_refresh_acct(ct, ctinfo, skb, extra);

        if unlikely((status & IPS_NAT_CLASH) != 0) {
            return NF_ACCEPT;
        }
        if stream && !test_and_set_bit(IPS_ASSURED_BIT, &mut (*ct).status) {
            nf_conntrack_event_cache(IPCT_ASSURED, ct);
        }
    } else {
        nf_ct_refresh_acct(ct, ctinfo, skb, *timeouts.add(UDP_CT_UNREPLIED as usize));
    }
    NF_ACCEPT
}

// CONFIG_NF_CONNTRACK_TIMEOUT conditional section.
#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
unsafe fn udp_timeout_nlattr_to_obj(tb: *mut *mut nlattr, net: *mut net, data: *mut c_void) -> i32 {
    let mut timeouts = data as *mut u32;
    let un = nf_udp_pernet(net);
    if timeouts.is_null() {
        timeouts = un.timeouts.as_mut_ptr();
    }
    *timeouts.add(UDP_CT_UNREPLIED as usize) = *un.timeouts.as_ptr().add(UDP_CT_UNREPLIED as usize);
    *timeouts.add(UDP_CT_REPLIED as usize) = *un.timeouts.as_ptr().add(UDP_CT_REPLIED as usize);
    if !(*tb.add(CTA_TIMEOUT_UDP_UNREPLIED as usize)).is_null() {
        *timeouts.add(UDP_CT_UNREPLIED as usize) = ntohl(nla_get_be32(*tb.add(CTA_TIMEOUT_UDP_UNREPLIED as usize))) * HZ;
    }
    if !(*tb.add(CTA_TIMEOUT_UDP_REPLIED as usize)).is_null() {
        *timeouts.add(UDP_CT_REPLIED as usize) = ntohl(nla_get_be32(*tb.add(CTA_TIMEOUT_UDP_REPLIED as usize))) * HZ;
    }
    0
}

#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
unsafe fn udp_timeout_obj_to_nlattr(skb: *mut sk_buff, data: *const c_void) -> i32 {
    let timeouts = data as *const u32;
    if nla_put_be32(skb, CTA_TIMEOUT_UDP_UNREPLIED, htonl(*timeouts.add(UDP_CT_UNREPLIED as usize) / HZ)) != 0
        || nla_put_be32(skb, CTA_TIMEOUT_UDP_REPLIED, htonl(*timeouts.add(UDP_CT_REPLIED as usize) / HZ)) != 0
    {
        return -ENOSPC;
    }
    0
}

#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
static UDP_TIMEOUT_NLA_POLICY: [nla_policy; (CTA_TIMEOUT_UDP_MAX + 1) as usize] = [
    [CTA_TIMEOUT_UDP_UNREPLIED as usize] = nla_policy { r#type: NLA_U32 },
    [CTA_TIMEOUT_UDP_REPLIED as usize] = nla_policy { r#type: NLA_U32 },
];

pub unsafe fn nf_conntrack_udp_init_net(net: *mut net) {
    let un = nf_udp_pernet(net);
    for i in 0..UDP_CT_MAX as usize {
        un.timeouts[i] = UDP_TIMEOUTS[i];
    }
    // IS_ENABLED(CONFIG_NF_FLOW_TABLE)
    #[cfg(CONFIG_NF_FLOW_TABLE)]
    { un.offload_timeout = 30 * HZ; }
}

pub static nf_conntrack_l4proto_udp: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_UDP,
    allow_clash: true,
    // IS_ENABLED(CONFIG_NF_CT_NETLINK) fields omitted when unavailable.
    #[cfg(CONFIG_NF_CT_NETLINK)]
    tuple_to_nlattr: Some(nf_ct_port_tuple_to_nlattr),
    #[cfg(CONFIG_NF_CT_NETLINK)]
    nlattr_to_tuple: Some(nf_ct_port_nlattr_to_tuple),
    #[cfg(CONFIG_NF_CT_NETLINK)]
    nlattr_tuple_size: Some(nf_ct_port_nlattr_tuple_size),
    #[cfg(CONFIG_NF_CT_NETLINK)]
    nla_policy: Some(nf_ct_port_nla_policy),
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: nf_conntrack_timeout { 
        nlattr_to_obj: Some(udp_timeout_nlattr_to_obj),
        obj_to_nlattr: Some(udp_timeout_obj_to_nlattr),
        nlattr_max: CTA_TIMEOUT_UDP_MAX,
        obj_size: core::mem::size_of::<u32>() * CTA_TIMEOUT_UDP_MAX as usize,
        nla_policy: Some(UDP_TIMEOUT_NLA_POLICY.as_ptr()),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
