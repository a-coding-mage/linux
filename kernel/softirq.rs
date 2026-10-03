// SPDX-License-Identifier: GPL-2.0-only
// Algorithm owner translated from kernel/softirq.c at c8eeb0b98b0b.
// The original C file is preserved as the source and test oracle.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unused_mut,
    unused_macros,
    unused_unsafe,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/softirq_generated.rs"
    ));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_uint, c_ulong, c_void};

// x86_64 compiler/architecture boundary. Kbuild forces native frame pointers
// for this entire Rust object, including inspection/listing targets. Keeping
// these always-inline avoids attributing BH transitions to a helper's callsite.
#[inline(always)]
unsafe fn caller_frame() -> *const c_ulong {
    let frame: *const c_ulong;
    core::arch::asm!("mov {}, rbp", out(reg) frame,
        options(nostack, nomem, preserves_flags));
    frame
}
#[inline(always)]
unsafe fn caller_ip() -> c_ulong {
    *caller_frame().add(1)
}
#[inline(always)]
unsafe fn instruction_ip() -> c_ulong {
    let ip: c_ulong;
    core::arch::asm!("lea {}, [rip + 2f]", "2:", out(reg) ip,
        options(nostack, nomem, preserves_flags));
    ip
}
#[inline(always)]
unsafe fn is_lock_function(_ip: c_ulong) -> bool {
    #[cfg(any(CONFIG_SMP, CONFIG_DEBUG_SPINLOCK))]
    {
        in_lock_functions(_ip) != 0
    }
    #[cfg(not(any(CONFIG_SMP, CONFIG_DEBUG_SPINLOCK)))]
    {
        false
    }
}
#[inline(always)]
unsafe fn lock_parent_ip() -> c_ulong {
    // ftrace.h::get_lock_parent_ip, including its zero-valued CALLER_ADDR1/2
    // policy when the surrounding kernel has no CONFIG_FRAME_POINTER.
    let frame = caller_frame();
    let first = *frame.add(1);
    if !is_lock_function(first) {
        return first;
    }
    #[cfg(CONFIG_FRAME_POINTER)]
    {
        let parent = *frame as *const c_ulong;
        let second = *parent.add(1);
        if !is_lock_function(second) {
            return second;
        }
        let grandparent = *parent as *const c_ulong;
        *grandparent.add(1)
    }
    #[cfg(not(CONFIG_FRAME_POINTER))]
    {
        0
    }
}

include!("softirq_layout.rs");
include!("softirq_storage.rs");
include!("softirq_header.rs");
include!("softirq_bh.rs");
include!("softirq_tasklet.rs");

#[inline(always)]
unsafe fn current_task() -> *mut task_struct {
    lupos_sirq_current()
}
#[inline(always)]
unsafe fn softirq_count() -> c_uint {
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        (*current_task()).softirq_disable_cnt as c_uint & RUST_SIRQ_SOFTIRQ_MASK
    }
    #[cfg(not(CONFIG_PREEMPT_RT))]
    {
        lupos_sirq_preempt_count() & RUST_SIRQ_SOFTIRQ_MASK
    }
}
#[inline(always)]
unsafe fn irq_count() -> c_uint {
    (lupos_sirq_preempt_count() & (RUST_SIRQ_NMI_MASK | RUST_SIRQ_HARDIRQ_MASK)) | softirq_count()
}
#[inline(always)]
unsafe fn in_interrupt() -> bool {
    irq_count() != 0
}
#[inline(always)]
unsafe fn in_hardirq() -> bool {
    lupos_sirq_preempt_count() & RUST_SIRQ_HARDIRQ_MASK != 0
}
#[inline(always)]
unsafe fn in_nmi() -> bool {
    lupos_sirq_preempt_count() & RUST_SIRQ_NMI_MASK != 0
}

unsafe fn wakeup_softirqd() {
    let task = lupos_sirq_ksoftirqd_read();
    if !task.is_null() {
        wake_up_process(task);
    }
}

