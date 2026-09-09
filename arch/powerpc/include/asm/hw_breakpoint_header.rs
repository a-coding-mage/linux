/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PowerPC BookIII S hardware breakpoint definitions
 *
 * Copyright 2010, IBM Corporation.
 * Author: K.Prasad <prasad@linux.vnet.ibm.com>
 */

/* Translated from the C header; declarations supplied by included headers remain external dependencies. */

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct arch_hw_breakpoint {
    pub address: ::core::ffi::c_ulong,
    pub type_: u16,
    pub len: u16, /* length of the target data symbol */
    pub hw_len: u16, /* length programmed in hw */
    pub flags: u8,
    pub perf_single_step: bool, /* temporarily uninstalled for a perf single step */
}

/* Note: Don't change the first 6 bits below as they are in the same order
 * as the dabr and dabrx.
 */
pub const HW_BRK_TYPE_READ: u32 = 0x01;
pub const HW_BRK_TYPE_WRITE: u32 = 0x02;
pub const HW_BRK_TYPE_TRANSLATE: u32 = 0x04;
pub const HW_BRK_TYPE_USER: u32 = 0x08;
pub const HW_BRK_TYPE_KERNEL: u32 = 0x10;
pub const HW_BRK_TYPE_HYP: u32 = 0x20;
pub const HW_BRK_TYPE_EXTRANEOUS_IRQ: u32 = 0x80;

/* bits that overlap with the bottom 3 bits of the dabr */
pub const HW_BRK_TYPE_RDWR: u32 = HW_BRK_TYPE_READ | HW_BRK_TYPE_WRITE;
pub const HW_BRK_TYPE_DABR: u32 = HW_BRK_TYPE_RDWR | HW_BRK_TYPE_TRANSLATE;
pub const HW_BRK_TYPE_PRIV_ALL: u32 = HW_BRK_TYPE_USER | HW_BRK_TYPE_KERNEL | HW_BRK_TYPE_HYP;

pub const HW_BRK_FLAG_DISABLED: u32 = 0x1;

/* Minimum granularity; CONFIG_PPC_8xx selects the 0x4 value. */
#[cfg(feature = "ppc_8xx")]
pub const HW_BREAKPOINT_SIZE: u32 = 0x4;
#[cfg(not(feature = "ppc_8xx"))]
pub const HW_BREAKPOINT_SIZE: u32 = 0x8;
pub const HW_BREAKPOINT_SIZE_QUADWORD: u32 = 0x10;

pub const DABR_MAX_LEN: u32 = 8;
pub const DAWR_MAX_LEN: u32 = 512;

#[inline]
pub unsafe fn nr_wp_slots() -> ::core::ffi::c_int {
    if cpu_has_feature(CPU_FTR_DAWR1) { 2 } else { 1 }
}

extern "C" {
    pub fn cpu_has_feature(feature: ::core::ffi::c_int) -> bool;
    pub fn ppc_breakpoint_available() -> bool;
    pub fn __set_breakpoint(nr: ::core::ffi::c_int, brk: *const arch_hw_breakpoint);

    pub fn wp_check_constraints(
        regs: *mut pt_regs,
        instr: ppc_inst_t,
        ea: ::core::ffi::c_ulong,
        type_: ::core::ffi::c_int,
        size: ::core::ffi::c_int,
        info: *mut arch_hw_breakpoint,
    ) -> bool;
    pub fn wp_get_instr_detail(
        regs: *mut pt_regs,
        instr: *mut ppc_inst_t,
        type_: *mut ::core::ffi::c_int,
        size: *mut ::core::ffi::c_int,
        ea: *mut ::core::ffi::c_ulong,
    );
}

/* CONFIG_HAVE_HW_BREAKPOINT conditionally provides the following declarations. */
#[cfg(feature = "have_hw_breakpoint")]
extern "C" {
    pub fn hw_breakpoint_slots(type_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn arch_bp_generic_fields(type_: ::core::ffi::c_int, gen_bp_type: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> ::core::ffi::c_int;
    pub fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> ::core::ffi::c_int;
    pub fn hw_breakpoint_exceptions_notify(unused: *mut notifier_block, val: ::core::ffi::c_ulong, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> ::core::ffi::c_int;
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
    pub fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct);
    pub static mut perf_ops_bp: pmu;
    pub fn ptrace_triggered(bp: *mut perf_event, data: *mut perf_sample_data, regs: *mut pt_regs);
    pub fn thread_change_pc(tsk: *mut task_struct, regs: *mut pt_regs);
    pub fn hw_breakpoint_handler(args: *mut die_args) -> ::core::ffi::c_int;
}

#[cfg(feature = "have_hw_breakpoint")]
#[inline]
pub unsafe fn hw_breakpoint_disable() {
    if !ppc_breakpoint_available() { return; }
    let null_brk = arch_hw_breakpoint { address: 0, type_: 0, len: 0, hw_len: 0, flags: 0, perf_single_step: false };
    let mut i = 0;
    while i < nr_wp_slots() {
        __set_breakpoint(i, &null_brk);
        i += 1;
    }
}

#[cfg(not(feature = "have_hw_breakpoint"))]
#[inline]
pub unsafe fn hw_breakpoint_disable() {}
#[cfg(not(feature = "have_hw_breakpoint"))]
#[inline]
pub unsafe fn thread_change_pc(_tsk: *mut task_struct, _regs: *mut pt_regs) {}

#[cfg(feature = "ppc_dawr")]
extern "C" { pub static mut dawr_force_enable: bool; }
#[cfg(feature = "ppc_dawr")]
#[inline]
pub unsafe fn dawr_enabled() -> bool { dawr_force_enable }
#[cfg(feature = "ppc_dawr")]
extern "C" { pub fn set_dawr(nr: ::core::ffi::c_int, brk: *mut arch_hw_breakpoint) -> ::core::ffi::c_int; }
#[cfg(not(feature = "ppc_dawr"))]
#[inline]
pub unsafe fn dawr_enabled() -> bool { false }
#[cfg(not(feature = "ppc_dawr"))]
#[inline]
pub unsafe fn set_dawr(_nr: ::core::ffi::c_int, _brk: *mut arch_hw_breakpoint) -> ::core::ffi::c_int { -1 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
