// SPDX-License-Identifier: GPL-2.0-only
//! Workqueue diagnostics; source authority 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
//!
//! Source-only proposal. All native representations, constants, primitives,
//! globals, callback tables, exports and registration belong to common. Native
//! generation, callback/varargs ABI, diagnostic-site metadata, notrace, init
//! sections and stack/code-generation admission remain unproved and held.

use super::common::*;
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
use super::completion::cancel_work_sync;
use super::completion::wq_barrier_func;
use super::dispatch::{
    find_worker_executing_work,
    pwq_is_empty, //
};
use super::execution::format_worker_id;
#[cfg(CONFIG_WQ_WATCHDOG)]
use super::dispatch::is_percpu_pool;
#[cfg(CONFIG_SYSFS)]
use super::resources::{
    apply_workqueue_attrs_locked,
    copy_workqueue_attrs,
    free_workqueue_attrs,
    parse_affn_scope,
    workqueue_apply_unbound_cpumask,
    workqueue_set_max_active, //
};
#[cfg(CONFIG_SYSFS)]
use core::mem::MaybeUninit;
use core::mem::{
    offset_of,
    size_of, //
};
use core::ptr::{
    addr_of,
    addr_of_mut,
    null_mut, //
};

/// Supplies the debug object's callback hint (C 626–629).
///
/// # Safety
/// `addr` is a live native work item supplied by the debug-object subsystem.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn work_debug_hint(addr: *mut c_void) -> *mut c_void {
    // SAFETY: The native leaf preserves the C function/data-pointer conversion.
    unsafe { lupos_wq_work_func_hint((*addr.cast::<work_struct>()).func) }
}

/// Tests the configured static initialization bit (C 631–636).
///
/// # Safety
/// `addr` is a live work item with native work-data storage.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn work_is_static_object(addr: *mut c_void) -> bool {
    // SAFETY: test_bit acts on the actual atomic-long storage, not a Rust copy.
    unsafe { lupos_wq_work_static_test_bit(addr.cast::<work_struct>()) }
}

/// Cancels an active object before debug reinitialization (C 642–654).
///
/// # Safety
/// The debug framework supplies a valid object and a context permitting sync cancel.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn work_fixup_init(
    addr: *mut c_void,
    state: debug_obj_state,
) -> bool {
    // SAFETY: Only the native active-state case permits cancellation/reinit.
    unsafe {
        if state != ODEBUG_STATE_ACTIVE {
            return false;
        }
        cancel_work_sync(addr.cast::<work_struct>());
        debug_object_init(addr, lupos_wq_work_debug_descr());
        true
    }
}

/// Cancels an active object before freeing its debug record (C 660–672).
///
/// # Safety
/// The debug framework supplies a valid object and a context permitting sync cancel.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn work_fixup_free(
    addr: *mut c_void,
    state: debug_obj_state,
) -> bool {
    // SAFETY: Cancellation completes before the object's tracking record is freed.
    unsafe {
        if state != ODEBUG_STATE_ACTIVE {
            return false;
        }
        cancel_work_sync(addr.cast::<work_struct>());
        debug_object_free(addr, lupos_wq_work_debug_descr());
        true
    }
}

/// Activates native debug tracking (C 682–685).
///
/// # Safety
/// `work` is initialized and retained while debug tracking is active.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe fn debug_work_activate(work: *mut work_struct) {
    // SAFETY: The sole common-owned descriptor retains its native static lifetime.
    unsafe { debug_object_activate(work.cast::<c_void>(), lupos_wq_work_debug_descr()) }
}

/// Deactivates native debug tracking (C 687–690).
///
/// # Safety
/// `work` is a valid tracked object being dequeued or canceled.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe fn debug_work_deactivate(work: *mut work_struct) {
    // SAFETY: The caller retains the live object through deactivation.
    unsafe { debug_object_deactivate(work.cast::<c_void>(), lupos_wq_work_debug_descr()) }
}

/// Registers heap/static or stack work with debug objects (C 692–698).
///
/// # Safety
/// `work` points to valid work storage; onstack truthfully describes its lifetime.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn __init_work(work: *mut work_struct, onstack: c_int) {
    // SAFETY: Tracking precedes publication and uses the configured stack variant.
    unsafe {
        if onstack != 0 {
            debug_object_init_on_stack(work.cast::<c_void>(), lupos_wq_work_debug_descr());
        } else {
            debug_object_init(work.cast::<c_void>(), lupos_wq_work_debug_descr());
        }
    }
}

/// Releases tracking of completed stack work (C 701–704).
///
/// # Safety
/// `work` is live stack storage and no longer queued or executing.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn destroy_work_on_stack(work: *mut work_struct) {
    // SAFETY: The caller has completed work before ending the tracked lifetime.
    unsafe { debug_object_free(work.cast::<c_void>(), lupos_wq_work_debug_descr()) }
}

/// Destroys the stack timer before releasing work tracking (C 707–711).
///
/// # Safety
/// `work` is live stack delayed-work storage with neither work nor timer active.
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub(super) unsafe extern "C" fn destroy_delayed_work_on_stack(work: *mut delayed_work) {
    // SAFETY: The timer's own stack debug lifetime ends before the work record.
    unsafe {
        lupos_wq_timer_destroy_on_stack(addr_of_mut!((*work).timer));
        debug_object_free(addr_of_mut!((*work).work).cast(), lupos_wq_work_debug_descr());
    }
}

/// Exact native debug-disabled branch (C 715).
///
/// # Safety
/// Matches the native disabled inline; no object access occurs.
#[cfg(not(CONFIG_DEBUG_OBJECTS_WORK))]
pub(super) unsafe fn debug_work_activate(_work: *mut work_struct) {}

/// Exact native debug-disabled branch (C 716).
///
/// # Safety
/// Matches the native disabled inline; no object access occurs.
#[cfg(not(CONFIG_DEBUG_OBJECTS_WORK))]
pub(super) unsafe fn debug_work_deactivate(_work: *mut work_struct) {}

/// Looks up an immortal CPU-intensive record in the RCU hash (C 1375–1385).
///
/// # Safety
/// Caller supplies the native IRQ/preemption context protecting the RCU walk.
#[cfg(CONFIG_WQ_CPU_INTENSIVE_REPORT)]
unsafe fn wci_find_ent(func: work_func_t) -> *mut wci_ent {
    // SAFETY: Entries are statically allocated and never removed after publication.
    unsafe {
        let mut node = lupos_wq_wci_hash_first_rcu(func);
        while !node.is_null() {
            let ent = lupos_wq_wci_from_hash_node(node);
            if lupos_wq_work_func_eq((*ent).func, func) {
                return ent;
            }
            node = lupos_wq_hlist_next_rcu(node);
        }
        null_mut()
    }
}

/// Counts and exponentially backs off CPU-intensive reports (C 1387–1438).
///
/// # Safety
/// Caller satisfies the native scheduler/RCU context; the native raw lock cannot sleep.
#[cfg(CONFIG_WQ_CPU_INTENSIVE_REPORT)]
pub(super) unsafe fn wq_cpu_intensive_report(func: work_func_t) {
    // SAFETY: The writer lock serializes allocation and publication; published
    // entries are immortal and their count uses native atomic64 operations.
    unsafe {
        loop {
            let ent = wci_find_ent(func);
            if !ent.is_null() {
                let cnt = lupos_wq_atomic64_inc_return_relaxed(addr_of_mut!((*ent).cnt)) as u64;
                if lupos_wq_cpu_intensive_warning_thresh() != 0
                    && cnt >= lupos_wq_cpu_intensive_warning_thresh() as u64
                    && {
                        // Native is_power_of_2 accepts unsigned long. Retain
                        // 32-bit truncation and the original tunable load sites.
                        let backoff = cnt.wrapping_add(1)
                            .wrapping_sub(lupos_wq_cpu_intensive_warning_thresh() as u64) as c_ulong;
                        backoff != 0 && backoff & backoff.wrapping_sub(1) == 0
                    }
                {
                    lupos_wq_print_cpu_intensive_c1405(
                        (*ent).func,
                        lupos_wq_cpu_intensive_thresh_us(),
                        lupos_wq_atomic64_read(addr_of!((*ent).cnt)) as u64,
                    );
                }
                return;
            }
            if lupos_wq_wci_nr_ents_read() >= WCI_MAX_ENTS as c_int {
                return;
            }
            lupos_wq_raw_spin_lock(lupos_wq_wci_lock());
            let nr = lupos_wq_wci_nr_ents_read();
            if nr >= WCI_MAX_ENTS as c_int {
                lupos_wq_raw_spin_unlock(lupos_wq_wci_lock());
                return;
            }
            if !wci_find_ent(func).is_null() {
                lupos_wq_raw_spin_unlock(lupos_wq_wci_lock());
                continue;
            }
            let ent = lupos_wq_wci_ents().add(nr as usize);
            lupos_wq_wci_nr_ents_write(nr.wrapping_add(1));
            (*ent).func = func;
            lupos_wq_atomic64_set(addr_of_mut!((*ent).cnt), 0);
            lupos_wq_wci_hash_add_rcu(addr_of_mut!((*ent).hash_node), func);
            lupos_wq_raw_spin_unlock(lupos_wq_wci_lock());
        }
    }
}

