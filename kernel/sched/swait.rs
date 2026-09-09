// SPDX-License-Identifier: GPL-2.0
/*
 * <linux/swait.h> (simple wait queues ) implementation:
 */

use core::ffi::{c_char, c_int, c_long, c_ulong};

extern "C" {
    fn raw_spin_lock_init(lock: *mut raw_spinlock_t);
    fn lockdep_set_class_and_name(
        lock: *mut raw_spinlock_t,
        key: *mut lock_class_key,
        name: *const c_char,
    );
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(list: *const list_head) -> bool;
    fn list_empty_careful(list: *const list_head) -> bool;
    fn list_first_entry(ptr: *mut list_head, type_size: usize, member_offset: usize) -> *mut swait_queue;
    fn try_to_wake_up(task: *mut task_struct, state: c_ulong, wake_flags: c_int) -> c_int;
    fn wake_up_state(task: *mut task_struct, state: c_ulong) -> c_int;
    fn list_del_init(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn raw_spin_lock_irq(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock_irq(lock: *mut raw_spinlock_t);
    fn set_current_state(state: c_int);
    fn __set_current_state(state: c_int);
    fn signal_pending_state(state: c_int, task: *mut task_struct) -> bool;
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct swait_queue_head { pub lock: raw_spinlock_t, pub task_list: list_head }
#[repr(C)]
pub struct swait_queue { pub task: *mut task_struct, pub task_list: list_head }

const TASK_NORMAL: c_ulong = 0;
const TASK_RUNNING: c_int = 0;
const ERESTARTSYS: c_long = 512;

pub unsafe fn __init_swait_queue_head(q: *mut swait_queue_head, name: *const c_char, key: *mut lock_class_key) {
    raw_spin_lock_init(&mut (*q).lock);
    lockdep_set_class_and_name(&mut (*q).lock, key, name);
    INIT_LIST_HEAD(&mut (*q).task_list);
}

pub unsafe fn swake_up_locked(q: *mut swait_queue_head, wake_flags: c_int) {
    if list_empty(&(*q).task_list) { return; }
    let curr = list_first_entry(&mut (*q).task_list, core::mem::size_of::<swait_queue>(), 0);
    try_to_wake_up((*curr).task, TASK_NORMAL, wake_flags);
    list_del_init(&mut (*curr).task_list);
}

pub unsafe fn swake_up_all_locked(q: *mut swait_queue_head) {
    while !list_empty(&(*q).task_list) { swake_up_locked(q, 0); }
}

pub unsafe fn swake_up_one(q: *mut swait_queue_head) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*q).lock, &mut flags);
    swake_up_locked(q, 0);
    raw_spin_unlock_irqrestore(&mut (*q).lock, flags);
}

pub unsafe fn swake_up_all(q: *mut swait_queue_head) {
    let mut tmp = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
    raw_spin_lock_irq(&mut (*q).lock);
    list_splice_init(&mut (*q).task_list, &mut tmp);
    while !list_empty(&tmp) {
        let curr = list_first_entry(&mut tmp, core::mem::size_of::<swait_queue>(), 0);
        wake_up_state((*curr).task, TASK_NORMAL);
        list_del_init(&mut (*curr).task_list);
        if list_empty(&tmp) { break; }
        raw_spin_unlock_irq(&mut (*q).lock);
        raw_spin_lock_irq(&mut (*q).lock);
    }
    raw_spin_unlock_irq(&mut (*q).lock);
}

pub unsafe fn __prepare_to_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    (*wait).task = current;
    if list_empty(&(*wait).task_list) { list_add_tail(&mut (*wait).task_list, &mut (*q).task_list); }
}

pub unsafe fn prepare_to_swait_exclusive(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*q).lock, &mut flags);
    __prepare_to_swait(q, wait);
    set_current_state(state);
    raw_spin_unlock_irqrestore(&mut (*q).lock, flags);
}

pub unsafe fn prepare_to_swait_event(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) -> c_long {
    let mut flags = 0;
    let mut ret: c_long = 0;
    raw_spin_lock_irqsave(&mut (*q).lock, &mut flags);
    if signal_pending_state(state, current) {
        list_del_init(&mut (*wait).task_list);
        ret = -ERESTARTSYS;
    } else {
        __prepare_to_swait(q, wait);
        set_current_state(state);
    }
    raw_spin_unlock_irqrestore(&mut (*q).lock, flags);
    ret
}

pub unsafe fn __finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    __set_current_state(TASK_RUNNING);
    if !list_empty(&(*wait).task_list) { list_del_init(&mut (*wait).task_list); }
}

pub unsafe fn finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    let mut flags = 0;
    __set_current_state(TASK_RUNNING);
    if !list_empty_careful(&(*wait).task_list) {
        raw_spin_lock_irqsave(&mut (*q).lock, &mut flags);
        list_del_init(&mut (*wait).task_list);
        raw_spin_unlock_irqrestore(&mut (*q).lock, flags);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
