// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010-2017 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 *
 * membarrier system call
 */

// Continued from the existing Rust at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native headers own all constants, layouts, atomics and scheduler primitives.
// This is a source-only proposal, not native or runtime acceptance evidence.
#[cfg(CONFIG_RUST_SCHED_MEMBARRIER)]
compile_error!("SOURCE ONLY HOLD: scheduler membarrier is not admitted");

use core::marker::PhantomData;
use core::mem::MaybeUninit;
use kernel::ffi::{c_int, c_long, c_uint, c_void};
use kernel::bindings::sched_membarrier_native as native;
use native::*;

macro_rules! native_int_constants {
    ($($name:ident),+ $(,)?) => {
        $(const $name: c_int = native::$name as c_int;)+
    };
}

native_int_constants!(
    MEMBARRIER_CMD_QUERY,
    MEMBARRIER_CMD_GLOBAL,
    MEMBARRIER_CMD_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_GET_REGISTRATIONS,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED,
    MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY,
    MEMBARRIER_STATE_GLOBAL_EXPEDITED,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_FLAG_SYNC_CORE,
    MEMBARRIER_FLAG_RSEQ,
    EINVAL, ENOMEM, EPERM,
);
const MEMBARRIER_CMD_FLAG_CPU: c_uint = native::MEMBARRIER_CMD_FLAG_CPU as c_uint;

// Native shared objects and bitmap fields are accessed only with raw pointers.
// The owned MaybeUninit container can be borrowed without claiming initialization
// of the unused native on-stack mask words.
struct CpuMask<'a> {
    storage: &'a mut MaybeUninit<lupos_membarrier_cpumask>,
}

impl<'a> CpuMask<'a> {
    unsafe fn new(storage: &'a mut MaybeUninit<lupos_membarrier_cpumask>) -> Option<Self> {
        // SAFETY: Native zalloc initializes exactly the configured mask storage.
        // Borrowing caller storage avoids moving/copying the native on-stack
        // bitmap and keeps its address stable until all IPIs have completed.
        if unsafe { lupos_membarrier_mask_zalloc(storage.as_mut_ptr()) } {
            Some(Self { storage })
        } else {
            // A failed allocation never constructs a resource-owning guard.
            None
        }
    }

    fn as_ptr(&self) -> *const lupos_membarrier_cpumask {
        self.storage.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut lupos_membarrier_cpumask {
        self.storage.as_mut_ptr()
    }
}

impl Drop for CpuMask<'_> {
    fn drop(&mut self) {
        // SAFETY: Only successful native allocations create this owner.
        unsafe { lupos_membarrier_mask_free(self.as_mut_ptr()) };
    }
}

// Guards are deliberately non-Send/non-Sync: locks and barriers belong to the
// calling task. Declaration order preserves the native cleanup order.
struct MemoryBarrier(PhantomData<*mut ()>);
impl MemoryBarrier {
    unsafe fn new() -> Self {
        unsafe { lupos_membarrier_mb() };
        Self(PhantomData)
    }
}
impl Drop for MemoryBarrier {
    fn drop(&mut self) {
        unsafe { lupos_membarrier_mb() };
    }
}

struct MutexGuard(*mut mutex);
impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        // SAFETY: Caller supplies initialized, static native mutex storage.
        unsafe { lupos_membarrier_mutex_lock(lock) };
        Self(lock)
    }
}
impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { lupos_membarrier_mutex_unlock(self.0) };
    }
}

struct CpusReadGuard(PhantomData<*mut ()>);
impl CpusReadGuard {
    unsafe fn new() -> Self {
        unsafe { lupos_membarrier_cpus_read_lock() };
        Self(PhantomData)
    }
}
impl Drop for CpusReadGuard {
    fn drop(&mut self) {
        unsafe { lupos_membarrier_cpus_read_unlock() };
    }
}

