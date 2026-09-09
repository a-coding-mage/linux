// SPDX-License-Identifier: GPL-2.0
/*
 * Asynchronous refcounty things
 *
 * Copyright 2010, 2011 Kent Overstreet <kent.overstreet@gmail.com>
 * Copyright 2012 Google, Inc.
 */

// Dependencies supplied by the Linux kernel and other translation units:
// linux/closure.h, linux/debugfs.h, linux/export.h, linux/rcupdate.h,
// linux/seq_file.h, linux/sched/debug.h

#[inline]
unsafe fn closure_put_after_sub_checks(flags: i32) {
    let mut r = flags & CLOSURE_REMAINING_MASK;

    if WARN(flags & CLOSURE_GUARD_MASK != 0,
            "closure has guard bits set: %x (%u)",
            flags & CLOSURE_GUARD_MASK, __fls(r) as u32) {
        r &= !CLOSURE_GUARD_MASK;
    }

    WARN(r == 0 && (flags & !CLOSURE_DESTRUCTOR) != 0,
         "closure ref hit 0 with incorrect flags set: %x (%u)",
         flags & !CLOSURE_DESTRUCTOR, __fls(flags) as u32);
}

#[inline]
unsafe fn closure_put_after_sub(cl: *mut closure, flags: i32) {
    closure_put_after_sub_checks(flags);

    if flags & CLOSURE_REMAINING_MASK == 0 {
        smp_acquire__after_ctrl_dep();
        (*cl).closure_get_happened = false;

        if !(*cl).fn_.is_null() && flags & CLOSURE_DESTRUCTOR == 0 {
            atomic_set(&mut (*cl).remaining, CLOSURE_REMAINING_INITIALIZER);
            closure_queue(cl);
        } else {
            let parent = (*cl).parent;
            let destructor = (*cl).fn_;
            closure_debug_destroy(cl);
            if !destructor.is_null() {
                destructor(&mut (*cl).work);
            }
            if !parent.is_null() {
                closure_put(parent);
            }
        }
    }
}

/* For clearing flags with the same atomic op as a put */
#[no_mangle]
pub unsafe extern "C" fn closure_sub(cl: *mut closure, v: i32) {
    closure_put_after_sub(cl, atomic_sub_return_release(v, &mut (*cl).remaining));
}

/* closure_put - decrement a closure's refcount */
#[no_mangle]
pub unsafe extern "C" fn closure_put(cl: *mut closure) {
    closure_put_after_sub(cl, atomic_dec_return_release(&mut (*cl).remaining));
}

/* closure_wake_up - wake up all closures on a wait list, without memory barrier */
#[no_mangle]
pub unsafe extern "C" fn __closure_wake_up(wait_list: *mut closure_waitlist) {
    let list = llist_del_all(&mut (*wait_list).list);
    let reverse = llist_reverse_order(list);
    let mut cl: *mut closure = core::ptr::null_mut();
    let mut t: *mut closure = core::ptr::null_mut();

    llist_for_each_entry_safe!(cl, t, reverse, list, {
        closure_set_waiting(cl, 0);
        closure_sub(cl, CLOSURE_WAITING + 1);
    });
}

