/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding kernel networking declarations. */

/* This structure contains an instance of an RX queue. */
#[repr(C)]
pub struct netdev_rx_queue {
    pub xdp_rxq: xdp_rxq_info,
    #[cfg(feature = "CONFIG_RPS")]
    pub rps_map: *mut rps_map,
    #[cfg(feature = "CONFIG_RPS")]
    pub rps_flow_table: rps_tag_ptr,
    pub kobj: kobject,
    pub groups: *const *const attribute_group,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,

    /* All fields below are "ops protected",
     * see comment about net_device::lock
     */
    #[cfg(feature = "CONFIG_XDP_SOCKETS")]
    pub pool: *mut xsk_buff_pool,
    pub napi: *mut napi_struct,
    pub qcfg: netdev_queue_config,
    pub mp_params: pp_memory_provider_params,

    /* If a queue is leased, then the lease pointer is always
     * valid. From the physical device it points to the virtual
     * queue, and from the virtual device it points to the
     * physical queue.
     */
    pub lease: *mut netdev_rx_queue,
    pub lease_tracker: netdevice_tracker,
}

/*
 * RX queue sysfs structures and functions.
 */
#[repr(C)]
pub struct rx_queue_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(queue: *mut netdev_rx_queue, buf: *mut c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(
        queue: *mut netdev_rx_queue,
        buf: *const c_char,
        len: size_t,
    ) -> ssize_t>,
}

#[inline]
pub unsafe fn __netif_get_rx_queue(dev: *mut net_device, rxq: c_uint) -> *mut netdev_rx_queue {
    (*dev)._rx.add(rxq as usize)
}

#[inline]
pub unsafe fn get_netdev_rx_queue_index(queue: *mut netdev_rx_queue) -> c_uint {
    let dev = (*queue).dev;
    let index = queue.offset_from((*dev)._rx);

    if index >= (*dev).num_rx_queues as isize {
        panic!("BUG_ON(index >= dev->num_rx_queues)");
    }
    index as c_uint
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netif_lease_dir {
    NETIF_VIRT_TO_PHYS,
    NETIF_PHYS_TO_VIRT,
}

unsafe extern "C" {
    pub fn __netif_get_rx_queue_lease(
        dev: *mut *mut net_device,
        rxq: *mut c_uint,
        dir: netif_lease_dir,
    ) -> *mut netdev_rx_queue;

    pub fn netdev_rx_queue_restart(dev: *mut net_device, rxq: c_uint) -> c_int;
    pub fn netdev_rx_queue_lease(
        rxq_dst: *mut netdev_rx_queue,
        rxq_src: *mut netdev_rx_queue,
    );
    pub fn netdev_rx_queue_unlease(
        rxq_dst: *mut netdev_rx_queue,
        rxq_src: *mut netdev_rx_queue,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
