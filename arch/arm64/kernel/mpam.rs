// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Arm Ltd. */

// Dependency supplied by <asm/mpam.h>.
// Dependency supplied by <linux/arm_mpam.h>.
// Dependency supplied by <linux/cpu_pm.h>.
// Dependency supplied by <linux/jump_label.h>.
// Dependency supplied by <linux/percpu.h>.

pub static mut mpam_enabled: bool = false;

// DEFINE_PER_CPU(u64, arm64_mpam_default);
pub static mut arm64_mpam_default: u64 = 0;
// DEFINE_PER_CPU(u64, arm64_mpam_current);
pub static mut arm64_mpam_current: u64 = 0;

pub static mut arm64_mpam_global_default: u64 = 0;

extern "C" {
    fn smp_processor_id() -> i32;
    fn read_sanitised_ftr_reg(reg: u32) -> u64;
    fn system_supports_sme() -> bool;
    fn system_supports_mpam() -> bool;
    fn cpu_pm_register_notifier(nb: *mut notifier_block) -> i32;
    fn mpam_register_requestor(partid_max: u16, pmg_max: u8) -> i32;
    fn write_sysreg_s(value: u64, reg: u32);
    fn isb();
}

extern "C" {
    static MPAM1_EL1_MPAMEN: u64;
    static MPAMSM_EL1_PARTID_D: u64;
    static MPAMSM_EL1_PMG_D: u64;
    static SYS_MPAM1_EL1: u32;
    static SYS_MPAMSM_EL1: u32;
    static SYS_MPAM0_EL1: u32;
    static SYS_MPAMIDR_EL1: u32;
    static MPAMIDR_EL1_PARTID_MAX: u64;
    static MPAMIDR_EL1_PMG_MAX: u64;
    static CPU_PM_EXIT: u64;
    static NOTIFY_OK: i32;
    static NOTIFY_DONE: i32;
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut core::ffi::c_void) -> i32>,
}

unsafe extern "C" fn mpam_pm_notifier(
    _self: *mut notifier_block,
    cmd: u64,
    _v: *mut core::ffi::c_void,
) -> i32 {
    let mut regval: u64;
    let cpu = smp_processor_id();

    match cmd {
        CPU_PM_EXIT => {
            /*
             * Don't use mpam_thread_switch() as the system register
             * value has changed under our feet.
             */
            // READ_ONCE(per_cpu(arm64_mpam_current, cpu));
            regval = core::ptr::read_volatile(&arm64_mpam_current);
            write_sysreg_s(regval | MPAM1_EL1_MPAMEN, SYS_MPAM1_EL1);
            if system_supports_sme() {
                write_sysreg_s(
                    regval & (MPAMSM_EL1_PARTID_D | MPAMSM_EL1_PMG_D),
                    SYS_MPAMSM_EL1,
                );
            }
            isb();

            write_sysreg_s(regval, SYS_MPAM0_EL1);

            NOTIFY_OK
        }
        _ => NOTIFY_DONE,
    }
}

static mut mpam_pm_nb: notifier_block = notifier_block {
    notifier_call: Some(mpam_pm_notifier),
};

unsafe extern "C" fn arm64_mpam_register_cpus() -> i32 {
    let mpamidr = read_sanitised_ftr_reg(SYS_MPAMIDR_EL1);
    let partid_max = ((mpamidr & MPAMIDR_EL1_PARTID_MAX) >> MPAMIDR_EL1_PARTID_MAX.trailing_zeros()) as u16;
    let pmg_max = ((mpamidr & MPAMIDR_EL1_PMG_MAX) >> MPAMIDR_EL1_PMG_MAX.trailing_zeros()) as u8;

    if !system_supports_mpam() {
        return 0;
    }

    cpu_pm_register_notifier(&raw mut mpam_pm_nb);
    mpam_register_requestor(partid_max, pmg_max)
}

/* Must occur before mpam_msc_driver_init() from subsys_initcall() */
// arch_initcall(arm64_mpam_register_cpus)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
