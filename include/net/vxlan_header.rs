/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/vxlan.h. Kernel dependencies are supplied externally.

pub const IANA_VXLAN_UDP_PORT: u32 = 4789;
pub const IANA_VXLAN_GPE_UDP_PORT: u32 = 4790;

#[repr(C)]
pub struct vxlanhdr {
    pub vx_flags: __be32,
    pub vx_vni: __be32,
}

pub const VXLAN_HF_VNI: __be32 = cpu_to_be32(BIT(27));
pub const VXLAN_N_VID: u32 = 1u32 << 24;
pub const VXLAN_VID_MASK: u32 = VXLAN_N_VID - 1;
pub const VXLAN_VNI_MASK: __be32 = cpu_to_be32(VXLAN_VID_MASK << 8);
pub const VXLAN_HLEN: usize = core::mem::size_of::<struct_udphdr>() + core::mem::size_of::<vxlanhdr>();
pub const VNI_HASH_BITS: u32 = 10;
pub const VNI_HASH_SIZE: u32 = 1 << VNI_HASH_BITS;
pub const FDB_HASH_BITS: u32 = 8;
pub const FDB_HASH_SIZE: u32 = 1 << FDB_HASH_BITS;

pub const VXLAN_HF_RCO: __be32 = cpu_to_be32(BIT(21));
pub const VXLAN_RCO_MASK: __be32 = cpu_to_be32(0x7f);
pub const VXLAN_RCO_UDP: __be32 = cpu_to_be32(0x80);
pub const VXLAN_RCO_SHIFT: u32 = 1;
pub const VXLAN_RCO_SHIFT_MASK: u32 = (1 << VXLAN_RCO_SHIFT) - 1;
pub const VXLAN_MAX_REMCSUM_START: u32 = 0x7f << VXLAN_RCO_SHIFT;

#[repr(C)]
pub struct vxlanhdr_gbp {
    pub vx_flags: u8,
    pub reserved_flags1: u8,
    pub policy_applied: u8,
    pub reserved_flags2: u8,
    pub dont_learn: u8,
    pub reserved_flags3: u8,
    pub policy_id: __be16,
    pub vx_vni: __be32,
}

pub const VXLAN_HF_GBP: __be32 = cpu_to_be32(BIT(31));
pub const VXLAN_GBP_USED_BITS: __be32 = VXLAN_HF_GBP | cpu_to_be32(0xFFFFFF);
pub const VXLAN_GBP_DONT_LEARN: u32 = BIT(6) << 16;
pub const VXLAN_GBP_POLICY_APPLIED: u32 = BIT(3) << 16;
pub const VXLAN_GBP_ID_MASK: u32 = 0xFFFF;
pub const VXLAN_GBP_MASK: u32 = VXLAN_GBP_DONT_LEARN | VXLAN_GBP_POLICY_APPLIED | VXLAN_GBP_ID_MASK;

#[repr(C)]
pub struct vxlanhdr_gpe {
    pub flags: u8,
    pub reserved_flags3: u8,
    pub reserved_flags4: u8,
    pub next_protocol: u8,
    pub vx_vni: __be32,
}

pub const VXLAN_HF_VER: __be32 = cpu_to_be32(BIT(29) | BIT(28));
pub const VXLAN_HF_NP: __be32 = cpu_to_be32(BIT(26));
pub const VXLAN_HF_OAM: __be32 = cpu_to_be32(BIT(24));
pub const VXLAN_GPE_USED_BITS: __be32 = VXLAN_HF_VER | VXLAN_HF_NP | VXLAN_HF_OAM | cpu_to_be32(0xff);

#[repr(C)]
pub struct vxlan_metadata { pub gbp: u32 }

#[repr(C)]
pub struct vxlan_sock {
    pub hlist: hlist_node,
    pub sk: *mut sock,
    pub rcu: rcu_head,
    pub vni_list: [hlist_head; VNI_HASH_SIZE as usize],
    pub refcnt: refcount_t,
    pub flags: u32,
}

#[repr(C)]
pub union vxlan_addr { pub sin: sockaddr_in, pub sin6: sockaddr_in6, pub sa: sockaddr }

#[repr(C)]
pub struct vxlan_rdst {
    pub remote_ip: vxlan_addr,
    pub remote_port: __be16,
    pub offloaded: u8,
    pub remote_vni: __be32,
    pub remote_ifindex: u32,
    pub remote_dev: *mut net_device,
    pub list: list_head,
    pub rcu: rcu_head,
    pub dst_cache: dst_cache,
}

#[repr(C)]
pub struct vxlan_config {
    pub remote_ip: vxlan_addr, pub saddr: vxlan_addr, pub vni: __be32,
    pub remote_ifindex: c_int, pub mtu: c_int, pub dst_port: __be16,
    pub port_min: u16, pub port_max: u16, pub tos: u8, pub ttl: u8,
    pub label: __be32, pub label_policy: ifla_vxlan_label_policy,
    pub flags: u32, pub age_interval: c_ulong, pub addrmax: c_uint,
    pub no_share: bool, pub df: ifla_vxlan_df, pub reserved_bits: vxlanhdr,
}

