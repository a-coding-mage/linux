// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux/RISC-V kernel environment are referenced
// below but are not defined in this translation unit.

use core::ffi::c_void;

// DEFINE_PER_CPU(u32, cpu_srmcfg) = U32_MAX;
#[no_mangle]
pub static mut cpu_srmcfg: u32 = u32::MAX;

// DEFINE_PER_CPU(u32, cpu_srmcfg_default);
#[no_mangle]
pub static mut cpu_srmcfg_default: u32 = 0;

extern "C" {
    fn __switch_to_srmcfg(task: *mut c_void);
    fn riscv_has_extension_unlikely(extension: u32) -> bool;
    fn cpuhp_setup_state(
        state: u32,
        name: *const i8,
        startup: unsafe extern "C" fn(u32) -> i32,
        teardown: unsafe extern "C" fn(u32) -> i32,
    ) -> i32;
    fn cpu_pm_register_notifier(nb: *mut notifier_block) -> i32;
    static mut current: *mut c_void;
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>,
}

const U32_MAX: u32 = u32::MAX;
const CPU_PM_EXIT: usize = 0;
const CPU_PM_ENTER_FAILED: usize = 0;
const NOTIFY_OK: i32 = 0;
const CPUHP_AP_ONLINE_DYN: u32 = 0;
const RISCV_ISA_EXT_SSQOSID: u32 = 0;

/*
 * Invalidate the per-CPU srmcfg cache. Used as both the cpuhp startup
 * and teardown callback.
 */
#[no_mangle]
pub unsafe extern "C" fn riscv_srmcfg_reset_cache(_cpu: u32) -> i32 {
    cpu_srmcfg = U32_MAX;
    0
}

/*
 * CPU PM notifier: invalidate the cached srmcfg on resume from a deep
 * idle / suspend.
 */
#[no_mangle]
pub unsafe extern "C" fn riscv_srmcfg_pm_notify(
    _nb: *mut notifier_block,
    action: usize,
    _unused: *mut c_void,
) -> i32 {
    match action {
        CPU_PM_EXIT | CPU_PM_ENTER_FAILED => {
            cpu_srmcfg = U32_MAX;
            __switch_to_srmcfg(current);
        }
        _ => {}
    }
    NOTIFY_OK
}

#[no_mangle]
pub static mut riscv_srmcfg_pm_nb: notifier_block = notifier_block {
    notifier_call: Some(riscv_srmcfg_pm_notify),
};

#[no_mangle]
pub unsafe extern "C" fn riscv_srmcfg_init() -> i32 {
    let mut err: i32;

    if !riscv_has_extension_unlikely(RISCV_ISA_EXT_SSQOSID) {
        return 0;
    }

    err = cpuhp_setup_state(
        CPUHP_AP_ONLINE_DYN,
        b"riscv/srmcfg:online\0".as_ptr() as *const i8,
        riscv_srmcfg_reset_cache,
        riscv_srmcfg_reset_cache,
    );
    if err < 0 {
        // pr_warn("srmcfg: cpuhp setup failed (%d), cache not invalidated on CPU online\n", err);
    }

    cpu_pm_register_notifier(&raw mut riscv_srmcfg_pm_nb);
    0
}

// arch_initcall(riscv_srmcfg_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
