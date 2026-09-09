// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by other translation units.

/// This is allocated by cpu_suspend_init(), and used to store a pointer to
/// the 'struct sleep_stack_data' the contains a particular CPUs state.
pub static mut sleep_save_stash: *mut u64 = core::ptr::null_mut();

/// This hook is provided so that cpu_suspend code can restore HW
/// breakpoints as early as possible in the resume path, before reenabling
/// debug exceptions. Code cannot be run from a CPU PM notifier since by the
/// time the notifier runs debug exceptions might have been enabled already,
/// with HW breakpoints registers content still in an unknown state.
static mut hw_breakpoint_restore: Option<unsafe extern "C" fn(u32) -> i32> = None;

extern "C" {
    fn WARN_ON(condition: bool) -> bool;
    fn smp_processor_id() -> u32;
    fn mte_suspend_exit();
    fn cpu_uninstall_idmap();
    fn system_supports_cnp() -> bool;
    fn cpu_enable_swapper_cnp();
    fn alternative_has_cap_unlikely(cap: i32) -> bool;
    fn set_pstate_dit(value: i32);
    fn __uaccess_enable_hw_pan();
    fn spectre_v4_enable_mitigation(arg: *mut core::ffi::c_void);
    fn sme_suspend_exit();
    fn ptrauth_suspend_exit();
    fn system_capabilities_finalized() -> bool;
    fn mte_suspend_enter();
    fn local_daif_save() -> u64;
    fn pause_graph_tracing();
    fn ct_cpuidle_enter();
    fn __cpu_suspend_enter(state: *mut sleep_stack_data) -> bool;
    fn ct_cpuidle_exit();
    fn __cpu_suspend_exit();
    fn unpause_graph_tracing();
    fn local_daif_restore(flags: u64);
    fn fn(arg: u64) -> i32;
    fn mpidr_hash_size() -> usize;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut u64;
}

#[repr(C)]
pub struct sleep_stack_data {
    _private: [u8; 0],
}

const ARM64_HAS_DIT: i32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EOPNOTSUPP: i32 = 95;

pub unsafe extern "C" fn cpu_suspend_set_dbg_restorer(
    hw_bp_restore: Option<unsafe extern "C" fn(u32) -> i32>,
) {
    /* Prevent multiple restore hook initializations */
    if WARN_ON(hw_breakpoint_restore.is_some()) {
        return;
    }
    hw_breakpoint_restore = hw_bp_restore;
}

pub unsafe extern "C" fn __cpu_suspend_exit() {
    let cpu: u32 = smp_processor_id();

    mte_suspend_exit();
    cpu_uninstall_idmap();

    /* Restore CnP bit in TTBR1_EL1 */
    if system_supports_cnp() {
        cpu_enable_swapper_cnp();
    }

    if alternative_has_cap_unlikely(ARM64_HAS_DIT) {
        set_pstate_dit(1);
    }
    __uaccess_enable_hw_pan();

    if let Some(restore) = hw_breakpoint_restore {
        restore(cpu);
    }

    spectre_v4_enable_mitigation(core::ptr::null_mut());
    sme_suspend_exit();
    ptrauth_suspend_exit();
}

pub unsafe extern "C" fn cpu_suspend(
    arg: u64,
    fn_: unsafe extern "C" fn(u64) -> i32,
) -> i32 {
    let mut ret: i32 = 0;
    let flags: u64;
    let mut state: sleep_stack_data = core::mem::zeroed();

    WARN_ON(!system_capabilities_finalized());
    mte_suspend_enter();
    flags = local_daif_save();
    pause_graph_tracing();
    ct_cpuidle_enter();

    if __cpu_suspend_enter(&mut state) {
        /* Call the suspend finisher */
        ret = fn_(arg);

        if ret == 0 {
            ret = -EOPNOTSUPP;
        }
        ct_cpuidle_exit();
    } else {
        ct_cpuidle_exit();
        __cpu_suspend_exit();
    }

    unpause_graph_tracing();
    local_daif_restore(flags);
    ret
}

unsafe extern "C" fn cpu_suspend_init() -> i32 {
    /* ctx_ptr is an array of physical addresses */
    sleep_save_stash = kcalloc(mpidr_hash_size(), core::mem::size_of::<u64>(), GFP_KERNEL);

    if WARN_ON(sleep_save_stash.is_null()) {
        return -ENOMEM;
    }

    0
}

// early_initcall(cpu_suspend_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
