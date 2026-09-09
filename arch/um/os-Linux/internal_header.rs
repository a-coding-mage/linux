/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the translated project:
//   mm_id.h, stub-data.h, signal.h

use core::ffi::{c_char, c_int};

// Opaque declaration corresponding to `struct mm_id` from mm_id.h.
#[repr(C)]
pub struct mm_id {
    _private: [u8; 0],
}

/*
 * elf_aux.c
 */
extern "C" {
    pub fn scan_elf_aux(envp: *mut *mut c_char);
}

/*
 * mem.c
 */
extern "C" {
    pub fn check_tmpexec();
}

/*
 * signal.c
 */
extern "C" {
    // C declaration: extern __thread int signals_enabled;
    #[thread_local]
    pub static mut signals_enabled: c_int;
    pub fn timer_alarm_pending() -> c_int;
}

/*
 * skas/process.c
 */
extern "C" {
    pub fn wait_stub_done(pid: c_int);
    pub fn wait_stub_done_seccomp(mm_idp: *mut mm_id, running: c_int, wait_sigsys: c_int);
}

/*
 * smp.c
 */
// `SIGRTMIN` is supplied by the target platform's signal definitions.
pub const IPI_SIGNAL: c_int = SIGRTMIN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
