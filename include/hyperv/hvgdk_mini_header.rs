/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of hvgdk_mini.h.  External kernel symbols are intentionally left external. */
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::mem::MaybeUninit;

pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32; pub type u64_ = u64; pub type s64_ = i64;
pub const NSEC_PER_SEC: u64 = 1_000_000_000;
pub const HV_CLOCK_HZ: u64 = NSEC_PER_SEC / 100;
pub const HV_HYP_PAGE_SHIFT: u32 = 12;
pub const HV_HYP_PAGE_SIZE: usize = 1usize << HV_HYP_PAGE_SHIFT;
pub const HV_HYP_PAGE_MASK: usize = !(HV_HYP_PAGE_SIZE - 1);
pub const HV_HYP_LARGE_PAGE_SHIFT: u32 = 21;

#[repr(C, packed)] #[derive(Copy, Clone)] pub struct hv_u128 { pub low_part:u64, pub high_part:u64 }

macro_rules! c { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u64=$v;)* }; }
c!(HV_STATUS_SUCCESS=0x0,HV_STATUS_INVALID_HYPERCALL_CODE=0x2,HV_STATUS_INVALID_HYPERCALL_INPUT=0x3,HV_STATUS_INVALID_ALIGNMENT=0x4,HV_STATUS_INVALID_PARAMETER=0x5,HV_STATUS_ACCESS_DENIED=0x6,HV_STATUS_INVALID_PARTITION_STATE=0x7,HV_STATUS_OPERATION_DENIED=0x8,HV_STATUS_UNKNOWN_PROPERTY=0x9,HV_STATUS_PROPERTY_VALUE_OUT_OF_RANGE=0xa,HV_STATUS_INSUFFICIENT_MEMORY=0xb,HV_STATUS_INVALID_PARTITION_ID=0xd,HV_STATUS_INVALID_VP_INDEX=0xe,HV_STATUS_NOT_FOUND=0x10,HV_STATUS_INVALID_PORT_ID=0x11,HV_STATUS_INVALID_CONNECTION_ID=0x12,HV_STATUS_INSUFFICIENT_BUFFERS=0x13,HV_STATUS_NOT_ACKNOWLEDGED=0x14,HV_STATUS_INVALID_VP_STATE=0x15,HV_STATUS_NO_RESOURCES=0x1d,HV_STATUS_PROCESSOR_FEATURE_NOT_SUPPORTED=0x20,HV_STATUS_INVALID_LP_INDEX=0x41,HV_STATUS_INVALID_REGISTER_VALUE=0x50,HV_STATUS_OPERATION_FAILED=0x71,HV_STATUS_INSUFFICIENT_ROOT_MEMORY=0x73,HV_STATUS_INSUFFICIENT_CONTIGUOUS_MEMORY=0x75,HV_STATUS_TIME_OUT=0x78,HV_STATUS_CALL_PENDING=0x79,HV_STATUS_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY=0x83,HV_STATUS_VTL_ALREADY_ENABLED=0x86);
pub const HV_PARTITION_ID_INVALID:u64=0; pub const HV_PARTITION_ID_SELF:u64=u64::MAX;

