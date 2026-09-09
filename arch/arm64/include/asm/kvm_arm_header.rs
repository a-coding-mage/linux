/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// C header dependencies are supplied by other translated units.

/* Because the original header maps names inherited from AArch32 to the
 * AArch64 nomenclature, retain the mapping as Rust macros. */
macro_rules! __HCR { ($x:ident) => { HCR_EL2_$x }; }

macro_rules! HCR_TID5 { () => { __HCR!(TID5) }; }
macro_rules! HCR_DCT { () => { __HCR!(DCT) }; }
macro_rules! HCR_ATA_SHIFT { () => { __HCR!(ATA_SHIFT) }; }
macro_rules! HCR_ATA { () => { __HCR!(ATA) }; }
macro_rules! HCR_TTLBOS { () => { __HCR!(TTLBOS) }; }
macro_rules! HCR_TTLBIS { () => { __HCR!(TTLBIS) }; }
macro_rules! HCR_ENSCXT { () => { __HCR!(EnSCXT) }; }
macro_rules! HCR_TOCU { () => { __HCR!(TOCU) }; }
macro_rules! HCR_AMVOFFEN { () => { __HCR!(AMVOFFEN) }; }
macro_rules! HCR_TICAB { () => { __HCR!(TICAB) }; }
macro_rules! HCR_TID4 { () => { __HCR!(TID4) }; }
macro_rules! HCR_FIEN { () => { __HCR!(FIEN) }; }
macro_rules! HCR_FWB { () => { __HCR!(FWB) }; }
macro_rules! HCR_NV2 { () => { __HCR!(NV2) }; }
macro_rules! HCR_AT { () => { __HCR!(AT) }; }
macro_rules! HCR_NV1 { () => { __HCR!(NV1) }; }
macro_rules! HCR_NV { () => { __HCR!(NV) }; }
macro_rules! HCR_API { () => { __HCR!(API) }; }
macro_rules! HCR_APK { () => { __HCR!(APK) }; }
macro_rules! HCR_TEA { () => { __HCR!(TEA) }; }
macro_rules! HCR_TERR { () => { __HCR!(TERR) }; }
macro_rules! HCR_TLOR { () => { __HCR!(TLOR) }; }
macro_rules! HCR_E2H { () => { __HCR!(E2H) }; }
macro_rules! HCR_ID { () => { __HCR!(ID) }; }
macro_rules! HCR_CD { () => { __HCR!(CD) }; }
macro_rules! HCR_RW { () => { __HCR!(RW) }; }
macro_rules! HCR_TRVM { () => { __HCR!(TRVM) }; }
macro_rules! HCR_HCD { () => { __HCR!(HCD) }; }
macro_rules! HCR_TDZ { () => { __HCR!(TDZ) }; }
macro_rules! HCR_TGE { () => { __HCR!(TGE) }; }
macro_rules! HCR_TVM { () => { __HCR!(TVM) }; }
macro_rules! HCR_TTLB { () => { __HCR!(TTLB) }; }
macro_rules! HCR_TPU { () => { __HCR!(TPU) }; }
macro_rules! HCR_TPC { () => { __HCR!(TPCP) }; }
macro_rules! HCR_TSW { () => { __HCR!(TSW) }; }
macro_rules! HCR_TACR { () => { __HCR!(TACR) }; }
macro_rules! HCR_TIDCP { () => { __HCR!(TIDCP) }; }
macro_rules! HCR_TSC { () => { __HCR!(TSC) }; }
macro_rules! HCR_TID3 { () => { __HCR!(TID3) }; }
macro_rules! HCR_TID2 { () => { __HCR!(TID2) }; }
macro_rules! HCR_TID1 { () => { __HCR!(TID1) }; }
macro_rules! HCR_TID0 { () => { __HCR!(TID0) }; }
macro_rules! HCR_TWE { () => { __HCR!(TWE) }; }
macro_rules! HCR_TWI { () => { __HCR!(TWI) }; }
macro_rules! HCR_DC { () => { __HCR!(DC) }; }
macro_rules! HCR_BSU { () => { __HCR!(BSU) }; }
macro_rules! HCR_BSU_IS { () => { __HCR!(BSU_IS) }; }
macro_rules! HCR_FB { () => { __HCR!(FB) }; }
macro_rules! HCR_VSE { () => { __HCR!(VSE) }; }
macro_rules! HCR_VI { () => { __HCR!(VI) }; }
macro_rules! HCR_VF { () => { __HCR!(VF) }; }
macro_rules! HCR_AMO { () => { __HCR!(AMO) }; }
macro_rules! HCR_IMO { () => { __HCR!(IMO) }; }
macro_rules! HCR_FMO { () => { __HCR!(FMO) }; }
macro_rules! HCR_PTW { () => { __HCR!(PTW) }; }
macro_rules! HCR_SWIO { () => { __HCR!(SWIO) }; }
macro_rules! HCR_VM { () => { __HCR!(VM) }; }

