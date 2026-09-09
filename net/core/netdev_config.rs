// SPDX-License-Identifier: GPL-2.0-only

// Translated from netdev_config.c. The kernel-provided types and helpers are
// represented here as external declarations; their definitions are supplied by
// the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pp_memory_provider_params {
    pub rx_page_size: u32,
}

#[repr(C)]
pub struct netdev_queue_config {
    pub rx_page_size: u32,
}

#[repr(C)]
pub struct netdev_rx_queue {
    pub mp_params: pp_memory_provider_params,
}

pub type netdev_qcfg_cb = unsafe extern "C" fn(
    dev: *mut net_device,
    qcfg: *mut netdev_queue_config,
    extack: *mut netlink_ext_ack,
) -> i32;

#[repr(C)]
pub struct netdev_queue_mgmt_ops {
    pub ndo_validate_qcfg: Option<netdev_qcfg_cb>,
    pub ndo_default_qcfg: Option<netdev_qcfg_cb>,
}

#[repr(C)]
pub struct net_device {
    pub queue_mgmt_ops: *const netdev_queue_mgmt_ops,
}

unsafe extern "C" {
    fn __netif_get_rx_queue(dev: *mut net_device, rxq_idx: i32) -> *mut netdev_rx_queue;
}

unsafe fn netdev_nop_validate_qcfg(
    _dev: *mut net_device,
    _qcfg: *mut netdev_queue_config,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    0
}

unsafe fn __netdev_queue_config(
    dev: *mut net_device,
    rxq_idx: i32,
    qcfg: *mut netdev_queue_config,
    extack: *mut netlink_ext_ack,
    validate: bool,
) -> i32 {
    let mut validate_cb: netdev_qcfg_cb = netdev_nop_validate_qcfg;
    let mpp: *mut pp_memory_provider_params;
    let err: i32;

    let ops = (*dev).queue_mgmt_ops;
    if validate {
        if let Some(cb) = (*ops).ndo_validate_qcfg {
            validate_cb = cb;
        }
    }

    core::ptr::write_bytes(qcfg as *mut u8, 0, core::mem::size_of::<netdev_queue_config>());

    /* Get defaults from the driver, in case user config not set */
    if let Some(cb) = (*ops).ndo_default_qcfg {
        cb(dev, qcfg, extack);
    }
    err = validate_cb(dev, qcfg, extack);
    if err != 0 {
        return err;
    }

    /* Apply MP overrides */
    mpp = &mut (*__netif_get_rx_queue(dev, rxq_idx)).mp_params;
    if (*mpp).rx_page_size != 0 {
        (*qcfg).rx_page_size = (*mpp).rx_page_size;
    }
    err = validate_cb(dev, qcfg, extack);
    if err != 0 {
        return err;
    }

    0
}

/**
 * netdev_queue_config() - get configuration for a given queue
 * @dev:      net_device instance
 * @rxq_idx:  index of the queue of interest
 * @qcfg: queue configuration struct (output)
 *
 * Render the configuration for a given queue. This helper should be used
 * by drivers which support queue configuration to retrieve config for a
 * particular queue.
 *
 * @qcfg is an output parameter and is always fully initialized by this
 * function. Some values may not be set by the user, drivers may either
 * deal with the "unset" values in @qcfg, or provide the callback
 * to populate defaults in queue_management_ops.
 */
pub unsafe extern "C" fn netdev_queue_config(
    dev: *mut net_device,
    rxq_idx: i32,
    qcfg: *mut netdev_queue_config,
) {
    __netdev_queue_config(dev, rxq_idx, qcfg, core::ptr::null_mut(), false);
}

pub unsafe extern "C" fn netdev_queue_config_validate(
    dev: *mut net_device,
    rxq_idx: i32,
    qcfg: *mut netdev_queue_config,
    extack: *mut netlink_ext_ack,
) -> i32 {
    __netdev_queue_config(dev, rxq_idx, qcfg, extack, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