/* The source contains CONFIG_X86/CONFIG_ARM64 alternatives; both forms are retained where layout differs. */
c!(HV_X64_MSR_GUEST_OS_ID=0x40000000,HV_X64_MSR_HYPERCALL=0x40000001,HV_X64_MSR_VP_INDEX=0x40000002,HV_X64_MSR_RESET=0x40000003,HV_X64_MSR_VP_RUNTIME=0x40000010,HV_X64_MSR_TIME_REF_COUNT=0x40000020,HV_X64_MSR_REFERENCE_TSC=0x40000021,HV_X64_MSR_TSC_FREQUENCY=0x40000022,HV_X64_MSR_APIC_FREQUENCY=0x40000023,HV_X64_MSR_EOI=0x40000070,HV_X64_MSR_ICR=0x40000071,HV_X64_MSR_TPR=0x40000072,HV_X64_MSR_VP_ASSIST_PAGE=0x40000073,HV_X64_MSR_SCONTROL=0x40000080,HV_X64_MSR_SVERSION=0x40000081,HV_X64_MSR_SIEFP=0x40000082,HV_X64_MSR_SIMP=0x40000083,HV_X64_MSR_EOM=0x40000084,HV_X64_MSR_SIRBP=0x40000085,HV_X64_MSR_SINT0=0x40000090,HV_X64_MSR_SINT15=0x4000009f,HV_X64_MSR_NESTED_SCONTROL=0x40001080,HV_X64_MSR_NESTED_SVERSION=0x40001081,HV_X64_MSR_NESTED_SIEFP=0x40001082,HV_X64_MSR_NESTED_SIMP=0x40001083,HV_X64_MSR_NESTED_EOM=0x40001084,HV_X64_MSR_NESTED_SINT0=0x40001090,HV_X64_MSR_STIMER0_CONFIG=0x400000b0,HV_X64_MSR_STIMER0_COUNT=0x400000b1,HV_X64_MSR_STIMER1_CONFIG=0x400000b2,HV_X64_MSR_STIMER1_COUNT=0x400000b3,HV_X64_MSR_STIMER2_CONFIG=0x400000b4,HV_X64_MSR_STIMER2_COUNT=0x400000b5,HV_X64_MSR_STIMER3_CONFIG=0x400000b6,HV_X64_MSR_STIMER3_COUNT=0x400000b7,HV_X64_MSR_GUEST_IDLE=0x400000f0,HV_X64_MSR_CRASH_P0=0x40000100,HV_X64_MSR_CRASH_P1=0x40000101,HV_X64_MSR_CRASH_P2=0x40000102,HV_X64_MSR_CRASH_P3=0x40000103,HV_X64_MSR_CRASH_P4=0x40000104,HV_X64_MSR_CRASH_CTL=0x40000105,HV_X64_MSR_REENLIGHTENMENT_CONTROL=0x40000106,HV_X64_MSR_TSC_EMULATION_CONTROL=0x40000107,HV_X64_MSR_TSC_EMULATION_STATUS=0x40000108,HV_X64_MSR_TSC_INVARIANT_CONTROL=0x40000118);
pub const HV_X64_MSR_HYPERCALL_ENABLE:u64=1; pub const HV_X64_MSR_HYPERCALL_PAGE_ADDRESS_SHIFT:u64=12; pub const HV_X64_MSR_HYPERCALL_PAGE_ADDRESS_MASK:u64=!((1u64<<12)-1); pub const HV_X64_MSR_CRASH_PARAMS:u64=6; pub const HV_IPI_LOW_VECTOR:u64=0x10; pub const HV_IPI_HIGH_VECTOR:u64=0xff; pub const HV_X64_MSR_VP_ASSIST_PAGE_ENABLE:u64=1; pub const HV_X64_MSR_VP_ASSIST_PAGE_ADDRESS_SHIFT:u64=12; pub const HV_X64_MSR_VP_ASSIST_PAGE_ADDRESS_MASK:u64=!((1u64<<12)-1); pub const HV_X64_ENLIGHTENED_VMCS_VERSION:u64=0xff; pub const HV_X64_MSR_TSC_REFERENCE_ENABLE:u64=1; pub const HV_X64_MSR_TSC_REFERENCE_ADDRESS_SHIFT:u64=12; pub const HV_HYPERCALL_MAX_XMM_REGISTERS:u64=6;

