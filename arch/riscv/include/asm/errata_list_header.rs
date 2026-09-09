/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2021 Sifive. */

// Translated from asm/errata_list.h.
// Dependencies supplied by the surrounding translation unit:
// asm/csr.h, asm/insn-def.h, asm/hwcap.h, asm/vendorid_list.h,
// asm/errata_list_vendors.h, and asm/vendor_extensions/mips.h.

// The following alternatives are assembler-only in the C header.
#[cfg(any())]
macro_rules! ALT_INSN_FAULT {
    ($x:expr) => {
        ALTERNATIVE!(
            __stringify!(RISCV_PTR do_trap_insn_fault),
            __stringify!(RISCV_PTR sifive_cip_453_insn_fault_trp),
            SIFIVE_VENDOR_ID, ERRATA_SIFIVE_CIP_453,
            CONFIG_ERRATA_SIFIVE_CIP_453
        )
    };
}

#[cfg(any())]
macro_rules! ALT_PAGE_FAULT {
    ($x:expr) => {
        ALTERNATIVE!(
            __stringify!(RISCV_PTR do_page_fault),
            __stringify!(RISCV_PTR sifive_cip_453_page_fault_trp),
            SIFIVE_VENDOR_ID, ERRATA_SIFIVE_CIP_453,
            CONFIG_ERRATA_SIFIVE_CIP_453
        )
    };
}

// C inline-assembly alternatives retained as Rust macro forms.  The
// ALTERNATIVE and assembler symbols are provided by other translated files.
macro_rules! ALT_SFENCE_VMA_ASID {
    ($asid:expr) => {{
        unsafe { core::arch::asm!("sfence.vma x0, {0}", in(reg) $asid, options(nostack)) }
    }};
}

macro_rules! ALT_SFENCE_VMA_ADDR {
    ($addr:expr) => {{
        unsafe { core::arch::asm!("sfence.vma {0}", in(reg) $addr, options(nostack)) }
    }};
}

macro_rules! ALT_SFENCE_VMA_ADDR_ASID {
    ($addr:expr, $asid:expr) => {{
        unsafe { core::arch::asm!("sfence.vma {0}, {1}", in(reg) $addr, in(reg) $asid, options(nostack)) }
    }};
}

macro_rules! ALT_RISCV_PAUSE {
    () => {{
        unsafe { core::arch::asm!("pause", options(nostack)) }
    }};
}

/* _val is marked as will be overwritten, so it is set to 0 by default. */
pub const ALT_SVPBMT_SHIFT: u32 = 61;
pub const ALT_THEAD_MAE_SHIFT: u32 = 59;

macro_rules! ALT_SVPBMT {
    ($val:expr, $prot:expr) => {{
        // Build-time ALTERNATIVE_2 selection is supplied by the target kernel.
        let _ = (&mut $val, $prot, ALT_SVPBMT_SHIFT, ALT_THEAD_MAE_SHIFT);
    }};
}

// CONFIG_ERRATA_THEAD_MAE controls whether the PMA alternative exists.
#[cfg(feature = "CONFIG_ERRATA_THEAD_MAE")]
macro_rules! ALT_THEAD_PMA {
    ($val:expr) => {{
        // The original performs the T-Head PMA test and update using t3.
        unsafe {
            core::arch::asm!(
                "li t3, 0",
                inout(reg) $val,
                out("t3") _,
                options(nostack)
            )
        }
    }};
}

#[cfg(not(feature = "CONFIG_ERRATA_THEAD_MAE"))]
macro_rules! ALT_THEAD_PMA {
    ($val:expr) => {{ let _ = &$val; }};
}

macro_rules! ALT_CMO_OP {
    ($op:ident, $start:expr, $size:expr, $cachesize:expr) => {{
        // The original emits CBO_##op in an ALTERNATIVE loop when ZICBOM is enabled.
        let _ = ($op, $start, $size, $cachesize);
    }};
}

pub const THEAD_C9XX_RV_IRQ_PMU: u32 = 17;
pub const THEAD_C9XX_CSR_SCOUNTEROF: u32 = 0x5c5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
