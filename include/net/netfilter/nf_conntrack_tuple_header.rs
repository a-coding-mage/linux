/* SPDX-License-Identifier: GPL-2.0 */
/* Definitions and declarations for tuple. */

/* The included C headers provide nf_inet_addr, nf_conntrack_man_proto,
 * hlist_nulls_node, enum ip_conntrack_dir, AF_INET/AF_INET6, and printk. */

pub const NF_CT_TUPLE_L3SIZE: usize = 4;

/* The manipulable part of the tuple. */
#[repr(C)]
pub struct nf_conntrack_man {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
    /* Layer 3 protocol */
    pub l3num: u16,
}

#[repr(C)]
pub union nf_conntrack_tuple_dst_u {
    /* Add other protocols here. */
    pub all: __be16,
    pub tcp: nf_conntrack_tuple_tcp,
    pub udp: nf_conntrack_tuple_udp,
    pub icmp: nf_conntrack_tuple_icmp,
    pub dccp: nf_conntrack_tuple_dccp,
    pub sctp: nf_conntrack_tuple_sctp,
    pub gre: nf_conntrack_tuple_gre,
}

#[repr(C)]
pub struct nf_conntrack_tuple_tcp { pub port: __be16 }
#[repr(C)]
pub struct nf_conntrack_tuple_udp { pub port: __be16 }
#[repr(C)]
pub struct nf_conntrack_tuple_icmp { pub type_: u8, pub code: u8 }
#[repr(C)]
pub struct nf_conntrack_tuple_dccp { pub port: __be16 }
#[repr(C)]
pub struct nf_conntrack_tuple_sctp { pub port: __be16 }
#[repr(C)]
pub struct nf_conntrack_tuple_gre { pub key: __be16 }

#[repr(C)]
pub struct nf_conntrack_tuple_dst {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_tuple_dst_u,
    /* The protocol. */
    pub protonum: u8,
    /* The direction must be ignored for the tuplehash. */
    pub __nfct_hash_offsetend: (),
    /* The direction (for tuplehash). */
    pub dir: u8,
}

/* This contains the information to distinguish a connection. */
#[repr(C)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_man,
    pub dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
pub struct nf_conntrack_tuple_mask_src {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
}

#[repr(C)]
pub struct nf_conntrack_tuple_mask {
    pub src: nf_conntrack_tuple_mask_src,
}

pub unsafe fn nf_ct_dump_tuple_ip(t: *const nf_conntrack_tuple) {
    #[cfg(debug_assertions)]
    {
        /* printk("tuple %p: %u %pI4:%hu -> %pI4:%hu\n", ...); */
        let _ = t;
    }
}

pub unsafe fn nf_ct_dump_tuple_ipv6(t: *const nf_conntrack_tuple) {
    #[cfg(debug_assertions)]
    {
        /* printk("tuple %p: %u %pI6 %hu -> %pI6 %hu\n", ...); */
        let _ = t;
    }
}

pub unsafe fn nf_ct_dump_tuple(t: *const nf_conntrack_tuple) {
    match (*t).src.l3num {
        AF_INET => nf_ct_dump_tuple_ip(t),
        AF_INET6 => nf_ct_dump_tuple_ipv6(t),
        _ => {}
    }
}

/* If we're the first tuple, it's the original dir. */
#[inline]
pub unsafe fn NF_CT_DIRECTION(h: *const nf_conntrack_tuple_hash) -> ip_conntrack_dir {
    (*h).tuple.dst.dir as ip_conntrack_dir
}

/* Connections have two entries in the hash table: one for each way. */
#[repr(C)]
pub struct nf_conntrack_tuple_hash {
    pub hnnode: hlist_nulls_node,
    pub tuple: nf_conntrack_tuple,
}

pub unsafe fn __nf_ct_tuple_src_equal(t1: *const nf_conntrack_tuple, t2: *const nf_conntrack_tuple) -> bool {
    nf_inet_addr_cmp(&(*t1).src.u3, &(*t2).src.u3) &&
        (*t1).src.u.all == (*t2).src.u.all &&
        (*t1).src.l3num == (*t2).src.l3num
}

pub unsafe fn __nf_ct_tuple_dst_equal(t1: *const nf_conntrack_tuple, t2: *const nf_conntrack_tuple) -> bool {
    nf_inet_addr_cmp(&(*t1).dst.u3, &(*t2).dst.u3) &&
        (*t1).dst.u.all == (*t2).dst.u.all &&
        (*t1).dst.protonum == (*t2).dst.protonum
}

pub unsafe fn nf_ct_tuple_equal(t1: *const nf_conntrack_tuple, t2: *const nf_conntrack_tuple) -> bool {
    __nf_ct_tuple_src_equal(t1, t2) && __nf_ct_tuple_dst_equal(t1, t2)
}

pub unsafe fn nf_ct_tuple_mask_equal(m1: *const nf_conntrack_tuple_mask, m2: *const nf_conntrack_tuple_mask) -> bool {
    nf_inet_addr_cmp(&(*m1).src.u3, &(*m2).src.u3) && (*m1).src.u.all == (*m2).src.u.all
}

pub unsafe fn nf_ct_tuple_src_mask_cmp(t1: *const nf_conntrack_tuple, t2: *const nf_conntrack_tuple, mask: *const nf_conntrack_tuple_mask) -> bool {
    for count in 0..NF_CT_TUPLE_L3SIZE {
        if (((*t1).src.u3.all[count] ^ (*t2).src.u3.all[count]) & (*mask).src.u3.all[count]) != 0 { return false; }
    }
    if (((*t1).src.u.all ^ (*t2).src.u.all) & (*mask).src.u.all) != 0 { return false; }
    if (*t1).src.l3num != (*t2).src.l3num || (*t1).dst.protonum != (*t2).dst.protonum { return false; }
    true
}

pub unsafe fn nf_ct_tuple_mask_cmp(t: *const nf_conntrack_tuple, tuple: *const nf_conntrack_tuple, mask: *const nf_conntrack_tuple_mask) -> bool {
    nf_ct_tuple_src_mask_cmp(t, tuple, mask) && __nf_ct_tuple_dst_equal(t, tuple)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
