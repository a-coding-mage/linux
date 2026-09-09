/*
 * The PowerPC (32/64) specific defines / externs for KGDB.  Based on
 * the previous 32bit and 64bit specific files, which had the following
 * copyrights:
 *
 * PPC64 Mods (C) 2005 Frank Rowand (frowand@mvista.com)
 * PPC Mods (C) 2004 Tom Rini (trini@mvista.com)
 * PPC Mods (C) 2003 John Whitney (john.whitney@timesys.com)
 * PPC Mods (C) 1998 Michael Tesch (tesch@cs.wisc.edu)
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Author: Tom Rini <trini@kernel.crashing.org>
 *
 * 2006 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// The original declarations are enabled only for the kernel, and omitted
// for assembler sources.

pub const BREAK_INSTR_SIZE: usize = 4;
pub const BREAK_INSTR: u32 = 0x7d82_1008; /* twge r2, r2 */

/* NUMREGBYTES is supplied by the selected PowerPC configuration below. */

#[cfg(feature = "CONFIG_PPC64")]
pub const NUMREGBYTES: usize = (68 * 8) + (3 * 4);

#[cfg(feature = "CONFIG_PPC64")]
pub const NUMCRITREGBYTES: usize = 184;

pub const DBG_MAX_REG_NUM: usize = 70;

#[cfg(all(not(feature = "CONFIG_PPC64"), not(feature = "CONFIG_PPC_E500")))]
pub const MAXREG: usize = (PT_FPSCR + 1);

#[cfg(all(not(feature = "CONFIG_PPC64"), feature = "CONFIG_PPC_E500"))]
pub const MAXREG: usize = ((32 * 2) + 6 + 2 + 1);

#[cfg(not(feature = "CONFIG_PPC64"))]
pub const NUMREGBYTES: usize = MAXREG * core::mem::size_of::<i32>();

#[cfg(not(feature = "CONFIG_PPC64"))]
pub const NUMCRITREGBYTES: usize = 23 * core::mem::size_of::<i32>();

pub const BUFMAX: usize = (NUMREGBYTES * 2) + 512;
pub const CACHE_FLUSH_IS_SAFE: usize = 1;

#[inline]
pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!(".long 0x7d821008");
}

/*
 * The number bytes of registers we have to save depends on a few
 * things. For 64bit we default to not including vector registers and
 * vector state registers.
 *
 * 64 bit (8 byte) registers:
 *   32 gpr, 32 fpr, nip, msr, link, ctr
 * 32 bit (4 byte) registers:
 *   ccr, xer, fpscr
 *
 * On non-E500 family PPC32 we determine the size by picking the last
 * register we need, but on E500 we skip sections so we list what we
 * need to store, and add it up.
 * 32 GPRs (8 bytes), nip, msr, ccr, link, ctr, xer, acc (8 bytes), spefscr
 * CR/LR, R1, R2, R13-R31 inclusive.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
