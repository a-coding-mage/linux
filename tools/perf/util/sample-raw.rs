// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_void};
use core::ptr;

// Translated from includes:
// "sample-raw.h", <elf.h>, <linux/string.h>, "env.h", "evlist.h",
// "header.h", and "session.h".

pub const EM_386: u16 = 3;
pub const EM_S390: u16 = 22;
pub const EM_X86_64: u16 = 62;

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

pub type trace_event_sample_raw_fn = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut c_void) -> u16;
    fn perf_env__cpuid(env: *mut perf_env) -> *const c_char;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn evlist__has_amd_ibs(evlist: *mut evlist) -> bool;
    fn evlist__set_trace_event_sample_raw(
        evlist: *mut evlist,
        sample_raw: trace_event_sample_raw_fn,
    );
    fn evlist__s390_sample_raw();
    fn evlist__amd_sample_raw();
}

/*
 * Check platform the perf data file was created on and perform platform
 * specific interpretation.
 */
#[no_mangle]
pub unsafe extern "C" fn evlist__init_trace_event_sample_raw(
    evlist: *mut evlist,
    env: *mut perf_env,
) {
    let e_machine: u16 = unsafe { perf_env__e_machine(env, ptr::null_mut()) };

    if e_machine == EM_S390 {
        unsafe {
            evlist__set_trace_event_sample_raw(evlist, Some(evlist__s390_sample_raw));
        }
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        let cpuid: *const c_char = unsafe { perf_env__cpuid(env) };

        if !cpuid.is_null()
            && unsafe { strstarts(cpuid, c"AuthenticAMD".as_ptr()) }
            && unsafe { evlist__has_amd_ibs(evlist) }
        {
            unsafe {
                evlist__set_trace_event_sample_raw(evlist, Some(evlist__amd_sample_raw));
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