/* C bit-fields are represented by their containing integer, preserving size and bit storage. */
#[repr(C, packed)] pub struct hv_reenlightenment_control { pub bits:u64 }
#[repr(C, packed)] pub struct hv_tsc_emulation_status { pub bits:u64 }
#[repr(C, packed)] pub struct hv_tsc_emulation_control { pub bits:u64 }
#[repr(C, packed)] pub struct hv_output_get_partition_id { pub partition_id:u64 }
#[repr(C)] pub union hv_reference_tsc_msr { pub as_uint64:u64, pub bits:u64 }
pub const HV_MAX_SPARSE_VCPU_BANKS:usize=64; pub const HV_VCPUS_PER_SPARSE_BANK:usize=64;
#[repr(C, packed)] pub struct hv_vpset { pub format:u64,pub valid_bank_mask:u64,pub bank_contents:[u64;0] }
#[repr(C)] pub union hv_hypervisor_version_info { pub words:[u32;4], pub fields:[u32;4] }

/* CPUID and feature constants. */
c!(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS=0x40000000,HYPERV_CPUID_INTERFACE=0x40000001,HYPERV_CPUID_VERSION=0x40000002,HYPERV_CPUID_FEATURES=0x40000003,HYPERV_CPUID_ENLIGHTMENT_INFO=0x40000004,HYPERV_CPUID_IMPLEMENT_LIMITS=0x40000005,HYPERV_CPUID_CPU_MANAGEMENT_FEATURES=0x40000007,HYPERV_CPUID_NESTED_FEATURES=0x4000000a,HYPERV_CPUID_ISOLATION_CONFIG=0x4000000c,HYPERV_CPUID_VIRT_STACK_INTERFACE=0x40000081,HYPERV_VS_INTERFACE_EAX_SIGNATURE=0x31235356,HYPERV_CPUID_VIRT_STACK_PROPERTIES=0x40000082,HYPERV_HYPERVISOR_PRESENT_BIT=0x80000000,HYPERV_CPUID_MIN=0x40000005,HYPERV_CPUID_MAX=0x4000ffff);
macro_rules! bits { ($($n:ident=$b:expr),* $(,)?) => { $(pub const $n:u64=1u64<<$b;)* }; }
bits!(HV_MSR_VP_RUNTIME_AVAILABLE=0,HV_MSR_TIME_REF_COUNT_AVAILABLE=1,HV_MSR_SYNIC_AVAILABLE=2,HV_MSR_SYNTIMER_AVAILABLE=3,HV_MSR_APIC_ACCESS_AVAILABLE=4,HV_MSR_HYPERCALL_AVAILABLE=5,HV_MSR_VP_INDEX_AVAILABLE=6,HV_MSR_RESET_AVAILABLE=7,HV_MSR_STAT_PAGES_AVAILABLE=8,HV_MSR_REFERENCE_TSC_AVAILABLE=9,HV_MSR_GUEST_IDLE_AVAILABLE=10,HV_ACCESS_FREQUENCY_MSRS=11,HV_ACCESS_REENLIGHTENMENT=13,HV_ACCESS_TSC_INVARIANT=15,HV_CREATE_PARTITIONS=0,HV_ACCESS_PARTITION_ID=1,HV_ACCESS_MEMORY_POOL=2,HV_ADJUST_MESSAGE_BUFFERS=3,HV_POST_MESSAGES=4,HV_SIGNAL_EVENTS=5,HV_CREATE_PORT=6,HV_CONNECT_PORT=7,HV_ACCESS_STATS=8,HV_DEBUGGING=11,HV_CPU_MANAGEMENT=12,HV_ENABLE_EXTENDED_HYPERCALLS=20,HV_ISOLATION=22);
pub const HV_MAXIMUM_PROCESSORS:usize=2048; pub const HV_MAX_VP_INDEX:u32=(HV_MAXIMUM_PROCESSORS-1) as u32; pub const HV_VP_INDEX_SELF:u32=u32::MAX-1; pub const HV_ANY_VP:u32=u32::MAX;
#[repr(C)] pub union hv_x64_msr_hypercall_contents { pub as_uint64:u64,pub bits:u64 }
#[repr(C)] pub union hv_vp_assist_msr_contents { pub as_uint64:u64,pub bits:u64 }

