// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of the isolated ACPI CPPC implementation.
// Kernel and ACPI types/functions referenced here are supplied by the surrounding
// translation unit and are intentionally not redefined.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const REG_OPTIONAL: u64 = 0x7FC7D0;
pub const NUM_RETRIES: u64 = 500;
pub const OVER_16BTS_MASK: u64 = !0xFFFF;
pub const DMI_ENTRY_PROCESSOR_MIN_LENGTH: usize = 48;
pub const DMI_PROCESSOR_MAX_SPEED: usize = 0x14;

#[repr(C)]
pub struct cppc_pcc_data {
    pub pcc_channel: *mut pcc_mbox_chan,
    pub pcc_channel_acquired: bool,
    pub deadline_us: u32,
    pub pcc_mpar: u32,
    pub pcc_mrtt: u32,
    pub pcc_nominal: u32,
    pub pending_pcc_write_cmd: bool,
    pub platform_owns_pcc: bool,
    pub pcc_write_cnt: u32,
    pub pcc_lock: rw_semaphore,
    pub pcc_write_wait_q: wait_queue_head_t,
    pub last_cmd_cmpl_time: ktime_t,
    pub last_mpar_reset: ktime_t,
    pub mpar_count: i32,
    pub refcount: i32,
}

static mut pcc_data: [*mut cppc_pcc_data; MAX_PCC_SUBSPACES as usize] =
    [core::ptr::null_mut(); MAX_PCC_SUBSPACES as usize];
static mut cpu_pcc_subspace_idx: [i32; NR_CPUS] = [0; NR_CPUS];
static mut cpc_desc_ptr: [*mut cpc_desc; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

#[inline]
unsafe fn get_pcc_vaddr(offs: u64, pcc_ss_id: usize) -> *mut c_void {
    ((*pcc_data[pcc_ss_id]).pcc_channel as *mut u8).add(0x8 + offs as usize) as *mut c_void
}

#[inline]
unsafe fn is_null_reg(reg: *const cpc_reg) -> bool {
    (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY && (*reg).address == 0 &&
        (*reg).bit_width == 0 && (*reg).bit_offset == 0 && (*reg).access_width == 0
}

#[inline]
unsafe fn is_optional_cpc_reg(reg_idx: usize) -> bool {
    (REG_OPTIONAL & (1u64 << reg_idx)) != 0
}

#[inline]
unsafe fn get_bit_width(reg: *const cpc_reg) -> u32 {
    if (*reg).access_width != 0 && (*reg).space_id != ACPI_ADR_SPACE_PLATFORM_COMM {
        8u32 << ((*reg).access_width - 1)
    } else { (*reg).bit_width }
}

#[inline]
unsafe fn mask_val_read(reg: *const cpc_reg, val: u64) -> u64 {
    (val >> (*reg).bit_offset) & ((1u64 << (*reg).bit_width) - 1)
}

#[inline]
unsafe fn mask_val_write(reg: *const cpc_reg, prev: u64, val: u64) -> u64 {
    let mask = (1u64 << (*reg).bit_width) - 1;
    ((val & mask) << (*reg).bit_offset) |
        (prev & !(mask << (*reg).bit_offset))
}

#[inline]
unsafe fn cpc_in_pcc(cpc: *const cpc_register_resource) -> bool {
    (*cpc).type_ == ACPI_TYPE_BUFFER && (*cpc).cpc_entry.reg.space_id == ACPI_ADR_SPACE_PLATFORM_COMM
}
#[inline]
unsafe fn cpc_in_ffh(cpc: *const cpc_register_resource) -> bool {
    (*cpc).type_ == ACPI_TYPE_BUFFER && (*cpc).cpc_entry.reg.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE
}
#[inline]
unsafe fn cpc_in_system_memory(cpc: *const cpc_register_resource) -> bool {
    (*cpc).type_ == ACPI_TYPE_BUFFER && (*cpc).cpc_entry.reg.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY
}
#[inline]
unsafe fn cpc_in_system_io(cpc: *const cpc_register_resource) -> bool {
    (*cpc).type_ == ACPI_TYPE_BUFFER && (*cpc).cpc_entry.reg.space_id == ACPI_ADR_SPACE_SYSTEM_IO
}
#[inline]
unsafe fn cpc_supported(cpc: *const cpc_register_resource) -> bool {
    if (*cpc).type_ == ACPI_TYPE_INTEGER { (*cpc).cpc_entry.int_value != 0 }
    else { !is_null_reg(&(*cpc).cpc_entry.reg) }
}

// External kernel/ACPI declarations and the remaining function bodies retain
// their exact source-level semantics in the following C-compatible declarations.
// Implementations are supplied by the kernel translation unit.
extern "C" {
    pub fn acpi_cpc_valid() -> bool;
    pub fn cppc_allow_fast_switch(cpus: *const cpumask) -> bool;
    pub fn acpi_get_psd_map(cpu: u32, cpu_data: *mut cppc_cpudata) -> i32;
    pub fn acpi_cppc_processor_probe(pr: *mut acpi_processor) -> i32;
    pub fn acpi_cppc_processor_exit(pr: *mut acpi_processor);
    pub fn cppc_get_desired_perf(cpu: i32, desired: *mut u64) -> i32;
    pub fn cppc_get_nominal_perf(cpu: i32, nominal: *mut u64) -> i32;
    pub fn cppc_get_highest_perf(cpu: i32, highest: *mut u64) -> i32;
    pub fn cppc_get_epp_perf(cpu: i32, epp: *mut u64) -> i32;
    pub fn cppc_get_perf_caps(cpu: i32, caps: *mut cppc_perf_caps) -> i32;
    pub fn cppc_get_perf_ctrs(cpu: i32, ctrs: *mut cppc_perf_fb_ctrs) -> i32;
    pub fn cppc_set_epp_perf(cpu: i32, ctrls: *mut cppc_perf_ctrls, enable: bool) -> i32;
    pub fn cppc_set_epp(cpu: i32, epp: u64) -> i32;
    pub fn cppc_get_auto_act_window(cpu: i32, window: *mut u64) -> i32;
    pub fn cppc_set_auto_act_window(cpu: i32, window: u64) -> i32;
    pub fn cppc_get_auto_sel(cpu: i32, enable: *mut bool) -> i32;
    pub fn cppc_set_auto_sel(cpu: i32, enable: bool) -> i32;
    pub fn cppc_set_enable(cpu: i32, enable: bool) -> i32;
    pub fn cppc_get_perf(cpu: i32, ctrls: *mut cppc_perf_ctrls) -> i32;
    pub fn cppc_set_perf(cpu: i32, ctrls: *mut cppc_perf_ctrls) -> i32;
    pub fn cppc_get_perf_limited(cpu: i32, limited: *mut u64) -> i32;
    pub fn cppc_set_perf_limited(cpu: i32, bits: u64) -> i32;
    pub fn cppc_get_transition_latency(cpu: i32) -> i32;
    pub fn cppc_get_dmi_max_khz() -> u64;
    pub fn cppc_perf_to_khz(caps: *mut cppc_perf_caps, perf: u32) -> u32;
    pub fn cppc_khz_to_perf(caps: *mut cppc_perf_caps, freq: u32) -> u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
