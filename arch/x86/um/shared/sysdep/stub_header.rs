/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <asm/unistd.h>, <sys/mman.h>, <signal.h>, <as-layout.h>,
// and <stub-data.h>.

// Architecture-specific declaration selected by the C preprocessor:
// #ifdef __i386__
// #include "stub_32.h"
// #else
// #include "stub_64.h"
// #endif

unsafe extern "C" {
    pub fn stub_segv_handler(
        signal: core::ffi::c_int,
        info: *mut siginfo_t,
        context: *mut core::ffi::c_void,
    );
    pub fn stub_syscall_handler();
    pub fn stub_signal_interrupt(
        signal: core::ffi::c_int,
        info: *mut siginfo_t,
        context: *mut core::ffi::c_void,
    );
    pub fn stub_signal_restorer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
