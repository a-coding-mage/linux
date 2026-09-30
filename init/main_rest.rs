// SPDX-License-Identifier: GPL-2.0-only
//! Create init and kthreadd, then hand the boot task to the idle loop.

use super::{bindings, main_completion, main_globals, main_kernel_init};
use core::ptr;
use kernel::ffi::{c_uint, c_ulong};

/// Read the stable boot CPU using the original architecture/configuration path.
///
/// # Safety
/// The boot task is pinned or preemption is disabled as required by the
/// original smp_processor_id contract; per-CPU/current-task state is live.
pub(super) unsafe fn boot_cpu_id() -> c_uint {
    #[cfg(CONFIG_DEBUG_PREEMPT)]
    // SAFETY: retain the original debug accessor and its preemption checks.
    return unsafe { bindings::debug_smp_processor_id() };

    #[cfg(all(not(CONFIG_DEBUG_PREEMPT), not(CONFIG_SMP)))]
    return 0;

    #[cfg(all(not(CONFIG_DEBUG_PREEMPT), CONFIG_SMP, CONFIG_X86_64))]
    {
        let cpu: c_uint;
        // SAFETY: the canonical x86 __this_cpu_read reads the 32-bit per-CPU
        // cpu_number without volatile semantics. GS names this CPU's region.
        unsafe {
            core::arch::asm!("mov {cpu:e}, gs:[{number}]",
                cpu = out(reg) cpu, number = sym bindings::cpu_number,
                options(pure, readonly, nostack, preserves_flags));
        }
        return cpu;
    }

    #[cfg(all(not(CONFIG_DEBUG_PREEMPT), CONFIG_SMP, CONFIG_ARM64))]
    // SAFETY: ARM64's original accessor reads current_thread_info()->cpu.
    return unsafe { ptr::addr_of!((*kernel::bindings::get_current()).thread_info.cpu).read() };

    #[cfg(all(
        not(CONFIG_DEBUG_PREEMPT),
        CONFIG_SMP,
        not(any(CONFIG_X86_64, CONFIG_ARM64))
    ))]
    compile_error!("rest_init requires the original stable CPU accessor for this architecture");
}

/// Return the canonical immutable single-CPU mask without allocating a mask.
///
/// # Safety
/// `cpu` is a valid configured CPU and cpu_bit_bitmap is initialized.
pub(super) unsafe fn boot_cpu_mask(cpu: c_uint) -> *const bindings::cpumask {
    let bits = c_ulong::BITS as usize;
    let row_words = core::mem::size_of::<bindings::cpumask>() / core::mem::size_of::<c_ulong>();
    // The canonical table has BITS_PER_LONG+1 rows, with the same word count
    // as cpumask. get_cpu_mask selects the bit's row and backs up by its word
    // index, using the preceding rows as the required leading zero padding.
    ptr::addr_of!(bindings::cpu_bit_bitmap)
        .cast::<c_ulong>()
        .wrapping_add((1 + cpu as usize % bits) * row_words)
        .wrapping_sub(cpu as usize / bits)
        .cast()
}

/// Start the two original boot tasks and enter the idle loop without returning.
///
/// # Safety
/// The caller is the initial boot task and has completed start_kernel through
/// its rest_init handoff, with preemption disabled and the boot CPU stable.
/// Kernel process creation, RCU, CPU masks and completion state are initialized.
#[inline(never)]
#[link_section = ".ref.text"]
pub(super) unsafe fn rest_init() -> ! {
    // SAFETY: kernel_clone_args consists of integers, bitfields, raw pointers
    // and nullable function pointers; zero initializes the omitted C members.
    let mut arguments: bindings::kernel_clone_args = unsafe { core::mem::zeroed() };
    arguments.flags = (bindings::CLONE_VM | bindings::CLONE_UNTRACED) as _;
    arguments.fn_ = Some(main_kernel_init::kernel_init);
    arguments.fn_arg = ptr::null_mut();

    // SAFETY: the caller establishes the original serialized boot phase. Both
    // task lookups occur under the actual RCU helper pair, and no task pointer
    // references survive across calls; field mutation uses raw pointers only.
    unsafe {
        bindings::rcu_scheduler_starting();
        let init_pid = bindings::kernel_clone(ptr::addr_of_mut!(arguments));
        kernel::bindings::rcu_read_lock();
        let init_task =
            bindings::find_task_by_pid_ns(init_pid, ptr::addr_of_mut!(bindings::init_pid_ns));
        let flags = ptr::addr_of_mut!((*init_task).flags);
        flags.write(flags.read() | bindings::PF_NO_SETAFFINITY);
        bindings::set_cpus_allowed_ptr(init_task, boot_cpu_mask(boot_cpu_id()));
        kernel::bindings::rcu_read_unlock();

        #[cfg(CONFIG_NUMA)]
        bindings::numa_default_policy();
        let thread_pid = bindings::kernel_thread(
            Some(bindings::kthreadd),
            ptr::null_mut(),
            ptr::null(),
            (bindings::CLONE_FS | bindings::CLONE_FILES) as c_ulong,
        );
        kernel::bindings::rcu_read_lock();
        bindings::kthreadd_task =
            bindings::find_task_by_pid_ns(thread_pid, ptr::addr_of_mut!(bindings::init_pid_ns));
        kernel::bindings::rcu_read_unlock();

        main_globals::system_state = bindings::system_states_SYSTEM_SCHEDULING;
        bindings::complete(ptr::addr_of_mut!(main_completion::kthreadd_done));
        bindings::schedule_preempt_disabled();
        bindings::cpu_startup_entry(bindings::cpuhp_state_CPUHP_ONLINE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
