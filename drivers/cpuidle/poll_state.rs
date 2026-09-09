// SPDX-License-Identifier: GPL-2.0-only
/*
 * poll_state.c - Polling idle state
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong};

const POLL_IDLE_RELAX_COUNT: c_uint = 200;

const CPUIDLE_NAME_LEN: usize = 16;
const CPUIDLE_DESC_LEN: usize = 32;
const CPUIDLE_FLAG_POLLING: c_uint = 1;

#[repr(C)]
pub struct cpuidle_device {
    pub poll_time_limit: bool,
}

#[repr(C)]
pub struct cpuidle_driver;

#[repr(C)]
pub struct cpuidle_state {
    pub name: [c_char; CPUIDLE_NAME_LEN],
    pub desc: [c_char; CPUIDLE_DESC_LEN],
    pub exit_latency: c_uint,
    pub target_residency: c_uint,
    pub exit_latency_ns: c_uint,
    pub target_residency_ns: c_uint,
    pub power_usage: c_int,
    pub enter: Option<unsafe extern "C" fn(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: c_int,
    ) -> c_int>,
    pub flags: c_uint,
}

extern "C" {
    fn local_clock_noinstr() -> c_ulonglong;
    fn raw_local_irq_enable();
    fn raw_local_irq_disable();
    fn current_set_polling_and_test() -> bool;
    fn cpuidle_poll_time(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> c_ulonglong;
    fn need_resched() -> bool;
    fn cpu_relax();
    fn current_clr_polling();
}

#[repr(C)]
pub struct cpuidle_driver_with_states {
    pub states: [cpuidle_state; 1],
}

unsafe extern "C" fn poll_idle(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    index: c_int,
) -> c_int {
    let time_start: c_ulonglong;

    time_start = local_clock_noinstr();

    (*dev).poll_time_limit = false;

    raw_local_irq_enable();
    if !current_set_polling_and_test() {
        let mut loop_count: c_uint = 0;
        let limit: c_ulonglong;

        limit = cpuidle_poll_time(drv, dev);

        while !need_resched() {
            cpu_relax();
            loop_count = loop_count.wrapping_add(1);
            if loop_count < POLL_IDLE_RELAX_COUNT {
                continue;
            }

            loop_count = 0;
            if local_clock_noinstr().wrapping_sub(time_start) > limit {
                (*dev).poll_time_limit = true;
                break;
            }
        }
    }
    raw_local_irq_disable();

    current_clr_polling();

    index
}

pub unsafe extern "C" fn cpuidle_poll_state_init(drv: *mut cpuidle_driver_with_states) {
    let state: *mut cpuidle_state = &mut (*drv).states[0];

    let name = b"POLL\0";
    (*state).name[..name.len()].copy_from_slice(core::slice::from_raw_parts(
        name.as_ptr() as *const c_char,
        name.len(),
    ));
    let desc = b"CPUIDLE CORE POLL IDLE\0";
    (*state).desc[..desc.len()].copy_from_slice(core::slice::from_raw_parts(
        desc.as_ptr() as *const c_char,
        desc.len(),
    ));
    (*state).exit_latency = 0;
    (*state).target_residency = 0;
    (*state).exit_latency_ns = 0;
    (*state).target_residency_ns = 0;
    (*state).power_usage = -1;
    (*state).enter = Some(poll_idle);
    (*state).flags = CPUIDLE_FLAG_POLLING;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
