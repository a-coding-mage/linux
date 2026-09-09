/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by linux/netfilter.h and
// linux/netfilter/nf_conntrack_tuple_common.h are referenced by name below.

pub const NF_NAT_RANGE_MAP_IPS: u32 = 1 << 0;
pub const NF_NAT_RANGE_PROTO_SPECIFIED: u32 = 1 << 1;
pub const NF_NAT_RANGE_PROTO_RANDOM: u32 = 1 << 2;
pub const NF_NAT_RANGE_PERSISTENT: u32 = 1 << 3;
pub const NF_NAT_RANGE_PROTO_RANDOM_FULLY: u32 = 1 << 4;
pub const NF_NAT_RANGE_PROTO_OFFSET: u32 = 1 << 5;
pub const NF_NAT_RANGE_NETMAP: u32 = 1 << 6;

pub const NF_NAT_RANGE_PROTO_RANDOM_ALL: u32 =
    NF_NAT_RANGE_PROTO_RANDOM | NF_NAT_RANGE_PROTO_RANDOM_FULLY;

pub const NF_NAT_RANGE_MASK: u32 = NF_NAT_RANGE_MAP_IPS
    | NF_NAT_RANGE_PROTO_SPECIFIED
    | NF_NAT_RANGE_PROTO_RANDOM
    | NF_NAT_RANGE_PERSISTENT
    | NF_NAT_RANGE_PROTO_RANDOM_FULLY
    | NF_NAT_RANGE_PROTO_OFFSET
    | NF_NAT_RANGE_NETMAP;

#[repr(C)]
pub struct nf_nat_ipv4_range {
    pub flags: ::std::os::raw::c_uint,
    pub min_ip: u32,
    pub max_ip: u32,
    pub min: nf_conntrack_man_proto,
    pub max: nf_conntrack_man_proto,
}

#[repr(C)]
pub struct nf_nat_ipv4_multi_range_compat {
    pub rangesize: ::std::os::raw::c_uint,
    pub range: [nf_nat_ipv4_range; 1],
}

#[repr(C)]
pub struct nf_nat_range {
    pub flags: ::std::os::raw::c_uint,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_conntrack_man_proto,
    pub max_proto: nf_conntrack_man_proto,
}

#[repr(C)]
pub struct nf_nat_range2 {
    pub flags: ::std::os::raw::c_uint,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_conntrack_man_proto,
    pub max_proto: nf_conntrack_man_proto,
    pub base_proto: nf_conntrack_man_proto,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
