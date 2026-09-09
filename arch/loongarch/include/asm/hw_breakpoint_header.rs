/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2022-2023 Loongson Technology Corporation Limited */

// Dependency supplied by asm/loongarch.h is intentionally external.

/* Breakpoint */
pub const LOONGARCH_BREAKPOINT_EXECUTE: u32 = 0 << 0;

/* Watchpoints */
pub const LOONGARCH_BREAKPOINT_LOAD: u32 = 1 << 0;
pub const LOONGARCH_BREAKPOINT_STORE: u32 = 1 << 1;

#[repr(C)]
pub struct arch_hw_breakpoint_ctrl {
    // C bit-field storage: __reserved:28, len:2, type:2.
    pub __reserved: u32,
    pub len: u32,
    pub type_: u32,
}

#[repr(C)]
pub struct arch_hw_breakpoint {
    pub address: u64,
    pub mask: u64,
    pub ctrl: arch_hw_breakpoint_ctrl,
}

/* Lengths */
pub const LOONGARCH_BREAKPOINT_LEN_1: u32 = 0b11;
pub const LOONGARCH_BREAKPOINT_LEN_2: u32 = 0b10;
pub const LOONGARCH_BREAKPOINT_LEN_4: u32 = 0b01;
pub const LOONGARCH_BREAKPOINT_LEN_8: u32 = 0b00;

/* Limits. Changing these will require modifications to the register accessors. */
pub const LOONGARCH_MAX_BRP: u32 = 14;
pub const LOONGARCH_MAX_WRP: u32 = 14;

/* Virtual debug register bases. */
pub const CSR_CFG_ADDR: u32 = 0;
pub const CSR_CFG_MASK: u32 = CSR_CFG_ADDR + LOONGARCH_MAX_BRP;
pub const CSR_CFG_CTRL: u32 = CSR_CFG_MASK + LOONGARCH_MAX_BRP;
pub const CSR_CFG_ASID: u32 = CSR_CFG_CTRL + LOONGARCH_MAX_WRP;

/* Debug register names. */
pub const LOONGARCH_CSR_NAME_ADDR: &str = "ADDR";
pub const LOONGARCH_CSR_NAME_MASK: &str = "MASK";
pub const LOONGARCH_CSR_NAME_CTRL: &str = "CTRL";
pub const LOONGARCH_CSR_NAME_ASID: &str = "ASID";

/* Accessor macros for the debug registers. */
#[macro_export]
macro_rules! LOONGARCH_CSR_WATCH_READ {
    ($n:ident, $reg:ident, $t:expr, $val:ident) => {
        if $t == 0 {
            $val = csr_read64(concat_idents!(LOONGARCH_CSR_, IB, $n, $reg));
        } else {
            $val = csr_read64(concat_idents!(LOONGARCH_CSR_, DB, $n, $reg));
        }
    };
}

#[macro_export]
macro_rules! LOONGARCH_CSR_WATCH_WRITE {
    ($n:ident, $reg:ident, $t:expr, $val:expr) => {
        if $t == 0 {
            csr_write64($val, concat_idents!(LOONGARCH_CSR_, IB, $n, $reg));
        } else {
            csr_write64($val, concat_idents!(LOONGARCH_CSR_, DB, $n, $reg));
        }
    };
}

/* Exact number */
pub const CSR_FWPC_NUM: u32 = 0x3f;
pub const CSR_MWPC_NUM: u32 = 0x3f;
pub const CTRL_PLV_ENABLE: u32 = 0x1e;
pub const CTRL_PLV0_ENABLE: u32 = 0x02;
pub const CTRL_PLV3_ENABLE: u32 = 0x10;
pub const MWPnCFG3_LoadEn: u32 = 8;
pub const MWPnCFG3_StoreEn: u32 = 9;
pub const MWPnCFG3_Type_mask: u32 = 0x3;
pub const MWPnCFG3_Size_mask: u32 = 0x3;

pub fn encode_ctrl_reg(ctrl: arch_hw_breakpoint_ctrl) -> u32 {
    (ctrl.len << 10) | (ctrl.type_ << 8)
}

pub unsafe fn decode_ctrl_reg(mut reg: u32, ctrl: *mut arch_hw_breakpoint_ctrl) {
    reg >>= 8;
    (*ctrl).type_ = reg & MWPnCFG3_Type_mask;
    reg >>= 2;
    (*ctrl).len = reg & MWPnCFG3_Size_mask;
}

pub struct task_struct;
pub struct notifier_block;
pub struct perf_event;
pub struct perf_event_attr;
pub struct pt_regs;

extern "C" {
    pub fn arch_bp_generic_fields(ctrl: arch_hw_breakpoint_ctrl, gen_len: *mut i32, gen_type: *mut i32) -> i32;
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_exceptions_notify(unused: *mut notifier_block, val: u64, data: *mut core::ffi::c_void) -> i32;
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32;
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
    pub fn hw_breakpoint_slots(type_: i32) -> i32;
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
    pub fn breakpoint_handler(regs: *mut pt_regs);
    pub fn watchpoint_handler(regs: *mut pt_regs);
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
extern "C" {
    pub fn ptrace_hw_copy_thread(task: *mut task_struct);
    pub fn hw_breakpoint_thread_switch(next: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn ptrace_hw_copy_thread(_task: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn hw_breakpoint_thread_switch(_next: *mut task_struct) {}

/* Determine number of BRP registers available. */
pub unsafe fn get_num_brps() -> i32 {
    (csr_read32(LOONGARCH_CSR_FWPC) & CSR_FWPC_NUM) as i32
}

/* Determine number of WRP registers available. */
pub unsafe fn get_num_wrps() -> i32 {
    (csr_read32(LOONGARCH_CSR_MWPC) & CSR_MWPC_NUM) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
