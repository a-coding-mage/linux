/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies are supplied by other translation units.

/* Each queued (to userspace) skbuff has one of these. */
#[repr(C)]
pub struct nf_queue_entry {
    pub list: list_head,
    pub hash_node: rhash_head,
    pub skb: *mut sk_buff,
    pub skb_dev: *mut net_device,
    pub id: ::core::ffi::c_uint,
    pub hook_index: ::core::ffi::c_uint, /* index in hook_entries->hook[] */
    // CONFIG_BRIDGE_NETFILTER conditionally supplies these fields.
    pub bridge_dev: *mut net_device,
    pub physin: *mut net_device,
    pub physout: *mut net_device,
    pub state: nf_hook_state,
    pub nf_ct_is_unconfirmed: bool,
    pub size: u16, /* sizeof(entry) + saved route keys */

    /* extra space to store route keys */
}

#[inline]
pub unsafe fn nf_queue_entry_reroute(x: *mut nf_queue_entry) -> *mut ::core::ffi::c_void {
    (x as *mut u8).add(core::mem::size_of::<nf_queue_entry>()) as *mut ::core::ffi::c_void
}

/* Packet queuing */
#[repr(C)]
pub struct nf_queue_handler {
    pub outfn: Option<unsafe extern "C" fn(
        entry: *mut nf_queue_entry,
        queuenum: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int>,
    pub nf_hook_drop: Option<unsafe extern "C" fn(net: *mut net)>,
}

extern "C" {
    pub fn nf_register_queue_handler(qh: *const nf_queue_handler);
    pub fn nf_unregister_queue_handler();

    pub fn nf_queue_entry_get_refs(entry: *mut nf_queue_entry) -> bool;
    pub fn nf_queue_entry_free(entry: *mut nf_queue_entry);
}

#[inline]
pub unsafe fn init_hashrandom(jhash_initval: *mut u32) {
    while *jhash_initval == 0 {
        *jhash_initval = get_random_u32();
    }
}

#[inline]
pub unsafe fn hash_v4(iph: *const iphdr, initval: u32) -> u32 {
    /* packets in either direction go into same queue */
    if (*iph).saddr < (*iph).daddr {
        return jhash_3words((*iph).saddr, (*iph).daddr, (*iph).protocol as u32, initval);
    }

    jhash_3words((*iph).daddr, (*iph).saddr, (*iph).protocol as u32, initval)
}

#[inline]
pub unsafe fn hash_v6(ip6h: *const ipv6hdr, initval: u32) -> u32 {
    let (mut a, mut b, c): (u32, u32, u32);

    if (*ip6h).saddr.s6_addr32[3] < (*ip6h).daddr.s6_addr32[3] {
        a = (*ip6h).saddr.s6_addr32[3];
        b = (*ip6h).daddr.s6_addr32[3];
    } else {
        b = (*ip6h).saddr.s6_addr32[3];
        a = (*ip6h).daddr.s6_addr32[3];
    }

    if (*ip6h).saddr.s6_addr32[1] < (*ip6h).daddr.s6_addr32[1] {
        c = (*ip6h).saddr.s6_addr32[1];
    } else {
        c = (*ip6h).daddr.s6_addr32[1];
    }

    jhash_3words(a, b, c, initval)
}

#[inline]
pub unsafe fn hash_bridge(skb: *const sk_buff, initval: u32) -> u32 {
    let mut ip6h: *mut ipv6hdr;
    let mut _ip6h: ipv6hdr = core::mem::zeroed();
    let mut iph: *mut iphdr;
    let mut _iph: iphdr = core::mem::zeroed();

    match eth_hdr(skb).h_proto {
        x if x == htons(ETH_P_IP) => {
            iph = skb_header_pointer(skb, skb_network_offset(skb), core::mem::size_of::<iphdr>(), &mut _iph as *mut _ as *mut ::core::ffi::c_void);
            if !iph.is_null() { return hash_v4(iph, initval); }
        }
        x if x == htons(ETH_P_IPV6) => {
            ip6h = skb_header_pointer(skb, skb_network_offset(skb), core::mem::size_of::<ipv6hdr>(), &mut _ip6h as *mut _ as *mut ::core::ffi::c_void);
            if !ip6h.is_null() { return hash_v6(ip6h, initval); }
        }
        _ => {}
    }

    0
}

#[inline]
pub unsafe fn nfqueue_hash(skb: *const sk_buff, mut queue: u16, queues_total: u16, family: u8, initval: u32) -> u16 {
    match family {
        NFPROTO_IPV4 => queue = queue.wrapping_add(reciprocal_scale(hash_v4(ip_hdr(skb), initval), queues_total)),
        NFPROTO_IPV6 => queue = queue.wrapping_add(reciprocal_scale(hash_v6(ipv6_hdr(skb), initval), queues_total)),
        NFPROTO_BRIDGE => queue = queue.wrapping_add(reciprocal_scale(hash_bridge(skb, initval), queues_total)),
        _ => {}
    }
    queue
}

extern "C" {
    pub fn nf_queue(skb: *mut sk_buff, state: *mut nf_hook_state, index: ::core::ffi::c_uint, verdict: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
