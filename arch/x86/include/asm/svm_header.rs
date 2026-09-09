/* SPDX-License-Identifier: GPL-2.0 */
// Translated from svm.h. External types and constants are supplied by dependencies.

#[repr(u32)]
pub enum InterceptWords { InterceptCr = 0, InterceptDr, InterceptException, InterceptWord3, InterceptWord4, InterceptWord5, MaxIntercept }

pub const INTERCEPT_CR0_READ: u32 = 0;
pub const INTERCEPT_CR3_READ: u32 = 3;
pub const INTERCEPT_CR4_READ: u32 = 4;
pub const INTERCEPT_CR8_READ: u32 = 8;
pub const INTERCEPT_CR0_WRITE: u32 = 16;
pub const INTERCEPT_CR3_WRITE: u32 = 19;
pub const INTERCEPT_CR4_WRITE: u32 = 20;
pub const INTERCEPT_CR8_WRITE: u32 = 24;
pub const INTERCEPT_DR0_READ: u32 = 32;
pub const INTERCEPT_DR1_READ: u32 = 33;
pub const INTERCEPT_DR2_READ: u32 = 34;
pub const INTERCEPT_DR3_READ: u32 = 35;
pub const INTERCEPT_DR4_READ: u32 = 36;
pub const INTERCEPT_DR5_READ: u32 = 37;
pub const INTERCEPT_DR6_READ: u32 = 38;
pub const INTERCEPT_DR7_READ: u32 = 39;
pub const INTERCEPT_DR0_WRITE: u32 = 48;
pub const INTERCEPT_DR1_WRITE: u32 = 49;
pub const INTERCEPT_DR2_WRITE: u32 = 50;
pub const INTERCEPT_DR3_WRITE: u32 = 51;
pub const INTERCEPT_DR4_WRITE: u32 = 52;
pub const INTERCEPT_DR5_WRITE: u32 = 53;
pub const INTERCEPT_DR6_WRITE: u32 = 54;
pub const INTERCEPT_DR7_WRITE: u32 = 55;
pub const INTERCEPT_EXCEPTION_OFFSET: u32 = 64;
pub const INTERCEPT_INTR: u32 = 96;
pub const INTERCEPT_NMI: u32 = 97;
pub const INTERCEPT_SMI: u32 = 98;
pub const INTERCEPT_INIT: u32 = 99;
pub const INTERCEPT_VINTR: u32 = 100;
pub const INTERCEPT_SELECTIVE_CR0: u32 = 101;
pub const INTERCEPT_STORE_IDTR: u32 = 102;
pub const INTERCEPT_STORE_GDTR: u32 = 103;
pub const INTERCEPT_STORE_LDTR: u32 = 104;
pub const INTERCEPT_STORE_TR: u32 = 105;
pub const INTERCEPT_LOAD_IDTR: u32 = 106;
pub const INTERCEPT_LOAD_GDTR: u32 = 107;
pub const INTERCEPT_LOAD_LDTR: u32 = 108;
pub const INTERCEPT_LOAD_TR: u32 = 109;
pub const INTERCEPT_RDTSC: u32 = 110;
pub const INTERCEPT_RDPMC: u32 = 111;
pub const INTERCEPT_PUSHF: u32 = 112;
pub const INTERCEPT_POPF: u32 = 113;
pub const INTERCEPT_CPUID: u32 = 114;
pub const INTERCEPT_RSM: u32 = 115;
pub const INTERCEPT_IRET: u32 = 116;
pub const INTERCEPT_INTN: u32 = 117;
pub const INTERCEPT_INVD: u32 = 118;
pub const INTERCEPT_PAUSE: u32 = 119;
pub const INTERCEPT_HLT: u32 = 120;
pub const INTERCEPT_INVLPG: u32 = 121;
pub const INTERCEPT_INVLPGA: u32 = 122;
pub const INTERCEPT_IOIO_PROT: u32 = 123;
pub const INTERCEPT_MSR_PROT: u32 = 124;
pub const INTERCEPT_TASK_SWITCH: u32 = 125;
pub const INTERCEPT_FERR_FREEZE: u32 = 126;
pub const INTERCEPT_SHUTDOWN: u32 = 127;
pub const INTERCEPT_VMRUN: u32 = 128;
pub const INTERCEPT_VMMCALL: u32 = 129;
pub const INTERCEPT_VMLOAD: u32 = 130;
pub const INTERCEPT_VMSAVE: u32 = 131;
pub const INTERCEPT_STGI: u32 = 132;
pub const INTERCEPT_CLGI: u32 = 133;
pub const INTERCEPT_SKINIT: u32 = 134;
pub const INTERCEPT_RDTSCP: u32 = 135;
pub const INTERCEPT_ICEBP: u32 = 136;
pub const INTERCEPT_WBINVD: u32 = 137;
pub const INTERCEPT_MONITOR: u32 = 138;
pub const INTERCEPT_MWAIT: u32 = 139;
pub const INTERCEPT_MWAIT_COND: u32 = 140;
pub const INTERCEPT_XSETBV: u32 = 141;
pub const INTERCEPT_RDPRU: u32 = 142;
pub const TRAP_EFER_WRITE: u32 = 143;
pub const TRAP_CR0_WRITE: u32 = 144;
pub const TRAP_CR1_WRITE: u32 = 145;
pub const TRAP_CR2_WRITE: u32 = 146;
pub const TRAP_CR3_WRITE: u32 = 147;
pub const TRAP_CR4_WRITE: u32 = 148;
pub const TRAP_CR5_WRITE: u32 = 149;
pub const TRAP_CR6_WRITE: u32 = 150;
pub const TRAP_CR7_WRITE: u32 = 151;
pub const TRAP_CR8_WRITE: u32 = 152;
pub const INTERCEPT_INVLPGB: u32 = 160;
pub const INTERCEPT_INVLPGB_ILLEGAL: u32 = 161;
pub const INTERCEPT_INVPCID: u32 = 162;
pub const INTERCEPT_MCOMMIT: u32 = 163;
pub const INTERCEPT_TLBSYNC: u32 = 164;
pub const INTERCEPT_BUSLOCK: u32 = 165;
pub const INTERCEPT_IDLE_HLT: u32 = 166;

