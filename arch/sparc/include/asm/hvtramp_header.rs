/* SPDX-License-Identifier: GPL-2.0 */

// The C header exposes these declarations only when not assembled.

#[repr(C)]
pub struct hvtramp_mapping {
    pub vaddr: u64,
    pub tte: u64,
}

#[repr(C)]
pub struct hvtramp_descr {
    pub cpu: u32,
    pub num_mappings: u32,
    pub fault_info_va: u64,
    pub fault_info_pa: u64,
    pub thread_reg: u64,
    // C flexible array member: mappings follow the fixed descriptor fields.
    pub maps: [hvtramp_mapping; 0],
}

unsafe extern "C" {
    pub fn hv_cpu_startup(hvdescr_pa: u64);
}

pub const HVTRAMP_DESCR_CPU: usize = 0x00;
pub const HVTRAMP_DESCR_NUM_MAPPINGS: usize = 0x04;
pub const HVTRAMP_DESCR_FAULT_INFO_VA: usize = 0x08;
pub const HVTRAMP_DESCR_FAULT_INFO_PA: usize = 0x10;
pub const HVTRAMP_DESCR_THREAD_REG: usize = 0x18;
pub const HVTRAMP_DESCR_MAPS: usize = 0x20;

pub const HVTRAMP_MAPPING_VADDR: usize = 0x00;
pub const HVTRAMP_MAPPING_TTE: usize = 0x08;
pub const HVTRAMP_MAPPING_SIZE: usize = 0x10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