// Each iteration consults the native configured bitmap primitive. The online
// caller holds cpus_read_lock; the possible iterator is used only at init.
struct CpuIter {
    previous: c_int,
    online: bool,
}
impl Iterator for CpuIter {
    type Item = c_int;
    fn next(&mut self) -> Option<c_int> {
        unsafe {
            let cpu = if self.online {
                lupos_membarrier_next_online_cpu(self.previous)
            } else {
                lupos_membarrier_next_possible_cpu(self.previous)
            };
            if cpu as c_uint >= lupos_membarrier_nr_cpu_ids() {
                return None;
            }
            self.previous = cpu;
            Some(cpu)
        }
    }
}

#[cfg(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)]
const MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK: c_int =
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE;
#[cfg(not(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE))]
const MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK: c_int = 0;

#[cfg(CONFIG_RSEQ)]
const MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK: c_int =
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ;
#[cfg(not(CONFIG_RSEQ))]
const MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK: c_int = 0;

const MEMBARRIER_CMD_BITMASK: c_int = MEMBARRIER_CMD_GLOBAL |
    MEMBARRIER_CMD_GLOBAL_EXPEDITED |
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED |
    MEMBARRIER_CMD_PRIVATE_EXPEDITED |
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED |
    MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK |
    MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK |
    MEMBARRIER_CMD_GET_REGISTRATIONS;

/// Initialize the native per-CPU mutexes from the native core initcall.
///
/// # Safety
/// Called once at core init, before membarrier syscalls can acquire these locks.
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_membarrier_init() -> c_int {
    unsafe {
        for cpu in (CpuIter { previous: -1, online: false }) {
            lupos_membarrier_cpu_mutex_init(cpu);
        }
        0
    }
}

unsafe extern "C" fn ipi_mb(_info: *mut c_void) {
    unsafe { lupos_membarrier_mb() };
}

#[cfg(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)]
unsafe extern "C" fn ipi_sync_core(_info: *mut c_void) {
    unsafe {
        // Core serialization may be deferred until usermode; the mb cannot be.
        lupos_membarrier_mb();
        lupos_membarrier_sync_core_before_usermode();
    }
}

#[cfg(CONFIG_RSEQ)]
unsafe extern "C" fn ipi_rseq(_info: *mut c_void) {
    unsafe {
        lupos_membarrier_mb();
        if lupos_membarrier_rseq_v2_current() {
            lupos_membarrier_rseq_switch_current();
        } else {
            lupos_membarrier_rseq_force_update();
        }
    }
}

unsafe extern "C" fn ipi_sync_rq_state(info: *mut c_void) {
    // SAFETY: The synchronous sender's current->mm reference lasts until this
    // callback completes. Recheck the interrupted task before updating its rq.
    unsafe {
        let mm = info.cast::<mm_struct>();
        if lupos_membarrier_current_mm() != mm { return; }
        lupos_membarrier_this_rq_state_write(lupos_membarrier_mm_state_read(mm));
        lupos_membarrier_mb();
    }
}

/// Clear exec's membarrier state after ordering preceding user accesses.
///
/// # Safety
/// Native exec caller owns the live mm and current-runqueue update context.
#[no_mangle]
pub unsafe extern "C" fn membarrier_exec_mmap(mm: *mut mm_struct) {
    unsafe {
        lupos_membarrier_mb();
        lupos_membarrier_mm_state_set(mm, 0);
        lupos_membarrier_this_rq_state_write(0);
    }
}

/// Publish the current task's changed mm state to its runqueue.
///
/// # Safety
/// Caller provides the native preemption/IRQ exclusion and a live next_mm (or
/// NULL) required by exit_mm and kthread use/unuse-mm transitions.
#[no_mangle]
pub unsafe extern "C" fn membarrier_update_current_mm(next_mm: *mut mm_struct) {
    unsafe {
        let rq = lupos_membarrier_this_rq();
        let mut membarrier_state: c_int = 0;
        if !next_mm.is_null() { membarrier_state = lupos_membarrier_mm_state_read(next_mm); }
        if lupos_membarrier_rq_state_read(rq) == membarrier_state { return; }
        lupos_membarrier_rq_state_write(rq, membarrier_state);
    }
}

