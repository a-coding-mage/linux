// SPDX-License-Identifier: GPL-2.0
/*
 * Emulate a local clock event device via a pseudo clock device.
 */

// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_int;

#[repr(C)]
pub struct hrtimer {
    pub base: *mut hrtimer_base,
}

#[repr(C)]
pub struct hrtimer_base {
    pub cpu_base: *mut hrtimer_cpu_base,
}

#[repr(C)]
pub struct hrtimer_cpu_base {
    pub cpu: c_int,
}

#[repr(C)]
pub struct clock_event_device {
    pub name: *const u8,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub set_next_ktime: Option<unsafe extern "C" fn(ktime_t, *mut clock_event_device) -> c_int>,
    pub features: u32,
    pub rating: c_int,
    pub bound_on: c_int,
    pub min_delta_ns: u64,
    pub max_delta_ns: u64,
    pub min_delta_ticks: u64,
    pub max_delta_ticks: usize,
    pub mult: u32,
    pub shift: u32,
    pub cpumask: *const cpu_mask,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
pub struct cpu_mask {
    _private: [u8; 0],
}

pub type ktime_t = i64;
pub type hrtimer_restart = c_int;

pub const HRTIMER_MODE_ABS_PINNED_HARD: u32 = 0;
pub const HRTIMER_MODE_ABS_HARD: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const HRTIMER_NORESTART: hrtimer_restart = 0;
pub const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 0;
pub const CLOCK_EVT_FEAT_HRTIMER: u32 = 1 << 1;
pub const KTIME_MAX: u64 = i64::MAX as u64;

extern "C" {
    static mut cpu_possible_mask: *const cpu_mask;
    fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_start(timer: *mut hrtimer, expires: ktime_t, mode: u32);
    fn hrtimer_setup(
        timer: *mut hrtimer,
        function: unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart,
        clock_id: u32,
        mode: u32,
    );
    fn clockevents_register_device(dev: *mut clock_event_device);
}

static mut bctimer: hrtimer = hrtimer { base: core::ptr::null_mut() };

unsafe extern "C" fn bc_shutdown(_evt: *mut clock_event_device) -> c_int {
    /*
     * Note, we cannot cancel the timer here as we might
     * run into the following live lock scenario:
     *
     * cpu 0		cpu1
     * lock(broadcast_lock);
     *			hrtimer_interrupt()
     *			bc_handler()
     *			   tick_handle_oneshot_broadcast();
     *			    lock(broadcast_lock);
     * hrtimer_cancel()
     *  wait_for_callback()
     */
    hrtimer_try_to_cancel(&raw mut bctimer);
    0
}

/*
 * This is called from the guts of the broadcast code when the cpu
 * which is about to enter idle has the earliest broadcast timer event.
 */
unsafe extern "C" fn bc_set_next(expires: ktime_t, bc: *mut clock_event_device) -> c_int {
    /*
     * This is called either from enter/exit idle code or from the
     * broadcast handler. In all cases tick_broadcast_lock is held.
     *
     * hrtimer_cancel() cannot be called here neither from the
     * broadcast handler nor from the enter/exit idle code. The idle
     * code can run into the problem described in bc_shutdown() and the
     * broadcast handler cannot wait for itself to complete for obvious
     * reasons.
     *
     * Each caller tries to arm the hrtimer on its own CPU, but if the
     * hrtimer callback function is currently running, then
     * hrtimer_start() cannot move it and the timer stays on the CPU on
     * which it is assigned at the moment.
     */
    hrtimer_start(&raw mut bctimer, expires, HRTIMER_MODE_ABS_PINNED_HARD);
    /*
     * The core tick broadcast mode expects bc->bound_on to be set
     * correctly to prevent a CPU which has the broadcast hrtimer
     * armed from going deep idle.
     *
     * As tick_broadcast_lock is held, nothing can change the cpu
     * base which was just established in hrtimer_start() above. So
     * the below access is safe even without holding the hrtimer
     * base lock.
     */
    (*bc).bound_on = (*(*bctimer.base).cpu_base).cpu;
    0
}

static mut ce_broadcast_hrtimer: clock_event_device = clock_event_device {
    name: b"bc_hrtimer\0".as_ptr(),
    set_state_shutdown: Some(bc_shutdown),
    set_next_ktime: Some(bc_set_next),
    features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_HRTIMER,
    rating: 0,
    bound_on: -1,
    min_delta_ns: 1,
    max_delta_ns: KTIME_MAX,
    min_delta_ticks: 1,
    max_delta_ticks: usize::MAX,
    mult: 1,
    shift: 0,
    cpumask: unsafe { cpu_possible_mask },
    event_handler: None,
};

unsafe extern "C" fn bc_handler(_t: *mut hrtimer) -> hrtimer_restart {
    if let Some(handler) = (*(&raw mut ce_broadcast_hrtimer)).event_handler {
        handler(&raw mut ce_broadcast_hrtimer);
    }
    HRTIMER_NORESTART
}

pub unsafe extern "C" fn tick_setup_hrtimer_broadcast() {
    hrtimer_setup(&raw mut bctimer, bc_handler, CLOCK_MONOTONIC, HRTIMER_MODE_ABS_HARD);
    clockevents_register_device(&raw mut ce_broadcast_hrtimer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
