/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: <asm/sigcontext.h> and <asm/ptrace.h>. */

extern "C" {
    pub fn fpu_libc_helper(regs: *mut crate::pt_regs) -> ::core::ffi::c_int;
    pub fn fpu_fpe(regs: *mut crate::pt_regs);
    pub fn mtcr(register: *const ::core::ffi::c_char, value: ::core::ffi::c_int);
    pub fn save_to_user_fp(user_fp: *mut crate::user_fp);
    pub fn restore_from_user_fp(user_fp: *mut crate::user_fp);
}

pub unsafe fn init_fpu() {
    mtcr(
        b"cr<1, 2>\0".as_ptr() as *const ::core::ffi::c_char,
        0,
    );
}

/*
 * Define the fesr bit for fpe handle.
 */
pub const FPE_ILLE: i32 = 1 << 16; /* Illegal instruction  */
pub const FPE_FEC: i32 = 1 << 7; /* Input float-point arithmetic exception */
pub const FPE_IDC: i32 = 1 << 5; /* Input denormalized exception */
pub const FPE_IXC: i32 = 1 << 4; /* Inexact exception */
pub const FPE_UFC: i32 = 1 << 3; /* Underflow exception */
pub const FPE_OFC: i32 = 1 << 2; /* Overflow exception */
pub const FPE_DZC: i32 = 1 << 1; /* Divide by zero exception */
pub const FPE_IOC: i32 = 1 << 0; /* Invalid operation exception */
pub const FPE_REGULAR_EXCEPTION: i32 = FPE_IXC | FPE_UFC | FPE_OFC | FPE_DZC | FPE_IOC;

#[cfg(CONFIG_OPEN_FPU_IDE)]
pub const IDE_STAT: i32 = 1 << 5;
#[cfg(not(CONFIG_OPEN_FPU_IDE))]
pub const IDE_STAT: i32 = 0;

#[cfg(CONFIG_OPEN_FPU_IXE)]
pub const IXE_STAT: i32 = 1 << 4;
#[cfg(not(CONFIG_OPEN_FPU_IXE))]
pub const IXE_STAT: i32 = 0;

#[cfg(CONFIG_OPEN_FPU_UFE)]
pub const UFE_STAT: i32 = 1 << 3;
#[cfg(not(CONFIG_OPEN_FPU_UFE))]
pub const UFE_STAT: i32 = 0;

#[cfg(CONFIG_OPEN_FPU_OFE)]
pub const OFE_STAT: i32 = 1 << 2;
#[cfg(not(CONFIG_OPEN_FPU_OFE))]
pub const OFE_STAT: i32 = 0;

#[cfg(CONFIG_OPEN_FPU_DZE)]
pub const DZE_STAT: i32 = 1 << 1;
#[cfg(not(CONFIG_OPEN_FPU_DZE))]
pub const DZE_STAT: i32 = 0;

#[cfg(CONFIG_OPEN_FPU_IOE)]
pub const IOE_STAT: i32 = 1 << 0;
#[cfg(not(CONFIG_OPEN_FPU_IOE))]
pub const IOE_STAT: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