#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_disable() {
    // include/linux/interrupt_rc.h::__local_interrupt_disable, in Rust.
    let flags = lupos_sirq_irq_save();
    lupos_sirq_interrupt_state_write(flags);
}
#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_enable() {
    lupos_sirq_irq_restore(lupos_sirq_interrupt_state_read());
}

unsafe fn lockdep_softirq_start() -> bool {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    {
        let was_hardirq = lupos_sirq_hardirq_context();
        if was_hardirq {
            lupos_sirq_hardirq_exit();
        }
        lupos_sirq_softirq_enter();
        return was_hardirq;
    }
    #[cfg(not(CONFIG_TRACE_IRQFLAGS))]
    {
        false
    }
}
unsafe fn lockdep_softirq_end(_was_hardirq: bool) {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    {
        lupos_sirq_softirq_exit();
        if _was_hardirq {
            lupos_sirq_hardirq_enter();
        }
    }
}

// Expanded vtime.h sequencing: configured out-of-line accounting providers
// retain their own ownership, but the pair/order of calls belongs to Rust.
unsafe fn account_enter(_task: *mut task_struct, _offset: c_uint) {
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
    vtime_account_irq(_task, _offset);
    #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
    irqtime_account_irq(_task, _offset);
}
unsafe fn account_softirq_exit(_task: *mut task_struct) {
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
    vtime_account_softirq(_task);
    #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
    irqtime_account_irq(_task, 0);
}
unsafe fn account_hardirq_exit(_task: *mut task_struct) {
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
    vtime_account_hardirq(_task);
    #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
    irqtime_account_irq(_task, 0);
}

unsafe fn handle_softirqs(_ksirqd: bool) {
    // Exact msecs_to_jiffies(2) result for all x86 CONFIG_HZ choices.
    const MAX_SOFTIRQ_TIME: c_ulong = (2 * RUST_SIRQ_HZ as c_ulong + 999) / 1000;
    let end = lupos_sirq_jiffies().wrapping_add(MAX_SOFTIRQ_TIME);
    let task = current_task();
    let old_flags = (*task).flags;
    let mut max_restart = 10;
    (*task).flags &= !(PF_MEMALLOC as c_uint);
    let mut pending = lupos_sirq_pending();
    softirq_handle_begin();
    let was_hardirq = lockdep_softirq_start();
    account_enter(task, RUST_SIRQ_SOFTIRQ_OFFSET);
    loop {
        // Publish the empty pending mask before enabling hard IRQs. Newly
        // raised bits are consumed only after the current snapshot finishes.
        lupos_sirq_set_pending(0);
        lupos_sirq_irq_enable();
        let mut base = 0usize;
        while pending != 0 {
            let skip = pending.trailing_zeros() as usize;
            let nr = base + skip;
            let vector = addr_of!(SOFTIRQ_VEC.value).cast::<softirq_action>().add(nr);
            let prev_count = lupos_sirq_preempt_count();
            lupos_sirq_kstat_inc(nr as c_uint);
            lupos_sirq_trace_entry(nr as c_uint);
            // C calls unconditionally; a missing installed action is a bug,
            // never a silently ignored vector.
            (*vector).action.unwrap_unchecked()();
            lupos_sirq_trace_exit(nr as c_uint);
            let after = lupos_sirq_preempt_count();
            if prev_count != after {
                lupos_sirq_count_error(
                    nr as c_uint,
                    SOFTIRQ_NAMES.value[nr],
                    (*vector).action,
                    prev_count,
                    after,
                );
                // Original prev_count is int; retain its native promotion
                // when restoring architectures' unsigned-long counter API.
                lupos_sirq_preempt_set(prev_count as c_int);
            }
            base = nr + 1;
            // NR_SOFTIRQS is 10 in this source. Avoid a Rust shift panic even
            // if the native vector count eventually reaches 32.
            pending = if skip == 31 { 0 } else { pending >> (skip + 1) };
        }
        #[cfg(not(CONFIG_PREEMPT_RT))]
        if _ksirqd {
            lupos_sirq_rcu_qs();
        }
        lupos_sirq_irq_disable();
        pending = lupos_sirq_pending();
        if pending != 0 {
            // time_before uses signed wrapping subtraction, not numeric <.
            if (lupos_sirq_jiffies().wrapping_sub(end) as isize) < 0 && !lupos_sirq_need_resched() {
                max_restart -= 1;
                if max_restart != 0 {
                    continue;
                }
            }
            wakeup_softirqd();
        }
        break;
    }
    account_softirq_exit(task);
    lockdep_softirq_end(was_hardirq);
    softirq_handle_end();
    // current_restore_flags restores only the borrowed PF_MEMALLOC bit.
    (*task).flags =
        ((*task).flags & !(PF_MEMALLOC as c_uint)) | (old_flags & PF_MEMALLOC as c_uint);
}

