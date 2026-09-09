/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub static mut init_fpstate: fpstate;
}

/* CPU feature check wrappers */
#[inline(always)]
pub fn use_xsave() -> bool {
    cpu_feature_enabled(X86_FEATURE_XSAVE)
}

#[inline(always)]
pub fn use_fxsr() -> bool {
    cpu_feature_enabled(X86_FEATURE_FXSR)
}

/* The CONFIG_X86_DEBUG_FPU branch is selected by the build configuration. */
#[cfg(CONFIG_X86_DEBUG_FPU)]
macro_rules! WARN_ON_FPU {
    ($x:expr) => {
        WARN_ON_ONCE($x)
    };
}

#[cfg(not(CONFIG_X86_DEBUG_FPU))]
macro_rules! WARN_ON_FPU {
    ($x:expr) => {{
        BUILD_BUG_ON_INVALID!($x);
        0
    }};
}

extern "C" {
    pub fn fpstate_init_user(fpstate: *mut fpstate);
    pub fn fpstate_reset(fpu: *mut fpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
