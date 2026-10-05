// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kernel/stop_machine.c
 *
 * Copyright (C) 2008, 2005 IBM Corporation.
 * Copyright (C) 2008, 2005 Rusty Russell rusty@rustcorp.com.au
 * Copyright (C) 2010 SUSE Linux Products GmbH
 * Copyright (C) 2010 Tejun Heo <tj@kernel.org>
 */

// Continue baseline Rust blob 33c68c2f904638fc139794a138449f6613551965.
// Native configured declarations own layouts, locks, per-CPU member storage,
// callback types and the integer representation of multi_stop_state.
#![no_std]
#![cfg(CONFIG_SMP)]
compile_error!("SOURCE ONLY HOLD: stop-machine Rust owner is not admitted");

use core::mem::MaybeUninit;
use core::ptr::null_mut;
use kernel::bindings::stop_machine_native::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// SAFETY: task is live; a stopper task cannot migrate, making task_cpu stable.
#[export_name = "lupos_stop_print_stop_info"]
unsafe extern "C" fn print_stop_info(log_lvl: *const c_char, task: *mut task_struct) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(lupos_stop_task_cpu(task));
        if task != lupos_stop_thread(stopper) { return; }
        lupos_stop_print_info(log_lvl, stopper);
    }
}

// SAFETY: done points to unmoved native stack storage, not yet published.
unsafe fn cpu_stop_init_done(done: *mut cpu_stop_done, nr_todo: c_uint) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        lupos_stop_done_zero(done);
        lupos_stop_done_count_set(done, nr_todo);
        lupos_stop_completion_init(done);
    }
}

// SAFETY: initialized done is pinned until all queued works have acknowledged.
unsafe fn cpu_stop_signal_done(done: *mut cpu_stop_done) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        if lupos_stop_done_dec_and_test(done) { lupos_stop_complete(done); }
    }
}

// SAFETY: caller holds this stopper's raw lock and owns the unqueued work node.
unsafe fn __cpu_stop_queue_work(stopper: *mut cpu_stopper, work: *mut cpu_stop_work) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe { lupos_stop_list_add_tail(stopper, work); }
}

// SAFETY: work and optional done stay live through execution/completion. The
// native lock serializes list/enabled observation; preemption spans both wakeups.
#[export_name = "lupos_stop_cpu_stop_queue_work"]
unsafe extern "C" fn cpu_stop_queue_work(cpu: c_uint, work: *mut cpu_stop_work) -> bool {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(cpu);
        lupos_stop_preempt_disable();
        let flags = lupos_stop_lock_irqsave(stopper);
        let enabled = lupos_stop_enabled(stopper);
        if enabled { __cpu_stop_queue_work(stopper, work); }
        else {
            let done = lupos_stop_work_done(work);
            if !done.is_null() { cpu_stop_signal_done(done); }
        }
        lupos_stop_unlock_irqrestore(stopper, flags);
        if enabled { lupos_stop_wake(lupos_stop_thread(stopper)); }
        lupos_stop_preempt_enable();
        enabled
    }
}

// SAFETY: native public entry pins fn for this call; initialization copies its
// callback value, and the stack slots are never moved after taking addresses.
#[export_name = "lupos_stop_one_cpu"]
unsafe extern "C" fn stop_one_cpu(cpu: c_uint, fn_: *const lupos_stop_callback,
                                 arg: *mut c_void, caller: c_ulong) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut done_slot = MaybeUninit::<cpu_stop_done>::uninit();
        let done = done_slot.as_mut_ptr();
        let mut work_slot = MaybeUninit::<cpu_stop_work>::uninit();
        let work = work_slot.as_mut_ptr();
        lupos_stop_work_init(work, fn_, arg, done, caller);
        cpu_stop_init_done(done, 1);
        if !cpu_stop_queue_work(cpu, work) { return -LUPOS_STOP_ENOENT; }
        lupos_stop_cond_resched();
        lupos_stop_wait_completion(done);
        lupos_stop_done_ret(done)
    }
}

// multi_stop_state is bindgen's newtype over the configured C enum integer,
// not a closed Rust enum. The last EXIT ack therefore legally publishes 5.
// SAFETY: this transition is by the initiating caller or the last native ack.
unsafe fn set_state(msdata: *mut multi_stop_data, newstate: multi_stop_state) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        lupos_stop_ack_set(msdata, lupos_stop_num_threads(msdata));
        // Publish the reset acknowledgement count before the new state.
        lupos_stop_wmb();
        lupos_stop_write_state(msdata, newstate);
    }
}

