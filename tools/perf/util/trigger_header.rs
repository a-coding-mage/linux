// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/trigger.h.
// Original C dependencies: asm/bug.h for WARN_ONCE.

use core::ffi::c_char;
use core::ptr::{read_volatile, write_volatile};

/*
 * Use trigger to model operations which need to be executed when
 * an event (a signal, for example) is observed.
 *
 * States and transits:
 *
 *
 *  OFF--> ON --> READY --(hit)--> HIT
 *                 ^               |
 *                 |            (ready)
 *                 |               |
 *                  \_____________/
 *
 * is_hit and is_ready are two key functions to query the state of
 * a trigger. is_hit means the event already happen; is_ready means the
 * trigger is waiting for the event.
 */

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum trigger_state {
    TRIGGER_ERROR = -2,
    TRIGGER_OFF = -1,
    TRIGGER_ON = 0,
    TRIGGER_READY = 1,
    TRIGGER_HIT = 2,
}

#[repr(C)]
pub struct trigger {
    pub state: trigger_state,
    pub name: *const c_char,
}

// C macro:
// WARN_ONCE(t->state != exp,
//           "trigger '%s' state transist error: %d in %s()\n",
//           t->name, t->state, __func__)
macro_rules! TRIGGER_WARN_ONCE {
    ($t:expr, $exp:expr) => {{
        let __trigger_state = read_volatile(core::ptr::addr_of!((*$t).state));
        WARN_ONCE!(
            __trigger_state != $exp,
            "trigger '%s' state transist error: %d in %s()\n",
            (*$t).name,
            __trigger_state as i32,
            core::concat!(core::module_path!(), "\0").as_ptr() as *const c_char
        );
    }};
}

pub(crate) use TRIGGER_WARN_ONCE;

#[inline]
pub unsafe fn trigger_is_available(t: *mut trigger) -> bool {
    read_volatile(core::ptr::addr_of!((*t).state)) as i32 >= 0
}

#[inline]
pub unsafe fn trigger_is_error(t: *mut trigger) -> bool {
    read_volatile(core::ptr::addr_of!((*t).state)) as i32 <= trigger_state::TRIGGER_ERROR as i32
}

#[inline]
pub unsafe fn trigger_on(t: *mut trigger) {
    TRIGGER_WARN_ONCE!(t, trigger_state::TRIGGER_OFF);
    write_volatile(
        core::ptr::addr_of_mut!((*t).state),
        trigger_state::TRIGGER_ON,
    );
}

#[inline]
pub unsafe fn trigger_ready(t: *mut trigger) {
    if !trigger_is_available(t) {
        return;
    }
    write_volatile(
        core::ptr::addr_of_mut!((*t).state),
        trigger_state::TRIGGER_READY,
    );
}

#[inline]
pub unsafe fn trigger_hit(t: *mut trigger) {
    if !trigger_is_available(t) {
        return;
    }
    TRIGGER_WARN_ONCE!(t, trigger_state::TRIGGER_READY);
    write_volatile(
        core::ptr::addr_of_mut!((*t).state),
        trigger_state::TRIGGER_HIT,
    );
}

#[inline]
pub unsafe fn trigger_off(t: *mut trigger) {
    if !trigger_is_available(t) {
        return;
    }
    write_volatile(
        core::ptr::addr_of_mut!((*t).state),
        trigger_state::TRIGGER_OFF,
    );
}

#[inline]
pub unsafe fn trigger_error(t: *mut trigger) {
    write_volatile(
        core::ptr::addr_of_mut!((*t).state),
        trigger_state::TRIGGER_ERROR,
    );
}

#[inline]
pub unsafe fn trigger_is_ready(t: *mut trigger) -> bool {
    read_volatile(core::ptr::addr_of!((*t).state)) == trigger_state::TRIGGER_READY
}

#[inline]
pub unsafe fn trigger_is_hit(t: *mut trigger) -> bool {
    read_volatile(core::ptr::addr_of!((*t).state)) == trigger_state::TRIGGER_HIT
}

macro_rules! DEFINE_TRIGGER {
    ($n:ident) => {
        static mut $n: trigger = trigger {
            state: trigger_state::TRIGGER_OFF,
            name: core::concat!(core::stringify!($n), "\0").as_ptr() as *const c_char,
        };
    };
}

pub(crate) use DEFINE_TRIGGER;
