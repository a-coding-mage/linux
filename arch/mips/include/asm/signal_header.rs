/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2003 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// Dependency supplied by the UAPI signal header: signal types and constants.
// Dependency supplied by asm/sigcontext.h.
// Dependency supplied by asm/siginfo.h.

#[cfg(CONFIG_MIPS32_O32)]
extern "C" {
    pub static mut mips_abi_32: mips_abi;
}

// The CONFIG_MIPS32_O32, CONFIG_64BIT, and CONFIG_TRAD_SIGNALS conditions are
// build-time conditions from the original header and are retained here.
#[cfg(CONFIG_MIPS32_O32)]
#[macro_export]
macro_rules! sig_uses_siginfo {
    ($ka:expr, $abi:expr) => {
        if ($abi as *const _) != unsafe { &mips_abi_32 as *const _ } {
            1
        } else {
            (($ka).sa.sa_flags & SA_SIGINFO)
        }
    };
}

#[cfg(not(CONFIG_MIPS32_O32))]
#[macro_export]
macro_rules! sig_uses_siginfo {
    ($ka:expr, $abi:expr) => {
        if cfg!(target_pointer_width = "64") {
            1
        } else if cfg!(CONFIG_TRAD_SIGNALS) {
            (($ka).sa.sa_flags & SA_SIGINFO)
        } else {
            1
        }
    };
}

pub const __ARCH_HAS_IRIX_SIGACTION: bool = true;

extern "C" {
    pub fn protected_save_fp_context(sc: *mut core::ffi::c_void) -> i32;
    pub fn protected_restore_fp_context(sc: *mut core::ffi::c_void) -> i32;
    pub fn do_notify_resume(
        regs: *mut pt_regs,
        unused: *mut core::ffi::c_void,
        thread_info_flags: __u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
