/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from m68k/include/asm/entry.h.
 * The source contains m68k assembler preprocessor macros; their bodies are
 * preserved below as comments because they have no direct Rust syntax.
 */

/*
 * Stack layout in `ret_from_exception`:
 *
 * This allows access to the syscall arguments in registers d1-d5.
 *
 *  0(sp) - d1       4(sp) - d2       8(sp) - d3       C(sp) - d4
 * 10(sp) - d5      14(sp) - a0      18(sp) - a1      1C(sp) - a2
 * 20(sp) - d0      24(sp) - orig_d0 28(sp) - stack adjustment
 * 2C(sp) - [ sr              ] [ format & vector ]
 * 2E(sp) - [ pc-hiword       ] [ sr              ]
 * 30(sp) - [ pc-loword       ] [ pc-hiword       ]
 * 32(sp) - [ format & vector ] [ pc-loword       ]
 *           ^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^
 *                  M68K              COLDFIRE
 */

/* The following macro is used when enabling interrupts. */
#[cfg(mach_atari_only)]
pub const ALLOWINT: u32 = !0x500u32;

#[cfg(not(mach_atari_only))]
pub const ALLOWINT: u32 = !0x700u32;

#[cfg(assembler)]
pub const SWITCH_STACK_SIZE: usize = 6 * 4 + 4;

/*
 * The remaining source declarations are assembler-only macros.  They are
 * retained verbatim in structured comments to preserve their operations,
 * control flow, register usage, and conditional intent for the assembler
 * integration that consumes this header.
 *
 * #if CONFIG_COLDFIRE && CONFIG_COLDFIRE_SW_A7
 * SAVE_ALL_SYS:
 *   move #0x2700,%sr; btst #5,%sp@(2); bnes 6f; movel %sp,sw_usp;
 *   addql #8,sw_usp; movel sw_ksp,%sp; subql #8,%sp; clrl %sp@-;
 *   movel %d0,%sp@-; movel %d0,%sp@-; lea %sp@(-32),%sp;
 *   moveml %d1-%d5/%a0-%a2,%sp@; movel sw_usp,%a0;
 *   movel %a0@-,%sp@(PT_OFF_PC); movel %a0@-,%sp@(PT_OFF_FORMATVEC);
 *   bra 7f; 6: clrl %sp@-; movel %d0,%sp@-; movel %d0,%sp@-;
 *   lea %sp@(-32),%sp; moveml %d1-%d5/%a0-%a2,%sp@; 7:
 * SAVE_ALL_INT: SAVE_ALL_SYS; moveq #-1,%d0; movel %d0,%sp@(PT_OFF_ORIG_D0)
 * RESTORE_USER: disable interrupts; copy PC and format/vector; restore
 *   registers and stack fields; restore usp; rte
 * RDUSP: movel sw_usp,%a3
 * WRUSP: movel %a3,sw_usp
 *
 * #if CONFIG_COLDFIRE && !CONFIG_COLDFIRE_SW_A7
 * SAVE_ALL_SYS, SAVE_ALL_INT, RESTORE_USER, RDUSP, and WRUSP perform the
 * corresponding ColdFire stack/register operations from the C header.
 *
 * #if !CONFIG_COLDFIRE
 * SAVE_ALL_INT, SAVE_ALL_SYS, RESTORE_ALL, SAVE_SWITCH_STACK, and
 * RESTORE_SWITCH_STACK perform the m68k stack/register operations from the
 * C header.
 *
 * #if CONFIG_MMU
 * curptr is register a2. GET_CURRENT(tmp) invokes get_current(tmp), which
 * masks the stack pointer by -THREAD_SIZE and loads the current task through
 * curptr.
 * #else
 * GET_CURRENT(tmp) expands to nothing.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
