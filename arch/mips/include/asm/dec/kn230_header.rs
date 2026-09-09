/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/kn230.h
 *
 *	DECsystem 5100 (MIPSmate or KN230) definitions.
 *
 *	Copyright (C) 2002, 2003  Maciej W. Rozycki
 */

/*
 * CPU interrupt bits.
 */
pub const KN230_CPU_INR_HALT: i32 = 6; /* HALT button */
pub const KN230_CPU_INR_BUS: i32 = 5; /* memory, I/O bus read/write errors */
pub const KN230_CPU_INR_RTC: i32 = 4; /* DS1287 RTC */
pub const KN230_CPU_INR_SII: i32 = 3; /* SII (DC7061) SCSI */
pub const KN230_CPU_INR_LANCE: i32 = 3; /* LANCE (Am7990) Ethernet */
pub const KN230_CPU_INR_DZ11: i32 = 2; /* DZ11 (DC7085) serial */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