/// Exact native reporting-disabled branch (C 1441).
///
/// # Safety
/// Matches the native disabled implementation; callback is not invoked.
#[cfg(not(CONFIG_WQ_CPU_INTENSIVE_REPORT))]
pub(super) unsafe fn wq_cpu_intensive_report(_func: work_func_t) {}

/// Returns current's executing work or NULL (C 6273–6278).
///
/// # Safety
/// Called in the native current-task context; the pointer grants no new lifetime.
pub(super) unsafe extern "C" fn current_work() -> *mut work_struct {
    // SAFETY: Self's execution fields remain valid until its callback returns.
    unsafe {
        let worker = lupos_wq_current_wq_worker();
        if worker.is_null() { null_mut() } else { (*worker).current_work }
    }
}

/// Reports whether current is a workqueue rescuer (C 6289–6294).
///
/// # Safety
/// Called in the native current-task context.
pub(super) unsafe extern "C" fn current_is_workqueue_rescuer() -> bool {
    // SAFETY: The current worker's rescue association is immutable while executing.
    unsafe {
        let worker = lupos_wq_current_wq_worker();
        !worker.is_null() && !(*worker).rescue_wq.is_null()
    }
}

/// Matches the non-legacy reclaim condition used by flush checks (C 6310–6317).
///
/// # Safety
/// A current worker supplies a live current_pwq, as required by the native query.
pub(super) unsafe extern "C" fn current_is_workqueue_mem_reclaim() -> bool {
    // SAFETY: Running work retains its PWQ; this intentionally adds no null guard
    // that would change the native current-worker contract.
    unsafe {
        let worker = lupos_wq_current_wq_worker();
        !worker.is_null()
            && lupos_wq_wq_flags_read((*(*worker).current_pwq).wq) & (WQ_MEM_RECLAIM | __WQ_LEGACY)
                == WQ_MEM_RECLAIM
    }
}

/// Returns an advisory inactive-list snapshot (C 6339–6355).
///
/// # Safety
/// `wq` is live and cpu is a valid possible CPU or WORK_CPU_UNBOUND.
pub(super) unsafe extern "C" fn workqueue_congested(
    mut cpu: c_int,
    wq: *mut workqueue_struct,
) -> bool {
    // SAFETY: Preemption pins the local CPU and supplies sched-RCU protection;
    // the native list primitive retains the intentionally advisory observation.
    unsafe {
        lupos_wq_preempt_disable();
        if cpu == WORK_CPU_UNBOUND as c_int {
            cpu = lupos_wq_smp_processor_id() as c_int;
        }
        let pwq = lupos_wq_cpu_pwq_dereference_sched(wq, cpu);
        let ret = !lupos_wq_list_empty(addr_of!((*pwq).inactive_works));
        lupos_wq_preempt_enable();
        ret
    }
}

/// Returns advisory PENDING/RUNNING bits without taking ownership (C 6369–6389).
///
/// # Safety
/// `work` is live for the whole call; the result is not a synchronization guarantee.
pub(super) unsafe extern "C" fn work_busy(work: *mut work_struct) -> c_uint {
    // SAFETY: RCU retains the pool while its lock protects busy-worker traversal.
    unsafe {
        let mut ret = 0;
        if lupos_wq_work_pending(work) {
            ret |= WORK_BUSY_PENDING as c_uint;
        }
        lupos_wq_rcu_read_lock();
        let pool = get_work_pool(work);
        if !pool.is_null() {
            let flags = lupos_wq_pool_lock_irqsave(pool);
            if !find_worker_executing_work(pool, work).is_null() {
                ret |= WORK_BUSY_RUNNING as c_uint;
            }
            lupos_wq_pool_unlock_irqrestore(pool, flags);
        }
        lupos_wq_rcu_read_unlock();
        ret
    }
}

/// Formats current's work description through the compiler's va_list (C 6402–6412).
///
/// # Safety
/// The format and variadic arguments obey native vsnprintf's contract. The
/// configured workqueue object must admit the reviewed compiler C-variadic ABI.
pub(super) unsafe extern "C" fn set_worker_desc(fmt: *const c_char, mut args: ...) {
    // SAFETY: Only self writes its description here; no Rust va_list projection
    // or function-pointer ABI replacement is used. Native admission is held.
    unsafe {
        let worker = lupos_wq_current_wq_worker();
        if !worker.is_null() {
            lupos_wq_vsnprintf(
                addr_of_mut!((*worker).desc).cast(),
                WORKER_DESC_LEN as usize,
                fmt,
                args.as_va_list(),
            );
        }
    }
}

/// Best-effort nofault worker description from an unsynchronized task (C 6428–6462).
///
/// # Safety
/// `task` itself is live. Its worker/PWQ/WQ pointers may be stale: never dereference
/// them in Rust, form inbounds field pointers, or create Rust function pointers.
pub(super) unsafe extern "C" fn print_worker_info(log_lvl: *const c_char, task: *mut task_struct) {
    // SAFETY: Potentially stale field addresses use wrapping byte arithmetic;
    // every read traversing the untrusted chain goes through native nofault copy.
    unsafe {
        let mut name = [0 as c_char; WQ_NAME_LEN as usize];
        let mut desc = [0 as c_char; WORKER_DESC_LEN as usize];
        let mut pwq: *mut pool_workqueue = null_mut();
        let mut wq: *mut workqueue_struct = null_mut();
        if lupos_wq_task_flags_read(task) & PF_WQ_WORKER == 0 {
            return;
        }
        let worker = kthread_probe_data(task).cast::<worker>();
        let func_bits = lupos_wq_nofault_work_func_bits(
            worker.cast::<u8>().wrapping_add(offset_of!(worker, current_func)).cast(),
        );
        copy_from_kernel_nofault(
            addr_of_mut!(pwq).cast(),
            worker.cast::<u8>().wrapping_add(offset_of!(worker, current_pwq)).cast(),
            size_of::<*mut pool_workqueue>(),
        );
        copy_from_kernel_nofault(
            addr_of_mut!(wq).cast(),
            pwq.cast::<u8>().wrapping_add(offset_of!(pool_workqueue, wq)).cast(),
            size_of::<*mut workqueue_struct>(),
        );
        copy_from_kernel_nofault(
            name.as_mut_ptr().cast(),
            wq.cast::<u8>().wrapping_add(offset_of!(workqueue_struct, name)).cast(),
            name.len() - 1,
        );
        copy_from_kernel_nofault(
            desc.as_mut_ptr().cast(),
            worker.cast::<u8>().wrapping_add(offset_of!(worker, desc)).cast(),
            desc.len() - 1,
        );
        if func_bits != 0 || name[0] != 0 || desc[0] != 0 {
            lupos_wq_print_worker_info_c6457(log_lvl, name.as_ptr(), func_bits);
            if strcmp(name.as_ptr(), desc.as_ptr()) != 0 {
                lupos_wq_pr_cont(c" (%s)".as_ptr().cast(), desc.as_ptr());
            }
            lupos_wq_pr_cont(c"\n".as_ptr().cast());
        }
    }
}

