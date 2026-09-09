/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 16-bit ljmpw to the real_mode_seg
 *
 * This must be open-coded since gas will choke on using a
 * relocatable symbol for the segment portion.
 *
 * Original assembler macro (available only when __ASSEMBLER__ is defined):
 * #define LJMPW_RM(to) .byte 0xea ; .word (to), real_mode_seg
 */

/* Signature at the end of the realmode region */
pub const REALMODE_END_SIGNATURE: u32 = 0x65a2_2c82;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
