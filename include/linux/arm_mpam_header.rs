/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2025 Arm Ltd. */

// Translated from linux/arm_mpam.h.
// C includes and configuration symbols are supplied by the surrounding build.

#[repr(C)]
pub struct mpam_msc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_mpam_msc_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rdt_resource {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum resctrl_event_id {
    _Invalid = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpam_msc_iface {
    MPAM_IFACE_MMIO,
    MPAM_IFACE_PCC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpam_class_types {
    MPAM_CLASS_CACHE,
    MPAM_CLASS_MEMORY,
    MPAM_CLASS_UNKNOWN,
}

pub const MPAM_CLASS_ID_DEFAULT: u8 = 255;

// Under CONFIG_ACPI_MPAM this is an external function; otherwise the inline
// implementation returns -EINVAL.
#[cfg(feature = "CONFIG_ACPI_MPAM")]
unsafe extern "C" {
    pub fn acpi_mpam_parse_resources(
        msc: *mut mpam_msc,
        tbl_msc: *mut acpi_mpam_msc_node,
    ) -> i32;
    pub fn acpi_mpam_count_msc() -> i32;
}

#[cfg(not(feature = "CONFIG_ACPI_MPAM"))]
#[inline]
pub unsafe fn acpi_mpam_parse_resources(
    _msc: *mut mpam_msc,
    _tbl_msc: *mut acpi_mpam_msc_node,
) -> i32 {
    -22
}

#[cfg(not(feature = "CONFIG_ACPI_MPAM"))]
#[inline]
pub fn acpi_mpam_count_msc() -> i32 {
    -22
}

// Under CONFIG_ARM64_MPAM_DRIVER this is an external function; otherwise the
// inline implementation returns -EINVAL.
#[cfg(feature = "CONFIG_ARM64_MPAM_DRIVER")]
unsafe extern "C" {
    pub fn mpam_ris_create(
        msc: *mut mpam_msc,
        ris_idx: u8,
        r#type: mpam_class_types,
        class_id: u8,
        component_id: i32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_ARM64_MPAM_DRIVER"))]
#[inline]
pub unsafe fn mpam_ris_create(
    _msc: *mut mpam_msc,
    _ris_idx: u8,
    _type: mpam_class_types,
    _class_id: u8,
    _component_id: i32,
) -> i32 {
    -22
}

unsafe extern "C" {
    pub fn resctrl_arch_alloc_capable() -> bool;
    pub fn resctrl_arch_mon_capable() -> bool;

    pub fn resctrl_arch_set_cpu_default_closid(cpu: i32, closid: u32);
    pub fn resctrl_arch_set_closid_rmid(tsk: *mut task_struct, closid: u32, rmid: u32);
    pub fn resctrl_arch_set_cpu_default_closid_rmid(cpu: i32, closid: u32, rmid: u32);
    pub fn resctrl_arch_sched_in(tsk: *mut task_struct);
    pub fn resctrl_arch_match_closid(tsk: *mut task_struct, closid: u32) -> bool;
    pub fn resctrl_arch_match_rmid(tsk: *mut task_struct, closid: u32, rmid: u32) -> bool;
    pub fn resctrl_arch_rmid_idx_encode(closid: u32, rmid: u32) -> u32;
    pub fn resctrl_arch_rmid_idx_decode(idx: u32, closid: *mut u32, rmid: *mut u32);
    pub fn resctrl_arch_system_num_rmid_idx() -> u32;

    pub fn resctrl_arch_mon_ctx_alloc(
        r: *mut rdt_resource,
        evtid: resctrl_event_id,
    ) -> *mut core::ffi::c_void;
    pub fn resctrl_arch_mon_ctx_free(
        r: *mut rdt_resource,
        evtid: resctrl_event_id,
        ctx: *mut core::ffi::c_void,
    );

    pub fn mpam_register_requestor(partid_max: u16, pmg_max: u8) -> i32;
}

#[inline]
pub fn resctrl_arch_enable_mon() {}
#[inline]
pub fn resctrl_arch_disable_mon() {}
#[inline]
pub fn resctrl_arch_enable_alloc() {}
#[inline]
pub fn resctrl_arch_disable_alloc() {}

#[inline]
pub fn resctrl_arch_round_mon_val(val: u32) -> u32 {
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
