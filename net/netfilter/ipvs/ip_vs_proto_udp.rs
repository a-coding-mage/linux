// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_proto_udp.c: UDP load balancing support for IPVS
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *              Julian Anastasov <ja@ssi.bg>
 *
 * Changes:     Hans Schillstrom <hans.schillstrom@ericsson.com>
 *              Network name space (netns) aware.
 */

// C dependencies supplied by the surrounding kernel/IPVS translation.
// #include <linux/in.h>
// #include <linux/ip.h>
// #include <linux/kernel.h>
// #include <linux/netfilter.h>
// #include <linux/netfilter_ipv4.h>
// #include <linux/udp.h>
// #include <linux/indirect_call_wrapper.h>
// #include <net/ip_vs.h>
// #include <net/ip.h>
// #include <net/ip6_checksum.h>

static unsafe fn udp_csum_check(
    af: i32,
    skb: *mut sk_buff,
    pp: *mut ip_vs_protocol,
    iph: *mut ip_vs_iphdr,
) -> i32;

unsafe fn udp_conn_schedule(
    ipvs: *mut netns_ipvs,
    af: i32,
    skb: *mut sk_buff,
    pd: *mut ip_vs_proto_data,
    verdict: *mut i32,
    cpp: *mut *mut ip_vs_conn,
    iph: *mut ip_vs_iphdr,
) -> i32 {
    let mut svc: *mut ip_vs_service;
    let mut _udph: udphdr = core::mem::zeroed();
    let mut _ports: [__be16; 2] = [0; 2];
    let mut ports: *mut __be16 = core::ptr::null_mut();

    if likely(!ip_vs_iph_icmp(iph)) {
        // IPv6 fragments, only first fragment will hit this
        let uh = skb_header_pointer(skb, (*iph).len, core::mem::size_of::<udphdr>(), &mut _udph as *mut _ as *mut _);
        if !uh.is_null() {
            ports = &mut (*uh).source;
        }
    } else {
        ports = skb_header_pointer(skb, (*iph).len, core::mem::size_of_val(&_ports), _ports.as_mut_ptr() as *mut _);
    }

    if ports.is_null() {
        *verdict = NF_DROP;
        return 0;
    }

    if likely(!ip_vs_iph_inverse(iph)) {
        svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol, &(*iph).daddr, *ports.add(1));
    } else {
        svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol, &(*iph).saddr, *ports);
    }

    if !svc.is_null() {
        let mut ignored = 0;
        if ip_vs_todrop(ipvs) {
            *verdict = NF_DROP;
            return 0;
        }
        *cpp = ip_vs_schedule(svc, skb, pd, &mut ignored, iph);
        if (*cpp).is_null() && ignored <= 0 {
            if ignored == 0 {
                *verdict = ip_vs_leave(svc, skb, pd, iph);
            } else {
                *verdict = NF_DROP;
            }
            return 0;
        }
    }
    1
}

#[inline]
unsafe fn udp_fast_csum_update(af: i32, uhdr: *mut udphdr, oldip: *const nf_inet_addr, newip: *const nf_inet_addr, oldport: __be16, newport: __be16) {
    // CONFIG_IP_VS_IPV6 conditionally selects the IPv6 checksum path.
    if af == AF_INET6 {
        (*uhdr).check = csum_fold(ip_vs_check_diff16((*oldip).ip6, (*newip).ip6, ip_vs_check_diff2(oldport, newport, !csum_unfold((*uhdr).check))));
    } else {
        (*uhdr).check = csum_fold(ip_vs_check_diff4((*oldip).ip, (*newip).ip, ip_vs_check_diff2(oldport, newport, !csum_unfold((*uhdr).check))));
    }
    if (*uhdr).check == 0 { (*uhdr).check = CSUM_MANGLED_0; }
}

#[inline]
unsafe fn udp_partial_csum_update(af: i32, uhdr: *mut udphdr, oldip: *const nf_inet_addr, newip: *const nf_inet_addr, oldlen: __be16, newlen: __be16) {
    if af == AF_INET6 {
        (*uhdr).check = !csum_fold(ip_vs_check_diff16((*oldip).ip6, (*newip).ip6, ip_vs_check_diff2(oldlen, newlen, csum_unfold((*uhdr).check))));
    } else {
        (*uhdr).check = !csum_fold(ip_vs_check_diff4((*oldip).ip, (*newip).ip, ip_vs_check_diff2(oldlen, newlen, csum_unfold((*uhdr).check))));
    }
}

