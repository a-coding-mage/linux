/* SPDX-License-Identifier: GPL-2.0 */

// Translated from vlan.h. External kernel types, constants, and functions are
// supplied by other translation units.

pub const VLAN_GROUP_ARRAY_SPLIT_PARTS: usize = 8;
pub const VLAN_GROUP_ARRAY_PART_LEN: usize = VLAN_N_VID / VLAN_GROUP_ARRAY_SPLIT_PARTS;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vlan_protos {
    VLAN_PROTO_8021Q = 0,
    VLAN_PROTO_8021AD,
    VLAN_PROTO_NUM,
}

#[repr(C)]
pub struct vlan_group {
    pub nr_vlan_devs: ::core::ffi::c_uint,
    pub hlist: hlist_node, // linked list
    pub vlan_devices_arrays:
        [[*mut *mut net_device; VLAN_GROUP_ARRAY_SPLIT_PARTS]; VLAN_PROTO_NUM as usize],
}

#[repr(C)]
pub struct vlan_info {
    pub real_dev: *mut net_device, // The ethernet(like) device the vlan is attached to.
    pub grp: vlan_group,
    pub vid_list: list_head,
    pub nr_vids: ::core::ffi::c_uint,
    pub auto_vid0: bool,
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn vlan_proto_idx(proto: __be16) -> ::core::ffi::c_int {
    match htons(proto) {
        ETH_P_8021Q => VLAN_PROTO_8021Q as ::core::ffi::c_int,
        ETH_P_8021AD => VLAN_PROTO_8021AD as ::core::ffi::c_int,
        _ => {
            WARN(1, "invalid VLAN protocol: 0x%04x\n", ntohs(proto));
            -EINVAL
        }
    }
}

#[inline]
pub unsafe fn __vlan_group_get_device(
    vg: *mut vlan_group,
    pidx: ::core::ffi::c_uint,
    vlan_id: u16,
) -> *mut net_device {
    let array = (*vg).vlan_devices_arrays[pidx as usize]
        [(vlan_id as usize) / VLAN_GROUP_ARRAY_PART_LEN];

    // paired with smp_wmb() in vlan_group_prealloc_vid()
    smp_rmb();

    if !array.is_null() {
        *array.add((vlan_id as usize) % VLAN_GROUP_ARRAY_PART_LEN)
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn vlan_group_get_device(
    vg: *mut vlan_group,
    vlan_proto: __be16,
    vlan_id: u16,
) -> *mut net_device {
    let pidx = vlan_proto_idx(vlan_proto);
    if pidx < 0 {
        return core::ptr::null_mut();
    }
    __vlan_group_get_device(vg, pidx as ::core::ffi::c_uint, vlan_id)
}

#[inline]
pub unsafe fn vlan_group_set_device(
    vg: *mut vlan_group,
    vlan_proto: __be16,
    vlan_id: u16,
    dev: *mut net_device,
) {
    let pidx = vlan_proto_idx(vlan_proto);
    if vg.is_null() || pidx < 0 {
        return;
    }
    let array = (*vg).vlan_devices_arrays[pidx as usize]
        [(vlan_id as usize) / VLAN_GROUP_ARRAY_PART_LEN];
    *array.add((vlan_id as usize) % VLAN_GROUP_ARRAY_PART_LEN) = dev;
}

// Must be invoked with rcu_read_lock or with RTNL.
#[inline]
pub unsafe fn vlan_find_dev(
    real_dev: *mut net_device,
    vlan_proto: __be16,
    vlan_id: u16,
) -> *mut net_device {
    let vlan_info = rcu_dereference_rtnl((*real_dev).vlan_info);
    if !vlan_info.is_null() {
        return vlan_group_get_device(&mut (*vlan_info).grp, vlan_proto, vlan_id);
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn vlan_tnl_features(real_dev: *mut net_device) -> netdev_features_t {
    let ret = (*real_dev).hw_enc_features
        & (NETIF_F_CSUM_MASK | NETIF_F_GSO_SOFTWARE | NETIF_F_GSO_ENCAP_ALL);
    if (ret & NETIF_F_GSO_ENCAP_ALL) != 0 && (ret & NETIF_F_CSUM_MASK) != 0 {
        return (ret & !NETIF_F_CSUM_MASK) | NETIF_F_HW_CSUM;
    }
    0
}

pub unsafe fn vlan_group_for_each_dev(
    grp: *mut vlan_group,
    i: &mut ::core::ffi::c_uint,
    dev: &mut *mut net_device,
) {
    *i = 0;
    while *i < VLAN_PROTO_NUM as ::core::ffi::c_uint * VLAN_N_VID as ::core::ffi::c_uint {
        *dev = __vlan_group_get_device(
            grp,
            (*i / VLAN_N_VID as ::core::ffi::c_uint),
            (*i % VLAN_N_VID as ::core::ffi::c_uint) as u16,
        );
        if !(*dev).is_null() {
            break;
        }
        *i += 1;
    }
}

extern "C" {
    pub fn vlan_filter_push_vids(vlan_info: *mut vlan_info, proto: __be16) -> ::core::ffi::c_int;
    pub fn vlan_filter_drop_vids(vlan_info: *mut vlan_info, proto: __be16);

    pub fn vlan_stacked_transfer_operstate(rootdev: *const net_device, dev: *mut net_device, vlan: *mut vlan_dev_priv);
    pub fn vlan_dev_set_ingress_priority(dev: *const net_device, skb_prio: u32, vlan_prio: u16);
    pub fn vlan_dev_set_egress_priority(dev: *const net_device, skb_prio: u32, vlan_prio: u16) -> ::core::ffi::c_int;
    pub fn vlan_dev_free_egress_priority(dev: *const net_device);
    pub fn vlan_dev_change_flags(dev: *const net_device, flag: u32, mask: u32) -> ::core::ffi::c_int;
    pub fn vlan_dev_get_realdev_name(dev: *const net_device, result: *mut ::core::ffi::c_char, size: usize);
    pub fn vlan_check_real_dev(real_dev: *mut net_device, protocol: __be16, vlan_id: u16, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn vlan_setup(dev: *mut net_device);
    pub fn register_vlan_dev(dev: *mut net_device, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn unregister_vlan_dev(dev: *mut net_device, head: *mut list_head);
    pub fn vlan_dev_inherit_address(dev: *mut net_device, real_dev: *mut net_device) -> bool;
}

#[inline]
pub unsafe fn vlan_get_ingress_priority(dev: *mut net_device, vlan_tci: u16) -> u32 {
    let vip = vlan_dev_priv(dev);
    (*vip).ingress_priority_map[((vlan_tci >> VLAN_PRIO_SHIFT) & 0x7) as usize]
}

#[cfg(CONFIG_VLAN_8021Q_GVRP)]
extern "C" {
    pub fn vlan_gvrp_request_join(dev: *const net_device) -> ::core::ffi::c_int;
    pub fn vlan_gvrp_request_leave(dev: *const net_device);
    pub fn vlan_gvrp_init_applicant(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn vlan_gvrp_uninit_applicant(dev: *mut net_device);
    pub fn vlan_gvrp_init() -> ::core::ffi::c_int;
    pub fn vlan_gvrp_uninit();
}
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_request_join(_: *const net_device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_request_leave(_: *const net_device) {}
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_init_applicant(_: *mut net_device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_uninit_applicant(_: *mut net_device) {}
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_init() -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_GVRP))]
#[inline] pub unsafe fn vlan_gvrp_uninit() {}

extern "C" {
    pub static vlan_fullname: ::core::ffi::c_char;
    pub static vlan_version: ::core::ffi::c_char;
    pub fn vlan_netlink_init() -> ::core::ffi::c_int;
    pub fn vlan_netlink_fini();
    pub static mut vlan_link_ops: rtnl_link_ops;
    pub static mut vlan_net_id: ::core::ffi::c_uint;
}

#[repr(C)]
pub struct vlan_net {
    // /proc/net/vlan
    pub proc_vlan_dir: *mut proc_dir_entry,
    // /proc/net/vlan/config
    pub proc_vlan_conf: *mut proc_dir_entry,
    // Determines interface naming scheme.
    pub name_type: u16,
}

pub const VLAN_WORK_LINK_STATE: u32 = BIT(0); // sync up/down with real_dev
pub const VLAN_WORK_MTU: u32 = BIT(1); // clamp mtu to real_dev's
pub const VLAN_WORK_FEATURES: u32 = BIT(2); // re-inherit real_dev features

#[cfg(CONFIG_VLAN_8021Q_MVRP)]
extern "C" {
    pub fn vlan_mvrp_request_join(dev: *const net_device) -> ::core::ffi::c_int;
    pub fn vlan_mvrp_request_leave(dev: *const net_device);
    pub fn vlan_mvrp_init_applicant(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn vlan_mvrp_uninit_applicant(dev: *mut net_device);
    pub fn vlan_mvrp_init() -> ::core::ffi::c_int;
    pub fn vlan_mvrp_uninit();
}
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_request_join(_: *const net_device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_request_leave(_: *const net_device) {}
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_init_applicant(_: *mut net_device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_uninit_applicant(_: *mut net_device) {}
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_init() -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_VLAN_8021Q_MVRP))]
#[inline] pub unsafe fn vlan_mvrp_uninit() {}

#[repr(C)]
pub struct proc_dir_entry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
