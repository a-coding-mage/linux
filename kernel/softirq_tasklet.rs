// SPDX-License-Identifier: GPL-2.0-only
// Queue, callback, cancellation and CPU-death ownership, from softirq.c.

unsafe fn tasklet_schedule_common(t: *mut tasklet_struct, high: bool, nr: c_uint) {
    let flags = lupos_sirq_irq_save();
    let head = lupos_sirq_this_tasklet(high);
    (*t).next = null_mut();
    *(*head).tail = t;
    (*head).tail = addr_of_mut!((*t).next);
    raise_softirq_irqoff(nr);
    lupos_sirq_irq_restore(flags);
}
#[no_mangle]
pub unsafe extern "C" fn __tasklet_schedule(t: *mut tasklet_struct) {
    tasklet_schedule_common(t, false, TASKLET_SOFTIRQ);
}
#[no_mangle]
pub unsafe extern "C" fn __tasklet_hi_schedule(t: *mut tasklet_struct) {
    tasklet_schedule_common(t, true, HI_SOFTIRQ);
}
unsafe fn tasklet_clear_sched(t: *mut tasklet_struct) -> bool {
    if lupos_sirq_clear_wake_bit(TASKLET_STATE_SCHED, addr_of_mut!((*t).state)) {
        return true;
    }
    lupos_sirq_tasklet_warn(t);
    false
}

unsafe fn tasklet_lock_callback() {
    #[cfg(CONFIG_PREEMPT_RT)]
    lupos_sirq_spin_lock(addr_of_mut!((*lupos_sirq_sync_callback()).cb_lock));
}
unsafe fn tasklet_unlock_callback() {
    #[cfg(CONFIG_PREEMPT_RT)]
    lupos_sirq_spin_unlock(addr_of_mut!((*lupos_sirq_sync_callback()).cb_lock));
}
#[cfg(any(CONFIG_SMP, CONFIG_PREEMPT_RT))]
unsafe fn tasklet_callback_cancel_wait_running() {
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        let sync = lupos_sirq_sync_callback();
        lupos_sirq_atomic_inc(addr_of_mut!((*sync).cb_waiters));
        lupos_sirq_spin_lock(addr_of_mut!((*sync).cb_lock));
        lupos_sirq_atomic_dec(addr_of_mut!((*sync).cb_waiters));
        lupos_sirq_spin_unlock(addr_of_mut!((*sync).cb_lock));
    }
}
unsafe fn tasklet_callback_sync_wait_running() {
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        let sync = lupos_sirq_sync_callback();
        if lupos_sirq_atomic_read(addr_of!((*sync).cb_waiters)) != 0 {
            lupos_sirq_spin_unlock(addr_of_mut!((*sync).cb_lock));
            lupos_sirq_spin_lock(addr_of_mut!((*sync).cb_lock));
        }
    }
}
#[inline(always)]
unsafe fn tasklet_trylock(_t: *mut tasklet_struct) -> bool {
    #[cfg(any(CONFIG_SMP, CONFIG_PREEMPT_RT))]
    {
        !lupos_sirq_test_set_bit(TASKLET_STATE_RUN, addr_of_mut!((*_t).state))
    }
    #[cfg(not(any(CONFIG_SMP, CONFIG_PREEMPT_RT)))]
    {
        true
    }
}

