/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The original Linux configuration conditions
// are represented with Rust feature gates where applicable.

pub const CPU_PROFILING: ::kernel::ffi::c_int = 1;
pub const SCHED_PROFILING: ::kernel::ffi::c_int = 2;
pub const KVM_PROFILING: ::kernel::ffi::c_int = 4;

pub enum proc_dir_entry {}
pub enum notifier_block {}

#[cfg(all(CONFIG_PROFILING, CONFIG_PROC_FS))]
unsafe extern "C" {
    pub fn create_proc_profile() -> ::kernel::ffi::c_int;
}

#[cfg(not(all(CONFIG_PROFILING, CONFIG_PROC_FS)))]
#[inline]
pub unsafe fn create_proc_profile() -> ::kernel::ffi::c_int {
    0
}

#[cfg(CONFIG_PROFILING)]
unsafe extern "C" {
    pub static mut prof_on: ::kernel::ffi::c_int;

    // init basic kernel profiler
    pub fn profile_init() -> ::kernel::ffi::c_int;
    pub fn profile_setup(str_: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn profile_tick(type_: ::kernel::ffi::c_int);
    pub fn setup_profiling_timer(multiplier: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;

    /*
     * Add multiple profiler hits to a given address:
     */
    pub fn profile_hits(
        type_: ::kernel::ffi::c_int,
        ip: *mut ::kernel::ffi::c_void,
        nr_hits: ::kernel::ffi::c_uint,
    );
}

#[cfg(CONFIG_PROFILING)]
#[inline]
pub unsafe fn profile_hit(type_: ::kernel::ffi::c_int, ip: *mut ::kernel::ffi::c_void) {
    /*
     * Speedup for the common (no profiling enabled) case:
     */
    if unlikely(prof_on == type_) {
        profile_hits(type_, ip, 1);
    }
}

#[cfg(not(CONFIG_PROFILING))]
pub const prof_on: ::kernel::ffi::c_int = 0;

#[cfg(not(CONFIG_PROFILING))]
#[inline]
pub unsafe fn profile_init() -> ::kernel::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PROFILING))]
#[inline]
pub unsafe fn profile_tick(_type_: ::kernel::ffi::c_int) {}

#[cfg(not(CONFIG_PROFILING))]
#[inline]
pub unsafe fn profile_hits(
    _type_: ::kernel::ffi::c_int,
    _ip: *mut ::kernel::ffi::c_void,
    _nr_hits: ::kernel::ffi::c_uint,
) {
}

#[cfg(not(CONFIG_PROFILING))]
#[inline]
pub unsafe fn profile_hit(_type_: ::kernel::ffi::c_int, _ip: *mut ::kernel::ffi::c_void) {}

// `unlikely` is supplied by the kernel dependencies.
unsafe extern "C" {
    fn unlikely(value: bool) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
