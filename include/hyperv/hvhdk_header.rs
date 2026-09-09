/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of hvhdk.h. External types are supplied by other headers. */

pub const HV_X64_REGISTER_CLASS_GENERAL: u32 = 0;
pub const HV_X64_REGISTER_CLASS_IP: u32 = 1;
pub const HV_X64_REGISTER_CLASS_XMM: u32 = 2;
pub const HV_X64_REGISTER_CLASS_SEGMENT: u32 = 3;
pub const HV_X64_REGISTER_CLASS_FLAGS: u32 = 4;
pub const HV_VP_REGISTER_PAGE_VERSION_1: u32 = 1;
pub const HV_VP_REGISTER_PAGE_MAX_VECTOR_COUNT: usize = 7;

#[repr(C, packed)] pub struct hv_stats_page { pub data: [u64; HV_HYP_PAGE_SIZE / 8] }
#[repr(C, packed)] pub struct hv_vp_register_page_interrupt_vectors_data { pub vector_count: u8, pub vector: [u8; 7] }
#[repr(C)] pub union hv_vp_register_page_interrupt_vectors { pub as_uint64: u64, pub data: hv_vp_register_page_interrupt_vectors_data }

#[repr(C, packed)] pub struct hv_x64_general_registers { pub rax:u64,pub rcx:u64,pub rdx:u64,pub rbx:u64,pub rsp:u64,pub rbp:u64,pub rsi:u64,pub rdi:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64 }
#[repr(C)] pub union hv_x64_general_registers_union { pub named: hv_x64_general_registers, pub gp_registers: [u64;16] }
#[repr(C, packed)] pub struct hv_x64_registers { pub general: hv_x64_general_registers_union, pub rip:u64, pub rflags:u64 }
#[repr(C)] pub union hv_vp_register_page_registers { pub named: hv_x64_registers, pub registers: [u64;18] }
#[repr(C, packed)] pub struct hv_xmm_registers { pub xmm0: hv_u128,pub xmm1:hv_u128,pub xmm2:hv_u128,pub xmm3:hv_u128,pub xmm4:hv_u128,pub xmm5:hv_u128 }
#[repr(C)] pub union hv_vp_register_page_xmm { pub named: hv_xmm_registers, pub xmm_registers: [hv_u128;6] }
#[repr(C, packed)] pub struct hv_segment_registers { pub es:hv_x64_segment_register,pub cs:hv_x64_segment_register,pub ss:hv_x64_segment_register,pub ds:hv_x64_segment_register,pub fs:hv_x64_segment_register,pub gs:hv_x64_segment_register }
#[repr(C)] pub union hv_vp_register_page_segments { pub named: hv_segment_registers, pub segment_registers: [hv_x64_segment_register;6] }
#[repr(C, packed)] pub struct hv_vp_register_page {
 pub version:u16,pub isvalid:u8,pub rsvdz:u8,pub dirty:u32,
 pub registers:hv_vp_register_page_registers,pub reserved:[u8;8],pub xmm:hv_vp_register_page_xmm,pub segments:hv_vp_register_page_segments,
 pub cr0:u64,pub cr3:u64,pub cr4:u64,pub cr8:u64,pub efer:u64,pub dr7:u64,
 pub pending_interruption:hv_x64_pending_interruption_register,pub interrupt_state:hv_x64_interrupt_state_register,pub instruction_emulation_hints:u64,pub xfem:u64,pub reserved1:[u8;0x100],pub interrupt_vectors:hv_vp_register_page_interrupt_vectors }

