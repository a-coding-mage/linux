/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by linux/types.h and linux/netfilter/ipset/ip_set.h.

/* Revision 0 interface: backward compatible with netfilter/iptables */

/*
 * Option flags for kernel operations (xt_set_info_v0)
 */
pub const IPSET_SRC: u32 = 0x01; /* Source match/add */
pub const IPSET_DST: u32 = 0x02; /* Destination match/add */
pub const IPSET_MATCH_INV: u32 = 0x04; /* Inverse matching */

#[repr(C)]
pub struct xt_set_info_v0 {
    pub index: ip_set_id_t,
    pub u: xt_set_info_v0_u,
}

#[repr(C)]
pub union xt_set_info_v0_u {
    pub flags: [__u32; (IPSET_DIM_MAX + 1) as usize],
    pub compat: xt_set_info_v0_u_compat,
}

#[repr(C)]
pub struct xt_set_info_v0_u_compat {
    pub __flags: [__u32; IPSET_DIM_MAX as usize],
    pub dim: __u8,
    pub flags: __u8,
}

/* match and target infos */
#[repr(C)]
pub struct xt_set_info_match_v0 {
    pub match_set: xt_set_info_v0,
}

#[repr(C)]
pub struct xt_set_info_target_v0 {
    pub add_set: xt_set_info_v0,
    pub del_set: xt_set_info_v0,
}

/* Revision 1  match and target */
#[repr(C)]
pub struct xt_set_info {
    pub index: ip_set_id_t,
    pub dim: __u8,
    pub flags: __u8,
}

/* match and target infos */
#[repr(C)]
pub struct xt_set_info_match_v1 {
    pub match_set: xt_set_info,
}

#[repr(C)]
pub struct xt_set_info_target_v1 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
}

/* Revision 2 target */
#[repr(C)]
pub struct xt_set_info_target_v2 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
    pub flags: __u32,
    pub timeout: __u32,
}

/* Revision 3 match */
#[repr(C)]
pub struct xt_set_info_match_v3 {
    pub match_set: xt_set_info,
    pub packets: ip_set_counter_match0,
    pub bytes: ip_set_counter_match0,
    pub flags: __u32,
}

/* Revision 3 target */
#[repr(C)]
pub struct xt_set_info_target_v3 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
    pub map_set: xt_set_info,
    pub flags: __u32,
    pub timeout: __u32,
}

/* Revision 4 match */
#[repr(C)]
pub struct xt_set_info_match_v4 {
    pub match_set: xt_set_info,
    pub packets: ip_set_counter_match,
    pub bytes: ip_set_counter_match,
    pub flags: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
