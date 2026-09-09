// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@android.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    type raw_notifier_head;
    type raw_spinlock_t;
    type notifier_block;
    type syscore;
    type syscore_ops;

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn raw_notifier_call_chain(chain: *mut raw_notifier_head, event: cpu_pm_event, data: *mut core::ffi::c_void) -> i32;
    fn raw_notifier_call_chain_robust(chain: *mut raw_notifier_head, event_up: cpu_pm_event, event_down: cpu_pm_event, data: *mut core::ffi::c_void) -> i32;
    fn raw_notifier_chain_register(chain: *mut raw_notifier_head, nb: *mut notifier_block) -> i32;
    fn raw_notifier_chain_unregister(chain: *mut raw_notifier_head, nb: *mut notifier_block) -> i32;
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: usize);
    fn notifier_to_errno(ret: i32) -> i32;
    fn register_syscore(syscore: *mut syscore);
}

type cpu_pm_event = i32;

#[repr(C)]
struct CpuPmNotifier {
    chain: raw_notifier_head,
    lock: raw_spinlock_t,
}

// RAW_NOTIFIER_INIT and __RAW_SPIN_LOCK_UNLOCKED are C initializers whose
// definitions are supplied by the kernel headers.
static mut cpu_pm_notifier: CpuPmNotifier = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

unsafe fn cpu_pm_notify(event: cpu_pm_event) -> i32 {
    let ret;
    rcu_read_lock();
    ret = raw_notifier_call_chain(&mut cpu_pm_notifier.chain, event, core::ptr::null_mut());
    rcu_read_unlock();
    notifier_to_errno(ret)
}

unsafe fn cpu_pm_notify_robust(event_up: cpu_pm_event, event_down: cpu_pm_event) -> i32 {
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut cpu_pm_notifier.lock, &mut flags);
    let ret = raw_notifier_call_chain_robust(
        &mut cpu_pm_notifier.chain,
        event_up,
        event_down,
        core::ptr::null_mut(),
    );
    raw_spin_unlock_irqrestore(&mut cpu_pm_notifier.lock, flags);
    notifier_to_errno(ret)
}

/**
 * cpu_pm_register_notifier - register a driver with cpu_pm
 * @nb: notifier block to register
 *
 * Add a driver to a list of drivers that are notified about
 * CPU and CPU cluster low power entry and exit.
 *
 * This function has the same return conditions as raw_notifier_chain_register.
 */
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_register_notifier(nb: *mut notifier_block) -> i32 {
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut cpu_pm_notifier.lock, &mut flags);
    let ret = raw_notifier_chain_register(&mut cpu_pm_notifier.chain, nb);
    raw_spin_unlock_irqrestore(&mut cpu_pm_notifier.lock, flags);
    ret
}

/**
 * cpu_pm_unregister_notifier - unregister a driver with cpu_pm
 * @nb: notifier block to be unregistered
 *
 * Remove a driver from the CPU PM notifier list.
 *
 * This function has the same return conditions as raw_notifier_chain_unregister.
 */
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_unregister_notifier(nb: *mut notifier_block) -> i32 {
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut cpu_pm_notifier.lock, &mut flags);
    let ret = raw_notifier_chain_unregister(&mut cpu_pm_notifier.chain, nb);
    raw_spin_unlock_irqrestore(&mut cpu_pm_notifier.lock, flags);
    ret
}

/** cpu_pm_enter - CPU low power entry notifier */
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_enter() -> i32 {
    cpu_pm_notify_robust(CPU_PM_ENTER, CPU_PM_ENTER_FAILED)
}

/** cpu_pm_exit - CPU low power exit notifier */
#[no_mangle]
pub unsafe extern "C" fn cpu_pm_exit() -> i32 {
    cpu_pm_notify(CPU_PM_EXIT)
}

/** cpu_cluster_pm_enter - CPU cluster low power entry notifier */
#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_pm_enter() -> i32 {
    cpu_pm_notify_robust(CPU_CLUSTER_PM_ENTER, CPU_CLUSTER_PM_ENTER_FAILED)
}

/** cpu_cluster_pm_exit - CPU cluster low power exit notifier */
#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_pm_exit() -> i32 {
    cpu_pm_notify(CPU_CLUSTER_PM_EXIT)
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn cpu_pm_suspend(_data: *mut core::ffi::c_void) -> i32 {
    let ret = cpu_pm_enter();
    if ret != 0 {
        return ret;
    }
    cpu_cluster_pm_enter()
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn cpu_pm_resume(_data: *mut core::ffi::c_void) {
    cpu_cluster_pm_exit();
    cpu_pm_exit();
}

#[cfg(CONFIG_PM)]
static cpu_pm_syscore_ops: syscore_ops = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

#[cfg(CONFIG_PM)]
static mut cpu_pm_syscore: syscore = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

#[cfg(CONFIG_PM)]
unsafe fn cpu_pm_init() -> i32 {
    register_syscore(&mut cpu_pm_syscore);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