#[repr(C, packed)]
pub struct VmcbControlArea {
    pub intercepts: [u32; 6], pub reserved_1: [u32; 9], pub pause_filter_thresh: u16, pub pause_filter_count: u16,
    pub iopm_base_pa: u64, pub msrpm_base_pa: u64, pub tsc_offset: u64, pub asid: u32, pub tlb_ctl: u8, pub erap_ctl: u8,
    pub reserved_2: [u8; 2], pub int_ctl: u32, pub int_vector: u32, pub int_state: u32, pub reserved_3: [u8; 4],
    pub exit_code: u64, pub exit_info_1: u64, pub exit_info_2: u64, pub exit_int_info: u32, pub exit_int_info_err: u32,
    pub misc_ctl: u64, pub avic_vapic_bar: u64, pub ghcb_gpa: u64, pub event_inj: u32, pub event_inj_err: u32,
    pub nested_cr3: u64, pub misc_ctl2: u64, pub clean: u32, pub reserved_5: u32, pub next_rip: u64, pub insn_len: u8,
    pub insn_bytes: [u8; 15], pub avic_backing_page: u64, pub reserved_6: [u8; 8], pub avic_logical_id: u64,
    pub avic_physical_id: u64, pub reserved_7: [u8; 8], pub vmsa_pa: u64, pub reserved_8: [u8; 16], pub bus_lock_counter: u16,
    pub reserved_9: [u8; 22], pub allowed_sev_features: u64, pub guest_sev_features: u64, pub reserved_10: [u8; 664],
    pub hv_enlightenments: HvVmcbEnlightenments,
}

