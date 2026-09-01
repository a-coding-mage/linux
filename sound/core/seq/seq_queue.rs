// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer Timing queue handling
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 *
 * MAJOR CHANGES
 *   Nov. 13, 1999	Takashi Iwai <iwai@ww.uni-erlangen.de>
 *     - Queues are allocated dynamically via ioctl.
 *     - When owner client is deleted, all owned queues are deleted, too.
 *     - Owner of unlocked queue is kept unmodified even if it is
 *	 manipulated by other clients.
 *     - Owner field in SET_QUEUE_OWNER ioctl must be identical with the
 *       caller client.  i.e. Changing owner to a third client is not
 *       allowed.
 *
 *  Aug. 30, 2000	Takashi Iwai
 *     - Queues are managed in static array again, but with better way.
 *       The API itself is identical.
 *     - The queue is locked when struct snd_seq_queue pointer is returned via
 *       queueptr().  This pointer *MUST* be released afterward by
 *       queuefree(ptr).
 *     - Addition of experimental sync support.
 */

// Dependencies from the original C includes:
// <linux/init.h>, <linux/slab.h>, <sound/core.h>
// "seq_memory.h", "seq_queue.h", "seq_clientmgr.h", "seq_fifo.h",
// "seq_timer.h", "seq_info.h"

use core::ptr;

/* list of allocated queues */
static mut queue_list: [*mut snd_seq_queue; SNDRV_SEQ_MAX_QUEUES as usize] =
    [ptr::null_mut(); SNDRV_SEQ_MAX_QUEUES as usize];
static mut queue_list_lock: spinlock_t = SPIN_LOCK_UNLOCKED;
/* number of queues allocated */
static mut num_queues: c_int = 0;

pub unsafe extern "C" fn snd_seq_queue_get_cur_queues() -> c_int {
    num_queues
}

/*----------------------------------------------------------------*/

/* assign queue id and insert to list */
unsafe fn queue_list_add(q: *mut snd_seq_queue) -> c_int {
    let mut i: c_int;

    spin_lock_irqsave(&raw mut queue_list_lock);
    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        if queue_list[i as usize].is_null() {
            queue_list[i as usize] = q;
            (*q).queue = i;
            num_queues += 1;
            spin_unlock_irqrestore(&raw mut queue_list_lock);
            return i;
        }
        i += 1;
    }
    spin_unlock_irqrestore(&raw mut queue_list_lock);
    -1
}

unsafe fn queue_list_remove(id: c_int, client: c_int) -> *mut snd_seq_queue {
    let q: *mut snd_seq_queue;

    spin_lock_irqsave(&raw mut queue_list_lock);
    q = queue_list[id as usize];
    if !q.is_null() {
        spin_lock(&mut (*q).owner_lock);
        if (*q).owner == client {
            /* found */
            (*q).klocked = 1;
            queue_list[id as usize] = ptr::null_mut();
            num_queues -= 1;
            spin_unlock(&mut (*q).owner_lock);
            spin_unlock_irqrestore(&raw mut queue_list_lock);
            return q;
        }
        spin_unlock(&mut (*q).owner_lock);
    }
    spin_unlock_irqrestore(&raw mut queue_list_lock);
    ptr::null_mut()
}

/*----------------------------------------------------------------*/

/* create new queue (constructor) */
unsafe fn queue_new(owner: c_int, locked: c_int) -> *mut snd_seq_queue {
    let q: *mut snd_seq_queue;

    q = kzalloc_obj::<snd_seq_queue>();
    if q.is_null() {
        return ptr::null_mut();
    }

    spin_lock_init(&mut (*q).owner_lock);
    spin_lock_init(&mut (*q).check_lock);
    mutex_init(&mut (*q).timer_mutex);
    snd_use_lock_init(&mut (*q).use_lock);
    (*q).queue = -1;

    (*q).tickq = snd_seq_prioq_new();
    (*q).timeq = snd_seq_prioq_new();
    (*q).timer = snd_seq_timer_new();
    if (*q).tickq.is_null() || (*q).timeq.is_null() || (*q).timer.is_null() {
        snd_seq_prioq_delete(&mut (*q).tickq);
        snd_seq_prioq_delete(&mut (*q).timeq);
        snd_seq_timer_delete(&mut (*q).timer);
        kfree(q.cast());
        return ptr::null_mut();
    }

    (*q).owner = owner;
    (*q).locked = locked;
    (*q).klocked = 0;

    q
}

