/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by <linux/unwind_user_types.h> and <asm/unwind_user.h>.

// #ifndef CONFIG_HAVE_UNWIND_USER_FP
// #define ARCH_INIT_USER_FP_FRAME(ws)
// #endif

// #ifndef ARCH_INIT_USER_FP_ENTRY_FRAME
// #define ARCH_INIT_USER_FP_ENTRY_FRAME(ws)
// #endif

// #ifndef unwind_user_at_function_start
#[inline]
pub unsafe fn unwind_user_at_function_start(regs: *mut pt_regs) -> bool {
    let _ = regs;
    false
}
// #define unwind_user_at_function_start unwind_user_at_function_start
// #endif

unsafe extern "C" {
    pub fn unwind_user(trace: *mut unwind_stacktrace, max_entries: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
