/* SPDX-License-Identifier: GPL-2.0 */
/* fpumacro.h: FPU related macros.
 *
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency intent preserved from <asm/asi.h> and <asm/visasm.h>.

#[repr(C)]
pub struct fpustate {
    pub regs: [u32; 64],
}

// Equivalent to:
// #define FPUSTATE (struct fpustate *)(current_thread_info()->fpregs)
#[macro_export]
macro_rules! FPUSTATE {
    () => {
        unsafe { (*current_thread_info()).fpregs as *mut $crate::fpustate }
    };
}

#[inline]
pub unsafe fn fprs_read() -> ::core::primitive::u64 {
    let mut retval: ::core::primitive::u64;

    ::core::arch::asm!("rd %fprs, {}", out(reg) retval);

    retval
}

#[inline]
pub unsafe fn fprs_write(val: ::core::primitive::u64) {
    ::core::arch::asm!("wr {}, 0x0, %fprs", in(reg) val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
