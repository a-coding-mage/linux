/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/unistd.h (NR_syscalls), asm/arcregs.h, asm/ptrace.h,
// asm/processor.h (VMALLOC_START), and asm/mmu.h.

/*
 * The original header contains assembler-only ISA and task/context-switch
 * macros. They are retained here as source-level assembly macro definitions;
 * their expansion depends on the target assembler and configuration.
 */
/*
#ifdef CONFIG_ISA_ARCOMPACT
#include <asm/entry-compact.h>
#else
#include <asm/entry-arcv2.h>
#endif

.macro SAVE_CALLEE_SAVED_USER
    SAVE_ABI_CALLEE_REGS
.endm
.macro RESTORE_CALLEE_SAVED_USER
    RESTORE_ABI_CALLEE_REGS
.endm
.macro SAVE_CALLEE_SAVED_KERNEL
    SAVE_ABI_CALLEE_REGS
.endm
.macro RESTORE_CALLEE_SAVED_KERNEL
    RESTORE_ABI_CALLEE_REGS
.endm
.macro DISCARD_CALLEE_SAVED_USER
    add sp, sp, SZ_CALLEE_REGS
.endm
.macro GET_TSK_STACK_BASE tsk, out
    ld \out, [\tsk, TASK_THREAD_INFO]
    add2 \out, \out, (THREAD_SIZE)/4
.endm
.macro GET_CURR_THR_INFO_FLAGS reg
    GET_CURR_THR_INFO_FROM_SP \reg
    ld \reg, [\reg, THREAD_INFO_FLAGS]
.endm

#ifdef CONFIG_SMP
.macro GET_CURR_TASK_ON_CPU reg
    GET_CPU_ID \reg
    ld.as \reg, [@_current_task, \reg]
.endm
.macro SET_CURR_TASK_ON_CPU tsk, tmp
    GET_CPU_ID \tmp
    add2 \tmp, @_current_task, \tmp
    st \tsk, [\tmp]
#ifdef CONFIG_ARC_CURR_IN_REG
    mov gp, \tsk
#endif
.endm
#else
.macro GET_CURR_TASK_ON_CPU reg
    ld \reg, [@_current_task]
.endm
.macro SET_CURR_TASK_ON_CPU tsk, tmp
    st \tsk, [@_current_task]
#ifdef CONFIG_ARC_CURR_IN_REG
    mov gp, \tsk
#endif
.endm
#endif

#ifdef CONFIG_ARC_CURR_IN_REG
.macro GET_CURR_TASK_FIELD_PTR off, reg
    add \reg, gp, \off
.endm
#else
.macro GET_CURR_TASK_FIELD_PTR off, reg
    GET_CURR_TASK_ON_CPU \reg
    add \reg, \reg, \off
.endm
#endif
*/

// C declarations translated from the non-assembler portion of the header.
unsafe extern "C" {
    pub fn do_signal(regs: *mut pt_regs);
    pub fn do_notify_resume(regs: *mut pt_regs);
    pub fn do_privilege_fault(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn do_extension_fault(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn insterror_is_error(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn do_memory_error(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn trap_is_brkpt(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn do_misaligned_error(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn do_trap5_error(arg: libc::c_ulong, regs: *mut pt_regs) -> libc::c_int;
    pub fn do_misaligned_access(
        arg: libc::c_ulong,
        regs: *mut pt_regs,
        callee: *mut callee_regs,
    ) -> libc::c_int;
    pub fn do_machine_check_fault(arg: libc::c_ulong, regs: *mut pt_regs);
    pub fn do_non_swi_trap(arg: libc::c_ulong, regs: *mut pt_regs);
    pub fn do_insterror_or_kprobe(arg: libc::c_ulong, regs: *mut pt_regs);
    pub fn do_page_fault(arg: libc::c_ulong, regs: *mut pt_regs);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