/// Appends native pool attributes to the current print line (C 6464–6475).
///
/// # Safety
/// `pool` and immutable attrs are retained, normally under pool lock/RCU.
unsafe fn pr_cont_pool_info(pool: *mut worker_pool) {
    // SAFETY: Leaves retain native C variadic formatting and cpumask representation.
    unsafe {
        lupos_wq_print_pool_cpumask_c6466(lupos_wq_attrs_cpumask((*pool).attrs));
        if (*pool).node != NUMA_NO_NODE {
            lupos_wq_pr_cont(c" node=%d".as_ptr().cast(), (*pool).node);
        }
        lupos_wq_pr_cont(c" flags=0x%x".as_ptr().cast(), lupos_wq_pool_flags_read(pool));
        if lupos_wq_pool_flags_read(pool) & POOL_BH != 0 {
            let suffix = if (*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL {
                c"-hi".as_ptr()
            } else {
                c"".as_ptr()
            };
            lupos_wq_pr_cont(c" bh%s".as_ptr().cast(), suffix.cast::<c_char>());
        } else {
            lupos_wq_pr_cont(c" nice=%d".as_ptr().cast(), (*(*pool).attrs).nice);
        }
    }
}

/// Appends a BH or task/rescuer worker identity (C 6477–6487).
///
/// # Safety
/// The caller holds pool lock and retains worker/task/attrs lifetimes.
unsafe fn pr_cont_worker_id(worker: *mut worker) {
    // SAFETY: The lock excludes worker death and association changes here.
    unsafe {
        let pool = (*worker).pool;
        if (*pool).flags & POOL_BH != 0 {
            let suffix = if (*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL {
                c"-hi".as_ptr()
            } else {
                c"".as_ptr()
            };
            lupos_wq_pr_cont(c"bh%s".as_ptr().cast(), suffix.cast::<c_char>());
        } else {
            let suffix = if !(*worker).rescue_wq.is_null() {
                c"(RESCUER)".as_ptr()
            } else {
                c"".as_ptr()
            };
            lupos_wq_pr_cont(
                c"%d%s".as_ptr().cast(),
                lupos_wq_task_pid_nr((*worker).task),
                suffix.cast::<c_char>(),
            );
        }
    }
}

// Private Rust-only aggregation state. It never crosses the native ABI and does
// not project any native allocation layout. None below is an explicit flush
// state; Some(None) still denotes C's valid NULL callback value if encountered.
struct PrintWorkState {
    comma: bool,
    func: work_func_t,
    ctr: c_long,
}

/// Flushes/repeats the current callback group without a forged -1 fn (C 6495–6514).
///
/// # Safety
/// Caller owns state and a native deferred-print region where required.
unsafe fn pr_cont_work_flush(comma: bool, next: Option<work_func_t>, state: &mut PrintWorkState) {
    // SAFETY: Only genuine native callbacks enter the state; flush is separate.
    unsafe {
        if state.ctr != 0 {
            if let Some(func) = next {
                if lupos_wq_work_func_eq(func, state.func) {
                    state.ctr = state.ctr.wrapping_add(1);
                    return;
                }
            }
            let sep = if state.comma { c",".as_ptr() } else { c"".as_ptr() };
            if state.ctr == 1 {
                lupos_wq_pr_cont(
                    c"%s %ps".as_ptr().cast(),
                    sep.cast::<c_char>(),
                    lupos_wq_work_func_hint(state.func),
                );
            } else {
                lupos_wq_pr_cont(
                    c"%s %ld*%ps".as_ptr().cast(),
                    sep.cast::<c_char>(),
                    state.ctr,
                    lupos_wq_work_func_hint(state.func),
                );
            }
            state.ctr = 0;
        }
        if let Some(func) = next {
            state.comma = comma;
            state.func = func;
            state.ctr = 1;
        }
    }
}

/// Groups ordinary callbacks but prints each barrier with its waiter (C 6516–6531).
///
/// # Safety
/// Pool lock retains the work and any enclosing barrier through this call.
unsafe fn pr_cont_work(comma: bool, work: *mut work_struct, state: &mut PrintWorkState) {
    // SAFETY: Callback identity is checked before interpreting the barrier layout.
    unsafe {
        if lupos_wq_work_func_eq((*work).func, Some(wq_barrier_func)) {
            let barr = work.cast::<u8>().sub(offset_of!(wq_barrier, work)).cast::<wq_barrier>();
            pr_cont_work_flush(comma, None, state);
            let sep = if comma { c",".as_ptr() } else { c"".as_ptr() };
            lupos_wq_pr_cont(c"%s BAR(%d)".as_ptr().cast(), sep.cast::<c_char>(), lupos_wq_task_pid_nr((*barr).task));
        } else {
            if !comma {
                pr_cont_work_flush(comma, None, state);
            }
            pr_cont_work_flush(comma, Some((*work).func), state);
        }
    }
}

/// Dumps PWQ execution, pending and inactive chains (C 6533–6608).
///
/// # Safety
/// Caller holds the PWQ pool's IRQ-safe raw lock. Normal dump callers defer
/// printk; the native destroy-warning call retains its original print context.
pub(super) unsafe fn show_pwq(pwq: *mut pool_workqueue) {
    // SAFETY: Locking pins all inspected intrusive nodes and callback fields.
    unsafe {
        let mut state = PrintWorkState { comma: false, func: None, ctr: 0 };
        let pool = (*pwq).pool;
        lupos_wq_pr_info(c"  pwq %d:".as_ptr().cast(), (*pool).id);
        pr_cont_pool_info(pool);
        let mayday = if !lupos_wq_list_empty(addr_of!((*pwq).mayday_node)) {
            c" MAYDAY".as_ptr()
        } else { c"".as_ptr() };
        lupos_wq_pr_cont(c" active=%d refcnt=%d%s\n".as_ptr().cast(), (*pwq).nr_active, (*pwq).refcnt, mayday.cast::<c_char>());

        let mut has_in_flight = false;
        for bkt in 0..(1usize << BUSY_WORKER_HASH_ORDER) {
            let mut node = (*lupos_wq_busy_hash_bucket(pool, bkt)).first;
            while !node.is_null() {
                let worker = lupos_wq_worker_from_hentry(node);
                if (*worker).current_pwq == pwq {
                    has_in_flight = true;
                    break;
                }
                node = (*node).next;
            }
            if has_in_flight { break; }
        }
        if has_in_flight {
            let mut comma = false;
            lupos_wq_pr_info(c"    in-flight:".as_ptr().cast());
            for bkt in 0..(1usize << BUSY_WORKER_HASH_ORDER) {
                let mut node = (*lupos_wq_busy_hash_bucket(pool, bkt)).first;
                while !node.is_null() {
                    let worker = lupos_wq_worker_from_hentry(node);
                    node = (*node).next;
                    if (*worker).current_pwq != pwq { continue; }
                    let sep = if comma { c",".as_ptr() } else { c"".as_ptr() };
                    lupos_wq_pr_cont(c" %s".as_ptr().cast(), sep.cast::<c_char>());
                    pr_cont_worker_id(worker);
                    lupos_wq_pr_cont(c":%ps".as_ptr().cast(), lupos_wq_work_func_hint((*worker).current_func));
                    let secs = lupos_wq_jiffies_to_msecs(lupos_wq_read_jiffies().wrapping_sub((*worker).current_start)) / 1000;
                    lupos_wq_pr_cont(c" for %us".as_ptr().cast(), secs);
                    let head = addr_of_mut!((*worker).scheduled);
                    let mut entry = (*head).next;
                    while entry != head {
                        pr_cont_work(false, lupos_wq_work_from_entry(entry), &mut state);
                        entry = (*entry).next;
                    }
                    pr_cont_work_flush(comma, None, &mut state);
                    comma = true;
                }
            }
            lupos_wq_pr_cont(c"\n".as_ptr().cast());
        }

        let head = addr_of_mut!((*pool).worklist);
        let mut entry = (*head).next;
        let mut has_pending = false;
        while entry != head {
            if get_work_pwq(lupos_wq_work_from_entry(entry)) == pwq {
                has_pending = true;
                break;
            }
            entry = (*entry).next;
        }
        if has_pending {
            let mut comma = false;
            lupos_wq_pr_info(c"    pending:".as_ptr().cast());
            let mut entry = (*head).next;
            while entry != head {
                let work = lupos_wq_work_from_entry(entry);
                entry = (*entry).next;
                if get_work_pwq(work) != pwq { continue; }
                pr_cont_work(comma, work, &mut state);
                comma = lupos_wq_work_data_bits_plain_read(work) & WORK_STRUCT_LINKED as c_ulong == 0;
            }
            pr_cont_work_flush(comma, None, &mut state);
            lupos_wq_pr_cont(c"\n".as_ptr().cast());
        }
        let head = addr_of_mut!((*pwq).inactive_works);
        if !lupos_wq_list_empty(head) {
            let mut comma = false;
            lupos_wq_pr_info(c"    inactive:".as_ptr().cast());
            let mut entry = (*head).next;
            while entry != head {
                let work = lupos_wq_work_from_entry(entry);
                pr_cont_work(comma, work, &mut state);
                comma = lupos_wq_work_data_bits_plain_read(work) & WORK_STRUCT_LINKED as c_ulong == 0;
                entry = (*entry).next;
            }
            pr_cont_work_flush(comma, None, &mut state);
            lupos_wq_pr_cont(c"\n".as_ptr().cast());
        }
    }
}

/// Dumps a nonidle workqueue with per-pool deferred printing (C 6614–6652).
///
/// # Safety
/// Caller retains `wq` and its PWQ list using the native RCU or mutex contract.
pub(super) unsafe extern "C" fn show_one_workqueue(wq: *mut workqueue_struct) {
    // SAFETY: PWQ list traversal is protected; per-pool locks protect live work.
    unsafe {
        let mut pwq = lupos_wq_pwq_first(wq);
        let mut idle = true;
        while !pwq.is_null() {
            if !pwq_is_empty(pwq) { idle = false; break; }
            pwq = lupos_wq_pwq_next(wq, pwq);
        }
        if idle { return; }
        lupos_wq_pr_info(c"workqueue %s: flags=0x%x\n".as_ptr().cast(), addr_of!((*wq).name).cast::<c_char>(), lupos_wq_wq_flags_read(wq));
        let mut pwq = lupos_wq_pwq_first(wq);
        while !pwq.is_null() {
            let pool = (*pwq).pool;
            let flags = lupos_wq_pool_lock_irqsave(pool);
            if !pwq_is_empty(pwq) {
                lupos_wq_printk_deferred_enter();
                show_pwq(pwq);
                lupos_wq_printk_deferred_exit();
            }
            lupos_wq_pool_unlock_irqrestore(pool, flags);
            lupos_wq_touch_nmi_watchdog();
            pwq = lupos_wq_pwq_next(wq, pwq);
        }
    }
}

/// Dumps a busy pool, preserving idle-worker identity lifetime (C 6658–6701).
///
/// # Safety
/// Caller retains `pool` using the native RCU or pool-mutex contract.
unsafe fn show_one_worker_pool(pool: *mut worker_pool) {
    // SAFETY: The lock excludes manager/idle-worker destruction during printing.
    unsafe {
        let flags = lupos_wq_pool_lock_irqsave(pool);
        if (*pool).nr_workers != (*pool).nr_idle {
            let hung = if !lupos_wq_list_empty(addr_of!((*pool).worklist)) {
                (lupos_wq_jiffies_to_msecs(lupos_wq_read_jiffies().wrapping_sub((*pool).last_progress_ts)) / 1000) as c_ulong
            } else { 0 };
            lupos_wq_printk_deferred_enter();
            lupos_wq_pr_info(c"pool %d:".as_ptr().cast(), (*pool).id);
            pr_cont_pool_info(pool);
            lupos_wq_pr_cont(c" hung=%lus workers=%d".as_ptr().cast(), hung, (*pool).nr_workers);
            if !(*pool).manager.is_null() {
                lupos_wq_pr_cont(c" manager: %d".as_ptr().cast(), lupos_wq_task_pid_nr((*(*pool).manager).task));
            }
            let head = addr_of_mut!((*pool).idle_list);
            let mut entry = (*head).next;
            let mut first = true;
            while entry != head {
                let prefix = if first { c"idle: ".as_ptr() } else { c"".as_ptr() };
                lupos_wq_pr_cont(c" %s".as_ptr().cast(), prefix.cast::<c_char>());
                pr_cont_worker_id(lupos_wq_worker_from_entry(entry));
                first = false;
                entry = (*entry).next;
            }
            lupos_wq_pr_cont(c"\n".as_ptr().cast());
            lupos_wq_printk_deferred_exit();
        }
        lupos_wq_pool_unlock_irqrestore(pool, flags);
        lupos_wq_touch_nmi_watchdog();
    }
}

/// Prints all busy queues and pools under RCU (C 6708–6725).
///
/// # Safety
/// Caller can enter native RCU and the diagnostic raw-lock/print paths.
pub(super) unsafe extern "C" fn show_all_workqueues() {
    // SAFETY: RCU protects list/IDR objects; nested helpers acquire pool locks.
    unsafe {
        lupos_wq_rcu_read_lock();
        lupos_wq_pr_info(c"Showing busy workqueues and worker pools:\n".as_ptr().cast());
        let mut wq = lupos_wq_workqueues_first_rcu();
        while !wq.is_null() {
            show_one_workqueue(wq);
            wq = lupos_wq_workqueues_next_rcu(wq);
        }
        let mut pi: c_int = 0;
        loop {
            let pool = idr_get_next(lupos_wq_worker_pool_idr(), addr_of_mut!(pi)).cast::<worker_pool>();
            if pool.is_null() { break; }
            lupos_wq_assert_rcu_or_pool_mutex();
            show_one_worker_pool(pool);
            pi = pi.wrapping_add(1);
        }
        lupos_wq_rcu_read_unlock();
    }
}

/// Prints only busy freezable workqueues under RCU (C 6733–6748).
///
/// # Safety
/// Caller can enter native RCU and the diagnostic raw-lock/print paths.
pub(super) unsafe extern "C" fn show_freezable_workqueues() {
    // SAFETY: RCU retains queues; native flag snapshots preserve advisory reads.
    unsafe {
        lupos_wq_rcu_read_lock();
        lupos_wq_pr_info(c"Showing freezable workqueues that are still busy:\n".as_ptr().cast());
        let mut wq = lupos_wq_workqueues_first_rcu();
        while !wq.is_null() {
            if lupos_wq_wq_flags_read(wq) & WQ_FREEZABLE != 0 {
                show_one_workqueue(wq);
            }
            wq = lupos_wq_workqueues_next_rcu(wq);
        }
        lupos_wq_rcu_read_unlock();
    }
}

/// Formats proc worker comm while pinning association and task identity (C 6751–6785).
///
/// # Safety
/// `buf` is writable for size bytes; task is live; caller may take the attach mutex.
pub(super) unsafe extern "C" fn wq_worker_comm(buf: *mut c_char, size: usize, task: *mut task_struct) {
    // SAFETY: Attach mutex stabilizes PF_WQ_WORKER and worker->pool. Pool lock
    // protects the current-work/description snapshot; no reference escapes it.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
        if (*task).flags & PF_WQ_WORKER != 0 {
            let worker = kthread_data(task).cast::<worker>();
            let pool = (*worker).pool;
            let off = format_worker_id(buf, size, worker, pool);
            if !pool.is_null() {
                lupos_wq_pool_lock_irq(pool);
                if lupos_wq_worker_desc_first_read(worker) != 0 {
                    let fmt = if !(*worker).current_work.is_null() {
                        c"+%s".as_ptr()
                    } else { c"-%s".as_ptr() };
                    // Keep C size_t subtraction and pointer arithmetic explicit.
                    scnprintf(buf.wrapping_add(off as usize), size.wrapping_sub(off as usize), fmt.cast(), addr_of!((*worker).desc).cast::<c_char>());
                }
                lupos_wq_pool_unlock_irq(pool);
            }
        } else {
            lupos_wq_strscpy(buf, addr_of!((*task).comm).cast(), size);
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
    }
}

/// Resolves the native sysfs wrapper without projecting its layout (C 7335–7340).
///
/// # Safety
/// `dev` is the embedded device in a live, common-declared native wq_device.
#[cfg(CONFIG_SYSFS)]
unsafe fn dev_to_wq(dev: *mut device) -> *mut workqueue_struct {
    // SAFETY: Exact native offset_of is generated from the common-owned declaration.
    unsafe {
        let wrapper = dev.cast::<u8>().sub(offset_of!(wq_device, dev)).cast::<wq_device>();
        (*wrapper).wq
    }
}

/// Shows whether a workqueue is per-CPU (C 7342–7348).
///
/// # Safety
/// Native sysfs keeps dev/wq and its PAGE_SIZE output buffer live.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn per_cpu_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: The snapshot is advisory and formatted with the native promoted int.
    unsafe {
        scnprintf(buf, PAGE_SIZE as usize, c"%d\n".as_ptr().cast(),
            (lupos_wq_wq_flags_read(dev_to_wq(dev)) & WQ_UNBOUND == 0) as c_int) as ssize_t
    }
}

/// Shows the saved max_active (C 7351–7357).
///
/// # Safety
/// Native sysfs keeps dev/wq and its PAGE_SIZE output buffer live.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn max_active_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: The original racy read remains a native snapshot boundary.
    unsafe {
        scnprintf(buf, PAGE_SIZE as usize, c"%d\n".as_ptr().cast(),
            lupos_wq_saved_max_active_read(dev_to_wq(dev))) as ssize_t
    }
}

