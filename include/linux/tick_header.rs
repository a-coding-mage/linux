/* SPDX-License-Identifier: GPL-2.0 */
/* Tick related global functions */

// C dependencies: linux/clockchips.h, irqflags.h, jiffies.h, ktime.h,
// percpu.h, context_tracking_state.h, cpumask.h, sched.h, rcupdate.h,
// and static_key.h.

#[cfg(feature = "CONFIG_GENERIC_CLOCKEVENTS")]
extern "C" {
    pub fn tick_init();
    pub fn tick_suspend_local();
    pub fn tick_resume_local();
}
#[cfg(not(feature = "CONFIG_GENERIC_CLOCKEVENTS"))]
pub unsafe fn tick_init() {}
#[cfg(not(feature = "CONFIG_GENERIC_CLOCKEVENTS"))]
pub unsafe fn tick_suspend_local() {}
#[cfg(not(feature = "CONFIG_GENERIC_CLOCKEVENTS"))]
pub unsafe fn tick_resume_local() {}

#[cfg(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_HOTPLUG_CPU"))]
extern "C" {
    pub fn tick_cpu_dying(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn tick_assert_timekeeping_handover();
}
#[cfg(not(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_HOTPLUG_CPU")))]
pub const tick_cpu_dying: Option<unsafe extern "C" fn(::core::ffi::c_uint) -> ::core::ffi::c_int> = None;
#[cfg(not(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_HOTPLUG_CPU")))]
pub unsafe fn tick_assert_timekeeping_handover() {}

#[cfg(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_SUSPEND"))]
extern "C" { pub fn tick_freeze(); pub fn tick_unfreeze(); }
#[cfg(not(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_SUSPEND")))]
pub unsafe fn tick_freeze() {}
#[cfg(not(all(feature = "CONFIG_GENERIC_CLOCKEVENTS", feature = "CONFIG_SUSPEND")))]
pub unsafe fn tick_unfreeze() {}

#[cfg(feature = "CONFIG_TICK_ONESHOT")]
extern "C" { pub fn tick_irq_enter(); }
#[cfg(not(feature = "CONFIG_TICK_ONESHOT"))]
pub unsafe fn tick_irq_enter() {}

#[cfg(all(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST", feature = "CONFIG_TICK_ONESHOT"))]
extern "C" { pub fn hotplug_cpu__broadcast_tick_pull(dead_cpu: ::core::ffi::c_int); }
#[cfg(not(all(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST", feature = "CONFIG_TICK_ONESHOT")))]
pub unsafe fn hotplug_cpu__broadcast_tick_pull(_dead_cpu: ::core::ffi::c_int) {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tick_broadcast_mode { TICK_BROADCAST_OFF, TICK_BROADCAST_ON, TICK_BROADCAST_FORCE }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tick_broadcast_state { TICK_BROADCAST_EXIT, TICK_BROADCAST_ENTER }

extern "C" { pub static mut arch_needs_tick_broadcast: static_key_false; }

#[cfg(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST")]
extern "C" { pub fn tick_broadcast_control(mode: tick_broadcast_mode); }
#[cfg(not(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST"))]
pub unsafe fn tick_broadcast_control(_mode: tick_broadcast_mode) {}

#[cfg(feature = "CONFIG_GENERIC_CLOCKEVENTS")]
extern "C" { pub fn tick_broadcast_oneshot_control(state: tick_broadcast_state) -> ::core::ffi::c_int; }
#[cfg(not(feature = "CONFIG_GENERIC_CLOCKEVENTS"))]
pub unsafe fn tick_broadcast_oneshot_control(_state: tick_broadcast_state) -> ::core::ffi::c_int { 0 }

pub unsafe fn tick_broadcast_enable() { tick_broadcast_control(tick_broadcast_mode::TICK_BROADCAST_ON); }
pub unsafe fn tick_broadcast_disable() { tick_broadcast_control(tick_broadcast_mode::TICK_BROADCAST_OFF); }
pub unsafe fn tick_broadcast_force() { tick_broadcast_control(tick_broadcast_mode::TICK_BROADCAST_FORCE); }
pub unsafe fn tick_broadcast_enter() -> ::core::ffi::c_int { tick_broadcast_oneshot_control(tick_broadcast_state::TICK_BROADCAST_ENTER) }
pub unsafe fn tick_broadcast_exit() { tick_broadcast_oneshot_control(tick_broadcast_state::TICK_BROADCAST_EXIT); }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tick_dep_bits {
    TICK_DEP_BIT_POSIX_TIMER = 0,
    TICK_DEP_BIT_PERF_EVENTS = 1,
    TICK_DEP_BIT_SCHED = 2,
    TICK_DEP_BIT_CLOCK_UNSTABLE = 3,
    TICK_DEP_BIT_RCU = 4,
    TICK_DEP_BIT_RCU_EXP = 5,
}
pub const TICK_DEP_BIT_MAX: tick_dep_bits = tick_dep_bits::TICK_DEP_BIT_RCU_EXP;
pub const TICK_DEP_MASK_NONE: u32 = 0;
pub const TICK_DEP_MASK_POSIX_TIMER: u32 = 1 << 0;
pub const TICK_DEP_MASK_PERF_EVENTS: u32 = 1 << 1;
pub const TICK_DEP_MASK_SCHED: u32 = 1 << 2;
pub const TICK_DEP_MASK_CLOCK_UNSTABLE: u32 = 1 << 3;
pub const TICK_DEP_MASK_RCU: u32 = 1 << 4;
pub const TICK_DEP_MASK_RCU_EXP: u32 = 1 << 5;

// The remaining declarations depend on kernel types and configuration supplied by other headers.
// They are intentionally retained as source-level declarations below.
#[cfg(feature = "CONFIG_NO_HZ_COMMON")]
extern "C" {
    pub static mut tick_nohz_enabled: bool;
    pub fn tick_nohz_is_active() -> bool;
    pub fn tick_nohz_tick_stopped() -> bool;
    pub fn tick_nohz_tick_stopped_cpu(cpu: ::core::ffi::c_int) -> bool;
    pub fn tick_nohz_idle_stop_tick(); pub fn tick_nohz_idle_retain_tick(); pub fn tick_nohz_idle_restart_tick();
    pub fn tick_nohz_idle_enter(); pub fn tick_nohz_idle_exit(); pub fn tick_nohz_irq_exit();
    pub fn tick_nohz_idle_got_tick() -> bool;
    pub fn tick_nohz_get_next_hrtimer() -> ktime_t;
    pub fn tick_nohz_get_sleep_length(delta_next: *mut ktime_t) -> ktime_t;
    pub fn tick_nohz_get_idle_calls_cpu(cpu: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub const tick_nohz_enabled: i32 = 0;
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_is_active() -> bool { false }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_tick_stopped() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_tick_stopped_cpu(_cpu: ::core::ffi::c_int) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_stop_tick() {}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_retain_tick() {}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_restart_tick() {}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_enter() {}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_exit() {}
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_idle_got_tick() -> bool { false }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_get_next_hrtimer() -> ktime_t { ktime_add(ktime_get(), TICK_NSEC) }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn tick_nohz_get_sleep_length(delta_next: *mut ktime_t) -> ktime_t { *delta_next = TICK_NSEC; *delta_next }

extern "C" {
    pub static mut tick_nohz_full_mask: cpumask_var_t;
}

// CONFIG_NO_HZ_FULL-dependent declarations and wrappers retain the original kernel interfaces.
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
extern "C" {
    pub static mut tick_nohz_full_running: bool;
    pub fn tick_nohz_full_enabled() -> bool;
    pub fn tick_nohz_full_cpu(cpu: ::core::ffi::c_int) -> bool;
    pub fn tick_nohz_dep_set(bit: tick_dep_bits); pub fn tick_nohz_dep_clear(bit: tick_dep_bits);
    pub fn tick_nohz_dep_set_cpu(cpu: ::core::ffi::c_int, bit: tick_dep_bits);
    pub fn tick_nohz_dep_clear_cpu(cpu: ::core::ffi::c_int, bit: tick_dep_bits);
    pub fn tick_nohz_dep_set_task(tsk: *mut task_struct, bit: tick_dep_bits);
    pub fn tick_nohz_dep_clear_task(tsk: *mut task_struct, bit: tick_dep_bits);
    pub fn tick_nohz_dep_set_signal(tsk: *mut task_struct, bit: tick_dep_bits);
    pub fn tick_nohz_dep_clear_signal(signal: *mut signal_struct, bit: tick_dep_bits);
    pub fn tick_nohz_cpu_hotpluggable(cpu: ::core::ffi::c_uint) -> bool;
    pub fn tick_nohz_full_kick_cpu(cpu: ::core::ffi::c_int); pub fn __tick_nohz_task_switch();
    pub fn tick_nohz_full_setup(cpumask: cpumask_var_t);
}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_nohz_full_enabled() -> bool { false }
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_nohz_full_cpu(_cpu: ::core::ffi::c_int) -> bool { false }
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_nohz_full_kick_cpu(_cpu: ::core::ffi::c_int) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn __tick_nohz_task_switch() {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_nohz_full_setup(_cpumask: cpumask_var_t) {}

#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_set(bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_set(bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_clear(bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_clear(bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_set_cpu(cpu: ::core::ffi::c_int, bit: tick_dep_bits) { if tick_nohz_full_cpu(cpu) { tick_nohz_dep_set_cpu(cpu, bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_clear_cpu(cpu: ::core::ffi::c_int, bit: tick_dep_bits) { if tick_nohz_full_cpu(cpu) { tick_nohz_dep_clear_cpu(cpu, bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_set_task(tsk: *mut task_struct, bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_set_task(tsk, bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_clear_task(tsk: *mut task_struct, bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_clear_task(tsk, bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_set_signal(tsk: *mut task_struct, bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_set_signal(tsk, bit); } }
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_clear_signal(signal: *mut signal_struct, bit: tick_dep_bits) { if tick_nohz_full_enabled() { tick_nohz_dep_clear_signal(signal, bit); } }
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_nohz_cpu_hotpluggable(_cpu: ::core::ffi::c_uint) -> bool { true }
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_set(_bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_clear(_bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_set_cpu(_cpu: ::core::ffi::c_int, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_clear_cpu(_cpu: ::core::ffi::c_int, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_set_task(_tsk: *mut task_struct, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_clear_task(_tsk: *mut task_struct, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_set_signal(_tsk: *mut task_struct, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_clear_signal(_signal: *mut signal_struct, _bit: tick_dep_bits) {}
#[cfg(not(feature = "CONFIG_NO_HZ_FULL"))]
pub unsafe fn tick_dep_init_task(_tsk: *mut task_struct) {}
#[cfg(feature = "CONFIG_NO_HZ_FULL")]
pub unsafe fn tick_dep_init_task(tsk: *mut task_struct) { atomic_set(&mut (*tsk).tick_dep_mask, 0); }

pub unsafe fn tick_nohz_task_switch() { if tick_nohz_full_enabled() { __tick_nohz_task_switch(); } }
pub unsafe fn tick_nohz_user_enter_prepare() {
    if tick_nohz_full_cpu(smp_processor_id()) { rcu_nocb_flush_deferred_wakeup(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
