// SPDX-License-Identifier: GPL-2.0-only
/* Common SMP CPU bringup/teardown functions. */

// Continued from Rust blob ee389ad855b53081701f78e01086dc434aa15ba9.
// Native oracle: smpboot.c blob 4503b60ce9bd2ad574fea4ceabf21b3e08b61f0a.
// Configured native declarations own all object layouts and header primitives.
#![no_std]
compile_error!("SOURCE ONLY HOLD: smpboot Rust owner is not admitted");

use kernel::bindings::smpboot_native::*;
use kernel::ffi::{c_int, c_uint, c_void};

#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
#[inline]
unsafe fn per_cpu_idle_threads(cpu: c_uint) -> *mut task_struct {
    // SAFETY: The caller supplies a possible CPU in the boot/hotplug protocol.
    unsafe { lupos_smpboot_idle_get(cpu) }
}

/// Return the cached idle task, or the original native ENOMEM error pointer.
///
/// # Safety
/// The caller supplies a possible CPU under the native boot/hotplug protocol.
#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
#[no_mangle]
pub unsafe extern "C" fn idle_thread_get(cpu: c_uint) -> *mut task_struct {
    // SAFETY: The idle cache is native per-CPU storage with boot lifetime.
    unsafe {
        let tsk = per_cpu_idle_threads(cpu);
        if tsk.is_null() {
            lupos_smpboot_err_ptr(-LUPOS_SMPBOOT_ENOMEM)
        } else {
            tsk
        }
    }
}

/// Cache the boot CPU's current task through the native __init entry adapter.
///
/// # Safety
/// Called only during boot, before init memory is released, on the boot CPU.
#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
#[export_name = "lupos_smpboot_idle_thread_set_boot_cpu"]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn idle_thread_set_boot_cpu() {
    // SAFETY: Early boot owns this CPU's idle slot; current remains live.
    unsafe {
        let cpu = lupos_smpboot_processor_id();
        lupos_smpboot_idle_set(cpu, lupos_smpboot_current());
    }
}

#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
#[inline(always)]
unsafe fn idle_init(cpu: c_uint) {
    // SAFETY: Called during boot for a possible nonboot CPU. A successful
    // fork_idle task is retained permanently in that CPU's native idle slot.
    unsafe {
        let tsk = per_cpu_idle_threads(cpu);
        if tsk.is_null() {
            let new_tsk = fork_idle(cpu as c_int);
            if lupos_smpboot_is_err(new_tsk) {
                lupos_smpboot_err_fork_idle(cpu);
            } else {
                lupos_smpboot_idle_set(cpu, new_tsk);
            }
        }
    }
}

/// Populate the idle cache through the native __init entry adapter.
///
/// # Safety
/// Called only during boot, before init memory is released. The native CPU
/// topology and fork_idle initialization prerequisites must already be valid.
#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
#[export_name = "lupos_smpboot_idle_threads_init"]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn idle_threads_init() {
    // SAFETY: The boot protocol stabilizes possible CPUs and owns idle slots.
    unsafe {
        let boot_cpu = lupos_smpboot_processor_id();
        let mut cpu = 0;
        loop {
            cpu = lupos_smpboot_next_possible_cpu(cpu);
            if cpu >= lupos_smpboot_cpu_iter_limit() {
                break;
            }
            if cpu != boot_cpu {
                idle_init(cpu);
            }
            cpu = cpu.wrapping_add(1);
        }
    }
}