/// Parses a positive max_active and applies the resource-owner policy (C 7359–7371).
///
/// # Safety
/// Native sysfs supplies a NUL-terminated input buffer and retains dev/wq.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn max_active_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: sscanf initializes val on successful conversion before it is used.
    unsafe {
        let wq = dev_to_wq(dev);
        let mut val: c_int = 0;
        if sscanf(buf, c"%d".as_ptr().cast(), addr_of_mut!(val)) != 1 || val <= 0 {
            return -(EINVAL as ssize_t);
        }
        workqueue_set_max_active(wq, val);
        count as ssize_t
    }
}

/// Makes ordered/BH base attributes read-only (C 7380–7392).
///
/// # Safety
/// kobj belongs to a live workqueue device and a points to its native attribute.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_sysfs_is_visible(kobj: *mut kobject, a: *mut attribute, _n: c_int) -> umode_t {
    // SAFETY: The device container is the native kobj_to_dev macro identity.
    unsafe {
        let wq = dev_to_wq(lupos_wq_kobj_to_dev(kobj));
        if lupos_wq_wq_flags_read(wq) & (WQ_BH | __WQ_ORDERED) != 0 {
            0o444 as umode_t
        } else { (*a).mode }
    }
}

/// Shows native worker niceness while retaining attrs (C 7400–7411).
///
/// # Safety
/// Native sysfs retains dev/wq and provides a PAGE_SIZE buffer; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_nice_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: wq mutex prevents attribute replacement while reading nice.
    unsafe {
        let wq = dev_to_wq(dev);
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let written = scnprintf(buf, PAGE_SIZE as usize, c"%d\n".as_ptr().cast(), (*(*wq).attrs).nice);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        written as ssize_t
    }
}

