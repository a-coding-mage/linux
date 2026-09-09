/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Chris Dearman (chris@mips.com)
 * Copyright (C) 2007 Mips Technologies, Inc.
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

// The original header defines assembler macros. These Rust macros preserve
// the emitted instruction sequences for use in MIPS kernel-entry code.

/// Prepare segments for EVA boot.
#[macro_export]
macro_rules! platform_eva_init {
    () => {{
        unsafe {
            core::arch::asm!(
                "mfc0 {t1}, $16",
                "andi {t1}, {t1}, 0x7",
                "move {t2}, {t1}",
                "ins {t2}, {t1}, 16, 3",
                "li {t0}, (((MIPS_SEGCFG_MK << MIPS_SEGCFG_AM_SHIFT) | (0 << MIPS_SEGCFG_PA_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) | (((MIPS_SEGCFG_MK << MIPS_SEGCFG_AM_SHIFT) | (0 << MIPS_SEGCFG_PA_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) << 16))",
                "or {t0}, {t0}, {t2}",
                "mtc0 {t0}, $5",
                "li {t0}, (((MIPS_SEGCFG_MUSUK << MIPS_SEGCFG_AM_SHIFT) | (0 << MIPS_SEGCFG_PA_SHIFT) | (2 << MIPS_SEGCFG_C_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) | (((MIPS_SEGCFG_MUSUK << MIPS_SEGCFG_AM_SHIFT) | (0 << MIPS_SEGCFG_PA_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) << 16))",
                "ins {t0}, {t1}, 16, 3",
                "mtc0 {t0}, $6",
                "li {t0}, (((MIPS_SEGCFG_MUSUK << MIPS_SEGCFG_AM_SHIFT) | (6 << MIPS_SEGCFG_PA_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) | (((MIPS_SEGCFG_MUSUK << MIPS_SEGCFG_AM_SHIFT) | (4 << MIPS_SEGCFG_PA_SHIFT) | (1 << MIPS_SEGCFG_EU_SHIFT)) << 16))",
                "or {t0}, {t0}, {t2}",
                "mtc0 {t0}, $7",
                "jal mips_ihb",
                "mfc0 {t0}, $16, 5",
                "li {t2}, 0x40000000",
                "or {t0}, {t0}, {t2}",
                "mtc0 {t0}, $16, 5",
                "sync",
                "jal mips_ihb",
                t0 = out(reg) _, t1 = out(reg) _, t2 = out(reg) _,
                options(nostack)
            );
        }
    }};
}

/// Kernel entry setup, including the CONFIG_EVA processor check and YAMON
/// failure path from the original assembler macro.
#[macro_export]
macro_rules! kernel_entry_setup {
    () => {{
        #[cfg(feature = "CONFIG_EVA")]
        unsafe {
            core::arch::asm!(
                "sync", "ehb",
                "mfc0 $9, $16", "bgez $9, 9f",
                "mfc0 $8, $16, 1", "bgez $8, 9f",
                "mfc0 $8, $16, 2", "bgez $8, 9f",
                "mfc0 $8, $16, 3", "sll $8, $8, 6", "bgez $8, 9f",
                "b 0f",
                "9: b 1b", "nop", "1: b 1b", "nop",
                options(nostack)
            );
            $crate::platform_eva_init!();
        }
    }};
}

/// SMP slave processor setup necessary before safely executing C code.
#[macro_export]
macro_rules! smp_slave_setup {
    () => {{
        #[cfg(feature = "CONFIG_EVA")]
        {
            unsafe { core::arch::asm!("sync", "ehb", options(nostack)); }
            $crate::platform_eva_init!();
        }
    }};
}

// Original __INITDATA string, used by the YAMON failure path.
#[allow(dead_code)]
pub static NONSC_PROCESSOR: &[u8] =
    b"EVA kernel requires a MIPS core with Segment Control implemented\n\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
