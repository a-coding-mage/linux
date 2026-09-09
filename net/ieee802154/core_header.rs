/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by <net/cfg802154.h>. */

#[repr(C)]
pub struct cfg802154_registered_device {
    pub ops: *const cfg802154_ops,
    pub list: list_head,

    /* wpan_phy index, internal only */
    pub wpan_phy_idx: core::ffi::c_int,

    /* also protected by devlist_mtx */
    pub opencount: core::ffi::c_int,
    pub dev_wait: wait_queue_head_t,

    /* protected by RTNL only */
    pub num_running_ifaces: core::ffi::c_int,

    /* associated wpan interfaces, protected by rtnl or RCU */
    pub wpan_dev_list: list_head,
    pub devlist_generation: core::ffi::c_int,
    pub wpan_dev_id: core::ffi::c_int,

    /* Must be last because of the way wpan_phy_priv() is implemented,
     * and it should at least be aligned to NETDEV_ALIGN.
     *
     * C declaration: struct wpan_phy wpan_phy __aligned(NETDEV_ALIGN);
     * The required build-time alignment is supplied by the surrounding
     * dependency environment.
     */
    pub wpan_phy: wpan_phy,
}

#[inline]
pub unsafe fn wpan_phy_to_rdev(wpan_phy: *mut wpan_phy) -> *mut cfg802154_registered_device {
    BUG_ON(wpan_phy.is_null());
    (wpan_phy as *mut u8)
        .sub(core::mem::offset_of!(cfg802154_registered_device, wpan_phy))
        as *mut cfg802154_registered_device
}

extern "C" {
    pub static mut cfg802154_rdev_list: list_head;
    pub static mut cfg802154_rdev_list_generation: core::ffi::c_int;

    pub fn cfg802154_switch_netns(
        rdev: *mut cfg802154_registered_device,
        net: *mut net,
    ) -> core::ffi::c_int;

    /* free object */
    pub fn cfg802154_dev_free(rdev: *mut cfg802154_registered_device);

    pub fn cfg802154_rdev_by_wpan_phy_idx(
        wpan_phy_idx: core::ffi::c_int,
    ) -> *mut cfg802154_registered_device;

    pub fn wpan_phy_idx_to_wpan_phy(wpan_phy_idx: core::ffi::c_int) -> *mut wpan_phy;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
