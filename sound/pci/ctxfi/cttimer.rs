// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCM timer handling on ctxfi
 */

// C dependencies: linux/slab.h, linux/math64.h, linux/moduleparam.h,
// sound/core.h, sound/pcm.h, ctatc.h, cthardware.h, cttimer.h

use core::ffi::c_void;
use core::ptr;

type bool_t = bool;
type u64_t = u64;

const GFP_KERNEL: u32 = 0;
const HZ: u32 = 100;
const IT_INT: u32 = 0;

static mut use_system_timer: bool_t = false;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub period_size: u32,
    pub buffer_size: u32,
    pub rate: u32,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub pointer: unsafe extern "C" fn(*mut snd_pcm_substream) -> u32,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub ops: *mut snd_pcm_ops,
}

#[repr(C)]
pub struct ct_atc_pcm {
    pub substream: *mut snd_pcm_substream,
    pub interrupt: unsafe extern "C" fn(*mut ct_atc_pcm),
}

#[repr(C)]
pub struct hw {
    pub set_timer_tick: Option<unsafe extern "C" fn(*mut hw, i32)>,
    pub set_timer_irq: Option<unsafe extern "C" fn(*mut hw, i32)>,
    pub get_wc: unsafe extern "C" fn(*mut hw) -> u32,
    pub irq_callback_data: *mut c_void,
    pub irq_callback: Option<unsafe extern "C" fn(*mut c_void, u32)>,
}

#[repr(C)]
pub struct ct_atc {
    pub hw: *mut hw,
    pub card: *mut snd_card,
}

#[repr(C)]
struct ct_timer_ops {
    init: Option<unsafe extern "C" fn(*mut ct_timer_instance)>,
    prepare: Option<unsafe extern "C" fn(*mut ct_timer_instance)>,
    start: Option<unsafe extern "C" fn(*mut ct_timer_instance)>,
    stop: Option<unsafe extern "C" fn(*mut ct_timer_instance)>,
    free_instance: Option<unsafe extern "C" fn(*mut ct_timer_instance)>,
    interrupt: Option<unsafe extern "C" fn(*mut ct_timer)>,
    free_global: Option<unsafe extern "C" fn(*mut ct_timer)>,
}

/* timer instance -- assigned to each PCM stream */
#[repr(C)]
pub struct ct_timer_instance {
    lock: spinlock_t,
    timer_base: *mut ct_timer,
    apcm: *mut ct_atc_pcm,
    substream: *mut snd_pcm_substream,
    timer: timer_list,
    instance_list: list_head,
    running_list: list_head,
    position: u32,
    frag_count: u32,
    running: u32,
    need_update: u32,
}

/* timer instance manager */
#[repr(C)]
pub struct ct_timer {
    lock: spinlock_t,      /* global timer lock (for xfitimer) */
    list_lock: spinlock_t, /* lock for instance list */
    atc: *mut ct_atc,
    ops: *const ct_timer_ops,
    instance_head: list_head,
    running_head: list_head,
    wc: u32,           /* current wallclock */
    irq_handling: u32, /* in IRQ handling */
    reprogram: u32,    /* need to reprogram the internval */
    running: u32,      /* global timer running */
}

extern "C" {
    static mut jiffies: u32;

    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *const list_head) -> i32;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: u32);
    fn mod_timer(timer: *mut timer_list, expires: u32) -> i32;
    fn timer_delete(timer: *mut timer_list) -> i32;
    fn timer_delete_sync_try(timer: *mut timer_list) -> i32;
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
}

unsafe fn div_u64(dividend: u64_t, divisor: u32) -> u32 {
    (dividend / divisor as u64_t) as u32
}

/*
 * system-timer-based updates
 */

unsafe extern "C" fn ct_systimer_callback(t: *mut timer_list) {
    /* C used timer_container_of(ti, t, timer) to recover ct_timer_instance. */
    let ti: *mut ct_timer_instance = timer_container_of_ct_timer_instance_timer(t);
    let substream = (*ti).substream;
    let runtime = (*substream).runtime;
    let apcm = (*ti).apcm;
    let period_size = (*runtime).period_size;
    let buffer_size = (*runtime).buffer_size;
    let position: u32;
    let dist: u32;
    let interval: u32;

    position = ((*(*substream).ops).pointer)(substream);
    dist = (position.wrapping_add(buffer_size).wrapping_sub((*ti).position)) % buffer_size;
    if dist >= period_size || position / period_size != (*ti).position / period_size {
        ((*apcm).interrupt)(apcm);
        (*ti).position = position;
    }
    /* Add extra HZ*5/1000 to avoid overrun issue when recording
     * at 8kHz in 8-bit format or at 88kHz in 24-bit format. */
    interval = ((period_size - (position % period_size)) * HZ + ((*runtime).rate - 1))
        / (*runtime).rate
        + HZ * 5 / 1000;
    spin_lock(&mut (*ti).lock);
    if (*ti).running != 0 {
        mod_timer(&mut (*ti).timer, jiffies.wrapping_add(interval));
    }
    spin_unlock(&mut (*ti).lock);
}