/* Hypercall codes. */
c!(HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE=2,HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST=3,HVCALL_GET_LOGICAL_PROCESSOR_RUN_TIME=4,HVCALL_NOTIFY_LONG_SPIN_WAIT=8,HVCALL_SEND_IPI=0xb,HVCALL_ENABLE_VP_VTL=0xf,HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE_EX=0x13,HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST_EX=0x14,HVCALL_SEND_IPI_EX=0x15,HVCALL_CREATE_PARTITION=0x40,HVCALL_INITIALIZE_PARTITION=0x41,HVCALL_FINALIZE_PARTITION=0x42,HVCALL_DELETE_PARTITION=0x43,HVCALL_GET_PARTITION_PROPERTY=0x44,HVCALL_SET_PARTITION_PROPERTY=0x45,HVCALL_GET_PARTITION_ID=0x46,HVCALL_DEPOSIT_MEMORY=0x48,HVCALL_WITHDRAW_MEMORY=0x49,HVCALL_MAP_GPA_PAGES=0x4b,HVCALL_UNMAP_GPA_PAGES=0x4c,HVCALL_INSTALL_INTERCEPT=0x4d,HVCALL_CREATE_VP=0x4e,HVCALL_DELETE_VP=0x4f,HVCALL_GET_VP_REGISTERS=0x50,HVCALL_SET_VP_REGISTERS=0x51,HVCALL_TRANSLATE_VIRTUAL_ADDRESS=0x52,HVCALL_CLEAR_VIRTUAL_INTERRUPT=0x56,HVCALL_DELETE_PORT=0x58,HVCALL_DISCONNECT_PORT=0x5b,HVCALL_POST_MESSAGE=0x5c,HVCALL_SIGNAL_EVENT=0x5d,HVCALL_POST_DEBUG_DATA=0x69,HVCALL_RETRIEVE_DEBUG_DATA=0x6a,HVCALL_RESET_DEBUG_SESSION=0x6b,HVCALL_MAP_STATS_PAGE=0x6c,HVCALL_UNMAP_STATS_PAGE=0x6d,HVCALL_SET_SYSTEM_PROPERTY=0x6f,HVCALL_ADD_LOGICAL_PROCESSOR=0x76,HVCALL_GET_SYSTEM_PROPERTY=0x7b,HVCALL_MAP_DEVICE_INTERRUPT=0x7c,HVCALL_UNMAP_DEVICE_INTERRUPT=0x7d,HVCALL_RETARGET_INTERRUPT=0x7e,HVCALL_ENTER_SLEEP_STATE=0x84,HVCALL_NOTIFY_PARTITION_EVENT=0x87,HVCALL_NOTIFY_PORT_RING_EMPTY=0x8b,HVCALL_REGISTER_INTERCEPT_RESULT=0x91,HVCALL_ASSERT_VIRTUAL_INTERRUPT=0x94,HVCALL_CREATE_PORT=0x95,HVCALL_CONNECT_PORT=0x96,HVCALL_START_VP=0x99,HVCALL_GET_VP_INDEX_FROM_APIC_ID=0x9a,HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_SPACE=0xaf,HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_LIST=0xb0,HVCALL_SIGNAL_EVENT_DIRECT=0xc0,HVCALL_POST_MESSAGE_DIRECT=0xc1,HVCALL_DISPATCH_VP=0xc2,HVCALL_GET_GPA_PAGES_ACCESS_STATES=0xc9,HVCALL_ACQUIRE_SPARSE_SPA_PAGE_HOST_ACCESS=0xd7,HVCALL_RELEASE_SPARSE_SPA_PAGE_HOST_ACCESS=0xd8,HVCALL_MODIFY_SPARSE_GPA_PAGE_HOST_VISIBILITY=0xdb,HVCALL_MAP_VP_STATE_PAGE=0xe1,HVCALL_UNMAP_VP_STATE_PAGE=0xe2,HVCALL_GET_VP_STATE=0xe3,HVCALL_SET_VP_STATE=0xe4,HVCALL_GET_VP_CPUID_VALUES=0xf4,HVCALL_GET_PARTITION_PROPERTY_EX=0x101,HVCALL_MMIO_READ=0x106,HVCALL_MMIO_WRITE=0x107,HVCALL_DISABLE_HYP_EX=0x10f,HVCALL_MAP_STATS_PAGE2=0x131);

