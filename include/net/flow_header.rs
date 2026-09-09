/* SPDX-License-Identifier: GPL-2.0 */
/* Generic internet FLOW. */

// Dependencies supplied by other translation units:
// linux/in6.h, linux/atomic.h, linux/container_of.h, linux/uidgid.h,
// and net/inet_dscp.h.

pub const LOOPBACK_IFINDEX: i32 = 1;

#[repr(C)]
pub struct flowi_tunnel {
    pub tun_id: __be64,
}

#[repr(C)]
pub struct flowi_common {
    pub flowic_oif: i32,
    pub flowic_iif: i32,
    pub flowic_l3mdev: i32,
    pub flowic_mark: __u32,
    pub flowic_dscp: dscp_t,
    pub flowic_scope: __u8,
    pub flowic_proto: __u8,
    pub flowic_flags: __u8,
    pub flowic_secid: __u32,
    pub flowic_uid: kuid_t,
    pub flowic_multipath_hash: __u32,
    pub flowic_tun_key: flowi_tunnel,
}

pub const FLOWI_FLAG_ANYSRC: __u8 = 0x01;
pub const FLOWI_FLAG_KNOWN_NH: __u8 = 0x02;
pub const FLOWI_FLAG_L3MDEV_OIF: __u8 = 0x04;
pub const FLOWI_FLAG_ANY_SPORT: __u8 = 0x08;

#[repr(C)]
pub struct flowi_uli_ports { pub dport: __be16, pub sport: __be16 }
#[repr(C)]
pub struct flowi_uli_icmpt { pub r#type: __u8, pub code: __u8 }
#[repr(C)]
pub struct flowi_uli_mht { pub r#type: __u8 }

#[repr(C)]
pub union flowi_uli {
    pub ports: flowi_uli_ports,
    pub icmpt: flowi_uli_icmpt,
    pub gre_key: __be32,
    pub mht: flowi_uli_mht,
}

#[repr(C, align(8))]
pub struct flowi4 {
    pub __fl_common: flowi_common,
    pub saddr: __be32,
    pub daddr: __be32,
    pub uli: flowi_uli,
}

#[inline]
pub unsafe fn flowi4_init_output(fl4: *mut flowi4, oif: i32, mark: __u32,
    tos: __u8, scope: __u8, proto: __u8, flags: __u8, daddr: __be32,
    saddr: __be32, dport: __be16, sport: __be16, uid: kuid_t) {
    (*fl4).__fl_common.flowic_oif = oif;
    (*fl4).__fl_common.flowic_iif = LOOPBACK_IFINDEX;
    (*fl4).__fl_common.flowic_l3mdev = 0;
    (*fl4).__fl_common.flowic_mark = mark;
    (*fl4).__fl_common.flowic_dscp = inet_dsfield_to_dscp(tos);
    (*fl4).__fl_common.flowic_scope = scope;
    (*fl4).__fl_common.flowic_proto = proto;
    (*fl4).__fl_common.flowic_flags = flags;
    (*fl4).__fl_common.flowic_secid = 0;
    (*fl4).__fl_common.flowic_tun_key.tun_id = 0;
    (*fl4).__fl_common.flowic_uid = uid;
    (*fl4).daddr = daddr;
    (*fl4).saddr = saddr;
    (*fl4).uli.ports.dport = dport;
    (*fl4).uli.ports.sport = sport;
    (*fl4).__fl_common.flowic_multipath_hash = 0;
}

#[inline]
pub unsafe fn flowi4_update_output(fl4: *mut flowi4, oif: i32, daddr: __be32, saddr: __be32) {
    (*fl4).__fl_common.flowic_oif = oif;
    (*fl4).daddr = daddr;
    (*fl4).saddr = saddr;
}

#[repr(C, align(8))]
pub struct flowi6 {
    pub __fl_common: flowi_common,
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub flowlabel: __be32,
    pub uli: flowi_uli,
    pub mp_hash: __u32,
}

#[repr(C, align(8))]
pub struct flowi_u {
    pub __fl_common: flowi_common,
    pub ip4: flowi4,
    pub ip6: flowi6,
}

#[repr(C, align(8))]
pub struct flowi { pub u: flowi_u }

#[inline]
pub unsafe fn flowi4_to_flowi(fl4: *mut flowi4) -> *mut flowi { fl4 as *mut flowi }
#[inline]
pub unsafe fn flowi4_to_flowi_common(fl4: *mut flowi4) -> *mut flowi_common { &mut (*fl4).__fl_common }
#[inline]
pub unsafe fn flowi6_to_flowi(fl6: *mut flowi6) -> *mut flowi { fl6 as *mut flowi }
#[inline]
pub unsafe fn flowi6_to_flowi_common(fl6: *mut flowi6) -> *mut flowi_common { &mut (*fl6).__fl_common }

extern "C" {
    pub fn __get_hash_from_flowi6(fl6: *const flowi6, keys: *mut flow_keys) -> __u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
