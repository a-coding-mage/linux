// SPDX-License-Identifier: GPL-2.0
/*
 * haltpoll.c - haltpoll idle governor
 *
 * Copyright 2019 Red Hat, Inc. and/or its affiliates.
 *
 * This work is licensed under the terms of version 2 of the GNU GPL.
 */

use core::ffi::c_int;

// Kernel-provided types and functions represented locally for this translation.
#[repr(C)]
pub struct CpuidleDriver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CpuidleDevice {
    pub cpu: c_int,
    pub poll_limit_ns: u32,
    pub poll_time_limit: bool,
    pub last_state_idx: c_int,
    pub last_residency_ns: u64,
}

#[repr(C)]
pub struct CpuidleGovernor {
    pub name: *const u8,
    pub rating: c_int,
    pub enable: Option<unsafe extern "C" fn(*mut CpuidleDriver, *mut CpuidleDevice) -> c_int>,
    pub select: Option<unsafe extern "C" fn(*mut CpuidleDriver, *mut CpuidleDevice, *mut bool) -> c_int>,
    pub reflect: Option<unsafe extern "C" fn(*mut CpuidleDevice, c_int)>,
}

extern "C" {
    fn cpuidle_governor_latency_req(cpu: c_int) -> u64;
    fn trace_guest_halt_poll_ns_grow(new: u32, old: u32);
    fn trace_guest_halt_poll_ns_shrink(new: u32, old: u32);
    fn kvm_para_available() -> bool;
    fn cpuidle_register_governor(governor: *mut CpuidleGovernor) -> c_int;
}

static mut GUEST_HALT_POLL_NS: u32 = 200000;

/* division factor to shrink halt_poll_ns */
static mut GUEST_HALT_POLL_SHRINK: u32 = 2;

/* multiplication factor to grow per-cpu poll_limit_ns */
static mut GUEST_HALT_POLL_GROW: u32 = 2;

/* value in us to start growing per-cpu halt_poll_ns */
static mut GUEST_HALT_POLL_GROW_START: u32 = 50000;

/* allow shrinking guest halt poll */
static mut GUEST_HALT_POLL_ALLOW_SHRINK: bool = true;

unsafe extern "C" fn haltpoll_select(
    _drv: *mut CpuidleDriver,
    dev: *mut CpuidleDevice,
    stop_tick: *mut bool,
) -> c_int {
    if cpuidle_governor_latency_req((*dev).cpu) == 0 {
        *stop_tick = false;
        return 0;
    }

    if (*dev).poll_limit_ns == 0 {
        return 1;
    }

    /* Last state was poll? */
    if (*dev).last_state_idx == 0 {
        /* Halt if no event occurred on poll window */
        if (*dev).poll_time_limit == true {
            return 1;
        }

        *stop_tick = false;
        /* Otherwise, poll again */
        return 0;
    }

    *stop_tick = false;
    /* Last state was halt: poll */
    0
}

unsafe fn adjust_poll_limit(dev: *mut CpuidleDevice, block_ns: u64) {
    let mut val: u32;

    /* Grow cpu_halt_poll_us if
     * cpu_halt_poll_us < block_ns < guest_halt_poll_us
     */
    if block_ns > (*dev).poll_limit_ns as u64 && block_ns <= GUEST_HALT_POLL_NS as u64 {
        val = (*dev).poll_limit_ns.wrapping_mul(GUEST_HALT_POLL_GROW);

        if val < GUEST_HALT_POLL_GROW_START {
            val = GUEST_HALT_POLL_GROW_START;
        }
        if val > GUEST_HALT_POLL_NS {
            val = GUEST_HALT_POLL_NS;
        }

        trace_guest_halt_poll_ns_grow(val, (*dev).poll_limit_ns);
        (*dev).poll_limit_ns = val;
    } else if block_ns > GUEST_HALT_POLL_NS as u64 && GUEST_HALT_POLL_ALLOW_SHRINK {
        let shrink = GUEST_HALT_POLL_SHRINK;

        val = (*dev).poll_limit_ns;
        if shrink == 0 {
            val = 0;
        } else {
            val /= shrink;
            /* Reset value to 0 if shrunk below grow_start */
            if val < GUEST_HALT_POLL_GROW_START {
                val = 0;
            }
        }

        trace_guest_halt_poll_ns_shrink(val, (*dev).poll_limit_ns);
        (*dev).poll_limit_ns = val;
    }
}

unsafe extern "C" fn haltpoll_reflect(dev: *mut CpuidleDevice, index: c_int) {
    (*dev).last_state_idx = index;

    if index != 0 {
        adjust_poll_limit(dev, (*dev).last_residency_ns);
    }
}

unsafe extern "C" fn haltpoll_enable_device(
    _drv: *mut CpuidleDriver,
    dev: *mut CpuidleDevice,
) -> c_int {
    (*dev).poll_limit_ns = 0;
    0
}

static mut HALTPOLL_GOVERNOR: CpuidleGovernor = CpuidleGovernor {
    name: b"haltpoll\0".as_ptr(),
    rating: 9,
    enable: Some(haltpoll_enable_device),
    select: Some(haltpoll_select),
    reflect: Some(haltpoll_reflect),
};

unsafe extern "C" fn init_haltpoll() -> c_int {
    if kvm_para_available() {
        return cpuidle_register_governor(&mut HALTPOLL_GOVERNOR);
    }

    0
}

// Equivalent of postcore_initcall(init_haltpoll).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
