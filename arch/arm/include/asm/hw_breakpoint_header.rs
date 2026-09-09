/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are active in the kernel build (__KERNEL__).
// CONFIG_HAVE_HW_BREAKPOINT selects the hardware-breakpoint implementation.

use core::ffi::{c_ulong, c_void};

pub struct task_struct;

#[repr(C)]
pub struct arch_hw_breakpoint_ctrl {
    // C bitfields: __reserved:9, mismatch:1, unnamed:9, len:8,
    // type:2, privilege:2, enabled:1.
    pub __reserved: u32,
    pub mismatch: u32,
    pub len: u32,
    pub type_: u32,
    pub privilege: u32,
    pub enabled: u32,
}

#[repr(C)]
pub struct arch_hw_breakpoint {
    pub address: u32,
    pub trigger: u32,
    pub step_ctrl: arch_hw_breakpoint_ctrl,
    pub ctrl: arch_hw_breakpoint_ctrl,
}

#[inline]
pub fn encode_ctrl_reg(ctrl: arch_hw_breakpoint_ctrl) -> u32 {
    (ctrl.mismatch << 22)
        | (ctrl.len << 5)
        | (ctrl.type_ << 3)
        | (ctrl.privilege << 1)
        | ctrl.enabled
}

#[inline]
pub unsafe fn decode_ctrl_reg(reg: u32, ctrl: *mut arch_hw_breakpoint_ctrl) {
    (*ctrl).enabled = reg & 0x1;
    let mut reg = reg >> 1;
    (*ctrl).privilege = reg & 0x3;
    reg >>= 2;
    (*ctrl).type_ = reg & 0x3;
    reg >>= 2;
    (*ctrl).len = reg & 0xff;
    reg >>= 17;
    (*ctrl).mismatch = reg & 0x1;
}

/* Debug architecture numbers. */
pub const ARM_DEBUG_ARCH_RESERVED: u32 = 0; /* In case of ptrace ABI updates. */
pub const ARM_DEBUG_ARCH_V6: u32 = 1;
pub const ARM_DEBUG_ARCH_V6_1: u32 = 2;
pub const ARM_DEBUG_ARCH_V7_ECP14: u32 = 3;
pub const ARM_DEBUG_ARCH_V7_MM: u32 = 4;
pub const ARM_DEBUG_ARCH_V7_1: u32 = 5;
pub const ARM_DEBUG_ARCH_V8: u32 = 6;
pub const ARM_DEBUG_ARCH_V8_1: u32 = 7;
pub const ARM_DEBUG_ARCH_V8_2: u32 = 8;
pub const ARM_DEBUG_ARCH_V8_4: u32 = 9;

/* Breakpoint */
pub const ARM_BREAKPOINT_EXECUTE: u32 = 0;

/* Watchpoints */
pub const ARM_BREAKPOINT_LOAD: u32 = 1;
pub const ARM_BREAKPOINT_STORE: u32 = 2;
pub const ARM_FSR_ACCESS_MASK: u32 = 1 << 11;

/* Privilege Levels */
pub const ARM_BREAKPOINT_PRIV: u32 = 1;
pub const ARM_BREAKPOINT_USER: u32 = 2;

/* Lengths */
pub const ARM_BREAKPOINT_LEN_1: u32 = 0x1;
pub const ARM_BREAKPOINT_LEN_2: u32 = 0x3;
pub const ARM_BREAKPOINT_LEN_4: u32 = 0xf;
pub const ARM_BREAKPOINT_LEN_8: u32 = 0xff;

/* Limits */
pub const ARM_MAX_BRP: u32 = 16;
pub const ARM_MAX_WRP: u32 = 16;
pub const ARM_MAX_HBP_SLOTS: u32 = ARM_MAX_BRP + ARM_MAX_WRP;

/* DSCR method of entry bits. */
#[inline]
pub const fn ARM_DSCR_MOE(x: u32) -> u32 { (x >> 2) & 0xf }
pub const ARM_ENTRY_BREAKPOINT: u32 = 0x1;
pub const ARM_ENTRY_ASYNC_WATCHPOINT: u32 = 0x2;
pub const ARM_ENTRY_CFI_BREAKPOINT: u32 = 0x3;
pub const ARM_ENTRY_SYNC_WATCHPOINT: u32 = 0xa;

/* DSCR monitor/halting bits. */
pub const ARM_DSCR_HDBGEN: u32 = 1 << 14;
pub const ARM_DSCR_MDBGEN: u32 = 1 << 15;

/* OSLSR os lock model bits */
pub const ARM_OSLSR_OSLM0: u32 = 1 << 0;

/* opcode2 numbers for the co-processor instructions. */
pub const ARM_OP2_BVR: u32 = 4;
pub const ARM_OP2_BCR: u32 = 5;
pub const ARM_OP2_WVR: u32 = 6;
pub const ARM_OP2_WCR: u32 = 7;

/* Base register numbers for the debug registers. */
pub const ARM_BASE_BVR: u32 = 64;
pub const ARM_BASE_BCR: u32 = 80;
pub const ARM_BASE_WVR: u32 = 96;
pub const ARM_BASE_WCR: u32 = 112;

/* ARM coprocessor accessor macros; their inline assembly is architecture/build specific. */

pub struct perf_event_attr;
pub struct notifier_block;
pub struct perf_event;
pub struct pmu;

extern "C" {
    pub fn arch_bp_generic_fields(ctrl: arch_hw_breakpoint_ctrl, gen_len: *mut i32, gen_type: *mut i32) -> i32;
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_exceptions_notify(unused: *mut notifier_block, val: c_ulong, data: *mut c_void) -> i32;
    pub fn arch_get_debug_arch() -> u8;
    pub fn arch_get_max_wp_len() -> u8;
    pub fn clear_ptrace_hw_breakpoint(tsk: *mut task_struct);
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32;
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
    pub fn hw_breakpoint_slots(type_: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