unsafe extern "C" fn ct_systimer_init(ti: *mut ct_timer_instance) {
    timer_setup(&mut (*ti).timer, ct_systimer_callback, 0);
}

unsafe extern "C" fn ct_systimer_start(ti: *mut ct_timer_instance) {
    let runtime = (*(*ti).substream).runtime;

    spin_lock(&mut (*ti).lock);
    (*ti).running = 1;
    mod_timer(
        &mut (*ti).timer,
        jiffies.wrapping_add(((*runtime).period_size * HZ + ((*runtime).rate - 1)) / (*runtime).rate),
    );
    spin_unlock(&mut (*ti).lock);
}

unsafe extern "C" fn ct_systimer_stop(ti: *mut ct_timer_instance) {
    spin_lock(&mut (*ti).lock);
    (*ti).running = 0;
    timer_delete(&mut (*ti).timer);
    spin_unlock(&mut (*ti).lock);
}

unsafe extern "C" fn ct_systimer_prepare(ti: *mut ct_timer_instance) {
    ct_systimer_stop(ti);
    timer_delete_sync_try(&mut (*ti).timer);
}

const ct_systimer_ops: ct_timer_ops = ct_timer_ops {
    init: Some(ct_systimer_init),
    free_instance: Some(ct_systimer_prepare),
    prepare: Some(ct_systimer_prepare),
    start: Some(ct_systimer_start),
    stop: Some(ct_systimer_stop),
    interrupt: None,
    free_global: None,
};

/*
 * Handling multiple streams using a global emu20k1 timer irq
 */

const CT_TIMER_FREQ: u32 = 48000;
const MIN_TICKS: u32 = 1;
const MAX_TICKS: i32 = (1 << 13) - 1;

unsafe fn ct_xfitimer_irq_rearm(atimer: *mut ct_timer, mut ticks: i32) {
    let hw = (*(*atimer).atc).hw;
    if ticks > MAX_TICKS {
        ticks = MAX_TICKS;
    }
    ((*hw).set_timer_tick.unwrap())(hw, ticks);
    if (*atimer).running == 0 {
        ((*hw).set_timer_irq.unwrap())(hw, 1);
    }
    (*atimer).running = 1;
}

unsafe fn ct_xfitimer_irq_stop(atimer: *mut ct_timer) {
    if (*atimer).running != 0 {
        let hw = (*(*atimer).atc).hw;
        ((*hw).set_timer_irq.unwrap())(hw, 0);
        ((*hw).set_timer_tick.unwrap())(hw, 0);
        (*atimer).running = 0;
    }
}

unsafe fn ct_xfitimer_get_wc(atimer: *mut ct_timer) -> u32 {
    let hw = (*(*atimer).atc).hw;
    ((*hw).get_wc)(hw)
}

/*
 * reprogram the timer interval;
 * checks the running instance list and determines the next timer interval.
 * also updates the each stream position, returns the number of streams
 * to call snd_pcm_period_elapsed() appropriately
 *
 * call this inside the lock and irq disabled
 */
