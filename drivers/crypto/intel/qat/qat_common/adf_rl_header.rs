/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Translated from adf_rl.h. C headers and build-time dependencies are supplied externally.

pub const RL_ROOT_MAX: usize = 4;
pub const RL_CLUSTER_MAX: usize = 16;
pub const RL_LEAF_MAX: usize = 64;
pub const RL_NODES_CNT_MAX: usize = RL_ROOT_MAX + RL_CLUSTER_MAX + RL_LEAF_MAX;
pub const RL_RP_CNT_PER_LEAF_MAX: u32 = 4;
pub const RL_RP_CNT_MAX: usize = 64;
pub const RL_SLA_EMPTY_ID: i32 = -1;
pub const RL_PARENT_DEFAULT_ID: i32 = -1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rl_node_type {
    RL_ROOT,
    RL_CLUSTER,
    RL_LEAF,
}

#[repr(C)]
pub struct adf_rl_sla_input_data {
    pub rp_mask: u64,
    pub sla_id: i32,
    pub parent_id: i32,
    pub cir: u32,
    pub pir: u32,
    pub type_: rl_node_type,
    pub srv: adf_base_services,
}

#[repr(C)]
pub struct rl_slice_cnt {
    pub dcpr_cnt: u8,
    pub pke_cnt: u8,
    pub cph_cnt: u8,
    pub cpr_cnt: u8,
}

#[repr(C)]
pub struct adf_rl_interface_data {
    pub input: adf_rl_sla_input_data,
    pub cap_rem_srv: adf_base_services,
    pub lock: rw_semaphore,
    pub sysfs_added: bool,
}

#[repr(C)]
pub struct adf_rl_hw_data {
    pub scale_ref: u32,
    pub scan_interval: u32,
    pub r2l_offset: u32,
    pub l2c_offset: u32,
    pub c2s_offset: u32,
    pub pciin_tb_offset: u32,
    pub pciout_tb_offset: u32,
    pub pcie_scale_mul: u32,
    pub pcie_scale_div: u32,
    pub dcpr_correction: u32,
    pub max_tp: [u32; RL_ROOT_MAX],
    pub svc_ae_mask: [u32; SVC_BASE_COUNT],
    pub slices: rl_slice_cnt,
}

#[repr(C)]
pub struct adf_rl {
    pub accel_dev: *mut adf_accel_dev,
    pub device_data: *mut adf_rl_hw_data,
    // mapping sla_id to SLA objects
    pub sla: [*mut rl_sla; RL_NODES_CNT_MAX],
    pub root: [*mut rl_sla; RL_ROOT_MAX],
    pub cluster: [*mut rl_sla; RL_CLUSTER_MAX],
    pub leaf: [*mut rl_sla; RL_LEAF_MAX],
    pub rp_in_use: [bool; RL_RP_CNT_MAX],
    // Mutex protecting writing to SLAs lists
    pub rl_lock: mutex,
    pub user_input: adf_rl_interface_data,
}

#[repr(C)]
pub struct rl_sla {
    pub parent: *mut rl_sla,
    pub type_: rl_node_type,
    pub srv: adf_base_services,
    pub sla_id: u32,
    pub node_id: u32,
    pub cir: u32,
    pub pir: u32,
    pub rem_cir: u32,
    pub ring_pairs_ids: [u16; RL_RP_CNT_PER_LEAF_MAX as usize],
    pub ring_pairs_cnt: u16,
}

extern "C" {
    pub fn adf_rl_get_sla_arr_of_type(
        rl_data: *mut adf_rl,
        type_: rl_node_type,
        sla_arr: *mut *mut *mut rl_sla,
    ) -> u32;
    pub fn adf_rl_add_sla(accel_dev: *mut adf_accel_dev, sla_in: *mut adf_rl_sla_input_data) -> i32;
    pub fn adf_rl_update_sla(accel_dev: *mut adf_accel_dev, sla_in: *mut adf_rl_sla_input_data) -> i32;
    pub fn adf_rl_get_sla(accel_dev: *mut adf_accel_dev, sla_in: *mut adf_rl_sla_input_data) -> i32;
    pub fn adf_rl_get_capability_remaining(
        accel_dev: *mut adf_accel_dev,
        srv: adf_base_services,
        sla_id: i32,
    ) -> i32;
    pub fn adf_rl_remove_sla(accel_dev: *mut adf_accel_dev, sla_id: u32) -> i32;
    pub fn adf_rl_remove_sla_all(accel_dev: *mut adf_accel_dev, incl_default: bool);
    pub fn adf_rl_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_rl_start(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_rl_stop(accel_dev: *mut adf_accel_dev);
    pub fn adf_rl_exit(accel_dev: *mut adf_accel_dev);
    pub fn adf_rl_calculate_pci_bw(
        accel_dev: *mut adf_accel_dev,
        sla_val: u32,
        svc_type: adf_base_services,
        is_bw_out: bool,
    ) -> u32;
    pub fn adf_rl_calculate_ae_cycles(
        accel_dev: *mut adf_accel_dev,
        sla_val: u32,
        svc_type: adf_base_services,
    ) -> u32;
    pub fn adf_rl_calculate_slice_tokens(
        accel_dev: *mut adf_accel_dev,
        sla_val: u32,
        svc_type: adf_base_services,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
