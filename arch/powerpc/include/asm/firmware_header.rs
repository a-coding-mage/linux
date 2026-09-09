/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 *
 *  Modifications for ppc64:
 *      Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
 */

// This header is used by the kernel configuration (__KERNEL__).
// The C include <asm/asm-const.h> supplied ASM_CONST for these values.

/* firmware feature bitmask values */
pub const FW_FEATURE_PFT: u64 = 0x0000000000000001;
pub const FW_FEATURE_TCE: u64 = 0x0000000000000002;
pub const FW_FEATURE_SPRG0: u64 = 0x0000000000000004;
pub const FW_FEATURE_DABR: u64 = 0x0000000000000008;
pub const FW_FEATURE_COPY: u64 = 0x0000000000000010;
pub const FW_FEATURE_ASR: u64 = 0x0000000000000020;
pub const FW_FEATURE_DEBUG: u64 = 0x0000000000000040;
pub const FW_FEATURE_TERM: u64 = 0x0000000000000080;
pub const FW_FEATURE_PERF: u64 = 0x0000000000000100;
pub const FW_FEATURE_DUMP: u64 = 0x0000000000000200;
pub const FW_FEATURE_INTERRUPT: u64 = 0x0000000000000400;
pub const FW_FEATURE_MIGRATE: u64 = 0x0000000000000800;
pub const FW_FEATURE_PERFMON: u64 = 0x0000000000001000;
pub const FW_FEATURE_CRQ: u64 = 0x0000000000002000;
pub const FW_FEATURE_VIO: u64 = 0x0000000000004000;
pub const FW_FEATURE_RDMA: u64 = 0x0000000000008000;
pub const FW_FEATURE_LLAN: u64 = 0x0000000000010000;
pub const FW_FEATURE_BULK_REMOVE: u64 = 0x0000000000020000;
pub const FW_FEATURE_XDABR: u64 = 0x0000000000040000;
pub const FW_FEATURE_PUT_TCE_IND: u64 = 0x0000000000080000;
pub const FW_FEATURE_SPLPAR: u64 = 0x0000000000100000;
pub const FW_FEATURE_LPAR: u64 = 0x0000000000400000;
pub const FW_FEATURE_PS3_LV1: u64 = 0x0000000000800000;
pub const FW_FEATURE_HPT_RESIZE: u64 = 0x0000000001000000;
pub const FW_FEATURE_CMO: u64 = 0x0000000002000000;
pub const FW_FEATURE_VPHN: u64 = 0x0000000004000000;
pub const FW_FEATURE_XCMO: u64 = 0x0000000008000000;
pub const FW_FEATURE_OPAL: u64 = 0x0000000010000000;
pub const FW_FEATURE_SET_MODE: u64 = 0x0000000040000000;
pub const FW_FEATURE_BEST_ENERGY: u64 = 0x0000000080000000;
pub const FW_FEATURE_FORM1_AFFINITY: u64 = 0x0000000100000000;
pub const FW_FEATURE_PRRN: u64 = 0x0000000200000000;
pub const FW_FEATURE_DRMEM_V2: u64 = 0x0000000400000000;
pub const FW_FEATURE_DRC_INFO: u64 = 0x0000000800000000;
pub const FW_FEATURE_BLOCK_REMOVE: u64 = 0x0000001000000000;
pub const FW_FEATURE_PAPR_SCM: u64 = 0x0000002000000000;
pub const FW_FEATURE_ULTRAVISOR: u64 = 0x0000004000000000;
pub const FW_FEATURE_STUFF_TCE: u64 = 0x0000008000000000;
pub const FW_FEATURE_RPT_INVALIDATE: u64 = 0x0000010000000000;
pub const FW_FEATURE_FORM2_AFFINITY: u64 = 0x0000020000000000;
pub const FW_FEATURE_ENERGY_SCALE_INFO: u64 = 0x0000040000000000;
pub const FW_FEATURE_WATCHDOG: u64 = 0x0000080000000000;
pub const FW_FEATURE_PLPKS: u64 = 0x0000100000000000;

