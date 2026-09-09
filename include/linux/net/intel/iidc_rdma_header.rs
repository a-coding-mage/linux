/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2021-2025, Intel Corporation. */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iidc_rdma_event_type {
    IIDC_RDMA_EVENT_BEFORE_MTU_CHANGE = 0,
    IIDC_RDMA_EVENT_AFTER_MTU_CHANGE,
    IIDC_RDMA_EVENT_BEFORE_TC_CHANGE,
    IIDC_RDMA_EVENT_AFTER_TC_CHANGE,
    IIDC_RDMA_EVENT_WARN_RESET,
    IIDC_RDMA_EVENT_CRIT_ERR,
    IIDC_RDMA_EVENT_NBITS, // must be last
}

#[repr(C)]
pub struct iidc_rdma_event {
    // DECLARE_BITMAP(type, IIDC_RDMA_EVENT_NBITS)
    pub type_: [usize; 1],
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iidc_rdma_reset_type {
    IIDC_FUNC_RESET = 0,
    IIDC_DEV_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iidc_rdma_protocol {
    IIDC_RDMA_PROTOCOL_IWARP = 1 << 0,
    IIDC_RDMA_PROTOCOL_ROCEV2 = 1 << 1,
}

/* Structure to be populated by core LAN PCI driver */
#[repr(C)]
pub struct iidc_rdma_core_dev_info {
    pub pdev: *mut pci_dev, // PCI device of corresponding to main function
    pub adev: *mut auxiliary_device,
    /* Current active RDMA protocol */
    pub rdma_protocol: iidc_rdma_protocol,
    pub iidc_priv: *mut core::ffi::c_void, // elements unique to each driver
}

/* Structure representing auxiliary driver tailored information about the core
 * PCI dev, each auxiliary driver using the IIDC interface will have an
 * instance of this struct dedicated to it.
 */
#[repr(C)]
pub struct iidc_rdma_core_auxiliary_dev {
    pub adev: auxiliary_device,
    pub cdev_info: *mut iidc_rdma_core_dev_info,
}

/* structure representing the auxiliary driver. This struct is to be
 * allocated and populated by the auxiliary driver's owner. The core PCI
 * driver will access these ops by performing a container_of on the
 * auxiliary_device->dev.driver.
 */
#[repr(C)]
pub struct iidc_rdma_core_auxiliary_drv {
    pub adrv: auxiliary_driver,
    pub event_handler: Option<unsafe extern "C" fn(
        cdev: *mut iidc_rdma_core_dev_info,
        event: *mut iidc_rdma_event,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
