/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

extern "C" {
    pub fn ip_set_get_ip4_port(
        skb: *const sk_buff,
        src: bool,
        port: *mut __be16,
        proto: *mut u8,
    ) -> bool;
}

/* Preserved from IS_ENABLED(CONFIG_IP6_NF_IPTABLES). */
#[cfg(feature = "CONFIG_IP6_NF_IPTABLES")]
extern "C" {
    pub fn ip_set_get_ip6_port(
        skb: *const sk_buff,
        src: bool,
        port: *mut __be16,
        proto: *mut u8,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_IP6_NF_IPTABLES"))]
#[inline]
pub unsafe fn ip_set_get_ip6_port(
    _skb: *const sk_buff,
    _src: bool,
    _port: *mut __be16,
    _proto: *mut u8,
) -> bool {
    false
}

#[inline]
pub fn ip_set_proto_with_ports(proto: u8) -> bool {
    match proto {
        IPPROTO_TCP | IPPROTO_SCTP | IPPROTO_UDP | IPPROTO_UDPLITE => true,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