/* C bitfields are represented by their packed storage words. */
#[repr(C)] pub union hv_partition_processor_features { pub as_uint64:[u64;2], pub bits:[u64;2] }
#[repr(C)] pub union hv_partition_processor_xsave_features { pub bits:u64, pub as_uint64:u64 }
#[repr(C, packed)] pub struct hv_partition_creation_properties { pub disabled_processor_features:hv_partition_processor_features,pub disabled_processor_xsave_features:hv_partition_processor_xsave_features }
#[repr(C)] pub union hv_partition_synthetic_processor_features { pub as_uint64:[u64;1], pub bits:u64 }
#[repr(C)] pub union hv_partition_isolation_properties { pub as_uint64:u64,pub bits:u64 }
pub const HV_PARTITION_PROCESSOR_FEATURES_BANKS:usize=2;
pub const HV_PARTITION_SYNTHETIC_PROCESSOR_FEATURES_BANKS:usize=1;
pub const HV_COMPATIBILITY_21_H2:u32=(0x6<<8)|0x9;
pub const HV_PARTITION_ISOLATION_TYPE_NONE:u32=0; pub const HV_PARTITION_ISOLATION_TYPE_SNP:u32=2; pub const HV_PARTITION_ISOLATION_TYPE_TDX:u32=3;
pub const HV_PARTITION_ISOLATION_HOST_TYPE_NONE:u32=0; pub const HV_PARTITION_ISOLATION_HOST_TYPE_HARDWARE:u32=1; pub const HV_PARTITION_ISOLATION_HOST_TYPE_RESERVED:u32=2;
pub const HV_PARTITION_CREATION_FLAG_SMT_ENABLED_GUEST:u64=1<<0; pub const HV_PARTITION_CREATION_FLAG_NESTED_VIRTUALIZATION_CAPABLE:u64=1<<1; pub const HV_PARTITION_CREATION_FLAG_GPA_SUPER_PAGES_ENABLED:u64=1<<4; pub const HV_PARTITION_CREATION_FLAG_EXO_PARTITION:u64=1<<8; pub const HV_PARTITION_CREATION_FLAG_LAPIC_ENABLED:u64=1<<13; pub const HV_PARTITION_CREATION_FLAG_INTERCEPT_MESSAGE_PAGE_ENABLED:u64=1<<19; pub const HV_PARTITION_CREATION_FLAG_X2APIC_CAPABLE:u64=1<<22;
#[repr(C, packed)] pub struct hv_input_create_partition { pub flags:u64,pub proximity_domain_info:hv_proximity_domain_info,pub compatibility_version:u32,pub padding:u32,pub partition_creation_properties:hv_partition_creation_properties,pub isolation_properties:hv_partition_isolation_properties }
#[repr(C, packed)] pub struct hv_output_create_partition{pub partition_id:u64} #[repr(C,packed)] pub struct hv_input_initialize_partition{pub partition_id:u64} #[repr(C,packed)] pub struct hv_input_finalize_partition{pub partition_id:u64} #[repr(C,packed)] pub struct hv_input_delete_partition{pub partition_id:u64}
#[repr(C,packed)] pub struct hv_input_get_partition_property{pub partition_id:u64,pub property_code:u32,pub padding:u32} #[repr(C,packed)] pub struct hv_output_get_partition_property{pub property_value:u64} #[repr(C,packed)] pub struct hv_input_set_partition_property{pub partition_id:u64,pub property_code:u32,pub padding:u32,pub property_value:u64}
#[repr(C)] pub union hv_partition_property_arg{pub as_uint64:u64,pub bits:u64}
#[repr(C,packed)] pub struct hv_input_get_partition_property_ex{pub partition_id:u64,pub property_code:u32,pub padding:u32,pub arg:u64}
pub const HV_PARTITION_PROPERTY_EX_MAX_VAR_SIZE:usize=HV_HYP_PAGE_SIZE-24;
#[repr(C)] pub union hv_partition_property_ex{pub buffer:[u8;HV_PARTITION_PROPERTY_EX_MAX_VAR_SIZE],pub vmm_capabilities:hv_partition_property_vmm_capabilities}
#[repr(C,packed)] pub struct hv_output_get_partition_property_ex{pub property_value:hv_partition_property_ex}

#[repr(u32)] pub enum hv_vp_state_page_type{HV_VP_STATE_PAGE_REGISTERS=0,HV_VP_STATE_PAGE_INTERCEPT_MESSAGE=1,HV_VP_STATE_PAGE_GHCB=2,HV_VP_STATE_PAGE_COUNT=3}
#[repr(C,packed)] pub struct hv_input_map_vp_state_page{pub partition_id:u64,pub vp_index:u32,pub type_:u16,pub input_vtl:hv_input_vtl,pub flags:u8,pub requested_map_location:u64}
#[repr(C,packed)] pub struct hv_output_map_vp_state_page{pub map_location:u64} #[repr(C,packed)] pub struct hv_input_unmap_vp_state_page{pub partition_id:u64,pub vp_index:u32,pub type_:u16,pub input_vtl:hv_input_vtl,pub reserved0:u8}
#[repr(C,packed)] pub struct hv_x64_apic_eoi_message{pub vp_index:u32,pub interrupt_vector:u32} #[repr(C,packed)] pub struct hv_opaque_intercept_message{pub vp_index:u32}
#[repr(u32)] pub enum hv_port_type{HV_PORT_TYPE_MESSAGE=1,HV_PORT_TYPE_EVENT=2,HV_PORT_TYPE_MONITOR=3,HV_PORT_TYPE_DOORBELL=4}
#[repr(C)] pub union hv_port_info_data{pub message_port_info:[u8;16],pub event_port_info:[u8;16],pub monitor_port_info:[u8;16],pub doorbell_port_info:[u8;16]}
#[repr(C,packed)] pub struct hv_port_info{pub port_type:u32,pub padding:u32,pub data:hv_port_info_data}
#[repr(C)] pub union hv_connection_info_data{pub message_connection_info:u64,pub event_connection_info:u64,pub monitor_connection_info:u64,pub doorbell_connection_info:[u64;3]}
#[repr(C,packed)] pub struct hv_connection_info{pub port_type:u32,pub padding:u32,pub data:hv_connection_info_data}

