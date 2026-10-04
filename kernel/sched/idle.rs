// SPDX-License-Identifier: GPL-2.0-only
// Generic idle loop and idle scheduling class. Authority: kernel/sched/idle.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/sched_idle_generated.rs"
    ));
}
use b::*;
use core::mem::{align_of, offset_of, size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
#[cfg(CONFIG_GENERIC_IDLE_POLL_SETUP)]
use kernel::ffi::c_char;
use kernel::ffi::{c_int, c_long, c_ulong};

#[link_section = ".data..read_mostly"]
static mut cpu_idle_force_poll: c_int = 0;

// These must inline at the Rust call site: putting begin/end in out-of-line
// helpers would annotate a different control-flow graph. Constants and format
// are the native linux/instrumentation.h + linux/annotate.h contract.
#[inline(always)]
unsafe fn instrumentation_begin() {
    #[cfg(CONFIG_NOINSTR_VALIDATION)]
    core::arch::asm!("2: nop", ".pushsection .discard.annotate_insn, \"M\", @progbits, 8",
        ".long 2b - ., {kind}", ".popsection",
        kind = const LUPOS_IDLE_ANNOTYPE_INSTR_BEGIN, options(nostack, preserves_flags));
}
#[inline(always)]
unsafe fn instrumentation_end() {
    #[cfg(CONFIG_NOINSTR_VALIDATION)]
    core::arch::asm!("2: nop", ".pushsection .discard.annotate_insn, \"M\", @progbits, 8",
        ".long 2b - ., {kind}", ".popsection",
        kind = const LUPOS_IDLE_ANNOTYPE_INSTR_END, options(nostack, preserves_flags));
}
#[no_mangle]
pub unsafe extern "C" fn sched_idle_set_state(idle_state: *mut cpuidle_state) {
    #[cfg(CONFIG_CPU_IDLE)]
    {
        (*lupos_idle_this_rq()).idle_state = idle_state;
    }
    #[cfg(not(CONFIG_CPU_IDLE))]
    let _ = idle_state;
}
#[no_mangle]
pub unsafe extern "C" fn cpu_idle_poll_ctrl(enable: bool) {
    let count = read_volatile(addr_of!(cpu_idle_force_poll));
    let count = if enable {
        count.wrapping_add(1)
    } else {
        count.wrapping_sub(1)
    };
    write_volatile(addr_of_mut!(cpu_idle_force_poll), count);
    if !enable {
        lupos_idle_warn_poll_negative(count < 0);
    }
}
#[cfg(CONFIG_GENERIC_IDLE_POLL_SETUP)]
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_idle_poll_setup(_unused: *mut c_char) -> c_int {
    write_volatile(addr_of_mut!(cpu_idle_force_poll), 1);
    1
}
#[cfg(CONFIG_GENERIC_IDLE_POLL_SETUP)]
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_idle_nopoll_setup(_unused: *mut c_char) -> c_int {
    write_volatile(addr_of_mut!(cpu_idle_force_poll), 0);
    1
}
#[inline(never)]
#[link_section = ".cpuidle.text"]
#[no_sanitize(address, hwaddress, memory, thread)]
unsafe fn cpu_idle_poll() -> c_int {
    instrumentation_begin();
    lupos_idle_trace(0, lupos_idle_cpu() as u32);
    lupos_idle_stop_critical_timings();
    // ct_cpuidle_enter's compiler annotation is carried by this Rust owner;
    // its architecture/lockdep/RCU transition runs in the noinstr leaf.
    instrumentation_end();
    lupos_idle_ct_enter();
    lupos_idle_raw_irq_enable();
    while !lupos_idle_tif_need_resched()
        && (read_volatile(addr_of!(cpu_idle_force_poll)) != 0 || lupos_idle_broadcast_expired())
    {
        lupos_idle_cpu_relax();
    }
    lupos_idle_raw_irq_disable();
    lupos_idle_ct_exit();
    instrumentation_begin();
    lupos_idle_start_critical_timings();
    lupos_idle_trace(LUPOS_IDLE_PWR_EVENT_EXIT as u32, lupos_idle_cpu() as u32);
    lupos_idle_irq_enable();
    instrumentation_end();
    1
}
// Native weak symbols call these defaults; strong architecture definitions
// remain authoritative. Empty defaults are exactly the original definitions.
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_arch_prepare_default() {}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_arch_enter_default() {}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_arch_exit_default() {}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_arch_dead_default() -> ! {
    loop {
        core::arch::asm!("", options(nomem, nostack, preserves_flags));
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_arch_idle_default() {
    write_volatile(addr_of_mut!(cpu_idle_force_poll), 1);
}
#[inline(always)]
unsafe fn cond_tick_broadcast_enter() {
    #[cfg(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST_IDLE)]
    if lupos_idle_needs_broadcast() {
        lupos_idle_tick_broadcast_enter();
    }
}
#[inline(always)]
unsafe fn cond_tick_broadcast_exit() {
    #[cfg(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST_IDLE)]
    if lupos_idle_needs_broadcast() {
        lupos_idle_tick_broadcast_exit();
    }
}
#[no_mangle]
#[inline(never)]
#[link_section = ".cpuidle.text"]
#[no_sanitize(address, hwaddress, memory, thread)]
pub unsafe extern "C" fn default_idle_call() {
    instrumentation_begin();
    if !lupos_idle_clr_polling_test() {
        cond_tick_broadcast_enter();
        lupos_idle_trace(1, lupos_idle_cpu() as u32);
        lupos_idle_stop_critical_timings();
        instrumentation_end();
        lupos_idle_ct_enter();
        arch_cpu_idle();
        lupos_idle_ct_exit();
        instrumentation_begin();
        lupos_idle_start_critical_timings();
        lupos_idle_trace(LUPOS_IDLE_PWR_EVENT_EXIT as u32, lupos_idle_cpu() as u32);
        cond_tick_broadcast_exit();
    }
    lupos_idle_irq_enable();
    instrumentation_end();
}
unsafe fn call_cpuidle_s2idle(
    drv: *mut cpuidle_driver,
    dev: *mut cpuidle_device,
    max_latency_ns: u64,
) -> c_int {
    if lupos_idle_clr_polling_test() {
        return -(LUPOS_IDLE_EBUSY as c_int);
    }
    lupos_idle_enter_s2idle(drv, dev, max_latency_ns)
}
unsafe fn call_cpuidle(
    drv: *mut cpuidle_driver,
    dev: *mut cpuidle_device,
    next_state: c_int,
) -> c_int {
    if lupos_idle_clr_polling_test() {
        (*dev).last_residency_ns = 0;
        lupos_idle_irq_enable();
        return -(LUPOS_IDLE_EBUSY as c_int);
    }
    lupos_idle_enter(drv, dev, next_state)
}
unsafe fn idle_call_stop_or_retain_tick(stop_tick: bool) {
    if stop_tick || lupos_idle_tick_stopped() {
        lupos_idle_stop_tick();
    } else {
        lupos_idle_retain_tick();
    }
}
unsafe fn cpuidle_idle_call(mut stop_tick: bool) {
    let dev = lupos_idle_get_device();
    let drv = lupos_idle_get_cpu_driver(dev);
    if lupos_idle_need_resched() {
        lupos_idle_irq_enable();
        return;
    }
    if lupos_idle_not_available(drv, dev) {
        idle_call_stop_or_retain_tick(stop_tick);
        default_idle_call();
    } else if lupos_idle_should_s2idle() || (*dev).forced_idle_latency_limit_ns != 0 {
        // The labeled block models goto exit_idle, retaining the IRQ warning
        // even after a successful suspend-to-idle entry.
        's2idle: {
            let max_latency_ns;
            if lupos_idle_should_s2idle() {
                max_latency_ns = (lupos_idle_wakeup_latency_qos_limit() as c_long)
                    .wrapping_mul(LUPOS_IDLE_NSEC_PER_USEC as c_long)
                    as u64;
                if call_cpuidle_s2idle(drv, dev, max_latency_ns) > 0 {
                    break 's2idle;
                }
            } else {
                max_latency_ns = (*dev).forced_idle_latency_limit_ns;
            }
            lupos_idle_stop_tick();
            let next_state = lupos_idle_find_deepest_state(drv, dev, max_latency_ns);
            call_cpuidle(drv, dev, next_state);
        }
    } else if (*drv).state_count > 1 {
        stop_tick = true;
        let next_state = lupos_idle_select(drv, dev, &mut stop_tick);
        idle_call_stop_or_retain_tick(stop_tick);
        let entered_state = call_cpuidle(drv, dev, next_state);
        lupos_idle_reflect(dev, entered_state);
    } else {
        idle_call_stop_or_retain_tick(stop_tick);
        call_cpuidle(drv, dev, 0);
    }
    lupos_idle_set_polling();
    if lupos_idle_warn_irqs_disabled(lupos_idle_irqs_disabled()) {
        lupos_idle_irq_enable();
    }
}
unsafe fn do_idle() {
    let cpu = lupos_idle_cpu();
    let mut got_tick = false;
    if lupos_idle_cpu_offline(cpu) {
        lupos_idle_irq_disable();
        lupos_idle_warn_offline_resched(lupos_idle_need_resched());
        lupos_idle_cpuhp_report_dead();
        arch_cpu_idle_dead();
    }
    lupos_idle_nohz_balance(cpu);
    lupos_idle_set_polling();
    lupos_idle_tick_enter();
    while !lupos_idle_need_resched() {
        lupos_idle_irq_disable();
        arch_cpu_idle_enter();
        lupos_idle_rcu_flush();
        if read_volatile(addr_of!(cpu_idle_force_poll)) != 0 || lupos_idle_broadcast_expired() {
            lupos_idle_restart_tick();
            cpu_idle_poll();
        } else {
            cpuidle_idle_call(got_tick);
        }
        got_tick = lupos_idle_got_tick();
        arch_cpu_idle_exit();
    }
    lupos_idle_preempt_set_need_resched();
    lupos_idle_tick_exit();
    lupos_idle_clr_polling();
    lupos_idle_mb_after_atomic();
    lupos_idle_flush_smp_queue();
    schedule_idle();
    if lupos_idle_patch_pending(lupos_idle_current()) {
        lupos_idle_update_patch(lupos_idle_current());
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpu_in_idle(pc: c_ulong) -> bool {
    pc >= addr_of!(__cpuidle_text_start) as c_ulong && pc < addr_of!(__cpuidle_text_end) as c_ulong
}
unsafe extern "C" fn idle_inject_timer_fn(timer: *mut hrtimer) -> hrtimer_restart {
    let it = timer
        .cast::<u8>()
        .sub(offset_of!(idle_timer, timer))
        .cast::<idle_timer>();
    write_volatile(addr_of_mut!((*it).done), 1);
    lupos_idle_set_task_need_resched(lupos_idle_current());
    HRTIMER_NORESTART
}
#[no_mangle]
pub unsafe extern "C" fn play_idle_precise(duration_ns: u64, latency_ns: u64) {
    let current = lupos_idle_current();
    lupos_idle_warn_policy((*current).policy != LUPOS_IDLE_SCHED_FIFO as _);
    lupos_idle_warn_affinity((*current).nr_cpus_allowed != 1);
    lupos_idle_warn_kthread((*current).flags & LUPOS_IDLE_PF_KTHREAD == 0);
    lupos_idle_warn_no_setaffinity((*current).flags & LUPOS_IDLE_PF_NO_SETAFFINITY == 0);
    lupos_idle_warn_duration(duration_ns == 0);
    lupos_idle_warn_mm(!(*current).mm.is_null());
    lupos_idle_rcu_sleep_check();
    lupos_idle_preempt_disable();
    (*lupos_idle_current()).flags |= LUPOS_IDLE_PF_IDLE;
    lupos_idle_use_deepest_state(latency_ns);
    let mut storage = MaybeUninit::<idle_timer>::uninit();
    let it = storage.as_mut_ptr();
    write_volatile(addr_of_mut!((*it).done), 0);
    lupos_idle_hrtimer_setup(addr_of_mut!((*it).timer), Some(idle_inject_timer_fn));
    lupos_idle_hrtimer_start(addr_of_mut!((*it).timer), duration_ns as i64);
    while read_volatile(addr_of!((*it).done)) == 0 {
        do_idle();
    }
    lupos_idle_use_deepest_state(0);
    (*lupos_idle_current()).flags &= !LUPOS_IDLE_PF_IDLE;
    lupos_idle_preempt_fold_need_resched();
    lupos_idle_preempt_enable();
}
#[no_mangle]
pub unsafe extern "C" fn cpu_startup_entry(state: cpuhp_state) {
    (*lupos_idle_current()).flags |= LUPOS_IDLE_PF_IDLE;
    arch_cpu_idle_prepare();
    lupos_idle_cpuhp_online(state);
    loop {
        do_idle();
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_select_task_rq(
    p: *mut task_struct,
    _cpu: c_int,
    _flags: c_int,
) -> c_int {
    lupos_idle_task_cpu(p)
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_balance(_rq: *mut rq, _rf: *mut rq_flags) -> c_int {
    lupos_idle_warn_balance(true) as c_int
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_wakeup_preempt(
    rq: *mut rq,
    _p: *mut task_struct,
    _flags: c_int,
) {
    resched_curr(rq);
}
#[inline(always)]
unsafe fn scx_update_idle(rq: *mut rq, idle: bool, notify: bool) {
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    if lupos_idle_scx_enabled() {
        __scx_update_idle(rq, idle, notify);
    }
    #[cfg(not(CONFIG_SCHED_CLASS_EXT))]
    let _ = (rq, idle, notify);
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_put_prev_task(
    rq: *mut rq,
    _prev: *mut task_struct,
    _next: *mut task_struct,
) {
    lupos_idle_update_curr(rq);
    scx_update_idle(rq, false, true);
    update_rq_avg_idle(rq);
}
// Native pelt.h's idle accounting is also Rust-owned. Only lockdep and the
// 32-bit duplicate-store memory-ordering leaves cross the boundary.
unsafe fn update_idle_rq_clock_pelt(rq: *mut rq) {
    let divider = ((LUPOS_IDLE_LOAD_AVG_MAX as u32).wrapping_sub(1024)
        << LUPOS_IDLE_SCHED_CAPACITY_SHIFT)
        .wrapping_sub(LUPOS_IDLE_LOAD_AVG_MAX as u32);
    let util_sum = (*rq)
        .cfs
        .avg
        .util_sum
        .wrapping_add((*rq).avg_rt.util_sum)
        .wrapping_add((*rq).avg_dl.util_sum);
    if util_sum >= divider {
        (*rq).lost_idle_time = (*rq)
            .lost_idle_time
            .wrapping_add(lupos_idle_rq_clock_task(rq).wrapping_sub((*rq).clock_pelt) as c_ulong);
    }
    (*rq).clock_pelt = lupos_idle_rq_clock_task(rq);
    lupos_idle_store_clock_idle(rq, lupos_idle_rq_clock(rq));
    lupos_idle_smp_wmb();
    lupos_idle_assert_clock(rq);
    lupos_idle_store_clock_pelt_idle(
        rq,
        (*rq).clock_pelt.wrapping_sub((*rq).lost_idle_time as u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_set_next_task(
    rq: *mut rq,
    next: *mut task_struct,
    _first: bool,
) {
    if lupos_idle_smt_active() {
        __update_idle_core(rq);
    }
    scx_update_idle(rq, true, true);
    #[cfg(CONFIG_SCHEDSTATS)]
    if lupos_idle_schedstat_enabled() {
        (*rq).sched_goidle = (*rq).sched_goidle.wrapping_add(1);
    }
    (*next).se.exec_start = lupos_idle_rq_clock_task(rq);
    update_idle_rq_clock_pelt(rq);
}
#[no_mangle]
pub unsafe extern "C" fn pick_task_idle(rq: *mut rq, _rf: *mut rq_flags) -> *mut task_struct {
    if lupos_idle_scx_enabled() {
        // sched.h aliases donor and curr only when proxy execution is disabled.
        #[cfg(CONFIG_SCHED_PROXY_EXEC)]
        let curr = (*rq).curr;
        #[cfg(not(CONFIG_SCHED_PROXY_EXEC))]
        let curr = (*rq).__bindgen_anon_1.curr;
        if (*curr).flags & LUPOS_IDLE_PF_IDLE != 0 {
            scx_update_idle(rq, true, false);
        }
    }
    (*rq).idle
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_dequeue_task(
    rq: *mut rq,
    _p: *mut task_struct,
    _flags: c_int,
) -> bool {
    lupos_idle_rq_unlock_irq(rq);
    lupos_idle_print_bad_schedule();
    lupos_idle_dump_stack();
    lupos_idle_rq_lock_irq(rq);
    true
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_task_tick(
    rq: *mut rq,
    _curr: *mut task_struct,
    _queued: c_int,
) {
    lupos_idle_update_curr(rq);
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_switching_to(_rq: *mut rq, _p: *mut task_struct) {
    lupos_idle_bug_switching_to();
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_prio_changed(_rq: *mut rq, p: *mut task_struct, oldprio: u64) {
    if (*p).prio as u64 == oldprio {
        return;
    }
    lupos_idle_bug_prio_changed();
}
#[no_mangle]
pub unsafe extern "C" fn lupos_idle_update_curr(rq: *mut rq) {
    let se = addr_of_mut!((*(*rq).idle).se);
    let now = lupos_idle_rq_clock_task(rq);
    let delta_exec = now.wrapping_sub((*se).exec_start) as i64;
    if delta_exec <= 0 {
        return;
    }
    (*se).exec_start = now;
    dl_server_update_idle(addr_of_mut!((*rq).fair_server), delta_exec);
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    dl_server_update_idle(addr_of_mut!((*rq).ext_server), delta_exec);
}

include!("sched_idle_layout.rs");
