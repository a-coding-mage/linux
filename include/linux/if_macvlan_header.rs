/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux bindings:
// linux/if_link.h, linux/if_vlan.h, linux/list.h, linux/netdevice.h,
// linux/netlink.h, net/netlink.h, and linux/u64_stats_sync.h.

pub const MACVLAN_MC_FILTER_BITS: u32 = 8;
pub const MACVLAN_MC_FILTER_SZ: u32 = 1 << MACVLAN_MC_FILTER_BITS;

#[repr(C)]
pub struct macvlan_port;

#[repr(C)]
pub struct macvlan_dev {
    pub dev: *mut net_device,
    pub list: list_head,
    pub hlist: hlist_node,
    pub port: *mut macvlan_port,
    pub lowerdev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub accel_priv: *mut core::ffi::c_void,
    pub pcpu_stats: *mut vlan_pcpu_stats,

    // DECLARE_BITMAP(mc_filter, MACVLAN_MC_FILTER_SZ)
    pub mc_filter: [core::ffi::c_ulong; 4],

    pub set_features: netdev_features_t,
    pub mode: macvlan_mode,
    pub flags: u16,
    pub macaddr_count: core::ffi::c_uint,
    pub bc_queue_len_req: u32,
    #[cfg(feature = "CONFIG_NET_POLL_CONTROLLER")]
    pub netpoll: *mut netpoll,
}

pub unsafe fn macvlan_count_rx(
    vlan: *const macvlan_dev,
    len: core::ffi::c_uint,
    success: bool,
    multicast: bool,
) {
    if likely(success) {
        let pcpu_stats: *mut vlan_pcpu_stats = get_cpu_ptr((*vlan).pcpu_stats);
        u64_stats_update_begin(&mut (*pcpu_stats).syncp);
        u64_stats_inc(&mut (*pcpu_stats).rx_packets);
        u64_stats_add(&mut (*pcpu_stats).rx_bytes, len);
        if multicast {
            u64_stats_inc(&mut (*pcpu_stats).rx_multicast);
        }
        u64_stats_update_end(&mut (*pcpu_stats).syncp);
        put_cpu_ptr((*vlan).pcpu_stats);
    } else {
        this_cpu_inc((*vlan).pcpu_stats, rx_errors);
    }
}

unsafe extern "C" {
    pub fn macvlan_common_setup(dev: *mut net_device);
    pub fn macvlan_common_newlink(
        dev: *mut net_device,
        params: *mut rtnl_newlink_params,
        extack: *mut netlink_ext_ack,
    ) -> core::ffi::c_int;
    pub fn macvlan_dellink(dev: *mut net_device, head: *mut list_head);
    pub fn macvlan_link_register(ops: *mut rtnl_link_ops) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_MACVLAN")]
pub unsafe fn macvlan_dev_real_dev(dev: *const net_device) -> *mut net_device {
    let macvlan: *mut macvlan_dev = netdev_priv(dev);
    (*macvlan).lowerdev
}

#[cfg(not(feature = "CONFIG_MACVLAN"))]
pub unsafe fn macvlan_dev_real_dev(_dev: *const net_device) -> *mut net_device {
    BUG();
    core::ptr::null_mut()
}

pub unsafe fn macvlan_accel_priv(dev: *mut net_device) -> *mut core::ffi::c_void {
    let macvlan: *mut macvlan_dev = netdev_priv(dev);
    (*macvlan).accel_priv
}

pub unsafe fn macvlan_supports_dest_filter(dev: *mut net_device) -> bool {
    let macvlan: *mut macvlan_dev = netdev_priv(dev);
    (*macvlan).mode == MACVLAN_MODE_PRIVATE
        || (*macvlan).mode == MACVLAN_MODE_VEPA
        || (*macvlan).mode == MACVLAN_MODE_BRIDGE
}

pub unsafe fn macvlan_release_l2fw_offload(dev: *mut net_device) -> core::ffi::c_int {
    let macvlan: *mut macvlan_dev = netdev_priv(dev);
    (*macvlan).accel_priv = core::ptr::null_mut();
    dev_uc_add((*macvlan).lowerdev, (*dev).dev_addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