unsafe fn membarrier_global_expedited() -> c_int {
    // SAFETY: Syscall context; native runqueue current tasks are inspected only
    // under RCU, and the hotplug lock remains held through synchronous IPIs.
    unsafe {
    if lupos_membarrier_num_online_cpus() == 1 { return 0; }
    let mut mask_storage = MaybeUninit::uninit();
    let Some(mut tmpmask) = CpuMask::new(&mut mask_storage) else { return -ENOMEM; };
    let _mb = MemoryBarrier::new();
    let _ipi = MutexGuard::new(lupos_membarrier_ipi_mutex());
    let _cpus = CpusReadGuard::new();
    lupos_membarrier_rcu_read_lock();
    for cpu in (CpuIter { previous: -1, online: true }) {
        if cpu == lupos_membarrier_raw_cpu() { continue; }
        if (lupos_membarrier_rq_state_read(lupos_membarrier_cpu_rq(cpu)) & MEMBARRIER_STATE_GLOBAL_EXPEDITED) == 0 { continue; }
        let p = lupos_membarrier_rq_curr_rcu(lupos_membarrier_cpu_rq(cpu));
        if lupos_membarrier_task_mm(p).is_null() { continue; }
        lupos_membarrier_mask_set_cpu(cpu as c_uint, tmpmask.as_mut_ptr());
    }
    lupos_membarrier_rcu_read_unlock();
    lupos_membarrier_preempt_disable();
    lupos_membarrier_call_many(tmpmask.as_ptr(), Some(ipi_mb), core::ptr::null_mut(), true);
    lupos_membarrier_preempt_enable();
    0
    }
}

unsafe fn membarrier_private_expedited(flags: c_int, cpu_id: c_int) -> c_int {
    // SAFETY: The syscall's current task pins mm across sleeping and all
    // synchronous callbacks; RCU pins only remote task pointers while sampled.
    unsafe {
    let mm = lupos_membarrier_current_mm();
    let ipi_func: smp_call_func_t;
    if flags == MEMBARRIER_FLAG_SYNC_CORE {
        #[cfg(not(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE))]
        { return -EINVAL; }
        #[cfg(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)]
        {
            if lupos_membarrier_mm_state_read(mm) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY == 0 { return -EPERM; }
            ipi_func = Some(ipi_sync_core);
            lupos_membarrier_prepare_sync_core_cmd(mm);
        }
    } else if flags == MEMBARRIER_FLAG_RSEQ {
        #[cfg(not(CONFIG_RSEQ))]
        { return -EINVAL; }
        #[cfg(CONFIG_RSEQ)]
        {
            if lupos_membarrier_mm_state_read(mm) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY == 0 { return -EPERM; }
            ipi_func = Some(ipi_rseq);
        }
    } else {
        lupos_membarrier_warn_private_flags(flags);
        if lupos_membarrier_mm_state_read(mm) & MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY == 0 { return -EPERM; }
        ipi_func = Some(ipi_mb);
    }
    if flags != MEMBARRIER_FLAG_SYNC_CORE && (lupos_membarrier_mm_users_read(mm) == 1 || lupos_membarrier_num_online_cpus() == 1) { return 0; }
    let _mb = MemoryBarrier::new();
    if cpu_id >= 0 {
        if cpu_id as c_uint >= lupos_membarrier_nr_cpu_ids()
            || !lupos_membarrier_cpu_possible(cpu_id as c_uint) { return 0; }
        let _ipi = MutexGuard::new(lupos_membarrier_cpu_mutex(cpu_id));
        let _cpus = CpusReadGuard::new();
        // Online state must be checked under hotplug protection, as in C.
        if !lupos_membarrier_cpu_online(cpu_id as c_uint) { return 0; }
        lupos_membarrier_rcu_read_lock();
        let p = lupos_membarrier_rq_curr_rcu(lupos_membarrier_cpu_rq(cpu_id));
        if p.is_null() || lupos_membarrier_task_mm(p) != mm { lupos_membarrier_rcu_read_unlock(); return 0; }
        lupos_membarrier_rcu_read_unlock();
        lupos_membarrier_call_single(cpu_id, ipi_func, core::ptr::null_mut(), true);
    } else {
        let mut mask_storage = MaybeUninit::uninit();
        let Some(mut tmpmask) = CpuMask::new(&mut mask_storage) else { return -ENOMEM; };
        let _ipi = MutexGuard::new(lupos_membarrier_ipi_mutex());
        let _cpus = CpusReadGuard::new();
        lupos_membarrier_rcu_read_lock();
        for cpu in (CpuIter { previous: -1, online: true }) {
            let p = lupos_membarrier_rq_curr_rcu(lupos_membarrier_cpu_rq(cpu));
            if !p.is_null() && lupos_membarrier_task_mm(p) == mm {
                lupos_membarrier_mask_set_cpu(cpu as c_uint, tmpmask.as_mut_ptr());
            }
        }
        lupos_membarrier_rcu_read_unlock();
        if flags != MEMBARRIER_FLAG_SYNC_CORE { lupos_membarrier_preempt_disable(); lupos_membarrier_call_many(tmpmask.as_ptr(), ipi_func, core::ptr::null_mut(), true); lupos_membarrier_preempt_enable(); }
        else { lupos_membarrier_call_each(tmpmask.as_ptr(), ipi_func, core::ptr::null_mut(), true); }
    }
    0
    }
}

