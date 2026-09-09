/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2025 Intel Corporation. */

/* Dependency: linux/auxiliary_bus.h and related kernel declarations. */

/* struct to be populated by core LAN PCI driver */
#[repr(C)]
pub struct iidc_rdma_vport_dev_info {
    pub adev: *mut auxiliary_device,
    pub core_adev: *mut auxiliary_device,
    pub netdev: *mut net_device,
    pub vport_id: u16,
}

#[repr(C)]
pub struct iidc_rdma_vport_auxiliary_dev {
    pub adev: auxiliary_device,
    pub vdev_info: *mut iidc_rdma_vport_dev_info,
}

#[repr(C)]
pub struct iidc_rdma_vport_auxiliary_drv {
    pub adrv: auxiliary_driver,
    pub event_handler:
        Option<unsafe extern "C" fn(vdev: *mut iidc_rdma_vport_dev_info,
                                     event: *mut iidc_rdma_event)>,
}

/* struct to be populated by core LAN PCI driver */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iidc_function_type {
    IIDC_FUNCTION_TYPE_PF,
    IIDC_FUNCTION_TYPE_VF,
}

#[repr(C)]
pub struct iidc_rdma_lan_mapped_mem_region {
    pub region_addr: *mut u8,
    pub size: u64,
    pub start_offset: u64,
}

#[repr(C)]
pub struct iidc_rdma_priv_dev_info {
    pub msix_entries: *mut msix_entry,
    pub msix_count: u16, /* How many vectors are reserved for this device */
    pub ftype: iidc_function_type,
    pub num_memory_regions: u16,
    pub mapped_mem_regions: *mut iidc_rdma_lan_mapped_mem_region,
}

extern "C" {
    pub fn idpf_idc_vport_dev_ctrl(
        cdev_info: *mut iidc_rdma_core_dev_info,
        up: bool,
    ) -> i32;
    pub fn idpf_idc_request_reset(
        cdev_info: *mut iidc_rdma_core_dev_info,
        reset_type: iidc_rdma_reset_type,
    ) -> i32;
    pub fn idpf_idc_rdma_vc_send_sync(
        cdev_info: *mut iidc_rdma_core_dev_info,
        send_msg: *mut u8,
        msg_size: u16,
        recv_msg: *mut u8,
        recv_len: *mut u16,
    ) -> i32;
}

/* External declarations supplied by related kernel headers. */
pub enum auxiliary_device {}
pub enum auxiliary_driver {}
pub enum net_device {}
pub enum iidc_rdma_event {}
pub enum msix_entry {}
pub enum iidc_rdma_core_dev_info {}
pub enum iidc_rdma_reset_type {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
