/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Parav Pandit <pandit.parav@gmail.com>
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rdmacg_resource_type {
    RDMACG_RESOURCE_HCA_HANDLE,
    RDMACG_RESOURCE_HCA_OBJECT,
    RDMACG_RESOURCE_MAX,
}

// CONFIG_CGROUP_RDMA is a build-time configuration condition preserved here.
#[cfg(CONFIG_CGROUP_RDMA)]
#[repr(C)]
pub struct rdma_cgroup {
    pub css: cgroup_subsys_state,

    /*
     * head to keep track of all resource pools
     * that belongs to this cgroup.
     */
    pub rpools: list_head,

    /* Handles for rdma.events[.local] */
    pub events_file: cgroup_file,
    pub events_local_file: cgroup_file,
}

#[cfg(CONFIG_CGROUP_RDMA)]
#[repr(C)]
pub struct rdmacg_device {
    pub dev_node: list_head,
    pub rpools: list_head,
    pub name: *mut core::ffi::c_char,
    pub index: u32,
}

/*
 * APIs for RDMA/IB stack to publish when a device wants to
 * participate in resource accounting
 */
#[cfg(CONFIG_CGROUP_RDMA)]
unsafe extern "C" {
    pub fn rdmacg_register_device(device: *mut rdmacg_device);
    pub fn rdmacg_unregister_device(device: *mut rdmacg_device);
}

/* APIs for RDMA/IB stack to charge/uncharge pool specific resources */
#[cfg(CONFIG_CGROUP_RDMA)]
unsafe extern "C" {
    pub fn rdmacg_try_charge(
        rdmacg: *mut *mut rdma_cgroup,
        device: *mut rdmacg_device,
        index: rdmacg_resource_type,
    ) -> core::ffi::c_int;
    pub fn rdmacg_uncharge(
        cg: *mut rdma_cgroup,
        device: *mut rdmacg_device,
        index: rdmacg_resource_type,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
