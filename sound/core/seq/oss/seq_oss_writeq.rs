// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * seq_oss_writeq.c - write queue and sync
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies in the original C source:
 * "seq_oss_writeq.h", "seq_oss_event.h", "seq_oss_timer.h",
 * <sound/seq_oss_legacy.h>, "../seq_lock.h", "../seq_clientmgr.h",
 * <linux/wait.h>, <linux/slab.h>, <linux/sched/signal.h>
 */

pub type abstime_t = u32;

pub const SNDRV_SEQ_IOCTL_SET_CLIENT_POOL: u32 = 0;
pub const SNDRV_SEQ_IOCTL_REMOVE_EVENTS: u32 = 0;
pub const SNDRV_SEQ_IOCTL_GET_CLIENT_POOL: u32 = 0;
pub const SNDRV_SEQ_REMOVE_OUTPUT: u32 = 0;
pub const SNDRV_SEQ_EVENT_ECHO: u8 = 0;
pub const SEQ_SYNCTIMER: u8 = 0;
pub const HZ: c_long = 0;

pub type c_int = i32;
pub type c_long = i64;
pub type c_void = core::ffi::c_void;

#[repr(C)]
pub struct seq_oss_writeq {
    pub dp: *mut seq_oss_devinfo,
    pub maxlen: c_int,
    pub sync_lock: spinlock_t,
    pub sync_event_put: c_int,
    pub sync_time: abstime_t,
    pub sync_sleep: wait_queue_head_t,
}

#[repr(C)]
pub struct seq_oss_devinfo {
    pub cseq: c_int,
    pub timer: *mut c_void,
    pub addr: snd_seq_addr,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: u8,
    pub port: u8,
}

#[repr(C)]
pub struct snd_seq_client_pool {
    pub client: c_int,
    pub output_pool: c_int,
    pub input_pool: c_int,
    pub output_room: c_int,
    pub output_free: c_int,
    pub input_free: c_int,
}

#[repr(C)]
pub struct snd_seq_remove_events {
    pub remove_mode: u32,
}

#[repr(C)]
pub struct snd_seq_event {
    pub flags: u8,
    pub type_: u8,
    pub time: snd_seq_event_time,
    pub data: snd_seq_event_data,
}

#[repr(C)]
pub union snd_seq_event_time {
    pub tick: abstime_t,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub raw: [u8; 64],
}

#[repr(C)]
pub union evrec {
    pub t: timer_event,
}

#[repr(C)]
pub struct timer_event {
    pub code: u8,
    pub time: abstime_t,
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
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut current: *mut task_struct;

    pub fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn spin_lock_init(lock: *mut spinlock_t);
    pub fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    pub fn wake_up(wq_head: *mut wait_queue_head_t);
    pub fn wait_event_interruptible_timeout(
        wq_head: *mut wait_queue_head_t,
        condition: bool,
        timeout: c_long,
    ) -> c_long;
    pub fn signal_pending(p: *mut task_struct) -> c_int;
    pub fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);

    pub fn snd_seq_oss_control(dp: *mut seq_oss_devinfo, cmd: u32, arg: *mut c_void) -> c_int;
    pub fn snd_seq_oss_timer_cur_tick(timer: *mut c_void) -> abstime_t;
    pub fn snd_seq_oss_fill_addr(
        dp: *mut seq_oss_devinfo,
        ev: *mut snd_seq_event,
        client: u8,
        port: u8,
    );
    pub fn snd_seq_kernel_client_enqueue(
        client: c_int,
        ev: *mut snd_seq_event,
        file: *mut c_void,
        blocking: bool,
    ) -> c_int;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(core::mem::size_of::<T>(), 0) as *mut T }
}

/*
 * create a write queue record
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_new(
    dp: *mut seq_oss_devinfo,
    maxlen: c_int,
) -> *mut seq_oss_writeq {
    let q: *mut seq_oss_writeq;
    let mut pool: snd_seq_client_pool = unsafe { core::mem::zeroed() };

    q = unsafe { kzalloc_obj::<seq_oss_writeq>() };
    if q.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        (*q).dp = dp;
        (*q).maxlen = maxlen;
        spin_lock_init(&mut (*q).sync_lock);
        (*q).sync_event_put = 0;
        (*q).sync_time = 0;
        init_waitqueue_head(&mut (*q).sync_sleep);

        memset(
            &mut pool as *mut snd_seq_client_pool as *mut c_void,
            0,
            core::mem::size_of::<snd_seq_client_pool>(),
        );
        pool.client = (*dp).cseq;
        pool.output_pool = maxlen;
        pool.output_room = maxlen / 2;

        snd_seq_oss_control(
            dp,
            SNDRV_SEQ_IOCTL_SET_CLIENT_POOL,
            &mut pool as *mut snd_seq_client_pool as *mut c_void,
        );
    }

    q
}

/*
 * delete the write queue
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_delete(q: *mut seq_oss_writeq) {
    if !q.is_null() {
        unsafe {
            snd_seq_oss_writeq_clear(q); /* to be sure */
            kfree(q as *mut c_void);
        }
    }
}