#[repr(C, packed)] pub struct hv_guest_mapping_flush { pub address_space:u64,pub flags:u64 }
pub const HV_MAX_FLUSH_PAGES:usize=2048; pub const HV_GPA_PAGE_RANGE_PAGE_SIZE_2MB:u64=0; pub const HV_GPA_PAGE_RANGE_PAGE_SIZE_1GB:u64=1;
#[repr(C)] pub union hv_gpa_page_range { pub address_space:u64,pub bits:u64 }
pub const HV_MAX_FLUSH_REP_COUNT:usize=(HV_HYP_PAGE_SIZE-16)/8;
#[repr(C)] pub struct hv_guest_mapping_flush_list { pub address_space:u64,pub flags:u64,pub gpa_list:[hv_gpa_page_range;HV_MAX_FLUSH_REP_COUNT] }
#[repr(C, packed)] pub struct hv_tlb_flush { pub address_space:u64,pub flags:u64,pub processor_mask:u64,pub gva_list:[u64;0] }
#[repr(C, packed)] pub struct hv_tlb_flush_ex { pub address_space:u64,pub flags:u64,pub hv_vp_set:hv_vpset,pub gva_list:[u64;0] }
#[repr(C, packed)] pub struct ms_hyperv_tsc_page { pub tsc_sequence:u32,pub reserved1:u32,pub tsc_scale:u64,pub tsc_offset:i64 }

pub const HV_SYNIC_SINT_COUNT:usize=16; pub const HV_SYNIC_VERSION_1:u64=1; pub const HV_SYNIC_FIRST_VALID_VECTOR:u64=16; pub const HV_SYNIC_CONTROL_ENABLE:u64=1; pub const HV_SYNIC_SIMP_ENABLE:u64=1; pub const HV_SYNIC_SIEFP_ENABLE:u64=1; pub const HV_SYNIC_SINT_MASKED:u64=1<<16; pub const HV_SYNIC_SINT_AUTO_EOI:u64=1<<17; pub const HV_SYNIC_SINT_VECTOR_MASK:u64=0xff; pub const HV_SYNIC_INTERCEPTION_SINT_INDEX:u64=0; pub const HV_SYNIC_IOMMU_FAULT_SINT_INDEX:u64=1; pub const HV_SYNIC_VMBUS_SINT_INDEX:u64=2; pub const HV_SYNIC_FIRST_UNUSED_SINT_INDEX:u64=5; pub const HV_SYNIC_DOORBELL_SINT_INDEX:u64=5;
#[repr(C)] pub union hv_synic_sint { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_x64_xsave_xfem_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_stimer_config { pub as_uint64:u64,pub bits:u64 }
pub const HV_SYNIC_STIMER_COUNT:usize=4;
#[repr(C)] pub union hv_port_id { pub asu32:u32,pub bits:u32 }
pub const HV_MESSAGE_SIZE:usize=256; pub const HV_MESSAGE_PAYLOAD_BYTE_COUNT:usize=240; pub const HV_MESSAGE_PAYLOAD_QWORD_COUNT:usize=30;
#[repr(C, packed)] pub struct hv_message_header { pub message_type:u32,pub payload_size:u8,pub message_flags:u8,pub reserved:[u8;2],pub sender:u64 }
#[repr(C, packed)] pub struct hv_notification_message_payload { pub sint_index:u32 }
#[repr(C, packed)] pub struct hv_message { pub header:hv_message_header,pub payload:[u64;HV_MESSAGE_PAYLOAD_QWORD_COUNT] }
#[repr(C, packed)] pub struct hv_message_page { pub sint_message:[hv_message;HV_SYNIC_SINT_COUNT] }
#[repr(C, packed)] pub struct hv_timer_message_payload { pub timer_index:u32,pub reserved:u32,pub expiration_time:u64,pub delivery_time:u64 }

