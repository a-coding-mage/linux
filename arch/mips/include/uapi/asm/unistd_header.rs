/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 *
 * Changed system calls macros _syscall5 - _syscall7 to push args 5 to 7 onto
 * the stack. Robin Farine for ACN S.A, Copyright (C) 1996 by ACN S.A
 */

// C dependency: <asm/sgidefs.h>

// The following feature names represent the corresponding C preprocessor
// conditions: _MIPS_SIM == _MIPS_SIM_ABI32, ABI64, and NABI32.

#[cfg(feature = "mips_sim_abi32")]
pub const __NR_Linux: i32 = 4000;
// C dependency included under this condition: <asm/unistd_o32.h>

#[cfg(feature = "mips_sim_abi64")]
pub const __NR_Linux: i32 = 5000;
// C dependency included under this condition: <asm/unistd_n64.h>

#[cfg(feature = "mips_sim_nabi32")]
pub const __NR_Linux: i32 = 6000;
// C dependency included under this condition: <asm/unistd_n32.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