// CONFIG_PPC64 selects the following pseries/POWERNV/PS3/native feature sets.
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_PSERIES_POSSIBLE: u64 =
    FW_FEATURE_PFT | FW_FEATURE_TCE | FW_FEATURE_SPRG0 | FW_FEATURE_DABR |
    FW_FEATURE_COPY | FW_FEATURE_ASR | FW_FEATURE_DEBUG | FW_FEATURE_TERM |
    FW_FEATURE_PERF | FW_FEATURE_DUMP | FW_FEATURE_INTERRUPT | FW_FEATURE_MIGRATE |
    FW_FEATURE_PERFMON | FW_FEATURE_CRQ | FW_FEATURE_VIO | FW_FEATURE_RDMA |
    FW_FEATURE_LLAN | FW_FEATURE_BULK_REMOVE | FW_FEATURE_XDABR |
    FW_FEATURE_PUT_TCE_IND | FW_FEATURE_STUFF_TCE | FW_FEATURE_SPLPAR |
    FW_FEATURE_LPAR | FW_FEATURE_CMO | FW_FEATURE_VPHN | FW_FEATURE_XCMO |
    FW_FEATURE_SET_MODE | FW_FEATURE_BEST_ENERGY | FW_FEATURE_FORM1_AFFINITY |
    FW_FEATURE_PRRN | FW_FEATURE_HPT_RESIZE | FW_FEATURE_DRMEM_V2 |
    FW_FEATURE_DRC_INFO | FW_FEATURE_BLOCK_REMOVE | FW_FEATURE_PAPR_SCM |
    FW_FEATURE_ULTRAVISOR | FW_FEATURE_RPT_INVALIDATE | FW_FEATURE_FORM2_AFFINITY |
    FW_FEATURE_ENERGY_SCALE_INFO | FW_FEATURE_WATCHDOG | FW_FEATURE_PLPKS;

#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_PSERIES_ALWAYS: u64 = 0;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_POWERNV_POSSIBLE: u64 = FW_FEATURE_OPAL | FW_FEATURE_ULTRAVISOR;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_POWERNV_ALWAYS: u64 = 0;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_PS3_POSSIBLE: u64 = FW_FEATURE_LPAR | FW_FEATURE_PS3_LV1;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_PS3_ALWAYS: u64 = FW_FEATURE_LPAR | FW_FEATURE_PS3_LV1;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_NATIVE_POSSIBLE: u64 = 0;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_NATIVE_ALWAYS: u64 = 0;

// The CONFIG_PPC_* branches below mirror the C preprocessor conditions.
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_POSSIBLE: u64 =
    FW_FEATURE_PSERIES_POSSIBLE | FW_FEATURE_POWERNV_POSSIBLE |
    FW_FEATURE_PS3_POSSIBLE | FW_FEATURE_NATIVE_ALWAYS;
#[cfg(feature = "CONFIG_PPC64")]
pub const FW_FEATURE_ALWAYS: u64 =
    FW_FEATURE_PSERIES_ALWAYS & FW_FEATURE_POWERNV_ALWAYS &
    FW_FEATURE_PS3_ALWAYS & FW_FEATURE_NATIVE_ALWAYS & FW_FEATURE_POSSIBLE;
#[cfg(not(feature = "CONFIG_PPC64"))]
pub const FW_FEATURE_POSSIBLE: u64 = 0;
#[cfg(not(feature = "CONFIG_PPC64"))]
pub const FW_FEATURE_ALWAYS: u64 = 0;

/* This is used to identify firmware features which are available
 * to the kernel.
 */
extern "C" {
    pub static mut powerpc_firmware_features: usize;
    pub fn system_reset_fwnmi();
    pub fn machine_check_fwnmi();
    pub static mut fwnmi_active: i32;
    pub static mut ibm_nmi_interlock_token: i32;
    pub static mut __start___fw_ftr_fixup: u32;
    pub static mut __stop___fw_ftr_fixup: u32;
}

#[inline]
pub unsafe fn firmware_has_feature(feature: u64) -> bool {
    (FW_FEATURE_ALWAYS & feature) != 0
        || (FW_FEATURE_POSSIBLE & powerpc_firmware_features as u64 & feature) != 0
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
extern "C" {
    pub fn pseries_probe_fw_features();
}

#[cfg(not(feature = "CONFIG_PPC_PSERIES"))]
#[inline]
pub fn pseries_probe_fw_features() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
