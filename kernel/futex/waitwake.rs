// SPDX-License-Identifier: GPL-2.0-or-later

/* Kernel futex wait/wake implementation translated directly from C.
 * Required kernel types, constants, macros, and functions are supplied by
 * the surrounding translation unit.
 */

pub unsafe fn __futex_wake_mark(q: *mut futex_q) -> bool {
    if WARN((*q).pi_state || (*q).rt_waiter, "refusing to wake PI futex\n") {
        return false;
    }

    __futex_unqueue(q);
    /* The waiting task can free futex_q as soon as lock_ptr is NULL. */
    smp_store_release(&mut (*q).lock_ptr, core::ptr::null_mut());
    true
}

pub unsafe fn futex_wake_mark(wake_q: *mut wake_q_head, q: *mut futex_q) {
    let p = (*q).task;
    get_task_struct(p);
    if !__futex_wake_mark(q) {
        put_task_struct(p);
        return;
    }
    wake_q_add_safe(wake_q, p);
}

unsafe fn futex_robust_unlock(uaddr: *mut u32, flags: u32, pop: *mut core::ffi::c_void) -> bool {
    if flags & FLAGS_ROBUST_UNLOCK == 0 {
        return true;
    }
    unsafe_atomic_store_release_user(0, uaddr);
    futex_robust_list_clear_pending(pop, flags)
}

pub unsafe fn futex_wake(uaddr: *mut u32, flags: u32, pop: *mut core::ffi::c_void,
                         nr_wake: i32, bitset: u32) -> i32 {
    let mut key = FUTEX_KEY_INIT;
    let mut ret: i32;
    let mut wake_q = DEFINE_WAKE_Q!();
    if bitset == 0 { return -EINVAL; }
    ret = get_futex_key(uaddr, flags, &mut key, FUTEX_READ);
    if unlikely(ret != 0) { return ret; }
    if !futex_robust_unlock(uaddr, flags, pop) { return -EFAULT; }
    if flags & FLAGS_STRICT != 0 && nr_wake == 0 { return 0; }
    let hb = Hbr::new(&key).hb;
    if !futex_hb_waiters_pending(hb) { return ret; }
    spin_lock(&mut (*hb).lock);
    plist_for_each_entry_safe!(this, next, &mut (*hb).chain, list, {
        if futex_match(&(*this).key, &key) {
            if (*this).pi_state || (*this).rt_waiter { ret = -EINVAL; break; }
            if (*this).bitset & bitset == 0 { continue; }
            ((*this).wake)(&mut wake_q, this);
            ret += 1;
            if ret >= nr_wake { break; }
        }
    });
    spin_unlock(&mut (*hb).lock);
    wake_up_q(&mut wake_q);
    ret
}

unsafe fn futex_atomic_op_inuser(encoded_op: u32, uaddr: *mut u32) -> i32 {
    let op = (encoded_op & 0x70000000) >> 28;
    let cmp = (encoded_op & 0x0f000000) >> 24;
    let mut oparg = sign_extend32((encoded_op & 0x00fff000) >> 12, 11);
    let cmparg = sign_extend32(encoded_op & 0x00000fff, 11);
    let mut oldval = 0i32;
    if encoded_op & (FUTEX_OP_OPARG_SHIFT << 28) != 0 {
        if oparg < 0 || oparg > 31 { oparg &= 31; }
        oparg = 1 << oparg;
    }
    pagefault_disable();
    let ret = arch_futex_atomic_op_inuser(op, oparg, &mut oldval, uaddr);
    pagefault_enable();
    if ret != 0 { return ret; }
    match cmp {
        FUTEX_OP_CMP_EQ => (oldval == cmparg) as i32,
        FUTEX_OP_CMP_NE => (oldval != cmparg) as i32,
        FUTEX_OP_CMP_LT => (oldval < cmparg) as i32,
        FUTEX_OP_CMP_GE => (oldval >= cmparg) as i32,
        FUTEX_OP_CMP_LE => (oldval <= cmparg) as i32,
        FUTEX_OP_CMP_GT => (oldval > cmparg) as i32,
        _ => -ENOSYS,
    }
}

