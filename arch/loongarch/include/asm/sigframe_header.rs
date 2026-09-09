/* SPDX-License-Identifier: GPL-2.0+ */

// Dependency declarations corresponding to:
// #include <asm/siginfo.h>
// #include <asm/ucontext.h>

#[repr(C)]
pub struct rt_sigframe {
    pub rs_info: siginfo,
    pub rs_uctx: ucontext,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
