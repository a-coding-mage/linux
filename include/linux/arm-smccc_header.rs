/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from linux/arm-smccc.h. C preprocessor/build conditions are retained in comments. */

pub const ARM_SMCCC_STD_CALL: u32 = 0;
pub const ARM_SMCCC_FAST_CALL: u32 = 1;
pub const ARM_SMCCC_TYPE_SHIFT: u32 = 31;
pub const ARM_SMCCC_SMC_32: u32 = 0;
pub const ARM_SMCCC_SMC_64: u32 = 1;
pub const ARM_SMCCC_CALL_CONV_SHIFT: u32 = 30;
pub const ARM_SMCCC_OWNER_MASK: u32 = 0x3f;
pub const ARM_SMCCC_OWNER_SHIFT: u32 = 24;
pub const ARM_SMCCC_FUNC_MASK: u32 = 0xffff;

#[inline] pub const fn ARM_SMCCC_IS_FAST_CALL(v: u32) -> u32 { v & (ARM_SMCCC_FAST_CALL << ARM_SMCCC_TYPE_SHIFT) }
#[inline] pub const fn ARM_SMCCC_IS_64(v: u32) -> u32 { v & (ARM_SMCCC_SMC_64 << ARM_SMCCC_CALL_CONV_SHIFT) }
#[inline] pub const fn ARM_SMCCC_FUNC_NUM(v: u32) -> u32 { v & ARM_SMCCC_FUNC_MASK }
#[inline] pub const fn ARM_SMCCC_OWNER_NUM(v: u32) -> u32 { (v >> ARM_SMCCC_OWNER_SHIFT) & ARM_SMCCC_OWNER_MASK }
#[inline] pub const fn ARM_SMCCC_CALL_VAL(t: u32, c: u32, o: u32, f: u32) -> u32 { (t << ARM_SMCCC_TYPE_SHIFT) | (c << ARM_SMCCC_CALL_CONV_SHIFT) | ((o & ARM_SMCCC_OWNER_MASK) << ARM_SMCCC_OWNER_SHIFT) | (f & ARM_SMCCC_FUNC_MASK) }

pub const ARM_SMCCC_OWNER_ARCH: u32 = 0; pub const ARM_SMCCC_OWNER_CPU: u32 = 1;
pub const ARM_SMCCC_OWNER_SIP: u32 = 2; pub const ARM_SMCCC_OWNER_OEM: u32 = 3;
pub const ARM_SMCCC_OWNER_STANDARD: u32 = 4; pub const ARM_SMCCC_OWNER_STANDARD_HYP: u32 = 5;
pub const ARM_SMCCC_OWNER_VENDOR_HYP: u32 = 6; pub const ARM_SMCCC_OWNER_TRUSTED_APP: u32 = 48;
pub const ARM_SMCCC_OWNER_TRUSTED_APP_END: u32 = 49; pub const ARM_SMCCC_OWNER_TRUSTED_OS: u32 = 50;
pub const ARM_SMCCC_OWNER_TRUSTED_OS_END: u32 = 63;
pub const ARM_SMCCC_FUNC_QUERY_CALL_UID: u32 = 0xff01;
pub const ARM_SMCCC_QUIRK_NONE: u32 = 0; pub const ARM_SMCCC_QUIRK_QCOM_A6: u32 = 1;
pub const ARM_SMCCC_VERSION_1_0: u32 = 0x10000; pub const ARM_SMCCC_VERSION_1_1: u32 = 0x10001;
pub const ARM_SMCCC_VERSION_1_2: u32 = 0x10002; pub const ARM_SMCCC_VERSION_1_3: u32 = 0x10003;
pub const ARM_SMCCC_1_3_SVE_HINT: u32 = 0x10000; pub const ARM_SMCCC_CALL_HINTS: u32 = ARM_SMCCC_1_3_SVE_HINT;

