/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/addr_location.h. */

pub enum thread {}
pub enum maps {}
pub enum map {}
pub enum symbol {}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub srcline: *const ::std::os::raw::c_char,
    pub addr: u64,
    pub level: ::std::os::raw::c_char,
    pub cpumode: u8,
    pub filtered: u16,
    pub cpu: i32,
    pub socket: i32,
    /* Same as machine.parallelism but within [1, nr_cpus]. */
    pub parallelism: ::std::os::raw::c_int,
    /* See he_stat.latency. */
    pub latency: u64,
}

extern "C" {
    pub fn addr_location__init(al: *mut addr_location);
    pub fn addr_location__exit(al: *mut addr_location);

    pub fn addr_location__copy(dst: *mut addr_location, src: *mut addr_location);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