unsafe fn tasklet_action_common(head: *mut tasklet_head, nr: c_uint) {
    lupos_sirq_irq_disable();
    let mut list = (*head).head;
    (*head).head = null_mut();
    (*head).tail = addr_of_mut!((*head).head);
    lupos_sirq_irq_enable();
    tasklet_lock_callback();
    while !list.is_null() {
        let t = list;
        list = (*list).next;
        if tasklet_trylock(t) {
            if lupos_sirq_atomic_read(addr_of!((*t).count)) == 0 {
                if tasklet_clear_sched(t) {
                    if (*t).use_callback {
                        lupos_sirq_tasklet_entry(t, callback_address(t));
                        (*t).__bindgen_anon_1.callback.unwrap_unchecked()(t);
                        lupos_sirq_tasklet_exit(t, callback_address(t));
                    } else {
                        lupos_sirq_tasklet_entry(t, func_address(t));
                        (*t).__bindgen_anon_1.func.unwrap_unchecked()((*t).data);
                        lupos_sirq_tasklet_exit(t, func_address(t));
                    }
                }
                tasklet_unlock(t);
                tasklet_callback_sync_wait_running();
                continue;
            }
            tasklet_unlock(t);
        }
        // Busy or disabled tasklets retain SCHED and go to the tail. Save
        // the detached next pointer above, before overwriting it here.
        lupos_sirq_irq_disable();
        (*t).next = null_mut();
        *(*head).tail = t;
        (*head).tail = addr_of_mut!((*t).next);
        __raise_softirq_irqoff(nr);
        lupos_sirq_irq_enable();
    }
    tasklet_unlock_callback();
}
unsafe fn callback_address(t: *mut tasklet_struct) -> *mut c_void {
    match (*t).__bindgen_anon_1.callback {
        Some(callback) => callback as *const () as *mut c_void,
        None => null_mut(),
    }
}
unsafe fn func_address(t: *mut tasklet_struct) -> *mut c_void {
    match (*t).__bindgen_anon_1.func {
        Some(func) => func as *const () as *mut c_void,
        None => null_mut(),
    }
}
unsafe extern "C" fn tasklet_action() {
    workqueue_softirq_action(false);
    tasklet_action_common(lupos_sirq_this_tasklet(false), TASKLET_SOFTIRQ);
}
unsafe extern "C" fn tasklet_hi_action() {
    workqueue_softirq_action(true);
    tasklet_action_common(lupos_sirq_this_tasklet(true), HI_SOFTIRQ);
}
#[no_mangle]
pub unsafe extern "C" fn tasklet_setup(
    t: *mut tasklet_struct,
    callback: Option<unsafe extern "C" fn(*mut tasklet_struct)>,
) {
    (*t).next = null_mut();
    (*t).state = 0;
    lupos_sirq_atomic_set(addr_of_mut!((*t).count), 0);
    (*t).__bindgen_anon_1.callback = callback;
    (*t).use_callback = true;
    (*t).data = 0;
}
#[no_mangle]
pub unsafe extern "C" fn tasklet_init(
    t: *mut tasklet_struct,
    func: Option<unsafe extern "C" fn(c_ulong)>,
    data: c_ulong,
) {
    (*t).next = null_mut();
    (*t).state = 0;
    lupos_sirq_atomic_set(addr_of_mut!((*t).count), 0);
    (*t).__bindgen_anon_1.func = func;
    (*t).use_callback = false;
    (*t).data = data;
}
#[cfg(any(CONFIG_SMP, CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_spin_wait(t: *mut tasklet_struct) {
    while lupos_sirq_test_bit(TASKLET_STATE_RUN, addr_of!((*t).state)) {
        #[cfg(CONFIG_PREEMPT_RT)]
        tasklet_callback_cancel_wait_running();
        #[cfg(not(CONFIG_PREEMPT_RT))]
        lupos_sirq_relax();
    }
}
#[no_mangle]
pub unsafe extern "C" fn tasklet_kill(t: *mut tasklet_struct) {
    if in_interrupt() {
        lupos_sirq_kill_notice();
    }
    lupos_sirq_wait_bit_lock(addr_of_mut!((*t).state), TASKLET_STATE_SCHED);
    tasklet_unlock_wait(t);
    tasklet_clear_sched(t);
}
#[cfg(any(CONFIG_SMP, CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock(t: *mut tasklet_struct) {
    lupos_sirq_clear_and_wake_bit(TASKLET_STATE_RUN, addr_of_mut!((*t).state));
}
#[cfg(any(CONFIG_SMP, CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_wait(t: *mut tasklet_struct) {
    lupos_sirq_wait_bit(addr_of_mut!((*t).state), TASKLET_STATE_RUN);
}
#[cfg(not(any(CONFIG_SMP, CONFIG_PREEMPT_RT)))]
unsafe fn tasklet_unlock(_t: *mut tasklet_struct) {}
#[cfg(not(any(CONFIG_SMP, CONFIG_PREEMPT_RT)))]
unsafe fn tasklet_unlock_wait(_t: *mut tasklet_struct) {}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn softirq_init() {
    let mut cpu = lupos_sirq_next_possible(-1);
    let nr_cpu_ids = lupos_sirq_nr_cpu_ids();
    while (cpu as c_uint) < nr_cpu_ids {
        let low = lupos_sirq_cpu_tasklet(false, cpu as c_uint);
        let high = lupos_sirq_cpu_tasklet(true, cpu as c_uint);
        (*low).tail = addr_of_mut!((*low).head);
        (*high).tail = addr_of_mut!((*high).head);
        cpu = lupos_sirq_next_possible(cpu);
    }
    open_softirq(TASKLET_SOFTIRQ as c_int, Some(tasklet_action));
    open_softirq(HI_SOFTIRQ as c_int, Some(tasklet_hi_action));
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn takeover_one_queue(cpu: c_uint, high: bool, nr: c_uint) {
    let old = lupos_sirq_cpu_tasklet(high, cpu);
    if addr_of_mut!((*old).head) != (*old).tail {
        let local = lupos_sirq_this_tasklet(high);
        *(*local).tail = (*old).head;
        (*local).tail = (*old).tail;
        (*old).head = null_mut();
        (*old).tail = addr_of_mut!((*old).head);
    }
    // This raise is unconditional in the source, including empty queues.
    raise_softirq_irqoff(nr);
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn takeover_tasklets(cpu: c_uint) -> c_int {
    workqueue_softirq_dead(cpu);
    lupos_sirq_irq_disable();
    takeover_one_queue(cpu, false, TASKLET_SOFTIRQ);
    takeover_one_queue(cpu, true, HI_SOFTIRQ);
    lupos_sirq_irq_enable();
    0
}