// The union's alternate byte array is layout-equivalent; the external type must have size 32.
#[repr(C, packed)] pub struct HvVmcbEnlightenments { pub bytes: [u8; 32] }

macro_rules! bit_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u64 = $v;) * }; }
bit_consts! {
    TLB_CONTROL_DO_NOTHING=0, TLB_CONTROL_FLUSH_ALL_ASID=1, TLB_CONTROL_FLUSH_ASID=3, TLB_CONTROL_FLUSH_ASID_LOCAL=7,
    TLB_CONTROL_MASK=7, ERAP_CONTROL_ALLOW_LARGER_RAP=1, ERAP_CONTROL_CLEAR_RAP=2, V_TPR_MASK=0x0f,
    V_IRQ_SHIFT=8, V_IRQ_MASK=1<<8, V_GIF_SHIFT=9, V_GIF_MASK=1<<9, V_NMI_PENDING_SHIFT=11, V_NMI_PENDING_MASK=1<<11,
    V_NMI_BLOCKING_SHIFT=12, V_NMI_BLOCKING_MASK=1<<12, V_INTR_PRIO_SHIFT=16, V_INTR_PRIO_MASK=0x0f<<16,
    V_IGN_TPR_SHIFT=20, V_IGN_TPR_MASK=1<<20, V_IRQ_INJECTION_BITS_MASK=(1<<8)|(0x0f<<16)|(1<<20),
    V_INTR_MASKING_SHIFT=24, V_INTR_MASKING_MASK=1<<24, V_GIF_ENABLE_SHIFT=25, V_GIF_ENABLE_MASK=1<<25,
    V_NMI_ENABLE_SHIFT=26, V_NMI_ENABLE_MASK=1<<26, AVIC_ENABLE_SHIFT=31, AVIC_ENABLE_MASK=1<<31,
    X2APIC_MODE_SHIFT=30, X2APIC_MODE_MASK=1<<30, SVM_INT_VECTOR_MASK=0xff, SVM_INTERRUPT_SHADOW_MASK=1,
    SVM_GUEST_INTERRUPT_MASK=2, SVM_IOIO_STR_SHIFT=2, SVM_IOIO_REP_SHIFT=3, SVM_IOIO_SIZE_SHIFT=4, SVM_IOIO_ASIZE_SHIFT=7,
    SVM_IOIO_TYPE_MASK=1, SVM_IOIO_STR_MASK=1<<2, SVM_IOIO_REP_MASK=1<<3, SVM_IOIO_SIZE_MASK=7<<4, SVM_IOIO_ASIZE_MASK=7<<7,
    SVM_MISC_ENABLE_NP=1, SVM_MISC_ENABLE_SEV=2, SVM_MISC_ENABLE_SEV_ES=4, SVM_MISC_ENABLE_GMET=8,
    SVM_MISC2_ENABLE_V_LBR=1, SVM_MISC2_ENABLE_V_VMLOAD_VMSAVE=2, SVM_TSC_RATIO_RSVD=0xffffff0000000000, SVM_TSC_RATIO_MIN=1,
    SVM_TSC_RATIO_MAX=0x000000ffffffffff, SVM_TSC_RATIO_DEFAULT=0x0100000000, AVIC_LOGICAL_ID_ENTRY_GUEST_PHYSICAL_ID_MASK=0xff,
    AVIC_LOGICAL_ID_ENTRY_VALID_BIT=31, AVIC_LOGICAL_ID_ENTRY_VALID_MASK=1<<31, AVIC_PHYSICAL_ID_ENTRY_GA_LOG_INTR=1<<61,
    AVIC_PHYSICAL_ID_ENTRY_HOST_PHYSICAL_ID_MASK=0xfff, AVIC_PHYSICAL_ID_ENTRY_BACKING_PAGE_MASK=0xffffffffff000,
    AVIC_PHYSICAL_ID_ENTRY_IS_RUNNING_MASK=1<<62, AVIC_PHYSICAL_ID_ENTRY_VALID_MASK=1<<63, AVIC_PHYSICAL_ID_TABLE_SIZE_MASK=0xff,
    AVIC_DOORBELL_PHYSICAL_ID_MASK=0xfff, AVIC_UNACCEL_ACCESS_WRITE_MASK=1, AVIC_UNACCEL_ACCESS_OFFSET_MASK=0xff0,
    AVIC_UNACCEL_ACCESS_VECTOR_MASK=0xffffffff, AVIC_PHYSICAL_MAX_INDEX_MASK=0xfff, AVIC_MAX_PHYSICAL_ID=0xfe,
    X2AVIC_MAX_PHYSICAL_ID=0x1ff, X2AVIC_4K_MAX_PHYSICAL_ID=0xfff, SVM_SEV_FEAT_SNP_ACTIVE=1,
    SVM_SEV_FEAT_RESTRICTED_INJECTION=1<<3, SVM_SEV_FEAT_ALTERNATE_INJECTION=1<<4, SVM_SEV_FEAT_DEBUG_SWAP=1<<5,
    SVM_SEV_FEAT_SECURE_TSC=1<<9, VMCB_ALLOWED_SEV_FEATURES_VALID=1<<63
}