/// Allocates and copies sysfs-editable attrs under the pool mutex (C 7414–7426).
///
/// # Safety
/// Caller holds wq_pool_mutex and keeps wq live; may allocate with GFP_KERNEL.
#[cfg(CONFIG_SYSFS)]
unsafe fn wq_sysfs_prep_attrs(wq: *mut workqueue_struct) -> *mut workqueue_attrs {
    // SAFETY: The site-specific profiling leaf calls only the Rust resource
    // allocator through alloc_hooks, preserving allocation context/attribution.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        let attrs = lupos_wq_alloc_workqueue_attrs_c7420();
        if attrs.is_null() { return null_mut(); }
        copy_workqueue_attrs(attrs, (*wq).attrs);
        attrs
    }
}

/// Applies only in-range nice values, preserving allocation/error cleanup (C 7428–7451).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a terminated input; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_nice_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: The private attrs copy is initialized, modified under pool mutex,
    // applied transactionally by D, and freed after unlock, including NULL failure.
    unsafe {
        let wq = dev_to_wq(dev);
        let mut ret = -(ENOMEM as c_int);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let attrs = wq_sysfs_prep_attrs(wq);
        if !attrs.is_null() {
            if sscanf(buf, c"%d".as_ptr().cast(), addr_of_mut!((*attrs).nice)) == 1
                && (*attrs).nice >= MIN_NICE && (*attrs).nice <= MAX_NICE
            {
                ret = apply_workqueue_attrs_locked(wq, attrs);
            } else { ret = -(EINVAL as c_int); }
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        free_workqueue_attrs(attrs);
        if ret != 0 { ret as ssize_t } else { count as ssize_t }
    }
}

/// Shows the queue's requested worker mask under its mutex (C 7453–7464).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a PAGE_SIZE output; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_cpumask_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: Mutex retains the attrs and native mask through formatting.
    unsafe {
        let wq = dev_to_wq(dev);
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let written = lupos_wq_scnprintf_cpumask(buf, PAGE_SIZE as usize, lupos_wq_attrs_cpumask((*wq).attrs));
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        written as ssize_t
    }
}

/// Parses a per-queue CPU mask and transactionally applies attrs (C 7466–7488).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a terminated input; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_cpumask_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: Only the owned copy is parsed; it is freed on every path after unlock.
    unsafe {
        let wq = dev_to_wq(dev);
        let mut ret = -(ENOMEM as c_int);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let attrs = wq_sysfs_prep_attrs(wq);
        if !attrs.is_null() {
            ret = lupos_wq_cpumask_parse(buf, lupos_wq_attrs_cpumask(attrs));
            if ret == 0 { ret = apply_workqueue_attrs_locked(wq, attrs); }
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        free_workqueue_attrs(attrs);
        if ret != 0 { ret as ssize_t } else { count as ssize_t }
    }
}

/// Shows configured/default affinity scope names (C 7490–7507).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a PAGE_SIZE output; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_affn_scope_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: wq mutex retains attrs; the names table has native static lifetime.
    unsafe {
        let wq = dev_to_wq(dev);
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let scope = (*(*wq).attrs).affn_scope;
        let written = if scope == WQ_AFFN_DFL {
            scnprintf(buf, PAGE_SIZE as usize, c"%s (%s)\n".as_ptr().cast(),
                lupos_wq_affn_name(WQ_AFFN_DFL as c_int), lupos_wq_affn_name(lupos_wq_affn_dfl() as c_int))
        } else {
            scnprintf(buf, PAGE_SIZE as usize, c"%s\n".as_ptr().cast(), lupos_wq_affn_name(scope as c_int))
        };
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        written as ssize_t
    }
}

/// Parses an affinity scope before allocating its replacement attrs (C 7509–7530).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a terminated input; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_affn_scope_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: A valid parsed enum enters only a private initialized attrs copy.
    unsafe {
        let wq = dev_to_wq(dev);
        let affn = parse_affn_scope(buf);
        if affn < 0 { return affn as ssize_t; }
        let mut ret = -(ENOMEM as c_int);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let attrs = wq_sysfs_prep_attrs(wq);
        if !attrs.is_null() {
            (*attrs).affn_scope = affn as _;
            ret = apply_workqueue_attrs_locked(wq, attrs);
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        free_workqueue_attrs(attrs);
        if ret != 0 { ret as ssize_t } else { count as ssize_t }
    }
}