pub unsafe fn futex_wake_op(uaddr1: *mut u32, flags: u32, uaddr2: *mut u32,
                            nr_wake: i32, nr_wake2: i32, op: i32) -> i32 {
    let mut key1 = FUTEX_KEY_INIT;
    let mut key2 = FUTEX_KEY_INIT;
    let mut ret = 0i32;
    let mut wake_q = DEFINE_WAKE_Q!();
    'retry: loop {
        ret = get_futex_key(uaddr1, flags, &mut key1, FUTEX_READ); if ret != 0 { return ret; }
        ret = get_futex_key(uaddr2, flags, &mut key2, FUTEX_WRITE); if ret != 0 { return ret; }
        'retry_private: loop {
            let hb1 = Hbr::new(&key1).hb; let hb2 = Hbr::new(&key2).hb;
            double_lock_hb(hb1, hb2);
            let mut op_ret = futex_atomic_op_inuser(op as u32, uaddr2);
            if op_ret < 0 {
                double_unlock_hb(hb1, hb2);
                if op_ret != -EFAULT && op_ret != -EAGAIN { return op_ret; }
                if op_ret == -EFAULT { ret = fault_in_user_writeable(uaddr2); if ret != 0 { return ret; } }
                cond_resched();
                if flags & FLAGS_SHARED == 0 { continue 'retry_private; }
                continue 'retry;
            }
            plist_for_each_entry_safe!(this, next, &mut (*hb1).chain, list, {
                if futex_match(&(*this).key, &key1) {
                    if (*this).pi_state || (*this).rt_waiter { ret = -EINVAL; break; }
                    ((*this).wake)(&mut wake_q, this); ret += 1; if ret >= nr_wake { break; }
                }
            });
            if op_ret > 0 {
                op_ret = 0;
                plist_for_each_entry_safe!(this, next, &mut (*hb2).chain, list, {
                    if futex_match(&(*this).key, &key2) {
                        if (*this).pi_state || (*this).rt_waiter { ret = -EINVAL; break; }
                        ((*this).wake)(&mut wake_q, this); op_ret += 1; if op_ret >= nr_wake2 { break; }
                    }
                });
                ret += op_ret;
            }
            double_unlock_hb(hb1, hb2); break;
        }
        wake_up_q(&mut wake_q); return ret;
    }
}

unsafe fn futex_wait_restart(restart: *mut restart_block) -> isize;

pub unsafe fn futex_do_wait(q: *mut futex_q, timeout: *mut hrtimer_sleeper) {
    if !timeout.is_null() { hrtimer_sleeper_start_expires(timeout, HRTIMER_MODE_ABS); }
    if !plist_node_empty(&(*q).list) && (timeout.is_null() || !(*timeout).task.is_null()) { schedule(); }
    __set_current_state(TASK_RUNNING);
}

pub unsafe fn futex_unqueue_multiple(v: *mut futex_vector, count: i32) -> i32 {
    let mut ret = -1; for i in 0..count { if !futex_unqueue(&mut (*v.add(i as usize)).q) { ret = i; } } ret
}

pub unsafe fn futex_wait_multiple_setup(vs: *mut futex_vector, count: i32, woken: *mut i32) -> i32 {
    let mut retry = false; let mut i = 0; let mut uval = 0u32;
    guard_private_hash!(current_mm());
    'retry: loop {
        for i in 0..count { let v = &mut *vs.add(i as usize); if v.w.flags & FLAGS_SHARED == 0 && retry { continue; } let r = get_futex_key(u64_to_user_ptr(v.w.uaddr), v.w.flags, &mut v.q.key, FUTEX_READ); if r != 0 { return r; } }
        set_current_state(TASK_INTERRUPTIBLE | TASK_FREEZABLE);
        for j in 0..count {
            i = j; let v = &mut *vs.add(j as usize); let uaddr = v.w.uaddr as *mut u32; let q = &mut v.q;
            let hb = Hbr::new(&q.key).hb; futex_q_lock(q, hb); let ret = futex_get_value_locked(&mut uval, uaddr);
            if ret == 0 && uval == v.w.val { futex_queue(q, hb, current); continue; }
            futex_q_unlock(hb); __release(q.lock_ptr);
            __set_current_state(TASK_RUNNING); *woken = futex_unqueue_multiple(vs, i); if *woken >= 0 { return 1; }
            if ret != 0 { if get_user(&mut uval, uaddr) != 0 { return -EFAULT; } retry = true; continue 'retry; }
            if uval != v.w.val { return -EWOULDBLOCK; }
        }
        return 0;
    }
}

