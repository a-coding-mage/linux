/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <asm/setup.h>
// #include <abi/regdef.h>

pub const LSAVE_PC: usize = 8;
pub const LSAVE_PSR: usize = 12;
pub const LSAVE_A0: usize = 24;
pub const LSAVE_A1: usize = 28;
pub const LSAVE_A2: usize = 32;
pub const LSAVE_A3: usize = 36;
pub const LSAVE_A4: usize = 40;
pub const LSAVE_A5: usize = 44;

// #define usp ss1
pub const usp: &str = "ss1";

// The following items preserve the C preprocessor assembly macros.  They are
// intentionally represented as Rust macro definitions because their bodies
// are target-specific Csky instructions rather than Rust operations.

macro_rules! USPTOKSP { () => {
    /* mtcr sp, usp; mfcr sp, ss0 */
}; }

macro_rules! KSPTOUSP { () => {
    /* mtcr sp, ss0; mfcr sp, usp */
}; }

macro_rules! SAVE_ALL { ($epc_inc:expr) => {
    /*
     * mtcr r13, ss2; mfcr r13, epsr; btsti r13, 31; bt 1f; USPTOKSP
     * subi sp, 32; subi sp, 32; subi sp, 16; stw r13, (sp, 12)
     * stw lr, (sp, 4); mfcr lr, epc; movi r13, $epc_inc; add lr, r13
     * stw lr, (sp, 8); mov lr, sp; addi lr, 32; addi lr, 32; addi lr, 16
     * bt 2f; mfcr lr, ss1; stw lr, (sp, 16); stw a0, (sp, 20)
     * stw a0, (sp, 24); stw a1, (sp, 28); stw a2, (sp, 32); stw a3, (sp, 36)
     * addi sp, 32; addi sp, 8; mfcr r13, ss2
     * stw r6, (sp); stw r7, (sp, 4); stw r8, (sp, 8); stw r9, (sp, 12)
     * stw r10, (sp, 16); stw r11, (sp, 20); stw r12, (sp, 24)
     * stw r13, (sp, 28); stw r14, (sp, 32); stw r1, (sp, 36)
     * subi sp, 32; subi sp, 8
     */
}; }

macro_rules! RESTORE_ALL { () => {
    /* Restore registers, exception state, user/kernel stack, then execute rte. */
}; }

macro_rules! SAVE_SWITCH_STACK { () => {
    /* subi sp, 32; stm r8-r15, (sp) */
}; }

macro_rules! RESTORE_SWITCH_STACK { () => {
    /* ldm r8-r15, (sp); addi sp, 32 */
}; }

macro_rules! RD_MIR { ($rx:ident) => { /* cprcr $rx, cpcr0 */ }; }
macro_rules! RD_MEH { ($rx:ident) => { /* cprcr $rx, cpcr4 */ }; }
macro_rules! RD_MCIR { ($rx:ident) => { /* cprcr $rx, cpcr8 */ }; }
macro_rules! RD_PGDR { ($rx:ident) => { /* cprcr $rx, cpcr29 */ }; }
macro_rules! WR_MEH { ($rx:ident) => { /* cpwcr $rx, cpcr4 */ }; }
macro_rules! WR_MCIR { ($rx:ident) => { /* cpwcr $rx, cpcr8 */ }; }

macro_rules! SETUP_MMU { () => {
    /*
     * Init psr and enable ee: lrw r6, DEFAULT_PSR_VALUE; mtcr r6, psr;
     * psrset ee. Select MMU: cpseti cp15.
     * cpcr30: cprcr r6, cpcr30; lsri r6, 29; lsli r6, 29; addi r6, 0xe;
     * cpwcr r6, cpcr30; movi r6, 0; cpwcr r6, cpcr31.
     */
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
