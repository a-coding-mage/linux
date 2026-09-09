/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Dependency supplied by ivpu_drv.h in the C source.
#[repr(C)]
pub struct ivpu_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn ivpu_hw_ip_host_ss_configure(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_ip_idle_gen_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_idle_gen_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_pwr_domain_enable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_ip_host_ss_axi_enable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_ip_top_noc_enable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_ip_read_perf_timer_counter(vdev: *mut ivpu_device) -> u64;
    pub fn ivpu_hw_ip_snoop_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_tbu_mmu_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_soc_cpu_boot(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_ip_wdt_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_diagnose_failure(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_ipc_rx_count_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_ip_irq_clear(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_irq_handler_37xx(vdev: *mut ivpu_device, irq: i32) -> bool;
    pub fn ivpu_hw_ip_irq_handler_40xx(vdev: *mut ivpu_device, irq: i32) -> bool;
    pub fn ivpu_hw_ip_db_set(vdev: *mut ivpu_device, db_id: u32);
    pub fn ivpu_hw_ip_ipc_rx_addr_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_ip_ipc_tx_set(vdev: *mut ivpu_device, vpu_addr: u32);
    pub fn ivpu_hw_ip_irq_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_irq_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_fabric_req_override_enable_50xx(vdev: *mut ivpu_device);
    pub fn ivpu_hw_ip_fabric_req_override_disable_50xx(vdev: *mut ivpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