unsafe fn futex_sleep_multiple(vs: *mut futex_vector, count: u32, to: *mut hrtimer_sleeper) {
    if !to.is_null() && (*to).task.is_null() { return; }
    for i in 0..count { if READ_ONCE((*vs.add(i as usize)).q.lock_ptr).is_null() { return; } }
    schedule();
}

pub unsafe fn futex_wait_multiple(vs: *mut futex_vector, count: u32, to: *mut hrtimer_sleeper) -> i32 {
    let mut hint = 0; if !to.is_null() { hrtimer_sleeper_start_expires(to, HRTIMER_MODE_ABS); }
    loop { let mut ret = futex_wait_multiple_setup(vs, count as i32, &mut hint); if ret != 0 { return if ret > 0 { hint } else { ret }; }
        futex_sleep_multiple(vs, count, to); __set_current_state(TASK_RUNNING); ret = futex_unqueue_multiple(vs, count as i32); if ret >= 0 { return ret; }
        if !to.is_null() && (*to).task.is_null() { return -ETIMEDOUT; } else if signal_pending(current) { return -ERESTARTSYS; }
    }
}

pub unsafe fn futex_wait_setup(uaddr: *mut u32, val: u32, flags: u32, q: *mut futex_q,
                               key2: *mut futex_key, task: *mut task_struct) -> i32 {
    let mut uval = 0; 'retry: loop {
        let mut ret = get_futex_key(uaddr, flags, &mut (*q).key, FUTEX_READ); if ret != 0 { return ret; }
        'private: loop { let hb = Hbr::new(&(*q).key).hb; futex_q_lock(q, hb); ret = futex_get_value_locked(&mut uval, uaddr);
            if ret != 0 { futex_q_unlock(hb); __release((*q).lock_ptr); ret = get_user(&mut uval, uaddr); if ret != 0 { return ret; } if flags & FLAGS_SHARED == 0 { continue 'private; } continue 'retry; }
            if uval != val { futex_q_unlock(hb); __release((*q).lock_ptr); return -EWOULDBLOCK; }
            if !key2.is_null() && futex_match(&(*q).key, key2) { futex_q_unlock(hb); __release((*q).lock_ptr); return -EINVAL; }
            if task == current { set_current_state(TASK_INTERRUPTIBLE | TASK_FREEZABLE); } futex_queue(q, hb, task); return ret;
        }
    }
}

pub unsafe fn __futex_wait(uaddr: *mut u32, flags: u32, val: u32, to: *mut hrtimer_sleeper, bitset: u32) -> i32 {
    let mut q = futex_q_init; if bitset == 0 { return -EINVAL; } q.bitset = bitset;
    loop { let ret = futex_wait_setup(uaddr, val, flags, &mut q, core::ptr::null_mut(), current); if ret != 0 { return ret; }
        futex_do_wait(&mut q, to); if !futex_unqueue(&mut q) { return 0; } if !to.is_null() && (*to).task.is_null() { return -ETIMEDOUT; } if !signal_pending(current) { continue; } return -ERESTARTSYS;
    }
}

pub unsafe fn futex_wait(uaddr: *mut u32, flags: u32, val: u32, abs_time: *mut ktime_t, bitset: u32) -> i32 {
    let mut timeout = core::mem::MaybeUninit::<hrtimer_sleeper>::uninit(); let to = futex_setup_timer(abs_time, timeout.as_mut_ptr(), flags, (*current).timer_slack_ns); let ret = __futex_wait(uaddr, flags, val, to, bitset); if to.is_null() { return ret; }
    hrtimer_cancel(&mut (*to).timer); destroy_hrtimer_on_stack(&mut (*to).timer); if ret == -ERESTARTSYS { let restart = &mut (*current).restart_block; restart.futex.uaddr = uaddr; restart.futex.val = val; restart.futex.time = *abs_time; restart.futex.bitset = bitset; restart.futex.flags = flags | FLAGS_HAS_TIMEOUT; return set_restart_fn(restart, futex_wait_restart); } ret
}

unsafe fn futex_wait_restart(restart: *mut restart_block) -> isize {
    let uaddr = (*restart).futex.uaddr; let mut tp = core::ptr::null_mut(); if (*restart).futex.flags & FLAGS_HAS_TIMEOUT != 0 { tp = &mut (*restart).futex.time; }
    (*restart).fn_ = do_no_restart_syscall; futex_wait(uaddr, (*restart).futex.flags, (*restart).futex.val, tp, (*restart).futex.bitset) as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
