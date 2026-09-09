/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/types.h> in the original header.

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

extern "C" {
    pub static init_net: net;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_seq_and_ts_off_fields {
    pub seq: __u32,
    pub ts_off: __u32,
}

#[repr(C)]
pub union tcp_seq_and_ts_off {
    pub fields: tcp_seq_and_ts_off_fields,
    pub hash64: __u64,
}

extern "C" {
    pub fn secure_ipv4_port_ephemeral(
        saddr: __be32,
        daddr: __be32,
        dport: __be16,
    ) -> __u64;

    pub fn secure_ipv6_port_ephemeral(
        saddr: *const __be32,
        daddr: *const __be32,
        dport: __be16,
    ) -> __u64;

    pub fn secure_tcp_seq_and_ts_off(
        net: *const net,
        saddr: __be32,
        daddr: __be32,
        sport: __be16,
        dport: __be16,
    ) -> tcp_seq_and_ts_off;

    pub fn secure_tcpv6_seq_and_ts_off(
        net: *const net,
        saddr: *const __be32,
        daddr: *const __be32,
        sport: __be16,
        dport: __be16,
    ) -> tcp_seq_and_ts_off;
}

#[inline]
pub unsafe fn secure_tcp_seq(
    saddr: __be32,
    daddr: __be32,
    sport: __be16,
    dport: __be16,
) -> __u32 {
    let ts = secure_tcp_seq_and_ts_off(&init_net, saddr, daddr, sport, dport);

    unsafe { ts.fields.seq }
}

#[inline]
pub unsafe fn secure_tcpv6_seq(
    saddr: *const __be32,
    daddr: *const __be32,
    sport: __be16,
    dport: __be16,
) -> __u32 {
    let ts = secure_tcpv6_seq_and_ts_off(&init_net, saddr, daddr, sport, dport);

    unsafe { ts.fields.seq }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
