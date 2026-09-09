/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2002 by Ralf Baechle
 */

/*
 * TLB debugging functions:
 */
unsafe extern "C" {
    pub fn dump_tlb_regs();
    pub fn dump_tlb_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