unsafe fn sync_runqueues_membarrier_state(mm: *mut mm_struct) -> c_int {
    // SAFETY: Caller pins mm. READY is published only after the grace period,
    // runqueue state callbacks and their mandatory post-publication barriers.
    unsafe {
    let state = lupos_membarrier_mm_state_read(mm);
    if lupos_membarrier_mm_users_read(mm) == 1 || lupos_membarrier_num_online_cpus() == 1 { lupos_membarrier_this_rq_state_write(state); lupos_membarrier_mb(); return 0; }
    let mut mask_storage = MaybeUninit::uninit();
    let Some(mut tmpmask) = CpuMask::new(&mut mask_storage) else { return -ENOMEM; };
    lupos_membarrier_synchronize_rcu();
    let _ipi = MutexGuard::new(lupos_membarrier_ipi_mutex());
    lupos_membarrier_cpus_read_lock(); lupos_membarrier_rcu_read_lock();
    for cpu in (CpuIter { previous: -1, online: true }) {
        let rq = lupos_membarrier_cpu_rq(cpu);
        let p = lupos_membarrier_rq_curr_rcu(rq);
        if !p.is_null() && lupos_membarrier_task_mm(p) == mm {
            lupos_membarrier_mask_set_cpu(cpu as c_uint, tmpmask.as_mut_ptr());
        }
    }
    lupos_membarrier_rcu_read_unlock(); lupos_membarrier_call_each(tmpmask.as_ptr(), Some(ipi_sync_rq_state), mm.cast(), true); drop(tmpmask); lupos_membarrier_cpus_read_unlock(); 0
    }
}

unsafe fn membarrier_register_global_expedited() -> c_int {
    unsafe {
    let mm = lupos_membarrier_current_mm();
    if lupos_membarrier_mm_state_read(mm) & MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY != 0 { return 0; }
    lupos_membarrier_mm_state_or(mm, MEMBARRIER_STATE_GLOBAL_EXPEDITED);
    let ret = sync_runqueues_membarrier_state(mm); if ret != 0 { return ret; }
    lupos_membarrier_mm_state_or(mm, MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY); 0
    }
}

