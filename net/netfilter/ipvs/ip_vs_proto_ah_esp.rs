// SPDX-License-Identifier: GPL-2.0-only
/*
 * ip_vs_proto_ah_esp.c: AH/ESP IPSec load balancing support for IPVS
 *
 * Authors: Julian Anastasov <ja@ssi.bg>, February 2002
 *          Wensong Zhang <wensong@linuxvirtualserver.org>
 */

// C dependencies: linux/in.h, linux/ip.h, linux/module.h, linux/kernel.h,
// linux/netfilter.h, linux/netfilter_ipv4.h, and net/ip_vs.h.

/* TODO:

struct isakmp_hdr {
        __u8            icookie[8];
        __u8            rcookie[8];
        __u8            np;
        __u8            version;
        __u8            xchgtype;
        __u8            flags;
        __u32           msgid;
        __u32           length;
};

*/

const PORT_ISAKMP: u16 = 500;

unsafe fn ah_esp_conn_fill_param_proto(
    ipvs: *mut netns_ipvs,
    af: i32,
    iph: *const ip_vs_iphdr,
    p: *mut ip_vs_conn_param,
) {
    if !ip_vs_iph_inverse(iph) {
        ip_vs_conn_fill_param(
            ipvs,
            af,
            IPPROTO_UDP,
            &(*iph).saddr,
            htons(PORT_ISAKMP),
            &(*iph).daddr,
            htons(PORT_ISAKMP),
            p,
        );
    } else {
        ip_vs_conn_fill_param(
            ipvs,
            af,
            IPPROTO_UDP,
            &(*iph).daddr,
            htons(PORT_ISAKMP),
            &(*iph).saddr,
            htons(PORT_ISAKMP),
            p,
        );
    }
}

unsafe fn ah_esp_conn_in_get(
    ipvs: *mut netns_ipvs,
    af: i32,
    _skb: *const sk_buff,
    iph: *const ip_vs_iphdr,
) -> *mut ip_vs_conn {
    let mut p: ip_vs_conn_param = core::mem::zeroed();

    ah_esp_conn_fill_param_proto(ipvs, af, iph, &mut p);
    let cp = ip_vs_conn_in_get(&mut p);
    if cp.is_null() {
        /*
         * We are not sure if the packet is from our
         * service, so our conn_schedule hook should return NF_ACCEPT
         */
        IP_VS_DBG_BUF(
            12,
            "Unknown ISAKMP entry for outin packet %s%s %s->%s\n",
            if ip_vs_iph_icmp(iph) { "ICMP+" } else { "" },
            (*ip_vs_proto_get((*iph).protocol)).name,
            IP_VS_DBG_ADDR(af, &(*iph).saddr),
            IP_VS_DBG_ADDR(af, &(*iph).daddr),
        );
    }

    cp
}

unsafe fn ah_esp_conn_out_get(
    ipvs: *mut netns_ipvs,
    af: i32,
    _skb: *const sk_buff,
    iph: *const ip_vs_iphdr,
) -> *mut ip_vs_conn {
    let mut p: ip_vs_conn_param = core::mem::zeroed();

    ah_esp_conn_fill_param_proto(ipvs, af, iph, &mut p);
    let cp = ip_vs_conn_out_get(&mut p);
    if cp.is_null() {
        IP_VS_DBG_BUF(
            12,
            "Unknown ISAKMP entry for inout packet %s%s %s->%s\n",
            if ip_vs_iph_icmp(iph) { "ICMP+" } else { "" },
            (*ip_vs_proto_get((*iph).protocol)).name,
            IP_VS_DBG_ADDR(af, &(*iph).saddr),
            IP_VS_DBG_ADDR(af, &(*iph).daddr),
        );
    }

    cp
}

unsafe fn ah_esp_conn_schedule(
    _ipvs: *mut netns_ipvs,
    _af: i32,
    _skb: *mut sk_buff,
    _pd: *mut ip_vs_proto_data,
    verdict: *mut i32,
    _cpp: *mut *mut ip_vs_conn,
    _iph: *mut ip_vs_iphdr,
) -> i32 {
    /*
     * AH/ESP is only related traffic. Pass the packet to IP stack.
     */
    *verdict = NF_ACCEPT;
    0
}

#[cfg(CONFIG_IP_VS_PROTO_AH)]
static mut ip_vs_protocol_ah: ip_vs_protocol = ip_vs_protocol {
    name: "AH",
    protocol: IPPROTO_AH,
    num_states: 1,
    dont_defrag: 1,
    init: None,
    exit: None,
    conn_schedule: Some(ah_esp_conn_schedule),
    conn_in_get: Some(ah_esp_conn_in_get),
    conn_out_get: Some(ah_esp_conn_out_get),
    snat_handler: None,
    dnat_handler: None,
    state_transition: None,
    register_app: None,
    unregister_app: None,
    app_conn_bind: None,
    debug_packet: Some(ip_vs_tcpudp_debug_packet),
    timeout_change: None, // ISAKMP
};

#[cfg(CONFIG_IP_VS_PROTO_ESP)]
static mut ip_vs_protocol_esp: ip_vs_protocol = ip_vs_protocol {
    name: "ESP",
    protocol: IPPROTO_ESP,
    num_states: 1,
    dont_defrag: 1,
    init: None,
    exit: None,
    conn_schedule: Some(ah_esp_conn_schedule),
    conn_in_get: Some(ah_esp_conn_in_get),
    conn_out_get: Some(ah_esp_conn_out_get),
    snat_handler: None,
    dnat_handler: None,
    state_transition: None,
    register_app: None,
    unregister_app: None,
    app_conn_bind: None,
    debug_packet: Some(ip_vs_tcpudp_debug_packet),
    timeout_change: None, // ISAKMP
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
