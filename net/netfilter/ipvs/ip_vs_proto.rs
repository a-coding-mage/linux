// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_proto.c: transport protocol load balancing support for IPVS
 *
 * Rust translation of the implementation source.
 */

// C dependencies are supplied by the surrounding kernel/IPVS translation.

const IP_VS_PROTO_TAB_SIZE: usize = 32;

#[inline]
const fn ip_vs_proto_hash(proto: u16) -> usize {
    (proto as usize) & (IP_VS_PROTO_TAB_SIZE - 1)
}

static mut IP_VS_PROTO_TABLE: [*mut ip_vs_protocol; IP_VS_PROTO_TAB_SIZE] =
    [core::ptr::null_mut(); IP_VS_PROTO_TAB_SIZE];

static IP_VS_CTPL_STATE_NAME_TABLE: [&'static core::ffi::CStr; 2] = [
    c"NONE",
    c"ASSURED",
];

unsafe fn register_ip_vs_protocol(pp: *mut ip_vs_protocol) -> i32 {
    let hash = ip_vs_proto_hash((*pp).protocol);
    (*pp).next = IP_VS_PROTO_TABLE[hash];
    IP_VS_PROTO_TABLE[hash] = pp;
    if let Some(init) = (*pp).init {
        init(pp);
    }
    0
}

unsafe fn register_ip_vs_proto_netns(
    ipvs: *mut netns_ipvs,
    pp: *mut ip_vs_protocol,
) -> i32 {
    let hash = ip_vs_proto_hash((*pp).protocol);
    let pd = kzalloc_obj::<ip_vs_proto_data>();
    if pd.is_null() {
        return -ENOMEM;
    }
    (*pd).pp = pp;
    (*pd).next = (*ipvs).proto_data_table[hash];
    (*ipvs).proto_data_table[hash] = pd;
    atomic_set(&mut (*pd).appcnt, 0);
    if let Some(init_netns) = (*pp).init_netns {
        let ret = init_netns(ipvs, pd);
        if ret != 0 {
            (*ipvs).proto_data_table[hash] = (*pd).next;
            kfree(pd);
            return ret;
        }
    }
    0
}

unsafe fn unregister_ip_vs_protocol(pp: *mut ip_vs_protocol) -> i32 {
    let hash = ip_vs_proto_hash((*pp).protocol);
    let mut pp_p = &mut IP_VS_PROTO_TABLE[hash] as *mut *mut ip_vs_protocol;
    while !(*pp_p).is_null() {
        if *pp_p == pp {
            *pp_p = (*pp).next;
            if let Some(exit) = (*pp).exit {
                exit(pp);
            }
            return 0;
        }
        pp_p = &mut (**pp_p).next;
    }
    -ESRCH
}

unsafe fn unregister_ip_vs_proto_netns(
    ipvs: *mut netns_ipvs,
    pd: *mut ip_vs_proto_data,
) -> i32 {
    let hash = ip_vs_proto_hash((*(*pd).pp).protocol);
    let mut pd_p = &mut (*ipvs).proto_data_table[hash] as *mut *mut ip_vs_proto_data;
    while !(*pd_p).is_null() {
        if *pd_p == pd {
            *pd_p = (*pd).next;
            if let Some(exit_netns) = (*(*pd).pp).exit_netns {
                exit_netns(ipvs, pd);
            }
            kfree(pd);
            return 0;
        }
        pd_p = &mut (**pd_p).next;
    }
    -ESRCH
}

pub unsafe fn ip_vs_proto_get(proto: u16) -> *mut ip_vs_protocol {
    let hash = ip_vs_proto_hash(proto);
    let mut pp = IP_VS_PROTO_TABLE[hash];
    while !pp.is_null() {
        if (*pp).protocol == proto {
            return pp;
        }
        pp = (*pp).next;
    }
    core::ptr::null_mut()
}

pub unsafe fn ip_vs_proto_data_get(
    ipvs: *mut netns_ipvs,
    proto: u16,
) -> *mut ip_vs_proto_data {
    let hash = ip_vs_proto_hash(proto);
    let mut pd = (*ipvs).proto_data_table[hash];
    while !pd.is_null() {
        if (*(*pd).pp).protocol == proto {
            return pd;
        }
        pd = (*pd).next;
    }
    core::ptr::null_mut()
}

pub unsafe fn ip_vs_protocol_timeout_change(ipvs: *mut netns_ipvs, flags: i32) {
    for i in 0..IP_VS_PROTO_TAB_SIZE {
        let mut pd = (*ipvs).proto_data_table[i];
        while !pd.is_null() {
            if let Some(timeout_change) = (*(*pd).pp).timeout_change {
                timeout_change(pd, flags);
            }
            pd = (*pd).next;
        }
    }
}

pub unsafe fn ip_vs_create_timeout_table(table: *mut i32, size: usize) -> *mut i32 {
    kmemdup(table, size, GFP_KERNEL)
}

