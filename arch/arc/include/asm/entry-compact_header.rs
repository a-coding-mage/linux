/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of asm/entry-compact.h.
 *
 * The original file is an ARC assembler header.  Its assembler macros are
 * retained below as Rust comments because ARC assembler interpolation and
 * register names have no file-local Rust equivalent.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm-offsets.h, irqflags-compact.h, and thread_info.h (THREAD_SIZE).

pub const EVENT_IRQ1: u32 = 0x0031_abcd;
pub const EVENT_IRQ2: u32 = 0x0032_abcd;

/*
macro_rules! PUSHAX { ($aux:tt) => { lr r9, [$aux]; push r9; }; }
macro_rules! POPAX { ($aux:tt) => { pop r9; sr r9, [$aux]; }; }

macro_rules! SAVE_R0_TO_R12 { () => {
    push r0; push r1; push r2; push r3; push r4; push r5; push r6;
    push r7; push r8; push r9; push r10; push r11; push r12;
}; }
macro_rules! RESTORE_R12_TO_R0 { () => {
    pop r12; pop r11; pop r10; pop r9; pop r8; pop r7; pop r6;
    pop r5; pop r4; pop r3; pop r2; pop r1; pop r0;
}; }
macro_rules! SAVE_ABI_CALLEE_REGS { () => {
    push r13; push r14; push r15; push r16; push r17; push r18;
    push r19; push r20; push r21; push r22; push r23; push r24; push r25;
}; }
macro_rules! RESTORE_ABI_CALLEE_REGS { () => {
    pop r25; pop r24; pop r23; pop r22; pop r21; pop r20; pop r19;
    pop r18; pop r17; pop r16; pop r15; pop r14; pop r13;
}; }

macro_rules! PROLOG_FREEUP_REG { ($reg:tt, $mem:tt) => { st $reg, [$mem]; }; }
macro_rules! PROLOG_RESTORE_REG { ($reg:tt, $mem:tt) => { ld $reg, [$mem]; }; }
*/

// SWITCH_TO_KERNEL_STK:
//   Tests STATUS_U_BIT in r9, optionally tests VMALLOC_START for compact IRQ
//   levels, saves the old SP at PT_sp - SZ_PT_REGS, and switches to the task's
//   kernel stack.  The complete ARC sequence is preserved verbatim below.
/*
.macro SWITCH_TO_KERNEL_STK
    bbit1 r9, STATUS_U_BIT, 88f
#ifdef CONFIG_ARC_COMPACT_IRQ_LEVELS
    brlo sp, VMALLOC_START, 88f
#endif
    b.d 66f
    st sp, [sp, PT_sp - SZ_PT_REGS]
88:
    GET_CURR_TASK_ON_CPU r9
    GET_TSK_STACK_BASE r9, r9
    st sp, [r9, PT_sp - SZ_PT_REGS]
    mov sp, r9
66:
.endm

.macro FAKE_RET_FROM_EXCPN
    lr r9, [status32]; bclr r9, r9, STATUS_AE_BIT
    or r9, r9, (STATUS_E1_MASK|STATUS_E2_MASK)
    sr r9, [erstatus]; mov r9, 55f; sr r9, [eret]; rtie
55:
.endm

/*
.macro EXCEPTION_PROLOGUE_KEEP_AE
    PROLOG_FREEUP_REG r9, @ex_saved_reg1
    lr r9, [erstatus]
    SWITCH_TO_KERNEL_STK
    st.a r0, [sp, -8]
    sub sp, sp, 4
    PROLOG_RESTORE_REG r9, @ex_saved_reg1
    SAVE_R0_TO_R12
    PUSH gp; PUSH fp; PUSH blink; PUSHAX eret; PUSHAX erstatus
    PUSH lp_count; PUSHAX lp_end; PUSHAX lp_start; PUSHAX erbta
    lr r10, [ecr]
    st r10, [sp, PT_event]
#ifdef CONFIG_ARC_CURR_IN_REG
    GET_CURR_TASK_ON_CPU gp
#endif
.endm

.macro EXCEPTION_PROLOGUE
    EXCEPTION_PROLOGUE_KEEP_AE
    lr r0, [efa]
    mov r1, sp
    FAKE_RET_FROM_EXCPN
.endm

.macro EXCEPTION_EPILOGUE
    POPAX erbta; POPAX lp_start; POPAX lp_end
    pop r9; mov lp_count, r9
    POPAX erstatus; POPAX eret; pop blink; pop fp; pop gp
    RESTORE_R12_TO_R0
    ld sp, [sp]
.endm

.macro INTERRUPT_PROLOGUE LVL
    PROLOG_FREEUP_REG r9, @int\\LVL\\()_saved_reg
    lr r9, [status32_l\\LVL\\()]
    SWITCH_TO_KERNEL_STK
    st.a 0x003\\LVL\\()abcd, [sp, -4]
    sub sp, sp, 8
    PROLOG_RESTORE_REG r9, @int\\LVL\\()_saved_reg
    SAVE_R0_TO_R12
    PUSH gp; PUSH fp; PUSH blink; PUSH ilink\\LVL\\()
    PUSHAX status32_l\\LVL\\(); PUSH lp_count; PUSHAX lp_end
    PUSHAX lp_start; PUSHAX bta_l\\LVL\\()
#ifdef CONFIG_ARC_CURR_IN_REG
    GET_CURR_TASK_ON_CPU gp
#endif
.endm

.macro INTERRUPT_EPILOGUE LVL
    POPAX bta_l\\LVL\\(); POPAX lp_start; POPAX lp_end
    pop r9; mov lp_count, r9
    POPAX status32_l\\LVL\\(); POP ilink\\LVL\\(); pop blink
    pop fp; pop gp; RESTORE_R12_TO_R0
    ld sp, [sp]
.endm
*/
*/

// GET_CURR_THR_INFO_FROM_SP(reg): bic reg, sp, (THREAD_SIZE - 1)
// GET_CPU_ID(reg): lr reg, [identity]; lsr reg, reg, 8; bmsk reg, reg, 7

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