pub const ARM_SMCCC_VERSION_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,0,0);
pub const ARM_SMCCC_ARCH_FEATURES_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,0,1);
pub const ARM_SMCCC_ARCH_SOC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,0,2);
pub const ARM_SMCCC_ARCH_SOC_ID64: u32 = ARM_SMCCC_CALL_VAL(1,1,0,2);
pub const ARM_SMCCC_ARCH_WORKAROUND_1: u32 = ARM_SMCCC_CALL_VAL(1,0,0,0x8000);
pub const ARM_SMCCC_ARCH_WORKAROUND_2: u32 = ARM_SMCCC_CALL_VAL(1,0,0,0x7fff);
pub const ARM_SMCCC_ARCH_WORKAROUND_3: u32 = ARM_SMCCC_CALL_VAL(1,0,0,0x3fff);
/* C1-Pro erratum 4193714: SME DVMSync early acknowledgement */
pub const ARM_SMCCC_CPU_WORKAROUND_4193714: u32 = ARM_SMCCC_CALL_VAL(1,0,1,0x10);
pub const ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,6,ARM_SMCCC_FUNC_QUERY_CALL_UID);
/* KVM UID value: 28b46fb6-2ec5-11e9-a9ca-4b564d003a74. UUID_INIT is an external C dependency. */

pub const ARM_SMCCC_KVM_FUNC_FEATURES: u32 = 0; pub const ARM_SMCCC_KVM_FUNC_PTP: u32 = 1;
pub const ARM_SMCCC_KVM_FUNC_HYP_MEMINFO: u32 = 2; pub const ARM_SMCCC_KVM_FUNC_MEM_SHARE: u32 = 3;
pub const ARM_SMCCC_KVM_FUNC_MEM_UNSHARE: u32 = 4;
pub const ARM_SMCCC_KVM_FUNC_MMIO_GUARD: u32 = 7;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_5: u32 = 5; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_6: u32 = 6;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_8: u32 = 8; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_9: u32 = 9;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_10: u32 = 10; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_11: u32 = 11;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_12: u32 = 12; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_13: u32 = 13;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_14: u32 = 14; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_15: u32 = 15;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_16: u32 = 16; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_17: u32 = 17;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_18: u32 = 18; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_19: u32 = 19;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_20: u32 = 20; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_21: u32 = 21;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_22: u32 = 22; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_23: u32 = 23;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_24: u32 = 24; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_25: u32 = 25;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_26: u32 = 26; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_27: u32 = 27;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_28: u32 = 28; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_29: u32 = 29;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_30: u32 = 30; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_31: u32 = 31;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_32: u32 = 32; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_33: u32 = 33;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_34: u32 = 34; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_35: u32 = 35;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_36: u32 = 36; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_37: u32 = 37;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_38: u32 = 38; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_39: u32 = 39;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_40: u32 = 40; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_41: u32 = 41;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_42: u32 = 42; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_43: u32 = 43;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_44: u32 = 44; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_45: u32 = 45;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_46: u32 = 46; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_47: u32 = 47;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_48: u32 = 48; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_49: u32 = 49;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_50: u32 = 50; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_51: u32 = 51;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_52: u32 = 52; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_53: u32 = 53;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_54: u32 = 54; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_55: u32 = 55;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_56: u32 = 56; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_57: u32 = 57;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_58: u32 = 58; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_59: u32 = 59;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_60: u32 = 60; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_61: u32 = 61;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_62: u32 = 62; pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_63: u32 = 63;
pub const ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_VER: u32 = 64; pub const ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_CPUS: u32 = 65;
pub const ARM_SMCCC_KVM_FUNC_FEATURES_2: u32 = 127; pub const ARM_SMCCC_KVM_NUM_FUNCS: u32 = 128;
pub const ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,6,0);
pub const SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED: i32 = 1;
pub const ARM_SMCCC_VENDOR_HYP_KVM_PTP_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,0,6,1);
pub const ARM_SMCCC_VENDOR_HYP_KVM_HYP_MEMINFO_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,2);
pub const ARM_SMCCC_VENDOR_HYP_KVM_MEM_SHARE_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,3);
pub const ARM_SMCCC_VENDOR_HYP_KVM_MEM_UNSHARE_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,4);
pub const ARM_SMCCC_VENDOR_HYP_KVM_MMIO_GUARD_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,7);
pub const ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_VER_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,64);
pub const ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_CPUS_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(1,1,6,65);
pub const KVM_PTP_VIRT_COUNTER: u32 = 0; pub const KVM_PTP_PHYS_COUNTER: u32 = 1;
pub const ARM_SMCCC_HV_PV_TIME_FEATURES: u32 = ARM_SMCCC_CALL_VAL(1,1,5,0x20);
pub const ARM_SMCCC_HV_PV_TIME_ST: u32 = ARM_SMCCC_CALL_VAL(1,1,5,0x21);
pub const ARM_SMCCC_TRNG_VERSION: u32 = ARM_SMCCC_CALL_VAL(1,0,4,0x50);
pub const ARM_SMCCC_TRNG_FEATURES: u32 = ARM_SMCCC_CALL_VAL(1,0,4,0x51);
pub const ARM_SMCCC_TRNG_GET_UUID: u32 = ARM_SMCCC_CALL_VAL(1,0,4,0x52);
pub const ARM_SMCCC_TRNG_RND32: u32 = ARM_SMCCC_CALL_VAL(1,0,4,0x53);
pub const ARM_SMCCC_TRNG_RND64: u32 = ARM_SMCCC_CALL_VAL(1,1,4,0x53);
pub const SMCCC_RET_SUCCESS: i32 = 0; pub const SMCCC_RET_NOT_SUPPORTED: i32 = -1;
pub const SMCCC_RET_NOT_REQUIRED: i32 = -2; pub const SMCCC_RET_INVALID_PARAMETER: i32 = -3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum arm_smccc_conduit { SMCCC_CONDUIT_NONE, SMCCC_CONDUIT_SMC, SMCCC_CONDUIT_HVC }
extern "C" { pub fn arm_smccc_1_1_get_conduit() -> arm_smccc_conduit; pub fn arm_smccc_get_version() -> u32; pub fn arm_smccc_version_init(version: u32, conduit: arm_smccc_conduit); pub fn arm_smccc_get_soc_id_version() -> i32; pub fn arm_smccc_get_soc_id_revision() -> i32; }