/* delete queue (destructor) */
unsafe fn queue_delete(q: *mut snd_seq_queue) {
    /* stop and release the timer */
    mutex_lock(&mut (*q).timer_mutex);
    snd_seq_timer_stop((*q).timer);
    snd_seq_timer_close(q);
    mutex_unlock(&mut (*q).timer_mutex);
    /* wait until access free */
    snd_use_lock_sync(&mut (*q).use_lock);
    /* release resources... */
    snd_seq_prioq_delete(&mut (*q).tickq);
    snd_seq_prioq_delete(&mut (*q).timeq);
    snd_seq_timer_delete(&mut (*q).timer);

    kfree(q.cast());
}

/*----------------------------------------------------------------*/

/* delete all existing queues */
pub unsafe extern "C" fn snd_seq_queues_delete() {
    let mut i: c_int;

    /* clear list */
    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        if !queue_list[i as usize].is_null() {
            queue_delete(queue_list[i as usize]);
        }
        i += 1;
    }
}

/* allocate a new queue -
 * return pointer to new queue or ERR_PTR(-errno) for error
 * The new queue's use_lock is set to 1. It is the caller's responsibility to
 * call snd_use_lock_free(&q->use_lock).
 */
pub unsafe extern "C" fn snd_seq_queue_alloc(
    client: c_int,
    locked: c_int,
    info_flags: c_uint,
) -> *mut snd_seq_queue {
    let q: *mut snd_seq_queue;

    q = queue_new(client, locked);
    if q.is_null() {
        return ERR_PTR(-ENOMEM).cast();
    }
    (*q).info_flags = info_flags;
    queue_use(q, client, 1);
    snd_use_lock_use(&mut (*q).use_lock);
    if queue_list_add(q) < 0 {
        snd_use_lock_free(&mut (*q).use_lock);
        queue_delete(q);
        return ERR_PTR(-ENOMEM).cast();
    }
    q
}

/* delete a queue - queue must be owned by the client */
pub unsafe extern "C" fn snd_seq_queue_delete(client: c_int, queueid: c_int) -> c_int {
    let q: *mut snd_seq_queue;

    if queueid < 0 || queueid >= SNDRV_SEQ_MAX_QUEUES {
        return -EINVAL;
    }
    q = queue_list_remove(queueid, client);
    if q.is_null() {
        return -EINVAL;
    }
    queue_delete(q);

    0
}

/* return pointer to queue structure for specified id */
pub unsafe extern "C" fn queueptr(queueid: c_int) -> *mut snd_seq_queue {
    let q: *mut snd_seq_queue;

    if queueid < 0 || queueid >= SNDRV_SEQ_MAX_QUEUES {
        return ptr::null_mut();
    }
    spin_lock_irqsave(&raw mut queue_list_lock);
    q = queue_list[queueid as usize];
    if !q.is_null() {
        snd_use_lock_use(&mut (*q).use_lock);
    }
    spin_unlock_irqrestore(&raw mut queue_list_lock);
    q
}

/* return the (first) queue matching with the specified name */
pub unsafe extern "C" fn snd_seq_queue_find_name(name: *mut c_char) -> *mut snd_seq_queue {
    let mut i: c_int;

    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queueptr(i);

        if !q.is_null() {
            if strncmp((*q).name.as_mut_ptr(), name, core::mem::size_of_val(&(*q).name)) == 0 {
                return q;
            }
            snd_use_lock_free(&mut (*q).use_lock);
        }
        i += 1;
    }
    ptr::null_mut()
}

