/* SPDX-License-Identifier: GPL-2.0 */
/*
 * x86 FPU signal frame handling methods:
 */

use core::ffi::c_void;

/* The C header includes linux/compat.h, linux/user.h, and asm/fpu/types.h. */

/* Under CONFIG_X86_64 these names are supplied by the x86 signal-context and
 * user32 headers.  Otherwise the C header aliases them as follows:
 *   user_i387_ia32_struct -> user_i387_struct
 *   user32_fxsr_struct    -> user_fxsr_struct
 */

unsafe extern "C" {
    pub fn convert_from_fxsr(
        env: *mut user_i387_ia32_struct,
        tsk: *mut task_struct,
    );

    pub fn convert_to_fxsr(
        fxsave: *mut fxregs_state,
        env: *const user_i387_ia32_struct,
    );

    pub fn fpu__alloc_mathframe(
        sp: c_ulong,
        ia32_frame: i32,
        buf_fx: *mut c_ulong,
        size: *mut c_ulong,
    ) -> c_ulong;

    pub fn fpu__get_fpstate_size() -> c_ulong;

    pub fn copy_fpstate_to_sigframe(
        buf: *mut c_void,
        fp: *mut c_void,
        size: i32,
        pkru: u32,
    ) -> bool;

    pub fn fpu__clear_user_states(fpu: *mut fpu);

    pub fn fpu__restore_sig(buf: *mut c_void, ia32_frame: i32) -> bool;

    pub fn restore_fpregs_from_fpstate(fpstate: *mut fpstate, mask: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