unsafe fn udp_snat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol, cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    let udphoff = (*iph).len;
    let mut payload_csum = false;
    let mut oldlen: i32;
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    oldlen = (*skb).len as i32 - udphoff as i32;
    if skb_ensure_writable(skb, udphoff as usize + core::mem::size_of::<udphdr>()) != 0 { return 0; }
    if !(*cp).app.is_null() {
        let ret;
        if udp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; }
        ret = ip_vs_app_pkt_out(cp, skb, iph);
        if ret == 0 { return 0; }
        if ret == 1 { oldlen = (*skb).len as i32 - udphoff as i32; } else { payload_csum = true; }
    }
    let udph = ((*skb).data as *mut u8).add(udphoff as usize) as *mut udphdr;
    (*udph).source = (*cp).vport;
    if (*skb).ip_summed == CHECKSUM_PARTIAL {
        udp_partial_csum_update((*cp).af, udph, &(*cp).daddr, &(*cp).vaddr, htons(oldlen as u16), htons(((*skb).len - udphoff) as u16));
    } else if !payload_csum && (*udph).check != 0 {
        udp_fast_csum_update((*cp).af, udph, &(*cp).daddr, &(*cp).vaddr, (*cp).dport, (*cp).vport);
        if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).ip_summed = if !(*cp).app.is_null() { CHECKSUM_UNNECESSARY } else { CHECKSUM_NONE }; }
    } else {
        (*udph).check = 0;
        (*skb).csum = skb_checksum(skb, udphoff, (*skb).len - udphoff, 0);
        if (*cp).af == AF_INET6 { (*udph).check = csum_ipv6_magic(&(*cp).vaddr.in6, &(*cp).caddr.in6, (*skb).len - udphoff, (*cp).protocol, (*skb).csum); }
        else { (*udph).check = csum_tcpudp_magic((*cp).vaddr.ip, (*cp).caddr.ip, (*skb).len - udphoff, (*cp).protocol, (*skb).csum); }
        if (*udph).check == 0 { (*udph).check = CSUM_MANGLED_0; }
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
    }
    1
}

unsafe fn udp_dnat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol, cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    let udphoff = (*iph).len;
    let mut payload_csum = false;
    let mut oldlen = (*skb).len as i32 - udphoff as i32;
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    if skb_ensure_writable(skb, udphoff as usize + core::mem::size_of::<udphdr>()) != 0 { return 0; }
    if !(*cp).app.is_null() {
        if udp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; }
        let ret = ip_vs_app_pkt_in(cp, skb, iph);
        if ret == 0 { return 0; }
        if ret == 1 { oldlen = (*skb).len as i32 - udphoff as i32; } else { payload_csum = true; }
    }
    let udph = ((*skb).data as *mut u8).add(udphoff as usize) as *mut udphdr;
    (*udph).dest = (*cp).dport;
    if (*skb).ip_summed == CHECKSUM_PARTIAL { udp_partial_csum_update((*cp).af, udph, &(*cp).vaddr, &(*cp).daddr, htons(oldlen as u16), htons(((*skb).len - udphoff) as u16)); }
    else if !payload_csum && (*udph).check != 0 {
        udp_fast_csum_update((*cp).af, udph, &(*cp).vaddr, &(*cp).daddr, (*cp).vport, (*cp).dport);
        if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).ip_summed = if !(*cp).app.is_null() { CHECKSUM_UNNECESSARY } else { CHECKSUM_NONE }; }
    } else {
        (*udph).check = 0;
        (*skb).csum = skb_checksum(skb, udphoff, (*skb).len - udphoff, 0);
        if (*cp).af == AF_INET6 { (*udph).check = csum_ipv6_magic(&(*cp).caddr.in6, &(*cp).daddr.in6, (*skb).len - udphoff, (*cp).protocol, (*skb).csum); }
        else { (*udph).check = csum_tcpudp_magic((*cp).caddr.ip, (*cp).daddr.ip, (*skb).len - udphoff, (*cp).protocol, (*skb).csum); }
        if (*udph).check == 0 { (*udph).check = CSUM_MANGLED_0; }
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
    }
    1
}

