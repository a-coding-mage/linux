/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of x86/include/asm/msr-index.h.
// The original header is a declaration-only register-index interface; the
// constants below retain its integer widths and names.  Kernel bit helpers
// are expressed locally so dependent code can use the same definitions.

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

macro_rules! BIT { ($n:expr) => { 1u64 << ($n) }; }
macro_rules! BIT_ULL { ($n:expr) => { 1u64 << ($n) }; }
macro_rules! GENMASK_ULL { ($h:expr, $l:expr) => { (((!0u64) << ($l)) & ((!0u64) >> (63 - ($h)))) }; }

pub const MSR_EFER: u32 = 0xc0000080;
pub const MSR_STAR: u32 = 0xc0000081;
pub const MSR_LSTAR: u32 = 0xc0000082;
pub const MSR_CSTAR: u32 = 0xc0000083;
pub const MSR_SYSCALL_MASK: u32 = 0xc0000084;
pub const MSR_FS_BASE: u32 = 0xc0000100;
pub const MSR_GS_BASE: u32 = 0xc0000101;
pub const MSR_KERNEL_GS_BASE: u32 = 0xc0000102;
pub const MSR_TSC_AUX: u32 = 0xc0000103;

pub const _EFER_SCE: u32 = 0;
pub const _EFER_LME: u32 = 8;
pub const _EFER_LMA: u32 = 10;
pub const _EFER_NX: u32 = 11;
pub const _EFER_SVME: u32 = 12;
pub const _EFER_LMSLE: u32 = 13;
pub const _EFER_FFXSR: u32 = 14;
pub const _EFER_TCE: u32 = 15;
pub const _EFER_AUTOIBRS: u32 = 21;
pub const EFER_SCE: u64 = BIT!(_EFER_SCE);
pub const EFER_LME: u64 = BIT!(_EFER_LME);
pub const EFER_LMA: u64 = BIT!(_EFER_LMA);
pub const EFER_NX: u64 = BIT!(_EFER_NX);
pub const EFER_SVME: u64 = BIT!(_EFER_SVME);
pub const EFER_LMSLE: u64 = BIT!(_EFER_LMSLE);
pub const EFER_FFXSR: u64 = BIT!(_EFER_FFXSR);
pub const EFER_TCE: u64 = BIT!(_EFER_TCE);
pub const EFER_AUTOIBRS: u64 = BIT!(_EFER_AUTOIBRS);

pub const X86_MEMTYPE_UC: u64 = 0;
pub const X86_MEMTYPE_WC: u64 = 1;
pub const X86_MEMTYPE_WT: u64 = 4;
pub const X86_MEMTYPE_WP: u64 = 5;
pub const X86_MEMTYPE_WB: u64 = 6;
pub const X86_MEMTYPE_UC_MINUS: u64 = 7;

pub const MSR_IA32_FRED_RSP0: u32 = 0x1cc;
pub const MSR_IA32_FRED_RSP1: u32 = 0x1cd;
pub const MSR_IA32_FRED_RSP2: u32 = 0x1ce;
pub const MSR_IA32_FRED_RSP3: u32 = 0x1cf;
pub const MSR_IA32_FRED_STKLVLS: u32 = 0x1d0;
pub const MSR_IA32_FRED_SSP1: u32 = 0x1d1;
pub const MSR_IA32_FRED_SSP2: u32 = 0x1d2;
pub const MSR_IA32_FRED_SSP3: u32 = 0x1d3;
pub const MSR_IA32_FRED_CONFIG: u32 = 0x1d4;

pub const MSR_TEST_CTRL: u32 = 0x33;
pub const MSR_TEST_CTRL_SPLIT_LOCK_DETECT_BIT: u32 = 29;
pub const MSR_TEST_CTRL_SPLIT_LOCK_DETECT: u64 = BIT!(MSR_TEST_CTRL_SPLIT_LOCK_DETECT_BIT);
pub const MSR_IA32_SPEC_CTRL: u32 = 0x48;
pub const SPEC_CTRL_IBRS: u64 = BIT!(0);
pub const SPEC_CTRL_STIBP_SHIFT: u32 = 1;
pub const SPEC_CTRL_STIBP: u64 = BIT!(SPEC_CTRL_STIBP_SHIFT);
pub const SPEC_CTRL_SSBD_SHIFT: u32 = 2;
pub const SPEC_CTRL_SSBD: u64 = BIT!(SPEC_CTRL_SSBD_SHIFT);
pub const SPEC_CTRL_RRSBA_DIS_S_SHIFT: u32 = 6;
pub const SPEC_CTRL_RRSBA_DIS_S: u64 = BIT!(SPEC_CTRL_RRSBA_DIS_S_SHIFT);
pub const SPEC_CTRL_BHI_DIS_S_SHIFT: u32 = 10;
pub const SPEC_CTRL_BHI_DIS_S: u64 = BIT!(SPEC_CTRL_BHI_DIS_S_SHIFT);
pub const SPEC_CTRL_MITIGATIONS_MASK: u64 = SPEC_CTRL_IBRS | SPEC_CTRL_STIBP | SPEC_CTRL_SSBD | SPEC_CTRL_RRSBA_DIS_S | SPEC_CTRL_BHI_DIS_S;

/* Remaining header definitions are intentionally exposed through the same
 * source-level names in dependent generated bindings. */
pub const MSR_IA32_FLUSH_CMD: u32 = 0x10b;
pub const L1D_FLUSH: u64 = BIT!(0);
pub const MSR_IA32_TSX_CTRL: u32 = 0x122;
pub const TSX_CTRL_RTM_DISABLE: u64 = BIT!(0);
pub const TSX_CTRL_CPUID_CLEAR: u64 = BIT!(1);
pub const MSR_IA32_MCG_CAP: u32 = 0x179;
pub const MSR_IA32_MCG_STATUS: u32 = 0x17a;
pub const MSR_IA32_MCG_CTL: u32 = 0x17b;
pub const MSR_IA32_APICBASE: u32 = 0x1b;
pub const MSR_IA32_APICBASE_BSP: u64 = 1 << 8;
pub const MSR_IA32_APICBASE_ENABLE: u64 = 1 << 11;
pub const MSR_IA32_APICBASE_BASE: u64 = 0xfffff << 12;
pub const MSR_IA32_TSC: u32 = 0x10;
pub const MSR_IA32_FEAT_CTL: u32 = 0x3a;
pub const FEAT_CTL_LOCKED: u64 = BIT!(0);
pub const FEAT_CTL_VMX_ENABLED_INSIDE_SMX: u64 = BIT!(1);
pub const FEAT_CTL_VMX_ENABLED_OUTSIDE_SMX: u64 = BIT!(2);
pub const FEAT_CTL_SGX_LC_ENABLED: u64 = BIT!(17);
pub const FEAT_CTL_SGX_ENABLED: u64 = BIT!(18);
pub const FEAT_CTL_LMCE_ENABLED: u64 = BIT!(20);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
