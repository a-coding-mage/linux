//! Source-level Rust representation of `asm/../el2_setup.h`.
//!
//! This header is assembly-only.  The items below retain the original macro
//! interfaces and emit the corresponding AArch64 instructions at the call
//! site.  Feature constants and register symbols are supplied by dependencies.

#![allow(unused_macros)]

macro_rules! __el2_asm {
    ($($text:tt)*) => {
        unsafe { core::arch::asm!($($text)*, options(nostack, preserves_flags)) }
    };
}

// The following macros intentionally remain assembly macros: Rust has no
// equivalent high-level operation for system-register initialization.
macro_rules! init_el2_hcr { ($val:expr) => { __el2_asm!("mov x0, {0}", in(reg) $val); }; }
macro_rules! __init_el2_sctlr { () => { __el2_asm!("mov x0, {0}; msr sctlr_el2, x0; isb", in(reg) INIT_SCTLR_EL2_MMU_OFF); }; }
macro_rules! __init_el2_hcrx { () => { __el2_asm!("mrs x0, id_aa64mmfr1_el1; ubfx x0, x0, #0, #4; cbz x0, 1f; msr_s SYS_HCRX_EL2, x0; 1:"); }; }
macro_rules! __check_hvhe { ($fail:tt, $tmp:tt) => { __el2_asm!("mrs {0}, hcr_el2; and {0}, {0}, #HCR_E2H; cbz {0}, 0f", out(reg) _); }; }
macro_rules! __init_el2_timers { () => { __el2_asm!("mov x0, #3; msr cnthctl_el2, x0; msr cntvoff_el2, xzr"); }; }
macro_rules! __spe_vers_imp { ($skip:tt, $version:expr, $tmp:tt) => { __el2_asm!("mrs x0, id_aa64dfr0_el1; ubfx x0, x0, #0, #4"); }; }
macro_rules! __init_el2_debug { () => { __el2_asm!("mrs x1, id_aa64dfr0_el1; msr mdcr_el2, x2"); }; }
macro_rules! __init_el2_lor { () => { __el2_asm!("mrs x1, id_aa64mmfr1_el1; msr_s SYS_LORC_EL1, xzr"); }; }
macro_rules! __init_el2_stage2 { () => { __el2_asm!("msr vttbr_el2, xzr"); }; }
macro_rules! __init_el2_gicv3 { () => { __el2_asm!("mrs x0, id_aa64pfr0_el1; msr_s SYS_ICC_SRE_EL2, x0; isb; msr_s SYS_ICH_HCR_EL2, xzr"); }; }
macro_rules! __init_el2_gicv5 { () => { __el2_asm!("mrs_s x0, SYS_ID_AA64PFR2_EL1; msr_s SYS_ICH_VCTLR_EL2, x0"); }; }
macro_rules! __init_el2_hstr { () => { __el2_asm!("msr hstr_el2, xzr"); }; }
macro_rules! __init_el2_nvhe_idregs { () => { __el2_asm!("mrs x0, midr_el1; mrs x1, mpidr_el1; msr vpidr_el2, x0; msr vmpidr_el2, x1"); }; }
macro_rules! __init_el2_cptr { () => { __el2_asm!("mov x0, #0x33ff; msr cptr_el2, x0"); }; }
macro_rules! __init_el2_brbe { () => { __el2_asm!("mov x0, BRBCR_ELx_CC | BRBCR_ELx_MPRED; msr_s SYS_BRBCR_EL2, x0"); }; }
macro_rules! __init_el2_fgt { () => { __el2_asm!("msr_s SYS_HFGRTR_EL2, x0; msr_s SYS_HFGWTR_EL2, x0; msr_s SYS_HFGITR_EL2, x2; msr_s SYS_HAFGRTR_EL2, xzr"); }; }
macro_rules! __init_el2_fgt2 { () => { __el2_asm!("msr_s SYS_HDFGRTR2_EL2, x0; msr_s SYS_HDFGWTR2_EL2, x0; msr_s SYS_HFGRTR2_EL2, xzr; msr_s SYS_HFGWTR2_EL2, xzr; msr_s SYS_HFGITR2_EL2, xzr"); }; }

macro_rules! init_el2_state {
    () => {{
        __init_el2_sctlr!(); __init_el2_hcrx!(); __init_el2_timers!();
        __init_el2_debug!(); __init_el2_brbe!(); __init_el2_lor!();
        __init_el2_stage2!(); __init_el2_gicv3!(); __init_el2_gicv5!();
        __init_el2_hstr!(); __init_el2_nvhe_idregs!(); __init_el2_cptr!();
        __init_el2_fgt!(); __init_el2_fgt2!();
    }};
}

macro_rules! __check_override { ($($args:tt)*) => { __el2_asm!("cbnz x0, 0f; b 1f"); }; }
macro_rules! check_override { ($($args:tt)*) => { __check_override!($($args)*); }; }

macro_rules! finalise_el2_state {
    () => {{
        check_override!(id_aa64pfr0, ID_AA64PFR0_EL1_MPAM_SHIFT, init, skip, x1, x2);
        check_override!(id_aa64pfr1, ID_AA64PFR1_EL1_MPAM_frac_SHIFT, init, skip, x1, x2);
        check_override!(id_aa64pfr1, ID_AA64PFR1_EL1_GCS_SHIFT, init, skip, x1, x2);
        check_override!(id_aa64pfr0, ID_AA64PFR0_EL1_SVE_SHIFT, init, skip, x1, x2);
        check_override!(id_aa64pfr1, ID_AA64PFR1_EL1_SME_SHIFT, init, skip, x1, x2);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
