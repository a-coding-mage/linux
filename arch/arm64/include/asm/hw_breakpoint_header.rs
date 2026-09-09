/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/cputype.h, asm/cpufeature.h, asm/sysreg.h, asm/virt.h

#[repr(C)]
pub struct arch_hw_breakpoint_ctrl {
    pub __reserved: u32,
    pub len: u32,
    pub r#type: u32,
    pub privilege: u32,
    pub enabled: u32,
}

#[repr(C)]
pub struct arch_hw_breakpoint {
    pub address: u64,
    pub trigger: u64,
    pub ctrl: arch_hw_breakpoint_ctrl,
}

/* Privilege Levels */
pub const AARCH64_BREAKPOINT_EL1: u32 = 1;
pub const AARCH64_BREAKPOINT_EL0: u32 = 2;

pub const DBG_HMC_HYP: u32 = 1 << 13;

pub unsafe fn encode_ctrl_reg(ctrl: arch_hw_breakpoint_ctrl) -> u32 {
    let mut val = (ctrl.len << 5) | (ctrl.r#type << 3) | (ctrl.privilege << 1) |
        ctrl.enabled;

    if is_kernel_in_hyp_mode() && ctrl.privilege == AARCH64_BREAKPOINT_EL1 {
        val |= DBG_HMC_HYP;
    }

    val
}

pub unsafe fn decode_ctrl_reg(mut reg: u32, ctrl: *mut arch_hw_breakpoint_ctrl) {
    (*ctrl).enabled = reg & 0x1;
    reg >>= 1;
    (*ctrl).privilege = reg & 0x3;
    reg >>= 2;
    (*ctrl).r#type = reg & 0x3;
    reg >>= 2;
    (*ctrl).len = reg & 0xff;
}

/* Breakpoint */
pub const ARM_BREAKPOINT_EXECUTE: u32 = 0;

/* Watchpoints */
pub const ARM_BREAKPOINT_LOAD: u32 = 1;
pub const ARM_BREAKPOINT_STORE: u32 = 2;

/* Lengths */
pub const ARM_BREAKPOINT_LEN_1: u32 = 0x1;
pub const ARM_BREAKPOINT_LEN_2: u32 = 0x3;
pub const ARM_BREAKPOINT_LEN_3: u32 = 0x7;
pub const ARM_BREAKPOINT_LEN_4: u32 = 0xf;
pub const ARM_BREAKPOINT_LEN_5: u32 = 0x1f;
pub const ARM_BREAKPOINT_LEN_6: u32 = 0x3f;
pub const ARM_BREAKPOINT_LEN_7: u32 = 0x7f;
pub const ARM_BREAKPOINT_LEN_8: u32 = 0xff;

/* Kernel stepping */
pub const ARM_KERNEL_STEP_NONE: u32 = 0;
pub const ARM_KERNEL_STEP_ACTIVE: u32 = 1;
pub const ARM_KERNEL_STEP_SUSPEND: u32 = 2;

/*
 * Limits.
 * Changing these will require modifications to the register accessors.
 */
pub const ARM_MAX_BRP: u32 = 16;
pub const ARM_MAX_WRP: u32 = 16;

/* Virtual debug register bases. */
pub const AARCH64_DBG_REG_BVR: u32 = 0;
pub const AARCH64_DBG_REG_BCR: u32 = AARCH64_DBG_REG_BVR + ARM_MAX_BRP;
pub const AARCH64_DBG_REG_WVR: u32 = AARCH64_DBG_REG_BCR + ARM_MAX_BRP;
pub const AARCH64_DBG_REG_WCR: u32 = AARCH64_DBG_REG_WVR + ARM_MAX_WRP;

/* Debug register names. */
// AARCH64_DBG_REG_NAME_BVR = bvr
// AARCH64_DBG_REG_NAME_BCR = bcr
// AARCH64_DBG_REG_NAME_WVR = wvr
// AARCH64_DBG_REG_NAME_WCR = wcr

/* Accessor macros for the debug registers. */
// C token-pasting accessors are retained as intent; register accessors are supplied externally.

pub struct task_struct;
pub struct notifier_block;
pub struct perf_event_attr;
pub struct perf_event;
pub struct pmu;

extern "C" {
    pub fn arch_bp_generic_fields(ctrl: arch_hw_breakpoint_ctrl, gen_len: *mut i32,
                                  gen_type: *mut i32, offset: *mut i32) -> i32;
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr,
                                    hw: *mut arch_hw_breakpoint) -> i32;
    pub fn hw_breakpoint_exceptions_notify(unused: *mut notifier_block, val: u64,
                                           data: *mut core::ffi::c_void) -> i32;
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32;
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
    pub fn hw_breakpoint_slots(r#type: i32) -> i32;

    // Under CONFIG_HAVE_HW_BREAKPOINT these are external functions; otherwise they are empty inline functions.
    pub fn hw_breakpoint_thread_switch(next: *mut task_struct);
    pub fn ptrace_hw_copy_thread(task: *mut task_struct);
}

/* Determine number of BRP registers available. */
pub unsafe fn get_num_brps() -> i32 {
    let dfr0 = read_sanitised_ftr_reg(SYS_ID_AA64DFR0_EL1);
    1 + cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_BRPs_SHIFT) as i32
}

/* Determine number of WRP registers available. */
pub unsafe fn get_num_wrps() -> i32 {
    let dfr0 = read_sanitised_ftr_reg(SYS_ID_AA64DFR0_EL1);
    1 + cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_WRPs_SHIFT) as i32
}

// Under CONFIG_CPU_PM this is external; otherwise it is an empty inline function.
extern "C" {
    pub fn cpu_suspend_set_dbg_restorer(hw_bp_restore: Option<unsafe extern "C" fn(u32) -> i32>);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