/* -------------------------------------------------------- */

const MAX_CELL_PROCESSES_IN_QUEUE: c_int = 1000;

pub unsafe extern "C" fn snd_seq_check_queue(q: *mut snd_seq_queue, atomic: c_int, hop: c_int) {
    let mut cell: *mut snd_seq_event_cell;
    let cur_tick: snd_seq_tick_time_t;
    let cur_time: snd_seq_real_time_t;
    let mut processed: c_int = 0;

    if q.is_null() {
        return;
    }

    /* make this function non-reentrant */
    spin_lock_irqsave(&mut (*q).check_lock);
    if (*q).check_blocked != 0 {
        (*q).check_again = 1;
        spin_unlock_irqrestore(&mut (*q).check_lock);
        return; /* other thread is already checking queues */
    }
    (*q).check_blocked = 1;
    spin_unlock_irqrestore(&mut (*q).check_lock);

    loop {
        /* Process tick queue... */
        cur_tick = snd_seq_timer_get_cur_tick((*q).timer);
        loop {
            cell = snd_seq_prioq_cell_out((*q).tickq, &cur_tick);
            if cell.is_null() {
                break;
            }
            snd_seq_dispatch_event(cell, atomic, hop);
            processed += 1;
            if processed >= MAX_CELL_PROCESSES_IN_QUEUE {
                break; /* the rest processed at the next batch */
            }
        }

        if processed < MAX_CELL_PROCESSES_IN_QUEUE {
            /* Process time queue... */
            cur_time = snd_seq_timer_get_cur_time((*q).timer, false);
            loop {
                cell = snd_seq_prioq_cell_out((*q).timeq, &cur_time);
                if cell.is_null() {
                    break;
                }
                snd_seq_dispatch_event(cell, atomic, hop);
                processed += 1;
                if processed >= MAX_CELL_PROCESSES_IN_QUEUE {
                    break; /* the rest processed at the next batch */
                }
            }
        }

        /* free lock */
        spin_lock_irqsave(&mut (*q).check_lock);
        if (*q).check_again != 0 {
            (*q).check_again = 0;
            if processed < MAX_CELL_PROCESSES_IN_QUEUE {
                spin_unlock_irqrestore(&mut (*q).check_lock);
                continue;
            }
        }
        (*q).check_blocked = 0;
        spin_unlock_irqrestore(&mut (*q).check_lock);
        break;
    }
}

/* enqueue a event to singe queue */
pub unsafe extern "C" fn snd_seq_enqueue_event(
    cell: *mut snd_seq_event_cell,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    let dest: c_int;
    let mut err: c_int;

    if snd_BUG_ON(cell.is_null()) != 0 {
        return -EINVAL;
    }
    dest = (*cell).event.queue; /* destination queue */

    let q: *mut snd_seq_queue = queueptr(dest);
    if q.is_null() {
        return -EINVAL;
    }
    /* handle relative time stamps, convert them into absolute */
    if ((*cell).event.flags & SNDRV_SEQ_TIME_MODE_MASK) == SNDRV_SEQ_TIME_MODE_REL {
        match (*cell).event.flags & SNDRV_SEQ_TIME_STAMP_MASK {
            SNDRV_SEQ_TIME_STAMP_TICK => {
                (*cell).event.time.tick += (*(*q).timer).tick.cur_tick;
            }
            SNDRV_SEQ_TIME_STAMP_REAL => {
                snd_seq_inc_real_time(&mut (*cell).event.time.time, &(*(*q).timer).cur_time);
            }
            _ => {}
        }
        (*cell).event.flags &= !SNDRV_SEQ_TIME_MODE_MASK;
        (*cell).event.flags |= SNDRV_SEQ_TIME_MODE_ABS;
    }
    /* enqueue event in the real-time or midi queue */
    match (*cell).event.flags & SNDRV_SEQ_TIME_STAMP_MASK {
        SNDRV_SEQ_TIME_STAMP_TICK => {
            err = snd_seq_prioq_cell_in((*q).tickq, cell);
        }
        SNDRV_SEQ_TIME_STAMP_REAL | _ => {
            err = snd_seq_prioq_cell_in((*q).timeq, cell);
        }
    }

    if err < 0 {
        snd_use_lock_free(&mut (*q).use_lock);
        return err;
    }

    /* trigger dispatching */
    snd_seq_check_queue(q, atomic, hop);
    snd_use_lock_free(&mut (*q).use_lock);

    0
}

