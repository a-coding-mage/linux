/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

/*
 * The NCSI device states seen from external. More NCSI device states are
 * only visible internally (in net/ncsi/internal.h). When the NCSI device
 * is registered, it's in ncsi_dev_state_registered state. The state
 * ncsi_dev_state_start is used to drive to choose active package and
 * channel. After that, its state is changed to ncsi_dev_state_functional.
 *
 * The state ncsi_dev_state_stop helps to shut down the currently active
 * package and channel while ncsi_dev_state_config helps to reconfigure
 * them.
 */
pub const ncsi_dev_state_registered: i32 = 0x0000;
pub const ncsi_dev_state_functional: i32 = 0x0100;
pub const ncsi_dev_state_probe: i32 = 0x0200;
pub const ncsi_dev_state_config: i32 = 0x0300;
pub const ncsi_dev_state_suspend: i32 = 0x0400;

#[repr(C)]
pub struct ncsi_dev {
    pub state: ::core::ffi::c_int,
    pub link_up: ::core::ffi::c_int,
    pub dev: *mut net_device,
    pub handler: Option<unsafe extern "C" fn(ndev: *mut ncsi_dev)>,
}

/* These names are provided by the surrounding translation. */
pub type __be16 = u16;
pub type u16 = u16;
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_NET_NCSI")]
unsafe extern "C" {
    pub fn ncsi_vlan_rx_add_vid(dev: *mut net_device, proto: __be16, vid: u16) -> i32;
    pub fn ncsi_vlan_rx_kill_vid(dev: *mut net_device, proto: __be16, vid: u16) -> i32;
    pub fn ncsi_register_dev(
        dev: *mut net_device,
        notifier: Option<unsafe extern "C" fn(nd: *mut ncsi_dev)>,
    ) -> *mut ncsi_dev;
    pub fn ncsi_start_dev(nd: *mut ncsi_dev) -> i32;
    pub fn ncsi_stop_dev(nd: *mut ncsi_dev);
    pub fn ncsi_unregister_dev(nd: *mut ncsi_dev);
}

/* !CONFIG_NET_NCSI: errno values are supplied by the kernel environment. */
#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_vlan_rx_add_vid(_dev: *mut net_device, _proto: __be16, _vid: u16) -> i32 {
    -EINVAL
}

#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_vlan_rx_kill_vid(_dev: *mut net_device, _proto: __be16, _vid: u16) -> i32 {
    -EINVAL
}

#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_register_dev(
    _dev: *mut net_device,
    _notifier: Option<unsafe extern "C" fn(nd: *mut ncsi_dev)>,
) -> *mut ncsi_dev {
    ::core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_start_dev(_nd: *mut ncsi_dev) -> i32 {
    -ENOTTY
}

#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_stop_dev(_nd: *mut ncsi_dev) {}

#[cfg(not(feature = "CONFIG_NET_NCSI"))]
pub unsafe fn ncsi_unregister_dev(_nd: *mut ncsi_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
