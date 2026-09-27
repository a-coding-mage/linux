/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2009
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Original include guard: __ASM_EXEC_H

extern "C" {
    pub fn arch_align_stack(sp: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
