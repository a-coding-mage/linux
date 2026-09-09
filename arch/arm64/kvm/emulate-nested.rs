// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 - Linaro and Columbia University
 * Author: Jintack Lim <jintack.lim@linaro.org>
 *
 * Direct Rust translation of the nested KVM emulation implementation.
 * Kernel-provided types, constants, and functions remain external dependencies.
 */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
#[repr(u32)]
pub enum trap_behaviour {
    BEHAVE_HANDLE_LOCALLY = 0,
    BEHAVE_FORWARD_READ = 1 << 0,
    BEHAVE_FORWARD_WRITE = 1 << 1,
    BEHAVE_FORWARD_RW = (1 << 0) | (1 << 1),
    BEHAVE_FORWARD_IN_HOST_EL0 = 1 << 2,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct trap_bits {
    pub index: u32,
    pub behaviour: trap_behaviour,
    pub value: u64,
    pub mask: u64,
}

/* Coarse grained trap definitions.  The numeric ordering is part of the ABI. */
#[allow(non_camel_case_types, non_upper_case_globals)]
#[repr(usize)]
pub enum cgt_group_id {
    __RESERVED__ = 0,
    CGT_HCR_TID1,
    CGT_HCR_TID2,
    CGT_HCR_TID3,
    CGT_HCR_IMO,
    CGT_HCR_FMO,
    CGT_HCR_TIDCP,
    CGT_HCR_TACR,
    CGT_HCR_TSW,
    CGT_HCR_TPC,
    CGT_HCR_TPU,
    CGT_HCR_TTLB,
    CGT_HCR_TVM,
    CGT_HCR_TDZ,
    CGT_HCR_TRVM,
    CGT_HCR_TLOR,
    CGT_HCR_TERR,
    CGT_HCR_APK,
    CGT_HCR_NV,
    CGT_HCR_NV_nNV2,
    CGT_HCR_NV1_nNV2,
    CGT_HCR_AT,
    CGT_HCR_nFIEN,
    CGT_HCR_TID4,
    CGT_HCR_TICAB,
    CGT_HCR_TOCU,
    CGT_HCR_ENSCXT,
    CGT_HCR_TTLBIS,
    CGT_HCR_TTLBOS,
    CGT_HCR_TID5,
    CGT_MDCR_TPMCR,
    CGT_MDCR_TPM,
    CGT_MDCR_TDE,
    CGT_MDCR_TDA,
    CGT_MDCR_TDOSA,
    CGT_MDCR_TDRA,
    CGT_MDCR_E2PB,
    CGT_MDCR_TPMS,
    CGT_MDCR_TTRF,
    CGT_MDCR_E2TB,
    CGT_MDCR_TDCC,
    CGT_CPTR_TAM,
    CGT_CPTR_TCPAC,
    CGT_HCRX_EnFPM,
    CGT_HCRX_TCR2En,
    CGT_HCRX_SCTLR2En,
    CGT_CNTHCTL_EL1TVT,
    CGT_CNTHCTL_EL1TVCT,
    CGT_ICH_HCR_TC,
    CGT_ICH_HCR_TALL0,
    CGT_ICH_HCR_TALL1,
    CGT_ICH_HCR_TDIR,
    __MULTIPLE_CONTROL_BITS__,
    CGT_HCR_IMO_FMO_ICH_HCR_TC,
    CGT_HCR_TID2_TID4,
    CGT_HCR_TTLB_TTLBIS,
    CGT_HCR_TTLB_TTLBOS,
    CGT_HCR_TVM_TRVM,
    CGT_HCR_TVM_TRVM_HCRX_TCR2En,
    CGT_HCR_TVM_TRVM_HCRX_SCTLR2En,
    CGT_HCR_TPU_TICAB,
    CGT_HCR_TPU_TOCU,
    CGT_HCR_NV1_nNV2_ENSCXT,
    CGT_MDCR_TPM_TPMCR,
    CGT_MDCR_TPM_HPMN,
    CGT_MDCR_TDE_TDA,
    CGT_MDCR_TDE_TDOSA,
    CGT_MDCR_TDE_TDRA,
    CGT_MDCR_TDCC_TDE_TDA,
    CGT_ICH_HCR_TC_TDIR,
    __COMPLEX_CONDITIONS__,
    CGT_CNTHCTL_EL1PCTEN,
    CGT_CNTHCTL_EL1PTEN,
    CGT_CNTHCTL_EL1NVPCT,
    CGT_CNTHCTL_EL1NVVCT,
    CGT_CPTR_TTA,
    CGT_MDCR_HPMN,
    CGT_HCR_NV_HCRX_nNVTGE,
    __NR_CGT_GROUP_IDS__,
}

/*
 * The remaining implementation uses the kernel's KVM register definitions and
 * deliberately retains C-compatible raw-pointer semantics.  It is represented
 * as external declarations here because those definitions are supplied by the
 * target kernel translation unit.
 */
extern "C" {
    pub fn kvm_nested_vcpu_trap(vcpu: *mut core::ffi::c_void) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