// SAFETY: each participant acknowledges every observed transition once. Native
// atomic_dec_and_test grants exactly one participant the successor publication.
unsafe fn ack_state(msdata: *mut multi_stop_data) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        if lupos_stop_ack_dec_and_test(msdata) {
            let next = multi_stop_state(lupos_stop_state(msdata).0.wrapping_add(1));
            set_state(msdata, next);
        }
    }
}

// SAFETY: mask remains live through the operation. Native weak/notrace entry
// preserves architecture override dispatch rather than fixing a local no-op.
unsafe fn stop_machine_yield(cpumask: *const cpumask) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe { lupos_stop_yield(cpumask); }
}

// SAFETY: data is native msdata pinned by the synchronous caller; callbacks are
// non-sleeping and all participants observe the native publication protocol.
#[export_name = "lupos_stop_multi_cpu_stop"]
unsafe extern "C" fn multi_cpu_stop(data: *mut c_void) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let msdata = data.cast::<multi_stop_data>();
        let mut curstate = multi_stop_state::MULTI_STOP_NONE;
        let cpu = lupos_stop_processor_id();
        let mut err = 0;
        let cpumask;
        let is_active;
        let flags = lupos_stop_save_flags();
        let active_cpus = lupos_stop_active_cpus(msdata);
        if active_cpus.is_null() {
            cpumask = lupos_stop_online_mask();
            is_active = cpu == lupos_stop_mask_first(cpumask);
        } else {
            cpumask = active_cpus;
            is_active = lupos_stop_mask_test(cpu, cpumask);
        }
        loop {
            stop_machine_yield(cpumask);
            let newstate = lupos_stop_read_state(msdata);
            if newstate.0 != curstate.0 {
                curstate = newstate;
                if curstate.0 == multi_stop_state::MULTI_STOP_DISABLE_IRQ.0 {
                    lupos_stop_irq_disable();
                    lupos_stop_hard_irq_disable();
                } else if curstate.0 == multi_stop_state::MULTI_STOP_RUN.0 {
                    if is_active {
                        // Native call leaf is exactly msdata->fn(msdata->data).
                        err = lupos_stop_invoke_msdata(msdata);
                    }
                }
                ack_state(msdata);
            } else if curstate.0 > multi_stop_state::MULTI_STOP_PREPARE.0 {
                lupos_stop_touch_nmi_watchdog();
                lupos_stop_rcu_momentary_eqs();
            }
            if curstate.0 == multi_stop_state::MULTI_STOP_EXIT.0 { break; }
        }
        lupos_stop_irq_restore(flags);
        err
    }
}

// SAFETY: ordered distinct CPU stoppers and their works are pinned. Preserve the
// original lock ordering and retry; do not enable preemption between wakeups.
unsafe fn cpu_stop_queue_two_works(cpu1: c_int, work1: *mut cpu_stop_work,
                                   cpu2: c_int, work2: *mut cpu_stop_work) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper1 = lupos_stop_stopper(cpu1 as c_uint);
        let stopper2 = lupos_stop_stopper(cpu2 as c_uint);
        loop {
            lupos_stop_preempt_disable();
            lupos_stop_lock_irq(stopper1);
            lupos_stop_lock_nested(stopper2);
            if !lupos_stop_enabled(stopper1) || !lupos_stop_enabled(stopper2) {
                lupos_stop_unlock(stopper2);
                lupos_stop_unlock_irq(stopper1);
                lupos_stop_preempt_enable();
                return -LUPOS_STOP_ENOENT;
            }
            if lupos_stop_in_progress() {
                lupos_stop_unlock(stopper2);
                lupos_stop_unlock_irq(stopper1);
                lupos_stop_preempt_enable();
                while lupos_stop_in_progress() { lupos_stop_relax(); }
                continue;
            }
            __cpu_stop_queue_work(stopper1, work1);
            __cpu_stop_queue_work(stopper2, work2);
            lupos_stop_unlock(stopper2);
            lupos_stop_unlock_irq(stopper1);
            lupos_stop_wake(lupos_stop_thread(stopper1));
            lupos_stop_wake(lupos_stop_thread(stopper2));
            lupos_stop_preempt_enable();
            return 0;
        }
    }
}

