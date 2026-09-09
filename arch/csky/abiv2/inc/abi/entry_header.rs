/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C/CSKY assembly header `entry.h`.
// The following values are offsets in the saved register frame.
pub const LSAVE_PC: usize = 8;
pub const LSAVE_PSR: usize = 12;
pub const LSAVE_A0: usize = 24;
pub const LSAVE_A1: usize = 28;
pub const LSAVE_A2: usize = 32;
pub const LSAVE_A3: usize = 36;
pub const LSAVE_A4: usize = 40;
pub const LSAVE_A5: usize = 44;

// KSPTOUSP and USPTOKSP are assembly marker macros with no emitted body.
// `usp` denotes CSKY control register cr<14, 1>.

/*
 * The source declarations below are CSKY assembler macros. Rust has no
 * portable representation for these target-specific instructions, so their
 * complete source bodies are retained verbatim as documentation of the
 * required low-level operations and interfaces.
 */

/*
.macro SAVE_ALL epc_inc
	subi sp, 152
	stw tls, (sp, 0)
	stw lr, (sp, 4)
	RD_MEH lr
	WR_MEH lr
	mfcr lr, epc
	movi tls, \\epc_inc
	add lr, tls
	stw lr, (sp, 8)
	mfcr lr, epsr
	stw lr, (sp, 12)
	btsti lr, 31
	bf 1f
	addi lr, sp, 152
	br 2f
1:
	mfcr lr, usp
2:
	stw lr, (sp, 16)
	stw a0, (sp, 20)
	stw a0, (sp, 24)
	stw a1, (sp, 28)
	stw a2, (sp, 32)
	stw a3, (sp, 36)
	addi sp, 40
	stm r4-r13, (sp)
	addi sp, 40
	stm r16-r30, (sp)
#ifdef CONFIG_CPU_HAS_HILO
	mfhi lr
	stw lr, (sp, 60)
	mflo lr
	stw lr, (sp, 64)
	mfcr lr, cr14
	stw lr, (sp, 68)
#endif
	subi sp, 80
.endm

.macro RESTORE_ALL
	ldw tls, (sp, 0)
	ldw lr, (sp, 4)
	ldw a0, (sp, 8)
	mtcr a0, epc
	ldw a0, (sp, 12)
	mtcr a0, epsr
	btsti a0, 31
	ldw a0, (sp, 16)
	mtcr a0, usp
	mtcr a0, ss0
#ifdef CONFIG_CPU_HAS_HILO
	ldw a0, (sp, 140)
	mthi a0
	ldw a0, (sp, 144)
	mtlo a0
	ldw a0, (sp, 148)
	mtcr a0, cr14
#endif
	ldw a0, (sp, 24)
	ldw a1, (sp, 28)
	ldw a2, (sp, 32)
	ldw a3, (sp, 36)
	addi sp, 40
	ldm r4-r13, (sp)
	addi sp, 40
	ldm r16-r30, (sp)
	addi sp, 72
	bf 1f
	mfcr sp, ss0
1:
	rte
.endm
*/

/*
The remaining assembler macros, preserved verbatim as target-specific source:

.macro SAVE_REGS_FTRACE
	subi sp, 152; stw tls, (sp, 0); stw lr, (sp, 4)
	mfcr lr, psr; stw lr, (sp, 12); addi lr, sp, 152; stw lr, (sp, 16)
	stw a0, (sp, 20); stw a0, (sp, 24); stw a1, (sp, 28); stw a2, (sp, 32); stw a3, (sp, 36)
	addi sp, 40; stm r4-r13, (sp); addi sp, 40; stm r16-r30, (sp)
#ifdef CONFIG_CPU_HAS_HILO
	mfhi lr; stw lr, (sp, 60); mflo lr; stw lr, (sp, 64); mfcr lr, cr14; stw lr, (sp, 68)
#endif
	subi sp, 80
.endm

.macro RESTORE_REGS_FTRACE
	ldw tls, (sp, 0)
#ifdef CONFIG_CPU_HAS_HILO
	ldw a0, (sp, 140); mthi a0; ldw a0, (sp, 144); mtlo a0; ldw a0, (sp, 148); mtcr a0, cr14
#endif
	ldw a0, (sp, 24); ldw a1, (sp, 28); ldw a2, (sp, 32); ldw a3, (sp, 36)
	addi sp, 40; ldm r4-r13, (sp); addi sp, 40; ldm r16-r30, (sp); addi sp, 72
.endm

.macro SAVE_SWITCH_STACK
	subi sp, 64; stm r4-r11, (sp); stw lr, (sp, 32); stw r16, (sp, 36); stw r17, (sp, 40)
	stw r26, (sp, 44); stw r27, (sp, 48); stw r28, (sp, 52); stw r29, (sp, 56); stw r30, (sp, 60)
#ifdef CONFIG_CPU_HAS_HILO
	subi sp, 16; mfhi lr; stw lr, (sp, 0); mflo lr; stw lr, (sp, 4); mfcr lr, cr14; stw lr, (sp, 8)
#endif
.endm

.macro RESTORE_SWITCH_STACK
#ifdef CONFIG_CPU_HAS_HILO
	ldw lr, (sp, 0); mthi lr; ldw lr, (sp, 4); mtlo lr; ldw lr, (sp, 8); mtcr lr, cr14; addi sp, 16
#endif
	ldm r4-r11, (sp); ldw lr, (sp, 32); ldw r16, (sp, 36); ldw r17, (sp, 40)
	ldw r26, (sp, 44); ldw r27, (sp, 48); ldw r28, (sp, 52); ldw r29, (sp, 56); ldw r30, (sp, 60); addi sp, 64
.endm

// MMU registers operators.
.macro RD_MIR rx; mfcr \\rx, cr<0, 15>; .endm
.macro RD_MEH rx; mfcr \\rx, cr<4, 15>; .endm
.macro RD_MCIR rx; mfcr \\rx, cr<8, 15>; .endm
.macro RD_PGDR rx; mfcr \\rx, cr<29, 15>; .endm
.macro RD_PGDR_K rx; mfcr \\rx, cr<28, 15>; .endm
.macro WR_MEH rx; mtcr \\rx, cr<4, 15>; .endm
.macro WR_MCIR rx; mtcr \\rx, cr<8, 15>; .endm

These macros contain the exact CSKY instructions from the source header.
*/

#[cfg(feature = "CONFIG_PAGE_OFFSET_80000000")]
pub const MSA_SET: (u8, u8) = (30, 15);
#[cfg(feature = "CONFIG_PAGE_OFFSET_80000000")]
pub const MSA_CLR: (u8, u8) = (31, 15);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
