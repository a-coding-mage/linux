/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The original header is excluded when building for the assembler.
 * Dependencies supplied by asm/barrier.h, asm/errata_list.h, and
 * asm/insn-def.h are referenced here as external symbols.
 */

unsafe extern "C" {
    fn ALT_RISCV_PAUSE();
    fn barrier();
}

#[inline]
pub unsafe fn cpu_relax() {
    // __riscv_muldiv: in lieu of a halt instruction, induce a long-latency stall.
    #[cfg(__riscv_muldiv)]
    {
        let mut dummy: i32;
        core::arch::asm!("div {0}, {0}, zero", out(reg) dummy);
    }

    /*
     * Reduce instruction retirement.
     * This assumes the PC changes.
     */
    ALT_RISCV_PAUSE();
    barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