#[no_mangle]
#[link_section = ".softirqentry.text"]
pub unsafe extern "C" fn __do_softirq() {
    handle_softirqs(false);
}

#[no_mangle]
pub unsafe extern "C" fn irq_enter_rcu() {
    // __irq_enter_raw, expanded to preserve count-before-lockdep order.
    lupos_sirq_preempt_add(RUST_SIRQ_HARDIRQ_OFFSET);
    lupos_sirq_hardirq_enter();
    lupos_sirq_hrtimer_rearm();
    let task = current_task();
    if lupos_sirq_tick_full(lupos_sirq_cpu())
        || (((*task).flags & PF_IDLE as c_uint != 0) && irq_count() == RUST_SIRQ_HARDIRQ_OFFSET)
    {
        lupos_sirq_tick_enter();
    }
    account_enter(task, RUST_SIRQ_HARDIRQ_OFFSET);
}
#[no_mangle]
pub unsafe extern "C" fn irq_enter() {
    lupos_sirq_ct_enter();
    irq_enter_rcu();
}

unsafe fn tick_irq_exit() {
    #[cfg(CONFIG_NO_HZ_COMMON)]
    {
        let cpu = lupos_sirq_cpu();
        if (lupos_sirq_core_idle(cpu as c_int) && !lupos_sirq_need_resched())
            || lupos_sirq_tick_full(cpu)
        {
            if !in_hardirq() {
                lupos_sirq_tick_exit();
            }
        }
    }
}
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
unsafe fn wake_timersd() {
    let task = lupos_sirq_ktimerd_read();
    if !task.is_null() {
        wake_up_process(task);
    }
}

unsafe fn irq_exit_common() {
    if RUST_SIRQ_ARCH_EXIT_IRQS_DISABLED == 0 {
        lupos_sirq_irq_disable();
    } else {
        lupos_sirq_assert_irqs_disabled();
    }
    account_hardirq_exit(current_task());
    lupos_sirq_preempt_sub(RUST_SIRQ_HARDIRQ_OFFSET);
    if !in_interrupt()
        && lupos_sirq_preempt_count() & RUST_SIRQ_HARDIRQ_DISABLE_MASK == 0
        && lupos_sirq_pending() != 0
    {
        lupos_sirq_hrtimer_rearm();
        invoke_softirq();
    }
    #[cfg(CONFIG_IRQ_FORCED_THREADING)]
    if lupos_sirq_force_irqthreads()
        && lupos_sirq_timer_pending() != 0
        && !(in_nmi() | in_hardirq())
    {
        wake_timersd();
    }
    tick_irq_exit();
}
#[no_mangle]
pub unsafe extern "C" fn irq_exit_rcu() {
    irq_exit_common();
    lupos_sirq_hardirq_exit(); // must be last
}
#[no_mangle]
pub unsafe extern "C" fn irq_exit() {
    irq_exit_common();
    lupos_sirq_ct_exit();
    lupos_sirq_hardirq_exit(); // must be last, after context tracking
}