// SAFETY: native API contract requires distinct CPUs; callback and argument
// remain valid until both works complete. No Rust reference aliases shared data.
#[export_name = "lupos_stop_two_cpus"]
unsafe extern "C" fn stop_two_cpus(cpu1: c_uint, cpu2: c_uint,
                                  fn_: *const lupos_stop_callback, arg: *mut c_void,
                                  caller: c_ulong) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut done_slot = MaybeUninit::<cpu_stop_done>::uninit();
        let done = done_slot.as_mut_ptr();
        let mut data_slot = MaybeUninit::<multi_stop_data>::uninit();
        let msdata = data_slot.as_mut_ptr();
        lupos_stop_msdata_init(msdata, fn_, arg, 2, lupos_stop_cpumask_of(cpu1));
        let mut work1_slot = MaybeUninit::<cpu_stop_work>::uninit();
        let work1 = work1_slot.as_mut_ptr();
        let mut work2_slot = MaybeUninit::<cpu_stop_work>::uninit();
        let work2 = work2_slot.as_mut_ptr();
        let callback = lupos_stop_multi_callback();
        lupos_stop_work_init(work1, callback, msdata.cast(), done, caller);
        lupos_stop_work_init(work2, callback, msdata.cast(), done, caller);
        cpu_stop_init_done(done, 2);
        set_state(msdata, multi_stop_state::MULTI_STOP_PREPARE);
        let (a, b) = if cpu1 > cpu2 { (cpu2, cpu1) } else { (cpu1, cpu2) };
        if cpu_stop_queue_two_works(a as c_int, work1, b as c_int, work2) != 0 {
            return -LUPOS_STOP_ENOENT;
        }
        lupos_stop_wait_completion(done);
        lupos_stop_done_ret(done)
    }
}

// SAFETY: caller owns unused work_buf and keeps it untouched until the callback
// starts; the native entry's borrowed callback value is copied before return.
#[export_name = "lupos_stop_one_cpu_nowait"]
unsafe extern "C" fn stop_one_cpu_nowait(cpu: c_uint, fn_: *const lupos_stop_callback,
                                       arg: *mut c_void, work_buf: *mut cpu_stop_work,
                                       caller: c_ulong) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        lupos_stop_work_init(work_buf, fn_, arg, null_mut(), caller);
        // Native WARN_ON_ONCE evaluates the queue call itself, once per entry.
        // Its adapter immediately re-enters cpu_stop_queue_work above; no
        // callback/context address is retained after this synchronous call.
        lupos_stop_warn_not_queued(cpu, work_buf);
    }
}

// SAFETY: caller serializes static stop_work reuse under stop_cpus_mutex. done
// remains pinned through completion. CPU membership follows the native mask.
#[export_name = "lupos_stop_queue_stop_cpus_work"]
unsafe extern "C" fn queue_stop_cpus_work(cpumask: *const cpumask,
                                        fn_: *const lupos_stop_callback,
                                        arg: *mut c_void, done: *mut cpu_stop_done,
                                        caller: c_ulong) -> bool {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut queued = false;
        lupos_stop_preempt_disable();
        lupos_stop_set_in_progress(true);
        // Keep the progress marker around queue publication under preempt disable.
        lupos_stop_barrier();
        let mut cpu = 0;
        loop {
            cpu = lupos_stop_mask_find_next(cpu, cpumask);
            if cpu >= lupos_stop_mask_limit() { break; }
            let work = lupos_stop_stop_work(lupos_stop_stopper(cpu));
            lupos_stop_work_set(work, fn_, arg, done, caller);
            if cpu_stop_queue_work(cpu, work) { queued = true; }
            cpu = cpu.wrapping_add(1);
        }
        lupos_stop_barrier();
        lupos_stop_set_in_progress(false);
        lupos_stop_preempt_enable();
        queued
    }
}

// SAFETY: caller holds the native mutex for all shared per-CPU work slots.
unsafe fn __stop_cpus(cpumask: *const cpumask, fn_: *const lupos_stop_callback,
                      arg: *mut c_void) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut done_slot = MaybeUninit::<cpu_stop_done>::uninit();
        let done = done_slot.as_mut_ptr();
        cpu_stop_init_done(done, lupos_stop_mask_weight(cpumask));
        if !lupos_stop_queue_entry(cpumask, fn_, arg, done) { return -LUPOS_STOP_ENOENT; }
        lupos_stop_wait_completion(done);
        lupos_stop_done_ret(done)
    }
}

