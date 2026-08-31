/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of the generated Buchi automaton header.
 *
 * Original C dependency: #include <linux/rv.h>
 */

use core::ffi::{c_char, c_uint, c_ulong};

pub const MONITOR_NAME: &str = "ltl_pertask";

unsafe extern "C" {
    pub static RV_MAX_LTL_ATOM: u32;
    pub static RV_MAX_BA_STATES: u32;

    pub fn test_bit(nr: c_ulong, addr: *const c_ulong) -> bool;
    pub fn __set_bit(nr: c_ulong, addr: *mut c_ulong);
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ltl_monitor {
    pub atoms: *mut c_ulong,
    pub states: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ltl_atom {
    LTL_EVENT_A,
    LTL_EVENT_B,
    LTL_NUM_ATOM,
}

const _: () = {
    /* C static_assert(LTL_NUM_ATOM <= RV_MAX_LTL_ATOM) depends on linux/rv.h. */
};

pub unsafe fn ltl_atom_str(atom: ltl_atom) -> *const c_char {
    static EV_A: &[u8] = b"ev_a\0";
    static EV_B: &[u8] = b"ev_b\0";
    static NAMES: [*const c_char; 2] = [
        EV_A.as_ptr() as *const c_char,
        EV_B.as_ptr() as *const c_char,
    ];

    unsafe { *NAMES.get_unchecked(atom as usize) }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ltl_buchi_state {
    S0,
    S1,
    S2,
    S3,
    S4,
    RV_NUM_BA_STATES,
}

const _: () = {
    /* C static_assert(RV_NUM_BA_STATES <= RV_MAX_BA_STATES) depends on linux/rv.h. */
};

pub unsafe fn ltl_start(_task: *mut task_struct, mon: *mut ltl_monitor) {
    let event_b = unsafe {
        test_bit(
            ltl_atom::LTL_EVENT_B as c_ulong,
            (*mon).atoms as *const c_ulong,
        )
    };
    let event_a = unsafe {
        test_bit(
            ltl_atom::LTL_EVENT_A as c_ulong,
            (*mon).atoms as *const c_ulong,
        )
    };
    let val1 = !event_a;

    if val1 {
        unsafe { __set_bit(ltl_buchi_state::S0 as c_ulong, (*mon).states) };
    }
    if true {
        unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, (*mon).states) };
    }
    if event_b {
        unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, (*mon).states) };
    }
}

pub unsafe fn ltl_possible_next_states(
    mon: *mut ltl_monitor,
    state: c_uint,
    next: *mut c_ulong,
) {
    let event_b = unsafe {
        test_bit(
            ltl_atom::LTL_EVENT_B as c_ulong,
            (*mon).atoms as *const c_ulong,
        )
    };
    let event_a = unsafe {
        test_bit(
            ltl_atom::LTL_EVENT_A as c_ulong,
            (*mon).atoms as *const c_ulong,
        )
    };
    let val1 = !event_a;

    match state {
        x if x == ltl_buchi_state::S0 as c_uint => {
            if val1 {
                unsafe { __set_bit(ltl_buchi_state::S0 as c_ulong, next) };
            }
            if true {
                unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, next) };
            }
            if event_b {
                unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, next) };
            }
        }
        x if x == ltl_buchi_state::S1 as c_uint => {
            if true {
                unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, next) };
            }
            if true && val1 {
                unsafe { __set_bit(ltl_buchi_state::S2 as c_ulong, next) };
            }
            if event_b && val1 {
                unsafe { __set_bit(ltl_buchi_state::S3 as c_ulong, next) };
            }
            if event_b {
                unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, next) };
            }
        }
        x if x == ltl_buchi_state::S2 as c_uint => {
            if true {
                unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, next) };
            }
            if true && val1 {
                unsafe { __set_bit(ltl_buchi_state::S2 as c_ulong, next) };
            }
            if event_b && val1 {
                unsafe { __set_bit(ltl_buchi_state::S3 as c_ulong, next) };
            }
            if event_b {
                unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, next) };
            }
        }
        x if x == ltl_buchi_state::S3 as c_uint => {
            if val1 {
                unsafe { __set_bit(ltl_buchi_state::S0 as c_ulong, next) };
            }
            if true {
                unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, next) };
            }
            if event_b {
                unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, next) };
            }
        }
        x if x == ltl_buchi_state::S4 as c_uint => {
            if val1 {
                unsafe { __set_bit(ltl_buchi_state::S0 as c_ulong, next) };
            }
            if true {
                unsafe { __set_bit(ltl_buchi_state::S1 as c_ulong, next) };
            }
            if event_b {
                unsafe { __set_bit(ltl_buchi_state::S4 as c_ulong, next) };
            }
        }
        _ => {}
    }
}