#[repr(C, packed)] pub struct hv_x64_segment_register { pub base:u64,pub limit:u32,pub selector:u16,pub attributes:u16 }
#[repr(C, packed)] pub struct hv_x64_table_register { pub pad:[u16;3],pub limit:u16,pub base:u64 }
pub const HV_NORMAL_VTL:u8=0;
#[repr(C, packed)] pub union hv_input_vtl { pub as_uint8:u8,pub bits:u8 }
#[repr(C, packed)] pub struct hv_init_vp_context { pub rip:u64,pub rsp:u64,pub rflags:u64,pub cs:hv_x64_segment_register,pub ds:hv_x64_segment_register,pub es:hv_x64_segment_register,pub fs:hv_x64_segment_register,pub gs:hv_x64_segment_register,pub ss:hv_x64_segment_register,pub tr:hv_x64_segment_register,pub ldtr:hv_x64_segment_register,pub idtr:hv_x64_table_register,pub gdtr:hv_x64_table_register,pub efer:u64,pub cr0:u64,pub cr3:u64,pub cr4:u64,pub msr_cr_pat:u64 }
#[repr(C, packed)] pub struct hv_enable_vp_vtl { pub partition_id:u64,pub vp_index:u32,pub target_vtl:hv_input_vtl,pub mbz0:u8,pub mbz1:u16,pub vp_context:hv_init_vp_context }
#[repr(C, packed)] pub struct hv_get_vp_from_apic_id_in { pub partition_id:u64,pub target_vtl:hv_input_vtl,pub res:[u8;7],pub apic_ids:[u32;0] }
#[repr(C)] pub union hv_register_vsm_partition_config { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_register_vsm_capabilities { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_register_vsm_page_offsets { pub as_uint64:u64,pub bits:u64 }
#[repr(C, packed)] pub struct hv_nested_enlightenments_control { pub features:u32,pub hypercall_controls:u32 }
#[repr(C, packed)] pub struct hv_vp_assist_page { pub apic_assist:u32,pub reserved1:u32,pub vtl_entry_reason:u32,pub vtl_reserved:u32,pub vtl_ret_x64rax:u64,pub vtl_ret_x64rcx:u64,pub nested_control:hv_nested_enlightenments_control,pub enlighten_vmentry:u8,pub reserved2:[u8;7],pub current_nested_vmcs:u64,pub synthetic_time_unhalted_timer_expired:u8,pub reserved3:[u8;7],pub virtualization_fault_information:[u8;40],pub reserved4:[u8;8],pub intercept_message:[u8;256],pub vtl_ret_actions:[u8;256] }

