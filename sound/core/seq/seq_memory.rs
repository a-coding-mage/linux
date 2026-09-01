// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  ALSA sequencer Memory Manager
 *  Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 *                        Jaroslav Kysela <perex@perex.cz>
 *                2000 by Takashi Iwai <tiwai@suse.de>
 */

// Depends on Linux kernel and ALSA sequencer definitions from:
// <linux/init.h>, <linux/export.h>, <linux/slab.h>,
// <linux/sched/signal.h>, <linux/mm.h>, <sound/core.h>,
// <sound/seq_kernel.h>, "seq_memory.h", "seq_queue.h",
// "seq_info.h", and "seq_lock.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub const EINVAL: c_int = 22;
pub const EFAULT: c_int = 14;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const ERESTARTSYS: c_int = 512;

pub const TASK_INTERRUPTIBLE: c_long = 1;

type c_long = i64;
type bool_t = bool;
pub type snd_seq_dump_func_t =
    Option<unsafe extern "C" fn(private_data: *mut c_void, buf: *mut c_void, size: c_int) -> c_int>;

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
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
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_ext {
    pub len: c_uint,
    pub ptr: *mut c_char,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub ext: snd_seq_ext,
}

#[repr(C)]
pub struct snd_seq_event {
    pub flags: c_uint,
    pub data: snd_seq_event_data,
}

#[repr(C)]
pub struct snd_seq_ump_event_raw {
    pub extra: u64,
}

#[repr(C)]
pub union snd_seq_ump_event {
    pub event: snd_seq_event,
    pub raw: snd_seq_ump_event_raw,
}

#[repr(C)]
pub struct snd_seq_event_cell {
    pub next: *mut snd_seq_event_cell,
    pub pool: *mut snd_seq_pool,
    pub ump: snd_seq_ump_event,
}

#[repr(C)]
pub struct snd_seq_pool {
    pub lock: spinlock_t,
    pub ptr: *mut snd_seq_event_cell,
    pub free: *mut snd_seq_event_cell,
    pub total_elements: c_int,
    pub counter: atomic_t,
    pub closing: c_int,
    pub output_sleep: wait_queue_head_t,
    pub size: c_int,
    pub room: c_int,
    pub max_used: c_int,
    pub event_alloc_success: c_int,
    pub event_alloc_failures: c_int,
}

pub type c_uint = u32;

impl snd_seq_event_cell {
    unsafe fn event_mut(&mut self) -> *mut snd_seq_event {
        &mut self.ump.event
    }

    unsafe fn event_ref(&self) -> *const snd_seq_event {
        &self.ump.event
    }
}

extern "C" {
    static mut current: *mut task_struct;

    static SNDRV_SEQ_EVENT_LENGTH_MASK: c_uint;
    static SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uint;
    static SNDRV_SEQ_EXT_MASK: c_uint;
    static SNDRV_SEQ_EXT_USRPTR: c_uint;
    static SNDRV_SEQ_EXT_CHAINED: c_uint;

    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn clear_user(to: *mut c_void, n: usize) -> c_ulong;
    fn memcpy(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut task_struct);
    fn add_wait_queue(wq_head: *mut wait_queue_head_t, wq_entry: *mut wait_queue_entry_t);
    fn remove_wait_queue(wq_head: *mut wait_queue_head_t, wq_entry: *mut wait_queue_entry_t);
    fn waitqueue_active(wq_head: *mut wait_queue_head_t) -> c_int;
    fn wake_up(wq_head: *mut wait_queue_head_t);
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn set_current_state(state: c_long);
    fn schedule();
    fn schedule_timeout_uninterruptible(timeout: c_long) -> c_long;
    fn signal_pending(task: *mut task_struct) -> c_int;

    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);

    fn snd_BUG_ON(cond: bool_t) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn poll_wait(file: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    fn snd_seq_ev_is_variable(event: *const snd_seq_event) -> c_int;
    fn snd_seq_event_packet_size(event: *const snd_seq_event) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);

    fn kvmalloc_objs_snd_seq_event_cell(n: c_int) -> *mut snd_seq_event_cell;
    fn kvfree(ptr: *mut c_void);
    fn kzalloc_obj_snd_seq_pool() -> *mut snd_seq_pool;
    fn kfree(ptr: *mut c_void);
}