pub const HV_EVENT_FLAGS_COUNT:usize=2048; pub const HV_EVENT_FLAGS_BYTE_COUNT:usize=256; pub const HV_EVENT_FLAGS32_COUNT:usize=64; pub const HV_EVENT_FLAGS_UL_COUNT:usize=256/ core::mem::size_of::<usize>();
#[repr(C)] pub union hv_synic_event_flags{pub flags8:[u8;256],pub flags32:[u32;64],pub flags:[usize;HV_EVENT_FLAGS_UL_COUNT]}
#[repr(C)] pub struct hv_synic_event_flags_page{pub event_flags:[core::cell::UnsafeCell<hv_synic_event_flags>;HV_SYNIC_SINT_COUNT]}
#[repr(C,packed)] pub struct hv_synic_event_ring{pub signal_masked:u8,pub ring_full:u8,pub reserved_z:u16,pub data:[u32;63]} #[repr(C)] pub struct hv_synic_event_ring_page{pub sint_event_ring:[hv_synic_event_ring;HV_SYNIC_SINT_COUNT]}
#[repr(C)] pub union hv_synic_scontrol{pub as_uint64:u64,pub bits:u64} #[repr(C)] pub union hv_synic_siefp{pub as_uint64:u64,pub bits:u64} #[repr(C)] pub union hv_synic_sirbp{pub as_uint64:u64,pub bits:u64} #[repr(C)] pub union hv_interrupt_control{pub as_uint64:u64,pub bits:[u32;2]}
#[repr(C,packed)] pub struct hv_stimer_state{pub flags:u32,pub resvd:u32,pub config:u64,pub count:u64,pub adjustment:u64,pub undelivered_exp_time:u64} #[repr(C,packed)] pub struct hv_synthetic_timers_state{pub timers:[hv_stimer_state;HV_SYNIC_STIMER_COUNT],pub reserved:[u64;5]} #[repr(C,packed)] pub struct hv_async_completion_message_payload{pub partition_id:u64,pub status:u32,pub completion_count:u32,pub sub_status:u64}
#[repr(C,packed)] pub union hv_input_delete_vp{pub as_uint64:[u64;2],pub data:[u8;16]} #[repr(C,packed)] pub struct hv_input_assert_virtual_interrupt{pub partition_id:u64,pub control:hv_interrupt_control,pub dest_addr:u64,pub vector:u32,pub target_vtl:u8,pub rsvd_z0:u8,pub rsvd_z1:u16}
#[repr(C,packed)] pub struct hv_input_create_port{pub port_partition_id:u64,pub port_id:hv_port_id,pub port_vtl:u8,pub min_connection_vtl:u8,pub padding:u16,pub connection_partition_id:u64,pub port_info:hv_port_info,pub proximity_domain_info:hv_proximity_domain_info}
#[repr(C,packed)] pub union hv_input_delete_port{pub as_uint64:[u64;2],pub data:[u8;16]} #[repr(C,packed)] pub struct hv_input_connect_port{pub connection_partition_id:u64,pub connection_id:hv_connection_id,pub connection_vtl:u8,pub rsvdz0:u8,pub rsvdz1:u16,pub port_partition_id:u64,pub port_id:hv_port_id,pub reserved2:u32,pub connection_info:hv_connection_info,pub proximity_domain_info:hv_proximity_domain_info}
#[repr(C,packed)] pub union hv_input_disconnect_port{pub as_uint64:[u64;2],pub data:[u8;16]} #[repr(C,packed)] pub union hv_input_notify_port_ring_empty{pub as_uint64:u64,pub data:[u32;2]}
#[repr(C,packed)] pub struct hv_vp_state_data_xsave{pub flags:u64,pub states:hv_x64_xsave_xfem_register} #[repr(C,packed)] pub struct hv_vp_state_data{pub type_:u32,pub rsvd:u32,pub xsave:hv_vp_state_data_xsave}
pub const HV_GET_SET_VP_STATE_TYPE_PFN:u32=1<<31; pub const HV_GET_SET_VP_STATE_LAPIC_STATE:u32=HV_GET_SET_VP_STATE_TYPE_PFN; pub const HV_GET_SET_VP_STATE_XSAVE:u32=1|HV_GET_SET_VP_STATE_TYPE_PFN; pub const HV_GET_SET_VP_STATE_SIM_PAGE:u32=2|HV_GET_SET_VP_STATE_TYPE_PFN; pub const HV_GET_SET_VP_STATE_SIEF_PAGE:u32=3|HV_GET_SET_VP_STATE_TYPE_PFN; pub const HV_GET_SET_VP_STATE_SYNTHETIC_TIMERS:u32=4;
#[repr(C,packed)] pub struct hv_input_get_vp_state{pub partition_id:u64,pub vp_index:u32,pub input_vtl:u8,pub rsvd0:u8,pub rsvd1:u16,pub state_data:hv_vp_state_data,pub output_data_pfns:[u64;0]} #[repr(C,packed)] pub union hv_output_get_vp_state{pub synthetic_timers_state:hv_synthetic_timers_state}
#[repr(C,packed)] pub union hv_input_set_vp_state_data{pub pfns:u64,pub bytes:u8} #[repr(C,packed)] pub struct hv_input_set_vp_state{pub partition_id:u64,pub vp_index:u32,pub input_vtl:u8,pub rsvd0:u8,pub rsvd1:u16,pub state_data:hv_vp_state_data,pub data:[hv_input_set_vp_state_data;0]}
#[repr(C)] pub union hv_x64_vp_execution_state{pub as_uint16:u16,pub bits:u16} #[repr(C,packed)] pub struct hv_x64_intercept_message_header{pub vp_index:u32,pub instruction_length_and_cr8:u8,pub intercept_access_type:u8,pub execution_state:hv_x64_vp_execution_state,pub cs_segment:hv_x64_segment_register,pub rip:u64,pub rflags:u64}
#[repr(C)] pub union hv_x64_memory_access_info{pub as_uint8:u8,pub bits:u8} #[repr(C,packed)] pub struct hv_x64_memory_intercept_message{pub header:hv_x64_intercept_message_header,pub cache_type:u32,pub instruction_byte_count:u8,pub memory_access_info:hv_x64_memory_access_info,pub tpr_priority:u8,pub reserved1:u8,pub guest_virtual_address:u64,pub guest_physical_address:u64,pub instruction_bytes:[u8;16]}
#[repr(u32)] pub enum hv_vp_dispatch_state{HV_VP_DISPATCH_STATE_INVALID=0,HV_VP_DISPATCH_STATE_BLOCKED=1,HV_VP_DISPATCH_STATE_READY=2} #[repr(u32)] pub enum hv_vp_dispatch_event{HV_VP_DISPATCH_EVENT_INVALID=0,HV_VP_DISPATCH_EVENT_SUSPEND=1,HV_VP_DISPATCH_EVENT_INTERCEPT=2}
pub const HV_ROOT_SCHEDULER_MAX_VPS_PER_CHILD_PARTITION:usize=1024; pub const HV_GENERIC_SET_QWORD_COUNT:usize=(((HV_ROOT_SCHEDULER_MAX_VPS_PER_CHILD_PARTITION-1)>>6)+1)+2;
#[repr(C,packed)] pub struct hv_vp_signal_bitset_scheduler_message{pub partition_id:u64,pub overflow_count:u32,pub vp_count:u16,pub reserved:u16,pub vp_bitset:[u64;HV_GENERIC_SET_QWORD_COUNT]}
pub const HV_DISPATCH_VP_FLAG_CLEAR_INTERCEPT_SUSPEND:u32=1; pub const HV_DISPATCH_VP_FLAG_ENABLE_CALLER_INTERRUPTS:u32=2; pub const HV_DISPATCH_VP_FLAG_SET_CALLER_SPEC_CTRL:u32=4; pub const HV_DISPATCH_VP_FLAG_SKIP_VP_SPEC_FLUSH:u32=8; pub const HV_DISPATCH_VP_FLAG_SKIP_CALLER_SPEC_FLUSH:u32=16; pub const HV_DISPATCH_VP_FLAG_SKIP_CALLER_USER_SPEC_FLUSH:u32=32; pub const HV_DISPATCH_VP_FLAG_SCAN_INTERRUPT_INJECTION:u32=64;
#[repr(C,packed)] pub struct hv_input_dispatch_vp{pub partition_id:u64,pub vp_index:u32,pub flags:u32,pub time_slice:u64,pub spec_ctrl:u64} #[repr(C,packed)] pub struct hv_output_dispatch_vp{pub dispatch_state:u32,pub dispatch_event:u32}
#[repr(C,packed)] pub struct hv_input_modify_sparse_spa_page_host_access{pub host_access_and_reserved:u32,pub flags:u32,pub partition_id:u64,pub spa_page_list:[u64;0]}
pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_MAKE_EXCLUSIVE:u32=1; pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_MAKE_SHARED:u32=2; pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_LARGE_PAGE:u32=4; pub const HV_MODIFY_SPA_PAGE_HOST_ACCESS_HUGE_PAGE:u32=8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
