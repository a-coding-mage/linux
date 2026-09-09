/* SPDX-License-Identifier: GPL-2.0 */
/*
 * tick internal variable and functions used by low/high res code
 */
// Dependencies supplied by the surrounding kernel translation:
// linux/hrtimer.h, linux/hrtimer_bases.h, linux/tick.h,
// timekeeping.h, and tick-sched.h

#[repr(C)]
pub struct timer_events {
    pub local: u64,
    pub global: u64,
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
pub const TICK_DO_TIMER_NONE: i32 = -1;
#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
pub const TICK_DO_TIMER_BOOT: i32 = -2;

#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
extern "C" {
    pub static mut tick_cpu_device: [tick_device; 0]; // DECLARE_PER_CPU
    pub static mut tick_next_period: ktime_t;
    pub static mut tick_do_timer_cpu: i32;

    pub fn tick_setup_periodic(dev: *mut clock_event_device, broadcast: i32);
    pub fn tick_handle_periodic(dev: *mut clock_event_device);
    pub fn tick_check_new_device(dev: *mut clock_event_device);
    pub fn tick_offline_cpu(cpu: u32);
    pub fn tick_shutdown();
    pub fn tick_suspend();
    pub fn tick_resume();
    pub fn tick_check_replacement(
        curdev: *mut clock_event_device,
        newdev: *mut clock_event_device,
    ) -> bool;
    pub fn tick_install_replacement(dev: *mut clock_event_device);
    pub fn tick_is_oneshot_available() -> i32;
    pub fn tick_get_device(cpu: i32) -> *mut tick_device;

    pub fn clockevents_tick_resume(dev: *mut clock_event_device) -> i32;
    pub fn clockevents_shutdown(dev: *mut clock_event_device);
    pub fn clockevents_exchange_device(
        old: *mut clock_event_device,
        new: *mut clock_event_device,
    );
    pub fn clockevents_switch_state(
        dev: *mut clock_event_device,
        state: clock_event_state,
    );
    pub fn clockevents_program_event(
        dev: *mut clock_event_device,
        expires: ktime_t,
        force: bool,
    ) -> i32;
    pub fn clockevents_handle_noop(dev: *mut clock_event_device);
    pub fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> i32;
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
#[inline]
pub unsafe fn tick_device_is_functional(dev: *mut clock_event_device) -> i32 {
    (!((*dev).features & CLOCK_EVT_FEAT_DUMMY != 0)) as i32
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
#[inline]
pub unsafe fn clockevent_get_state(dev: *mut clock_event_device) -> clock_event_state {
    (*dev).state_use_accessors
}

#[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
#[inline]
pub unsafe fn clockevent_set_state(dev: *mut clock_event_device, state: clock_event_state) {
    (*dev).state_use_accessors = state;
}

#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, CONFIG_GENERIC_CLOCKEVENTS_BROADCAST))]
extern "C" {
    pub fn tick_device_uses_broadcast(dev: *mut clock_event_device, cpu: i32) -> i32;
    pub fn tick_install_broadcast_device(dev: *mut clock_event_device, cpu: i32);
    pub fn tick_is_broadcast_device(dev: *mut clock_event_device) -> i32;
    pub fn tick_suspend_broadcast();
    pub fn tick_resume_broadcast();
    pub fn tick_resume_check_broadcast() -> bool;
    pub fn tick_broadcast_init();
    pub fn tick_set_periodic_handler(dev: *mut clock_event_device, broadcast: i32);
    pub fn tick_broadcast_update_freq(dev: *mut clock_event_device, freq: u32) -> i32;
    pub fn tick_get_broadcast_device() -> *mut tick_device;
    pub fn tick_get_broadcast_mask() -> *mut cpumask;
    pub fn tick_get_wakeup_device(cpu: i32) -> *const clock_event_device;
}

#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_install_broadcast_device(_: *mut clock_event_device, _: i32) {}
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_is_broadcast_device(_: *mut clock_event_device) -> i32 { 0 }
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_device_uses_broadcast(_: *mut clock_event_device, _: i32) -> i32 { 0 }
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_do_periodic_broadcast(_: *mut clock_event_device) {}
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_suspend_broadcast() {}
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_resume_broadcast() {}
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_resume_check_broadcast() -> bool { false }
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_broadcast_init() {}
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline] pub unsafe fn tick_broadcast_update_freq(_: *mut clock_event_device, _: u32) -> i32 { -ENODEV }
#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS, not(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)))]
#[inline]
pub unsafe fn tick_set_periodic_handler(dev: *mut clock_event_device, _: i32) {
    (*dev).event_handler = Some(tick_handle_periodic);
}

#[cfg(not(CONFIG_GENERIC_CLOCKEVENTS))]
#[inline] pub unsafe fn tick_suspend() {}
#[cfg(not(CONFIG_GENERIC_CLOCKEVENTS))]
#[inline] pub unsafe fn tick_resume() {}

#[cfg(CONFIG_TICK_ONESHOT)]
extern "C" {
    pub fn tick_setup_oneshot(dev: *mut clock_event_device, handler: Option<unsafe extern "C" fn(*mut clock_event_device)>, nextevt: ktime_t);
    pub fn tick_program_event(expires: ktime_t, force: i32) -> i32;
    pub fn tick_oneshot_notify();
    pub fn tick_switch_to_oneshot(handler: Option<unsafe extern "C" fn(*mut clock_event_device)>) -> i32;
    pub fn tick_resume_oneshot();
    pub fn tick_oneshot_mode_active() -> i32;
    pub fn tick_clock_notify();
    pub fn tick_check_oneshot_change(allow_nohz: i32) -> i32;
    pub fn tick_init_highres() -> i32;
}
#[cfg(CONFIG_TICK_ONESHOT)]
#[inline] pub fn tick_oneshot_possible() -> bool { true }

#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub unsafe fn tick_setup_oneshot(_: *mut clock_event_device, _: Option<unsafe extern "C" fn(*mut clock_event_device)>, _: ktime_t) { BUG() }
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub unsafe fn tick_resume_oneshot() { BUG() }
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_program_event(_: ktime_t, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_oneshot_notify() {}
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_oneshot_possible() -> bool { false }
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_oneshot_mode_active() -> i32 { 0 }
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_clock_notify() {}
#[cfg(not(CONFIG_TICK_ONESHOT))]
#[inline] pub fn tick_check_oneshot_change(_: i32) -> i32 { 0 }

#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_TICK_ONESHOT))]
extern "C" {
    pub fn tick_broadcast_switch_to_oneshot();
    pub fn tick_broadcast_oneshot_active() -> i32;
    pub fn tick_check_oneshot_broadcast_this_cpu();
    pub fn tick_broadcast_oneshot_available() -> bool;
    pub fn tick_get_broadcast_oneshot_mask() -> *mut cpumask;
}
#[cfg(not(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_TICK_ONESHOT)))]
#[inline] pub fn tick_broadcast_switch_to_oneshot() {}
#[cfg(not(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_TICK_ONESHOT)))]
#[inline] pub fn tick_broadcast_oneshot_active() -> i32 { 0 }
#[cfg(not(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_TICK_ONESHOT)))]
#[inline] pub fn tick_check_oneshot_broadcast_this_cpu() {}
#[cfg(not(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_TICK_ONESHOT)))]
#[inline] pub fn tick_broadcast_oneshot_available() -> bool { tick_oneshot_possible() }

#[cfg(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_HOTPLUG_CPU))]
extern "C" { pub fn tick_broadcast_offline(cpu: u32); }
#[cfg(not(all(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST, CONFIG_HOTPLUG_CPU)))]
#[inline] pub fn tick_broadcast_offline(_: u32) {}

#[cfg(CONFIG_NO_HZ_FULL)]
extern "C" { pub fn tick_nohz_init(); }
#[cfg(not(CONFIG_NO_HZ_FULL))]
#[inline] pub fn tick_nohz_init() {}

#[cfg(CONFIG_NO_HZ_COMMON)]
extern "C" {
    pub fn timers_update_nohz();
    pub fn get_jiffies_update(basej: *mut c_ulong) -> u64;
}
#[cfg(all(CONFIG_NO_HZ_COMMON, CONFIG_SMP))]
extern "C" {
    pub static mut timers_migration_enabled: static_key_false;
    pub fn fetch_next_timer_interrupt(basej: c_ulong, basem: u64, tevt: *mut timer_events, cpu: u32);
    pub fn timer_lock_remote_bases(cpu: u32);
    pub fn timer_unlock_remote_bases(cpu: u32);
    pub fn timer_base_is_idle() -> bool;
    pub fn timer_expire_remote(cpu: u32);
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
#[inline] pub fn timers_update_nohz() {}

extern "C" {
    pub static mut hrtimer_bases: [hrtimer_cpu_base; 0]; // DECLARE_PER_CPU
    pub fn get_next_timer_interrupt(basej: c_ulong, basem: u64) -> u64;
    pub fn timer_base_try_to_set_idle(basej: c_ulong, basem: u64, idle: *mut bool) -> u64;
    pub fn timer_clear_idle();
    pub fn clock_was_set(bases: u32);
    pub fn clock_was_set_delayed();
    pub fn hrtimers_resume_local();
    pub fn sysfs_get_uname(buf: *const c_char, dst: *mut c_char, cnt: usize) -> isize;
}

// CLOCK_SET_WALL and CLOCK_SET_BOOT are composed from HRTIMER_BASE_* values.
pub const CLOCK_SET_WALL: u32 = (1 << HRTIMER_BASE_REALTIME)
    | (1 << HRTIMER_BASE_REALTIME_SOFT)
    | (1 << HRTIMER_BASE_TAI)
    | (1 << HRTIMER_BASE_TAI_SOFT);
pub const CLOCK_SET_BOOT: u32 = (1 << HRTIMER_BASE_BOOTTIME) | (1 << HRTIMER_BASE_BOOTTIME_SOFT);

/* Since jiffies uses a simple TICK_NSEC multiplier
 * conversion, the .shift value could be zero. However
 * this would make NTP adjustments impossible as they are
 * in units of 1/2^.shift. Thus we use JIFFIES_SHIFT to
 * shift both the nominator and denominator the same
 * amount, and give ntp adjustments in units of 1/2^8
 *
 * The value 8 is somewhat carefully chosen, as anything
 * larger can result in overflows. TICK_NSEC grows as HZ
 * shrinks, so values greater than 8 overflow 32bits when
 * HZ=100.
 */
#[cfg(HZ < 34)]
pub const JIFFIES_SHIFT: u32 = 6;
#[cfg(all(HZ >= 34, HZ < 67))]
pub const JIFFIES_SHIFT: u32 = 7;
#[cfg(HZ >= 67)]
pub const JIFFIES_SHIFT: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
