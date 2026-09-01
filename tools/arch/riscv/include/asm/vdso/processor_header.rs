// SPDX-License-Identifier: GPL-2.0-only

// C header guard omitted.
// Original dependency: <asm-generic/barrier.h>

// Original code is excluded when __ASSEMBLER__ is defined.

#[inline]
pub unsafe fn cpu_relax() {
    // Original C condition: #ifdef __riscv_muldiv
    #[cfg(any(target_feature = "m", target_feature = "zmmul"))]
    {
        let dummy: isize;
        // In lieu of a halt instruction, induce a long-latency stall.
        core::arch::asm!("div {0}, {0}, zero", out(reg) dummy, options(nostack));
        let _ = dummy;
    }

    // Original C condition: #ifdef CONFIG_TOOLCHAIN_HAS_ZIHINTPAUSE
    #[cfg(CONFIG_TOOLCHAIN_HAS_ZIHINTPAUSE)]
    {
        /*
         * Reduce instruction retirement.
         * This assumes the PC changes.
         */
        core::arch::asm!("pause", options(nostack));
    }
    #[cfg(not(CONFIG_TOOLCHAIN_HAS_ZIHINTPAUSE))]
    {
        // Encoding of the pause instruction
        core::arch::asm!(".4byte 0x100000F", options(nostack));
    }

    barrier();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
