/* SPDX-License-Identifier: GPL-2.0 */
/*
 * IPv4 support for nf_conntrack.
 *
 * 23 Mar 2004: Yasuyuki Kozakai @ USAGI <yasuyuki.kozakai@toshiba.co.jp>
 *	- move L3 protocol dependent part from include/linux/netfilter_ipv4/
 *	  ip_conntarck.h
 */

// External type supplied by the surrounding nf_conntrack implementation.
extern "C" {
    pub static nf_conntrack_l4proto_tcp: nf_conntrack_l4proto;
    pub static nf_conntrack_l4proto_udp: nf_conntrack_l4proto;
    pub static nf_conntrack_l4proto_icmp: nf_conntrack_l4proto;
}

// Preserves the CONFIG_NF_CT_PROTO_SCTP conditional declaration.
#[cfg(feature = "CONFIG_NF_CT_PROTO_SCTP")]
extern "C" {
    pub static nf_conntrack_l4proto_sctp: nf_conntrack_l4proto;
}

// Preserves the CONFIG_NF_CT_PROTO_GRE conditional declaration.
#[cfg(feature = "CONFIG_NF_CT_PROTO_GRE")]
extern "C" {
    pub static nf_conntrack_l4proto_gre: nf_conntrack_l4proto;
}

// Preserves IS_ENABLED(CONFIG_NF_CONNTRACK_PPTP).
#[cfg(feature = "CONFIG_NF_CONNTRACK_PPTP")]
extern "C" {
    pub fn gre_pptp_destroy_siblings(ct: *mut nf_conn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
