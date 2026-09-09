/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016,2017 ARM Limited, All Rights Reserved.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Translated from the C header. Build-time declarations and dependencies are
// supplied by the surrounding kernel translation.

pub const GICv4_ITS_LIST_MAX: usize = 16;

#[repr(C)]
pub struct its_vm {
    pub fwnode: *mut fwnode_handle,
    pub domain: *mut irq_domain,
    pub vprop_page: *mut page,
    pub vpes: *mut *mut its_vpe,
    pub nr_vpes: ::core::ffi::c_int,
    pub db_lpi_base: irq_hw_number_t,
    pub db_bitmap: *mut ::core::ffi::c_ulong,
    pub nr_db_lpis: ::core::ffi::c_int,
    pub vmapp_lock: raw_spinlock_t,
    pub vlpi_count: [u32; GICv4_ITS_LIST_MAX],
}

#[repr(C)]
pub struct its_vpe {
    pub vpt_page: *mut page,
    pub its_vm: *mut its_vm,
    pub vlpi_count: atomic_t,
    pub irq: ::core::ffi::c_int,
    pub vpe_db_lpi: irq_hw_number_t,
    pub resident: bool,
    pub ready: bool,
    pub impl_: its_vpe_impl,
    pub vmapp_count: atomic_t,
    pub vpe_lock: raw_spinlock_t,
    pub col_idx: u16,
    pub vpe_id: u16,
    pub pending_last: bool,
}

#[repr(C)]
pub union its_vpe_impl {
    pub gicv4_0: its_vpe_gicv4_0,
    pub gicv4_1: its_vpe_gicv4_1,
}

#[repr(C)]
pub struct its_vpe_gicv4_0 {
    pub vpe_proxy_event: ::core::ffi::c_int,
    pub idai: bool,
}

#[repr(C)]
pub struct its_vpe_gicv4_1 {
    pub fwnode: *mut fwnode_handle,
    pub sgi_domain: *mut irq_domain,
    pub sgi_config: [its_vpe_sgi_config; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_vpe_sgi_config {
    pub priority: u8,
    pub enabled: bool,
    pub group: bool,
}

#[repr(C)]
pub struct its_vlpi_map {
    pub vm: *mut its_vm,
    pub vpe: *mut its_vpe,
    pub vintid: u32,
    pub properties: u8,
    pub db_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum its_vcpu_info_cmd_type {
    MAP_VLPI,
    GET_VLPI,
    PROP_UPDATE_VLPI,
    PROP_UPDATE_AND_INV_VLPI,
    SCHEDULE_VPE,
    DESCHEDULE_VPE,
    COMMIT_VPE,
    INVALL_VPE,
    PROP_UPDATE_VSGI,
}

#[repr(C)]
pub union its_cmd_info_data {
    pub map: *mut its_vlpi_map,
    pub config: u8,
    pub req_db: bool,
    pub g0_g1: its_cmd_info_g0_g1,
    pub priority_group: its_cmd_info_priority_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_cmd_info_g0_g1 {
    pub g0en: bool,
    pub g1en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_cmd_info_priority_group {
    pub priority: u8,
    pub group: bool,
}

#[repr(C)]
pub struct its_cmd_info {
    pub cmd_type: its_vcpu_info_cmd_type,
    pub data: its_cmd_info_data,
}

extern "C" {
    pub fn its_alloc_vcpu_irqs(vm: *mut its_vm) -> ::core::ffi::c_int;
    pub fn its_free_vcpu_irqs(vm: *mut its_vm);
    pub fn its_make_vpe_resident(vpe: *mut its_vpe, g0en: bool, g1en: bool) -> ::core::ffi::c_int;
    pub fn its_make_vpe_non_resident(vpe: *mut its_vpe, db: bool) -> ::core::ffi::c_int;
    pub fn its_commit_vpe(vpe: *mut its_vpe) -> ::core::ffi::c_int;
    pub fn its_invall_vpe(vpe: *mut its_vpe) -> ::core::ffi::c_int;
    pub fn its_map_vlpi(irq: ::core::ffi::c_int, map: *mut its_vlpi_map) -> ::core::ffi::c_int;
    pub fn its_get_vlpi(irq: ::core::ffi::c_int, map: *mut its_vlpi_map) -> ::core::ffi::c_int;
    pub fn its_unmap_vlpi(irq: ::core::ffi::c_int);
    pub fn its_prop_update_vlpi(irq: ::core::ffi::c_int, config: u8, inv: bool) -> ::core::ffi::c_int;
    pub fn its_prop_update_vsgi(irq: ::core::ffi::c_int, priority: u8, group: bool) -> ::core::ffi::c_int;
    pub fn its_init_v4(
        domain: *mut irq_domain,
        vpe_ops: *const irq_domain_ops,
        sgi_ops: *const irq_domain_ops,
    ) -> ::core::ffi::c_int;
    pub fn gic_cpuif_has_vsgi() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
