/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/netdevice.h
// Dependency: net/6lowpan.h

/* caller need to be sure it's dev->type is ARPHRD_6LOWPAN */
#[inline]
pub unsafe fn lowpan_is_ll(dev: *const net_device, lltype: lowpan_lltypes) -> bool {
    (*lowpan_dev(dev)).lltype == lltype
}

extern "C" {
    pub static lowpan_ndisc_ops: ndisc_ops;

    pub fn addrconf_ifid_802154_6lowpan(eui: *mut u8, dev: *mut net_device) -> i32;
}

#[cfg(CONFIG_6LOWPAN_DEBUGFS)]
extern "C" {
    pub fn lowpan_dev_debugfs_init(dev: *mut net_device);
    pub fn lowpan_dev_debugfs_exit(dev: *mut net_device);

    pub fn lowpan_debugfs_init();
    pub fn lowpan_debugfs_exit();
}

#[cfg(not(CONFIG_6LOWPAN_DEBUGFS))]
#[inline]
pub unsafe fn lowpan_dev_debugfs_init(_dev: *mut net_device) {}

#[cfg(not(CONFIG_6LOWPAN_DEBUGFS))]
#[inline]
pub unsafe fn lowpan_dev_debugfs_exit(_dev: *mut net_device) {}

#[cfg(not(CONFIG_6LOWPAN_DEBUGFS))]
#[inline]
pub unsafe fn lowpan_debugfs_init() {}

#[cfg(not(CONFIG_6LOWPAN_DEBUGFS))]
#[inline]
pub unsafe fn lowpan_debugfs_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
