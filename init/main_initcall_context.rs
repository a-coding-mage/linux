// SPDX-License-Identifier: GPL-2.0-only
//! Native preemption counter and IRQ repair for the staged initcall owner.
//!
//! The x86-64 and ARM64 implementations follow their actual architecture
//! headers. Configurations requiring paravirtual IRQ operations or ARM priority
//! masking remain explicit build errors until those paths are translated.

use super::bindings;
use kernel::ffi::c_int;

#[cfg(not(any(CONFIG_X86_64, CONFIG_ARM64)))]
compile_error!("initcall context requires the original architecture implementation");
#[cfg(all(CONFIG_X86_64, CONFIG_PARAVIRT_XXL))]
compile_error!("initcall context requires paravirtual IRQ operations");
#[cfg(all(CONFIG_ARM64, CONFIG_ARM64_PSEUDO_NMI))]
compile_error!("initcall context requires ARM64 priority masking operations");

/// Read the current CPU's preemption count, excluding its reschedule state.
///
/// # Safety
/// Kernel per-CPU/current-task access must be available in this context.
#[cfg(CONFIG_X86_64)]
pub(super) unsafe fn preempt_count() -> c_int {
    let count: usize;
    // SAFETY: __preempt_count is the canonical per-CPU unsigned-long owner.
    // A volatile asm read matches raw_cpu_read without an implicit guard.
    unsafe {
        #[cfg(CONFIG_SMP)]
        core::arch::asm!("mov {count}, gs:[{counter}]", count = out(reg) count,
            counter = sym bindings::__preempt_count, options(nostack, preserves_flags));
        #[cfg(not(CONFIG_SMP))]
        core::arch::asm!("mov {count}, [{counter}]", count = out(reg) count,
            counter = sym bindings::__preempt_count, options(nostack, preserves_flags));
    }
    (count & !(bindings::RUST_INIT_MAIN_PREEMPT_NEED_RESCHED as usize)) as c_int
}

/// Restore the count without overwriting a concurrent reschedule-bit change.
///
/// # Safety
/// Kernel per-CPU access must be available, and `count` is a saved count for
/// this initcall context. This operation does not acquire a preemption guard.
#[cfg(CONFIG_X86_64)]
pub(super) unsafe fn preempt_count_set(count: c_int) {
    let mut old: usize;
    // SAFETY: the unprefixed per-CPU cmpxchg matches raw_cpu_try_cmpxchg_8,
    // including retrying if an interrupt changes the same CPU's counter.
    unsafe {
        #[cfg(CONFIG_SMP)]
        core::arch::asm!("mov {old}, gs:[{counter}]", old = out(reg) old,
            counter = sym bindings::__preempt_count, options(nostack, preserves_flags));
        #[cfg(not(CONFIG_SMP))]
        core::arch::asm!("mov {old}, [{counter}]", old = out(reg) old,
            counter = sym bindings::__preempt_count, options(nostack, preserves_flags));
        loop {
            let next = (old & bindings::RUST_INIT_MAIN_PREEMPT_NEED_RESCHED as usize)
                | ((count as usize) & !(bindings::RUST_INIT_MAIN_PREEMPT_NEED_RESCHED as usize));
            let success: u8;
            #[cfg(CONFIG_SMP)]
            core::arch::asm!("cmpxchg gs:[{counter}], {next}", "sete {success}",
                counter = sym bindings::__preempt_count, next = in(reg) next,
                inout("rax") old, success = out(reg_byte) success, options(nostack));
            #[cfg(not(CONFIG_SMP))]
            core::arch::asm!("cmpxchg [{counter}], {next}", "sete {success}",
                counter = sym bindings::__preempt_count, next = in(reg) next,
                inout("rax") old, success = out(reg_byte) success, options(nostack));
            if success != 0 {
                break;
            }
        }
    }
}

// Both supported 64-bit architectures select separate reschedule bits. Do not
// silently use an eight-byte operation for a different counter representation.
#[cfg(not(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS))]
compile_error!("initcall context requires the original narrow preemption counter operations");

/// Read only ARM64's count member, leaving its separate reschedule member alone.
///
/// # Safety
/// `current` and its embedded thread_info are live in this kernel context.
#[cfg(CONFIG_ARM64)]
pub(super) unsafe fn preempt_count() -> c_int {
    // SAFETY: canonical ARM64 current/thread_info layout and READ_ONCE width.
    unsafe {
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        core::ptr::addr_of!((*current).thread_info.__bindgen_anon_1.preempt.count).read_volatile()
            as c_int
    }
}

/// Restore ARM64's count member with the original WRITE_ONCE operation.
///
/// # Safety
/// `current` is live and `count` is the saved value for this initcall context.
#[cfg(CONFIG_ARM64)]
pub(super) unsafe fn preempt_count_set(count: c_int) {
    // SAFETY: writing only count preserves the adjacent need_resched member.
    unsafe {
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        core::ptr::addr_of_mut!((*current).thread_info.__bindgen_anon_1.preempt.count)
            .write_volatile(count as u32);
    }
}

/// Query the real interrupt mask, even on a non-preemptible kernel.
///
/// # Safety
/// This executes the architecture's kernel interrupt-state instructions.
pub(super) unsafe fn irqs_disabled() -> bool {
    let flags: usize;
    // SAFETY: these are the original native save-flags instructions. The x86
    // push/pop uses the stack and deliberately has no `nostack` annotation.
    unsafe {
        #[cfg(CONFIG_X86_64)]
        core::arch::asm!("pushfq", "pop {flags}", flags = out(reg) flags,
            options(preserves_flags));
        #[cfg(CONFIG_ARM64)]
        core::arch::asm!("mrs {flags}, daif", flags = out(reg) flags,
            options(nostack, preserves_flags));
    }
    #[cfg(CONFIG_X86_64)]
    return flags & bindings::RUST_INIT_MAIN_IRQ_MASK as usize == 0;
    #[cfg(CONFIG_ARM64)]
    return flags & bindings::RUST_INIT_MAIN_IRQ_MASK as usize != 0;
}

/// Repair interrupt disablement, preserving the original tracing-before-enable
/// order independently of the configured preemption model.
///
/// # Safety
/// The caller may enable IRQs in this context.
pub(super) unsafe fn local_irq_enable() {
    // SAFETY: these operations implement local_irq_enable, including tracing.
    // The asm blocks retain memory clobbers, providing the native barriers.
    unsafe {
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        bindings::trace_hardirqs_on();
        #[cfg(CONFIG_X86_64)]
        core::arch::asm!("sti", options(nostack));
        #[cfg(CONFIG_ARM64)]
        core::arch::asm!("msr daifclr, #3", options(nostack, preserves_flags));
    }
}

/// Disable IRQs and notify tracing only when this changes their prior state.
///
/// # Safety
/// The caller may disable local interrupts in the current kernel context.
pub(super) unsafe fn local_irq_disable() {
    // SAFETY: preserve the original query/disable/trace ordering. The native
    // instructions retain memory clobbers, as in their architecture headers.
    unsafe {
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        let was_disabled = irqs_disabled();
        #[cfg(CONFIG_X86_64)]
        core::arch::asm!("cli", options(nostack));
        #[cfg(CONFIG_ARM64)]
        core::arch::asm!("msr daifset, #3", options(nostack, preserves_flags));
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        if !was_disabled {
            bindings::trace_hardirqs_off();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
