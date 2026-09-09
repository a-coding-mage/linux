/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

// Translated from the C header guard: _ASM_POWERPC_EXEC_H

unsafe extern "C" {
    pub fn arch_align_stack(sp: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
