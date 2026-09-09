/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 04, 07 Ralf Baechle <ralf@linux-mips.org>
 * Copyright (C) MIPS Technologies, Inc.
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// Translation of the MIPS hazard header.  The original CONFIG_* conditions
// are represented by the corresponding Rust cfg feature names.

#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const ___SSNOP: &str = "sll $0, $0, 1";
pub const ___EHB: &str = "sll $0, $0, 3";

#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __MTC0_TLBW_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __MTC0_TLBR_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __TLBW_USE_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __TLB_READ_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __TLB_PROBE_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __IRQ_ENABLE_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __IRQ_DISABLE_HAZARD: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __BACK_TO_BACK_C0_HAZARD: &str = ___EHB;

#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
#[inline(always)]
pub unsafe fn instruction_hazard() {
    core::arch::asm!(
        ".set push", ".set {isa}", "dla {tmp}, 1f", "jr.hb {tmp}",
        ".set pop", "1:",
        isa = const "mips64r2", tmp = lateout(reg) _, options(nostack, preserves_flags)
    );
}

#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __MTC0_TLBW_HAZARD: &str = "nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __MTC0_TLBR_HAZARD: &str = "nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __TLBW_USE_HAZARD: &str = "nop; nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __TLB_READ_HAZARD: &str = "nop; nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __TLB_PROBE_HAZARD: &str = "nop; nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __IRQ_ENABLE_HAZARD: &str = "sll $0, $0, 1; sll $0, $0, 1; sll $0, $0, 1";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __IRQ_DISABLE_HAZARD: &str = "nop; nop; nop";
#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
pub const __BACK_TO_BACK_C0_HAZARD: &str = "sll $0, $0, 1; sll $0, $0, 1; sll $0, $0, 1";

#[cfg(not(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6")))]
#[inline(always)]
pub unsafe fn instruction_hazard() {}

#[cfg(feature = "CONFIG_CPU_SB1")]
pub const __ENABLE_FPU_HAZARD: &str = ".set push; .set mips64; .set noreorder; sll $0, $0, 1; bnezl $0, .+4; sll $0, $0, 1; .set pop";
#[cfg(not(feature = "CONFIG_CPU_SB1"))]
pub const __ENABLE_FPU_HAZARD: &str = "nop; nop; nop; nop";
#[cfg(any(feature = "CONFIG_CPU_MIPSR2", feature = "CONFIG_CPU_MIPSR5", feature = "CONFIG_CPU_MIPSR6"))]
pub const __ENABLE_FPU_HAZARD_R2: &str = ___EHB;
#[cfg(any(feature = "CONFIG_CPU_SB1"))]
pub const __DISABLE_FPU_HAZARD: &str = "";
#[cfg(not(feature = "CONFIG_CPU_SB1"))]
pub const __DISABLE_FPU_HAZARD: &str = ___EHB;

#[macro_export]
macro_rules! hazard_asm { ($code:expr) => {{ core::arch::asm!($code, options(nostack, preserves_flags)); }} }

#[inline(always)] pub unsafe fn _ssnop() { hazard_asm!("sll $0, $0, 1"); }
#[inline(always)] pub unsafe fn _ehb() { hazard_asm!("sll $0, $0, 3"); }
#[inline(always)] pub unsafe fn mtc0_tlbw_hazard() { hazard_asm!(__MTC0_TLBW_HAZARD); }
#[inline(always)] pub unsafe fn mtc0_tlbr_hazard() { hazard_asm!(__MTC0_TLBR_HAZARD); }
#[inline(always)] pub unsafe fn tlbw_use_hazard() { hazard_asm!(__TLBW_USE_HAZARD); }
#[inline(always)] pub unsafe fn tlb_read_hazard() { hazard_asm!(__TLB_READ_HAZARD); }
#[inline(always)] pub unsafe fn tlb_probe_hazard() { hazard_asm!(__TLB_PROBE_HAZARD); }
#[inline(always)] pub unsafe fn irq_enable_hazard() { hazard_asm!(__IRQ_ENABLE_HAZARD); }
#[inline(always)] pub unsafe fn irq_disable_hazard() { hazard_asm!(__IRQ_DISABLE_HAZARD); }
#[inline(always)] pub unsafe fn back_to_back_c0_hazard() { hazard_asm!(__BACK_TO_BACK_C0_HAZARD); }
#[inline(always)] pub unsafe fn enable_fpu_hazard() { hazard_asm!(__ENABLE_FPU_HAZARD); }
#[inline(always)] pub unsafe fn disable_fpu_hazard() { hazard_asm!(__DISABLE_FPU_HAZARD); }

/* MIPS R2 instruction hazard barrier. Needs to be called as a subroutine. */
extern "C" { pub fn mips_ihb(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
