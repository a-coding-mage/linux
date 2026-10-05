// SPDX-License-Identifier: GPL-2.0-only
//! Workqueue completion owner; native authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
//!
//! Source-only overlay. Shared types, constants, primitives and static keys come
//! exclusively from `common` and its configured `workqueue_native` bindings.
//! Native private layouts must not be approximated by Rust-owned structures.
//! No reference into shared native memory is retained across lock transitions,
//! callback execution, completion, or waits. Source/build/runtime validation is
//! not implied by the presence of these bodies.

use super::common::*;
use super::dispatch::{
    __queue_work,
    find_worker_executing_work,
    insert_work,
    pwq_is_empty,
    queue_work_on,
    work_grab_pending, //
};
#[cfg(CONFIG_PREEMPT_RT)]
use super::execution::workqueue_callback_cancel_wait_running;
use core::mem::MaybeUninit;
use core::mem::offset_of;
use core::ptr::{
    addr_of,
    addr_of_mut,
    null_mut, //
};

/// Checks reclaim forward-progress dependencies (C 3896–3917).
///
/// # Safety
/// `target_wq` and optional `target_work` must be live. The calling context must
/// preserve the current worker and its executing PWQ for these diagnostic reads.
unsafe fn check_flush_dependency(
    target_wq: *mut workqueue_struct,
    target_work: *mut work_struct,
    from_cancel: bool,
) {
    // SAFETY: The caller supplies live native objects; no borrow escapes a read.
    unsafe {
        if from_cancel || (*target_wq).flags & WQ_MEM_RECLAIM != 0 {
            return;
        }
        let worker = lupos_wq_current_wq_worker();
        let target_func = if target_work.is_null() { None } else { (*target_work).func };
        let task = lupos_wq_current();
        if (*task).flags & PF_MEMALLOC != 0 {
            lupos_wq_warn_c3909(
                (*task).pid,
                addr_of!((*task).comm).cast(),
                addr_of!((*target_wq).name).cast(),
                target_func,
            );
        }
        if !worker.is_null() {
            let source_wq = (*(*worker).current_pwq).wq;
            if (*source_wq).flags & (WQ_MEM_RECLAIM | __WQ_LEGACY) == WQ_MEM_RECLAIM {
                lupos_wq_warn_c3912(
                    addr_of!((*source_wq).name).cast(),
                    (*worker).current_func,
                    addr_of!((*target_wq).name).cast(),
                    target_func,
                );
            }
        }
    }
}

/// Signals a single barrier completion (C 3925–3929).
///
/// # Safety
/// `work` must be the `work` member of a live, initialized `wq_barrier`. The
/// waiting caller keeps the stack barrier alive until this completion is issued.
pub(super) unsafe extern "C" fn wq_barrier_func(work: *mut work_struct) {
    // SAFETY: The enclosing native object and completion are live by contract.
    unsafe {
        let barr = work.cast::<u8>().sub(offset_of!(wq_barrier, work)).cast::<wq_barrier>();
        lupos_wq_complete(addr_of_mut!((*barr).done));
    }
}

/// Links an uncancelable, inactive barrier behind one execution (C 3955–4006).
///
/// # Safety
/// The PWQ's pool lock is held with IRQs disabled. `barr` is uninitialized stack
/// storage retained through completion. `target` is queued on this PWQ or is
/// executing on `worker`; a non-null worker supplies the authoritative PWQ/color.
unsafe fn insert_wq_barrier(
    pwq: *mut pool_workqueue,
    barr: *mut wq_barrier,
    target: *mut work_struct,
    worker: *mut worker,
) {
    // SAFETY: The caller holds the native pool exclusion and barrier lifetime.
    unsafe {
        let barrier_work = addr_of_mut!((*barr).work);
        let key = if (*(*pwq).wq).flags & WQ_BH != 0 {
            lupos_wq_barrier_bh_key()
        } else {
            lupos_wq_barrier_thr_key()
        };
        init_work_with_key(
            barrier_work,
            Some(wq_barrier_func),
            true,
            b"(work_completion)(&barr->work)\0".as_ptr().cast::<c_char>(),
            key,
        );
        lupos_wq___set_bit(WORK_STRUCT_PENDING_BIT, work_data_bits(barrier_work));
        lupos_wq_init_completion_work_map(addr_of_mut!((*barr).done), target);
        (*barr).task = lupos_wq_current();

        // Barrier work contributes to color retirement but never nr_active.
        let mut work_flags = WORK_STRUCT_INACTIVE as c_uint;
        let head;
        let work_color;
        if !worker.is_null() {
            head = (*worker).scheduled.next;
            work_color = (*worker).current_color;
        } else {
            let bits = work_data_bits(target);
            head = (*target).entry.next;
            work_flags |= (*bits & WORK_STRUCT_LINKED as c_ulong) as c_uint;
            work_color = get_work_color(*bits) as c_uint;
            lupos_wq___set_bit(WORK_STRUCT_LINKED_BIT, bits);
        }
        let in_flight = addr_of_mut!((*pwq).nr_in_flight).cast::<c_int>().add(work_color as usize);
        *in_flight = (*in_flight).wrapping_add(1);
        work_flags |= work_color_to_flags(work_color as c_int);
        insert_work(pwq, barrier_work, head, work_flags);
    }
}