unsafe fn ct_xfitimer_reprogram(atimer: *mut ct_timer, can_update: i32) -> i32 {
    let mut min_intr: u32 = !0;
    let mut updates: i32 = 0;
    let wc: u32;
    let diff: u32;

    if list_empty(&(*atimer).running_head) != 0 {
        ct_xfitimer_irq_stop(atimer);
        (*atimer).reprogram = 0; /* clear flag */
        return 0;
    }

    wc = ct_xfitimer_get_wc(atimer);
    diff = wc.wrapping_sub((*atimer).wc);
    (*atimer).wc = wc;
    let mut pos_node = (*atimer).running_head.next;
    while pos_node != &mut (*atimer).running_head {
        let ti = container_of_ct_timer_instance_running_list(pos_node);
        if (*ti).frag_count > diff {
            (*ti).frag_count = (*ti).frag_count.wrapping_sub(diff);
        } else {
            let mut pos: u32;
            let period_size: u32;
            let rate: u32;

            period_size = (*(*(*ti).substream).runtime).period_size;
            rate = (*(*(*ti).substream).runtime).rate;
            pos = ((*(*(*ti).substream).ops).pointer)((*ti).substream);
            if pos / period_size != (*ti).position / period_size {
                (*ti).need_update = 1;
                (*ti).position = pos;
                updates += 1;
            }
            pos %= period_size;
            pos = period_size - pos;
            (*ti).frag_count = div_u64(
                (pos as u64_t) * CT_TIMER_FREQ as u64_t + rate as u64_t - 1,
                rate,
            );
        }
        if (*ti).need_update != 0 && can_update == 0 {
            min_intr = 0; /* pending to the next irq */
        }
        if (*ti).frag_count < min_intr {
            min_intr = (*ti).frag_count;
        }
        pos_node = (*pos_node).next;
    }

    if min_intr < MIN_TICKS {
        min_intr = MIN_TICKS;
    }
    ct_xfitimer_irq_rearm(atimer, min_intr as i32);
    (*atimer).reprogram = 0; /* clear flag */
    updates
}

/* look through the instance list and call period_elapsed if needed */
unsafe fn ct_xfitimer_check_period(atimer: *mut ct_timer) {
    spin_lock(&mut (*atimer).list_lock);
    let mut pos_node = (*atimer).instance_head.next;
    while pos_node != &mut (*atimer).instance_head {
        let ti = container_of_ct_timer_instance_instance_list(pos_node);
        if (*ti).running != 0 && (*ti).need_update != 0 {
            (*ti).need_update = 0;
            ((*(*ti).apcm).interrupt)((*ti).apcm);
        }
        pos_node = (*pos_node).next;
    }
    spin_unlock(&mut (*atimer).list_lock);
}

/* Handle timer-interrupt */
unsafe extern "C" fn ct_xfitimer_callback(atimer: *mut ct_timer) {
    let mut update: i32;

    spin_lock(&mut (*atimer).lock);
    (*atimer).irq_handling = 1;
    loop {
        update = ct_xfitimer_reprogram(atimer, 1);
        spin_unlock(&mut (*atimer).lock);
        if update != 0 {
            ct_xfitimer_check_period(atimer);
        }
        spin_lock(&mut (*atimer).lock);
        if (*atimer).reprogram == 0 {
            break;
        }
    }
    (*atimer).irq_handling = 0;
    spin_unlock(&mut (*atimer).lock);
}

unsafe extern "C" fn ct_xfitimer_prepare(ti: *mut ct_timer_instance) {
    (*ti).frag_count = (*(*ti).substream).runtime.as_ref().unwrap().period_size;
    (*ti).running = 0;
    (*ti).need_update = 0;
}

/* start/stop the timer */
unsafe fn ct_xfitimer_update(atimer: *mut ct_timer) {
    spin_lock(&mut (*atimer).lock);
    if (*atimer).irq_handling != 0 {
        /* reached from IRQ handler; let it handle later */
        (*atimer).reprogram = 1;
        spin_unlock(&mut (*atimer).lock);
        return;
    }

    ct_xfitimer_irq_stop(atimer);
    ct_xfitimer_reprogram(atimer, 0);
    spin_unlock(&mut (*atimer).lock);
}

unsafe extern "C" fn ct_xfitimer_start(ti: *mut ct_timer_instance) {
    let atimer = (*ti).timer_base;

    spin_lock(&mut (*atimer).lock);
    if list_empty(&(*ti).running_list) != 0 {
        (*atimer).wc = ct_xfitimer_get_wc(atimer);
    }
    (*ti).running = 1;
    (*ti).need_update = 0;
    list_add(&mut (*ti).running_list, &mut (*atimer).running_head);
    spin_unlock(&mut (*atimer).lock);
    ct_xfitimer_update(atimer);
}

unsafe extern "C" fn ct_xfitimer_stop(ti: *mut ct_timer_instance) {
    let atimer = (*ti).timer_base;

    spin_lock(&mut (*atimer).lock);
    list_del_init(&mut (*ti).running_list);
    (*ti).running = 0;
    spin_unlock(&mut (*atimer).lock);
    ct_xfitimer_update(atimer);
}

unsafe extern "C" fn ct_xfitimer_free_global(atimer: *mut ct_timer) {
    ct_xfitimer_irq_stop(atimer);
}

const ct_xfitimer_ops: ct_timer_ops = ct_timer_ops {
    prepare: Some(ct_xfitimer_prepare),
    start: Some(ct_xfitimer_start),
    stop: Some(ct_xfitimer_stop),
    interrupt: Some(ct_xfitimer_callback),
    free_global: Some(ct_xfitimer_free_global),
    init: None,
    free_instance: None,
};

