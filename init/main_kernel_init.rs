// SPDX-License-Identifier: GPL-2.0-only
//! Final kernel-init lifecycle, after all `kernel_init_freeable` work completes.

use super::{
    bindings, main_bootconfig, main_completion, main_exec, main_freeable, main_globals, main_rodata,
};
use kernel::ffi::{c_int, c_void};

/// Run the original init task through completion of boot and userspace exec.
///
/// # Safety
/// This is the init task's entry point. Boot has established the original
/// scheduler, filesystem, completion and init-memory prerequisites.
#[link_section = ".ref.text"]
pub(super) unsafe extern "C" fn kernel_init(_unused: *mut c_void) -> c_int {
    // SAFETY: This task waits for kthreadd before freeable initialization, then
    // completes the lifecycle from code that survives freeing init memory.
    unsafe {
        bindings::init_userspace_fs();
        bindings::wait_for_completion(core::ptr::addr_of_mut!(main_completion::kthreadd_done));
        main_freeable::kernel_init_freeable();
        finish_kernel_init()
    }
}

/// Drain init work, release init memory, finalize protection and enter userspace.
///
/// This function must remain outside init memory: it executes across
/// `free_initmem`, exactly as the original `__ref kernel_init` does.
///
/// # Safety
///
/// The caller is the init task, has waited for kthreadd and completed
/// `kernel_init_freeable`, and owns the original serialized boot transition.
#[link_section = ".ref.text"]
pub(super) unsafe fn finish_kernel_init() -> c_int {
    // SAFETY: The caller establishes the original kernel_init phase. In
    // particular, asynchronous init users finish before init memory is freed.
    unsafe {
        bindings::async_synchronize_full();

        main_globals::system_state = bindings::system_states_SYSTEM_FREEING_INITMEM;
        #[cfg(CONFIG_KPROBES)]
        bindings::kprobe_free_init_mem();
        #[cfg(all(CONFIG_FUNCTION_TRACER, CONFIG_DYNAMIC_FTRACE))]
        bindings::ftrace_free_init_mem();
        // The non-dynamic inline implementation still takes a boot snapshot.
        #[cfg(all(
            CONFIG_FUNCTION_TRACER,
            not(CONFIG_DYNAMIC_FTRACE),
            CONFIG_TRACER_SNAPSHOT
        ))]
        bindings::ftrace_boot_snapshot();
        #[cfg(CONFIG_KGDB)]
        bindings::kgdb_free_init_mem();
        main_bootconfig::exit_boot_config();
        // Resolve the public symbol, preserving architecture overrides of the
        // staged weak default instead of calling its implementation directly.
        bindings::free_initmem();
        main_rodata::mark_readonly();

        #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
        bindings::pti_finalize();

        main_globals::system_state = bindings::system_states_SYSTEM_RUNNING;
        #[cfg(CONFIG_NUMA)]
        bindings::numa_default_policy();
        #[cfg(CONFIG_TREE_RCU)]
        bindings::rcu_end_inkernel_boot();
        #[cfg(CONFIG_SYSCTL)]
        bindings::do_sysctl_args();

        main_exec::execute_init_processes()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
