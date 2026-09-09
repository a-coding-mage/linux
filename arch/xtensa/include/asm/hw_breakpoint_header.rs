/*
 * Xtensa hardware breakpoints/watchpoints handling functions
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2016 Cadence Design Systems Inc.
 */

/* C conditional: CONFIG_HAVE_HW_BREAKPOINT */
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
mod have_hw_breakpoint {
    /* Breakpoint */
    pub const XTENSA_BREAKPOINT_EXECUTE: i32 = 0;

    /* Watchpoints */
    pub const XTENSA_BREAKPOINT_LOAD: i32 = 1;
    pub const XTENSA_BREAKPOINT_STORE: i32 = 2;

    #[repr(C)]
    pub struct ArchHwBreakpoint {
        pub address: usize,
        pub len: u16,
        pub type_: u16,
    }

    pub enum PerfEventAttr {}
    pub enum PerfEvent {}
    pub enum PtRegs {}
    pub enum TaskStruct {}
    pub enum NotifierBlock {}

    unsafe extern "C" {
        pub fn hw_breakpoint_slots(type_: i32) -> i32;
        pub fn arch_check_bp_in_kernelspace(hw: *mut ArchHwBreakpoint) -> i32;
        pub fn hw_breakpoint_arch_parse(
            bp: *mut PerfEvent,
            attr: *const PerfEventAttr,
            hw: *mut ArchHwBreakpoint,
        ) -> i32;
        pub fn hw_breakpoint_exceptions_notify(
            unused: *mut NotifierBlock,
            val: usize,
            data: *mut core::ffi::c_void,
        ) -> i32;

        pub fn arch_install_hw_breakpoint(bp: *mut PerfEvent) -> i32;
        pub fn arch_uninstall_hw_breakpoint(bp: *mut PerfEvent);
        pub fn hw_breakpoint_pmu_read(bp: *mut PerfEvent);
        pub fn check_hw_breakpoint(regs: *mut PtRegs) -> i32;
        pub fn clear_ptrace_hw_breakpoint(tsk: *mut TaskStruct);
        pub fn restore_dbreak();
    }
}

/* C conditional: !CONFIG_HAVE_HW_BREAKPOINT */
#[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
pub enum TaskStruct {}

#[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
#[inline]
pub unsafe fn clear_ptrace_hw_breakpoint(_tsk: *mut TaskStruct) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
