/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Declarations for to Hexagon Virtal Machine.
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

/*
 * In principle, a Linux kernel for the VM could
 * selectively define the virtual instructions
 * as inline assembler macros, but for a first pass,
 * we'll use subroutines for both the VM and the native
 * kernels.  It's costing a subroutine call/return,
 * but it makes for a single set of entry points
 * for tracing/debugging.
 */

pub const HVM_TRAP1_VMVERSION: i32 = 0;
pub const HVM_TRAP1_VMRTE: i32 = 1;
pub const HVM_TRAP1_VMSETVEC: i32 = 2;
pub const HVM_TRAP1_VMSETIE: i32 = 3;
pub const HVM_TRAP1_VMGETIE: i32 = 4;
pub const HVM_TRAP1_VMINTOP: i32 = 5;
pub const HVM_TRAP1_VMCLRMAP: i32 = 10;
pub const HVM_TRAP1_VMNEWMAP: i32 = 11;
pub const HVM_TRAP1_FORMERLY_VMWIRE: i32 = 12;
pub const HVM_TRAP1_VMCACHE: i32 = 13;
pub const HVM_TRAP1_VMGETTIME: i32 = 14;
pub const HVM_TRAP1_VMSETTIME: i32 = 15;
pub const HVM_TRAP1_VMWAIT: i32 = 16;
pub const HVM_TRAP1_VMYIELD: i32 = 17;
pub const HVM_TRAP1_VMSTART: i32 = 18;
pub const HVM_TRAP1_VMSTOP: i32 = 19;
pub const HVM_TRAP1_VMVPID: i32 = 20;
pub const HVM_TRAP1_VMSETREGS: i32 = 21;
pub const HVM_TRAP1_VMGETREGS: i32 = 22;
pub const HVM_TRAP1_VMTIMEROP: i32 = 24;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VM_CACHE_OPS {
    hvmc_ickill,
    hvmc_dckill,
    hvmc_l2kill,
    hvmc_dccleaninva,
    hvmc_icinva,
    hvmc_idsync,
    hvmc_fetch_cfg,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VM_INT_OPS {
    hvmi_nop,
    hvmi_globen,
    hvmi_globdis,
    hvmi_locen,
    hvmi_locdis,
    hvmi_affinity,
    hvmi_get,
    hvmi_peek,
    hvmi_status,
    hvmi_post,
    hvmi_clear,
}

unsafe extern "C" {
    pub fn _K_VM_event_vector();
    pub fn __vmrte();
    pub fn __vmsetvec(arg: *mut core::ffi::c_void) -> isize;
    pub fn __vmsetie(arg: isize) -> isize;
    pub fn __vmgetie() -> isize;
    pub fn __vmintop(op: VM_INT_OPS, arg1: isize, arg2: isize, arg3: isize, arg4: isize) -> isize;
    pub fn __vmclrmap(arg: *mut core::ffi::c_void, flags: usize) -> isize;
    pub fn __vmnewmap(arg: *mut core::ffi::c_void) -> isize;
    pub fn __vmcache(op: VM_CACHE_OPS, addr: usize, len: usize) -> isize;
    pub fn __vmgettime() -> u64;
    pub fn __vmsettime(value: u64) -> isize;
    pub fn __vmstart(arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> isize;
    pub fn __vmstop();
    pub fn __vmwait() -> isize;
    pub fn __vmyield();
    pub fn __vmvpid() -> isize;
}

#[inline]
pub unsafe fn __vmcache_ickill() -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_ickill, 0, 0) } }
#[inline]
pub unsafe fn __vmcache_dckill() -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_dckill, 0, 0) } }
#[inline]
pub unsafe fn __vmcache_l2kill() -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_l2kill, 0, 0) } }
#[inline]
pub unsafe fn __vmcache_dccleaninva(addr: usize, len: usize) -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_dccleaninva, addr, len) } }
#[inline]
pub unsafe fn __vmcache_icinva(addr: usize, len: usize) -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_icinva, addr, len) } }
#[inline]
pub unsafe fn __vmcache_idsync(addr: usize, len: usize) -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_idsync, addr, len) } }
#[inline]
pub unsafe fn __vmcache_fetch_cfg(val: usize) -> isize { unsafe { __vmcache(VM_CACHE_OPS::hvmc_fetch_cfg, val, 0) } }