// The native creation leaf supplies this symbol with its exact int(void *)
// prototype. No nullable function pointer crosses the Rust/C boundary.
#[no_mangle]
unsafe extern "C" fn smpboot_thread_fn(data: *mut c_void) -> c_int {
    // SAFETY: Creation transfers its native zeroed td to this worker. The
    // descriptor and callbacks stay live until stop_put joins the worker.
    // Only this worker modifies td.status; each preempt-disable is paired on
    // every branch. The stop branch frees td once and never touches it again.
    unsafe {
        let td = data.cast::<smpboot_thread_data>();
        let ht = lupos_smpboot_data_ht(td);
        loop {
            lupos_smpboot_set_interruptible();
            lupos_smpboot_preempt_disable();
            if kthread_should_stop() {
                lupos_smpboot_set_running_stop();
                lupos_smpboot_preempt_enable();
                // Cleanup must mirror setup, including a parked worker.
                if lupos_smpboot_has_cleanup(ht)
                    && lupos_smpboot_data_status(td) != HP_THREAD_NONE
                {
                    lupos_smpboot_cleanup(
                        ht,
                        lupos_smpboot_data_cpu(td),
                        lupos_smpboot_cpu_online(lupos_smpboot_data_cpu(td)),
                    );
                }
                lupos_smpboot_free_data(td);
                return 0;
            }
            if kthread_should_park() {
                lupos_smpboot_set_running_park();
                lupos_smpboot_preempt_enable();
                if lupos_smpboot_has_park(ht)
                    && lupos_smpboot_data_status(td) == HP_THREAD_ACTIVE
                {
                    lupos_smpboot_bug_park_cpu(td);
                    lupos_smpboot_park(ht, lupos_smpboot_data_cpu(td));
                    lupos_smpboot_set_data_status(td, HP_THREAD_PARKED);
                }
                kthread_parkme();
                // A wakeup from park can request stop rather than work.
                continue;
            }
            lupos_smpboot_bug_run_cpu(td);
            match lupos_smpboot_data_status(td) {
                HP_THREAD_NONE => {
                    lupos_smpboot_set_running_setup();
                    lupos_smpboot_preempt_enable();
                    if lupos_smpboot_has_setup(ht) {
                        lupos_smpboot_setup(ht, lupos_smpboot_data_cpu(td));
                    }
                    lupos_smpboot_set_data_status(td, HP_THREAD_ACTIVE);
                    continue;
                }
                HP_THREAD_PARKED => {
                    lupos_smpboot_set_running_unpark();
                    lupos_smpboot_preempt_enable();
                    if lupos_smpboot_has_unpark(ht) {
                        lupos_smpboot_unpark(ht, lupos_smpboot_data_cpu(td));
                    }
                    lupos_smpboot_set_data_status(td, HP_THREAD_ACTIVE);
                    continue;
                }
                _ => {}
            }
            if lupos_smpboot_thread_should_run(ht, lupos_smpboot_data_cpu(td)) == 0 {
                lupos_smpboot_preempt_enable_no_resched();
                schedule();
            } else {
                lupos_smpboot_set_running_work();
                lupos_smpboot_preempt_enable();
                lupos_smpboot_thread_run(ht, lupos_smpboot_data_cpu(td));
            }
        }
    }
}

unsafe fn __smpboot_create_thread(ht: *mut smp_hotplug_thread, cpu: c_uint) -> c_int {
    // SAFETY: The descriptor, callbacks and per-CPU store are live; the caller
    // holds smpboot_threads_lock. Before successful creation td belongs here;
    // afterwards only the worker frees it, including registration unwind.
    unsafe {
        let mut tsk = lupos_smpboot_task_get(ht, cpu);
        if !tsk.is_null() {
            return 0;
        }
        let td = lupos_smpboot_alloc_data(cpu);
        if td.is_null() {
            return -LUPOS_SMPBOOT_ENOMEM;
        }
        lupos_smpboot_set_data_cpu(td, cpu);
        lupos_smpboot_set_data_ht(td, ht);

        tsk = lupos_smpboot_create_on_cpu(td, cpu, ht);
        if lupos_smpboot_is_err(tsk) {
            lupos_smpboot_free_data(td);
            return lupos_smpboot_ptr_err(tsk) as c_int;
        }
        kthread_set_per_cpu(tsk, cpu as c_int);
        // Park before retaining/publishing the task, exactly as the C owner.
        let _ = kthread_park(tsk);
        lupos_smpboot_get_task_struct(tsk);
        lupos_smpboot_task_set(ht, cpu, tsk);
        if lupos_smpboot_has_create(ht) {
            // The migration create callback requires the task off-runqueue.
            if lupos_smpboot_wait_parked(tsk) == 0 {
                lupos_smpboot_warn_create_inactive();
            } else {
                lupos_smpboot_create(ht, cpu);
            }
        }
        0
    }
}

/// Create parked registered workers for a CPU being brought online.
///
/// # Safety
/// The caller follows the CPU-hotplug bringup protocol for a possible CPU.
#[no_mangle]
pub unsafe extern "C" fn smpboot_create_threads(cpu: c_uint) -> c_int {
    // SAFETY: The native mutex pins list nodes and their live descriptors.
    unsafe {
        let mut ret = 0;
        lupos_smpboot_lock();
        let head = lupos_smpboot_list_head();
        let mut node = lupos_smpboot_list_next(head);
        while node != head {
            ret = __smpboot_create_thread(lupos_smpboot_list_entry(node), cpu);
            if ret != 0 {
                break;
            }
            node = lupos_smpboot_list_next(node);
        }
        lupos_smpboot_unlock();
        ret
    }
}

unsafe fn smpboot_unpark_thread(ht: *mut smp_hotplug_thread, cpu: c_uint) {
    // SAFETY: The mutex stabilizes ht/store; the caller has completed creation.
    // Like C, non-selfparking descriptors require a nonnull stored task here.
    unsafe {
        let tsk = lupos_smpboot_task_get(ht, cpu);
        if !lupos_smpboot_selfparking(ht) {
            kthread_unpark(tsk);
        }
    }
}

