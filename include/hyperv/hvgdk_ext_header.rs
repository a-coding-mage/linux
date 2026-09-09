/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Type definitions for the Microsoft Hypervisor.
 */

// Dependency supplied by the original header's hvgdk_mini.h include.

/* Extended hypercalls */
pub const HV_EXT_CALL_QUERY_CAPABILITIES: u32 = 0x8001;
pub const HV_EXT_CALL_MEMORY_HEAT_HINT: u32 = 0x8003;

/* Extended hypercalls */
pub const HV_EXTCALL_QUERY_CAPABILITIES: u32 = 0x8001;
pub const HV_EXTCALL_MEMORY_HEAT_HINT: u32 = 0x8003;

/* HV_EXT_OUTPUT_QUERY_CAPABILITIES */
pub const HV_EXT_CAPABILITY_MEMORY_COLD_DISCARD_HINT: u64 = 1u64 << 8;

/* HV_EXT_MEMORY_HEAT_HINT_TYPE */
pub const HV_EXTMEM_HEAT_HINT_COLD: u32 = 0;
pub const HV_EXTMEM_HEAT_HINT_HOT: u32 = 1;
pub const HV_EXTMEM_HEAT_HINT_COLD_DISCARD: u32 = 2;
pub const HV_EXTMEM_HEAT_HINT_MAX: u32 = 3;

/*
 * The whole argument should fit in a page to be able to pass to the hypervisor
 * in one hypercall.
 */
pub const HV_MEMORY_HINT_MAX_GPA_PAGE_RANGES: usize =
    ((HV_HYP_PAGE_SIZE as usize - core::mem::size_of::<hv_memory_hint>()) /
        core::mem::size_of::<hv_gpa_page_range>());

/* HvExtCallMemoryHeatHint hypercall */
pub const HV_EXT_MEMORY_HEAT_HINT_TYPE_COLD_DISCARD: u32 = 2;

#[repr(C, packed)]
pub struct hv_memory_hint {
    // C bitfields: heat_type occupies bits 0..=1; reserved occupies bits 2..=63.
    pub heat_type_reserved: u64,
    pub ranges: [hv_gpa_page_range; 0],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
