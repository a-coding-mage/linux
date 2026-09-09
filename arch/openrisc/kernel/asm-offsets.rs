// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC asm-offsets.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others. All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * This program is used to generate definitions needed by
 * assembly language modules.
 *
 * The original C source uses the kernel DEFINE/offsetof machinery to emit
 * assembler #defines. The corresponding names and layout expressions remain
 * explicit below; kernel-provided types and the DEFINE backend are external
 * dependencies.
 */

// C preprocessor/build configuration: COMPILE_OFFSETS.
// Required kernel headers: linux/{signal,sched,kernel,errno,string,types,
// ptrace,mman,mm,io,thread_info,kbuild}.h and asm/{page,processor}.h.

extern "C" {
    fn DEFINE(name: *const u8, value: usize);
}

#[allow(non_snake_case)]
unsafe fn define(name: &[u8], value: usize) {
    DEFINE(name.as_ptr(), value);
}

fn main() {
    unsafe {
        // offsets into the task_struct
        define(b"TASK_FLAGS\0", core::mem::offset_of!(task_struct, flags));
        define(b"TASK_PTRACE\0", core::mem::offset_of!(task_struct, ptrace));
        define(b"TASK_THREAD\0", core::mem::offset_of!(task_struct, thread));
        define(b"TASK_MM\0", core::mem::offset_of!(task_struct, mm));
        define(
            b"TASK_ACTIVE_MM\0",
            core::mem::offset_of!(task_struct, active_mm),
        );

        // offsets into thread_info
        define(b"TI_TASK\0", core::mem::offset_of!(thread_info, task));
        define(b"TI_FLAGS\0", core::mem::offset_of!(thread_info, flags));
        define(
            b"TI_PREEMPT\0",
            core::mem::offset_of!(thread_info, preempt_count),
        );
        define(b"TI_KSP\0", core::mem::offset_of!(thread_info, ksp));

        define(b"PT_SIZE\0", core::mem::size_of::<pt_regs>());

        // Interrupt register frame
        define(b"STACK_FRAME_OVERHEAD\0", STACK_FRAME_OVERHEAD);
        define(
            b"INT_FRAME_SIZE\0",
            STACK_FRAME_OVERHEAD + core::mem::size_of::<pt_regs>(),
        );

        define(b"NUM_USER_SEGMENTS\0", TASK_SIZE >> 28);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
