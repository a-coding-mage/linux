// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation.

unsafe fn __netdev_queue_get_dma_dev(
    dev: *mut net_device,
    idx: c_uint,
) -> *mut device {
    let queue_ops = (*dev).queue_mgmt_ops;
    let dma_dev: *mut device;

    if !queue_ops.is_null() && !(*queue_ops).ndo_queue_get_dma_dev.is_null() {
        dma_dev = ((*queue_ops).ndo_queue_get_dma_dev)(dev, idx);
    } else {
        dma_dev = (*dev).dev.parent;
    }

    if !dma_dev.is_null() && !(*dma_dev).dma_mask.is_null() {
        dma_dev
    } else {
        core::ptr::null_mut()
    }
}

/**
 * netdev_queue_get_dma_dev() - get dma device for zero-copy operations
 * @dev: net_device
 * @idx: queue index
 * @type: queue type (RX or TX)
 *
 * Get dma device for zero-copy operations to be used for this queue. If
 * the queue is an RX queue leased from a physical queue, we retrieve the
 * physical queue's dma device. When the dma device is not available or
 * valid, the function will return NULL.
 *
 * Return: Device or NULL on error
 */
pub unsafe fn netdev_queue_get_dma_dev(
    dev: *mut net_device,
    mut idx: c_uint,
    type_: netdev_queue_type,
) -> *mut device {
    let hw_rxq: *mut netdev_rx_queue;
    let dma_dev: *mut device;

    netdev_assert_locked(dev);

    /* Only RX side supports queue leasing today. */
    if type_ != NETDEV_QUEUE_TYPE_RX || !netif_rxq_is_leased(dev, idx) {
        return __netdev_queue_get_dma_dev(dev, idx);
    }
    if !netif_is_queue_leasee(dev) {
        return core::ptr::null_mut();
    }

    hw_rxq = (*__netif_get_rx_queue(dev, idx)).lease;

    netdev_lock((*hw_rxq).dev);
    idx = get_netdev_rx_queue_index(hw_rxq);
    dma_dev = __netdev_queue_get_dma_dev((*hw_rxq).dev, idx);
    netdev_unlock((*hw_rxq).dev);

    dma_dev
}

pub unsafe fn netdev_can_create_queue(
    dev: *const net_device,
    extack: *mut netlink_ext_ack,
) -> bool {
    if !(*dev).dev.parent.is_null() {
        NL_SET_ERR_MSG(extack, "Device is not a virtual device");
        return false;
    }
    if (*dev).queue_mgmt_ops.is_null()
        || (*(*dev).queue_mgmt_ops).ndo_queue_create.is_null()
    {
        NL_SET_ERR_MSG(extack, "Device does not support queue creation");
        return false;
    }
    if (*dev).real_num_rx_queues < 1 || (*dev).real_num_tx_queues < 1 {
        NL_SET_ERR_MSG(extack, "Device must have at least one real queue");
        return false;
    }
    true
}

pub unsafe fn netdev_can_lease_queue(
    dev: *const net_device,
    extack: *mut netlink_ext_ack,
) -> bool {
    if (*dev).dev.parent.is_null() {
        NL_SET_ERR_MSG(extack, "Lease device is a virtual device");
        return false;
    }
    if !netif_device_present(dev) {
        NL_SET_ERR_MSG(extack, "Lease device has been removed from the system");
        return false;
    }
    if (*dev).queue_mgmt_ops.is_null() {
        NL_SET_ERR_MSG(extack, "Lease device does not support queue management operations");
        return false;
    }
    true
}

pub unsafe fn netdev_queue_busy(
    dev: *mut net_device,
    idx: c_uint,
    type_: netdev_queue_type,
    extack: *mut netlink_ext_ack,
) -> bool {
    if !xsk_get_pool_from_qid(dev, idx).is_null() {
        NL_SET_ERR_MSG(extack, "Device queue in use by AF_XDP");
        return true;
    }
    if type_ == NETDEV_QUEUE_TYPE_TX {
        return false;
    }
    if netif_rxq_is_leased(dev, idx) {
        NL_SET_ERR_MSG(extack, "Device queue in use due to queue leasing");
        return true;
    }
    if netif_rxq_has_mp(dev, idx) {
        NL_SET_ERR_MSG(extack, "Device queue in use by memory provider");
        return true;
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
