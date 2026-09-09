/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The original header has an assembler-only branch (__ASSEMBLER__).  The
 * assembler source is preserved here as a comment because it has no direct
 * executable Rust representation:
 *
 * kfr2r09 board specific boot code:
 * converts the "partner-jet-script.txt" script into assembly
 * the assembly code is the first code to be executed in the romImage
 *
 * Includes: <asm/romimage-macros.h>, <mach/partner-jet-setup.txt>
 *
 *     /* execute icbi after enabling cache */
 *     mov.l   1f, r0
 *     icbi    @r0
 *
 *     /* jump to cached area */
 *     mova    2f, r0
 *     jmp     @r0
 *      nop
 *
 *     .align 2
 * 1:  .long 0xa8000000
 * 2:
 */

/* Non-assembler branch of the original header. */
#[inline]
fn mmcif_update_progress(_nr: i32) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
