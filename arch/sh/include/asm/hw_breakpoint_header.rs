/* SPDX-License-Identifier: GPL-2.0 */
// Translated from <uapi/asm/hw_breakpoint.h>, <linux/kdebug.h>, and
// <linux/types.h>.  The included declarations are supplied by dependencies.

#[repr(C)]
pub struct ArchHwBreakpoint {
    pub address: usize,
    pub len: u16,
    pub type_: u16,
}

pub const SH_BREAKPOINT_READ: u32 = 1 << 1;
pub const SH_BREAKPOINT_WRITE: u32 = 1 << 2;
pub const SH_BREAKPOINT_RW: u32 = SH_BREAKPOINT_READ | SH_BREAKPOINT_WRITE;

pub const SH_BREAKPOINT_LEN_1: u32 = 1 << 12;
pub const SH_BREAKPOINT_LEN_2: u32 = 1 << 13;
pub const SH_BREAKPOINT_LEN_4: u32 = SH_BREAKPOINT_LEN_1 | SH_BREAKPOINT_LEN_2;
pub const SH_BREAKPOINT_LEN_8: u32 = 1 << 14;

#[repr(C)]
pub struct ShUbc {
    pub name: *const core::ffi::c_char,
    pub num_events: u32,
    pub trap_nr: u32,
    pub enable: Option<unsafe extern "C" fn(*mut ArchHwBreakpoint, i32)>,
    pub disable: Option<unsafe extern "C" fn(*mut ArchHwBreakpoint, i32)>,
    pub enable_all: Option<unsafe extern "C" fn(usize)>,
    pub disable_all: Option<unsafe extern "C" fn()>,
    pub active_mask: Option<unsafe extern "C" fn() -> usize>,
    pub triggered_mask: Option<unsafe extern "C" fn() -> usize>,
    pub clear_triggered_mask: Option<unsafe extern "C" fn(usize)>,
    pub clk: *mut Clk, // optional interface clock / MSTP bit
}

#[repr(C)]
pub struct PerfEventAttr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PerfEvent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Pmu {
    _private: [u8; 0],
}

// Supplied by the Linux clock dependency.
#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

// Maximum number of UBC channels
pub const HBP_NUM: usize = 2;

#[inline]
pub const fn hw_breakpoint_slots<T>(_type: T) -> usize {
    HBP_NUM
}

unsafe extern "C" {
    pub fn arch_check_bp_in_kernelspace(hw: *mut ArchHwBreakpoint) -> i32;
    pub fn arch_bp_generic_fields(
        sh_len: i32,
        sh_type: i32,
        gen_len: *mut i32,
        gen_type: *mut i32,
    ) -> i32;
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

    pub fn arch_fill_perf_breakpoint(bp: *mut PerfEvent);
    pub fn register_sh_ubc(ubc: *mut ShUbc) -> i32;

    pub static mut perf_ops_bp: Pmu;
}

// Supplied by <linux/kdebug.h>.
#[repr(C)]
pub struct NotifierBlock {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
