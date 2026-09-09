/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (c) 2003-2006 Advanced Micro Devices, Inc.
 */

// Types supplied by the surrounding kernel translation.
#[repr(C)]
pub struct cpufreq_frequency_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_processor_performance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct powernow_k8_data {
    pub cpu: u32,
    pub numps: u32,
    pub batps: u32,
    pub rvo: u32,
    pub irt: u32,
    pub vidmvs: u32,
    pub vstable: u32,
    pub plllock: u32,
    pub exttype: u32,
    pub currvid: u32,
    pub currfid: u32,
    pub powernow_table: *mut cpufreq_frequency_table,
    pub acpi_data: acpi_processor_performance,
    pub available_cores: *mut cpumask,
}

pub const CPUID_PROCESSOR_SIGNATURE: u32 = 1;
pub const CPUID_XFAM: u32 = 0x0ff00000;
pub const CPUID_XFAM_K8: u32 = 0;
pub const CPUID_XMOD: u32 = 0x000f0000;
pub const CPUID_XMOD_REV_MASK: u32 = 0x000c0000;
pub const CPUID_XFAM_10H: u32 = 0x00100000;
pub const CPUID_USE_XFAM_XMOD: u32 = 0x00000f00;
pub const CPUID_GET_MAX_CAPABILITIES: u32 = 0x80000000;
pub const CPUID_FREQ_VOLT_CAPABILITIES: u32 = 0x80000007;
pub const P_STATE_TRANSITION_CAPABLE: u32 = 6;

pub const MSR_FIDVID_CTL: u32 = 0xc0010041;
pub const MSR_FIDVID_STATUS: u32 = 0xc0010042;
pub const MSR_C_LO_INIT_FID_VID: u32 = 0x00010000;
pub const MSR_C_LO_NEW_VID: u32 = 0x00003f00;
pub const MSR_C_LO_NEW_FID: u32 = 0x0000003f;
pub const MSR_C_LO_VID_SHIFT: u32 = 8;
pub const MSR_C_HI_STP_GNT_TO: u32 = 0x000fffff;
pub const MSR_S_LO_CHANGE_PENDING: u32 = 0x80000000;
pub const MSR_S_LO_MAX_RAMP_VID: u32 = 0x3f000000;
pub const MSR_S_LO_MAX_FID: u32 = 0x003f0000;
pub const MSR_S_LO_START_FID: u32 = 0x00003f00;
pub const MSR_S_LO_CURRENT_FID: u32 = 0x0000003f;
pub const MSR_S_HI_MIN_WORKING_VID: u32 = 0x3f000000;
pub const MSR_S_HI_MAX_WORKING_VID: u32 = 0x003f0000;
pub const MSR_S_HI_START_VID: u32 = 0x00003f00;
pub const MSR_S_HI_CURRENT_VID: u32 = 0x0000003f;
pub const MSR_C_HI_STP_GNT_BENIGN: u32 = 0x00000001;

pub const LO_FID_TABLE_TOP: u32 = 7;
pub const HI_FID_TABLE_BOTTOM: u32 = 8;
pub const LO_VCOFREQ_TABLE_TOP: u32 = 1400;
pub const HI_VCOFREQ_TABLE_BOTTOM: u32 = 1600;
pub const MIN_FREQ_RESOLUTION: u32 = 200;
pub const MAX_FID: u32 = 0x2a;
pub const LEAST_VID: u32 = 0x3e;
pub const MIN_FREQ: u32 = 800;
pub const MAX_FREQ: u32 = 5000;
pub const INVALID_FID_MASK: u32 = 0xffffffc0;
pub const INVALID_VID_MASK: u32 = 0xffffffc0;
pub const VID_OFF: u32 = 0x3f;
pub const STOP_GRANT_5NS: u32 = 1;
pub const PLL_LOCK_CONVERSION: u32 = 1000 / 5;
pub const MAXIMUM_VID_STEPS: u32 = 1;
pub const VST_UNITS_20US: u32 = 20;

pub const IRT_SHIFT: u32 = 30;
pub const RVO_SHIFT: u32 = 28;
pub const EXT_TYPE_SHIFT: u32 = 27;
pub const PLL_L_SHIFT: u32 = 20;
pub const MVS_SHIFT: u32 = 18;
pub const VST_SHIFT: u32 = 11;
pub const VID_SHIFT: u32 = 6;
pub const IRT_MASK: u32 = 3;
pub const RVO_MASK: u32 = 3;
pub const EXT_TYPE_MASK: u32 = 1;
pub const PLL_L_MASK: u32 = 0x7f;
pub const MVS_MASK: u32 = 3;
pub const VST_MASK: u32 = 0x7f;
pub const VID_MASK: u32 = 0x1f;
pub const FID_MASK: u32 = 0x1f;
pub const EXT_VID_MASK: u32 = 0x3f;
pub const EXT_FID_MASK: u32 = 0x3f;

pub const PSB_ID_STRING: &[u8; 10] = b"AMDK7PNOW!";
pub const PSB_ID_STRING_LEN: usize = 10;
pub const PSB_VERSION_1_4: u8 = 0x14;

#[repr(C)]
pub struct psb_s {
    pub signature: [u8; 10],
    pub tableversion: u8,
    pub flags1: u8,
    pub vstable: u16,
    pub flags2: u8,
    pub num_tables: u8,
    pub cpuid: u32,
    pub plllocktime: u8,
    pub maxfid: u8,
    pub maxvid: u8,
    pub numps: u8,
}

#[repr(C)]
pub struct pst_s {
    pub fid: u8,
    pub vid: u8,
}

extern "C" {
    fn core_voltage_pre_transition(data: *mut powernow_k8_data, reqvid: u32, regfid: u32) -> i32;
    fn core_voltage_post_transition(data: *mut powernow_k8_data, reqvid: u32) -> i32;
    fn core_frequency_transition(data: *mut powernow_k8_data, reqfid: u32) -> i32;
    fn powernow_k8_acpi_pst_values(data: *mut powernow_k8_data, index: u32);
    fn fill_powernow_table_fidvid(
        data: *mut powernow_k8_data,
        powernow_table: *mut cpufreq_frequency_table,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