/// Arms one flush generation and/or advances PWQ enqueue colors (C 4039–4091).
///
/// # Safety
/// `wq->mutex` is held. Nonnegative colors are valid native color indices;
/// nonnegative `flush_color` requires a live `wq->first_flusher`. IRQs are enabled
/// on entry, as required by the native lock_irq/unlock_irq sequence.
unsafe fn flush_workqueue_prep_pwqs(
    wq: *mut workqueue_struct,
    flush_color: c_int,
    work_color: c_int,
) -> bool {
    // SAFETY: The mutex pins the native PWQ list; each mutable PWQ operation is
    // under its pool lock. The extra atomic reference prevents premature signal.
    unsafe {
        let mut wait = false;
        let mut current_pool: *mut worker_pool = null_mut();
        let count = addr_of_mut!((*wq).nr_pwqs_to_flush);
        if flush_color >= 0 {
            if lupos_wq_atomic_read(count) != 0 {
                lupos_wq_warn_c4047();
            }
            lupos_wq_atomic_set(count, 1);
        }
        let mut pwq = lupos_wq_pwq_first(wq);
        while !pwq.is_null() {
            if current_pool != (*pwq).pool {
                if !current_pool.is_null() {
                    lupos_wq_pool_unlock_irq(current_pool);
                }
                current_pool = (*pwq).pool;
                lupos_wq_pool_lock_irq(current_pool);
            }
            if flush_color >= 0 {
                if (*pwq).flush_color != -1 {
                    lupos_wq_warn_c4068();
                }
                let in_flight = addr_of!((*pwq).nr_in_flight).cast::<c_int>().add(flush_color as usize);
                if *in_flight != 0 {
                    (*pwq).flush_color = flush_color;
                    lupos_wq_atomic_inc(count);
                    wait = true;
                }
            }
            if work_color >= 0 {
                if work_color != work_next_color((*pwq).work_color) {
                    lupos_wq_warn_c4078();
                }
                (*pwq).work_color = work_color;
            }
            pwq = lupos_wq_pwq_next(wq, pwq);
        }
        if !current_pool.is_null() {
            lupos_wq_pool_unlock_irq(current_pool);
        }
        if flush_color >= 0 && lupos_wq_atomic_dec_and_test(count) {
            lupos_wq_complete(addr_of_mut!((*(*wq).first_flusher).done));
        }
        wait
    }
}

