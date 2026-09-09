/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for CPU-MF instructions
 *
 * Copyright IBM Corp. 2019
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

/*
 * The source declaration is assembler-only.  It defines the STCCTM
 * instruction with a customized M3 field designating the counter set:
 *
 * .macro STCCTM r1 m3 db2
 *     .insn rsy,0xeb0000000017,\\r1,\\m3 & 0xf,\\db2
 * .endm
 *
 * No direct Rust item can represent an assembler macro without the
 * surrounding assembler interface; preserve the instruction encoding here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
