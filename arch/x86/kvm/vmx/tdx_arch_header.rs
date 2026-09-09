/* SPDX-License-Identifier: GPL-2.0 */
/* architectural constants/data definitions for TDX SEAMCALLs */

pub const TDX_NON_ARCH: u64 = 1u64 << 63;
pub const TDX_CLASS_SHIFT: u32 = 56;
pub const TDX_FIELD_MASK: u64 = 0xffff_ffff;

#[inline]
pub const fn __build_tdx_field(non_arch: bool, class: u64, field: u64) -> u64 {
    (if non_arch { TDX_NON_ARCH } else { 0 })
        | (class << TDX_CLASS_SHIFT)
        | (field & TDX_FIELD_MASK)
}

#[inline]
pub const fn build_tdx_field(class: u64, field: u64) -> u64 {
    __build_tdx_field(false, class, field)
}

#[inline]
pub const fn build_tdx_field_non_arch(class: u64, field: u64) -> u64 {
    __build_tdx_field(true, class, field)
}

pub const TD_CLASS_EXECUTION_CONTROLS: u64 = 17;
pub const TDVPS_CLASS_VMCS: u64 = 0;
pub const TDVPS_CLASS_GUEST_GPR: u64 = 16;
pub const TDVPS_CLASS_OTHER_GUEST: u64 = 17;
pub const TDVPS_CLASS_MANAGEMENT: u64 = 32;

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TdxTdcsExecutionControl {
    TdTdcsExecTscOffset = 10,
    TdTdcsExecTscMultiplier = 11,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TdxVcpuGuestOtherState {
    TdVcpuStateDetailsNonArch = 0x100,
}

pub const TDX_VCPU_STATE_DETAILS_INTR_PENDING: u64 = 1u64 << 0;

#[inline]
pub const fn tdx_vcpu_state_details_intr_pending(vcpu_state_details: u64) -> bool {
    (vcpu_state_details & TDX_VCPU_STATE_DETAILS_INTR_PENDING) != 0
}

#[inline]
pub const fn tdcs_exec(field: u64) -> u64 {
    build_tdx_field(TD_CLASS_EXECUTION_CONTROLS, field)
}

#[inline]
pub const fn tdvps_vmcs(field: u64) -> u64 {
    build_tdx_field(TDVPS_CLASS_VMCS, field)
}

#[inline]
pub const fn tdvps_state(field: u64) -> u64 {
    build_tdx_field(TDVPS_CLASS_OTHER_GUEST, field)
}

#[inline]
pub const fn tdvps_state_non_arch(field: u64) -> u64 {
    build_tdx_field_non_arch(TDVPS_CLASS_OTHER_GUEST, field)
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TdxVcpuGuestManagement {
    TdVcpuPendNmi = 11,
}

#[inline]
pub const fn tdvps_management(field: u64) -> u64 {
    build_tdx_field(TDVPS_CLASS_MANAGEMENT, field)
}

pub const TDX_EXTENDMR_CHUNKSIZE: usize = 256;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TdxCpuidValue {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub const TDX_EXT_EXIT_QUAL_TYPE_MASK: u64 = 0xf;
pub const TDX_EXT_EXIT_QUAL_TYPE_PENDING_EPT_VIOLATION: u64 = 6;

#[repr(C, align(1024))]
pub struct TdParams {
    pub attributes: u64,
    pub xfam: u64,
    pub max_vcpus: u16,
    pub reserved0: [u8; 6],
    pub eptp_controls: u64,
    pub config_flags: u64,
    pub tsc_frequency: u16,
    pub reserved1: [u8; 38],
    pub mrconfigid: [u64; 6],
    pub mrowner: [u64; 6],
    pub mrownerconfig: [u64; 6],
    pub reserved2: [u64; 4],
    pub cpuid_values: [TdxCpuidValue; 0],
    pub reserved3: [u8; 768],
}

pub const TDX_CONFIG_FLAGS_MAX_GPAW: u64 = 1u64 << 0;
pub const TDX_CONFIG_FLAGS_NO_RBP_MOD: u64 = 1u64 << 2;

#[inline]
pub const fn tdx_tsc_khz_to_25mhz(tsc_in_khz: u64) -> u64 {
    tsc_in_khz / (25 * 1000)
}

#[inline]
pub const fn tdx_tsc_25mhz_to_khz(tsc_in_25mhz: u64) -> u64 {
    tsc_in_25mhz * (25 * 1000)
}

pub const TDX_MIN_TSC_FREQUENCY_KHZ: u64 = 100 * 1000;
pub const TDX_MAX_TSC_FREQUENCY_KHZ: u64 = 10 * 1000 * 1000;

pub const TDX_SEPT_LEVEL_MASK: u64 = 0x7;
pub const TDX_SEPT_STATE_MASK: u64 = 0xff00;
pub const TDX_SEPT_STATE_SHIFT: u32 = 8;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TdxSeptEntryState {
    TdxSeptFree = 0,
    TdxSeptBlocked = 1,
    TdxSeptPending = 2,
    TdxSeptPendingBlocked = 3,
    TdxSeptPresent = 4,
}

#[inline]
pub const fn tdx_get_sept_level(sept_entry_info: u64) -> u8 {
    (sept_entry_info & TDX_SEPT_LEVEL_MASK) as u8
}

#[inline]
pub const fn tdx_get_sept_state(sept_entry_info: u64) -> u8 {
    ((sept_entry_info & TDX_SEPT_STATE_MASK) >> TDX_SEPT_STATE_SHIFT) as u8
}

pub const MD_FIELD_ID_FEATURES0_TOPOLOGY_ENUM: u64 = 1u64 << 20;
pub const TD_MD_FIELD_ID_CPUID_VALUES: u64 = 0x9410_0003_0000_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
