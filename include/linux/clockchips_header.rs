/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/include/linux/clockchips.h. */

/* CONFIG_GENERIC_CLOCKEVENTS-dependent declarations are preserved here
 * unconditionally; feature selection is supplied by the surrounding build. */

pub enum clock_event_state {
    CLOCK_EVT_STATE_DETACHED,
    CLOCK_EVT_STATE_SHUTDOWN,
    CLOCK_EVT_STATE_PERIODIC,
    CLOCK_EVT_STATE_ONESHOT,
    CLOCK_EVT_STATE_ONESHOT_STOPPED,
}

pub const CLOCK_EVT_FEAT_PERIODIC: u32 = 0x000001;
pub const CLOCK_EVT_FEAT_ONESHOT: u32 = 0x000002;
pub const CLOCK_EVT_FEAT_CLOCKSOURCE_COUPLED: u32 = 0x000004;
pub const CLOCK_EVT_FEAT_C3STOP: u32 = 0x000008;
pub const CLOCK_EVT_FEAT_DUMMY: u32 = 0x000010;
pub const CLOCK_EVT_FEAT_DYNIRQ: u32 = 0x000020;
pub const CLOCK_EVT_FEAT_PERCPU: u32 = 0x000040;
pub const CLOCK_EVT_FEAT_HRTIMER: u32 = 0x000080;

#[repr(C)]
pub struct clock_event_device {
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub set_next_ktime: Option<unsafe extern "C" fn(ktime_t, *mut clock_event_device) -> i32>,
    pub set_next_coupled: Option<unsafe extern "C" fn(u64, *mut clock_event_device)>,
    pub next_event: ktime_t,
    pub max_delta_ns: u64,
    pub min_delta_ns: u64,
    pub mult: u32,
    pub shift: u32,
    pub state_use_accessors: clock_event_state,
    pub features: u32,
    pub cs_id: clocksource_ids,
    pub next_event_forced: u32,
    pub retries: usize,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot_stopped: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub broadcast: Option<unsafe extern "C" fn(*const cpumask)>,
    pub suspend: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub resume: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub min_delta_ticks: usize,
    pub max_delta_ticks: usize,
    pub name: *const std::ffi::c_char,
    pub rating: i32,
    pub irq: i32,
    pub bound_on: i32,
    pub cpumask: *const cpumask,
    pub list: list_head,
    pub owner: *mut module,
}

pub unsafe fn clockevent_state_detached(dev: *mut clock_event_device) -> bool { (*dev).state_use_accessors == clock_event_state::CLOCK_EVT_STATE_DETACHED }
pub unsafe fn clockevent_state_shutdown(dev: *mut clock_event_device) -> bool { (*dev).state_use_accessors == clock_event_state::CLOCK_EVT_STATE_SHUTDOWN }
pub unsafe fn clockevent_state_periodic(dev: *mut clock_event_device) -> bool { (*dev).state_use_accessors == clock_event_state::CLOCK_EVT_STATE_PERIODIC }
pub unsafe fn clockevent_state_oneshot(dev: *mut clock_event_device) -> bool { (*dev).state_use_accessors == clock_event_state::CLOCK_EVT_STATE_ONESHOT }
pub unsafe fn clockevent_state_oneshot_stopped(dev: *mut clock_event_device) -> bool { (*dev).state_use_accessors == clock_event_state::CLOCK_EVT_STATE_ONESHOT_STOPPED }

pub unsafe fn div_sc(ticks: usize, nsec: usize, shift: i32) -> usize {
    let mut tmp = (ticks as u64) << shift;
    tmp /= nsec as u64;
    tmp as usize
}

unsafe extern "C" {
    pub fn clockevent_delta2ns(latch: usize, evt: *mut clock_event_device) -> u64;
    pub fn clockevents_register_device(dev: *mut clock_event_device);
    pub fn clockevents_unbind_device(ced: *mut clock_event_device, cpu: i32) -> i32;
    pub fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min_delta: usize, max_delta: usize);
    pub fn clockevents_update_freq(ce: *mut clock_event_device, freq: u32) -> i32;
    pub fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32, from: u64, to: u32, maxsec: u32);
    pub fn clockevents_suspend();
    pub fn clockevents_resume();
    pub fn tick_broadcast(mask: *const cpumask);
    pub fn tick_receive_broadcast() -> i32;
    pub fn tick_setup_hrtimer_broadcast();
    pub fn tick_check_broadcast_expired() -> i32;
}

pub unsafe fn clockevents_calc_mult_shift(ce: *mut clock_event_device, freq: u32, maxsec: u32) {
    clocks_calc_mult_shift(&mut (*ce).mult, &mut (*ce).shift, NSEC_PER_SEC, freq, maxsec);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
