/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arm/arm64/include/asm/current.h
 *
 * Copyright (C) 2016 ARM
 * Copyright (C) 2017 SiFive
 */

/* The C header's include guard is not needed in Rust. */
/* Dependencies: linux/bug.h and linux/compiler.h. */

/* These declarations are omitted when compiling as an assembler source. */

/// Opaque declaration corresponding to `struct task_struct`.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/*
 * This only works because "struct thread_info" is at offset 0 from "struct
 * task_struct".  This constraint seems to be necessary on other architectures
 * as well, but __switch_to enforces it.  We can't check TASK_TI here because
 * <asm/asm-offsets.h> includes this, and I can't get the definition of "struct
 * task_struct" here due to some header ordering problems.
 */

/*
 * C declares this as a register variable bound to the RISC-V `tp` register.
 * The external declaration preserves that dependency for the surrounding
 * target-specific bindings.
 */
extern "C" {
    pub static mut riscv_current_is_tp: *mut task_struct;
    pub static mut current_stack_pointer: usize;
}

#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    riscv_current_is_tp
}

/* C macro equivalent: current expands to get_current(). */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