/*
 * reset the write queue
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_clear(q: *mut seq_oss_writeq) {
    let mut reset: snd_seq_remove_events = unsafe { core::mem::zeroed() };

    unsafe {
        memset(
            &mut reset as *mut snd_seq_remove_events as *mut c_void,
            0,
            core::mem::size_of::<snd_seq_remove_events>(),
        );
        reset.remove_mode = SNDRV_SEQ_REMOVE_OUTPUT; /* remove all */
        snd_seq_oss_control(
            (*q).dp,
            SNDRV_SEQ_IOCTL_REMOVE_EVENTS,
            &mut reset as *mut snd_seq_remove_events as *mut c_void,
        );

        /* wake up sleepers if any */
        snd_seq_oss_writeq_wakeup(q, 0);
    }
}

/*
 * wait until the write buffer has enough room
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_sync(q: *mut seq_oss_writeq) -> c_int {
    let dp: *mut seq_oss_devinfo = unsafe { (*q).dp };
    let time: abstime_t;

    unsafe {
        time = snd_seq_oss_timer_cur_tick((*dp).timer);
        if (*q).sync_time >= time {
            return 0; /* already finished */
        }

        if (*q).sync_event_put == 0 {
            let mut ev: snd_seq_event = core::mem::zeroed();
            let rec: *mut evrec;

            /* put echoback event */
            memset(
                &mut ev as *mut snd_seq_event as *mut c_void,
                0,
                core::mem::size_of::<snd_seq_event>(),
            );
            ev.flags = 0;
            ev.type_ = SNDRV_SEQ_EVENT_ECHO;
            ev.time.tick = time;
            /* echo back to itself */
            snd_seq_oss_fill_addr(dp, &mut ev, (*dp).addr.client, (*dp).addr.port);
            rec = &mut ev.data as *mut snd_seq_event_data as *mut evrec;
            (*rec).t.code = SEQ_SYNCTIMER;
            (*rec).t.time = time;
            (*q).sync_event_put = 1;
            snd_seq_kernel_client_enqueue((*dp).cseq, &mut ev, core::ptr::null_mut(), true);
        }

        wait_event_interruptible_timeout(&mut (*q).sync_sleep, (*q).sync_event_put == 0, HZ);
        if signal_pending(current) != 0 {
            /* interrupted - return 0 to finish sync */
            (*q).sync_event_put = 0;
        }
        if (*q).sync_event_put == 0 || (*q).sync_time >= time {
            return 0;
        }
    }
    1
}

/*
 * wake up sync - echo event was catched
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_wakeup(q: *mut seq_oss_writeq, time: abstime_t) {
    unsafe {
        let flags = spin_lock_irqsave(&mut (*q).sync_lock);
        (*q).sync_time = time;
        (*q).sync_event_put = 0;
        spin_unlock_irqrestore(&mut (*q).sync_lock, flags);
        wake_up(&mut (*q).sync_sleep);
    }
}

/*
 * return the unused pool size
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_get_free_size(q: *mut seq_oss_writeq) -> c_int {
    let mut pool: snd_seq_client_pool = unsafe { core::mem::zeroed() };
    unsafe {
        pool.client = (*(*q).dp).cseq;
        snd_seq_oss_control(
            (*q).dp,
            SNDRV_SEQ_IOCTL_GET_CLIENT_POOL,
            &mut pool as *mut snd_seq_client_pool as *mut c_void,
        );
    }
    pool.output_free
}

/*
 * set output threshold size from ioctl
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_seq_oss_writeq_set_output(q: *mut seq_oss_writeq, val: c_int) {
    let mut pool: snd_seq_client_pool = unsafe { core::mem::zeroed() };
    unsafe {
        pool.client = (*(*q).dp).cseq;
        snd_seq_oss_control(
            (*q).dp,
            SNDRV_SEQ_IOCTL_GET_CLIENT_POOL,
            &mut pool as *mut snd_seq_client_pool as *mut c_void,
        );
        pool.output_room = val;
        snd_seq_oss_control(
            (*q).dp,
            SNDRV_SEQ_IOCTL_SET_CLIENT_POOL,
            &mut pool as *mut snd_seq_client_pool as *mut c_void,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
