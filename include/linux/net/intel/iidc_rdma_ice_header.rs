/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2021-2025, Intel Corporation. */

/* Translated from iidc_rdma_ice.h.  linux/dcbnl.h supplies the DCB constants. */

pub const IIDC_MAX_USER_PRIORITY: usize = 8;
pub const IIDC_DSCP_PFC_MODE: u32 = 0x1;

/**
 * struct iidc_rdma_qset_params - Struct to hold per RDMA Qset info
 * @teid: TEID of the Qset node
 * @qs_handle: SW index of the Qset, RDMA provides this
 * @vport_id: VSI index
 * @tc: Traffic Class branch the QSet should belong to
 */
#[repr(C)]
pub struct iidc_rdma_qset_params {
    /* Qset TEID returned to the RDMA driver in
     * ice_add_rdma_qset and used by RDMA driver
     * for calls to ice_del_rdma_qset
     */
    pub teid: u32,
    pub qs_handle: u16,
    pub vport_id: u16,
    pub tc: u8,
}

#[repr(C)]
pub struct iidc_rdma_qos_info {
    pub tc_ctx: u64,
    pub rel_bw: u8,
    pub prio_type: u8,
    pub egress_virt_up: u8,
    pub ingress_virt_up: u8,
}

/* Struct to pass QoS info */
#[repr(C)]
pub struct iidc_rdma_qos_params {
    pub tc_info: [iidc_rdma_qos_info; IEEE_8021QAZ_MAX_TCS],
    pub up2tc: [u8; IIDC_MAX_USER_PRIORITY],
    pub vport_relative_bw: u8,
    pub vport_priority_type: u8,
    pub num_tc: u8,
    pub pfc_mode: u8,
    pub dscp_map: [u8; DSCP_MAX],
}

#[repr(C)]
pub struct iidc_rdma_priv_dev_info {
    pub pf_id: u8,
    pub vport_id: u16,
    pub netdev: *mut net_device,
    pub qos_info: iidc_rdma_qos_params,
    pub hw_addr: *mut u8,
}

unsafe extern "C" {
    pub fn ice_add_rdma_qset(
        cdev: *mut iidc_rdma_core_dev_info,
        qset: *mut iidc_rdma_qset_params,
    ) -> i32;
    pub fn ice_del_rdma_qset(
        cdev: *mut iidc_rdma_core_dev_info,
        qset: *mut iidc_rdma_qset_params,
    ) -> i32;
    pub fn ice_rdma_request_reset(
        cdev: *mut iidc_rdma_core_dev_info,
        reset_type: iidc_rdma_reset_type,
    ) -> i32;
    pub fn ice_rdma_update_vsi_filter(
        cdev: *mut iidc_rdma_core_dev_info,
        vsi_id: u16,
        enable: bool,
    ) -> i32;
    pub fn ice_alloc_rdma_qvector(
        cdev: *mut iidc_rdma_core_dev_info,
        entry: *mut msix_entry,
    ) -> i32;
    pub fn ice_free_rdma_qvector(
        cdev: *mut iidc_rdma_core_dev_info,
        entry: *mut msix_entry,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
