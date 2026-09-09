/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit:
// ivpu_drv.h, ivpu_hw_btrs.h, and ivpu_hw_ip.h.

#[repr(C)]
pub struct ivpu_addr_range {
    pub start: resource_size_t,
    pub end: resource_size_t,
}

#[repr(C)]
pub struct ivpu_hw_info {
    pub irq: ivpu_hw_info_irq,
    pub ranges: ivpu_hw_info_ranges,
    pub pll: ivpu_hw_info_pll,
    pub hws: ivpu_hw_info_hws,
    pub tile_fuse: u32,
    pub sku: u32,
    pub config: u16,
    pub dma_bits: core::ffi::c_int,
    pub d0i3_entry_host_ts: ktime_t,
    pub d0i3_entry_vpu_ts: u64,
    pub firewall_irq_counter: atomic_t,
}

#[repr(C)]
pub struct ivpu_hw_info_irq {
    pub btrs_irq_handler:
        Option<unsafe extern "C" fn(vdev: *mut ivpu_device, irq: core::ffi::c_int) -> bool>,
    pub ip_irq_handler:
        Option<unsafe extern "C" fn(vdev: *mut ivpu_device, irq: core::ffi::c_int) -> bool>,
}

#[repr(C)]
pub struct ivpu_hw_info_ranges {
    pub runtime: ivpu_addr_range,
    pub global: ivpu_addr_range,
    pub user: ivpu_addr_range,
    pub shave: ivpu_addr_range,
    pub dma: ivpu_addr_range,
}

#[repr(C)]
pub struct ivpu_hw_info_pll {
    /* Hardware min and max pll ratio */
    pub min_ratio: u8,
    pub max_ratio: u8,
    /*
     * Pll ratio for the efficiency frequency. The VPU has optimum
     * performance to power ratio at this frequency.
     */
    pub pn_ratio: u8,
    /* Pll ratios configured via sysfs interface */
    pub cfg_min_ratio: u8,
    pub cfg_max_ratio: u8,
    pub profiling_freq: u32,
}

#[repr(C)]
pub struct ivpu_hw_info_hws {
    pub grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS as usize],
    pub process_quantum: [u32; VPU_HWS_NUM_PRIORITY_BANDS as usize],
    pub process_grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS as usize],
}

extern "C" {
    pub fn ivpu_hw_init(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_hw_range_init(
        vdev: *mut ivpu_device,
        range: *mut ivpu_addr_range,
        start: u64,
        size: u64,
    ) -> core::ffi::c_int;
    pub fn ivpu_hw_power_up(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_hw_power_down(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_hw_reset(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_hw_boot_fw(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_hw_profiling_freq_drive(vdev: *mut ivpu_device, enable: bool);
    pub fn ivpu_irq_handlers_init(vdev: *mut ivpu_device);
    pub fn ivpu_hw_irq_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_irq_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_irq_handler(irq: core::ffi::c_int, ptr: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn ivpu_hw_uses_ecc_mca_signal(vdev: *mut ivpu_device) -> bool;
}

#[inline]
pub unsafe fn ivpu_hw_btrs_irq_handler(vdev: *mut ivpu_device, irq: core::ffi::c_int) -> u32 {
    ((*(*vdev).hw).irq.btrs_irq_handler.expect("null btrs_irq_handler")(vdev, irq) as u32)
}

#[inline]
pub unsafe fn ivpu_hw_ip_irq_handler(vdev: *mut ivpu_device, irq: core::ffi::c_int) -> u32 {
    ((*(*vdev).hw).irq.ip_irq_handler.expect("null ip_irq_handler")(vdev, irq) as u32)
}

#[inline]
pub unsafe fn ivpu_hw_range_size(range: *const ivpu_addr_range) -> u64 {
    (*range).end - (*range).start
}

#[inline]
pub unsafe fn ivpu_hw_irq_clear(vdev: *mut ivpu_device) {
    ivpu_hw_ip_irq_clear(vdev);
}

#[inline]
pub unsafe fn ivpu_hw_profiling_freq_get(vdev: *mut ivpu_device) -> u32 {
    (*(*vdev).hw).pll.profiling_freq
}

#[inline]
pub unsafe fn ivpu_hw_diagnose_failure(vdev: *mut ivpu_device) {
    ivpu_hw_ip_diagnose_failure(vdev);
    ivpu_hw_btrs_diagnose_failure(vdev);
}

#[inline]
pub unsafe fn ivpu_hw_telemetry_offset_get(vdev: *mut ivpu_device) -> u32 {
    ivpu_hw_btrs_telemetry_offset_get(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_telemetry_size_get(vdev: *mut ivpu_device) -> u32 {
    ivpu_hw_btrs_telemetry_size_get(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_telemetry_enable_get(vdev: *mut ivpu_device) -> u32 {
    ivpu_hw_btrs_telemetry_enable_get(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_is_idle(vdev: *mut ivpu_device) -> bool {
    ivpu_hw_btrs_is_idle(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_wait_for_idle(vdev: *mut ivpu_device) -> core::ffi::c_int {
    ivpu_hw_btrs_wait_for_idle(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_ipc_tx_set(vdev: *mut ivpu_device, vpu_addr: u32) {
    ivpu_hw_ip_ipc_tx_set(vdev, vpu_addr);
}

#[inline]
pub unsafe fn ivpu_hw_db_set(vdev: *mut ivpu_device, db_id: u32) {
    ivpu_hw_ip_db_set(vdev, db_id);
}

#[inline]
pub unsafe fn ivpu_hw_ipc_rx_addr_get(vdev: *mut ivpu_device) -> u32 {
    ivpu_hw_ip_ipc_rx_addr_get(vdev)
}

#[inline]
pub unsafe fn ivpu_hw_ipc_rx_count_get(vdev: *mut ivpu_device) -> u32 {
    ivpu_hw_ip_ipc_rx_count_get(vdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
