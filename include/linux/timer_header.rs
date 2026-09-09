/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// referenced by name rather than redefined here.

#[cfg(CONFIG_LOCKDEP)]
// The C initializer copies the lockdep map and uses FILE_LINE as its key/name.
macro_rules! __TIMER_LOCKDEP_MAP_INITIALIZER {
    ($kn:expr) => { .lockdep_map = STATIC_LOCKDEP_MAP_INIT!($kn, &$kn), };
}

pub const TIMER_CPUMASK: u32 = 0x0003FFFF;
pub const TIMER_MIGRATING: u32 = 0x00040000;
pub const TIMER_BASEMASK: u32 = TIMER_CPUMASK | TIMER_MIGRATING;
pub const TIMER_DEFERRABLE: u32 = 0x00080000;
pub const TIMER_PINNED: u32 = 0x00100000;
pub const TIMER_IRQSAFE: u32 = 0x00200000;
pub const TIMER_INIT_FLAGS: u32 = TIMER_DEFERRABLE | TIMER_PINNED | TIMER_IRQSAFE;
pub const TIMER_ARRAYSHIFT: u32 = 22;
pub const TIMER_ARRAYMASK: u32 = 0xFFC00000;
pub const TIMER_TRACE_FLAGMASK: u32 =
    TIMER_MIGRATING | TIMER_DEFERRABLE | TIMER_PINNED | TIMER_IRQSAFE;

// C macro: __TIMER_INITIALIZER(_function, _flags)
macro_rules! __TIMER_INITIALIZER {
    ($function:expr, $flags:expr) => {
        timer_list {
            entry: timer_entry { next: TIMER_ENTRY_STATIC },
            function: $function,
            flags: $flags,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

// C macro: DEFINE_TIMER(_name, _function)
macro_rules! DEFINE_TIMER {
    ($name:ident, $function:expr) => {
        static mut $name: timer_list = __TIMER_INITIALIZER!($function, 0);
    };
}

extern "C" {
    pub fn timer_init_key(
        timer: *mut timer_list,
        func: Option<unsafe extern "C" fn(*mut timer_list)>,
        flags: u32,
        name: *const core::ffi::c_char,
        key: *mut lock_class_key,
    );

    #[cfg(CONFIG_DEBUG_OBJECTS_TIMERS)]
    pub fn timer_init_key_on_stack(
        timer: *mut timer_list,
        func: Option<unsafe extern "C" fn(*mut timer_list)>,
        flags: u32,
        name: *const core::ffi::c_char,
        key: *mut lock_class_key,
    );

    pub fn add_timer_on(timer: *mut timer_list, cpu: i32);
    pub fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> i32;
    pub fn mod_timer_pending(timer: *mut timer_list, expires: c_ulong) -> i32;
    pub fn timer_reduce(timer: *mut timer_list, expires: c_ulong) -> i32;
    pub fn add_timer(timer: *mut timer_list);
    pub fn add_timer_local(timer: *mut timer_list);
    pub fn add_timer_global(timer: *mut timer_list);
    pub fn timer_delete_sync_try(timer: *mut timer_list) -> i32;
    pub fn timer_delete_sync(timer: *mut timer_list) -> i32;
    pub fn timer_delete(timer: *mut timer_list) -> i32;
    pub fn timer_shutdown_sync(timer: *mut timer_list) -> i32;
    pub fn timer_shutdown(timer: *mut timer_list) -> i32;
    pub fn timers_init();
    pub fn it_real_fn(timer: *mut hrtimer) -> hrtimer_restart;
    pub fn __round_jiffies_relative(j: c_ulong, cpu: i32) -> c_ulong;
    pub fn round_jiffies(j: c_ulong) -> c_ulong;
    pub fn round_jiffies_relative(j: c_ulong) -> c_ulong;
    pub fn __round_jiffies_up_relative(j: c_ulong, cpu: i32) -> c_ulong;
    pub fn round_jiffies_up(j: c_ulong) -> c_ulong;
    pub fn round_jiffies_up_relative(j: c_ulong) -> c_ulong;
}

#[cfg(CONFIG_DEBUG_OBJECTS_TIMERS)]
extern "C" {
    pub fn timer_destroy_on_stack(timer: *mut timer_list);
}

#[cfg(not(CONFIG_DEBUG_OBJECTS_TIMERS))]
#[inline]
pub unsafe fn timer_destroy_on_stack(_timer: *mut timer_list) {}

#[inline]
pub unsafe fn timer_init_key_on_stack(
    timer: *mut timer_list,
    func: Option<unsafe extern "C" fn(*mut timer_list)>,
    flags: u32,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
) {
    timer_init_key(timer, func, flags, name, key);
}

macro_rules! timer_setup { ($timer:expr, $callback:expr, $flags:expr) => { __timer_init!($timer, $callback, $flags) }; }
macro_rules! timer_setup_on_stack { ($timer:expr, $callback:expr, $flags:expr) => { __timer_init_on_stack!($timer, $callback, $flags) }; }

macro_rules! __timer_init {
    ($timer:expr, $fn:expr, $flags:expr) => { unsafe { timer_init_key($timer, Some($fn), $flags, core::ptr::null(), core::ptr::null_mut()) } };
}
macro_rules! __timer_init_on_stack {
    ($timer:expr, $fn:expr, $flags:expr) => { unsafe { timer_init_key_on_stack($timer, Some($fn), $flags, core::ptr::null(), core::ptr::null_mut()) } };
}

#[inline]
pub unsafe fn timer_pending(timer: *const timer_list) -> i32 {
    (!hlist_unhashed_lockless!((*timer).entry)).into()
}

pub const TIMER_NEXT_MAX_DELTA: c_ulong = (1 as c_ulong << 30) - 1;

#[cfg(CONFIG_HOTPLUG_CPU)]
extern "C" {
    pub fn timers_prepare_cpu(cpu: u32) -> i32;
    pub fn timers_dead_cpu(cpu: u32) -> i32;
}

#[cfg(not(CONFIG_HOTPLUG_CPU))]
pub const timers_prepare_cpu: Option<unsafe extern "C" fn()> = None;
#[cfg(not(CONFIG_HOTPLUG_CPU))]
pub const timers_dead_cpu: Option<unsafe extern "C" fn()> = None;

#[cfg(all(CONFIG_SMP, CONFIG_NO_HZ_COMMON))]
extern "C" { pub fn tmigr_isolated_exclude_cpumask(exclude_cpumask: *mut cpumask) -> i32; }
#[cfg(not(all(CONFIG_SMP, CONFIG_NO_HZ_COMMON)))]
#[inline]
pub unsafe fn tmigr_isolated_exclude_cpumask(_exclude_cpumask: *mut cpumask) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
