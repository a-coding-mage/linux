/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from x86/include/uapi/asm/kvm.h. */

pub const KVM_PIO_PAGE_OFFSET: u32 = 1;
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 2;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u32 = 64;

pub const DE_VECTOR:u32=0; pub const DB_VECTOR:u32=1; pub const BP_VECTOR:u32=3; pub const OF_VECTOR:u32=4;
pub const BR_VECTOR:u32=5; pub const UD_VECTOR:u32=6; pub const NM_VECTOR:u32=7; pub const DF_VECTOR:u32=8;
pub const TS_VECTOR:u32=10; pub const NP_VECTOR:u32=11; pub const SS_VECTOR:u32=12; pub const GP_VECTOR:u32=13;
pub const PF_VECTOR:u32=14; pub const MF_VECTOR:u32=16; pub const AC_VECTOR:u32=17; pub const MC_VECTOR:u32=18;
pub const XM_VECTOR:u32=19; pub const VE_VECTOR:u32=20; pub const CP_VECTOR:u32=21;
pub const HV_VECTOR:u32=28; pub const VC_VECTOR:u32=29; pub const SX_VECTOR:u32=30;
pub const KVM_NR_INTERRUPTS:usize=256;

pub const KVM_IOAPIC_NUM_PINS:usize=24;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_pic_state { pub last_irr:u8,pub irr:u8,pub imr:u8,pub isr:u8,pub priority_add:u8,pub irq_base:u8,pub read_reg_select:u8,pub poll:u8,pub special_mask:u8,pub init_state:u8,pub auto_eoi:u8,pub rotate_on_auto_eoi:u8,pub special_fully_nested_mode:u8,pub init4:u8,pub elcr:u8,pub elcr_mask:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_ioapic_redir_fields { pub vector:u8, pub delivery_mode:u8, pub dest_mode:u8, pub delivery_status:u8, pub polarity:u8, pub remote_irr:u8, pub trig_mode:u8, pub mask:u8, pub reserve:u8, pub reserved:[u8;4], pub dest_id:u8 }
#[repr(C)] pub union kvm_ioapic_redir { pub bits:u64, pub fields:kvm_ioapic_redir_fields }
#[repr(C)] pub struct kvm_ioapic_state { pub base_address:u64,pub ioregsel:u32,pub id:u32,pub irr:u32,pub pad:u32,pub redirtbl:[kvm_ioapic_redir;KVM_IOAPIC_NUM_PINS] }

pub const KVM_IRQCHIP_PIC_MASTER:u32=0; pub const KVM_IRQCHIP_PIC_SLAVE:u32=1; pub const KVM_IRQCHIP_IOAPIC:u32=2; pub const KVM_NR_IRQCHIPS:u32=3;
pub const KVM_RUN_X86_SMM:u32=1<<0; pub const KVM_RUN_X86_BUS_LOCK:u32=1<<1; pub const KVM_RUN_X86_GUEST_MODE:u32=1<<2;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_regs { pub rax:u64,pub rbx:u64,pub rcx:u64,pub rdx:u64,pub rsi:u64,pub rdi:u64,pub rsp:u64,pub rbp:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64,pub rip:u64,pub rflags:u64 }
pub const KVM_APIC_REG_SIZE:usize=0x400; #[repr(C)] pub struct kvm_lapic_state { pub regs:[i8;KVM_APIC_REG_SIZE] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_segment { pub base:u64,pub limit:u32,pub selector:u16,pub type_:u8,pub present:u8,pub dpl:u8,pub db:u8,pub s:u8,pub l:u8,pub g:u8,pub avl:u8,pub unusable:u8,pub padding:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_dtable { pub base:u64,pub limit:u16,pub padding:[u16;3] }
#[repr(C)] pub struct kvm_sregs { pub cs:kvm_segment,pub ds:kvm_segment,pub es:kvm_segment,pub fs:kvm_segment,pub gs:kvm_segment,pub ss:kvm_segment,pub tr:kvm_segment,pub ldt:kvm_segment,pub gdt:kvm_dtable,pub idt:kvm_dtable,pub cr0:u64,pub cr2:u64,pub cr3:u64,pub cr4:u64,pub cr8:u64,pub efer:u64,pub apic_base:u64,pub interrupt_bitmap:[u64;4] }
#[repr(C)] pub struct kvm_sregs2 { pub cs:kvm_segment,pub ds:kvm_segment,pub es:kvm_segment,pub fs:kvm_segment,pub gs:kvm_segment,pub ss:kvm_segment,pub tr:kvm_segment,pub ldt:kvm_segment,pub gdt:kvm_dtable,pub idt:kvm_dtable,pub cr0:u64,pub cr2:u64,pub cr3:u64,pub cr4:u64,pub cr8:u64,pub efer:u64,pub apic_base:u64,pub flags:u64,pub pdptrs:[u64;4] }
pub const KVM_SREGS2_FLAGS_PDPTRS_VALID:u32=1;
#[repr(C)] pub struct kvm_fpu { pub fpr:[[u8;16];8],pub fcw:u16,pub fsw:u16,pub ftwx:u8,pub pad1:u8,pub last_opcode:u16,pub last_ip:u64,pub last_dp:u64,pub xmm:[[u8;16];16],pub mxcsr:u32,pub pad2:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_msr_entry { pub index:u32,pub reserved:u32,pub data:u64 }
#[repr(C)] pub struct kvm_msrs { pub nmsrs:u32,pub pad:u32,pub entries:[kvm_msr_entry;0] }
#[repr(C)] pub struct kvm_msr_list { pub nmsrs:u32,pub indices:[u32;0] }
pub const KVM_MSR_FILTER_MAX_BITMAP_SIZE:u32=0x600; pub const KVM_MSR_FILTER_READ:u32=1<<0; pub const KVM_MSR_FILTER_WRITE:u32=1<<1; pub const KVM_MSR_FILTER_RANGE_VALID_MASK:u32=3;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_msr_filter_range { pub flags:u32,pub nmsrs:u32,pub base:u32,pub bitmap:*mut u8 }
pub const KVM_MSR_FILTER_MAX_RANGES:usize=16; pub const KVM_MSR_FILTER_DEFAULT_ALLOW:u32=0; pub const KVM_MSR_FILTER_DEFAULT_DENY:u32=1; pub const KVM_MSR_FILTER_VALID_MASK:u32=1;
#[repr(C)] pub struct kvm_msr_filter { pub flags:u32,pub ranges:[kvm_msr_filter_range;KVM_MSR_FILTER_MAX_RANGES] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_cpuid_entry { pub function:u32,pub eax:u32,pub ebx:u32,pub ecx:u32,pub edx:u32,pub padding:u32 }
#[repr(C)] pub struct kvm_cpuid { pub nent:u32,pub padding:u32,pub entries:[kvm_cpuid_entry;0] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_cpuid_entry2 { pub function:u32,pub index:u32,pub flags:u32,pub eax:u32,pub ebx:u32,pub ecx:u32,pub edx:u32,pub padding:[u32;3] }
pub const KVM_CPUID_FLAG_SIGNIFCANT_INDEX:u32=1; pub const KVM_CPUID_FLAG_STATEFUL_FUNC:u32=2; pub const KVM_CPUID_FLAG_STATE_READ_NEXT:u32=4;
#[repr(C)] pub struct kvm_cpuid2 { pub nent:u32,pub padding:u32,pub entries:[kvm_cpuid_entry2;0] }

#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_pit_channel_state { pub count:u32,pub latched_count:u16,pub count_latched:u8,pub status_latched:u8,pub status:u8,pub read_state:u8,pub write_state:u8,pub write_latch:u8,pub rw_mode:u8,pub mode:u8,pub bcd:u8,pub gate:u8,pub count_load_time:i64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_debug_exit_arch { pub exception:u32,pub pad:u32,pub pc:u64,pub dr6:u64,pub dr7:u64 }
pub const KVM_GUESTDBG_USE_SW_BP:u32=0x10000; pub const KVM_GUESTDBG_USE_HW_BP:u32=0x20000; pub const KVM_GUESTDBG_INJECT_DB:u32=0x40000; pub const KVM_GUESTDBG_INJECT_BP:u32=0x80000; pub const KVM_GUESTDBG_BLOCKIRQ:u32=0x100000;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_guest_debug_arch { pub debugreg:[u64;8] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_pit_state { pub channels:[kvm_pit_channel_state;3] }
pub const KVM_PIT_FLAGS_HPET_LEGACY:u32=1; pub const KVM_PIT_FLAGS_SPEAKER_DATA_ON:u32=2;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_pit_state2 { pub channels:[kvm_pit_channel_state;3],pub flags:u32,pub reserved:[u32;9] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_reinject_control { pub pit_reinject:u8,pub reserved:[u8;31] }

pub const KVM_VCPUEVENT_VALID_NMI_PENDING:u32=1; pub const KVM_VCPUEVENT_VALID_SIPI_VECTOR:u32=2; pub const KVM_VCPUEVENT_VALID_SHADOW:u32=4; pub const KVM_VCPUEVENT_VALID_SMM:u32=8; pub const KVM_VCPUEVENT_VALID_PAYLOAD:u32=16; pub const KVM_VCPUEVENT_VALID_TRIPLE_FAULT:u32=32;
pub const KVM_X86_SHADOW_INT_MOV_SS:u32=1; pub const KVM_X86_SHADOW_INT_STI:u32=2;
#[repr(C)] pub struct kvm_vcpu_events { pub exception:[u8;4],pub exception_error_code:u32,pub interrupt:[u8;4],pub nmi:[u8;4],pub sipi_vector:u32,pub flags:u32,pub smi:[u8;4],pub triple_fault:[u8;1],pub reserved:[u8;26],pub exception_has_payload:u8,pub exception_payload:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_debugregs { pub db:[u64;4],pub dr6:u64,pub dr7:u64,pub flags:u64,pub reserved:[u64;9] }
#[repr(C)] pub struct kvm_xsave { pub region:[u32;1024],pub extra:[u32;0] }
pub const KVM_MAX_XCRS:usize=16; #[repr(C)] #[derive(Copy,Clone)] pub struct kvm_xcr { pub xcr:u32,pub reserved:u32,pub value:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_xcrs { pub nr_xcrs:u32,pub flags:u32,pub xcrs:[kvm_xcr;KVM_MAX_XCRS],pub padding:[u64;16] }
pub const KVM_X86_REG_TYPE_MSR:u64=2; pub const KVM_X86_REG_TYPE_KVM:u64=3; pub const KVM_REG_GUEST_SSP:u64=0;
/* KVM_REG_X86, KVM_REG_SIZE_U64 are supplied by the generic KVM header. */
#[inline] pub const fn kvm_x86_kvm_reg_size(reg:u64)->u64 { if reg==KVM_REG_GUEST_SSP { KVM_REG_SIZE_U64 } else { 0 } }
#[inline] pub const fn kvm_x86_reg_type_size(ty:u64,reg:u64)->u64 { (ty<<32) | if ty==KVM_X86_REG_TYPE_MSR { KVM_REG_SIZE_U64 } else if ty==KVM_X86_REG_TYPE_KVM { kvm_x86_kvm_reg_size(reg) } else { 0 } }
#[inline] pub const fn kvm_x86_reg_id(ty:u64,index:u64)->u64 { KVM_REG_X86 | kvm_x86_reg_type_size(ty,index) | index }
#[inline] pub const fn kvm_x86_reg_msr(index:u64)->u64 { kvm_x86_reg_id(KVM_X86_REG_TYPE_MSR,index) }
#[inline] pub const fn kvm_x86_reg_kvm(index:u64)->u64 { kvm_x86_reg_id(KVM_X86_REG_TYPE_KVM,index) }
pub const KVM_SYNC_X86_REGS:u64=1; pub const KVM_SYNC_X86_SREGS:u64=2; pub const KVM_SYNC_X86_EVENTS:u64=4; pub const KVM_SYNC_X86_VALID_FIELDS:u64=7;
#[repr(C)] pub struct kvm_sync_regs { pub regs:kvm_regs,pub sregs:kvm_sregs,pub events:kvm_vcpu_events }

pub const KVM_X86_QUIRK_LINT0_REENABLED:u32=1; pub const KVM_X86_QUIRK_CD_NW_CLEARED:u32=2; pub const KVM_X86_QUIRK_LAPIC_MMIO_HOLE:u32=4; pub const KVM_X86_QUIRK_OUT_7E_INC_RIP:u32=8; pub const KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT:u32=16; pub const KVM_X86_QUIRK_FIX_HYPERCALL_INSN:u32=32; pub const KVM_X86_QUIRK_MWAIT_NEVER_UD_FAULTS:u32=64; pub const KVM_X86_QUIRK_SLOT_ZAP_ALL:u32=128; pub const KVM_X86_QUIRK_STUFF_FEATURE_MSRS:u32=256; pub const KVM_X86_QUIRK_IGNORE_GUEST_PAT:u32=512; pub const KVM_X86_QUIRK_VMCS12_ALLOW_FREEZE_IN_SMM:u32=1024; pub const KVM_X86_QUIRK_NESTED_SVM_SHARED_PAT:u32=2048;

pub const KVM_STATE_NESTED_FORMAT_VMX:u32=0; pub const KVM_STATE_NESTED_FORMAT_SVM:u32=1; pub const KVM_STATE_NESTED_GUEST_MODE:u32=1; pub const KVM_STATE_NESTED_RUN_PENDING:u32=2; pub const KVM_STATE_NESTED_EVMCS:u32=4; pub const KVM_STATE_NESTED_MTF_PENDING:u32=8; pub const KVM_STATE_NESTED_GIF_SET:u32=0x100; pub const KVM_STATE_NESTED_SMM_GUEST_MODE:u32=1; pub const KVM_STATE_NESTED_SMM_VMXON:u32=2; pub const KVM_STATE_NESTED_VMX_VMCS_SIZE:usize=0x1000; pub const KVM_STATE_NESTED_SVM_VMCB_SIZE:usize=0x1000; pub const KVM_STATE_VMX_PREEMPTION_TIMER_DEADLINE:u32=1;
pub const KVM_X86_GRP_SYSTEM:u32=0; pub const KVM_X86_XCOMP_GUEST_SUPP:u32=0; pub const KVM_X86_GRP_SEV:u32=1; pub const KVM_X86_SEV_VMSA_FEATURES:u32=0; pub const KVM_X86_SNP_POLICY_BITS:u32=1; pub const KVM_X86_SEV_SNP_REQ_CERTS:u32=2;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_vmx_nested_state_data { pub vmcs12:[u8;0x1000],pub shadow_vmcs12:[u8;0x1000] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_vmx_nested_state_hdr_smm { pub flags:u16 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_vmx_nested_state_hdr { pub vmxon_pa:u64,pub vmcs12_pa:u64,pub smm:kvm_vmx_nested_state_hdr_smm,pub pad:u16,pub flags:u32,pub preemption_timer_deadline:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_svm_nested_state_data { pub vmcb12:[u8;0x1000] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_svm_nested_state_hdr { pub vmcb_pa:u64,pub gpat:u64 }
#[repr(C)] pub union kvm_nested_state_hdr { pub vmx:kvm_vmx_nested_state_hdr,pub svm:kvm_svm_nested_state_hdr,pub pad:[u8;120] }
#[repr(C)] pub union kvm_nested_state_data { pub vmx:[kvm_vmx_nested_state_data;0],pub svm:[kvm_svm_nested_state_data;0] }
#[repr(C)] pub struct kvm_nested_state { pub flags:u16,pub format:u16,pub size:u32,pub hdr:kvm_nested_state_hdr,pub data:kvm_nested_state_data }
#[repr(C)] pub struct kvm_pmu_event_filter { pub action:u32,pub nevents:u32,pub fixed_counter_bitmap:u32,pub flags:u32,pub pad:[u32;4],pub events:[u64;0] }
pub const KVM_PMU_EVENT_ALLOW:u32=0; pub const KVM_PMU_EVENT_DENY:u32=1; pub const KVM_PMU_EVENT_FLAG_MASKED_EVENTS:u64=1; pub const KVM_PMU_EVENT_FLAGS_VALID_MASK:u64=1;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_x86_mce { pub status:u64,pub addr:u64,pub misc:u64,pub mcg_status:u64,pub bank:u8,pub pad1:[u8;7],pub pad2:[u64;3] }

pub const KVM_XEN_HVM_CONFIG_HYPERCALL_MSR:u32=1; pub const KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL:u32=2; pub const KVM_XEN_HVM_CONFIG_SHARED_INFO:u32=4; pub const KVM_XEN_HVM_CONFIG_RUNSTATE:u32=8; pub const KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL:u32=16; pub const KVM_XEN_HVM_CONFIG_EVTCHN_SEND:u32=32; pub const KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG:u32=64; pub const KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE:u32=128; pub const KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA:u32=256; pub const KVM_XEN_MSR_MIN_INDEX:u32=0x40000000; pub const KVM_XEN_MSR_MAX_INDEX:u32=0x4fffffff;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_xen_hvm_config { pub flags:u32,pub msr:u32,pub blob_addr_32:u64,pub blob_addr_64:u64,pub blob_size_32:u8,pub blob_size_64:u8,pub pad2:[u8;30] }
pub const KVM_XEN_INVALID_GFN:u64=u64::MAX; pub const KVM_XEN_INVALID_GPA:u64=u64::MAX;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_xen_hvm_attr { pub type_:u16,pub pad:[u16;3],pub u:[u64;8] }
pub const KVM_XEN_ATTR_TYPE_LONG_MODE:u32=0; pub const KVM_XEN_ATTR_TYPE_SHARED_INFO:u32=1; pub const KVM_XEN_ATTR_TYPE_UPCALL_VECTOR:u32=2; pub const KVM_XEN_ATTR_TYPE_EVTCHN:u32=3; pub const KVM_XEN_ATTR_TYPE_XEN_VERSION:u32=4; pub const KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG:u32=5; pub const KVM_XEN_ATTR_TYPE_SHARED_INFO_HVA:u32=6;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_xen_vcpu_attr { pub type_:u16,pub pad:[u16;3],pub u:[u64;8] }
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO:u32=0; pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_TIME_INFO:u32=1; pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR:u32=2; pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_CURRENT:u32=3; pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_DATA:u32=4; pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST:u32=5; pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID:u32=6; pub const KVM_XEN_VCPU_ATTR_TYPE_TIMER:u32=7; pub const KVM_XEN_VCPU_ATTR_TYPE_UPCALL_VECTOR:u32=8; pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA:u32=9;

#[repr(u32)] pub enum sev_cmd_id { KVM_SEV_INIT=0,KVM_SEV_ES_INIT,KVM_SEV_LAUNCH_START,KVM_SEV_LAUNCH_UPDATE_DATA,KVM_SEV_LAUNCH_UPDATE_VMSA,KVM_SEV_LAUNCH_SECRET,KVM_SEV_LAUNCH_MEASURE,KVM_SEV_LAUNCH_FINISH,KVM_SEV_SEND_START,KVM_SEV_SEND_UPDATE_DATA,KVM_SEV_SEND_UPDATE_VMSA,KVM_SEV_SEND_FINISH,KVM_SEV_RECEIVE_START,KVM_SEV_RECEIVE_UPDATE_DATA,KVM_SEV_RECEIVE_UPDATE_VMSA,KVM_SEV_RECEIVE_FINISH,KVM_SEV_GUEST_STATUS,KVM_SEV_DBG_DECRYPT,KVM_SEV_DBG_ENCRYPT,KVM_SEV_CERT_EXPORT,KVM_SEV_GET_ATTESTATION_REPORT,KVM_SEV_SEND_CANCEL,KVM_SEV_INIT2=23,KVM_SEV_SNP_LAUNCH_START=100,KVM_SEV_SNP_LAUNCH_UPDATE,KVM_SEV_SNP_LAUNCH_FINISH,KVM_SEV_SNP_ENABLE_REQ_CERTS,KVM_SEV_NR_MAX }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_cmd { pub id:u32,pub pad0:u32,pub data:u64,pub error:u32,pub sev_fd:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_init { pub vmsa_features:u64,pub flags:u32,pub ghcb_version:u16,pub pad1:u16,pub pad2:[u32;8] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_launch_start { pub handle:u32,pub policy:u32,pub dh_uaddr:u64,pub dh_len:u32,pub pad0:u32,pub session_uaddr:u64,pub session_len:u32,pub pad1:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_launch_update_data { pub uaddr:u64,pub len:u32,pub pad0:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_launch_secret { pub hdr_uaddr:u64,pub hdr_len:u32,pub pad0:u32,pub guest_uaddr:u64,pub guest_len:u32,pub pad1:u32,pub trans_uaddr:u64,pub trans_len:u32,pub pad2:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_launch_measure { pub uaddr:u64,pub len:u32,pub pad0:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_guest_status { pub handle:u32,pub policy:u32,pub state:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_dbg { pub src_uaddr:u64,pub dst_uaddr:u64,pub len:u32,pub pad0:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_attestation_report { pub mnonce:[u8;16],pub uaddr:u64,pub len:u32,pub pad0:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_send_start { pub policy:u32,pub pad0:u32,pub pdh_cert_uaddr:u64,pub pdh_cert_len:u32,pub pad1:u32,pub plat_certs_uaddr:u64,pub plat_certs_len:u32,pub pad2:u32,pub amd_certs_uaddr:u64,pub amd_certs_len:u32,pub pad3:u32,pub session_uaddr:u64,pub session_len:u32,pub pad4:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_send_update_data { pub hdr_uaddr:u64,pub hdr_len:u32,pub pad0:u32,pub guest_uaddr:u64,pub guest_len:u32,pub pad1:u32,pub trans_uaddr:u64,pub trans_len:u32,pub pad2:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_receive_start { pub handle:u32,pub policy:u32,pub pdh_uaddr:u64,pub pdh_len:u32,pub pad0:u32,pub session_uaddr:u64,pub session_len:u32,pub pad1:u32 }
pub type kvm_sev_receive_update_data=kvm_sev_send_update_data;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_snp_launch_start { pub policy:u64,pub gosvw:[u8;16],pub flags:u16,pub pad0:[u8;6],pub pad1:[u64;4] }
pub const KVM_SEV_PAGE_TYPE_INVALID:u32=0; pub const KVM_SEV_SNP_PAGE_TYPE_NORMAL:u32=1; pub const KVM_SEV_SNP_PAGE_TYPE_ZERO:u32=3; pub const KVM_SEV_SNP_PAGE_TYPE_UNMEASURED:u32=4; pub const KVM_SEV_SNP_PAGE_TYPE_SECRETS:u32=5; pub const KVM_SEV_SNP_PAGE_TYPE_CPUID:u32=6;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_snp_launch_update { pub gfn_start:u64,pub uaddr:u64,pub len:u64,pub type_:u8,pub pad0:u8,pub flags:u16,pub pad1:u32,pub pad2:[u64;4] }
pub const KVM_SEV_SNP_ID_BLOCK_SIZE:usize=96; pub const KVM_SEV_SNP_ID_AUTH_SIZE:usize=4096; pub const KVM_SEV_SNP_FINISH_DATA_SIZE:usize=32;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_sev_snp_launch_finish { pub id_block_uaddr:u64,pub id_auth_uaddr:u64,pub id_block_en:u8,pub auth_key_en:u8,pub vcek_disabled:u8,pub host_data:[u8;32],pub pad0:[u8;3],pub flags:u16,pub pad1:[u64;4] }
pub const KVM_X2APIC_API_USE_32BIT_IDS:u64=1; pub const KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK:u64=2; pub const KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST:u64=4; pub const KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST:u64=8;
#[repr(C)] #[derive(Copy,Clone)] pub struct kvm_hyperv_eventfd { pub conn_id:u32,pub fd:i32,pub flags:u32,pub padding:[u32;3] }
pub const KVM_HYPERV_CONN_ID_MASK:u32=0x00ffffff; pub const KVM_HYPERV_EVENTFD_DEASSIGN:u32=1;
#[inline] pub const fn kvm_pmu_encode_masked_entry(event_select:u64,mask:u64,match_:u64,exclude:u64)->u64 { (event_select&0xff)|((event_select&0xf00)<<24)|((mask&0xff)<<56)|((match_&0xff)<<8)|((exclude!=0) as u64<<55) }
pub const KVM_PMU_MASKED_ENTRY_EVENT_SELECT:u64=((1u64<<8)-1)|(((1u64<<4)-1)<<32); pub const KVM_PMU_MASKED_ENTRY_UMASK_MASK:u64=0xffu64<<56; pub const KVM_PMU_MASKED_ENTRY_UMASK_MATCH:u64=0xffu64<<8; pub const KVM_PMU_MASKED_ENTRY_EXCLUDE:u64=1u64<<55;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