pub const VXLAN_F_LEARN: u32 = 0x01;
pub const VXLAN_F_PROXY: u32 = 0x02;
pub const VXLAN_F_RSC: u32 = 0x04;
pub const VXLAN_F_L2MISS: u32 = 0x08;
pub const VXLAN_F_L3MISS: u32 = 0x10;
pub const VXLAN_F_IPV6: u32 = 0x20;
pub const VXLAN_F_UDP_ZERO_CSUM_TX: u32 = 0x40;
pub const VXLAN_F_UDP_ZERO_CSUM6_TX: u32 = 0x80;
pub const VXLAN_F_UDP_ZERO_CSUM6_RX: u32 = 0x100;
pub const VXLAN_F_REMCSUM_TX: u32 = 0x200;
pub const VXLAN_F_REMCSUM_RX: u32 = 0x400;
pub const VXLAN_F_GBP: u32 = 0x800;
pub const VXLAN_F_REMCSUM_NOPARTIAL: u32 = 0x1000;
pub const VXLAN_F_COLLECT_METADATA: u32 = 0x2000;
pub const VXLAN_F_GPE: u32 = 0x4000;
pub const VXLAN_F_IPV6_LINKLOCAL: u32 = 0x8000;
pub const VXLAN_F_TTL_INHERIT: u32 = 0x10000;
pub const VXLAN_F_VNIFILTER: u32 = 0x20000;
pub const VXLAN_F_MDB: u32 = 0x40000;
pub const VXLAN_F_LOCALBYPASS: u32 = 0x80000;
pub const VXLAN_F_MC_ROUTE: u32 = 0x100000;
pub const VXLAN_F_RCV_FLAGS: u32 = VXLAN_F_GBP | VXLAN_F_GPE | VXLAN_F_UDP_ZERO_CSUM6_RX | VXLAN_F_REMCSUM_RX | VXLAN_F_REMCSUM_NOPARTIAL | VXLAN_F_COLLECT_METADATA | VXLAN_F_VNIFILTER;
pub const VXLAN_F_ALLOWED_GPE: u32 = VXLAN_F_GPE | VXLAN_F_IPV6 | VXLAN_F_IPV6_LINKLOCAL | VXLAN_F_UDP_ZERO_CSUM_TX | VXLAN_F_UDP_ZERO_CSUM6_TX | VXLAN_F_UDP_ZERO_CSUM6_RX | VXLAN_F_COLLECT_METADATA | VXLAN_F_VNIFILTER | VXLAN_F_LOCALBYPASS | VXLAN_F_MC_ROUTE;

#[repr(C)] pub struct vxlan_vni_stats { pub rx_packets:u64, pub rx_bytes:u64, pub rx_drops:u64, pub rx_errors:u64, pub tx_packets:u64, pub tx_bytes:u64, pub tx_drops:u64, pub tx_errors:u64 }
#[repr(C)] pub struct vxlan_vni_stats_pcpu { pub stats: vxlan_vni_stats, pub syncp: u64_stats_sync }
#[repr(C)] pub struct vxlan_dev_node { pub hlist: hlist_node, pub vxlan: *mut vxlan_dev }
#[repr(C)] pub struct vxlan_vni_node { pub vnode: rhash_head, pub hlist4: vxlan_dev_node, pub hlist6: vxlan_dev_node, pub vlist: list_head, pub vni: __be32, pub remote_ip: vxlan_addr, pub stats: *mut vxlan_vni_stats_pcpu, pub rcu: rcu_head }
#[repr(C)] pub struct vxlan_vni_group { pub vni_hash: rhashtable, pub vni_list: list_head, pub num_vnis: u32 }

#[repr(C)] pub struct vxlan_dev {
    pub hlist4: vxlan_dev_node, pub hlist6: vxlan_dev_node, pub next: list_head,
    pub vn4_sock: *mut vxlan_sock, pub vn6_sock: *mut vxlan_sock, pub dev: *mut net_device,
    pub net: *mut net, pub default_dst: vxlan_rdst, pub age_timer: timer_list,
    pub hash_lock: spinlock_t, pub addrcnt: c_uint, pub gro_cells: gro_cells,
    pub cfg: vxlan_config, pub vnigrp: *mut vxlan_vni_group, pub fdb_hash_tbl: rhashtable,
    pub mdb_tbl: rhashtable, pub fdb_list: hlist_head, pub mdb_list: hlist_head, pub mdb_seq: c_uint,
}

