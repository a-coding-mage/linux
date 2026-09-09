/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency supplied by linux/pagemap.h and asm-generic/tlb.h in the source.

unsafe extern "C" {
    pub fn set_mmu_pid(pid: ::core::ffi::c_ulong);
}

/*
 * NIOS32 does have flush_tlb_range(), but it lacks a limit and fallback to
 * full mm invalidation. So use flush_tlb_mm() for everything.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
