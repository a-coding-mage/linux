/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* The C header includes <linux/threads.h> for NR_CPUS. */

#[repr(C)]
pub struct mm_context_t {
    /* 8 bit MMU PID + Generation cycle */
    pub asid: [usize; NR_CPUS],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn do_tlb_overlap_fault(
        arg1: usize,
        arg2: usize,
        regs: *mut pt_regs,
    );
}

/* Declarations from <asm/mmu-arcv2.h> are supplied by the surrounding translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