/// Shows native advisory affinity strictness (C 7532–7539).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies PAGE_SIZE bytes; native unsynchronized
/// attrs dereference remains an explicit boundary/race-lifetime admission hold.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_affinity_strict_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: No Rust dereference is made along the native racy attrs chain.
    unsafe {
        scnprintf(buf, PAGE_SIZE as usize, c"%d\n".as_ptr().cast(),
            lupos_wq_affn_strict_read(dev_to_wq(dev)) as c_int) as ssize_t
    }
}

/// Uses native integer-to-bool conversion, without narrowing to 0/1 input (C 7541–7561).
///
/// # Safety
/// Native sysfs retains dev/wq and supplies a terminated input; caller may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_affinity_strict_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: Successful sscanf initializes v; attrs are private until D commits them.
    unsafe {
        let wq = dev_to_wq(dev);
        let mut v: c_int = 0;
        if sscanf(buf, c"%d".as_ptr().cast(), addr_of_mut!(v)) != 1 {
            return -(EINVAL as ssize_t);
        }
        let mut ret = -(ENOMEM as c_int);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let attrs = wq_sysfs_prep_attrs(wq);
        if !attrs.is_null() {
            (*attrs).affn_strict = v != 0;
            ret = apply_workqueue_attrs_locked(wq, attrs);
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        free_workqueue_attrs(attrs);
        if ret != 0 { ret as ssize_t } else { count as ssize_t }
    }
}

/// Applies an explicit possible-CPU mask, permitting isolated CPUs (C 7588–7608).
///
/// # Safety
/// `mask` is an owned writable native cpumask; caller may take the pool mutex.
#[cfg(CONFIG_SYSFS)]
unsafe fn workqueue_set_unbound_cpumask(mask: *mut cpumask) -> c_int {
    // SAFETY: Native masks preserve their configured representation; global
    // requested/effective masks are updated only after the resource commit succeeds.
    unsafe {
        lupos_wq_cpumask_and(mask, mask, lupos_wq_cpu_possible_mask());
        let mut ret = -(EINVAL as c_int);
        if !lupos_wq_cpumask_empty(mask) {
            ret = 0;
            lupos_wq_mutex_lock(lupos_wq_pool_mutex());
            if !lupos_wq_cpumask_equal(mask, lupos_wq_unbound_cpumask()) {
                ret = workqueue_apply_unbound_cpumask(mask);
            }
            if ret == 0 {
                lupos_wq_cpumask_copy(lupos_wq_requested_unbound_cpumask().cast_mut(), mask);
            }
            lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        }
        ret
    }
}

/// Formats one global mask while holding the pool mutex (C 7610–7620).
///
/// # Safety
/// Mask is one of the live common-owned global masks; buf has PAGE_SIZE bytes.
#[cfg(CONFIG_SYSFS)]
unsafe fn __wq_cpumask_show(buf: *mut c_char, mask: *const cpumask) -> ssize_t {
    // SAFETY: Mutex serializes all writes to the selected global mask.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let written = lupos_wq_scnprintf_cpumask(buf, PAGE_SIZE as usize, mask);
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        written as ssize_t
    }
}

/// Shows the user-requested unbound mask (C 7622–7626).
///
/// # Safety
/// Native sysfs supplies PAGE_SIZE output bytes and a sleeping context.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn cpumask_requested_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: The selected mask is initialized before sysfs publication.
    unsafe { __wq_cpumask_show(buf, lupos_wq_requested_unbound_cpumask()) }
}

/// Shows the isolated CPU mask (C 7629–7633).
///
/// # Safety
/// Native sysfs supplies PAGE_SIZE output bytes and a sleeping context.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn cpumask_isolated_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: The selected mask is initialized before sysfs publication.
    unsafe { __wq_cpumask_show(buf, lupos_wq_isolated_cpumask()) }
}

/// Shows the effective unbound mask (C 7636–7640).
///
/// # Safety
/// Native sysfs supplies PAGE_SIZE output bytes and a sleeping context.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn cpumask_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    // SAFETY: The selected mask is initialized before sysfs publication.
    unsafe { __wq_cpumask_show(buf, lupos_wq_unbound_cpumask()) }
}

/// Parses, applies, and always frees a temporary global-mask edit (C 7642–7657).
///
/// # Safety
/// Native sysfs supplies a terminated input and a context that may sleep.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn cpumask_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    // SAFETY: The configured cpumask_var_t is native-generated. Only its address
    // crosses leaves that handle offstack/inline storage and allocation profiling.
    unsafe {
        let mut storage = MaybeUninit::<cpumask_var_t>::uninit();
        if !lupos_wq_zalloc_cpumask_var_c7648(storage.as_mut_ptr(), GFP_KERNEL) {
            return -(ENOMEM as ssize_t);
        }
        let mask = lupos_wq_cpumask_var_ptr(storage.as_mut_ptr());
        let mut ret = lupos_wq_cpumask_parse(buf, mask);
        if ret == 0 { ret = workqueue_set_unbound_cpumask(mask); }
        lupos_wq_free_cpumask_var_storage(storage.as_mut_ptr());
        if ret != 0 { ret as ssize_t } else { count as ssize_t }
    }
}

/// Registers the native workqueue bus and global mask groups (C 7668–7671).
///
/// # Safety
/// Invoked exactly once at the native core_initcall phase; table identity and
/// __init placement must be established before admitting this source.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_sysfs_init() -> c_int {
    // SAFETY: Common owns the static bus/groups; registration is a native driver-core call.
    unsafe { subsys_virtual_register(lupos_wq_subsys(), lupos_wq_sysfs_cpumask_groups()) }
}

/// Releases only the enclosing device allocation (C 7674–7679).
///
/// # Safety
/// Native driver core invokes this after the final device reference is dropped.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn wq_device_release(dev: *mut device) {
    // SAFETY: The exact native container was kzalloc_obj-allocated by registration.
    unsafe {
        let wrapper = dev.cast::<u8>().sub(offset_of!(wq_device, dev)).cast::<wq_device>();
        kfree(wrapper.cast::<c_void>());
    }
}

/// Publishes sysfs only after all unbound files have been created (C 7696–7739).
///
/// # Safety
/// wq is live, unregistered, and serialized against destruction/attrs publication.
/// Caller may allocate/sleep; device release uses the exact native callback ABI.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe extern "C" fn workqueue_sysfs_register(wq: *mut workqueue_struct) -> c_int {
    // SAFETY: Device core takes allocation ownership on registration; both error
    // paths drop its reference before clearing the wq pointer and returning errno.
    unsafe {
        let wrapper = lupos_wq_kzalloc_wq_device_c7701();
        (*wq).wq_dev = wrapper;
        if wrapper.is_null() { return -(ENOMEM as c_int); }
        (*wrapper).wq = wq;
        let dev = addr_of_mut!((*wrapper).dev);
        (*dev).bus = lupos_wq_subsys();
        (*dev).release = Some(wq_device_release);
        dev_set_name(dev, c"%s".as_ptr().cast(), addr_of!((*wq).name).cast::<c_char>());
        lupos_wq_dev_set_uevent_suppress(dev, true);
        let ret = device_register(dev);
        if ret != 0 {
            put_device(dev);
            (*wq).wq_dev = null_mut();
            return ret;
        }
        if (*wq).flags & WQ_UNBOUND != 0 {
            let mut attr = lupos_wq_sysfs_unbound_attrs();
            while !(*attr).attr.name.is_null() {
                let ret = device_create_file(dev, attr);
                if ret != 0 {
                    device_unregister(dev);
                    (*wq).wq_dev = null_mut();
                    return ret;
                }
                attr = attr.add(1);
            }
        }
        lupos_wq_dev_set_uevent_suppress(dev, false);
        kobject_uevent(addr_of_mut!((*dev).kobj), KOBJ_ADD);
        0
    }
}