#[no_mangle]
pub unsafe extern "C" fn raise_softirq_irqoff(nr: c_uint) {
    __raise_softirq_irqoff(nr);
    if !in_interrupt() && should_wake_ksoftirqd() {
        wakeup_softirqd();
    }
}
#[no_mangle]
pub unsafe extern "C" fn raise_softirq(nr: c_uint) {
    let flags = lupos_sirq_irq_save();
    raise_softirq_irqoff(nr);
    lupos_sirq_irq_restore(flags);
}
#[no_mangle]
pub unsafe extern "C" fn __raise_softirq_irqoff(nr: c_uint) {
    lupos_sirq_assert_irqs_disabled();
    lupos_sirq_trace_raise(nr);
    lupos_sirq_or_pending(1u32 << nr);
}
#[no_mangle]
pub unsafe extern "C" fn open_softirq(nr: c_int, action: Option<unsafe extern "C" fn()>) {
    (*addr_of_mut!(SOFTIRQ_VEC.value)
        .cast::<softirq_action>()
        .add(nr as usize))
    .action = action;
}

unsafe extern "C" fn ksoftirqd_should_run(_cpu: c_uint) -> c_int {
    lupos_sirq_pending() as c_int
}
unsafe extern "C" fn run_ksoftirqd(_cpu: c_uint) {
    ksoftirqd_run_begin();
    if lupos_sirq_pending() != 0 {
        handle_softirqs(true);
        ksoftirqd_run_end();
        lupos_sirq_cond_resched();
        return;
    }
    ksoftirqd_run_end();
}
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
unsafe extern "C" fn ktimerd_setup(_cpu: c_uint) {
    sched_set_fifo_low(current_task());
}
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
unsafe extern "C" fn ktimerd_should_run(_cpu: c_uint) -> c_int {
    lupos_sirq_timer_pending() as c_int
}
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
#[no_mangle]
pub unsafe extern "C" fn raise_ktimers_thread(nr: c_uint) {
    lupos_sirq_trace_raise(nr);
    lupos_sirq_timer_or((1 as c_ulong) << nr);
}
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
unsafe extern "C" fn run_ktimerd(_cpu: c_uint) {
    ksoftirqd_run_begin();
    let timer_si = lupos_sirq_timer_pending();
    lupos_sirq_timer_clear();
    lupos_sirq_or_pending(timer_si);
    __do_softirq();
    ksoftirqd_run_end();
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_spawn_ksoftirqd() -> c_int {
    #[cfg(CONFIG_HOTPLUG_CPU)]
    let dead = Some(takeover_tasklets as unsafe extern "C" fn(c_uint) -> c_int);
    #[cfg(not(CONFIG_HOTPLUG_CPU))]
    let dead = None;
    // As in the source, hotplug-registration return is deliberately ignored.
    lupos_sirq_cpuhp(dead);
    lupos_sirq_bug(smpboot_register_percpu_thread(addr_of_mut!(SOFTIRQ_THREADS)) != 0);
    #[cfg(CONFIG_IRQ_FORCED_THREADING)]
    if lupos_sirq_force_irqthreads() {
        lupos_sirq_bug(smpboot_register_percpu_thread(addr_of_mut!(TIMER_THREAD)) != 0);
    }
    0
}

#[no_mangle]
#[linkage = "weak"]
#[link_section = ".init.text"]
pub unsafe extern "C" fn early_irq_init() -> c_int {
    0
}
#[no_mangle]
#[linkage = "weak"]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_probe_nr_irqs() -> c_int {
    RUST_SIRQ_NR_IRQS_LEGACY as c_int
}
#[no_mangle]
#[linkage = "weak"]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_early_irq_init() -> c_int {
    0
}
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn arch_dynirq_lower_bound(from: c_uint) -> c_uint {
    from
}