#[inline]
unsafe fn snd_seq_pool_available(pool: *mut snd_seq_pool) -> c_int {
    (*pool).total_elements - atomic_read(&(*pool).counter)
}

#[inline]
unsafe fn snd_seq_output_ok(pool: *mut snd_seq_pool) -> c_int {
    (snd_seq_pool_available(pool) >= (*pool).room) as c_int
}

/*
 * Variable length event:
 * The event like sysex uses variable length type.
 * The external data may be stored in three different formats.
 * 1) kernel space
 *    This is the normal case.
 *      ext.data.len = length
 *      ext.data.ptr = buffer pointer
 * 2) user space
 *    When an event is generated via read(), the external data is
 *    kept in user space until expanded.
 *      ext.data.len = length | SNDRV_SEQ_EXT_USRPTR
 *      ext.data.ptr = userspace pointer
 * 3) chained cells
 *    When the variable length event is enqueued (in prioq or fifo),
 *    the external data is decomposed to several cells.
 *      ext.data.len = length | SNDRV_SEQ_EXT_CHAINED
 *      ext.data.ptr = the additiona cell head
 *         -> cell.next -> cell.next -> ..
 */

/*
 * exported:
 * call dump function to expand external data.
 */

unsafe fn get_var_len(event: *const snd_seq_event) -> c_int {
    if ((*event).flags & SNDRV_SEQ_EVENT_LENGTH_MASK) != SNDRV_SEQ_EVENT_LENGTH_VARIABLE {
        return -EINVAL;
    }

    ((*event).data.ext.len & !SNDRV_SEQ_EXT_MASK) as c_int
}