/// Detaches a published sysfs device exactly once (C 7747–7756).
///
/// # Safety
/// Caller serializes wq destruction/registration; native driver core owns release.
#[cfg(CONFIG_SYSFS)]
pub(super) unsafe fn workqueue_sysfs_unregister(wq: *mut workqueue_struct) {
    // SAFETY: Clear the queue's association before device_unregister may free it.
    unsafe {
        let wrapper = (*wq).wq_dev;
        if wrapper.is_null() { return; }
        (*wq).wq_dev = null_mut();
        device_unregister(addr_of_mut!((*wrapper).dev));
    }
}

/// Exact native SYSFS-disabled unregister branch (C 7758).
///
/// # Safety
/// Matches the native disabled inline; no queue access occurs.
#[cfg(not(CONFIG_SYSFS))]
pub(super) unsafe fn workqueue_sysfs_unregister(_wq: *mut workqueue_struct) {}

/// Exact public-header SYSFS-disabled registration branch.
///
/// # Safety
/// Matches workqueue.h:894–895; no queue access or registration occurs.
#[cfg(not(CONFIG_SYSFS))]
pub(super) unsafe extern "C" fn workqueue_sysfs_register(_wq: *mut workqueue_struct) -> c_int { 0 }

/// Reports a pool that has no runnable worker, including its last wake target (C 7798–7821).
///
/// # Safety
/// Pool raw lock is held. Its last_woken_worker and task remain live because B's
/// set_worker_dying clears the pointer under that same lock before WORKER_DIE.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn show_pool_no_running_worker(pool: *mut worker_pool) {
    // SAFETY: All last-woken dereferences occur under the pool lock; no borrowed
    // worker/task survives unlock. Nested deferred printk is deliberate/native.
    unsafe {
        lupos_wq_assert_pool_lock_held_c7800(pool);
        lupos_wq_printk_deferred_enter();
        let state = if idle_cpu((*pool).cpu) != 0 { c"idle".as_ptr() } else { c"busy".as_ptr() };
        lupos_wq_pr_info(
            c"pool %d: no worker in running state, cpu=%d is %s (nr_workers=%d nr_idle=%d)\n".as_ptr().cast(),
            (*pool).id, (*pool).cpu, state.cast::<c_char>(), (*pool).nr_workers, (*pool).nr_idle,
        );
        lupos_wq_pr_info(c"The pool might have trouble waking an idle worker.\n".as_ptr().cast());
        if !(*pool).last_woken_worker.is_null() {
            lupos_wq_pr_info(c"Backtrace of last woken worker:\n".as_ptr().cast());
            sched_show_task((*(*pool).last_woken_worker).task);
        } else {
            lupos_wq_pr_info(c"Last woken worker empty\n".as_ptr().cast());
        }
        lupos_wq_printk_deferred_exit();
    }
}

/// Shows running workers or the missing-wakeup case before an optional CPU NMI (C 7828–7877).
///
/// # Safety
/// RCU retains pool throughout; the caller has identified a stalled per-CPU pool.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn show_cpu_pool_busy_workers(pool: *mut worker_pool) {
    // SAFETY: Raw locking retains hash entries and tasks. Only a scalar CPU ID
    // escapes the lock; offline CPUs are excluded before the native NMI request.
    unsafe {
        let flags = lupos_wq_pool_lock_irqsave(pool);
        let cpu = (*pool).cpu;
        let mut found_running = false;
        for bkt in 0..(1usize << BUSY_WORKER_HASH_ORDER) {
            let mut node = (*lupos_wq_busy_hash_bucket(pool, bkt)).first;
            while !node.is_null() {
                let worker = lupos_wq_worker_from_hentry(node);
                node = (*node).next;
                if !lupos_wq_task_is_running((*worker).task) { continue; }
                found_running = true;
                lupos_wq_printk_deferred_enter();
                lupos_wq_pr_info(c"pool %d:\n".as_ptr().cast(), (*pool).id);
                sched_show_task((*worker).task);
                lupos_wq_printk_deferred_exit();
            }
        }
        if !found_running { show_pool_no_running_worker(pool); }
        lupos_wq_pool_unlock_irqrestore(pool, flags);
        if !found_running && lupos_wq_cpu_online(cpu) {
            lupos_wq_trigger_single_cpu_backtrace(cpu);
        }
    }
}

/// Inspects only pools marked stalled by this watchdog iteration (C 7879–7895).
///
/// # Safety
/// Runs in the serialized watchdog context; may enter RCU and pool raw locks.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn show_cpu_pools_busy_workers() {
    // SAFETY: RCU retains each IDR member; cpu_stall is owned by this timer scan.
    unsafe {
        lupos_wq_pr_info(c"Showing backtraces of busy workers in stalled worker pools:\n".as_ptr().cast());
        lupos_wq_rcu_read_lock();
        let mut pi: c_int = 0;
        loop {
            let pool = idr_get_next(lupos_wq_worker_pool_idr(), addr_of_mut!(pi)).cast::<worker_pool>();
            if pool.is_null() { break; }
            lupos_wq_assert_rcu_or_pool_mutex();
            if (*pool).cpu_stall { show_cpu_pool_busy_workers(pool); }
            pi = pi.wrapping_add(1);
        }
        lupos_wq_rcu_read_unlock();
    }
}

/// Enforces both cumulative stall-count and elapsed-stall panic thresholds (C 7902–7916).
///
/// # Safety
/// Called only by the serialized native watchdog timer; the count is common-owned.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn panic_on_wq_watchdog(stall_time_sec: c_uint) {
    // SAFETY: Only this serialized callback writes wq_stall; tunables retain their
    // native separate advisory read sites and panic leaves never return.
    unsafe {
        if lupos_wq_panic_on_stall() != 0 {
            let counter = lupos_wq_stall_count();
            *counter = (*counter).wrapping_add(1);
            if *counter >= lupos_wq_panic_on_stall() {
                lupos_wq_panic_watchdog_count_c7909(*counter, lupos_wq_panic_on_stall());
            }
        }
        if lupos_wq_panic_on_stall_time() != 0 && stall_time_sec >= lupos_wq_panic_on_stall_time() {
            lupos_wq_panic_watchdog_time_c7914(stall_time_sec, lupos_wq_panic_on_stall_time());
        }
    }
}

/// Refreshes global and each possible CPU's progress grace timestamp (C 7918–7925).
///
/// # Safety
/// Called from the serialized timer or after timer_delete_sync. Concurrent touches
/// remain native plain writes, not Rust data races or new synchronization.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn wq_watchdog_reset_touched() {
    // SAFETY: Each native per-CPU slot exists for possible CPUs; every assignment
    // rereads jiffies at the same native source point instead of one shared snapshot.
    unsafe {
        lupos_wq_write_ulong(lupos_wq_watchdog_touched(), lupos_wq_read_jiffies());
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            lupos_wq_write_ulong(lupos_wq_watchdog_touched_cpu(cpu as c_int), lupos_wq_read_jiffies());
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
    }
}

