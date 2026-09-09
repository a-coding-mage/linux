/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

// Dependency supplied by asm/hw_breakpoint.h.
use crate::asm::hw_breakpoint::arch_hw_breakpoint;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
extern "C" {
    pub static mut __debugger: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_ipi: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_bpt: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_sstep: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_iabr_match: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_break_match: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
    pub static mut __debugger_fault_handler: Option<unsafe extern "C" fn(regs: *mut pt_regs) -> i32>;
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger(regs: *mut pt_regs) -> i32 {
    match __debugger {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_ipi(regs: *mut pt_regs) -> i32 {
    match __debugger_ipi {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_bpt(regs: *mut pt_regs) -> i32 {
    match __debugger_bpt {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_sstep(regs: *mut pt_regs) -> i32 {
    match __debugger_sstep {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_iabr_match(regs: *mut pt_regs) -> i32 {
    match __debugger_iabr_match {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_break_match(regs: *mut pt_regs) -> i32 {
    match __debugger_break_match {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE"))]
#[inline]
pub unsafe fn debugger_fault_handler(regs: *mut pt_regs) -> i32 {
    match __debugger_fault_handler {
        Some(handler) => handler(regs),
        None => 0,
    }
}

#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_ipi(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_bpt(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_sstep(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_iabr_match(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_break_match(_regs: *mut pt_regs) -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_DEBUGGER", feature = "CONFIG_KEXEC_CORE")))]
#[inline]
pub unsafe fn debugger_fault_handler(_regs: *mut pt_regs) -> i32 { 0 }

extern "C" {
    pub fn __set_breakpoint(nr: i32, brk: *mut arch_hw_breakpoint);
    pub fn suspend_breakpoints();
    pub fn restore_breakpoints();
    pub fn ppc_breakpoint_available() -> bool;
}

#[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
extern "C" {
    pub fn do_send_trap(
        regs: *mut pt_regs,
        address: usize,
        error_code: usize,
        brkpt: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