/*----------------------------------------------------------------*/

unsafe fn check_access(q: *mut snd_seq_queue, client: c_int) -> c_int {
    (((*q).owner == client) || ((*q).locked == 0 && (*q).klocked == 0)) as c_int
}

/* check if the client has permission to modify queue parameters.
 * if it does, lock the queue
 */
unsafe fn queue_access_lock(q: *mut snd_seq_queue, client: c_int) -> c_int {
    let access_ok: c_int;

    spin_lock_irqsave(&mut (*q).owner_lock);
    access_ok = check_access(q, client);
    if access_ok != 0 {
        (*q).klocked = 1;
    }
    spin_unlock_irqrestore(&mut (*q).owner_lock);
    access_ok
}

/* unlock the queue */
unsafe fn queue_access_unlock(q: *mut snd_seq_queue) {
    spin_lock_irqsave(&mut (*q).owner_lock);
    (*q).klocked = 0;
    spin_unlock_irqrestore(&mut (*q).owner_lock);
}

/* exported - only checking permission */
pub unsafe extern "C" fn snd_seq_queue_check_access(queueid: c_int, client: c_int) -> c_int {
    let q: *mut snd_seq_queue = queueptr(queueid);

    if q.is_null() {
        return 0;
    }
    spin_lock_irqsave(&mut (*q).owner_lock);
    let ret = check_access(q, client);
    spin_unlock_irqrestore(&mut (*q).owner_lock);
    snd_use_lock_free(&mut (*q).use_lock);
    ret
}

/*----------------------------------------------------------------*/

/*
 * change queue's owner and permission
 */
pub unsafe extern "C" fn snd_seq_queue_set_owner(
    queueid: c_int,
    client: c_int,
    locked: c_int,
) -> c_int {
    let q: *mut snd_seq_queue = queueptr(queueid);

    if q.is_null() {
        return -EINVAL;
    }

    if queue_access_lock(q, client) == 0 {
        snd_use_lock_free(&mut (*q).use_lock);
        return -EPERM;
    }

    spin_lock_irqsave(&mut (*q).owner_lock);
    (*q).locked = if locked != 0 { 1 } else { 0 };
    (*q).owner = client;
    spin_unlock_irqrestore(&mut (*q).owner_lock);
    queue_access_unlock(q);
    snd_use_lock_free(&mut (*q).use_lock);

    0
}

/*----------------------------------------------------------------*/

/* open timer -
 * q->use mutex should be down before calling this function to avoid
 * confliction with snd_seq_queue_use()
 */
pub unsafe extern "C" fn snd_seq_queue_timer_open(queueid: c_int) -> c_int {
    let mut result: c_int = 0;
    let tmr: *mut snd_seq_timer;
    let queue: *mut snd_seq_queue = queueptr(queueid);

    if queue.is_null() {
        return -EINVAL;
    }
    tmr = (*queue).timer;
    result = snd_seq_timer_open(queue);
    if result < 0 {
        snd_seq_timer_defaults(tmr);
        result = snd_seq_timer_open(queue);
    }
    snd_use_lock_free(&mut (*queue).use_lock);
    result
}

/* close timer -
 * q->use mutex should be down before calling this function
 */
