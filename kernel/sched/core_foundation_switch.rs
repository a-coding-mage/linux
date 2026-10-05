// SPDX-License-Identifier: GPL-2.0-only
#[no_mangle]
pub static mut balance_push_callback: balance_callback = balance_callback {
    next: null_mut(),
    func: Some(balance_push),
};
unsafe fn zap_balance_callbacks(rq: *mut rq) {
    // SAFETY: The __schedule caller holds the live rq's lock and owns its
    // callback list; each next link is saved before detaching the current node.
    unsafe {
        lupos_core_assert_rq_held(rq);
        let mut head = (*rq).balance_callback;
        let mut found = false;
        while !head.is_null() {
            if head == addr_of_mut!(balance_push_callback) {
                found = true;
            }
            let next = (*head).next;
            (*head).next = null_mut();
            head = next;
        }
        (*rq).balance_callback = if found {
            addr_of_mut!(balance_push_callback)
        } else {
            null_mut()
        };
    }
}
unsafe fn do_balance_callbacks(rq: *mut rq, mut head: *mut balance_callback) {
    // SAFETY: The caller holds rq's lock and owns the live callback list.
    // Callbacks retain their native lock contract; links are saved before calls.
    unsafe {
        lupos_core_assert_rq_held(rq);
        while !head.is_null() {
            let func = (*head).func.unwrap_unchecked();
            let next = (*head).next;
            (*head).next = null_mut();
            head = next;
            func(rq);
        }
    }
}
unsafe fn __splice_balance_callbacks(rq: *mut rq, split: bool) -> *mut balance_callback {
    // SAFETY: The caller holds the live rq's lock when detaching callbacks;
    // split mode leaves the special balance-push callback installed.
    unsafe {
        let head = (*rq).balance_callback;
        if head.is_null() {
            return null_mut();
        }
        lupos_core_assert_rq_held(rq);
        if split && head == addr_of_mut!(balance_push_callback) {
            return null_mut();
        }
        (*rq).balance_callback = null_mut();
        head
    }
}
#[no_mangle]
pub unsafe extern "C" fn splice_balance_callbacks(rq: *mut rq) -> *mut balance_callback {
    // SAFETY: The caller holds rq's lock and assumes ownership of the
    // detached native callback list for later balance_callbacks processing.
    unsafe {
        __splice_balance_callbacks(rq, true)
    }
}
#[no_mangle]
pub unsafe extern "C" fn __balance_callbacks(rq: *mut rq, rf: *mut rq_flags) {
    // SAFETY: The caller holds rq's lock and, when non-null, rf matches its
    // lock pin. Callbacks preserve locking between unpin and repin.
    unsafe {
        if !rf.is_null() {
            lupos_core_rq_unpin_lock(rq, rf);
        }
        do_balance_callbacks(rq, __splice_balance_callbacks(rq, false));
        if !rf.is_null() {
            lupos_core_rq_repin_lock(rq, rf);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn balance_callbacks(rq: *mut rq, head: *mut balance_callback) {
    // SAFETY: rq and the detached callback list remain live. The acquired
    // IRQ-saving rq lock covers callback execution and is restored afterward.
    unsafe {
        if !head.is_null() {
            let flags = lupos_core_raw_spin_rq_lock_irqsave(rq);
            do_balance_callbacks(rq, head);
            lupos_core_raw_spin_rq_unlock_irqrestore(rq, flags);
        }
    }
}
#[inline(always)]
unsafe fn prepare_task(next: *mut task_struct) {
    // SAFETY: The scheduler caller keeps next live under the rq lock; the
    // native on_cpu store publishes ownership before the context transfer.
    unsafe {
        lupos_core_write_once_u8(addr_of_mut!((*next).on_cpu), 1);
    }
}
#[inline(always)]
unsafe fn finish_task(prev: *mut task_struct) {
    // SAFETY: The switch caller has completed prev's final running-task
    // reads; the native release store pairs with wakeup's on_cpu acquire.
    unsafe {
        // Must remain after the prev->__state read in finish_task_switch.
        lupos_core_store_release_on_cpu(prev, 0);
    }
}
unsafe fn prepare_lock_switch(rq: *mut rq, _next: *mut task_struct, rf: *mut rq_flags) {
    // SAFETY: The caller owns the live rq lock and matching pin flags;
    // lockdep and optional debug owner updates prepare its native handoff.
    unsafe {
        lupos_core_rq_unpin_lock(rq, rf);
        lupos_core_spin_release_switch(rq, lupos_core_this_ip!());
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lupos_core_switch_lock_owner(rq, _next);
        }
        lupos_core_context_transfer_rq(rq);
    }
}
unsafe fn finish_lock_switch(rq: *mut rq) {
    // SAFETY: The resumed task owns the carried-over live rq lock with IRQs
    // disabled; callbacks/timer exit preserve it until the native unlock.
    unsafe {
        lupos_core_spin_acquire_switch(rq, lupos_core_this_ip!());
        __balance_callbacks(rq, null_mut());
        hrtick_schedule_exit(rq);
        lupos_core_raw_spin_rq_unlock_irq(rq);
    }
}
#[cfg(CONFIG_KMAP_LOCAL)]
unsafe fn kmap_local_sched_out() {
    // SAFETY: The scheduler caller keeps current live across the native
    // local-map save operation before switching away.
    unsafe {
        if (*lupos_core_current()).kmap_ctrl.idx != 0 {
            __kmap_local_sched_out();
        }
    }
}
#[cfg(not(CONFIG_KMAP_LOCAL))]
unsafe fn kmap_local_sched_out() {}
#[cfg(CONFIG_KMAP_LOCAL)]
unsafe fn kmap_local_sched_in() {
    // SAFETY: The resumed scheduler task keeps current live while the native
    // local-map restore operation reinstates its saved mapping state.
    unsafe {
        if (*lupos_core_current()).kmap_ctrl.idx != 0 {
            __kmap_local_sched_in();
        }
    }
}
#[cfg(not(CONFIG_KMAP_LOCAL))]
unsafe fn kmap_local_sched_in() {}
unsafe fn prepare_task_switch(rq: *mut rq, prev: *mut task_struct, next: *mut task_struct) {
    // SAFETY: The caller holds rq's lock with IRQs disabled and keeps prev
    // and next live; this must be paired with finish_task_switch after transfer.
    unsafe {
        lupos_core_kcov_prepare_switch(prev);
        lupos_core_sched_info_switch(rq, prev, next);
        lupos_core_perf_task_sched_out(prev, next);
        fire_sched_out_preempt_notifiers(prev, next);
        kmap_local_sched_out();
        prepare_task(next);
        lupos_core_prepare_arch_switch(next);
    }
}
unsafe fn finish_task_switch(prev: *mut task_struct) -> *mut rq {
    // SAFETY: The native switch contract supplies prev and the carried rq
    // lock. Its state is sampled before release; DEAD-task references last until
    // the final put, and deferred MM release happens after unlocking.
    unsafe {
        // Recompute after switch: this task may have migrated since suspending.
        let rq = lupos_core_this_rq();
        let mm = (*rq).prev_mm;
        let count = lupos_core_preempt_count();
        if lupos_core_warn_switch_preempt(count != 2 * LUPOS_CORE_PREEMPT_DISABLE_OFFSET, count) {
            lupos_core_preempt_count_set(LUPOS_CORE_FORK_PREEMPT_COUNT);
        }
        (*rq).prev_mm = null_mut();
        let prev_state = lupos_core_read_once_uint(addr_of!((*prev).__state));
        lupos_core_vtime_task_switch(prev);
        lupos_core_perf_task_sched_in(prev, lupos_core_current());
        finish_task(prev);
        lupos_core_tick_nohz_task_switch();
        finish_lock_switch(rq);
        lupos_core_finish_arch_post_lock_switch();
        lupos_core_kcov_finish_switch(lupos_core_current());
        kmap_local_sched_in();
        lupos_core_blk_plug_invalidate_ts();
        fire_sched_in_preempt_notifiers(lupos_core_current());
        if !mm.is_null() {
            lupos_core_membarrier_sync_core(mm);
            lupos_core_mmdrop_lazy_tlb_sched(mm);
        }
        if prev_state == LUPOS_CORE_TASK_DEAD {
            if let Some(task_dead) = (*(*prev).sched_class).task_dead {
                task_dead(prev);
            }
            lupos_core_sched_ext_dead(prev);
            lupos_core_cgroup_task_dead(prev);
            lupos_core_put_task_stack(prev);
            put_task_struct_rcu_user(prev);
        }
        rq
    }
}
#[no_mangle]
pub unsafe extern "C" fn schedule_tail(prev: *mut task_struct) {
    // SAFETY: The native first-run entry supplies prev and the carried rq
    // lock/preempt state; fork completion releases them before user-TID storage.
    unsafe {
        finish_task_switch(prev);
        lupos_core_trace_sched_exit(true);
        lupos_core_preempt_enable();
        let current = lupos_core_current();
        if !(*current).set_child_tid.is_null() {
            // Original ignores put_user failure; preserve that behavior.
            lupos_core_put_user_pid(lupos_core_task_pid_vnr(current), (*current).set_child_tid);
        }
        calculate_sigpending();
    }
}
#[inline(always)]
unsafe fn context_switch(
    rq: *mut rq,
    prev: *mut task_struct,
    next: *mut task_struct,
    rf: *mut rq_flags,
) -> *mut rq {
    // SAFETY: The caller owns rq with IRQs disabled and live prev/next/MM
    // state. Only raw pointers cross the native transfer; emitted switch ABI,
    // stack and protection qualification remain mandatory admission gates.
    unsafe {
        prepare_task_switch(rq, prev, next);
        lupos_core_arch_start_context_switch(prev);
        if (*next).mm.is_null() {
            lupos_core_enter_lazy_tlb((*prev).active_mm, next);
            (*next).active_mm = (*prev).active_mm;
            if !(*prev).mm.is_null() {
                lupos_core_mmgrab_lazy_tlb((*prev).active_mm);
            } else {
                (*prev).active_mm = null_mut();
            }
        } else {
            lupos_core_membarrier_switch_mm(rq, (*prev).active_mm, (*next).mm);
            lupos_core_switch_mm_irqs_off((*prev).active_mm, (*next).mm, next);
            lupos_core_lru_gen_use_mm((*next).mm);
            if (*prev).mm.is_null() {
                (*rq).prev_mm = (*prev).active_mm;
                (*prev).active_mm = null_mut();
            }
        }
        lupos_core_mm_cid_switch_to(prev, next);
        lupos_core_rseq_sched_switch_event(next);
        prepare_lock_switch(rq, next, rf);
        // X86_64 source candidate is the exact switch_to macro expansion from
        // asm/switch_to.h, with no added C stack frame or body forwarding.
        // Native compiler-policy and emitted ABI review remain admission gates.
        #[cfg(CONFIG_X86_64)]
        let prev = __switch_to_asm(prev, next);
        #[cfg(not(CONFIG_X86_64))]
        let prev = lupos_core_arch_switch_to(prev, next);
        // Original barrier() is a call-site compiler memory clobber.
        #[cfg(CONFIG_X86_64)]
        core::arch::asm!("", options(nostack, preserves_flags));
        #[cfg(not(CONFIG_X86_64))]
        lupos_core_compiler_barrier();
        finish_task_switch(prev)
    }
}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
#[no_mangle]
pub unsafe extern "C" fn preempt_notifier_inc() {
    // SAFETY: The caller follows the native notifier-enable lifetime
    // contract; the native static-key operation updates permanent storage.
    unsafe {
        lupos_core_preempt_notifier_key_inc();
    }
}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
#[no_mangle]
pub unsafe extern "C" fn preempt_notifier_dec() {
    // SAFETY: The caller pairs a prior notifier-enable reference under the
    // native contract; the static-key operation updates permanent storage.
    unsafe {
        lupos_core_preempt_notifier_key_dec();
    }
}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
#[no_mangle]
pub unsafe extern "C" fn preempt_notifier_register(notifier: *mut preempt_notifier) {
    // SAFETY: The caller owns a live unlinked notifier and serializes
    // registration on current; it remains live until native unregistration.
    unsafe {
        lupos_core_warn_notifier_disabled(!lupos_core_preempt_notifier_enabled());
        lupos_core_hlist_add_head(
            addr_of_mut!((*notifier).link),
            addr_of_mut!((*lupos_core_current()).preempt_notifiers),
        );
    }
}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
#[no_mangle]
pub unsafe extern "C" fn preempt_notifier_unregister(notifier: *mut preempt_notifier) {
    // SAFETY: The caller owns a registered notifier and serializes removal
    // outside a notifier callback, as required by the native iteration contract.
    unsafe {
        lupos_core_hlist_del(addr_of_mut!((*notifier).link));
    }
}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
unsafe fn __fire_sched_in_preempt_notifiers(curr: *mut task_struct) {
    // SAFETY: curr and registered notifiers remain live during the scheduler
    // callback traversal; callbacks must not unregister nodes during iteration.
    unsafe {
        let mut node = (*curr).preempt_notifiers.first;
        while !node.is_null() {
            let notifier = node
                .cast::<u8>()
                .sub(offset_of!(preempt_notifier, link))
                .cast::<preempt_notifier>();
            ((*(*notifier).ops).sched_in.unwrap_unchecked())(
                notifier,
                lupos_core_raw_smp_processor_id(),
            );
            node = (*node).next;
        }
    }
}
#[inline(always)]
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
unsafe fn fire_sched_in_preempt_notifiers(curr: *mut task_struct) {
    // SAFETY: The switch caller keeps curr and its notifier list live;
    // the native static key guards callbacks under the registration contract.
    unsafe {
        if lupos_core_preempt_notifier_enabled() {
            __fire_sched_in_preempt_notifiers(curr);
        }
    }
}
#[cfg(not(CONFIG_PREEMPT_NOTIFIERS))]
#[inline(always)]
unsafe fn fire_sched_in_preempt_notifiers(_curr: *mut task_struct) {}
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
unsafe fn __fire_sched_out_preempt_notifiers(curr: *mut task_struct, next: *mut task_struct) {
    // SAFETY: curr, next and registered notifier nodes remain live during
    // the scheduler traversal; callbacks must not unregister nodes here.
    unsafe {
        let mut node = (*curr).preempt_notifiers.first;
        while !node.is_null() {
            let notifier = node
                .cast::<u8>()
                .sub(offset_of!(preempt_notifier, link))
                .cast::<preempt_notifier>();
            ((*(*notifier).ops).sched_out.unwrap_unchecked())(notifier, next);
            node = (*node).next;
        }
    }
}
#[inline(always)]
#[cfg(CONFIG_PREEMPT_NOTIFIERS)]
unsafe fn fire_sched_out_preempt_notifiers(curr: *mut task_struct, next: *mut task_struct) {
    // SAFETY: The switch caller keeps curr/next and registered notifiers
    // live; the native key guards their sched-out callbacks.
    unsafe {
        if lupos_core_preempt_notifier_enabled() {
            __fire_sched_out_preempt_notifiers(curr, next);
        }
    }
}
#[cfg(not(CONFIG_PREEMPT_NOTIFIERS))]
#[inline(always)]
unsafe fn fire_sched_out_preempt_notifiers(_curr: *mut task_struct, _next: *mut task_struct) {}