unsafe fn dump_var_event(
    event: *const snd_seq_event,
    func: snd_seq_dump_func_t,
    private_data: *mut c_void,
    mut offset: c_int,
    maxlen: c_int,
) -> c_int {
    let mut len: c_int;
    let mut err: c_int;
    let mut cell: *mut snd_seq_event_cell;

    len = get_var_len(event);
    if len <= 0 {
        return len;
    }
    if len <= offset {
        return 0;
    }
    if maxlen != 0 && len > offset + maxlen {
        len = offset + maxlen;
    }

    if ((*event).data.ext.len & SNDRV_SEQ_EXT_USRPTR) != 0 {
        let mut buf = [0i8; 32];
        let mut curptr = (*event).data.ext.ptr;
        curptr = curptr.add(offset as usize);
        len -= offset;
        while len > 0 {
            let mut size = size_of::<[c_char; 32]>() as c_int;
            if len < size {
                size = len;
            }
            if copy_from_user(buf.as_mut_ptr() as *mut c_void, curptr as *const c_void, size as usize)
                != 0
            {
                return -EFAULT;
            }
            err = func.unwrap()(private_data, buf.as_mut_ptr() as *mut c_void, size);
            if err < 0 {
                return err;
            }
            curptr = curptr.add(size as usize);
            len -= size;
        }
        return 0;
    }
    if ((*event).data.ext.len & SNDRV_SEQ_EXT_CHAINED) == 0 {
        return func.unwrap()(
            private_data,
            (*event).data.ext.ptr.add(offset as usize) as *mut c_void,
            len - offset,
        );
    }

    cell = (*event).data.ext.ptr as *mut snd_seq_event_cell;
    while len > 0 && !cell.is_null() {
        let mut size = size_of::<snd_seq_event>() as c_int;
        let curptr = (*cell).event_mut() as *mut c_char;

        if offset >= size {
            offset -= size;
            len -= size;
            cell = (*cell).next;
            continue;
        }
        if len < size {
            size = len;
        }
        err = func.unwrap()(private_data, curptr.add(offset as usize) as *mut c_void, size - offset);
        if err < 0 {
            return err;
        }
        offset = 0;
        len -= size;
        cell = (*cell).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_dump_var_event(
    event: *const snd_seq_event,
    func: snd_seq_dump_func_t,
    private_data: *mut c_void,
) -> c_int {
    dump_var_event(event, func, private_data, 0, 0)
}

/*
 * exported:
 * expand the variable length event to linear buffer space.
 */

unsafe extern "C" fn seq_copy_in_kernel(ptr: *mut c_void, src: *mut c_void, size: c_int) -> c_int {
    let bufptr = ptr as *mut *mut c_char;

    memcpy(*bufptr as *mut c_void, src as *const c_void, size as usize);
    *bufptr = (*bufptr).add(size as usize);
    0
}

unsafe extern "C" fn seq_copy_in_user(ptr: *mut c_void, src: *mut c_void, size: c_int) -> c_int {
    let bufptr = ptr as *mut *mut c_char;

    if copy_to_user(*bufptr as *mut c_void, src as *const c_void, size as usize) != 0 {
        return -EFAULT;
    }
    *bufptr = (*bufptr).add(size as usize);
    0
}

unsafe fn expand_var_event(
    event: *const snd_seq_event,
    offset: c_int,
    size: c_int,
    buf: *mut c_char,
    in_kernel: bool_t,
) -> c_int {
    if ((*event).data.ext.len & SNDRV_SEQ_EXT_USRPTR) != 0 {
        if !in_kernel {
            return -EINVAL;
        }
        if copy_from_user(
            buf as *mut c_void,
            (*event).data.ext.ptr.add(offset as usize) as *const c_void,
            size as usize,
        ) != 0
        {
            return -EFAULT;
        }
        return 0;
    }
    dump_var_event(
        event,
        if in_kernel {
            Some(seq_copy_in_kernel)
        } else {
            Some(seq_copy_in_user)
        },
        &buf as *const *mut c_char as *mut c_void,
        offset,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_expand_var_event(
    event: *const snd_seq_event,
    count: c_int,
    buf: *mut c_char,
    in_kernel: c_int,
    size_aligned: c_int,
) -> c_int {
    let mut len: c_int;
    let mut newlen: c_int;
    let err: c_int;

    len = get_var_len(event);
    if len < 0 {
        return len;
    }
    newlen = len;
    if size_aligned > 0 {
        newlen = roundup(len, size_aligned);
    }
    if count < newlen {
        return -EAGAIN;
    }
    err = expand_var_event(event, 0, len, buf, in_kernel != 0);
    if err < 0 {
        return err;
    }
    if len != newlen {
        if in_kernel != 0 {
            memset(buf.add(len as usize) as *mut c_void, 0, (newlen - len) as usize);
        } else if clear_user(buf.add(len as usize) as *mut c_void, (newlen - len) as usize) != 0 {
            return -EFAULT;
        }
    }
    newlen
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_expand_var_event_at(
    event: *const snd_seq_event,
    count: c_int,
    buf: *mut c_char,
    offset: c_int,
) -> c_int {
    let mut len: c_int;
    let err: c_int;

    len = get_var_len(event);
    if len < 0 {
        return len;
    }
    if len <= offset {
        return 0;
    }
    len -= offset;
    if len > count {
        len = count;
    }
    err = expand_var_event(event, offset, len, buf, true);
    if err < 0 {
        return err;
    }
    len
}

/*
 * release this cell, free extended data if available
 */

#[inline]
unsafe fn free_cell(pool: *mut snd_seq_pool, cell: *mut snd_seq_event_cell) {
    (*cell).next = (*pool).free;
    (*pool).free = cell;
    atomic_dec(&mut (*pool).counter);
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_cell_free(cell: *mut snd_seq_event_cell) {
    let pool: *mut snd_seq_pool;

    if snd_BUG_ON(cell.is_null()) != 0 {
        return;
    }
    pool = (*cell).pool;
    if snd_BUG_ON(pool.is_null()) != 0 {
        return;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    free_cell(pool, cell);
    if snd_seq_ev_is_variable((*cell).event_ref()) != 0 {
        if ((*(*cell).event_ref()).data.ext.len & SNDRV_SEQ_EXT_CHAINED) != 0 {
            let mut curp: *mut snd_seq_event_cell;
            let mut nextptr: *mut snd_seq_event_cell;
            curp = (*(*cell).event_ref()).data.ext.ptr as *mut snd_seq_event_cell;
            while !curp.is_null() {
                nextptr = (*curp).next;
                (*curp).next = (*pool).free;
                free_cell(pool, curp);
                curp = nextptr;
            }
        }
    }
    if waitqueue_active(&mut (*pool).output_sleep) != 0 {
        /* has enough space now? */
        if snd_seq_output_ok(pool) != 0 {
            wake_up(&mut (*pool).output_sleep);
        }
    }
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
}

/*
 * allocate an event cell.
 */
unsafe fn snd_seq_cell_alloc(
    pool: *mut snd_seq_pool,
    cellp: *mut *mut snd_seq_event_cell,
    nonblock: c_int,
    _file: *mut file,
    mutexp: *mut mutex,
) -> c_int {
    let mut cell: *mut snd_seq_event_cell;
    let mut flags: c_ulong = 0;
    let mut err: c_int = -EAGAIN;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();

    if pool.is_null() {
        return -EINVAL;
    }

    *cellp = ptr::null_mut();

    init_waitqueue_entry(&mut wait, current);
    spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    if (*pool).ptr.is_null() {
        /* not initialized */
        pr_debug(c"ALSA: seq: pool is not initialized\n".as_ptr());
        err = -EINVAL;
        goto_error(pool, flags);
        return err;
    }
    while (*pool).free.is_null() && nonblock == 0 && (*pool).closing == 0 {
        set_current_state(TASK_INTERRUPTIBLE);
        add_wait_queue(&mut (*pool).output_sleep, &mut wait);
        spin_unlock_irqrestore(&mut (*pool).lock, flags);
        if !mutexp.is_null() {
            mutex_unlock(mutexp);
        }
        schedule();
        if !mutexp.is_null() {
            mutex_lock(mutexp);
        }
        spin_lock_irqsave(&mut (*pool).lock, &mut flags);
        remove_wait_queue(&mut (*pool).output_sleep, &mut wait);
        /* interrupted? */
        if signal_pending(current) != 0 {
            err = -ERESTARTSYS;
            goto_error(pool, flags);
            return err;
        }
    }
    if (*pool).closing != 0 {
        /* closing.. */
        err = -ENOMEM;
        goto_error(pool, flags);
        return err;
    }

    cell = (*pool).free;
    if !cell.is_null() {
        let used: c_int;
        (*pool).free = (*cell).next;
        atomic_inc(&mut (*pool).counter);
        used = atomic_read(&(*pool).counter);
        if (*pool).max_used < used {
            (*pool).max_used = used;
        }
        (*pool).event_alloc_success += 1;
        /* clear cell pointers */
        (*cell).next = ptr::null_mut();
        err = 0;
    } else {
        (*pool).event_alloc_failures += 1;
    }
    *cellp = cell;

    spin_unlock_irqrestore(&mut (*pool).lock, flags);
    err
}

unsafe fn goto_error(pool: *mut snd_seq_pool, flags: c_ulong) {
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
}

/*
 * duplicate the event to a cell.
 * if the event has external data, the data is decomposed to additional
 * cells.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_event_dup(
    pool: *mut snd_seq_pool,
    event: *mut snd_seq_event,
    cellp: *mut *mut snd_seq_event_cell,
    nonblock: c_int,
    file: *mut file,
    mutexp: *mut mutex,
) -> c_int {
    let mut ncells: c_int;
    let mut err: c_int;
    let mut extlen: c_uint;
    let mut cell: *mut snd_seq_event_cell = ptr::null_mut();
    let mut size: c_int;

    *cellp = ptr::null_mut();

    ncells = 0;
    extlen = 0;
    if snd_seq_ev_is_variable(event) != 0 {
        extlen = (*event).data.ext.len & !SNDRV_SEQ_EXT_MASK;
        ncells = div_round_up(extlen as c_int, size_of::<snd_seq_event>() as c_int);
    }
    if ncells >= (*pool).total_elements {
        return -ENOMEM;
    }

    err = snd_seq_cell_alloc(pool, &mut cell, nonblock, file, mutexp);
    if err < 0 {
        return err;
    }

    /* copy the event */
    size = snd_seq_event_packet_size(event);
    memcpy(
        &mut (*cell).ump as *mut snd_seq_ump_event as *mut c_void,
        event as *const c_void,
        size as usize,
    );
    // #if IS_ENABLED(CONFIG_SND_SEQ_UMP)
    // if size < sizeof(cell->ump)
    //     cell->ump.raw.extra = 0;
    // #endif

    /* decompose */
    if snd_seq_ev_is_variable(event) != 0 {
        let mut len = extlen as c_int;
        let is_chained = ((*event).data.ext.len & SNDRV_SEQ_EXT_CHAINED) as c_int;
        let is_usrptr = ((*event).data.ext.len & SNDRV_SEQ_EXT_USRPTR) as c_int;
        let mut src: *mut snd_seq_event_cell;
        let mut tmp: *mut snd_seq_event_cell = ptr::null_mut();
        let mut tail: *mut snd_seq_event_cell;
        let mut buf: *mut c_char;

        (*(*cell).event_mut()).data.ext.len = extlen | SNDRV_SEQ_EXT_CHAINED;
        (*(*cell).event_mut()).data.ext.ptr = ptr::null_mut();

        src = (*event).data.ext.ptr as *mut snd_seq_event_cell;
        buf = (*event).data.ext.ptr;
        tail = ptr::null_mut();

        while {
            let old = ncells;
            ncells -= 1;
            old > 0
        } {
            size = size_of::<snd_seq_event>() as c_int;
            if len < size {
                size = len;
            }
            err = snd_seq_cell_alloc(pool, &mut tmp, nonblock, file, mutexp);
            if err < 0 {
                snd_seq_cell_free(cell);
                return err;
            }
            if (*(*cell).event_mut()).data.ext.ptr.is_null() {
                (*(*cell).event_mut()).data.ext.ptr = tmp as *mut c_char;
            }
            if !tail.is_null() {
                (*tail).next = tmp;
            }
            tail = tmp;
            /* copy chunk */
            if is_chained != 0 && !src.is_null() {
                ptr::copy_nonoverlapping(
                    (*src).event_ref(),
                    (*tmp).event_mut(),
                    1,
                );
                src = (*src).next;
            } else if is_usrptr != 0 {
                if copy_from_user(
                    (*tmp).event_mut() as *mut c_void,
                    buf as *const c_void,
                    size as usize,
                ) != 0
                {
                    err = -EFAULT;
                    snd_seq_cell_free(cell);
                    return err;
                }
            } else {
                memcpy((*tmp).event_mut() as *mut c_void, buf as *const c_void, size as usize);
            }
            buf = buf.add(size as usize);
            len -= size;
        }
    }

    *cellp = cell;
    0
}

/* poll wait */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_poll_wait(
    pool: *mut snd_seq_pool,
    file: *mut file,
    wait: *mut poll_table,
) -> c_int {
    poll_wait(file, &mut (*pool).output_sleep, wait);
    spin_lock_irq(&mut (*pool).lock);
    let ret = snd_seq_output_ok(pool);
    spin_unlock_irq(&mut (*pool).lock);
    ret
}

/* allocate room specified number of events */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_init(pool: *mut snd_seq_pool) -> c_int {
    let mut cell: c_int;
    let mut cellptr: *mut snd_seq_event_cell;

    if snd_BUG_ON(pool.is_null()) != 0 {
        return -EINVAL;
    }

    cellptr = kvmalloc_objs_snd_seq_event_cell((*pool).size);
    if cellptr.is_null() {
        return -ENOMEM;
    }

    /* add new cells to the free cell list */
    spin_lock_irq(&mut (*pool).lock);
    if !(*pool).ptr.is_null() {
        spin_unlock_irq(&mut (*pool).lock);
        kvfree(cellptr as *mut c_void);
        return 0;
    }

    (*pool).ptr = cellptr;
    (*pool).free = ptr::null_mut();

    cell = 0;
    while cell < (*pool).size {
        cellptr = (*pool).ptr.add(cell as usize);
        (*cellptr).pool = pool;
        (*cellptr).next = (*pool).free;
        (*pool).free = cellptr;
        cell += 1;
    }
    (*pool).room = ((*pool).size + 1) / 2;

    /* init statistics */
    (*pool).max_used = 0;
    (*pool).total_elements = (*pool).size;
    spin_unlock_irq(&mut (*pool).lock);
    0
}

/* refuse the further insertion to the pool */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_mark_closing(pool: *mut snd_seq_pool) {
    if snd_BUG_ON(pool.is_null()) != 0 {
        return;
    }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    (*pool).closing = 1;
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
}

/* remove events */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_done(pool: *mut snd_seq_pool) -> c_int {
    let ptr_: *mut snd_seq_event_cell;

    if snd_BUG_ON(pool.is_null()) != 0 {
        return -EINVAL;
    }

    /* wait for closing all threads */
    if waitqueue_active(&mut (*pool).output_sleep) != 0 {
        wake_up(&mut (*pool).output_sleep);
    }

    while atomic_read(&(*pool).counter) > 0 {
        schedule_timeout_uninterruptible(1);
    }

    /* release all resources */
    spin_lock_irq(&mut (*pool).lock);
    ptr_ = (*pool).ptr;
    (*pool).ptr = ptr::null_mut();
    (*pool).free = ptr::null_mut();
    (*pool).total_elements = 0;
    spin_unlock_irq(&mut (*pool).lock);

    kvfree(ptr_ as *mut c_void);

    spin_lock_irq(&mut (*pool).lock);
    (*pool).closing = 0;
    spin_unlock_irq(&mut (*pool).lock);

    0
}

/* init new memory pool */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_new(poolsize: c_int) -> *mut snd_seq_pool {
    let pool: *mut snd_seq_pool;

    /* create pool block */
    pool = kzalloc_obj_snd_seq_pool();
    if pool.is_null() {
        return ptr::null_mut();
    }
    spin_lock_init(&mut (*pool).lock);
    (*pool).ptr = ptr::null_mut();
    (*pool).free = ptr::null_mut();
    (*pool).total_elements = 0;
    atomic_set(&mut (*pool).counter, 0);
    (*pool).closing = 0;
    init_waitqueue_head(&mut (*pool).output_sleep);

    (*pool).size = poolsize;

    /* init statistics */
    (*pool).max_used = 0;
    pool
}

/* remove memory pool */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_pool_delete(ppool: *mut *mut snd_seq_pool) -> c_int {
    let pool: *mut snd_seq_pool = *ppool;

    *ppool = ptr::null_mut();
    if pool.is_null() {
        return 0;
    }
    snd_seq_pool_mark_closing(pool);
    snd_seq_pool_done(pool);
    kfree(pool as *mut c_void);
    0
}

/* exported to seq_clientmgr.c */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_info_pool(
    buffer: *mut snd_info_buffer,
    pool: *mut snd_seq_pool,
    space: *mut c_char,
) {
    if pool.is_null() {
        return;
    }
    snd_iprintf(
        buffer,
        c"%sPool size          : %d\n".as_ptr(),
        space,
        (*pool).total_elements,
    );
    snd_iprintf(
        buffer,
        c"%sCells in use       : %d\n".as_ptr(),
        space,
        atomic_read(&(*pool).counter),
    );
    snd_iprintf(
        buffer,
        c"%sPeak cells in use  : %d\n".as_ptr(),
        space,
        (*pool).max_used,
    );
    snd_iprintf(
        buffer,
        c"%sAlloc success      : %d\n".as_ptr(),
        space,
        (*pool).event_alloc_success,
    );
    snd_iprintf(
        buffer,
        c"%sAlloc failures     : %d\n".as_ptr(),
        space,
        (*pool).event_alloc_failures,
    );
}

#[inline]
fn div_round_up(n: c_int, d: c_int) -> c_int {
    (n + d - 1) / d
}

#[inline]
fn roundup(x: c_int, y: c_int) -> c_int {
    div_round_up(x, y) * y
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