macro_rules! HCR_GUEST_FLAGS { () => { HCR_TSC!() | HCR_TSW!() | HCR_TWE!() | HCR_TWI!() | HCR_VM!() | HCR_BSU_IS!() | HCR_FB!() | HCR_TACR!() | HCR_AMO!() | HCR_SWIO!() | HCR_TIDCP!() | HCR_RW!() | HCR_TLOR!() | HCR_FMO!() | HCR_IMO!() | HCR_PTW!() | HCR_TID3!() | HCR_TID1!() }; }
macro_rules! HCR_HOST_NVHE_FLAGS { () => { HCR_RW!() | HCR_API!() | HCR_APK!() }; }
macro_rules! HCR_HOST_NVHE_PROTECTED_FLAGS { () => { HCR_HOST_NVHE_FLAGS!() | HCR_TSC!() }; }
macro_rules! HCR_HOST_VHE_FLAGS { () => { HCR_RW!() | HCR_TGE!() | HCR_E2H!() | HCR_AMO!() | HCR_IMO!() | HCR_FMO!() }; }

pub const MPAMHCR_HOST_FLAGS: u64 = 0;
pub const TCR_EL2_DS: u64 = 1u64 << 32;
pub const TCR_EL2_RES1: u64 = (1u64 << 31) | (1u64 << 23);
pub const TCR_EL2_HPD: u64 = 1 << 24;
pub const TCR_EL2_HA: u64 = 1 << 21;
pub const TCR_EL2_TBI: u64 = 1 << 20;
pub const TCR_EL2_PS_SHIFT: u32 = 16;
macro_rules! TCR_EL2_PS_MASK { () => { 7u64 << TCR_EL2_PS_SHIFT }; }
macro_rules! TCR_EL2_PS_40B { () => { 2u64 << TCR_EL2_PS_SHIFT }; }
macro_rules! TCR_EL2_TG0_MASK { () => { TCR_TG0_MASK }; }
macro_rules! TCR_EL2_SH0_MASK { () => { TCR_SH0_MASK }; }
macro_rules! TCR_EL2_ORGN0_MASK { () => { TCR_ORGN0_MASK }; }
macro_rules! TCR_EL2_IRGN0_MASK { () => { TCR_IRGN0_MASK }; }
pub const TCR_EL2_T0SZ_MASK: u64 = 0x3f;
macro_rules! TCR_EL2_MASK { () => { TCR_EL2_TG0_MASK!() | TCR_EL2_SH0_MASK!() | TCR_EL2_ORGN0_MASK!() | TCR_EL2_IRGN0_MASK!() }; }

// Page-size configuration is a build-time condition; the 4K branch is the default.
#[cfg(CONFIG_ARM64_64K_PAGES)]
pub const VTCR_EL2_TGRAN: u64 = 64;
#[cfg(CONFIG_ARM64_64K_PAGES)]
pub const VTCR_EL2_TGRAN_SL0_BASE: u64 = 3;
#[cfg(all(not(CONFIG_ARM64_64K_PAGES), CONFIG_ARM64_16K_PAGES))]
pub const VTCR_EL2_TGRAN: u64 = 16;
#[cfg(all(not(CONFIG_ARM64_64K_PAGES), CONFIG_ARM64_16K_PAGES))]
pub const VTCR_EL2_TGRAN_SL0_BASE: u64 = 3;
#[cfg(all(not(CONFIG_ARM64_64K_PAGES), not(CONFIG_ARM64_16K_PAGES)))]
pub const VTCR_EL2_TGRAN: u64 = 4;
#[cfg(all(not(CONFIG_ARM64_64K_PAGES), not(CONFIG_ARM64_16K_PAGES)))]
pub const VTCR_EL2_TGRAN_SL0_BASE: u64 = 2;