/*
 * timer instance
 */

#[no_mangle]
pub unsafe extern "C" fn ct_timer_instance_new(
    atimer: *mut ct_timer,
    apcm: *mut ct_atc_pcm,
) -> *mut ct_timer_instance {
    let ti: *mut ct_timer_instance;

    ti = kzalloc(core::mem::size_of::<ct_timer_instance>(), GFP_KERNEL) as *mut ct_timer_instance;
    if ti.is_null() {
        return ptr::null_mut();
    }
    spin_lock_init(&mut (*ti).lock);
    INIT_LIST_HEAD(&mut (*ti).instance_list);
    INIT_LIST_HEAD(&mut (*ti).running_list);
    (*ti).timer_base = atimer;
    (*ti).apcm = apcm;
    (*ti).substream = (*apcm).substream;
    if let Some(init) = (*(*atimer).ops).init {
        init(ti);
    }

    spin_lock(&mut (*atimer).list_lock);
    list_add(&mut (*ti).instance_list, &mut (*atimer).instance_head);
    spin_unlock(&mut (*atimer).list_lock);

    ti
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_prepare(ti: *mut ct_timer_instance) {
    if let Some(prepare) = (*(*(*ti).timer_base).ops).prepare {
        prepare(ti);
    }
    (*ti).position = 0;
    (*ti).running = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_start(ti: *mut ct_timer_instance) {
    let atimer = (*ti).timer_base;
    ((*(*atimer).ops).start.unwrap())(ti);
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_stop(ti: *mut ct_timer_instance) {
    let atimer = (*ti).timer_base;
    ((*(*atimer).ops).stop.unwrap())(ti);
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_instance_free(ti: *mut ct_timer_instance) {
    let atimer = (*ti).timer_base;

    ((*(*atimer).ops).stop.unwrap())(ti); /* to be sure */
    if let Some(free_instance) = (*(*atimer).ops).free_instance {
        free_instance(ti);
    }

    spin_lock(&mut (*atimer).list_lock);
    list_del(&mut (*ti).instance_list);
    spin_unlock(&mut (*atimer).list_lock);

    kfree(ti as *mut c_void);
}

/*
 * timer manager
 */

unsafe extern "C" fn ct_timer_interrupt(data: *mut c_void, status: u32) {
    let timer = data as *mut ct_timer;

    /* Interval timer interrupt */
    if (status & IT_INT) != 0 {
        if let Some(interrupt) = (*(*timer).ops).interrupt {
            interrupt(timer);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_new(atc: *mut ct_atc) -> *mut ct_timer {
    let atimer: *mut ct_timer;
    let hw: *mut hw;

    atimer = kzalloc(core::mem::size_of::<ct_timer>(), GFP_KERNEL) as *mut ct_timer;
    if atimer.is_null() {
        return ptr::null_mut();
    }
    spin_lock_init(&mut (*atimer).lock);
    spin_lock_init(&mut (*atimer).list_lock);
    INIT_LIST_HEAD(&mut (*atimer).instance_head);
    INIT_LIST_HEAD(&mut (*atimer).running_head);
    (*atimer).atc = atc;
    hw = (*atc).hw;
    if !use_system_timer && (*hw).set_timer_irq.is_some() {
        dev_info((*(*atc).card).dev, b"Use xfi-native timer\n\0".as_ptr());
        (*atimer).ops = &ct_xfitimer_ops;
        (*hw).irq_callback_data = atimer as *mut c_void;
        (*hw).irq_callback = Some(ct_timer_interrupt);
    } else {
        dev_info((*(*atc).card).dev, b"Use system timer\n\0".as_ptr());
        (*atimer).ops = &ct_systimer_ops;
    }
    atimer
}

#[no_mangle]
pub unsafe extern "C" fn ct_timer_free(atimer: *mut ct_timer) {
    let hw = (*(*atimer).atc).hw;
    (*hw).irq_callback = None;
    if let Some(free_global) = (*(*atimer).ops).free_global {
        free_global(atimer);
    }
    kfree(atimer as *mut c_void);
}

extern "C" {
    fn timer_container_of_ct_timer_instance_timer(t: *mut timer_list) -> *mut ct_timer_instance;
    fn container_of_ct_timer_instance_running_list(l: *mut list_head) -> *mut ct_timer_instance;
    fn container_of_ct_timer_instance_instance_list(l: *mut list_head) -> *mut ct_timer_instance;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
