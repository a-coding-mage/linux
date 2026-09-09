/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000 Silicon Graphics, Inc.
 * Copyright (C) 2005 Ralf Baechle <ralf@linux-mips.org>
 */

// Dependencies supplied by the surrounding MIPS IP27 environment:
// asm/sn/addrs.h, asm/sn/agent.h, and asm/sn/klkernvars.h.

/* TLB bits */
pub const PAGE_GLOBAL: u64 = 1 << 6;
pub const PAGE_VALID: u64 = 1 << 7;
pub const PAGE_DIRTY: u64 = 1 << 8;
pub const CACHE_CACHABLE_COW: u64 = 5 << 9;

/*
 * Inputs are the text nasid in t1 and data nasid in t2.
 *
 * This is a direct Rust representation of the original assembler macro.  The
 * assembler operations and symbols are intentionally left for the MIPS
 * target/toolchain to resolve.
 */
#[macro_export]
macro_rules! mapped_kernel_setup_tlb {
    () => {{
        #[cfg(feature = "CONFIG_MAPPED_KERNEL")]
        unsafe {
            core::arch::asm!(
                "dli t0, 0xffffffffc0000000",
                "dmtc0 t0, CP0_ENTRYHI",
                "li t0, 0x1c000",
                "dsll t1, NASID_SHFT",
                "dsll t2, NASID_SHFT",
                "or t1, t1, t0",
                "or t2, t2, t0",
                "dsrl t1, 12",
                "dsrl t2, 12",
                "dsll t1, 6",
                "dsll t2, 6",
                "li t0, ((PAGE_GLOBAL | PAGE_VALID | CACHE_CACHABLE_COW) >> 6)",
                "or t0, t0, t1",
                "mtc0 t0, CP0_ENTRYLO0",
                "li t0, ((PAGE_GLOBAL | PAGE_VALID | PAGE_DIRTY | CACHE_CACHABLE_COW) >> 6)",
                "or t0, t0, t2",
                "mtc0 t0, CP0_ENTRYLO1",
                "li t0, 0x1ffe000",
                "mtc0 t0, CP0_PAGEMASK",
                "li t0, 0",
                "mtc0 t0, CP0_INDEX",
                "li t0, 1",
                "mtc0 t0, CP0_WIRED",
                "tlbwi",
            );
        }
        #[cfg(not(feature = "CONFIG_MAPPED_KERNEL"))]
        unsafe {
            core::arch::asm!("mtc0 zero, CP0_WIRED");
        }
    }};
}

/* Intentionally empty extension point, used in head.S. */
#[macro_export]
macro_rules! kernel_entry_setup {
    () => {{
        unsafe {
            core::arch::asm!(
                "GET_NASID_ASM t1",
                "move t2, t1",
            );
        }
        mapped_kernel_setup_tlb!();
    }};
}

/* Do SMP slave processor setup necessary before safely executing C code. */
#[macro_export]
macro_rules! smp_slave_setup {
    () => {{
        unsafe {
            core::arch::asm!(
                "GET_NASID_ASM t1",
                "dli t0, KLDIR_OFFSET + (KLI_KERN_VARS * KLDIR_ENT_SIZE) + KLDIR_OFF_POINTER + CAC_BASE",
                "dsll t1, NASID_SHFT",
                "or t0, t0, t1",
                "ld t0, 0(t0)",
                "lh t1, KV_RO_NASID_OFFSET(t0)",
                "lh t2, KV_RW_NASID_OFFSET(t0)",
            );
        }
        mapped_kernel_setup_tlb!();
        unsafe {
            core::arch::asm!(
                "PTR_LA t0, 0f",
                "jr t0",
                "0:",
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
