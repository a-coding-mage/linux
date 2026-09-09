/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit.
#[allow(non_camel_case_types)]
pub enum ctl_table_header {}
#[allow(non_camel_case_types)]
pub enum nf_conntrack_ecache {}
#[allow(non_camel_case_types)]
pub enum list_head {}
#[allow(non_camel_case_types)]
pub enum atomic_t {}
#[allow(non_camel_case_types)]
pub enum ip_conntrack_stat {}
#[allow(non_camel_case_types)]
pub enum nf_ct_event_notifier {}

// These sizes are supplied by the corresponding protocol headers.
// TODO: bind these to the external kernel constants in the containing build.
const TCP_CONNTRACK_TIMEOUT_MAX: usize = 0;
const UDP_CT_MAX: usize = 2;
const SCTP_CONNTRACK_MAX: usize = 0;
const GRE_CT_MAX: usize = 2;

#[repr(C)]
pub struct nf_generic_net {
    pub timeout: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct nf_tcp_net {
    pub timeouts: [::core::ffi::c_uint; TCP_CONNTRACK_TIMEOUT_MAX],
    pub tcp_loose: u8,
    pub tcp_be_liberal: u8,
    pub tcp_max_retrans: u8,
    pub tcp_ignore_invalid_rst: u8,
    // Preserved from IS_ENABLED(CONFIG_NF_FLOW_TABLE).
    #[cfg(feature = "CONFIG_NF_FLOW_TABLE")]
    pub offload_timeout: ::core::ffi::c_uint,
}

#[repr(C)]
pub enum udp_conntrack {
    UDP_CT_UNREPLIED,
    UDP_CT_REPLIED,
    UDP_CT_MAX,
}

#[repr(C)]
pub struct nf_udp_net {
    pub timeouts: [::core::ffi::c_uint; UDP_CT_MAX],
    // Preserved from IS_ENABLED(CONFIG_NF_FLOW_TABLE).
    #[cfg(feature = "CONFIG_NF_FLOW_TABLE")]
    pub offload_timeout: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct nf_icmp_net {
    pub timeout: ::core::ffi::c_uint,
}

// Preserved from CONFIG_NF_CT_PROTO_SCTP.
#[cfg(feature = "CONFIG_NF_CT_PROTO_SCTP")]
#[repr(C)]
pub struct nf_sctp_net {
    pub timeouts: [::core::ffi::c_uint; SCTP_CONNTRACK_MAX],
}

// Preserved from CONFIG_NF_CT_PROTO_GRE.
#[cfg(feature = "CONFIG_NF_CT_PROTO_GRE")]
#[repr(C)]
pub enum gre_conntrack {
    GRE_CT_UNREPLIED,
    GRE_CT_REPLIED,
    GRE_CT_MAX,
}

// Preserved from CONFIG_NF_CT_PROTO_GRE.
#[cfg(feature = "CONFIG_NF_CT_PROTO_GRE")]
#[repr(C)]
pub struct nf_gre_net {
    pub keymap_list: *mut list_head,
    pub timeouts: [::core::ffi::c_uint; GRE_CT_MAX],
}

#[repr(C)]
pub struct nf_ip_net {
    pub generic: nf_generic_net,
    pub tcp: nf_tcp_net,
    pub udp: nf_udp_net,
    pub icmp: nf_icmp_net,
    pub icmpv6: nf_icmp_net,
    // Preserved from CONFIG_NF_CT_PROTO_SCTP.
    #[cfg(feature = "CONFIG_NF_CT_PROTO_SCTP")]
    pub sctp: nf_sctp_net,
    // Preserved from CONFIG_NF_CT_PROTO_GRE.
    #[cfg(feature = "CONFIG_NF_CT_PROTO_GRE")]
    pub gre: nf_gre_net,
}

#[repr(C)]
pub struct netns_ct {
    // Preserved from CONFIG_NF_CONNTRACK_EVENTS.
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    pub ecache_dwork_pending: bool,
    pub sysctl_log_invalid: u8, // Log invalid packets
    pub sysctl_events: u8,
    pub sysctl_acct: u8,
    pub sysctl_tstamp: u8,
    pub sysctl_checksum: u8,
    pub stat: *mut ip_conntrack_stat,
    pub nf_conntrack_event_cb: *mut nf_ct_event_notifier,
    pub nf_ct_proto: nf_ip_net,
    // Preserved from CONFIG_NF_CONNTRACK_LABELS.
    #[cfg(feature = "CONFIG_NF_CONNTRACK_LABELS")]
    pub labels_used: atomic_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