macro_rules! VTCR_EL2_LVLS_TO_SL0 { ($levels:expr) => { FIELD_PREP(VTCR_EL2_SL0, VTCR_EL2_TGRAN_SL0_BASE - (4 - ($levels))) }; }
macro_rules! VTCR_EL2_SL0_TO_LVLS { ($sl0:expr) => { ($sl0) + 4 - VTCR_EL2_TGRAN_SL0_BASE }; }
macro_rules! VTCR_EL2_LVLS { ($vtcr:expr) => { VTCR_EL2_SL0_TO_LVLS!(FIELD_GET(VTCR_EL2_SL0, $vtcr)) }; }
macro_rules! VTCR_EL2_FLAGS { () => { SYS_FIELD_PREP_ENUM(VTCR_EL2, SH0, INNER) | SYS_FIELD_PREP_ENUM(VTCR_EL2, ORGN0, WBWA) | SYS_FIELD_PREP_ENUM(VTCR_EL2, IRGN0, WBWA) | SYS_FIELD_PREP_ENUM(VTCR_EL2, TG0, VTCR_EL2_TGRAN) | VTCR_EL2_RES1 }; }
macro_rules! VTCR_EL2_IPA { ($vtcr:expr) => { 64 - FIELD_GET(VTCR_EL2_T0SZ, $vtcr) }; }
macro_rules! ARM64_VTTBR_X { ($ipa:expr, $levels:expr) => { ($ipa) - (($levels) * (PAGE_SHIFT - 3)) }; }
macro_rules! VTTBR_CNP_BIT { () => { 1u64 }; }
macro_rules! VTTBR_VMID_SHIFT { () => { 48u64 }; }
macro_rules! VTTBR_VMID_MASK { ($size:expr) => { (((1u64 << ($size)) - 1) << VTTBR_VMID_SHIFT!()) }; }
macro_rules! HSTR_EL2_T { ($x:expr) => { 1u64 << ($x) }; }
pub const CPTR_EL2_TFP_SHIFT: u32 = 10;
pub const CPTR_EL2_TCPAC: u64 = 1u64 << 31;
pub const CPTR_EL2_TAM: u64 = 1 << 30;
pub const CPTR_EL2_TTA: u64 = 1 << 20;
pub const CPTR_EL2_TSM: u64 = 1 << 12;
pub const CPTR_EL2_TFP: u64 = 1 << CPTR_EL2_TFP_SHIFT;
pub const CPTR_EL2_TZ: u64 = 1 << 8;
macro_rules! CPTR_NVHE_EL2_RES1 { () => { BIT(13) | BIT(9) | GENMASK(7, 0) }; }
macro_rules! CPTR_NVHE_EL2_RES0 { () => { GENMASK(63, 32) | GENMASK(29, 21) | GENMASK(19, 14) | BIT(11) }; }
macro_rules! CPTR_VHE_EL2_RES0 { () => { GENMASK(63, 32) | GENMASK(27, 26) | GENMASK(23, 22) | GENMASK(19, 18) | GENMASK(15, 0) }; }
macro_rules! HPFAR_MASK { () => { !0u64 & !0xf }; }
macro_rules! PAR_TO_HPFAR { ($par:expr) => { (($par) & GENMASK_ULL(52 - 1, 12)) >> 8 }; }
macro_rules! FAR_TO_FIPA_OFFSET { ($far:expr) => { ($far) & GENMASK_ULL(11, 0) }; }

macro_rules! ECN { ($x:ident) => { (ESR_ELx_EC_$x, stringify!($x)) }; }

// The C initializer-list macros are retained as Rust expression macros.
macro_rules! kvm_arm_exception_class { () => { [ECN!(UNKNOWN), ECN!(WFx), ECN!(CP15_32), ECN!(CP15_64), ECN!(CP14_MR), ECN!(CP14_LS), ECN!(FP_ASIMD), ECN!(CP10_ID), ECN!(PAC), ECN!(CP14_64), ECN!(SVC64), ECN!(HVC64), ECN!(SMC64), ECN!(SYS64), ECN!(SVE), ECN!(IMP_DEF), ECN!(IABT_LOW), ECN!(IABT_CUR), ECN!(PC_ALIGN), ECN!(DABT_LOW), ECN!(DABT_CUR), ECN!(SP_ALIGN), ECN!(FP_EXC32), ECN!(FP_EXC64), ECN!(SERROR), ECN!(BREAKPT_LOW), ECN!(BREAKPT_CUR), ECN!(SOFTSTP_LOW), ECN!(SOFTSTP_CUR), ECN!(WATCHPT_LOW), ECN!(WATCHPT_CUR), ECN!(BKPT32), ECN!(VECTOR32), ECN!(BRK64), ECN!(ERET)] }; }
macro_rules! kvm_mode_names { () => { [(PSR_MODE_EL0t, "EL0t"), (PSR_MODE_EL1t, "EL1t"), (PSR_MODE_EL1h, "EL1h"), (PSR_MODE_EL2t, "EL2t"), (PSR_MODE_EL2h, "EL2h"), (PSR_MODE_EL3t, "EL3t"), (PSR_MODE_EL3h, "EL3h"), (PSR_AA32_MODE_USR, "32-bit USR"), (PSR_AA32_MODE_FIQ, "32-bit FIQ"), (PSR_AA32_MODE_IRQ, "32-bit IRQ"), (PSR_AA32_MODE_SVC, "32-bit SVC"), (PSR_AA32_MODE_ABT, "32-bit ABT"), (PSR_AA32_MODE_HYP, "32-bit HYP"), (PSR_AA32_MODE_UND, "32-bit UND"), (PSR_AA32_MODE_SYS, "32-bit SYS")] }; }

pub const VCPU_RESET_PSTATE_EL1: u64 = PSR_MODE_EL1h | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_D_BIT;
pub const VCPU_RESET_PSTATE_EL2: u64 = PSR_MODE_EL2h | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_D_BIT;
pub const VCPU_RESET_PSTATE_SVC: u64 = PSR_AA32_MODE_SVC | PSR_AA32_A_BIT | PSR_AA32_I_BIT | PSR_AA32_F_BIT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