#[repr(u32)] pub enum AvicIpiFailureCause { InvalidIntType, TargetNotRunning, InvalidTarget, InvalidBackingPage, InvalidIpiVector }
#[repr(C, packed)] pub struct VmcbSeg { pub selector:u16, pub attrib:u16, pub limit:u32, pub base:u64 }

#[repr(C, packed)] pub struct VmcbSaveArea {
 pub es:VmcbSeg,pub cs:VmcbSeg,pub ss:VmcbSeg,pub ds:VmcbSeg,pub fs:VmcbSeg,pub gs:VmcbSeg,pub gdtr:VmcbSeg,pub ldtr:VmcbSeg,pub idtr:VmcbSeg,pub tr:VmcbSeg,
 pub reserved_0xa0:[u8;42],pub vmpl:u8,pub cpl:u8,pub reserved_0xcc:[u8;4],pub efer:u64,pub reserved_0xd8:[u8;112],pub cr4:u64,pub cr3:u64,pub cr0:u64,pub dr7:u64,pub dr6:u64,pub rflags:u64,pub rip:u64,pub reserved_0x180:[u8;88],pub rsp:u64,pub s_cet:u64,pub ssp:u64,pub isst_addr:u64,pub rax:u64,pub star:u64,pub lstar:u64,pub cstar:u64,pub sfmask:u64,pub kernel_gs_base:u64,pub sysenter_cs:u64,pub sysenter_esp:u64,pub sysenter_eip:u64,pub cr2:u64,pub reserved_0x248:[u8;32],pub g_pat:u64,pub dbgctl:u64,pub br_from:u64,pub br_to:u64,pub last_excp_from:u64,pub last_excp_to:u64,pub reserved_0x298:[u8;72],pub spec_ctrl:u64
}