/// Records the workqueue flush lock dependency (C 4093–4108).
///
/// # Safety
/// `wq` must remain live, including its optional native lockdep map.
unsafe fn touch_wq_lockdep_map(wq: *mut workqueue_struct) {
    #[cfg(CONFIG_LOCKDEP)]
    // SAFETY: The caller pins the queue/map; BH state is balanced on every path.
    unsafe {
        if (*wq).lockdep_map.is_null() {
            return;
        }
        if (*wq).flags & WQ_BH != 0 {
            lupos_wq_local_bh_disable();
        }
        lupos_wq_lock_map_acquire_c4102((*wq).lockdep_map);
        lupos_wq_lock_map_release_c4103((*wq).lockdep_map);
        if (*wq).flags & WQ_BH != 0 {
            lupos_wq_local_bh_enable();
        }
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    let _ = wq;
}

/// Records the per-work flush lock dependency (C 4110–4123).
///
/// # Safety
/// `work` and `wq` must remain live through the map acquire/release pair.
unsafe fn touch_work_lockdep_map(work: *mut work_struct, wq: *mut workqueue_struct) {
    #[cfg(CONFIG_LOCKDEP)]
    // SAFETY: Both objects are pinned by the caller; no reference survives calls.
    unsafe {
        if (*wq).flags & WQ_BH != 0 {
            lupos_wq_local_bh_disable();
        }
        lupos_wq_lock_map_acquire_c4117(addr_of_mut!((*work).lockdep_map));
        lupos_wq_lock_map_release_c4118(addr_of_mut!((*work).lockdep_map));
        if (*wq).flags & WQ_BH != 0 {
            lupos_wq_local_bh_enable();
        }
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    let _ = (work, wq);
}

/// Waits for the queue's entry generation without blocking incoming work (C 4132–4279).
///
/// # Safety
/// The caller must retain the native workqueue throughout this sleepable call,
/// enter with IRQs enabled, and satisfy the native flush dependency rules.
pub unsafe extern "C" fn __flush_workqueue(wq: *mut workqueue_struct) {
    // SAFETY: The caller pins the queue. The stack flusher is initialized in its
    // final location before publication and retained until dequeued/completed.
    unsafe {
        let mut storage = MaybeUninit::<wq_flusher>::uninit();
        let this = storage.as_mut_ptr();
        let this_list = addr_of_mut!((*this).list);
        lupos_wq_init_list_head(this_list);
        (*this).flush_color = -1;
        lupos_wq_init_completion_wq_map(addr_of_mut!((*this).done), wq);
        if !lupos_wq_online() {
            lupos_wq_warn_c4141();
            return;
        }
        touch_wq_lockdep_map(wq);
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let queue = addr_of_mut!((*wq).flusher_queue);
        let overflow = addr_of_mut!((*wq).flusher_overflow);
        let next_color = work_next_color((*wq).work_color);
        if next_color != (*wq).flush_color {
            if !lupos_wq_list_empty(overflow) {
                lupos_wq_warn_c4159();
            }
            (*this).flush_color = (*wq).work_color;
            (*wq).work_color = next_color;
            if (*wq).first_flusher.is_null() {
                if (*wq).flush_color != (*this).flush_color {
                    lupos_wq_warn_c4165();
                }
                (*wq).first_flusher = this;
                if !flush_workqueue_prep_pwqs(wq, (*wq).flush_color, (*wq).work_color) {
                    (*wq).flush_color = next_color;
                    (*wq).first_flusher = null_mut();
                    lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
                    return;
                }
            } else {
                if (*wq).flush_color == (*this).flush_color {
                    lupos_wq_warn_c4178();
                }
                lupos_wq_list_add_tail(this_list, queue);
                flush_workqueue_prep_pwqs(wq, -1, (*wq).work_color);
            }
        } else {
            lupos_wq_list_add_tail(this_list, overflow);
        }
        check_flush_dependency(wq, null_mut(), false);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        lupos_wq_wait_for_completion(addr_of_mut!((*this).done));

        // Preserve native READ_ONCE and recheck under the mutex: another first
        // flusher may have completed and retired our stack flusher already.
        if lupos_wq_read_first_flusher(wq) != this {
            return;
        }
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        if (*wq).first_flusher != this {
            lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            return;
        }
        lupos_wq_write_first_flusher(wq, null_mut());
        if !lupos_wq_list_empty(this_list) {
            lupos_wq_warn_c4214();
        }
        if (*wq).flush_color != (*this).flush_color {
            lupos_wq_warn_c4215();
        }
        loop {
            // Popping the head before signaling avoids touching another
            // flusher's stack after complete() permits its waiter to return.
            while !lupos_wq_list_empty(queue) {
                let next = (*queue).next.cast::<u8>()
                    .sub(offset_of!(wq_flusher, list)).cast::<wq_flusher>();
                if (*next).flush_color != (*wq).flush_color {
                    break;
                }
                lupos_wq_list_del_init(addr_of_mut!((*next).list));
                lupos_wq_complete(addr_of_mut!((*next).done));
            }
            if !lupos_wq_list_empty(overflow)
                && (*wq).flush_color != work_next_color((*wq).work_color)
            {
                lupos_wq_warn_c4228();
            }
            (*wq).flush_color = work_next_color((*wq).flush_color);
            if !lupos_wq_list_empty(overflow) {
                let mut node = (*overflow).next;
                while node != overflow {
                    let flusher = node.cast::<u8>()
                        .sub(offset_of!(wq_flusher, list)).cast::<wq_flusher>();
                    (*flusher).flush_color = (*wq).work_color;
                    node = (*node).next;
                }
                (*wq).work_color = work_next_color((*wq).work_color);
                lupos_wq_list_splice_tail_init(overflow, queue);
                flush_workqueue_prep_pwqs(wq, -1, (*wq).work_color);
            }
            if lupos_wq_list_empty(queue) {
                if (*wq).flush_color != (*wq).work_color {
                    lupos_wq_warn_c4253();
                }
                break;
            }
            let next = (*queue).next.cast::<u8>()
                .sub(offset_of!(wq_flusher, list)).cast::<wq_flusher>();
            if (*wq).flush_color == (*wq).work_color {
                lupos_wq_warn_c4261();
            }
            if (*wq).flush_color != (*next).flush_color {
                lupos_wq_warn_c4262();
            }
            lupos_wq_list_del_init(addr_of_mut!((*next).list));
            (*wq).first_flusher = next;
            if flush_workqueue_prep_pwqs(wq, (*wq).flush_color, -1) {
                break;
            }
            (*wq).first_flusher = null_mut();
        }
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
    }
}

/// Drains a queue while allowing only chained submissions (C 4293–4334).
///
/// # Safety
/// The queue remains live across all repeated sleepable flushes. IRQs are
/// enabled, and the caller satisfies native queue draining dependency rules.
pub unsafe extern "C" fn drain_workqueue(wq: *mut workqueue_struct) {
    // SAFETY: The queue is pinned. Its mutex serializes drainer/flag accounting;
    // the separate pool lock protects each empty check, exactly as in native C.
    unsafe {
        let mut flush_cnt: c_uint = 0;
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let previous = (*wq).nr_drainers;
        (*wq).nr_drainers = (*wq).nr_drainers.wrapping_add(1);
        if previous == 0 {
            (*wq).flags |= __WQ_DRAINING;
        }
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        'reflush: loop {
            __flush_workqueue(wq);
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
            let mut pwq = lupos_wq_pwq_first(wq);
            while !pwq.is_null() {
                let pool = (*pwq).pool;
                lupos_wq_pool_lock_irq(pool);
                let drained = pwq_is_empty(pwq);
                lupos_wq_pool_unlock_irq(pool);
                if !drained {
                    flush_cnt = flush_cnt.wrapping_add(1);
                    if flush_cnt == 10 || (flush_cnt % 100 == 0 && flush_cnt <= 1000) {
                        lupos_wq_warn_c4324(addr_of!((*wq).name).cast(), flush_cnt);
                    }
                    lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
                    continue 'reflush;
                }
                pwq = lupos_wq_pwq_next(wq, pwq);
            }
            (*wq).nr_drainers = (*wq).nr_drainers.wrapping_sub(1);
            if (*wq).nr_drainers == 0 {
                (*wq).flags &= !__WQ_DRAINING;
            }
            lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            break;
        }
    }
}

/// Validates the execution instance and attaches its barrier (C 4337–4391).
///
/// # Safety
/// `work` remains live and may be queued/running. `barr` is caller-owned stack
/// storage retained until its completion whenever this function returns true.
/// IRQs must be enabled. `from_cancel` requires retained disable/pending control.
unsafe fn start_flush_work(work: *mut work_struct, barr: *mut wq_barrier, from_cancel: bool) -> bool {
    // SAFETY: RCU pins pool/PWQ lifetime; pool locking revalidates work placement.
    // All exits drop each acquired protection, and no Rust borrow crosses them.
    unsafe {
        lupos_wq_rcu_read_lock();
        let pool = get_work_pool(work);
        if pool.is_null() {
            lupos_wq_rcu_read_unlock();
            return false;
        }
        lupos_wq_pool_lock_irq(pool);
        let mut worker = null_mut();
        let mut pwq = get_work_pwq(work);
        if !pwq.is_null() {
            if (*pwq).pool != pool {
                lupos_wq_pool_unlock_irq(pool);
                lupos_wq_rcu_read_unlock();
                return false;
            }
        } else {
            worker = find_worker_executing_work(pool, work);
            if worker.is_null() {
                lupos_wq_pool_unlock_irq(pool);
                lupos_wq_rcu_read_unlock();
                return false;
            }
            pwq = (*worker).current_pwq;
        }
        let wq = (*pwq).wq;
        check_flush_dependency(wq, work, from_cancel);
        insert_wq_barrier(pwq, barr, work, worker);
        lupos_wq_pool_unlock_irq(pool);
        touch_work_lockdep_map(work, wq);
        if !from_cancel && ((*wq).saved_max_active == 1 || !(*wq).rescuer.is_null()) {
            touch_wq_lockdep_map(wq);
        }
        lupos_wq_rcu_read_unlock();
        true
    }
}

/// Waits on a per-item barrier, with the native BH-cancel path (C 4393–4444).
///
/// # Safety
/// `work` remains live. Normal flush requires a sleepable context; cancellation
/// requires a disable count excluding new queueing, and permits BH atomic
/// context only when native off-queue data records a BH execution.
unsafe fn __flush_work(work: *mut work_struct, from_cancel: bool) -> bool {
    // SAFETY: Caller guarantees the work lifetime. The barrier is initialized
    // before publication, completed once, then destroyed before stack storage dies.
    unsafe {
        let mut storage = MaybeUninit::<wq_barrier>::uninit();
        let barr = storage.as_mut_ptr();
        if !lupos_wq_online() {
            lupos_wq_warn_c4397();
            return false;
        }
        if (*work).func.is_none() {
            lupos_wq_warn_c4400();
            return false;
        }
        if !start_flush_work(work, barr, from_cancel) {
            return false;
        }
        let mut bh_cancel = false;
        if from_cancel {
            let data = *work_data_bits(work);
            if data & WORK_STRUCT_PWQ as c_ulong != 0 {
                lupos_wq_warn_c4416();
            } else {
                bh_cancel = data & WORK_OFFQ_BH as c_ulong != 0;
            }
        }
        if bh_cancel {
            while !lupos_wq_try_wait_for_completion(addr_of_mut!((*barr).done)) {
                #[cfg(CONFIG_PREEMPT_RT)]
                {
                    lupos_wq_rcu_read_lock();
                    let pool = get_work_pool(work);
                    if !pool.is_null() {
                        workqueue_callback_cancel_wait_running(pool);
                    }
                    lupos_wq_rcu_read_unlock();
                }
                #[cfg(not(CONFIG_PREEMPT_RT))]
                lupos_wq_cpu_relax();
            }
        } else {
            lupos_wq_wait_for_completion(addr_of_mut!((*barr).done));
        }
        lupos_wq_destroy_work_on_stack(addr_of_mut!((*barr).work));
        true
    }
}

/// Flushes the last queueing instance of one ordinary work item (C 4457–4461).
///
/// # Safety
/// `work` is a live initialized work item and the caller may sleep.
pub unsafe extern "C" fn flush_work(work: *mut work_struct) -> bool {
    // SAFETY: The caller guarantees the native work lifetime and sleepable context.
    unsafe {
        lupos_wq_might_sleep_c4459();
        __flush_work(work, false)
    }
}

/// Cancels the delay, queues the pending instance, then flushes it (C 4476–4483).
///
/// # Safety
/// `dwork` remains live, is initialized as delayed work with an IRQ-safe timer,
/// and the caller enters sleepable with interrupts enabled.
pub unsafe extern "C" fn flush_delayed_work(dwork: *mut delayed_work) -> bool {
    // SAFETY: Native timer synchronization plus local IRQ exclusion preserves
    // pending ownership until dispatch queues the same work instance.
    unsafe {
        lupos_wq_local_irq_disable();
        if lupos_wq_timer_delete_sync(addr_of_mut!((*dwork).timer)) != 0 {
            __queue_work((*dwork).cpu, (*dwork).wq, addr_of_mut!((*dwork).work));
        }
        lupos_wq_local_irq_enable();
        flush_work(addr_of_mut!((*dwork).work))
    }
}

/// Flushes an RCU work instance, including a pending callback (C 4494–4503).
///
/// # Safety
/// `rwork` remains live, was initialized as RCU work, and caller may sleep.
pub unsafe extern "C" fn flush_rcu_work(rwork: *mut rcu_work) -> bool {
    // SAFETY: The retained RCU-work lifetime covers the full RCU barrier and work
    // flush; the pending bit is tested with the native bit operation.
    unsafe {
        let work = addr_of_mut!((*rwork).work);
        if lupos_wq_test_bit(WORK_STRUCT_PENDING_BIT, work_data_bits(work)) {
            lupos_wq_rcu_barrier();
            flush_work(work);
            true
        } else {
            flush_work(work)
        }
    }
}

/// Increments a bounded native nested-disable depth (C 4506–4514).
///
/// # Safety
/// `offqd` is exclusively held, initialized off-queue state unpacked under
/// pending ownership; no concurrent writer may access this temporary structure.
unsafe fn work_offqd_disable(offqd: *mut work_offq_data) {
    // SAFETY: The caller owns the local decoded state; only the bounded count
    // changes. Overflow warns without inventing another disable reference.
    unsafe {
        let max = ((1 as c_ulong) << WORK_OFFQ_DISABLE_BITS) - 1;
        if ((*offqd).disable as c_ulong) < max {
            (*offqd).disable = (*offqd).disable.wrapping_add(1);
        } else {
            lupos_wq_warn_c4513();
        }
    }
}

/// Decrements native nested-disable depth, diagnosing underflow (C 4516–4522).
///
/// # Safety
/// `offqd` is exclusively held initialized local off-queue state.
unsafe fn work_offqd_enable(offqd: *mut work_offq_data) {
    // SAFETY: Exclusive temporary-state access and the positive-count check
    // guarantee the decrement does not wrap; native underflow remains a warning.
    unsafe {
        if (*offqd).disable > 0 {
            (*offqd).disable -= 1;
        } else {
            lupos_wq_warn_c4521();
        }
    }
}

/// Takes pending ownership, removes pending work and optionally disables it (C 4524–4541).
///
/// # Safety
/// `work` remains live and is ordinary or delayed work, never RCU work. `cflags`
/// correctly identifies delayed work. Native pending-grab context rules apply.
unsafe fn __cancel_work(work: *mut work_struct, cflags: u32) -> bool {
    // SAFETY: Dispatch acquires PENDING and returns with IRQs disabled. The
    // unpacked pool/BH flags are preserved; common clearing supplies wmb/mb.
    unsafe {
        let mut storage = MaybeUninit::<work_offq_data>::uninit();
        let offqd = storage.as_mut_ptr();
        let mut irq_flags: c_ulong = 0;
        let ret = work_grab_pending(work, cflags, addr_of_mut!(irq_flags));
        work_offqd_unpack(offqd, *work_data_bits(work));
        if cflags & WORK_CANCEL_DISABLE as u32 != 0 {
            work_offqd_disable(offqd);
        }
        set_work_pool_and_clear_pending(
            work,
            (*offqd).pool_id as c_int,
            work_offqd_pack_flags(offqd),
        );
        lupos_wq_local_irq_restore(irq_flags);
        ret
    }
}

/// Cancels with temporary requeue exclusion and waits for execution (C 4543–4565).
///
/// # Safety
/// `work` remains live, is not RCU work, and `cflags` identifies its timer kind.
/// The caller may sleep for non-BH work; BH work permits non-hardirq atomic
/// context. Concurrent queueing must be excluded for a durable idle guarantee.
unsafe fn __cancel_work_sync(work: *mut work_struct, cflags: u32) -> bool {
    // SAFETY: Pending removal plus temporary disable prevents new queueing while
    // the running instance is flushed. The same disable reference is restored.
    unsafe {
        let ret = __cancel_work(work, cflags | WORK_CANCEL_DISABLE as u32);
        if *work_data_bits(work) & WORK_OFFQ_BH as c_ulong != 0 {
            if lupos_wq_in_hardirq() {
                lupos_wq_warn_c4550();
            }
        } else {
            lupos_wq_might_sleep_c4552();
        }
        if lupos_wq_online() {
            __flush_work(work, true);
        }
        if cflags & WORK_CANCEL_DISABLE as u32 == 0 {
            enable_work(work);
        }
        ret
    }
}

/// Cancels an ordinary work's pending instance without waiting (C 4570–4573).
///
/// # Safety
/// `work` is live ordinary work, not delayed or RCU work. Running callbacks may
/// still access it on return; the caller must preserve their required lifetime.
pub unsafe extern "C" fn cancel_work(work: *mut work_struct) -> bool {
    // SAFETY: The caller provides the lifetime and work-kind requirements.
    unsafe { __cancel_work(work, 0) }
}

/// Cancels and waits for an ordinary work item (C 4594–4597).
///
/// # Safety
/// `work` is live ordinary work, not delayed or RCU work. The caller satisfies
/// the sleepable/non-hardirq-BH context and requeue-exclusion rules above.
pub unsafe extern "C" fn cancel_work_sync(work: *mut work_struct) -> bool {
    // SAFETY: The caller provides __cancel_work_sync's lifetime/context contract.
    unsafe { __cancel_work_sync(work, 0) }
}

/// Cancels pending timer or queue membership without waiting (C 4616–4619).
///
/// # Safety
/// `dwork` is live initialized delayed work; a concurrently running callback
/// must retain the storage it uses even after this function returns.
pub unsafe extern "C" fn cancel_delayed_work(dwork: *mut delayed_work) -> bool {
    // SAFETY: The native embedded work is identified as delayed for timer stealing.
    unsafe { __cancel_work(addr_of_mut!((*dwork).work), WORK_CANCEL_DELAYED as u32) }
}

/// Cancels delayed work and waits for any executing instance (C 4631–4634).
///
/// # Safety
/// `dwork` remains live and the caller satisfies native sync-cancel context and
/// concurrent-requeue rules.
pub unsafe extern "C" fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool {
    // SAFETY: The embedded native work remains live and has the correct timer flag.
    unsafe { __cancel_work_sync(addr_of_mut!((*dwork).work), WORK_CANCEL_DELAYED as u32) }
}

/// Adds a disable reference and cancels pending ordinary work (C 4649–4652).
///
/// # Safety
/// `work` is live ordinary work, not delayed or RCU work; running callback
/// lifetime must remain valid after this non-waiting operation returns.
pub unsafe extern "C" fn disable_work(work: *mut work_struct) -> bool {
    // SAFETY: The caller supplies ordinary-work lifetime and kind guarantees.
    unsafe { __cancel_work(work, WORK_CANCEL_DISABLE as u32) }
}

/// Adds a disable reference, cancels and waits for ordinary work (C 4668–4671).
///
/// # Safety
/// `work` is live ordinary work and native sync-cancel context rules hold.
pub unsafe extern "C" fn disable_work_sync(work: *mut work_struct) -> bool {
    // SAFETY: The caller supplies ordinary-work lifetime and sync context.
    unsafe { __cancel_work_sync(work, WORK_CANCEL_DISABLE as u32) }
}

/// Removes one disable reference and reports whether depth is zero (C 4684–4698).
///
/// # Safety
/// `work` remains live and is disabled ordinary or delayed work, never RCU work.
/// The caller balances a prior disable; violations retain native diagnostics.
pub unsafe extern "C" fn enable_work(work: *mut work_struct) -> bool {
    // SAFETY: Pending ownership serializes off-queue data; common clearing
    // preserves native barriers and retained pool/BH metadata before IRQ restore.
    unsafe {
        let mut storage = MaybeUninit::<work_offq_data>::uninit();
        let offqd = storage.as_mut_ptr();
        let mut irq_flags: c_ulong = 0;
        work_grab_pending(work, 0, addr_of_mut!(irq_flags));
        work_offqd_unpack(offqd, *work_data_bits(work));
        work_offqd_enable(offqd);
        set_work_pool_and_clear_pending(
            work,
            (*offqd).pool_id as c_int,
            work_offqd_pack_flags(offqd),
        );
        lupos_wq_local_irq_restore(irq_flags);
        (*offqd).disable == 0
    }
}

/// Adds a disable reference and cancels pending delayed work (C 4707–4711).
///
/// # Safety
/// `dwork` is initialized delayed work and outlives all still-running callbacks.
pub unsafe extern "C" fn disable_delayed_work(dwork: *mut delayed_work) -> bool {
    // SAFETY: Both flags describe the requested timer-aware cancel/disable action.
    unsafe {
        __cancel_work(
            addr_of_mut!((*dwork).work),
            (WORK_CANCEL_DELAYED | WORK_CANCEL_DISABLE) as u32,
        )
    }
}

/// Adds a disable reference, cancels and waits for delayed work (C 4720–4724).
///
/// # Safety
/// `dwork` remains live and native sync-cancel context/requeue rules hold.
pub unsafe extern "C" fn disable_delayed_work_sync(dwork: *mut delayed_work) -> bool {
    // SAFETY: The live embedded work has the correct timer and disable flags.
    unsafe {
        __cancel_work_sync(
            addr_of_mut!((*dwork).work),
            (WORK_CANCEL_DELAYED | WORK_CANCEL_DISABLE) as u32,
        )
    }
}

/// Removes one disable reference from delayed work (C 4733–4736).
///
/// # Safety
/// `dwork` is live disabled delayed work; the caller balances a prior disable.
pub unsafe extern "C" fn enable_delayed_work(dwork: *mut delayed_work) -> bool {
    // SAFETY: Disabled delayed work has no live pending timer to steal here.
    unsafe { enable_work(addr_of_mut!((*dwork).work)) }
}

/// Executes and flushes one callback on every online CPU (C 4750–4774).
///
/// # Safety
/// `func` is a valid non-null native work callback. The caller may allocate and
/// sleep, and callback lifetimes extend through all per-CPU work completions.
pub unsafe extern "C" fn schedule_on_each_cpu(func: work_func_t) -> c_int {
    // SAFETY: Native per-CPU allocation/addressing supplies configured alignment
    // and layout. The hotplug read lock freezes both online-CPU traversals.
    unsafe {
        let works = lupos_wq_alloc_percpu_work();
        if works.is_null() {
            return -(ENOMEM as c_int);
        }
        lupos_wq_cpus_read_lock();
        let mut cpu = lupos_wq_first_online_cpu();
        while cpu >= 0 {
            let work = lupos_wq_percpu_work_at(works, cpu);
            init_work_with_key(
                work,
                func,
                false,
                b"(work_completion)(work)\0".as_ptr().cast::<c_char>(),
                lupos_wq_schedule_each_cpu_key(),
            );
            queue_work_on(cpu, system_percpu_wq, work);
            cpu = lupos_wq_next_online_cpu(cpu);
        }
        cpu = lupos_wq_first_online_cpu();
        while cpu >= 0 {
            flush_work(lupos_wq_percpu_work_at(works, cpu));
            cpu = lupos_wq_next_online_cpu(cpu);
        }
        lupos_wq_cpus_read_unlock();
        lupos_wq_free_percpu_work(works);
        0
    }
}

/// Runs immediately in process context or schedules from interrupt context (C 4788–4799).
///
/// # Safety
/// `func` is a non-null native callback valid for the selected execution context.
/// `ew` has suitable work storage retained through asynchronous execution. No
/// conflicting work use may overlap initialization on the interrupt path.
pub unsafe extern "C" fn execute_in_process_context(func: work_func_t, ew: *mut execute_work) -> c_int {
    // SAFETY: The callback is non-null by contract. Only its work address is
    // passed; after immediate invocation no field of possibly freed ew is read.
    unsafe {
        let work = addr_of_mut!((*ew).work);
        if !lupos_wq_in_interrupt() {
            let callback = func.unwrap_unchecked();
            callback(work);
            return 0;
        }
        init_work_with_key(
            work,
            func,
            false,
            b"(work_completion)(&ew->work)\0".as_ptr().cast::<c_char>(),
            lupos_wq_execute_process_context_key(),
        );
        queue_work_on(WORK_CPU_UNBOUND as c_int, system_percpu_wq, work);
        1
    }
}
