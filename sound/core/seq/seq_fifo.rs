// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer FIFO
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* Dependencies in the C source:
 * <sound/core.h>, <linux/slab.h>, <linux/sched/signal.h>,
 * "seq_fifo.h", and "seq_lock.h".
 */

use core::ptr;

pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const EAGAIN: i32 = 11;
pub const ERESTARTSYS: i32 = 512;
pub const TASK_INTERRUPTIBLE: i64 = 1;

#[repr(C)]
pub struct snd_seq_fifo {
    pub pool: *mut snd_seq_pool,
    pub lock: spinlock_t,
    pub use_lock: snd_use_lock_t,
    pub input_sleep: wait_queue_head_t,
    pub overflow: atomic_t,
    pub head: *mut snd_seq_event_cell,
    pub tail: *mut snd_seq_event_cell,
    pub cells: i32,
}

#[repr(C)]
pub struct snd_seq_event_cell {
    pub next: *mut snd_seq_event_cell,
    pub pool: *mut snd_seq_pool,
}

#[repr(C)]
pub struct snd_seq_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_use_lock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;

    fn kzalloc_obj_snd_seq_fifo() -> *mut snd_seq_fifo;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn snd_seq_pool_new(poolsize: i32) -> *mut snd_seq_pool;
    fn snd_seq_pool_init(pool: *mut snd_seq_pool) -> i32;
    fn snd_seq_pool_delete(pool: *mut *mut snd_seq_pool);
    fn snd_seq_pool_done(pool: *mut snd_seq_pool);
    fn snd_seq_pool_mark_closing(pool: *mut snd_seq_pool);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: core::ffi::c_ulong);
    fn snd_use_lock_init(lock: *mut snd_use_lock_t);
    fn snd_use_lock_sync(lock: *mut snd_use_lock_t);
    fn snd_seq_fifo_use_lock(f: *mut snd_seq_fifo);
    fn snd_seq_fifo_use_unlock(f: *mut snd_seq_fifo);
    fn init_waitqueue_head(queue: *mut wait_queue_head_t);
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut task_struct);
    fn add_wait_queue(queue: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn remove_wait_queue(queue: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn waitqueue_active(queue: *mut wait_queue_head_t) -> bool;
    fn wake_up(queue: *mut wait_queue_head_t);
    fn poll_wait(file: *mut file, queue: *mut wait_queue_head_t, wait: *mut poll_table);
    fn atomic_set(v: *mut atomic_t, i: i32);
    fn atomic_inc(v: *mut atomic_t);
    fn snd_seq_event_dup(
        pool: *mut snd_seq_pool,
        event: *mut snd_seq_event,
        cellp: *mut *mut snd_seq_event_cell,
        nonblock: i32,
        file: *mut core::ffi::c_void,
        tmppool: *mut core::ffi::c_void,
    ) -> i32;
    fn snd_seq_cell_free(cell: *mut snd_seq_event_cell);
    fn snd_seq_unused_cells(pool: *mut snd_seq_pool) -> i32;
    fn set_current_state(state: i64);
    fn schedule();
    fn signal_pending(task: *mut task_struct) -> i32;
    fn snd_BUG_ON(condition: bool) -> bool;
}

unsafe fn read_once_pool(ptr: *mut *mut snd_seq_pool) -> *mut snd_seq_pool {
    ptr::read_volatile(ptr)
}

unsafe fn write_once_pool(ptr: *mut *mut snd_seq_pool, value: *mut snd_seq_pool) {
    ptr::write_volatile(ptr, value);
}

/* FIFO */

/* create new fifo */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_new(poolsize: i32) -> *mut snd_seq_fifo {
    let f: *mut snd_seq_fifo;

    f = kzalloc_obj_snd_seq_fifo();
    if f.is_null() {
        return ptr::null_mut();
    }

    (*f).pool = snd_seq_pool_new(poolsize);
    if (*f).pool.is_null() {
        kfree(f as *mut core::ffi::c_void);
        return ptr::null_mut();
    }
    if snd_seq_pool_init((*f).pool) < 0 {
        snd_seq_pool_delete(&mut (*f).pool);
        kfree(f as *mut core::ffi::c_void);
        return ptr::null_mut();
    }

    spin_lock_init(&mut (*f).lock);
    snd_use_lock_init(&mut (*f).use_lock);
    init_waitqueue_head(&mut (*f).input_sleep);
    atomic_set(&mut (*f).overflow, 0);

    (*f).head = ptr::null_mut();
    (*f).tail = ptr::null_mut();
    (*f).cells = 0;

    f
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_delete(fifo: *mut *mut snd_seq_fifo) {
    let f: *mut snd_seq_fifo;

    if snd_BUG_ON(fifo.is_null()) {
        return;
    }
    f = *fifo;
    if snd_BUG_ON(f.is_null()) {
        return;
    }
    *fifo = ptr::null_mut();

    if !(*f).pool.is_null() {
        snd_seq_pool_mark_closing((*f).pool);
    }

    snd_seq_fifo_clear(f);

    /* wake up clients if any */
    if waitqueue_active(&mut (*f).input_sleep) {
        wake_up(&mut (*f).input_sleep);
    }

    /* release resources...*/
    /*....................*/

    if !(*f).pool.is_null() {
        snd_seq_pool_done((*f).pool);
        snd_seq_pool_delete(&mut (*f).pool);
    }

    kfree(f as *mut core::ffi::c_void);
}

/* clear queue */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_clear(f: *mut snd_seq_fifo) {
    let mut cell: *mut snd_seq_event_cell;

    /* clear overflow flag */
    atomic_set(&mut (*f).overflow, 0);

    snd_use_lock_sync(&mut (*f).use_lock);
    spin_lock_irq(&mut (*f).lock);
    /* drain the fifo */
    loop {
        cell = fifo_cell_out(f);
        if cell.is_null() {
            break;
        }
        snd_seq_cell_free(cell);
    }
    spin_unlock_irq(&mut (*f).lock);
}

/* enqueue event to fifo */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_event_in(
    f: *mut snd_seq_fifo,
    event: *mut snd_seq_event,
) -> i32 {
    let mut cell: *mut snd_seq_event_cell = ptr::null_mut();
    let mut pool: *mut snd_seq_pool;
    let mut linked: bool;
    let mut err: i32;

    if snd_BUG_ON(f.is_null()) {
        return -EINVAL;
    }

    snd_seq_fifo_use_lock(f);
    loop {
        pool = read_once_pool(&mut (*f).pool);
        err = snd_seq_event_dup(
            pool,
            event,
            &mut cell,
            1,
            ptr::null_mut(),
            ptr::null_mut(),
        ); /* always non-blocking */
        if err < 0 {
            if err == -ENOMEM || err == -EAGAIN {
                atomic_inc(&mut (*f).overflow);
            }
            snd_seq_fifo_use_unlock(f);
            return err;
        }

        /* append new cells to fifo */
        linked = false;
        let mut flags: core::ffi::c_ulong = 0;
        spin_lock_irqsave(&mut (*f).lock, &mut flags);
        if (*cell).pool == (*f).pool {
            if !(*f).tail.is_null() {
                (*(*f).tail).next = cell;
            }
            (*f).tail = cell;
            if (*f).head.is_null() {
                (*f).head = cell;
            }
            (*cell).next = ptr::null_mut();
            (*f).cells += 1;
            linked = true;
        }
        spin_unlock_irqrestore(&mut (*f).lock, flags);

        if linked {
            break;
        }

        /* Retry against the replacement pool after resize publishes it. */
        snd_seq_cell_free(cell);
    }

    snd_seq_fifo_use_unlock(f);

    /* wakeup client */
    if waitqueue_active(&mut (*f).input_sleep) {
        wake_up(&mut (*f).input_sleep);
    }

    0 /* success */
}

/* dequeue cell from fifo */
unsafe fn fifo_cell_out(f: *mut snd_seq_fifo) -> *mut snd_seq_event_cell {
    let cell: *mut snd_seq_event_cell;

    cell = (*f).head;
    if !cell.is_null() {
        (*f).head = (*cell).next;

        /* reset tail if this was the last element */
        if (*f).tail == cell {
            (*f).tail = ptr::null_mut();
        }

        (*cell).next = ptr::null_mut();
        (*f).cells -= 1;
    }

    cell
}

/* dequeue cell from fifo and copy on user space */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_cell_out(
    f: *mut snd_seq_fifo,
    cellp: *mut *mut snd_seq_event_cell,
    nonblock: i32,
) -> i32 {
    let mut cell: *mut snd_seq_event_cell;
    let mut flags: core::ffi::c_ulong = 0;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();

    if snd_BUG_ON(f.is_null()) {
        return -EINVAL;
    }

    *cellp = ptr::null_mut();
    init_waitqueue_entry(&mut wait, current);
    spin_lock_irqsave(&mut (*f).lock, &mut flags);
    loop {
        cell = fifo_cell_out(f);
        if !cell.is_null() {
            break;
        }
        if nonblock != 0 {
            /* non-blocking - return immediately */
            spin_unlock_irqrestore(&mut (*f).lock, flags);
            return -EAGAIN;
        }
        set_current_state(TASK_INTERRUPTIBLE);
        add_wait_queue(&mut (*f).input_sleep, &mut wait);
        spin_unlock_irqrestore(&mut (*f).lock, flags);
        schedule();
        spin_lock_irqsave(&mut (*f).lock, &mut flags);
        remove_wait_queue(&mut (*f).input_sleep, &mut wait);
        if signal_pending(current) != 0 {
            spin_unlock_irqrestore(&mut (*f).lock, flags);
            return -ERESTARTSYS;
        }
    }
    spin_unlock_irqrestore(&mut (*f).lock, flags);
    *cellp = cell;

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_cell_putback(
    f: *mut snd_seq_fifo,
    cell: *mut snd_seq_event_cell,
) {
    let mut linked = false;

    if !cell.is_null() {
        let mut flags: core::ffi::c_ulong = 0;
        spin_lock_irqsave(&mut (*f).lock, &mut flags);
        if (*cell).pool == (*f).pool {
            (*cell).next = (*f).head;
            (*f).head = cell;
            if (*f).tail.is_null() {
                (*f).tail = cell;
            }
            (*f).cells += 1;
            linked = true;
        }
        spin_unlock_irqrestore(&mut (*f).lock, flags);
        if !linked {
            snd_seq_cell_free(cell);
        }
    }
}

/* polling; return non-zero if queue is available */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_poll_wait(
    f: *mut snd_seq_fifo,
    file: *mut file,
    wait: *mut poll_table,
) -> i32 {
    let ret: i32;

    poll_wait(file, &mut (*f).input_sleep, wait);
    spin_lock_irq(&mut (*f).lock);
    ret = ((*f).cells > 0) as i32;
    spin_unlock_irq(&mut (*f).lock);
    ret
}

/* change the size of pool; all old events are removed */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_resize(f: *mut snd_seq_fifo, poolsize: i32) -> i32 {
    let mut newpool: *mut snd_seq_pool;
    let mut oldpool: *mut snd_seq_pool = ptr::null_mut();
    let mut cell: *mut snd_seq_event_cell;
    let mut next: *mut snd_seq_event_cell;
    let mut oldhead: *mut snd_seq_event_cell = ptr::null_mut();

    if snd_BUG_ON(f.is_null() || (*f).pool.is_null()) {
        return -EINVAL;
    }

    /* allocate new pool */
    newpool = snd_seq_pool_new(poolsize);
    if newpool.is_null() {
        return -ENOMEM;
    }
    if snd_seq_pool_init(newpool) < 0 {
        snd_seq_pool_delete(&mut newpool);
        return -ENOMEM;
    }

    spin_lock_irq(&mut (*f).lock);
    /* remember old pool */
    oldpool = (*f).pool;
    oldhead = (*f).head;
    /* exchange pools */
    write_once_pool(&mut (*f).pool, newpool);
    (*f).head = ptr::null_mut();
    (*f).tail = ptr::null_mut();
    (*f).cells = 0;
    /* NOTE: overflow flag is not cleared */
    spin_unlock_irq(&mut (*f).lock);

    /* close the old pool and wait until all users are gone */
    snd_seq_pool_mark_closing(oldpool);
    snd_use_lock_sync(&mut (*f).use_lock);

    /* release cells in old pool */
    cell = oldhead;
    while !cell.is_null() {
        next = (*cell).next;
        snd_seq_cell_free(cell);
        cell = next;
    }
    snd_seq_pool_delete(&mut oldpool);

    0
}

/* get the number of unused cells safely */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_fifo_unused_cells(f: *mut snd_seq_fifo) -> i32 {
    let ret: i32;

    if f.is_null() {
        return 0;
    }

    snd_seq_fifo_use_lock(f);
    let mut flags: core::ffi::c_ulong = 0;
    spin_lock_irqsave(&mut (*f).lock, &mut flags);
    ret = snd_seq_unused_cells((*f).pool);
    spin_unlock_irqrestore(&mut (*f).lock, flags);
    snd_seq_fifo_use_unlock(f);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