pub unsafe extern "C" fn snd_seq_queue_timer_close(queueid: c_int) -> c_int {
    let result: c_int = 0;
    let queue: *mut snd_seq_queue = queueptr(queueid);

    if queue.is_null() {
        return -EINVAL;
    }
    snd_seq_timer_close(queue);
    snd_use_lock_free(&mut (*queue).use_lock);
    result
}

/* change queue tempo and ppq */
pub unsafe extern "C" fn snd_seq_queue_timer_set_tempo(
    queueid: c_int,
    client: c_int,
    info: *mut snd_seq_queue_tempo,
) -> c_int {
    let q: *mut snd_seq_queue = queueptr(queueid);
    let mut result: c_int;

    if q.is_null() {
        return -EINVAL;
    }
    if queue_access_lock(q, client) == 0 {
        snd_use_lock_free(&mut (*q).use_lock);
        return -EPERM;
    }

    result = snd_seq_timer_set_tempo_ppq(
        (*q).timer,
        (*info).tempo,
        (*info).ppq,
        (*info).tempo_base,
    );
    if result >= 0 && (*info).skew_base > 0 {
        result = snd_seq_timer_set_skew((*q).timer, (*info).skew_value, (*info).skew_base);
    }
    queue_access_unlock(q);
    snd_use_lock_free(&mut (*q).use_lock);
    result
}

/* use or unuse this queue */
unsafe fn queue_use(queue: *mut snd_seq_queue, client: c_int, use_: c_int) {
    if use_ != 0 {
        if test_and_set_bit(client, (*queue).clients_bitmap.as_mut_ptr()) == 0 {
            (*queue).clients += 1;
        }
    } else if test_and_clear_bit(client, (*queue).clients_bitmap.as_mut_ptr()) != 0 {
        (*queue).clients -= 1;
    }
    if (*queue).clients != 0 {
        if use_ != 0 && (*queue).clients == 1 {
            snd_seq_timer_defaults((*queue).timer);
        }
        snd_seq_timer_open(queue);
    } else {
        snd_seq_timer_close(queue);
    }
}

/* use or unuse this queue -
 * if it is the first client, starts the timer.
 * if it is not longer used by any clients, stop the timer.
 */
pub unsafe extern "C" fn snd_seq_queue_use(queueid: c_int, client: c_int, use_: c_int) -> c_int {
    let queue: *mut snd_seq_queue = queueptr(queueid);

    if queue.is_null() {
        return -EINVAL;
    }
    mutex_lock(&mut (*queue).timer_mutex);
    queue_use(queue, client, use_);
    mutex_unlock(&mut (*queue).timer_mutex);
    snd_use_lock_free(&mut (*queue).use_lock);
    0
}

/*
 * check if queue is used by the client
 * return negative value if the queue is invalid.
 * return 0 if not used, 1 if used.
 */
pub unsafe extern "C" fn snd_seq_queue_is_used(queueid: c_int, client: c_int) -> c_int {
    let q: *mut snd_seq_queue = queueptr(queueid);

    if q.is_null() {
        return -EINVAL; /* invalid queue */
    }
    let ret = if test_bit(client, (*q).clients_bitmap.as_mut_ptr()) != 0 {
        1
    } else {
        0
    };
    snd_use_lock_free(&mut (*q).use_lock);
    ret
}

/*----------------------------------------------------------------*/

/* final stage notification -
 * remove cells for no longer exist client (for non-owned queue)
 * or delete this queue (for owned queue)
 */
pub unsafe extern "C" fn snd_seq_queue_client_leave(client: c_int) {
    let mut i: c_int;

    /* delete own queues from queue list */
    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queue_list_remove(i, client);
        if !q.is_null() {
            queue_delete(q);
        }
        i += 1;
    }

    /* remove cells from existing queues -
     * they are not owned by this client
     */
    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queueptr(i);
        if !q.is_null() {
            if test_bit(client, (*q).clients_bitmap.as_mut_ptr()) != 0 {
                snd_seq_prioq_leave((*q).tickq, client, 0);
                snd_seq_prioq_leave((*q).timeq, client, 0);
                snd_seq_queue_use((*q).queue, client, 0);
            }
            snd_use_lock_free(&mut (*q).use_lock);
        }
        i += 1;
    }
}

