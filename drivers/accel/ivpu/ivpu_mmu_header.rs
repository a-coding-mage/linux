/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2023 Intel Corporation
 */

// C header dependencies and include guards are omitted; the referenced types
// are supplied by other translated components.

#[repr(C)]
pub struct ivpu_mmu_cdtab {
    pub base: *mut core::ffi::c_void,
    pub dma: dma_addr_t,
}

#[repr(C)]
pub struct ivpu_mmu_strtab {
    pub base: *mut core::ffi::c_void,
    pub dma: dma_addr_t,
    pub dma_q: u64,
    pub base_cfg: u32,
}

#[repr(C)]
pub struct ivpu_mmu_queue {
    pub base: *mut core::ffi::c_void,
    pub dma: dma_addr_t,
    pub dma_q: u64,
    pub prod: u32,
    pub cons: u32,
}

#[repr(C)]
pub struct ivpu_mmu_info {
    pub lock: mutex, /* Protects cdtab, strtab, cmdq, on */
    pub cdtab: ivpu_mmu_cdtab,
    pub strtab: ivpu_mmu_strtab,
    pub cmdq: ivpu_mmu_queue,
    pub evtq: ivpu_mmu_queue,
    pub on: bool,
}

unsafe extern "C" {
    pub fn ivpu_mmu_init(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_mmu_disable(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_enable(vdev: *mut ivpu_device) -> core::ffi::c_int;
    pub fn ivpu_mmu_cd_set(
        vdev: *mut ivpu_device,
        ssid: core::ffi::c_int,
        pgtable: *mut ivpu_mmu_pgtable,
    ) -> core::ffi::c_int;
    pub fn ivpu_mmu_cd_clear(vdev: *mut ivpu_device, ssid: core::ffi::c_int);
    pub fn ivpu_mmu_invalidate_tlb(
        vdev: *mut ivpu_device,
        ssid: u16,
    ) -> core::ffi::c_int;

    pub fn ivpu_mmu_irq_evtq_handler(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_irq_gerr_handler(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_evtq_dump(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_discard_events(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_disable_ssid_events(vdev: *mut ivpu_device, ssid: u32) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