#[repr(C, packed)] pub struct SevEsSaveArea {
 pub es:VmcbSeg,pub cs:VmcbSeg,pub ss:VmcbSeg,pub ds:VmcbSeg,pub fs:VmcbSeg,pub gs:VmcbSeg,pub gdtr:VmcbSeg,pub ldtr:VmcbSeg,pub idtr:VmcbSeg,pub tr:VmcbSeg,pub pl0_ssp:u64,pub pl1_ssp:u64,pub pl2_ssp:u64,pub pl3_ssp:u64,pub u_cet:u64,pub reserved_0xc8:[u8;2],pub vmpl:u8,pub cpl:u8,pub reserved_0xcc:[u8;4],pub efer:u64,pub reserved_0xd8:[u8;104],pub xss:u64,pub cr4:u64,pub cr3:u64,pub cr0:u64,pub dr7:u64,pub dr6:u64,pub rflags:u64,pub rip:u64,pub dr0:u64,pub dr1:u64,pub dr2:u64,pub dr3:u64,pub dr0_addr_mask:u64,pub dr1_addr_mask:u64,pub dr2_addr_mask:u64,pub dr3_addr_mask:u64,pub reserved_0x1c0:[u8;24],pub rsp:u64,pub s_cet:u64,pub ssp:u64,pub isst_addr:u64,pub rax:u64,pub star:u64,pub lstar:u64,pub cstar:u64,pub sfmask:u64,pub kernel_gs_base:u64,pub sysenter_cs:u64,pub sysenter_esp:u64,pub sysenter_eip:u64,pub cr2:u64,pub reserved_0x248:[u8;32],pub g_pat:u64,pub dbgctl:u64,pub br_from:u64,pub br_to:u64,pub last_excp_from:u64,pub last_excp_to:u64,pub reserved_0x298:[u8;80],pub pkru:u32,pub tsc_aux:u32,pub tsc_scale:u64,pub tsc_offset:u64,pub reserved_0x300:[u8;8],pub rcx:u64,pub rdx:u64,pub rbx:u64,pub reserved_0x320:u64,pub rbp:u64,pub rsi:u64,pub rdi:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64,pub reserved_0x380:[u8;16],pub guest_exit_info_1:u64,pub guest_exit_info_2:u64,pub guest_exit_int_info:u64,pub guest_nrip:u64,pub sev_features:u64,pub vintr_ctrl:u64,pub guest_exit_code:u64,pub virtual_tom:u64,pub tlb_id:u64,pub pcpu_id:u64,pub event_inj:u64,pub xcr0:u64,pub reserved_0x3f0:[u8;16],pub x87_dp:u64,pub mxcsr:u32,pub x87_ftw:u16,pub x87_fsw:u16,pub x87_fcw:u16,pub x87_fop:u16,pub x87_ds:u16,pub x87_cs:u16,pub x87_rip:u64,pub fpreg_x87:[u8;80],pub fpreg_xmm:[u8;256],pub fpreg_ymm:[u8;256]
}

#[repr(C, packed)] pub struct GhcbSaveArea { pub reserved_0x0:[u8;203],pub cpl:u8,pub reserved_0xcc:[u8;116],pub xss:u64,pub reserved_0x148:[u8;24],pub dr7:u64,pub reserved_0x168:[u8;16],pub rip:u64,pub reserved_0x180:[u8;88],pub rsp:u64,pub reserved_0x1e0:[u8;24],pub rax:u64,pub reserved_0x200:[u8;264],pub rcx:u64,pub rdx:u64,pub rbx:u64,pub reserved_0x320:[u8;8],pub rbp:u64,pub rsi:u64,pub rdi:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64,pub reserved_0x380:[u8;16],pub sw_exit_code:u64,pub sw_exit_info_1:u64,pub sw_exit_info_2:u64,pub sw_scratch:u64,pub reserved_0x3b0:[u8;56],pub xcr0:u64,pub valid_bitmap:[u8;16],pub x87_state_gpa:u64 }
pub const GHCB_SHARED_BUF_SIZE: usize = 2032;
#[repr(C, packed)] pub struct Ghcb { pub save:GhcbSaveArea,pub reserved_save:[u8;2048-1032],pub shared_buffer:[u8;GHCB_SHARED_BUF_SIZE],pub reserved_0xff0:[u8;10],pub protocol_version:u16,pub ghcb_usage:u32 }
#[repr(C, packed)] pub union VmcbSave { pub save:VmcbSaveArea,pub host_sev_es_save:SevEsSaveArea }
#[repr(C, packed)] pub struct Vmcb { pub control:VmcbControlArea,pub save:VmcbSave }