// SAFETY: normal process-context callers may sleep while serializing the request.
unsafe fn stop_cpus(cpumask: *const cpumask, fn_: *const lupos_stop_callback,
                   arg: *mut c_void) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        lupos_stop_mutex_lock();
        let ret = __stop_cpus(cpumask, fn_, arg);
        lupos_stop_mutex_unlock();
        ret
    }
}

// SAFETY: smpboot invokes the mandatory predicate with preemption disabled.
#[export_name = "lupos_stop_cpu_stop_should_run"]
unsafe extern "C" fn cpu_stop_should_run(cpu: c_uint) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(cpu);
        let flags = lupos_stop_lock_irqsave(stopper);
        let run = !lupos_stop_list_empty(stopper);
        lupos_stop_unlock_irqrestore(stopper, flags);
        run as c_int
    }
}

// SAFETY: this is the bound stopper thread. Queue entries are removed while
// locked, and fn/arg/done/caller are captured before a callback can free work.
#[export_name = "lupos_stop_cpu_stopper_thread"]
unsafe extern "C" fn cpu_stopper_thread(cpu: c_uint) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(cpu);
        loop {
            let mut work = null_mut();
            lupos_stop_lock_irq(stopper);
            if !lupos_stop_list_empty(stopper) {
                work = lupos_stop_list_first(stopper);
                lupos_stop_list_del_init(work);
            }
            lupos_stop_unlock_irq(stopper);
            if work.is_null() { return; }
            let mut fn_slot = MaybeUninit::<lupos_stop_callback>::uninit();
            let fn_ = fn_slot.as_mut_ptr();
            lupos_stop_work_callback(work, fn_);
            let arg = lupos_stop_work_arg(work);
            let done = lupos_stop_work_done(work);
            lupos_stop_set_caller(stopper, lupos_stop_work_caller(work));
            lupos_stop_set_callback(stopper, fn_);
            lupos_stop_preempt_count_inc();
            let ret = lupos_stop_invoke(fn_, arg);
            if !done.is_null() {
                if ret != 0 { lupos_stop_set_done_ret(done, ret); }
                cpu_stop_signal_done(done);
            }
            lupos_stop_preempt_count_dec();
            lupos_stop_clear_callback(stopper);
            lupos_stop_set_caller(stopper, 0);
            lupos_stop_warn_preempt(fn_, arg);
        }
    }
}

// SAFETY: hotplug serialization pins the thread. Lockless disable deliberately
// permits the stopper's locked drain to complete queued work before parking.
#[export_name = "lupos_stop_machine_park"]
unsafe extern "C" fn stop_machine_park(cpu: c_int) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(cpu as c_uint);
        lupos_stop_set_enabled(stopper, false);
        lupos_stop_park_thread(lupos_stop_thread(stopper));
    }
}

// SAFETY: smpboot has stored and retained the parked task before this callback.
#[export_name = "lupos_stop_cpu_stop_create"]
unsafe extern "C" fn cpu_stop_create(cpu: c_uint) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe { lupos_stop_set_stop_task(cpu, lupos_stop_thread(lupos_stop_stopper(cpu))); }
}

// SAFETY: smpboot invokes park only in the bound thread after work was drained.
#[export_name = "lupos_stop_cpu_stop_park"]
unsafe extern "C" fn cpu_stop_park(cpu: c_uint) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe { lupos_stop_warn_park_work(lupos_stop_stopper(cpu)); }
}

// SAFETY: hotplug serialization pins the registered task through unpark.
#[export_name = "lupos_stop_machine_unpark"]
unsafe extern "C" fn stop_machine_unpark(cpu: c_int) {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let stopper = lupos_stop_stopper(cpu as c_uint);
        lupos_stop_set_enabled(stopper, true);
        lupos_stop_unpark_thread(lupos_stop_thread(stopper));
    }
}

// cpu_stop_threads is native storage: .store = &cpu_stopper.thread, with the
// original selfparking descriptor and actual unsigned-int callback signatures.
// SAFETY: early-init entry runs once before normal stop-machine publication.
#[export_name = "lupos_stop_cpu_stop_init"]
#[link_section = ".init.text"]
#[cold]
unsafe extern "C" fn cpu_stop_init() -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut cpu = 0;
        loop {
            cpu = lupos_stop_possible_find_next(cpu);
            if cpu >= lupos_stop_possible_limit() { break; }
            let stopper = lupos_stop_stopper(cpu);
            lupos_stop_lock_init(stopper);
            lupos_stop_list_init(stopper);
            cpu = cpu.wrapping_add(1);
        }
        lupos_stop_register_threads();
        stop_machine_unpark(lupos_stop_raw_processor_id() as c_int);
        lupos_stop_set_initialized();
        0
    }
}