pub unsafe fn ip_vs_state_name(cp: *const ip_vs_conn) -> *const core::ffi::c_char {
    let state = (*cp).state as usize;
    if ((*cp).flags & IP_VS_CONN_F_TEMPLATE) != 0 {
        if state >= IP_VS_CTPL_S_LAST as usize {
            return c"ERR!".as_ptr();
        }
        return IP_VS_CTPL_STATE_NAME_TABLE[state].as_ptr();
    }
    let pp = ip_vs_proto_get((*cp).protocol);
    if pp.is_null() || (*pp).state_name.is_none() {
        return if (*cp).protocol == IPPROTO_IP { c"NONE".as_ptr() } else { c"ERR!".as_ptr() };
    }
    (*pp).state_name.unwrap()(state as u32)
}

unsafe fn ip_vs_tcpudp_debug_packet_v4(
    pp: *mut ip_vs_protocol,
    skb: *const sk_buff,
    offset: i32,
    msg: *const core::ffi::c_char,
) {
    let mut _iph = iphdr::default();
    let ih = skb_header_pointer(skb, offset, core::mem::size_of::<ip_hdr>(), &mut _iph as *mut _ as *mut _);
    let mut buf = [0i8; 128];
    if ih.is_null() {
        sprintf(buf.as_mut_ptr(), c"TRUNCATED".as_ptr());
    } else if (*ih).frag_off & htons(IP_OFFSET) != 0 {
        sprintf(buf.as_mut_ptr(), c"%pI4->%pI4 frag".as_ptr(), &(*ih).saddr, &(*ih).daddr);
    } else {
        let mut ports = [0u16; 2];
        let pptr = skb_header_pointer(skb, offset + ((*ih).ihl as i32) * 4, 4, ports.as_mut_ptr() as *mut _);
        if pptr.is_null() {
            sprintf(buf.as_mut_ptr(), c"TRUNCATED %pI4->%pI4".as_ptr(), &(*ih).saddr, &(*ih).daddr);
        } else {
            sprintf(buf.as_mut_ptr(), c"%pI4:%u->%pI4:%u".as_ptr(), &(*ih).saddr, ntohs(ports[0]), &(*ih).daddr, ntohs(ports[1]));
        }
    }
    pr_debug(c"%s: %s %s\n".as_ptr(), msg, (*pp).name, buf.as_ptr());
}

pub unsafe fn ip_vs_tcpudp_debug_packet(
    af: i32,
    pp: *mut ip_vs_protocol,
    skb: *const sk_buff,
    offset: i32,
    msg: *const core::ffi::c_char,
) {
    // CONFIG_IP_VS_IPV6 selects the IPv6 implementation in the kernel build.
    if af == AF_INET6 {
        // IPv6 implementation is supplied when CONFIG_IP_VS_IPV6 is enabled.
        ip_vs_tcpudp_debug_packet_v6(pp, skb, offset, msg);
    } else {
        ip_vs_tcpudp_debug_packet_v4(pp, skb, offset, msg);
    }
}

pub unsafe fn ip_vs_protocol_net_init(ipvs: *mut netns_ipvs) -> i32 {
    let protos: [*mut ip_vs_protocol; 5] = [
        &mut ip_vs_protocol_tcp,
        &mut ip_vs_protocol_udp,
        &mut ip_vs_protocol_sctp,
        &mut ip_vs_protocol_ah,
        &mut ip_vs_protocol_esp,
    ];
    for pp in protos {
        let ret = register_ip_vs_proto_netns(ipvs, pp);
        if ret < 0 {
            ip_vs_protocol_net_cleanup(ipvs);
            return ret;
        }
    }
    0
}

pub unsafe fn ip_vs_protocol_net_cleanup(ipvs: *mut netns_ipvs) {
    for i in 0..IP_VS_PROTO_TAB_SIZE {
        while !(*ipvs).proto_data_table[i].is_null() {
            unregister_ip_vs_proto_netns(ipvs, (*ipvs).proto_data_table[i]);
        }
    }
}

pub unsafe fn ip_vs_protocol_init() -> i32 {
    let protos: [*mut ip_vs_protocol; 5] = [
        &mut ip_vs_protocol_tcp,
        &mut ip_vs_protocol_udp,
        &mut ip_vs_protocol_sctp,
        &mut ip_vs_protocol_ah,
        &mut ip_vs_protocol_esp,
    ];
    for pp in protos {
        register_ip_vs_protocol(pp);
    }
    pr_info(c"Registered protocols\n".as_ptr());
    0
}

pub unsafe fn ip_vs_protocol_cleanup() {
    for i in 0..IP_VS_PROTO_TAB_SIZE {
        while !IP_VS_PROTO_TABLE[i].is_null() {
            unregister_ip_vs_protocol(IP_VS_PROTO_TABLE[i]);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