#[repr(C)] pub struct arm_smccc_res { pub a0: usize, pub a1: usize, pub a2: usize, pub a3: usize }
#[cfg(target_arch = "aarch64")]
#[repr(C)] pub struct arm_smccc_1_2_regs { pub a0: usize, pub a1: usize, pub a2: usize, pub a3: usize, pub a4: usize, pub a5: usize, pub a6: usize, pub a7: usize, pub a8: usize, pub a9: usize, pub a10: usize, pub a11: usize, pub a12: usize, pub a13: usize, pub a14: usize, pub a15: usize, pub a16: usize, pub a17: usize }
#[repr(C)] pub union arm_smccc_quirk_state { pub a6: usize }
#[repr(C)] pub struct arm_smccc_quirk { pub id: i32, pub state: arm_smccc_quirk_state }

extern "C" { pub fn arm_smccc_hypervisor_has_uuid(uuid: *const core::ffi::c_void) -> bool; pub fn __arm_smccc_smc(a0: usize,a1: usize,a2: usize,a3: usize,a4: usize,a5: usize,a6: usize,a7: usize,res: *mut arm_smccc_res,quirk: *mut arm_smccc_quirk); pub fn __arm_smccc_hvc(a0: usize,a1: usize,a2: usize,a3: usize,a4: usize,a5: usize,a6: usize,a7: usize,res: *mut arm_smccc_res,quirk: *mut arm_smccc_quirk); }
#[cfg(target_arch = "aarch64")]
extern "C" { pub fn arm_smccc_1_2_hvc(args: *const arm_smccc_1_2_regs, res: *mut arm_smccc_1_2_regs); pub fn arm_smccc_1_2_smc(args: *const arm_smccc_1_2_regs, res: *mut arm_smccc_1_2_regs); }

#[inline] pub unsafe fn smccc_res_to_uuid(r0:u32,r1:u32,r2:u32,r3:u32) -> [u8;16] { [r0 as u8,(r0>>8) as u8,(r0>>16) as u8,(r0>>24) as u8,r1 as u8,(r1>>8) as u8,(r1>>16) as u8,(r1>>24) as u8,r2 as u8,(r2>>8) as u8,(r2>>16) as u8,(r2>>24) as u8,r3 as u8,(r3>>8) as u8,(r3>>16) as u8,(r3>>24) as u8] }
#[inline] pub unsafe fn smccc_uuid_to_reg(uuid: *const [u8;16], reg: i32) -> u32 { let b=(*uuid); (b[(4*reg) as usize] as u32)|((b[(4*reg+1) as usize] as u32)<<8)|((b[(4*reg+2) as usize] as u32)<<16)|((b[(4*reg+3) as usize] as u32)<<24) }

/* Variadic C invocation macros and architecture-specific inline assembly are not representable as Rust macros; their external call interfaces remain above. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