/// Unpark registered non-selfparking workers in forward list order.
///
/// # Safety
/// The caller follows CPU-hotplug ordering after successful worker creation.
#[no_mangle]
pub unsafe extern "C" fn smpboot_unpark_threads(cpu: c_uint) -> c_int {
    // SAFETY: The mutex keeps every traversed descriptor and store valid.
    unsafe {
        lupos_smpboot_lock();
        let head = lupos_smpboot_list_head();
        let mut node = lupos_smpboot_list_next(head);
        while node != head {
            smpboot_unpark_thread(lupos_smpboot_list_entry(node), cpu);
            node = lupos_smpboot_list_next(node);
        }
        lupos_smpboot_unlock();
        0
    }
}

unsafe fn smpboot_park_thread(ht: *mut smp_hotplug_thread, cpu: c_uint) {
    // SAFETY: The mutex pins ht/store and any published task reference.
    unsafe {
        let tsk = lupos_smpboot_task_get(ht, cpu);
        if !tsk.is_null() && !lupos_smpboot_selfparking(ht) {
            let _ = kthread_park(tsk);
        }
    }
}

/// Park existing non-selfparking workers in reverse list order.
///
/// # Safety
/// The caller follows the CPU-hotplug teardown protocol for the target CPU.
#[no_mangle]
pub unsafe extern "C" fn smpboot_park_threads(cpu: c_uint) -> c_int {
    // SAFETY: The native mutex protects the complete reverse traversal.
    unsafe {
        lupos_smpboot_lock();
        let head = lupos_smpboot_list_head();
        let mut node = lupos_smpboot_list_prev(head);
        while node != head {
            smpboot_park_thread(lupos_smpboot_list_entry(node), cpu);
            node = lupos_smpboot_list_prev(node);
        }
        lupos_smpboot_unlock();
        0
    }
}

unsafe fn smpboot_destroy_threads(ht: *mut smp_hotplug_thread) {
    // SAFETY: CPU read lock and smpboot mutex prevent competing lifecycle
    // changes. stop_put joins the worker and releases the retained task ref;
    // the descriptor/store remain live until every possible CPU is cleared.
    unsafe {
        let mut cpu = 0;
        loop {
            cpu = lupos_smpboot_next_possible_cpu(cpu);
            if cpu >= lupos_smpboot_cpu_iter_limit() {
                break;
            }
            // Include parked threads on offline CPUs, not just online CPUs.
            let tsk = lupos_smpboot_task_get(ht, cpu);
            if !tsk.is_null() {
                let _ = kthread_stop_put(tsk);
                lupos_smpboot_task_set(ht, cpu, core::ptr::null_mut());
            }
            cpu = cpu.wrapping_add(1);
        }
    }
}

/// Create and start the descriptor's workers on all online CPUs.
///
/// # Safety
/// The unregistered descriptor has a valid per-CPU task store and required
/// callbacks. Descriptor/store/callback lifetimes extend through unregister.
/// The caller may sleep and must obey the native lock ordering contract.
#[no_mangle]
pub unsafe extern "C" fn smpboot_register_percpu_thread(
    plug_thread: *mut smp_hotplug_thread,
) -> c_int {
    // SAFETY: CPU read lock precedes the owner mutex. Both remain held across
    // creation, unpark and complete partial-failure unwind. No list entry is
    // published before all online CPUs succeed, and both locks always release.
    unsafe {
        let mut ret = 0;
        lupos_smpboot_cpus_read_lock();
        lupos_smpboot_lock();
        let mut cpu = 0;
        loop {
            cpu = lupos_smpboot_next_online_cpu(cpu);
            if cpu >= lupos_smpboot_cpu_iter_limit() {
                break;
            }
            ret = __smpboot_create_thread(plug_thread, cpu);
            if ret != 0 {
                smpboot_destroy_threads(plug_thread);
                break;
            }
            smpboot_unpark_thread(plug_thread, cpu);
            cpu = cpu.wrapping_add(1);
        }
        if ret == 0 {
            lupos_smpboot_list_add(plug_thread);
        }
        lupos_smpboot_unlock();
        lupos_smpboot_cpus_read_unlock();
        ret
    }
}

/// Remove a registered descriptor and stop workers on all possible CPUs.
///
/// # Safety
/// The descriptor is registered and remains live with its store and callbacks
/// until this call returns. The caller may sleep and obeys lock ordering.
#[no_mangle]
pub unsafe extern "C" fn smpboot_unregister_percpu_thread(
    plug_thread: *mut smp_hotplug_thread,
) {
    // SAFETY: Remove under CPU read lock then owner mutex before destruction;
    // stop_put joins every worker before descriptor ownership returns.
    unsafe {
        lupos_smpboot_cpus_read_lock();
        lupos_smpboot_lock();
        lupos_smpboot_list_del(plug_thread);
        smpboot_destroy_threads(plug_thread);
        lupos_smpboot_unlock();
        lupos_smpboot_cpus_read_unlock();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