/* interrupt operations */

#[inline]
pub unsafe fn __vmintop_nop() -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_nop, 0, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_globen(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_globen, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_globdis(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_globdis, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_locen(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_locen, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_locdis(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_locdis, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_affinity(i: isize, cpu: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_affinity, i, cpu, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_get() -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_get, 0, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_peek() -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_peek, 0, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_status(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_status, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_post(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_post, i, 0, 0, 0) } }
#[inline]
pub unsafe fn __vmintop_clear(i: isize) -> isize { unsafe { __vmintop(VM_INT_OPS::hvmi_clear, i, 0, 0, 0) } }

/* Constants for virtual instruction parameters and return values */
pub const VM_TRANS_TYPE_LINEAR: i32 = 0;
pub const VM_TRANS_TYPE_TABLE: i32 = 1;
pub const VM_TLB_INVALIDATE_FALSE: i32 = 0;
pub const VM_TLB_INVALIDATE_TRUE: i32 = 1;
pub const VM_INT_DISABLE: i32 = 0;
pub const VM_INT_ENABLE: i32 = 1;
pub const VM_INT_UNMASK: i32 = 0;
pub const VM_INT_MASK: i32 = 1;
pub const VM_NEWMAP_TYPE_LINEAR: i32 = 0;
pub const VM_NEWMAP_TYPE_PGTABLES: i32 = 1;

/* Event Record definitions useful to both C and Assembler */
pub const HVM_VMEST_UM_SFT: i32 = 31;
pub const HVM_VMEST_UM_MSK: i32 = 1;
pub const HVM_VMEST_IE_SFT: i32 = 30;
pub const HVM_VMEST_IE_MSK: i32 = 1;
pub const HVM_VMEST_SS_SFT: i32 = 29;
pub const HVM_VMEST_SS_MSK: i32 = 1;
pub const HVM_VMEST_EVENTNUM_SFT: i32 = 16;
pub const HVM_VMEST_EVENTNUM_MSK: i32 = 0xff;
pub const HVM_VMEST_CAUSE_SFT: i32 = 0;
pub const HVM_VMEST_CAUSE_MSK: i32 = 0xffff;

/* The initial program gets to find a system environment descriptor on its stack when it begins execution. */
pub const HEXAGON_VM_SED_NULL: i32 = 0;

/* Event numbers for vector binding */
pub const HVM_EV_RESET: i32 = 0;
pub const HVM_EV_MACHCHECK: i32 = 1;
pub const HVM_EV_GENEX: i32 = 2;
pub const HVM_EV_TRAP: i32 = 8;
pub const HVM_EV_INTR: i32 = 15;
/* These shoud be nuked as soon as we know the VM is up to spec v0.1.1 */
pub const HVM_EV_INTR_0: i32 = 16;
pub const HVM_MAX_INTR: i32 = 240;

/* Cause values for General Exception */
pub const HVM_GE_C_BUS: i32 = 0x01;
pub const HVM_GE_C_XPROT: i32 = 0x11;
pub const HVM_GE_C_XUSER: i32 = 0x14;
pub const HVM_GE_C_INVI: i32 = 0x15;
pub const HVM_GE_C_PRIVI: i32 = 0x1B;
pub const HVM_GE_C_XMAL: i32 = 0x1C;
pub const HVM_GE_C_WREG: i32 = 0x1D;
pub const HVM_GE_C_PCAL: i32 = 0x1E;
pub const HVM_GE_C_RMAL: i32 = 0x20;
pub const HVM_GE_C_WMAL: i32 = 0x21;
pub const HVM_GE_C_RPROT: i32 = 0x22;
pub const HVM_GE_C_WPROT: i32 = 0x23;
pub const HVM_GE_C_RUSER: i32 = 0x24;
pub const HVM_GE_C_WUSER: i32 = 0x25;
pub const HVM_GE_C_CACHE: i32 = 0x28;

/* Cause codes for Machine Check */
pub const HVM_MCHK_C_DOWN: i32 = 0x00;
pub const HVM_MCHK_C_BADSP: i32 = 0x01;
pub const HVM_MCHK_C_BADEX: i32 = 0x02;
pub const HVM_MCHK_C_BADPT: i32 = 0x03;
pub const HVM_MCHK_C_REGWR: i32 = 0x29;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
