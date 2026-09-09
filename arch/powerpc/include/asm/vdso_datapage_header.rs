/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * This file is the Rust translation of the kernel-only PowerPC vDSO data-page
 * header.  The C header guard and preprocessor conditions are represented by
 * this module's source-level conditional intent.
 */

/*
 * Copyright (C) 2002 Peter Bergner <bergner@vnet.ibm.com>, IBM
 * Copyright (C) 2005 Benjamin Herrenschmidy <benh@kernel.crashing.org>,
 *                  IBM Corp.
 */

/*
 * C source dependency:
 *   #include <vdso/datapage.h>
 *
 * The declarations supplied by that header remain external dependencies of
 * this translation and are intentionally not redefined here.
 */

/*
 * The following assembler macro is present only for __ASSEMBLER__ builds in
 * the original header.  Rust code does not execute assembler preprocessor
 * macros; the exact PowerPC instruction sequence is retained here as a
 * source-level record of that conditional interface:
 *
 * .macro get_datapage ptr symbol
 *     bcl     20, 31, .+4
 * 999:
 *     mflr    \ptr
 *     addis   \ptr, \ptr, (\symbol - 999b)@ha
 *     addi    \ptr, \ptr, (\symbol - 999b)@l
 * .endm
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