/**
 * closure_wait - add a closure to a waitlist
 * @waitlist: will own a ref on @cl, which will be released when
 * closure_wake_up() is called on @waitlist.
 * @cl: closure pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn closure_wait(waitlist: *mut closure_waitlist,
                                       cl: *mut closure) -> bool {
    if atomic_read(&(*cl).remaining) & CLOSURE_WAITING != 0 {
        return false;
    }
    (*cl).closure_get_happened = true;
    closure_set_waiting(cl, _RET_IP_);
    atomic_add(CLOSURE_WAITING + 1, &mut (*cl).remaining);
    llist_add(&mut (*cl).list, &mut (*waitlist).list);
    true
}

#[repr(C)]
struct closure_syncer {
    task: *mut task_struct,
    done: i32,
}

unsafe extern "C" fn closure_sync_fn(ws: *mut work_struct) {
    let cl = container_of!(ws, closure, work);
    let s = (*cl).s as *mut closure_syncer;
    let p: *mut task_struct;
    rcu_read_lock();
    p = READ_ONCE((*s).task);
    (*s).done = 1;
    wake_up_process(p);
    rcu_read_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn __closure_sync(cl: *mut closure) {
    let mut s = closure_syncer { task: current, done: 0 };
    (*cl).s = &mut s as *mut _ as *mut core::ffi::c_void;
    continue_at(cl, closure_sync_fn, core::ptr::null_mut());
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if s.done != 0 { break; }
        schedule();
    }
    __set_current_state(TASK_RUNNING);
}

#[no_mangle]
pub unsafe extern "C" fn closure_return_sync(cl: *mut closure) {
    let mut s = closure_syncer { task: current, done: 0 };
    (*cl).s = &mut s as *mut _ as *mut core::ffi::c_void;
    set_closure_fn(cl, closure_sync_fn, core::ptr::null_mut());
    let flags = atomic_sub_return_release(1 + CLOSURE_RUNNING - CLOSURE_DESTRUCTOR,
                                          &mut (*cl).remaining);
    closure_put_after_sub_checks(flags);
    if flags & CLOSURE_REMAINING_MASK != 0 {
        loop {
            set_current_state(TASK_UNINTERRUPTIBLE);
            if s.done != 0 { break; }
            schedule();
        }
        __set_current_state(TASK_RUNNING);
    }
    if !(*cl).parent.is_null() { closure_put((*cl).parent); }
}

#[no_mangle]
pub unsafe extern "C" fn __closure_sync_timeout(cl: *mut closure,
                                                  mut timeout: c_ulong) -> i32 {
    let mut s = closure_syncer { task: current, done: 0 };
    let mut ret = 0;
    (*cl).s = &mut s as *mut _ as *mut core::ffi::c_void;
    continue_at(cl, closure_sync_fn, core::ptr::null_mut());
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if s.done != 0 { break; }
        if timeout == 0 {
            let mut old;
            let mut new;
            let mut v = atomic_read(&(*cl).remaining);
            loop {
                old = v;
                if old == 0 || old & CLOSURE_RUNNING != 0 { break; }
                new = old + CLOSURE_REMAINING_INITIALIZER;
                v = atomic_cmpxchg(&mut (*cl).remaining, old, new);
                if v == old { ret = -ETIME; break; }
            }
            if old == 0 || old & CLOSURE_RUNNING != 0 { break; }
        }
        timeout = schedule_timeout(timeout);
    }
    __set_current_state(TASK_RUNNING);
    ret
}

#[cfg(CONFIG_DEBUG_CLOSURES)]
static mut closure_list: list_head = list_head::new();
#[cfg(CONFIG_DEBUG_CLOSURES)]
static mut closure_list_lock: spinlock_t = spinlock_t::new();

#[cfg(CONFIG_DEBUG_CLOSURES)]
#[no_mangle]
pub unsafe extern "C" fn closure_debug_create(cl: *mut closure) {
    BUG_ON((*cl).magic == CLOSURE_MAGIC_ALIVE);
    (*cl).magic = CLOSURE_MAGIC_ALIVE;
    let mut flags = 0;
    spin_lock_irqsave(&mut closure_list_lock, &mut flags);
    list_add(&mut (*cl).all, &mut closure_list);
    spin_unlock_irqrestore(&mut closure_list_lock, flags);
}

#[cfg(CONFIG_DEBUG_CLOSURES)]
#[no_mangle]
pub unsafe extern "C" fn closure_debug_destroy(cl: *mut closure) {
    if (*cl).magic == CLOSURE_MAGIC_STACK { return; }
    BUG_ON((*cl).magic != CLOSURE_MAGIC_ALIVE);
    (*cl).magic = CLOSURE_MAGIC_DEAD;
    let mut flags = 0;
    spin_lock_irqsave(&mut closure_list_lock, &mut flags);
    list_del(&mut (*cl).all);
    spin_unlock_irqrestore(&mut closure_list_lock, flags);
}

#[cfg(not(CONFIG_DEBUG_CLOSURES))]
#[inline]
unsafe fn closure_debug_create(_cl: *mut closure) {}
#[cfg(not(CONFIG_DEBUG_CLOSURES))]
#[inline]
unsafe fn closure_debug_destroy(_cl: *mut closure) {}

#[cfg(CONFIG_DEBUG_CLOSURES)]
unsafe extern "C" fn debug_show(f: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 {
    spin_lock_irq(&mut closure_list_lock);
    list_for_each_entry!(cl, closure_list, all, {
        let r = atomic_read(&(*cl).remaining);
        seq_printf(f, "%p: %pS -> %pS p %p r %i ",
                   cl, (*cl).ip as *mut core::ffi::c_void, (*cl).fn_,
                   (*cl).parent, r & CLOSURE_REMAINING_MASK);
        seq_printf(f, "%s%s\n",
                   if test_bit(WORK_STRUCT_PENDING_BIT,
                               work_data_bits(&mut (*cl).work)) { "Q" } else { "" },
                   if r & CLOSURE_RUNNING != 0 { "R" } else { "" });
        if r & CLOSURE_WAITING != 0 {
            seq_printf(f, " W %pS\n", (*cl).waiting_on as *mut core::ffi::c_void);
        }
        seq_putc(f, '\n');
    });
    spin_unlock_irq(&mut closure_list_lock);
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(debug).
#[cfg(CONFIG_DEBUG_CLOSURES)]
static debug_fops: file_operations = DEFINE_SHOW_ATTRIBUTE!(debug);

#[cfg(CONFIG_DEBUG_CLOSURES)]
unsafe extern "C" fn closure_debug_init() -> i32 {
    debugfs_create_file("closures", 0o400, core::ptr::null_mut(),
                        core::ptr::null_mut(), &debug_fops);
    0
}

// Equivalent of late_initcall(closure_debug_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