/// Scans for stalls and rechecks timestamps under lock before reporting (C 7927–8013).
///
/// # Safety
/// This is the native TIMER_DEFERRABLE callback context. Its original softirq/RCU
/// protection must be proven in configured admission; no extra RCU critical
/// section is invented. Concurrent timestamp/flag observations use native leaves.
#[cfg(CONFIG_WQ_WATCHDOG)]
pub(super) unsafe extern "C" fn wq_watchdog_timer_fn(_unused: *mut timer_list) {
    // SAFETY: Timer-core serialization owns the scan and cpu_stall fields. The
    // native callback context retains IDR objects; lockless reads are revalidated
    // under the pool lock, with no references kept across unlocking or reporting.
    unsafe {
        let thresh = lupos_wq_read_once_ulong(lupos_wq_watchdog_thresh()).wrapping_mul(HZ as c_ulong);
        let mut max_stall_time: c_uint = 0;
        let mut lockup_detected = false;
        let mut cpu_pool_stall = false;
        let now = lupos_wq_read_jiffies();
        if thresh == 0 { return; }
        let mut pi: c_int = 0;
        loop {
            let pool = idr_get_next(lupos_wq_worker_pool_idr(), addr_of_mut!(pi)).cast::<worker_pool>();
            if pool.is_null() { break; }
            lupos_wq_assert_rcu_or_pool_mutex();
            pi = pi.wrapping_add(1);
            (*pool).cpu_stall = false;
            if lupos_wq_list_empty(addr_of!((*pool).worklist)) { continue; }
            // Native return is intentionally ignored: it may touch the watchdog
            // as a side effect when a VM was paused by its host.
            lupos_wq_kvm_check_and_clear_guest_paused();
            let touched = if (*pool).cpu >= 0 {
                lupos_wq_read_once_ulong(lupos_wq_watchdog_touched_cpu((*pool).cpu))
            } else {
                lupos_wq_read_once_ulong(lupos_wq_watchdog_touched())
            };
            let mut pool_ts = lupos_wq_read_once_ulong(addr_of!((*pool).last_progress_ts));
            let mut ts = if lupos_wq_time_after(pool_ts, touched) { pool_ts } else { touched };
            if lupos_wq_time_after(now, ts.wrapping_add(thresh)) {
                // Pinned scoped_guard(raw_spinlock_irqsave) uses refcounted
                // local-interrupt disable/enable, not legacy IRQ flags save.
                lupos_wq_pool_lock_irq_disable_c7979(pool);
                pool_ts = (*pool).last_progress_ts;
                ts = if lupos_wq_time_after(pool_ts, touched) { pool_ts } else { touched };
                lupos_wq_pool_unlock_irq_enable_c7979(pool);
                if !lupos_wq_time_after(now, ts.wrapping_add(thresh)) { continue; }
                lockup_detected = true;
                let stall_time = lupos_wq_jiffies_to_msecs(now.wrapping_sub(pool_ts)) / 1000;
                max_stall_time = core::cmp::max(max_stall_time, stall_time);
                if is_percpu_pool(pool) && lupos_wq_pool_flags_read(pool) & POOL_BH == 0 {
                    (*pool).cpu_stall = true;
                    cpu_pool_stall = true;
                }
                lupos_wq_pr_emerg(c"BUG: workqueue lockup - pool".as_ptr().cast());
                pr_cont_pool_info(pool);
                lupos_wq_pr_cont(c" stuck for %us!\n".as_ptr().cast(), stall_time);
            }
        }
        if lockup_detected { show_all_workqueues(); }
        if cpu_pool_stall { show_cpu_pools_busy_workers(); }
        if lockup_detected { panic_on_wq_watchdog(max_stall_time); }
        wq_watchdog_reset_touched();
        mod_timer(lupos_wq_watchdog_timer(), lupos_wq_read_jiffies().wrapping_add(thresh));
    }
}

/// Touches local/global watchdog progress with native throttling (C 8015–8029).
///
/// # Safety
/// cpu is a possible CPU (negative input warns as native). This function's native
/// notrace, callback, integer and code-generation attributes remain admission holds.
#[cfg(CONFIG_WQ_WATCHDOG)]
pub(super) unsafe extern "C" fn wq_watchdog_touch(cpu: c_int) {
    // SAFETY: Shared stores/READ_ONCE/WRITE_ONCE remain native primitives; no
    // stronger lock or Rust volatile substitute changes the original protocol.
    unsafe {
        let thresh = lupos_wq_read_once_ulong(lupos_wq_watchdog_thresh()).wrapping_mul(HZ as c_ulong);
        let touch_ts = lupos_wq_read_once_ulong(lupos_wq_watchdog_touched());
        let now = lupos_wq_read_jiffies();
        if cpu >= 0 {
            lupos_wq_write_ulong(lupos_wq_watchdog_touched_cpu(cpu), now);
        } else {
            lupos_wq_warn_watchdog_invalid_cpu_c8023();
        }
        if lupos_wq_time_after(now, touch_ts.wrapping_add(thresh / 4)) {
            lupos_wq_write_once_ulong(lupos_wq_watchdog_touched(), lupos_wq_read_jiffies());
        }
    }
}

/// Stops the timer before updating/restarting a nonzero threshold (C 8031–8041).
///
/// # Safety
/// Called during initialization or serialized native parameter update; may wait
/// for timer completion and must not run from that timer itself.
#[cfg(CONFIG_WQ_WATCHDOG)]
unsafe fn wq_watchdog_set_thresh(thresh: c_ulong) {
    // SAFETY: Setting zero precedes synchronous deletion; a new timer is armed
    // only after old callback completion, timestamp reset, and threshold write.
    unsafe {
        lupos_wq_write_ulong(lupos_wq_watchdog_thresh(), 0);
        lupos_wq_timer_delete_sync(lupos_wq_watchdog_timer());
        if thresh != 0 {
            lupos_wq_write_ulong(lupos_wq_watchdog_thresh(), thresh);
            wq_watchdog_reset_touched();
            mod_timer(lupos_wq_watchdog_timer(), lupos_wq_read_jiffies().wrapping_add(thresh.wrapping_mul(HZ as c_ulong)));
        }
    }
}

/// Parses a watchdog threshold, supporting early-boot assignment (C 8043–8059).
///
/// # Safety
/// Native parameter core serializes updates and supplies a terminated input.
#[cfg(CONFIG_WQ_WATCHDOG)]
pub(super) unsafe extern "C" fn wq_watchdog_param_set_thresh(val: *const c_char, _kp: *const kernel_param) -> c_int {
    // SAFETY: Successful native parse initializes thresh; system queue publication
    // decides whether timer infrastructure is initialized, exactly as native.
    unsafe {
        let mut thresh: c_ulong = 0;
        let ret = lupos_wq_kstrtoul(val, 0, addr_of_mut!(thresh));
        if ret != 0 { return ret; }
        if !lupos_wq_system_percpu_wq().is_null() {
            wq_watchdog_set_thresh(thresh);
        } else {
            lupos_wq_write_ulong(lupos_wq_watchdog_thresh(), thresh);
        }
        0
    }
}

/// Sets up the deferrable watchdog timer then applies its native threshold (C 8069–8073).
///
/// # Safety
/// Boot caller invokes this once after native timer infrastructure exists; the
/// common-owned timer setup leaf retains its static lockdep key and callback ABI.
#[cfg(CONFIG_WQ_WATCHDOG)]
pub(super) unsafe fn wq_watchdog_init() {
    // SAFETY: Timer is initialized before set_thresh synchronously deletes/rearms it.
    unsafe {
        lupos_wq_timer_setup_c8071(lupos_wq_watchdog_timer(), Some(wq_watchdog_timer_fn), TIMER_DEFERRABLE);
        wq_watchdog_set_thresh(lupos_wq_read_ulong(lupos_wq_watchdog_thresh()));
    }
}

/// Exact native watchdog-disabled init branch (C 8077).
///
/// # Safety
/// Matches the native disabled inline; no timer or state access occurs.
#[cfg(not(CONFIG_WQ_WATCHDOG))]
pub(super) unsafe fn wq_watchdog_init() {}

/// Exact public-header watchdog-disabled touch branch.
///
/// # Safety
/// Matches workqueue.h:901; no CPU state access occurs.
#[cfg(not(CONFIG_WQ_WATCHDOG))]
pub(super) unsafe extern "C" fn wq_watchdog_touch(_cpu: c_int) {}

/// Warns and dumps the stack on system-wide flush (C 8616–8620).
///
/// # Safety
/// Caller is an actual system-wide flush warning site; native diagnostic source
/// attribution/export/stack code-generation contracts remain admission holds.
pub(super) unsafe extern "C" fn __warn_flushing_systemwide_wq() {
    // SAFETY: The warning is unconditional and precedes the original stack dump.
    unsafe {
        lupos_wq_pr_warn(c"WARNING: Flushing system-wide workqueues will be prohibited in near future.\n".as_ptr().cast());
        #[cfg(CONFIG_PRINTK)]
        dump_stack();
    }
}
