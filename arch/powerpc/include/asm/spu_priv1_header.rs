/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Defines an spu hypervisor abstraction layer.
 *
 *  Copyright 2006 Sony Corp.
 */

// The declarations in this header are available when building the kernel.

use core::ffi::c_void;

#[repr(C)]
pub struct spu;
#[repr(C)]
pub struct spu_context;

/* access to priv1 registers */

#[repr(C)]
pub struct spu_priv1_ops {
    pub int_mask_and: unsafe extern "C" fn(*mut spu, i32, u64),
    pub int_mask_or: unsafe extern "C" fn(*mut spu, i32, u64),
    pub int_mask_set: unsafe extern "C" fn(*mut spu, i32, u64),
    pub int_mask_get: unsafe extern "C" fn(*mut spu, i32) -> u64,
    pub int_stat_clear: unsafe extern "C" fn(*mut spu, i32, u64),
    pub int_stat_get: unsafe extern "C" fn(*mut spu, i32) -> u64,
    pub cpu_affinity_set: unsafe extern "C" fn(*mut spu, i32),
    pub mfc_dar_get: unsafe extern "C" fn(*mut spu) -> u64,
    pub mfc_dsisr_get: unsafe extern "C" fn(*mut spu) -> u64,
    pub mfc_dsisr_set: unsafe extern "C" fn(*mut spu, u64),
    pub mfc_sdr_setup: unsafe extern "C" fn(*mut spu),
    pub mfc_sr1_set: unsafe extern "C" fn(*mut spu, u64),
    pub mfc_sr1_get: unsafe extern "C" fn(*mut spu) -> u64,
    pub mfc_tclass_id_set: unsafe extern "C" fn(*mut spu, u64),
    pub mfc_tclass_id_get: unsafe extern "C" fn(*mut spu) -> u64,
    pub tlb_invalidate: unsafe extern "C" fn(*mut spu),
    pub resource_allocation_groupID_set: unsafe extern "C" fn(*mut spu, u64),
    pub resource_allocation_groupID_get: unsafe extern "C" fn(*mut spu) -> u64,
    pub resource_allocation_enable_set: unsafe extern "C" fn(*mut spu, u64),
    pub resource_allocation_enable_get: unsafe extern "C" fn(*mut spu) -> u64,
}

unsafe extern "C" {
    pub static spu_priv1_ops: *const spu_priv1_ops;
}

pub unsafe fn spu_int_mask_and(spu: *mut spu, class: i32, mask: u64) {
    ((*spu_priv1_ops).int_mask_and)(spu, class, mask)
}
pub unsafe fn spu_int_mask_or(spu: *mut spu, class: i32, mask: u64) {
    ((*spu_priv1_ops).int_mask_or)(spu, class, mask)
}
pub unsafe fn spu_int_mask_set(spu: *mut spu, class: i32, mask: u64) {
    ((*spu_priv1_ops).int_mask_set)(spu, class, mask)
}
pub unsafe fn spu_int_mask_get(spu: *mut spu, class: i32) -> u64 {
    ((*spu_priv1_ops).int_mask_get)(spu, class)
}
pub unsafe fn spu_int_stat_clear(spu: *mut spu, class: i32, stat: u64) {
    ((*spu_priv1_ops).int_stat_clear)(spu, class, stat)
}
pub unsafe fn spu_int_stat_get(spu: *mut spu, class: i32) -> u64 {
    ((*spu_priv1_ops).int_stat_get)(spu, class)
}
pub unsafe fn spu_cpu_affinity_set(spu: *mut spu, cpu: i32) {
    ((*spu_priv1_ops).cpu_affinity_set)(spu, cpu)
}
pub unsafe fn spu_mfc_dar_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).mfc_dar_get)(spu) }
pub unsafe fn spu_mfc_dsisr_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).mfc_dsisr_get)(spu) }
pub unsafe fn spu_mfc_dsisr_set(spu: *mut spu, dsisr: u64) { ((*spu_priv1_ops).mfc_dsisr_set)(spu, dsisr) }
pub unsafe fn spu_mfc_sdr_setup(spu: *mut spu) { ((*spu_priv1_ops).mfc_sdr_setup)(spu) }
pub unsafe fn spu_mfc_sr1_set(spu: *mut spu, sr1: u64) { ((*spu_priv1_ops).mfc_sr1_set)(spu, sr1) }
pub unsafe fn spu_mfc_sr1_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).mfc_sr1_get)(spu) }
pub unsafe fn spu_mfc_tclass_id_set(spu: *mut spu, tclass_id: u64) { ((*spu_priv1_ops).mfc_tclass_id_set)(spu, tclass_id) }
pub unsafe fn spu_mfc_tclass_id_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).mfc_tclass_id_get)(spu) }
pub unsafe fn spu_tlb_invalidate(spu: *mut spu) { ((*spu_priv1_ops).tlb_invalidate)(spu) }
pub unsafe fn spu_resource_allocation_groupID_set(spu: *mut spu, id: u64) { ((*spu_priv1_ops).resource_allocation_groupID_set)(spu, id) }
pub unsafe fn spu_resource_allocation_groupID_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).resource_allocation_groupID_get)(spu) }
pub unsafe fn spu_resource_allocation_enable_set(spu: *mut spu, enable: u64) { ((*spu_priv1_ops).resource_allocation_enable_set)(spu, enable) }
pub unsafe fn spu_resource_allocation_enable_get(spu: *mut spu) -> u64 { ((*spu_priv1_ops).resource_allocation_enable_get)(spu) }

/* spu management abstraction */

#[repr(C)]
pub struct spu_management_ops {
    pub enumerate_spus: unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void) -> i32) -> i32,
    pub create_spu: unsafe extern "C" fn(*mut spu, *mut c_void) -> i32,
    pub destroy_spu: unsafe extern "C" fn(*mut spu) -> i32,
    pub enable_spu: unsafe extern "C" fn(*mut spu_context),
    pub disable_spu: unsafe extern "C" fn(*mut spu_context),
    pub init_affinity: unsafe extern "C" fn() -> i32,
}

unsafe extern "C" {
    pub static spu_management_ops: *const spu_management_ops;
    pub static spu_management_of_ops: spu_management_ops;
}

pub unsafe fn spu_enumerate_spus(fn_: unsafe extern "C" fn(*mut c_void) -> i32) -> i32 {
    ((*spu_management_ops).enumerate_spus)(fn_)
}
pub unsafe fn spu_create_spu(spu: *mut spu, data: *mut c_void) -> i32 {
    ((*spu_management_ops).create_spu)(spu, data)
}
pub unsafe fn spu_destroy_spu(spu: *mut spu) -> i32 { ((*spu_management_ops).destroy_spu)(spu) }
pub unsafe fn spu_init_affinity() -> i32 { ((*spu_management_ops).init_affinity)() }
pub unsafe fn spu_enable_spu(ctx: *mut spu_context) { ((*spu_management_ops).enable_spu)(ctx) }
pub unsafe fn spu_disable_spu(ctx: *mut spu_context) { ((*spu_management_ops).disable_spu)(ctx) }

/*
 * The declarations following are put here for convenience
 * and only intended to be used by the platform setup code.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