unsafe fn membarrier_register_private_expedited(flags: c_int) -> c_int {
    unsafe {
    let mm = lupos_membarrier_current_mm();
    let mut ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY;
    let mut set_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED;
    if flags == MEMBARRIER_FLAG_SYNC_CORE { if !cfg!(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE) { return -EINVAL; } ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY; }
    else if flags == MEMBARRIER_FLAG_RSEQ { if !cfg!(CONFIG_RSEQ) { return -EINVAL; } ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY; }
    else { lupos_membarrier_warn_register_flags(flags); }
    if lupos_membarrier_mm_state_read(mm) & ready_state == ready_state { return 0; }
    if flags & MEMBARRIER_FLAG_SYNC_CORE != 0 { set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE; }
    if flags & MEMBARRIER_FLAG_RSEQ != 0 { set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ; }
    lupos_membarrier_mm_state_or(mm, set_state);
    let ret = sync_runqueues_membarrier_state(mm); if ret != 0 { return ret; }
    lupos_membarrier_mm_state_or(mm, ready_state); 0
    }
}

unsafe fn membarrier_get_registrations() -> c_int {
    unsafe {
    let mm = lupos_membarrier_current_mm(); let mut registrations_mask = 0; let mut state = lupos_membarrier_mm_state_read(mm);
    const STATES: [c_int; 4] = [MEMBARRIER_STATE_GLOBAL_EXPEDITED | MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED | MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE | MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY, MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ | MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY];
    const CMDS: [c_int; STATES.len()] = [MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ];
    for (&bits, &cmd) in STATES.iter().zip(CMDS.iter()) {
        if state & bits != 0 { registrations_mask |= cmd; state &= !bits; }
    }
    lupos_membarrier_warn_registration_state(state); registrations_mask
    }
}

// The copied dispatcher still owns every command and error decision. The native
// SYSCALL_DEFINE3 thunk exports the architecture ABI and calls the long bridge.
unsafe fn sys_membarrier(cmd: c_int, flags: c_uint, mut cpu_id: c_int) -> c_int {
    unsafe {
    if cmd == MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ { if flags != 0 && flags != MEMBARRIER_CMD_FLAG_CPU { return -EINVAL; } } else if flags != 0 { return -EINVAL; }
    if flags & MEMBARRIER_CMD_FLAG_CPU == 0 { cpu_id = -1; }
    match cmd {
        MEMBARRIER_CMD_QUERY => { let mut mask = MEMBARRIER_CMD_BITMASK; if lupos_membarrier_nohz_full_enabled() { mask &= !MEMBARRIER_CMD_GLOBAL; } mask }
        MEMBARRIER_CMD_GLOBAL => { if lupos_membarrier_nohz_full_enabled() { return -EINVAL; } if lupos_membarrier_num_online_cpus() > 1 { lupos_membarrier_synchronize_rcu(); } 0 }
        MEMBARRIER_CMD_GLOBAL_EXPEDITED => membarrier_global_expedited(),
        MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED => membarrier_register_global_expedited(),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED => membarrier_private_expedited(0, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED => membarrier_register_private_expedited(0),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE => membarrier_private_expedited(MEMBARRIER_FLAG_SYNC_CORE, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE => membarrier_register_private_expedited(MEMBARRIER_FLAG_SYNC_CORE),
        MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ => membarrier_private_expedited(MEMBARRIER_FLAG_RSEQ, cpu_id),
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ => membarrier_register_private_expedited(MEMBARRIER_FLAG_RSEQ),
        MEMBARRIER_CMD_GET_REGISTRATIONS => membarrier_get_registrations(),
        _ => -EINVAL,
    }
    }
}

/// Native syscall thunk target; sign-extend errno and results to native long.
///
/// # Safety
/// Entered only from the native membarrier syscall wrapper in user-task context.
#[no_mangle]
pub unsafe extern "C" fn lupos_membarrier_syscall(
    cmd: c_int, flags: c_uint, cpu_id: c_int,
) -> c_long {
    unsafe { sys_membarrier(cmd, flags, cpu_id) as c_long }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
