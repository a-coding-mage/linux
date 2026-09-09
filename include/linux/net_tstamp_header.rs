/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding UAPI and kernel headers.

pub const SOF_TIMESTAMPING_SOFTWARE_MASK: u32 =
    SOF_TIMESTAMPING_RX_SOFTWARE | SOF_TIMESTAMPING_TX_SOFTWARE | SOF_TIMESTAMPING_SOFTWARE;

pub const SOF_TIMESTAMPING_HARDWARE_MASK: u32 =
    SOF_TIMESTAMPING_RX_HARDWARE | SOF_TIMESTAMPING_TX_HARDWARE | SOF_TIMESTAMPING_RAW_HARDWARE;

/**
 * struct hwtstamp_provider_desc - hwtstamp provider description
 *
 * @index: index of the hwtstamp provider.
 * @qualifier: hwtstamp provider qualifier.
 */
#[repr(C)]
pub struct hwtstamp_provider_desc {
    pub index: ::core::ffi::c_int,
    pub qualifier: hwtstamp_provider_qualifier,
}

/**
 * struct hwtstamp_provider - hwtstamp provider object
 *
 * @rcu_head: RCU callback used to free the struct.
 * @source: source of the hwtstamp provider.
 * @phydev: pointer of the phydev source in case a PTP coming from phylib
 * @desc: hwtstamp provider description.
 */
#[repr(C)]
pub struct hwtstamp_provider {
    pub rcu_head: rcu_head,
    pub source: hwtstamp_source,
    pub phydev: *mut phy_device,
    pub desc: hwtstamp_provider_desc,
}

/**
 * struct kernel_hwtstamp_config - Kernel copy of struct hwtstamp_config
 *
 * @flags: see struct hwtstamp_config
 * @tx_type: see struct hwtstamp_config
 * @rx_filter: see struct hwtstamp_config
 * @ifr: pointer to ifreq structure from the original ioctl request, to pass to
 *	a legacy implementation of a lower driver
 * @copied_to_user: request was passed to a legacy implementation which already
 * copied the ioctl request back to user space
 * @source: indication whether timestamps should come from the netdev or from
 * an attached phylib PHY
 * @qualifier: qualifier of the hwtstamp provider
 *
 * Prefer using this structure for in-kernel processing of hardware
 * timestamping configuration, over the inextensible struct hwtstamp_config
 * exposed to the %SIOCGHWTSTAMP and %SIOCSHWTSTAMP ioctl UAPI.
 */
#[repr(C)]
pub struct kernel_hwtstamp_config {
    pub flags: ::core::ffi::c_int,
    pub tx_type: ::core::ffi::c_int,
    pub rx_filter: ::core::ffi::c_int,
    pub ifr: *mut ifreq,
    pub copied_to_user: bool,
    pub source: hwtstamp_source,
    pub qualifier: hwtstamp_provider_qualifier,
}

pub unsafe fn hwtstamp_config_to_kernel(
    kernel_cfg: *mut kernel_hwtstamp_config,
    cfg: *const hwtstamp_config,
) {
    (*kernel_cfg).flags = (*cfg).flags;
    (*kernel_cfg).tx_type = (*cfg).tx_type;
    (*kernel_cfg).rx_filter = (*cfg).rx_filter;
}

pub unsafe fn hwtstamp_config_from_kernel(
    cfg: *mut hwtstamp_config,
    kernel_cfg: *const kernel_hwtstamp_config,
) {
    (*cfg).flags = (*kernel_cfg).flags;
    (*cfg).tx_type = (*kernel_cfg).tx_type;
    (*cfg).rx_filter = (*kernel_cfg).rx_filter;
}

pub unsafe fn kernel_hwtstamp_config_changed(
    a: *const kernel_hwtstamp_config,
    b: *const kernel_hwtstamp_config,
) -> bool {
    (*a).flags != (*b).flags
        || (*a).tx_type != (*b).tx_type
        || (*a).rx_filter != (*b).rx_filter
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