pub const EXPECTED_VMCB_SAVE_AREA_SIZE:usize=744; pub const EXPECTED_GHCB_SAVE_AREA_SIZE:usize=1032; pub const EXPECTED_SEV_ES_SAVE_AREA_SIZE:usize=1648; pub const EXPECTED_VMCB_CONTROL_AREA_SIZE:usize=1024; pub const EXPECTED_GHCB_SIZE:usize=4096;
pub const SVM_CPUID_FUNC:u32=0x8000000a;
pub const SVM_SELECTOR_S_SHIFT:u64=4; pub const SVM_SELECTOR_DPL_SHIFT:u64=5; pub const SVM_SELECTOR_P_SHIFT:u64=7; pub const SVM_SELECTOR_AVL_SHIFT:u64=8; pub const SVM_SELECTOR_L_SHIFT:u64=9; pub const SVM_SELECTOR_DB_SHIFT:u64=10; pub const SVM_SELECTOR_G_SHIFT:u64=11;
pub const SVM_SELECTOR_TYPE_MASK:u64=0xf; pub const SVM_SELECTOR_S_MASK:u64=1<<4; pub const SVM_SELECTOR_DPL_MASK:u64=3<<5; pub const SVM_SELECTOR_P_MASK:u64=1<<7; pub const SVM_SELECTOR_AVL_MASK:u64=1<<8; pub const SVM_SELECTOR_L_MASK:u64=1<<9; pub const SVM_SELECTOR_DB_MASK:u64=1<<10; pub const SVM_SELECTOR_G_MASK:u64=1<<11; pub const SVM_SELECTOR_WRITE_MASK:u64=1<<1; pub const SVM_SELECTOR_READ_MASK:u64=1<<1; pub const SVM_SELECTOR_CODE_MASK:u64=1<<3;
pub const SVM_EVTINJ_VEC_MASK:u64=0xff; pub const SVM_EVTINJ_TYPE_SHIFT:u64=8; pub const SVM_EVTINJ_TYPE_MASK:u64=7<<8; pub const SVM_EVTINJ_TYPE_INTR:u64=0; pub const SVM_EVTINJ_TYPE_NMI:u64=2<<8; pub const SVM_EVTINJ_TYPE_EXEPT:u64=3<<8; pub const SVM_EVTINJ_TYPE_SOFT:u64=4<<8; pub const SVM_EVTINJ_VALID:u64=1<<31; pub const SVM_EVTINJ_VALID_ERR:u64=1<<11; pub const SVM_EVTINJ_RESERVED_BITS:u64=!(0xff|(7<<8)|(1<<11)|(1<<31));
pub const SVM_EXITINTINFO_VEC_MASK:u64=0xff; pub const SVM_EXITINTINFO_TYPE_MASK:u64=7<<8; pub const SVM_EXITINTINFO_TYPE_INTR:u64=0; pub const SVM_EXITINTINFO_TYPE_NMI:u64=2<<8; pub const SVM_EXITINTINFO_TYPE_EXEPT:u64=3<<8; pub const SVM_EXITINTINFO_TYPE_SOFT:u64=4<<8; pub const SVM_EXITINTINFO_VALID:u64=1<<31; pub const SVM_EXITINTINFO_VALID_ERR:u64=1<<11; pub const SVM_EXITINFOSHIFT_TS_REASON_IRET:u64=36; pub const SVM_EXITINFOSHIFT_TS_REASON_JMP:u64=38; pub const SVM_EXITINFOSHIFT_TS_HAS_ERROR_CODE:u64=44; pub const SVM_EXITINFO_REG_MASK:u64=0x0f;

// GHCB accessors preserve the C validity bitmap protocol. Field offsets are supplied by repr(C, packed) layout.
pub unsafe fn ghcb_field_is_valid(ghcb: *const Ghcb, bitmap_index: usize) -> bool {
    let bitmap = core::ptr::addr_of!((*ghcb).save.valid_bitmap) as *const u8;
    ((*bitmap.add(bitmap_index / 8)) & (1u8 << (bitmap_index % 8))) != 0
}
pub unsafe fn ghcb_set_field_valid(ghcb: *mut Ghcb, bitmap_index: usize) {
    let bitmap = core::ptr::addr_of_mut!((*ghcb).save.valid_bitmap) as *mut u8;
    *bitmap.add(bitmap_index / 8) |= 1u8 << (bitmap_index % 8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