pub const VXLAN_VNI_STATS_RX: u32 = 0;
pub const VXLAN_VNI_STATS_RX_DROPS: u32 = 1;
pub const VXLAN_VNI_STATS_RX_ERRORS: u32 = 2;
pub const VXLAN_VNI_STATS_TX: u32 = 3;
pub const VXLAN_VNI_STATS_TX_DROPS: u32 = 4;
pub const VXLAN_VNI_STATS_TX_ERRORS: u32 = 5;

#[inline] pub unsafe fn vxlan_headroom(flags: u32) -> usize {
    let ip = if flags & VXLAN_F_IPV6 != 0 { core::mem::size_of::<ipv6hdr>() } else { core::mem::size_of::<iphdr>() };
    ip + core::mem::size_of::<struct_udphdr>() + core::mem::size_of::<vxlanhdr>() + if flags & VXLAN_F_GPE != 0 { 0 } else { ETH_HLEN as usize }
}
#[inline] pub unsafe fn vxlan_hdr(skb: *mut sk_buff) -> *mut vxlanhdr { (udp_hdr(skb).add(1)) as *mut vxlanhdr }
#[inline] pub fn vxlan_vni(vni_field: __be32) -> __be32 { ((vni_field & VXLAN_VNI_MASK) << 8) as __be32 }
#[inline] pub fn vxlan_vni_field(vni: __be32) -> __be32 { (vni >> 8) as __be32 }
#[inline] pub fn vxlan_rco_start(vni_field: __be32) -> usize { ((be32_to_cpu(vni_field & VXLAN_RCO_MASK)) << VXLAN_RCO_SHIFT) as usize }
#[inline] pub unsafe fn vxlan_rco_offset(vni_field: __be32) -> usize { if vni_field & VXLAN_RCO_UDP != 0 { core::mem::offset_of!(struct_udphdr, check) } else { core::mem::offset_of!(tcphdr, check) } }
#[inline] pub unsafe fn vxlan_compute_rco(start: c_uint, offset: c_uint) -> __be32 {
    let mut vni_field = cpu_to_be32(start >> VXLAN_RCO_SHIFT);
    if offset as usize == core::mem::offset_of!(struct_udphdr, check) { vni_field |= VXLAN_RCO_UDP; }
    vni_field
}
#[inline] pub unsafe fn vxlan_get_sk_family(vs: *mut vxlan_sock) -> c_ushort { (*(*vs).sk).sk_family }

#[inline] pub unsafe fn vxlan_addr_any(ipa: *const vxlan_addr) -> bool {
    if (*ipa).sa.sa_family == AF_INET6 { ipv6_addr_any(&(*ipa).sin6.sin6_addr) } else { (*ipa).sin.sin_addr.s_addr == htonl(INADDR_ANY) }
}
#[inline] pub unsafe fn vxlan_addr_multicast(ipa: *const vxlan_addr) -> bool {
    if (*ipa).sa.sa_family == AF_INET6 { ipv6_addr_is_multicast(&(*ipa).sin6.sin6_addr) } else { ipv4_is_multicast((*ipa).sin.sin_addr.s_addr) }
}
#[inline] pub unsafe fn netif_is_vxlan(dev: *const net_device) -> bool { !(*dev).rtnl_link_ops.is_null() && !strcmp((*(*dev).rtnl_link_ops).kind, b"vxlan\0".as_ptr() as *const i8) }

#[repr(C)] pub struct switchdev_notifier_vxlan_fdb_info {
    pub info: switchdev_notifier_info, pub remote_ip: vxlan_addr, pub remote_port: __be16,
    pub remote_vni: __be32, pub remote_ifindex: u32, pub eth_addr: [u8; ETH_ALEN as usize],
    pub vni: __be32, pub offloaded: bool, pub added_by_user: bool,
}

#[cfg(feature = "CONFIG_VXLAN")]
extern "C" {
    pub fn vxlan_fdb_find_uc(dev: *mut net_device, mac: *const u8, vni: __be32, fdb_info: *mut switchdev_notifier_vxlan_fdb_info) -> c_int;
    pub fn vxlan_fdb_replay(dev: *const net_device, vni: __be32, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> c_int;
    pub fn vxlan_fdb_clear_offload(dev: *const net_device, vni: __be32);
}

#[inline] pub unsafe fn vxlan_build_gbp_hdr(vxh: *mut vxlanhdr, md: *const vxlan_metadata) {
    if (*md).gbp == 0 { return; }
    let gbp = vxh as *mut vxlanhdr_gbp;
    (*vxh).vx_flags |= VXLAN_HF_GBP;
    if (*md).gbp & VXLAN_GBP_DONT_LEARN != 0 { (*gbp).dont_learn = 1; }
    if (*md).gbp & VXLAN_GBP_POLICY_APPLIED != 0 { (*gbp).policy_applied = 1; }
    (*gbp).policy_id = htons(((*md).gbp & VXLAN_GBP_ID_MASK) as __be16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