// SAFETY: caller holds the CPU read lock; native callback/argument remain live
// through completion. Early boot uses the same IRQ-save/callback/restore order.
#[export_name = "lupos_stop_machine_cpuslocked"]
unsafe extern "C" fn stop_machine_cpuslocked(fn_: *const lupos_stop_callback,
                                           data: *mut c_void,
                                           cpus: *const cpumask) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut data_slot = MaybeUninit::<multi_stop_data>::uninit();
        let msdata = data_slot.as_mut_ptr();
        lupos_stop_msdata_init(msdata, fn_, data, lupos_stop_num_online(), cpus);
        lupos_stop_assert_cpus_held();
        if !lupos_stop_initialized() {
            lupos_stop_warn_early_threads(msdata);
            let flags = lupos_stop_irq_save();
            lupos_stop_hard_irq_disable();
            let ret = lupos_stop_invoke(fn_, data);
            lupos_stop_irq_restore(flags);
            return ret;
        }
        set_state(msdata, multi_stop_state::MULTI_STOP_PREPARE);
        stop_cpus(lupos_stop_online_mask(), lupos_stop_multi_callback(), msdata.cast())
    }
}

// SAFETY: native public entry retains fn/data/mask; this API may acquire CPU lock.
#[export_name = "lupos_stop_machine"]
unsafe extern "C" fn stop_machine(fn_: *const lupos_stop_callback, data: *mut c_void,
                                 cpus: *const cpumask) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        lupos_stop_cpus_read_lock();
        let ret = stop_machine_cpuslocked(fn_, data, cpus);
        lupos_stop_cpus_read_unlock();
        ret
    }
}

// SAFETY: caller holds CPU read lock and pins the callback/argument until return.
#[cfg(CONFIG_SCHED_SMT)]
#[export_name = "lupos_stop_core_cpuslocked"]
unsafe extern "C" fn stop_core_cpuslocked(cpu: c_uint,
                                       fn_: *const lupos_stop_callback,
                                       data: *mut c_void) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let smt_mask = lupos_stop_smt_mask(cpu);
        let mut data_slot = MaybeUninit::<multi_stop_data>::uninit();
        let msdata = data_slot.as_mut_ptr();
        lupos_stop_msdata_init(msdata, fn_, data, lupos_stop_mask_weight(smt_mask), smt_mask);
        lupos_stop_assert_cpus_held();
        set_state(msdata, multi_stop_state::MULTI_STOP_PREPARE);
        stop_cpus(smt_mask, lupos_stop_multi_callback(), msdata.cast())
    }
}

// SAFETY: CPU hotplug is in progress, local CPU is inactive and cannot sleep.
// Native completion, mutex trylock and relax leaves preserve the busy-wait path.
#[export_name = "lupos_stop_machine_from_inactive_cpu"]
unsafe extern "C" fn stop_machine_from_inactive_cpu(fn_: *const lupos_stop_callback,
                                                 data: *mut c_void,
                                                 cpus: *const cpumask) -> c_int {
    // SAFETY: The native lifetime and locking contract stated above applies.
    unsafe {
        let mut data_slot = MaybeUninit::<multi_stop_data>::uninit();
        let msdata = data_slot.as_mut_ptr();
        lupos_stop_msdata_init(msdata, fn_, data, 0, cpus);
        let mut done_slot = MaybeUninit::<cpu_stop_done>::uninit();
        let done = done_slot.as_mut_ptr();
        lupos_stop_bug_active_cpu();
        lupos_stop_set_num_threads(msdata, lupos_stop_num_active().wrapping_add(1));
        while !lupos_stop_mutex_trylock() { lupos_stop_relax(); }
        set_state(msdata, multi_stop_state::MULTI_STOP_PREPARE);
        cpu_stop_init_done(done, lupos_stop_num_active());
        lupos_stop_queue_entry(lupos_stop_active_mask(), lupos_stop_multi_callback(),
                              msdata.cast(), done);
        let ret = multi_cpu_stop(msdata.cast());
        while !lupos_stop_completion_done(done) { lupos_stop_relax(); }
        lupos_stop_mutex_unlock();
        if ret != 0 { ret } else { lupos_stop_done_ret(done) }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
