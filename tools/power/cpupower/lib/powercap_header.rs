/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (C) 2016 SUSE Software Solutions GmbH
 *           Thomas Renninger <trenn@suse.de>
 */

use std::os::raw::{c_char, c_int};

pub const PATH_TO_POWERCAP: &[u8] = b"/sys/devices/virtual/powercap\0";
pub const PATH_TO_RAPL: &[u8] = b"/sys/devices/virtual/powercap/intel-rapl\0";
pub const PATH_TO_RAPL_CLASS: &[u8] = b"/sys/devices/virtual/powercap/intel-rapl\0";

pub const POWERCAP_MAX_CHILD_ZONES: usize = 10;
pub const POWERCAP_MAX_TREE_DEPTH: usize = 10;

pub const MAX_LINE_LEN: usize = 4096;
pub const SYSFS_PATH_MAX: usize = 255;

#[repr(C)]
pub struct powercap_zone {
    pub name: [c_char; MAX_LINE_LEN],
    /*
     * sys_name relative to PATH_TO_POWERCAP,
     * do not forget the / in between
     */
    pub sys_name: [c_char; SYSFS_PATH_MAX],
    pub tree_depth: c_int,
    pub parent: *mut powercap_zone,
    pub children: [*mut powercap_zone; POWERCAP_MAX_CHILD_ZONES],
    /* More possible caps or attributes to be added? */
    pub has_power_uw_has_energy_uj: u32,
}

pub const POWERCAP_ZONE_HAS_POWER_UW_MASK: u32 = 1 << 0;
pub const POWERCAP_ZONE_HAS_ENERGY_UJ_MASK: u32 = 1 << 1;

pub type powercap_zone_callback = Option<unsafe extern "C" fn(zone: *mut powercap_zone) -> c_int>;

unsafe extern "C" {
    pub fn powercap_walk_zones(zone: *mut powercap_zone, f: powercap_zone_callback) -> c_int;

    pub fn powercap_init_zones() -> *mut powercap_zone;
    pub fn powercap_get_enabled(mode: *mut c_int) -> c_int;
    pub fn powercap_set_enabled(mode: c_int) -> c_int;
    pub fn powercap_get_driver(driver: *mut c_char, buflen: c_int) -> c_int;

    pub fn powercap_get_max_energy_range_uj(zone: *mut powercap_zone, val: *mut u64) -> c_int;
    pub fn powercap_get_energy_uj(zone: *mut powercap_zone, val: *mut u64) -> c_int;
    pub fn powercap_get_max_power_range_uw(zone: *mut powercap_zone, val: *mut u64) -> c_int;
    pub fn powercap_get_power_uw(zone: *mut powercap_zone, val: *mut u64) -> c_int;
    pub fn powercap_zone_get_enabled(zone: *mut powercap_zone, mode: *mut c_int) -> c_int;
    pub fn powercap_zone_set_enabled(zone: *mut powercap_zone, mode: c_int) -> c_int;
}
