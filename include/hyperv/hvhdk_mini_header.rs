/* SPDX-License-Identifier: GPL-2.0 */
/* Type definitions for the Microsoft Hypervisor. */

// Dependency supplied by hvgdk_mini.h in the original header.

pub const HV_MAX_CONTIGUOUS_ALLOCATION_PAGES: u32 = 8;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_MASK: u32 = 0x00000007;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_ANY: u32 = 0x00000000;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE: u32 = 0x00000001;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_WORD: u32 = 0x00000002;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD: u32 = 0x00000003;
pub const HV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD: u32 = 0x00000004;
pub const HV_DOORBELL_FLAG_TRIGGER_ANY_VALUE: u32 = 0x80000000;
pub const HV_GENERIC_SET_SHIFT: u32 = 6;
pub const HV_GENERIC_SET_MASK: u32 = 63;

#[repr(u32)]
pub enum hv_generic_set_format { HV_GENERIC_SET_SPARSE_4K, HV_GENERIC_SET_ALL }

#[repr(u32)]
pub enum hv_scheduler_type { HV_SCHEDULER_TYPE_LP = 1, HV_SCHEDULER_TYPE_LP_SMT, HV_SCHEDULER_TYPE_CORE_SMT, HV_SCHEDULER_TYPE_ROOT, HV_SCHEDULER_TYPE_MAX }
#[repr(u32)]
pub enum hv_stats_area_type { HV_STATS_AREA_SELF = 0, HV_STATS_AREA_PARENT, HV_STATS_AREA_INTERNAL, HV_STATS_AREA_COUNT }
#[repr(u32)]
pub enum hv_stats_object_type { HV_STATS_OBJECT_HYPERVISOR = 1, HV_STATS_OBJECT_LOGICAL_PROCESSOR = 2, HV_STATS_OBJECT_PARTITION = 0x00010001, HV_STATS_OBJECT_VP = 0x00010002 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_stats_hypervisor { pub reserved: [u8; 15], pub stats_area_type: u8 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_stats_logical_processor { pub lp_index: u32, pub reserved: [u8; 11], pub stats_area_type: u8 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_stats_partition { pub partition_id: u64, pub reserved: [u8; 7], pub stats_area_type: u8 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_stats_vp { pub partition_id: u64, pub vp_index: u32, pub flags: u16, pub reserved: u8, pub stats_area_type: u8 }
#[repr(C)]
pub union hv_stats_object_identity { pub hv: hv_stats_hypervisor, pub lp: hv_stats_logical_processor, pub partition: hv_stats_partition, pub vp: hv_stats_vp }

#[repr(u32)]
pub enum hv_partition_property_code { HV_PARTITION_PROPERTY_PRIVILEGE_FLAGS = 0x00010000, HV_PARTITION_PROPERTY_SYNTHETIC_PROC_FEATURES = 0x00010001, HV_PARTITION_PROPERTY_INTEGRATED_SCHEDULER_ENABLED = 0x00020005, HV_PARTITION_PROPERTY_GPA_PAGE_ACCESS_TRACKING = 0x00050005, HV_PARTITION_PROPERTY_UNIMPLEMENTED_MSR_ACTION = 0x00050017, HV_PARTITION_PROPERTY_PROCESSOR_XSAVE_FEATURES = 0x00060002, HV_PARTITION_PROPERTY_XSAVE_STATES = 0x00060007, HV_PARTITION_PROPERTY_MAX_XSAVE_DATA_SIZE = 0x00060008, HV_PARTITION_PROPERTY_PROCESSOR_CLOCK_FREQUENCY = 0x00060009, HV_PARTITION_PROPERTY_VMM_CAPABILITIES = 0x00090007 }
pub const HV_PARTITION_VMM_CAPABILITIES_BANK_COUNT: usize = 1;
pub const HV_PARTITION_VMM_CAPABILITIES_RESERVED_BITFIELD_COUNT: u32 = 57;
#[repr(C, packed)]
pub struct hv_partition_property_vmm_capabilities { pub bank_count: u16, pub reserved: [u16; 3], pub capabilities: hv_partition_property_vmm_capabilities_union }
#[repr(C)]
pub union hv_partition_property_vmm_capabilities_union { pub as_uint64: [u64; 1], pub bitfield: u64 }
pub const HV_VMM_CAP_MAP_GPA_PRESERVE_ADJUSTABLE: u64 = 1 << 0;
pub const HV_VMM_CAP_VMM_CAN_PROVIDE_OVERLAY_GPFN: u64 = 1 << 1;
pub const HV_VMM_CAP_VP_AFFINITY_PROPERTY: u64 = 1 << 2;
pub const HV_VMM_CAP_VMM_CAN_PROVIDE_GIC_OVERLAY_LOCATIONS: u64 = 1 << 3;
pub const HV_VMM_CAP_ASSIGNABLE_SYNTHETIC_PROC_FEATURES: u64 = 1 << 4;
pub const HV_VMM_CAP_VMM_ENABLE_INTEGRATED_SCHEDULER: u64 = 1 << 6;

#[repr(u32)]
pub enum hv_snp_status { HV_SNP_STATUS_NONE = 0, HV_SNP_STATUS_AVAILABLE, HV_SNP_STATUS_INCOMPATIBLE, HV_SNP_STATUS_PSP_UNAVAILABLE, HV_SNP_STATUS_PSP_INIT_FAILED, HV_SNP_STATUS_PSP_BAD_FW_VERSION, HV_SNP_STATUS_BAD_CONFIGURATION, HV_SNP_STATUS_PSP_FW_UPDATE_IN_PROGRESS, HV_SNP_STATUS_PSP_RB_INIT_FAILED, HV_SNP_STATUS_PSP_PLATFORM_STATUS_FAILED, HV_SNP_STATUS_PSP_INIT_LATE_FAILED }
#[repr(u32)]
pub enum hv_system_property { HV_SYSTEM_PROPERTY_SLEEP_STATE = 3, HV_SYSTEM_PROPERTY_SCHEDULER_TYPE = 15, HV_DYNAMIC_PROCESSOR_FEATURE_PROPERTY = 21, HV_SYSTEM_PROPERTY_CRASHDUMPAREA = 47 }
pub const HV_PFN_RANGE_PGBITS: u32 = 24;
#[repr(C)]
pub union hv_pfn_range { pub as_uint64: u64, pub bits: hv_pfn_range_bits }
#[repr(C, packed)]
pub struct hv_pfn_range_bits { pub base_pfn: u64, pub add_pfns: u64 }

#[repr(u32)]
pub enum hv_sleep_state { HV_SLEEP_STATE_S1 = 1, HV_SLEEP_STATE_S2, HV_SLEEP_STATE_S3, HV_SLEEP_STATE_S4, HV_SLEEP_STATE_S5, HV_SLEEP_STATE_LOCK }
#[repr(u32)]
pub enum hv_dynamic_processor_feature_property { HV_X64_DYNAMIC_PROCESSOR_FEATURE_MAX_ENCRYPTED_PARTITIONS = 13, HV_X64_DYNAMIC_PROCESSOR_FEATURE_SNP_STATUS = 16 }

#[repr(C, packed)]
pub struct hv_input_get_system_property { pub property_id: u32, pub reserved: u32, pub value: hv_input_get_system_property_value }
#[repr(C)]
pub union hv_input_get_system_property_value { pub as_uint64: u64, pub hv_processor_feature: u32 }
#[repr(C, packed)]
pub struct hv_output_get_system_property { pub value: hv_output_get_system_property_value }
#[repr(C)]
pub union hv_output_get_system_property_value { pub scheduler_type: u32, pub hv_processor_feature_value: u64, pub hv_cda_info: hv_pfn_range, pub hv_tramp_pa: u64 }
#[repr(C, packed)]
pub struct hv_sleep_state_info { pub sleep_state: u32, pub pm1a_slp_typ: u8, pub pm1b_slp_typ: u8 }
#[repr(C, packed)]
pub struct hv_input_set_system_property { pub property_id: u32, pub reserved: u32, pub value: hv_input_set_system_property_value }
#[repr(C)]
pub union hv_input_set_system_property_value { pub set_sleep_state_info: hv_sleep_state_info, pub reserved0: [u64; 8] }
#[repr(C, packed)]
pub struct hv_input_enter_sleep_state { pub sleep_state: u32 }
#[repr(C, packed)]
pub struct hv_input_map_stats_page { pub type_: u32, pub padding: u32, pub identity: hv_stats_object_identity }
#[repr(C, packed)]
pub struct hv_input_map_stats_page2 { pub type_: u32, pub padding: u32, pub identity: hv_stats_object_identity, pub map_location: u64 }
#[repr(C, packed)]
pub struct hv_output_map_stats_page { pub map_location: u64 }
#[repr(C, packed)]
pub struct hv_input_unmap_stats_page { pub type_: u32, pub padding: u32, pub identity: hv_stats_object_identity }

#[repr(C, packed)]
pub struct hv_proximity_domain_flags { pub bits: u32 }
pub const HV_PROXIMITY_PREFERRED: u32 = 1;
pub const HV_PROXIMITY_INFO_VALID: u32 = 1 << 31;
#[repr(C, packed)]
pub struct hv_proximity_domain_info { pub domain_id: u32, pub flags: hv_proximity_domain_flags }
#[repr(C, packed)]
pub struct hv_deposit_memory { pub partition_id: u64, pub gpa_page_list: [u64; 0] }
#[repr(C, packed)]
pub struct hv_input_withdraw_memory { pub partition_id: u64, pub proximity_domain_info: hv_proximity_domain_info }
#[repr(C, packed)]
pub struct hv_output_withdraw_memory { pub gpa_page_list: [u64; 0] }

pub const HV_MAP_GPA_PERMISSIONS_NONE: u32 = 0x0; pub const HV_MAP_GPA_READABLE: u32 = 0x1; pub const HV_MAP_GPA_WRITABLE: u32 = 0x2; pub const HV_MAP_GPA_KERNEL_EXECUTABLE: u32 = 0x4; pub const HV_MAP_GPA_USER_EXECUTABLE: u32 = 0x8; pub const HV_MAP_GPA_EXECUTABLE: u32 = 0xC; pub const HV_MAP_GPA_PERMISSIONS_MASK: u32 = 0xF; pub const HV_MAP_GPA_ADJUSTABLE: u32 = 0x8000; pub const HV_MAP_GPA_NO_ACCESS: u32 = 0x10000; pub const HV_MAP_GPA_NOT_CACHED: u32 = 0x200000; pub const HV_MAP_GPA_LARGE_PAGE: u32 = 0x80000000;
#[repr(C, packed)] pub struct hv_input_map_gpa_pages { pub target_partition_id: u64, pub target_gpa_base: u64, pub map_flags: u32, pub padding: u32, pub source_gpa_page_list: [u64; 0] }
#[repr(C, packed)] pub struct hv_gpa_page_access_state_flags { pub bits: u64 }
pub const HV_GPA_CLEAR_ACCESSED: u64 = 1; pub const HV_GPA_SET_ACCESSED: u64 = 2; pub const HV_GPA_CLEAR_DIRTY: u64 = 4; pub const HV_GPA_SET_DIRTY: u64 = 8;
#[repr(C, packed)] pub struct hv_input_get_gpa_pages_access_state { pub partition_id: u64, pub flags: hv_gpa_page_access_state_flags, pub hv_gpa_page_number: u64 }
#[repr(C)] pub union hv_gpa_page_access_state { pub bits: u8, pub as_uint8: u8 }

#[repr(u32)] pub enum hv_crashdump_action { HV_CRASHDUMP_NONE = 0, HV_CRASHDUMP_SUSPEND_ALL_VPS, HV_CRASHDUMP_PREPARE_FOR_STATE_SAVE, HV_CRASHDUMP_STATE_SAVED, HV_CRASHDUMP_ENTRY }
#[repr(C, packed)] pub struct hv_partition_event_root_crashdump_input { pub crashdump_action: u32 }
#[repr(C, packed)] pub struct hv_input_disable_hyp_ex { pub rip: u64, pub arg: u64 }
#[repr(C, packed)] pub struct hv_crashdump_area { pub version: u32, pub flags_as_uint32: u32 }
#[repr(C)] pub union hv_partition_event_input { pub crashdump_input: hv_partition_event_root_crashdump_input }
#[repr(u32)] pub enum hv_partition_event { HV_PARTITION_EVENT_ROOT_CRASHDUMP = 2, HV_PARTITION_ALL_LOGICAL_PROCESSORS_STARTED = 4 }
#[repr(C, packed)] pub struct hv_input_notify_partition_event { pub event: u32, pub input: hv_partition_event_input }
#[repr(C, packed)] pub struct hv_input_get_logical_processor_run_time { pub lp_index: u32 }
#[repr(C, packed)] pub struct hv_output_get_logical_processor_run_time { pub global_time: u64, pub local_run_time: u64, pub rsvdz0: u64, pub hypervisor_time: u64 }
#[repr(C, packed)] pub struct hv_lp_startup_status { pub hv_status: u64, pub substatus1: u64, pub substatus2: u64, pub substatus3: u64, pub substatus4: u64, pub substatus5: u64, pub substatus6: u64 }
#[repr(C, packed)] pub struct hv_input_add_logical_processor { pub lp_index: u32, pub apic_id: u32, pub proximity_domain_info: hv_proximity_domain_info }
#[repr(C, packed)] pub struct hv_output_add_logical_processor { pub startup_status: hv_lp_startup_status }
#[repr(i32)] pub enum hv_subnode_type { HV_SUBNODE_ANY = 0, HV_SUBNODE_SOCKET, HV_SUBNODE_CLUSTER, HV_SUBNODE_L3, HV_SUBNODE_COUNT, HV_SUBNODE_INVALID = -1 }
#[repr(C, packed)] pub struct hv_create_vp { pub partition_id: u64, pub vp_index: u32, pub padding: [u8; 3], pub subnode_type: u8, pub subnode_id: u64, pub proximity_domain_info: hv_proximity_domain_info, pub flags: u64 }
#[repr(u32)] pub enum hv_interrupt_trigger_mode { HV_INTERRUPT_TRIGGER_MODE_EDGE = 0, HV_INTERRUPT_TRIGGER_MODE_LEVEL = 1 }

// Types supplied by hvgdk_mini.h.
#[repr(C, packed)] pub struct hv_device_interrupt_descriptor { pub interrupt_type: u32, pub trigger_mode: u32, pub vector_count: u32, pub reserved: u32, pub target: hv_device_interrupt_target }
#[repr(C, packed)] pub struct hv_input_map_device_interrupt { pub partition_id: u64, pub device_id: u64, pub flags: u32, pub base_irt_idx: u32, pub logical_interrupt_entry: hv_interrupt_entry, pub interrupt_descriptor: hv_device_interrupt_descriptor }
#[repr(C, packed)] pub struct hv_output_map_device_interrupt { pub interrupt_entry: hv_interrupt_entry, pub ext_status_deprecated: [u64; 5] }
#[repr(C, packed)] pub struct hv_input_unmap_device_interrupt { pub partition_id: u64, pub device_id: u64, pub interrupt_entry: hv_interrupt_entry, pub flags: u32 }
pub const HV_SOURCE_SHADOW_NONE: u32 = 0; pub const HV_SOURCE_SHADOW_BRIDGE_BUS_RANGE: u32 = 1;
#[repr(C, packed)] pub struct hv_send_ipi_ex { pub vector: u32, pub reserved: u32, pub vp_set: hv_vpset }
pub type hv_pci_rid = u16; pub type hv_pci_segment = u16; pub type hv_logical_device_id = u64;
#[repr(C)] pub union hv_pci_bdf { pub as_uint16: u16, pub fields: hv_pci_bdf_fields }
#[repr(C, packed)] pub struct hv_pci_bdf_fields { pub function_device: u8, pub bus: u8 }
pub const HV_PCI_BDF_FUNCTION_MASK: u8 = 0x07;
pub const HV_PCI_BDF_DEVICE_MASK: u8 = 0xf8;
#[repr(C)] pub union hv_pci_bus_range { pub as_uint16: u16, pub fields: hv_pci_bus_range_fields }
#[repr(C, packed)] pub struct hv_pci_bus_range_fields { pub subordinate_bus: u8, pub secondary_bus: u8 }
#[repr(u32)] pub enum hv_device_type { HV_DEVICE_TYPE_LOGICAL = 0, HV_DEVICE_TYPE_PCI, HV_DEVICE_TYPE_IOAPIC, HV_DEVICE_TYPE_ACPI }
#[repr(C)] pub union hv_device_id {
    pub as_uint64: u64,
    pub reserved: hv_device_id_type_bits,
    pub logical: hv_device_id_logical,
    pub pci: hv_device_id_pci,
    pub ioapic: hv_device_id_ioapic,
    pub acpi: hv_device_id_acpi,
}
#[repr(C, packed)] pub struct hv_device_id_type_bits { pub bits: u64 }
pub const HV_DEVICE_ID_DEVICE_TYPE_SHIFT: u32 = 62;
pub const HV_DEVICE_ID_DEVICE_TYPE_MASK: u64 = 0x3 << 62;
#[repr(C, packed)] pub struct hv_device_id_logical { pub bits: u64 }
#[repr(C)] pub union hv_device_id_pci_rid { pub rid: hv_pci_rid, pub bdf: hv_pci_bdf }
#[repr(C, packed)] pub struct hv_device_id_pci { pub rid_or_bdf: hv_device_id_pci_rid, pub segment: hv_pci_segment, pub shadow_bus_range: hv_pci_bus_range, pub flags: u16 }
pub const HV_DEVICE_ID_PCI_PHANTOM_FUNCTION_BITS_MASK: u16 = 0x3;
pub const HV_DEVICE_ID_PCI_SOURCE_SHADOW_MASK: u16 = 0x4;
pub const HV_DEVICE_ID_PCI_DEVICE_TYPE_MASK: u16 = 0xc000;
#[repr(C, packed)] pub struct hv_device_id_ioapic { pub ioapic_id: u8, pub rsvdz0: u8, pub rsvdz1: u16, pub rsvdz2: u16, pub flags: u16 }
pub const HV_DEVICE_ID_IOAPIC_DEVICE_TYPE_MASK: u16 = 0xc000;
#[repr(C, packed)] pub struct hv_device_id_acpi { pub input_mapping_base: u32, pub input_mapping_count_and_type: u32 }
pub const HV_DEVICE_ID_ACPI_INPUT_MAPPING_COUNT_MASK: u32 = 0x3fffffff;
pub const HV_DEVICE_ID_ACPI_DEVICE_TYPE_MASK: u32 = 0xc0000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
