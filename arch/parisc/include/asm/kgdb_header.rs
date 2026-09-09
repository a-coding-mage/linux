/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PA-RISC KGDB support
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 *
 */

pub const BREAK_INSTR_SIZE: usize = 4;
pub const PARISC_KGDB_COMPILED_BREAK_INSN: u32 = 0x3ffc01f;
pub const PARISC_KGDB_BREAK_INSN: u32 = 0x3ffa01f;

pub const NUMREGBYTES: usize = core::mem::size_of::<parisc_gdb_regs>();
pub const BUFMAX: usize = 4096;

pub const KGDB_MAX_BREAKPOINTS: usize = 40;

pub const CACHE_FLUSH_IS_SAFE: usize = 1;

pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!(
        ".word {0}",
        const PARISC_KGDB_COMPILED_BREAK_INSN,
        options(nostack)
    );
}

#[repr(C)]
pub struct parisc_gdb_regs {
    pub gpr: [usize; 32],
    pub sar: usize,
    pub iaoq_f: usize,
    pub iasq_f: usize,
    pub iaoq_b: usize,
    pub iasq_b: usize,
    pub eiem: usize,
    pub iir: usize,
    pub isr: usize,
    pub ior: usize,
    pub ipsw: usize,
    pub __unused0: usize,
    pub sr4: usize,
    pub sr0: usize,
    pub sr1: usize,
    pub sr2: usize,
    pub sr3: usize,
    pub sr5: usize,
    pub sr6: usize,
    pub sr7: usize,
    pub cr0: usize,
    pub pid1: usize,
    pub pid2: usize,
    pub scrccr: usize,
    pub pid3: usize,
    pub pid4: usize,
    pub cr24: usize,
    pub cr25: usize,
    pub cr26: usize,
    pub cr27: usize,
    pub cr28: usize,
    pub cr29: usize,
    pub cr30: usize,
    pub fr: [u64; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
