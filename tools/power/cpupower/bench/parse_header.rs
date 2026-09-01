/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

use core::ffi::{c_char, c_int, c_long};

/* FILE is supplied by the surrounding C/Rust bindings for stdio. */
use crate::FILE;

/* possible scheduler priorities */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sched_prio {
	SCHED_ERR = -1,
	SCHED_HIGH = 0,
	SCHED_DEFAULT = 1,
	SCHED_LOW = 2,
}

/* struct that holds the required config parameters */
#[repr(C)]
pub struct config {
	pub sleep: c_long,              /* sleep time in µs */
	pub load: c_long,               /* load time in µs */
	pub sleep_step: c_long,         /* time value which changes the
	                                 * sleep time after every round in µs */
	pub load_step: c_long,          /* time value which changes the
	                                 * load time after every round in µs */
	pub cycles: c_int,              /* calculation cycles with the same sleep/load time */
	pub rounds: c_int,              /* calculation rounds with iterated sleep/load time */
	pub cpu: c_int,                 /* cpu for which the affinity is set */
	pub governor: [c_char; 15],     /* cpufreq governor */
	pub prio: sched_prio,

	pub verbose: c_int,             /* verbose output */
	pub output: *mut FILE,          /* logfile */
	pub output_filename: *mut c_char, /* logfile name, must be freed at the end
	                                   if output != NULL and output != stdout*/
}

unsafe extern "C" {
	pub fn string_to_prio(str: *const c_char) -> sched_prio;

	pub fn prepare_output(dir: *const c_char) -> *mut FILE;

	pub fn prepare_config(path: *const c_char, config: *mut config) -> c_int;
	pub fn prepare_default_config() -> *mut config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
