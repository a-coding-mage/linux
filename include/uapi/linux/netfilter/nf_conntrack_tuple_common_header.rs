/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the Linux type definitions.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ip_conntrack_dir {
    IP_CT_DIR_ORIGINAL,
    IP_CT_DIR_REPLY,
    IP_CT_DIR_MAX,
}

/* The protocol-specific manipulable parts of the tuple: always in
 * network order
 */
#[repr(C)]
pub union nf_conntrack_man_proto {
    /* Add other protocols here. */
    pub all: __be16,

    pub tcp: nf_conntrack_man_proto_tcp,
    pub udp: nf_conntrack_man_proto_udp,
    pub icmp: nf_conntrack_man_proto_icmp,
    pub dccp: nf_conntrack_man_proto_dccp,
    pub sctp: nf_conntrack_man_proto_sctp,
    pub gre: nf_conntrack_man_proto_gre,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_tcp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_udp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_icmp {
    pub id: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_dccp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_sctp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto_gre {
    pub key: __be16, /* GRE key is 32bit, PPtP only uses 16bit */
}

#[macro_export]
macro_rules! CTINFO2DIR {
    ($ctinfo:expr) => {
        if ($ctinfo) >= IP_CT_IS_REPLY {
            ip_conntrack_dir::IP_CT_DIR_REPLY
        } else {
            ip_conntrack_dir::IP_CT_DIR_ORIGINAL
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
