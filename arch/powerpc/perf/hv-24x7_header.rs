/* SPDX-License-Identifier: GPL-2.0 */

// The HV_PERF_DOMAIN_* values are populated by the external hv-24x7-domains
// definition in the C header.  Preserve the generated-domain dependency here.
#[repr(i32)]
pub enum hv_perf_domains {
    // DOMAIN(n, v, x, c) => HV_PERF_DOMAIN_##n = v
    HV_PERF_DOMAIN_MAX,
}

#[inline]
pub const fn h24x7_request_size(iface_version: u8) -> usize {
    if iface_version == 1 { 16 } else { 32 }
}

#[repr(C, packed)]
pub struct hv_24x7_request {
    // PHYSICAL domains require enabling via phyp/hmc.
    pub performance_domain: u8,
    pub reserved: [u8; 0x1],
    // Bytes to read starting at data_offset; must be a multiple of 8.
    pub data_size: u16,
    // Byte offset within the perf domain to read from; must be 8-byte aligned.
    pub data_offset: u32,
    // Only valid for VIRTUAL_PROCESSOR domains; -1 means current partition only.
    pub starting_lpar_ix: u16,
    // -1 means infinite or all.
    pub max_num_lpars: u16,
    // Chip, core, or virtual processor based on performance_domain.
    pub starting_ix: u16,
    pub max_ix: u16,
    // The following fields were added in v2 of the 24x7 interface.
    pub starting_thread_group_ix: u8,
    // -1 means all thread groups starting at starting_thread_group_ix.
    pub max_num_thread_groups: u8,
    pub reserved2: [u8; 0xE],
}

#[repr(C, packed)]
pub struct hv_24x7_request_buffer {
    pub interface_version: u8,
    pub num_requests: u8,
    pub reserved: [u8; 0xE],
    pub requests: [hv_24x7_request; 0],
}

#[repr(C, packed)]
pub struct hv_24x7_result_element_v1 {
    pub lpar_ix: u16,
    // Represents the core, chip, or virtual processor based on the request's
    // performance_domain.
    pub domain_ix: u16,
    // -1 if performance_domain does not refer to a virtual processor.
    pub lpar_cfg_instance_id: u32,
    // Size is result_element_data_size of the containing result.
    pub element_data: [u64; 0],
}

// Separate v2 structure because the offset of element_data changed between versions.
#[repr(C, packed)]
pub struct hv_24x7_result_element_v2 {
    pub lpar_ix: u16,
    // Represents the core, chip, or virtual processor based on the request's
    // performance_domain.
    pub domain_ix: u16,
    // -1 if performance_domain does not refer to a virtual processor.
    pub lpar_cfg_instance_id: u32,
    pub thread_group_ix: u8,
    pub reserved: [u8; 7],
    // Size is result_element_data_size of the containing result.
    pub element_data: [u64; 0],
}

#[repr(C, packed)]
pub struct hv_24x7_result {
    // Index of the 24x7 Request Structure in the 24x7 Request Buffer.
    pub result_ix: u8,
    // 0 means additional requests are required; 1 means all elements returned.
    pub results_complete: u8,
    pub num_elements_returned: u16,
    // Copy of data_size from the corresponding hv_24x7_request.
    pub result_element_data_size: u16,
    pub reserved: [u8; 0x2],
    // v1 or v2 result elements, selected by the containing buffer's interface_version.
    pub elements: [u8; 0],
}

#[repr(C, packed)]
pub struct hv_24x7_data_result_buffer {
    pub interface_version: u8,
    pub num_results: u8,
    pub reserved: [u8; 0x1],
    pub failing_request_ix: u8,
    pub detailed_rc: u32,
    pub cec_cfg_instance_id: u64,
    pub catalog_version_num: u64,
    pub reserved2: [u8; 0x8],
    // [num_results] variable-sized results.
    pub results: [hv_24x7_result; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