/*----------------------------------------------------------------*/

/* remove cells based on flush criteria */
pub unsafe extern "C" fn snd_seq_queue_remove_cells(
    client: c_int,
    info: *mut snd_seq_remove_events,
) {
    let mut i: c_int;

    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queueptr(i);
        if !q.is_null() {
            if test_bit(client, (*q).clients_bitmap.as_mut_ptr()) != 0
                && (((*info).remove_mode & SNDRV_SEQ_REMOVE_DEST) == 0
                    || (*q).queue == (*info).queue)
            {
                snd_seq_prioq_remove_events((*q).tickq, client, info);
                snd_seq_prioq_remove_events((*q).timeq, client, info);
            }
            snd_use_lock_free(&mut (*q).use_lock);
        }
        i += 1;
    }
}

/*----------------------------------------------------------------*/

/*
 * send events to all subscribed ports
 */
unsafe fn queue_broadcast_event(
    q: *mut snd_seq_queue,
    ev: *mut snd_seq_event,
    atomic: c_int,
    hop: c_int,
) {
    let mut sev: snd_seq_event;

    sev = *ev;

    sev.flags = SNDRV_SEQ_TIME_STAMP_TICK | SNDRV_SEQ_TIME_MODE_ABS;
    sev.time.tick = (*(*q).timer).tick.cur_tick;
    sev.queue = (*q).queue;
    sev.data.queue.queue = (*q).queue;

    /* broadcast events from Timer port */
    sev.source.client = SNDRV_SEQ_CLIENT_SYSTEM;
    sev.source.port = SNDRV_SEQ_PORT_SYSTEM_TIMER;
    sev.dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
    snd_seq_kernel_client_dispatch(SNDRV_SEQ_CLIENT_SYSTEM, &mut sev, atomic, hop);
}

/*
 * process a received queue-control event.
 * this function is exported for seq_sync.c.
 */
unsafe fn snd_seq_queue_process_event(
    q: *mut snd_seq_queue,
    ev: *mut snd_seq_event,
    atomic: c_int,
    hop: c_int,
) {
    match (*ev).type_ {
        SNDRV_SEQ_EVENT_START => {
            snd_seq_prioq_leave((*q).tickq, (*ev).source.client, 1);
            snd_seq_prioq_leave((*q).timeq, (*ev).source.client, 1);
            if snd_seq_timer_start((*q).timer) == 0 {
                queue_broadcast_event(q, ev, atomic, hop);
            }
        }
        SNDRV_SEQ_EVENT_CONTINUE => {
            if snd_seq_timer_continue((*q).timer) == 0 {
                queue_broadcast_event(q, ev, atomic, hop);
            }
        }
        SNDRV_SEQ_EVENT_STOP => {
            snd_seq_timer_stop((*q).timer);
            queue_broadcast_event(q, ev, atomic, hop);
        }
        SNDRV_SEQ_EVENT_TEMPO => {
            snd_seq_timer_set_tempo((*q).timer, (*ev).data.queue.param.value);
            queue_broadcast_event(q, ev, atomic, hop);
        }
        SNDRV_SEQ_EVENT_SETPOS_TICK => {
            if snd_seq_timer_set_position_tick((*q).timer, (*ev).data.queue.param.time.tick) == 0 {
                queue_broadcast_event(q, ev, atomic, hop);
            }
        }
        SNDRV_SEQ_EVENT_SETPOS_TIME => {
            if snd_seq_timer_set_position_time((*q).timer, (*ev).data.queue.param.time.time) == 0 {
                queue_broadcast_event(q, ev, atomic, hop);
            }
        }
        SNDRV_SEQ_EVENT_QUEUE_SKEW => {
            if snd_seq_timer_set_skew(
                (*q).timer,
                (*ev).data.queue.param.skew.value,
                (*ev).data.queue.param.skew.base,
            ) == 0
            {
                queue_broadcast_event(q, ev, atomic, hop);
            }
        }
        _ => {}
    }
}