/* Register values and remaining hypercall payloads. */
#[repr(C)] pub union hv_explicit_suspend_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_intercept_suspend_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_dispatch_suspend_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_arm64_pending_interruption_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_arm64_interrupt_state_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_arm64_pending_synthetic_exception_event { pub as_uint64:[u64;2],pub bits:[u64;2] } #[repr(C)] pub union hv_x64_interrupt_state_register { pub as_uint64:u64,pub bits:u64 } #[repr(C)] pub union hv_x64_pending_interruption_register { pub as_uint64:u64,pub bits:[u64;2] }
#[repr(C)] pub union hv_register_value { pub reg128:hv_u128,pub reg64:u64,pub reg32:u32,pub reg16:u16,pub reg8:u8,pub segment:hv_x64_segment_register,pub table:hv_x64_table_register,pub explicit_suspend:hv_explicit_suspend_register,pub intercept_suspend:hv_intercept_suspend_register,pub dispatch_suspend:hv_dispatch_suspend_register,pub pending_synthetic_exception_event:hv_arm64_pending_synthetic_exception_event }
#[repr(C)] pub struct hv_output_get_vp_registers { pub values:[hv_register_value;0] }
#[repr(C)] pub struct hv_register_assoc { pub name:u32,pub reserved1:u32,pub reserved2:u64,pub value:hv_register_value }
#[repr(C)] pub struct hv_input_get_vp_registers { pub partition_id:u64,pub vp_index:u32,pub input_vtl:hv_input_vtl,pub rsvd_z8:u8,pub rsvd_z16:u16,pub names:[u32;0] }
#[repr(C)] pub struct hv_input_set_vp_registers { pub partition_id:u64,pub vp_index:u32,pub input_vtl:hv_input_vtl,pub rsvd_z8:u8,pub rsvd_z16:u16,pub elements:[hv_register_assoc;0] }
pub const HV_UNMAP_GPA_LARGE_PAGE:u64=2; #[repr(C, packed)] pub struct hv_send_ipi { pub vector:u32,pub reserved:u32,pub cpu_mask:u64 }
pub const HV_VTL_MASK:u32=0xf;
#[repr(C, packed)] pub struct hv_gpa_range_for_visibility { pub partition_id:u64,pub host_visibility:u32,pub reserved1:u32,pub gpa_page_list:[u64;HV_HYP_PAGE_SIZE/8-2] }
pub const HV_HYPERCALL_MMIO_MAX_DATA_LENGTH:usize=64; #[repr(C, packed)] pub struct hv_mmio_read_input { pub gpa:u64,pub size:u32,pub reserved:u32 } #[repr(C, packed)] pub struct hv_mmio_read_output { pub data:[u8;64] } #[repr(C, packed)] pub struct hv_mmio_write_input { pub gpa:u64,pub size:u32,pub reserved:u32,pub data:[u8;64] }

