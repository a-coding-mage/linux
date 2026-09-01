// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2016 SUSE Software Solutions GmbH
 *           Thomas Renninger <trenn@suse.de>
 */

// Original C code is compiled only when defined(__i386__) || defined(__x86_64__).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MAX_RAPL_ZONES: usize = 10;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const CSTATE_NAME_LEN: usize = 64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const CSTATE_DESC_LEN: usize = 256;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MAX_LINE_LEN: usize = 4096;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const RANGE_MACHINE: c_int = 0;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct powercap_zone {
    pub name: *const c_char,
    pub sys_name: *const c_char,
    pub has_energy_uj: c_int,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate_t {
    pub name: [c_char; CSTATE_NAME_LEN],
    pub desc: [c_char; CSTATE_DESC_LEN],
    pub id: c_int,
    pub range: c_int,
    pub get_count: Option<
        unsafe extern "C" fn(id: c_uint, count: *mut c_ulonglong, cpu: c_uint) -> c_int,
    >,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl cstate_t {
    pub const fn zeroed() -> Self {
        Self {
            name: [0; CSTATE_NAME_LEN],
            desc: [0; CSTATE_DESC_LEN],
            id: 0,
            range: 0,
            get_count: None,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_int,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct cpuidle_monitor {
    pub name: *const c_char,
    pub hw_states: *mut cstate_t,
    pub hw_states_num: c_int,
    pub start: Option<unsafe extern "C" fn() -> c_int>,
    pub stop: Option<unsafe extern "C" fn() -> c_int>,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub flags: cpuidle_monitor_flags,
    pub overflow_s: c_int,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn powercap_get_energy_uj(zone: *mut powercap_zone, val: *mut u64) -> c_int;
    fn powercap_get_driver(line: *mut c_char, len: c_int) -> c_int;
    fn powercap_get_enabled(val: *mut c_int) -> c_int;
    fn powercap_init_zones() -> *mut powercap_zone;
    fn powercap_walk_zones(
        zone: *mut powercap_zone,
        cb: unsafe extern "C" fn(zone: *mut powercap_zone) -> c_int,
    );

    fn dprint(format: *const c_char, ...);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_zone_count: c_int = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_zones: [cstate_t; MAX_RAPL_ZONES] = [cstate_t::zeroed(); MAX_RAPL_ZONES];
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_zones_pt: [*mut powercap_zone; MAX_RAPL_ZONES] =
    [std::ptr::null_mut(); MAX_RAPL_ZONES];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_zone_previous_count: [c_ulonglong; MAX_RAPL_ZONES] = [0; MAX_RAPL_ZONES];
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_zone_current_count: [c_ulonglong; MAX_RAPL_ZONES] = [0; MAX_RAPL_ZONES];
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub static mut rapl_max_count: c_ulonglong = 0;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn rapl_get_count_uj(
    id: c_uint,
    count: *mut c_ulonglong,
    _cpu: c_uint,
) -> c_int {
    if rapl_zones_pt[id as usize].is_null() {
        /* error */
        return -1;
    }

    *count = rapl_zone_current_count[id as usize].wrapping_sub(rapl_zone_previous_count[id as usize]);

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn powercap_count_zones(zone: *mut powercap_zone) -> c_int {
    let mut val: u64 = 0;
    let uj: c_int;

    if rapl_zone_count >= MAX_RAPL_ZONES as c_int {
        return -1;
    }

    if (*zone).has_energy_uj == 0 {
        return 0;
    }

    printf(b"%s\n\0".as_ptr() as *const c_char, (*zone).sys_name);
    uj = powercap_get_energy_uj(zone, &mut val);
    printf(b"%d\n\0".as_ptr() as *const c_char, uj);

    strncpy(
        rapl_zones[rapl_zone_count as usize].name.as_mut_ptr(),
        (*zone).name,
        CSTATE_NAME_LEN - 1,
    );
    strcpy(
        rapl_zones[rapl_zone_count as usize].desc.as_mut_ptr(),
        b"\0".as_ptr() as *const c_char,
    );
    rapl_zones[rapl_zone_count as usize].id = rapl_zone_count;
    rapl_zones[rapl_zone_count as usize].range = RANGE_MACHINE;
    rapl_zones[rapl_zone_count as usize].get_count = Some(rapl_get_count_uj);
    rapl_zones_pt[rapl_zone_count as usize] = zone;
    rapl_zone_count += 1;

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn rapl_start() -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut uj_val: u64 = 0;

    i = 0;
    while i < rapl_zone_count {
        ret = powercap_get_energy_uj(rapl_zones_pt[i as usize], &mut uj_val);
        if ret != 0 {
            return ret;
        }
        rapl_zone_previous_count[i as usize] = uj_val as c_ulonglong;
        i += 1;
    }

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn rapl_stop() -> c_int {
    let mut i: c_int;
    let mut uj_val: u64 = 0;

    i = 0;
    while i < rapl_zone_count {
        let ret: c_int;

        ret = powercap_get_energy_uj(rapl_zones_pt[i as usize], &mut uj_val);
        if ret != 0 {
            return ret;
        }
        rapl_zone_current_count[i as usize] = uj_val as c_ulonglong;
        if rapl_max_count < uj_val as c_ulonglong {
            rapl_max_count = (uj_val as c_ulonglong).wrapping_sub(rapl_zone_previous_count[i as usize]);
        }
        i += 1;
    }
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rapl_register() -> *mut cpuidle_monitor {
    let mut root_zone: *mut powercap_zone;
    let mut line: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
    let mut ret: c_int;
    let mut val: c_int = 0;

    ret = powercap_get_driver(line.as_mut_ptr(), MAX_LINE_LEN as c_int);
    if ret < 0 {
        dprint(b"No powercapping driver loaded\n\0".as_ptr() as *const c_char);
        return std::ptr::null_mut();
    }

    dprint(b"Driver: %s\n\0".as_ptr() as *const c_char, line.as_ptr());
    ret = powercap_get_enabled(&mut val);
    if ret < 0 {
        return std::ptr::null_mut();
    }
    if val == 0 {
        dprint(b"Powercapping is disabled\n\0".as_ptr() as *const c_char);
        return std::ptr::null_mut();
    }

    dprint(b"Powercap domain hierarchy:\n\n\0".as_ptr() as *const c_char);
    root_zone = powercap_init_zones();

    if root_zone.is_null() {
        dprint(b"No powercap info found\n\0".as_ptr() as *const c_char);
        return std::ptr::null_mut();
    }

    powercap_walk_zones(root_zone, powercap_count_zones);
    rapl_monitor.hw_states_num = rapl_zone_count;

    &raw mut rapl_monitor
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
pub static mut rapl_monitor: cpuidle_monitor = cpuidle_monitor {
    name: b"RAPL\0".as_ptr() as *const c_char,
    hw_states: &raw mut rapl_zones as *mut cstate_t,
    hw_states_num: 0,
    start: Some(rapl_start),
    stop: Some(rapl_stop),
    do_register: Some(rapl_register),
    flags: cpuidle_monitor_flags { needs_root: 0 },
    overflow_s: 60 * 60 * 24 * 100, /* To be implemented */
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