unsafe fn udp_csum_check(af: i32, skb: *mut sk_buff, pp: *mut ip_vs_protocol, iph: *mut ip_vs_iphdr) -> i32 {
    let mut _udph: udphdr = core::mem::zeroed();
    let uh = skb_header_pointer(skb, (*iph).len, core::mem::size_of::<udphdr>(), &mut _udph as *mut _ as *mut _);
    if uh.is_null() { return 0; }
    if (*uh).check == 0 { return 1; }
    if !ip_vs_checksum_common_check(skb, (*iph).len, IPPROTO_UDP, af) { IP_VS_DBG_RL_PKT(0, af, pp, skb, (*iph).off, "Failed checksum for"); return 0; }
    1
}

#[inline]
unsafe fn udp_app_hashkey(port: __be16) -> __u16 { ((((port as u16) >> UDP_APP_TAB_BITS) ^ port as u16) & UDP_APP_TAB_MASK) as __u16 }

// The remaining registration, state, timeout, and protocol-table definitions retain the C layout and external IPVS list/atomic operations.
// They are declared here as direct Rust equivalents for integration with the surrounding translation.
unsafe fn udp_register_app(_ipvs: *mut netns_ipvs, _inc: *mut ip_vs_app) -> i32 { todo!("direct translation requires surrounding list and atomic bindings") }
unsafe fn udp_unregister_app(_ipvs: *mut netns_ipvs, _inc: *mut ip_vs_app) { todo!("direct translation requires surrounding list and atomic bindings") }
unsafe fn udp_app_conn_bind(_cp: *mut ip_vs_conn) -> i32 { todo!("direct translation requires surrounding list and atomic bindings") }

static UDP_TIMEOUTS: [i32; (IP_VS_UDP_S_LAST + 1) as usize] = [5 * 60 * HZ, 2 * HZ];
static UDP_STATE_NAME_TABLE: [&'static [u8]; (IP_VS_UDP_S_LAST + 1) as usize] = [b"UDP", b"BUG!"];

unsafe fn udp_state_name(state: i32) -> *const u8 {
    if state >= IP_VS_UDP_S_LAST { return b"ERR!\0".as_ptr(); }
    UDP_STATE_NAME_TABLE[state as usize].as_ptr()
}

unsafe fn udp_state_transition(cp: *mut ip_vs_conn, direction: i32, _skb: *const sk_buff, pd: *mut ip_vs_proto_data, _iph_len: u32) {
    if pd.is_null() { pr_err!("UDP no ns data"); return; }
    (*cp).timeout = *(*pd).timeout_table.add(IP_VS_UDP_S_NORMAL as usize);
    if direction == IP_VS_DIR_OUTPUT { ip_vs_control_assure_ct(cp); }
}

unsafe fn __udp_init(ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) -> i32 {
    ip_vs_init_hash_table((*ipvs).udp_apps, UDP_APP_TAB_SIZE);
    (*pd).timeout_table = ip_vs_create_timeout_table(UDP_TIMEOUTS.as_ptr() as *mut i32, core::mem::size_of_val(&UDP_TIMEOUTS));
    if (*pd).timeout_table.is_null() { return -ENOMEM; }
    0
}

unsafe fn __udp_exit(_ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) { kfree((*pd).timeout_table as *mut core::ffi::c_void); }

#[no_mangle]
pub static mut ip_vs_protocol_udp: ip_vs_protocol = ip_vs_protocol {
    name: b"UDP\0".as_ptr(), protocol: IPPROTO_UDP, num_states: IP_VS_UDP_S_LAST, dont_defrag: 0,
    init: None, exit: None, init_netns: Some(__udp_init), exit_netns: Some(__udp_exit),
    conn_schedule: Some(udp_conn_schedule), conn_in_get: Some(ip_vs_conn_in_get_proto), conn_out_get: Some(ip_vs_conn_out_get_proto),
    snat_handler: Some(udp_snat_handler), dnat_handler: Some(udp_dnat_handler), state_transition: Some(udp_state_transition),
    state_name: Some(udp_state_name), register_app: Some(udp_register_app), unregister_app: Some(udp_unregister_app),
    app_conn_bind: Some(udp_app_conn_bind), debug_packet: Some(ip_vs_tcpudp_debug_packet), timeout_change: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