#[repr(u32)] pub enum hv_isolation_type { HV_ISOLATION_TYPE_NONE=0,HV_ISOLATION_TYPE_VBS=1,HV_ISOLATION_TYPE_SNP=2,HV_ISOLATION_TYPE_TDX=3 }
#[repr(u32)] pub enum hv_interrupt_type { HV_X64_INTERRUPT_TYPE_FIXED=0,HV_X64_INTERRUPT_TYPE_LOWESTPRIORITY=1,HV_X64_INTERRUPT_TYPE_SMI=2,HV_X64_INTERRUPT_TYPE_REMOTEREAD=3,HV_X64_INTERRUPT_TYPE_NMI=4,HV_X64_INTERRUPT_TYPE_INIT=5,HV_X64_INTERRUPT_TYPE_SIPI=6,HV_X64_INTERRUPT_TYPE_EXTINT=7,HV_X64_INTERRUPT_TYPE_LOCALINT0=8,HV_X64_INTERRUPT_TYPE_LOCALINT1=9,HV_X64_INTERRUPT_TYPE_MAXIMUM=10 }
#[repr(u32)] pub enum hv_message_type { HVMSG_NONE=0,HVMSG_UNMAPPED_GPA=0x80000000,HVMSG_GPA_INTERCEPT=0x80000001,HVMSG_TIMER_EXPIRED=0x80000010,HVMSG_INVALID_VP_REGISTER_VALUE=0x80000020,HVMSG_UNRECOVERABLE_EXCEPTION=0x80000021,HVMSG_UNSUPPORTED_FEATURE=0x80000022,HVMSG_OPAQUE_INTERCEPT=0x8000003f,HVMSG_EVENTLOG_BUFFERCOMPLETE=0x80000040,HVMSG_HYPERCALL_INTERCEPT=0x80000050,HVMSG_SYNIC_EVENT_INTERCEPT=0x80000060,HVMSG_SYNIC_SINT_INTERCEPT=0x80000061,HVMSG_SYNIC_SINT_DELIVERABLE=0x80000062,HVMSG_ASYNC_CALL_COMPLETION=0x80000070,HVMSG_SCHEDULER_VP_SIGNAL_BITSET=0x80000100,HVMSG_SCHEDULER_VP_SIGNAL_PAIR=0x80000101,HVMSG_X64_IO_PORT_INTERCEPT=0x80010000,HVMSG_X64_MSR_INTERCEPT=0x80010001,HVMSG_X64_CPUID_INTERCEPT=0x80010002,HVMSG_X64_EXCEPTION_INTERCEPT=0x80010003,HVMSG_X64_APIC_EOI=0x80010004,HVMSG_X64_LEGACY_FP_ERROR=0x80010005,HVMSG_X64_IOMMU_PRQ=0x80010006,HVMSG_X64_HALT=0x80010007,HVMSG_X64_INTERRUPTION_DELIVERABLE=0x80010008,HVMSG_X64_SIPI_INTERCEPT=0x80010009 }
#[repr(u32)] pub enum hv_mem_host_visibility { VMBUS_PAGE_NOT_VISIBLE=0,VMBUS_PAGE_VISIBLE_READ_ONLY=1,VMBUS_PAGE_VISIBLE_READ_WRITE=3 }
#[repr(u32)] pub enum hv_interrupt_source { HV_INTERRUPT_SOURCE_MSI=1,HV_INTERRUPT_SOURCE_IOAPIC=2 }
#[repr(C, packed)] pub struct hv_interrupt_entry { pub source:u32,pub reserved1:u32,pub entry:u64 }
pub const HV_DEVICE_INTERRUPT_TARGET_MULTICAST:u32=1; pub const HV_DEVICE_INTERRUPT_TARGET_PROCESSOR_SET:u32=2;
#[repr(C, packed)] pub struct hv_device_interrupt_target { pub vector:u32,pub flags:u32,pub vp_mask:u64 }
#[repr(C, packed)] pub struct hv_retarget_device_interrupt { pub partition_id:u64,pub device_id:u64,pub int_entry:hv_interrupt_entry,pub reserved2:u64,pub int_target:hv_device_interrupt_target }
#[repr(u32)] pub enum hv_intercept_access_type { HV_INTERCEPT_ACCESS_READ=0,HV_INTERCEPT_ACCESS_WRITE=1,HV_INTERCEPT_ACCESS_EXECUTE=2 }
#[repr(C)] pub union hv_intercept_parameters { pub as_uint64:u64,pub io_port:u16,pub cpuid_index:u32,pub apic_write_mask:u32,pub exception_vector:u16,pub msr_index:u32 }
#[repr(u32)] pub enum hv_intercept_type { HV_INTERCEPT_TYPE_X64_IO_PORT=0,HV_INTERCEPT_TYPE_X64_MSR=1,HV_INTERCEPT_TYPE_X64_CPUID=2,HV_INTERCEPT_TYPE_EXCEPTION=3,HV_INTERCEPT_TYPE_RESERVED0=4,HV_INTERCEPT_TYPE_MMIO=5,HV_INTERCEPT_TYPE_X64_GLOBAL_CPUID=6,HV_INTERCEPT_TYPE_X64_APIC_SMI=7,HV_INTERCEPT_TYPE_HYPERCALL=8,HV_INTERCEPT_TYPE_X64_APIC_INIT_SIPI=9,HV_INTERCEPT_MC_UPDATE_PATCH_LEVEL_MSR_READ=10,HV_INTERCEPT_TYPE_X64_APIC_WRITE=11,HV_INTERCEPT_TYPE_X64_MSR_INDEX=12,HV_INTERCEPT_TYPE_MAX=13,HV_INTERCEPT_TYPE_INVALID=0xffff_ffff }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