/*
 * Queue control via timer control port:
 * this function is exported as a callback of timer port.
 */
pub unsafe extern "C" fn snd_seq_control_queue(
    ev: *mut snd_seq_event,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    if snd_BUG_ON(ev.is_null()) != 0 {
        return -EINVAL;
    }

    let q: *mut snd_seq_queue = queueptr((*ev).data.queue.queue);

    if q.is_null() {
        return -EINVAL;
    }

    if queue_access_lock(q, (*ev).source.client) == 0 {
        snd_use_lock_free(&mut (*q).use_lock);
        return -EPERM;
    }

    snd_seq_queue_process_event(q, ev, atomic, hop);

    queue_access_unlock(q);
    snd_use_lock_free(&mut (*q).use_lock);
    0
}

/*----------------------------------------------------------------*/

/* CONFIG_SND_PROC_FS conditional code from the C source. */
#[cfg(CONFIG_SND_PROC_FS)]
/* exported to seq_info.c */
pub unsafe extern "C" fn snd_seq_info_queues_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut i: c_int;
    let mut bpm: c_int;
    let tmr: *mut snd_seq_timer;
    let locked: bool;
    let owner: c_int;

    i = 0;
    while i < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queueptr(i);
        if !q.is_null() {
            tmr = (*q).timer;
            if (*tmr).tempo != 0 {
                bpm = (60000 * (*tmr).tempo_base) / (*tmr).tempo;
            } else {
                bpm = 0;
            }

            spin_lock_irq(&mut (*q).owner_lock);
            locked = (*q).locked != 0;
            owner = (*q).owner;
            spin_unlock_irq(&mut (*q).owner_lock);

            snd_iprintf(buffer, c"queue %d: [%s]\n".as_ptr(), (*q).queue, (*q).name.as_ptr());
            snd_iprintf(buffer, c"owned by client    : %d\n".as_ptr(), owner);
            snd_iprintf(
                buffer,
                c"lock status        : %s\n".as_ptr(),
                if locked { c"Locked".as_ptr() } else { c"Free".as_ptr() },
            );
            snd_iprintf(
                buffer,
                c"queued time events : %d\n".as_ptr(),
                snd_seq_prioq_avail((*q).timeq),
            );
            snd_iprintf(
                buffer,
                c"queued tick events : %d\n".as_ptr(),
                snd_seq_prioq_avail((*q).tickq),
            );
            snd_iprintf(
                buffer,
                c"timer state        : %s\n".as_ptr(),
                if (*tmr).running != 0 {
                    c"Running".as_ptr()
                } else {
                    c"Stopped".as_ptr()
                },
            );
            snd_iprintf(buffer, c"timer PPQ          : %d\n".as_ptr(), (*tmr).ppq);
            snd_iprintf(buffer, c"current tempo      : %d\n".as_ptr(), (*tmr).tempo);
            snd_iprintf(buffer, c"tempo base         : %d ns\n".as_ptr(), (*tmr).tempo_base);
            snd_iprintf(buffer, c"current BPM        : %d\n".as_ptr(), bpm);
            snd_iprintf(
                buffer,
                c"current time       : %d.%09d s\n".as_ptr(),
                (*tmr).cur_time.tv_sec,
                (*tmr).cur_time.tv_nsec,
            );
            snd_iprintf(buffer, c"current tick       : %d\n".as_ptr(), (*tmr).tick.cur_tick);
            snd_iprintf(buffer, c"\n".as_ptr());
            snd_use_lock_free(&mut (*q).use_lock);
        }
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
