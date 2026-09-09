/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard and include dependencies are omitted.  `siginfo` and
 * `ucontext` are supplied by the surrounding translated headers.
 */

#[repr(C)]
pub struct rt_sigframe {
    /* holds original return address */
    pub tramp: [core::ffi::c_uint; 2],
    pub info: siginfo,
    pub uc: ucontext,
}

pub const SIGFRAME: usize = 128;
pub const FUNCTIONCALLFRAME: usize = 96;
pub const PARISC_RT_SIGFRAME_SIZE: usize =
    (core::mem::size_of::<rt_sigframe>() + FUNCTIONCALLFRAME + SIGFRAME)
        & !((SIGFRAME) - 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
