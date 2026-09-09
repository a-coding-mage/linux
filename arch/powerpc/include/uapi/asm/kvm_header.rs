/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright IBM Corp. 2007 */

// Translated from the PowerPC Linux UAPI KVM header.
// __KVM_HAVE_SPAPR_TCE, __KVM_HAVE_PPC_SMT, __KVM_HAVE_IRQCHIP and
// __KVM_HAVE_IRQ_LINE are build-time feature markers.

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 1;

#[repr(C)]
pub struct kvm_regs { pub pc:u64, pub cr:u64, pub ctr:u64, pub lr:u64, pub xer:u64, pub msr:u64, pub srr0:u64, pub srr1:u64, pub pid:u64, pub sprg0:u64, pub sprg1:u64, pub sprg2:u64, pub sprg3:u64, pub sprg4:u64, pub sprg5:u64, pub sprg6:u64, pub sprg7:u64, pub gpr:[u64;32] }

pub const KVM_SREGS_E_IMPL_NONE:u32=0; pub const KVM_SREGS_E_IMPL_FSL:u32=1;
pub const KVM_SREGS_E_FSL_PIDn:u32=1<<0;
pub const KVM_RUN_PPC_NMI_DISP_MASK:u32=3<<0; pub const KVM_RUN_PPC_NMI_DISP_FULLY_RECOV:u32=1<<0; pub const KVM_RUN_PPC_NMI_DISP_LIMITED_RECOV:u32=2<<0; pub const KVM_RUN_PPC_NMI_DISP_NOT_RECOV:u32=3<<0;
pub const KVM_SREGS_E_BASE:u32=1<<0; pub const KVM_SREGS_E_ARCH206:u32=1<<1; pub const KVM_SREGS_E_64:u32=1<<2; pub const KVM_SREGS_E_SPRG8:u32=1<<3; pub const KVM_SREGS_E_MCIVPR:u32=1<<4; pub const KVM_SREGS_E_IVOR:u32=1<<5; pub const KVM_SREGS_E_ARCH206_MMU:u32=1<<6; pub const KVM_SREGS_E_DEBUG:u32=1<<7; pub const KVM_SREGS_E_ED:u32=1<<8; pub const KVM_SREGS_E_SPE:u32=1<<9; pub const KVM_SREGS_EXP:u32=1<<10; pub const KVM_SREGS_E_PD:u32=1<<11; pub const KVM_SREGS_E_PC:u32=1<<12; pub const KVM_SREGS_E_PT:u32=1<<13; pub const KVM_SREGS_E_PM:u32=1<<14;
pub const KVM_SREGS_E_UPDATE_MCSR:u32=1<<0; pub const KVM_SREGS_E_UPDATE_TSR:u32=1<<1; pub const KVM_SREGS_E_UPDATE_DEC:u32=1<<2; pub const KVM_SREGS_E_UPDATE_DBSR:u32=1<<3;

#[repr(C)] pub struct kvm_sregs_slb { pub slbe:u64, pub slbv:u64 }
#[repr(C)] pub struct kvm_sregs_ppc64 { pub slb:[kvm_sregs_slb;64] }
#[repr(C)] pub struct kvm_sregs_ppc32 { pub sr:[u32;16], pub ibat:[u64;8], pub dbat:[u64;8] }
#[repr(C)] pub struct kvm_sregs_s { pub sdr1:u64, pub ppc64:kvm_sregs_ppc64, pub ppc32:kvm_sregs_ppc32 }
#[repr(C)] pub struct kvm_sregs_fsl { pub features:u32, pub svr:u32, pub mcar:u64, pub hid0:u32, pub pid1:u32, pub pid2:u32 }
#[repr(C)] pub union kvm_sregs_impl { pub fsl:kvm_sregs_fsl, pub pad:[u8;256] }
#[repr(C)] pub struct kvm_sregs_e {
 pub impl_:kvm_sregs_impl, pub features:u32, pub impl_id:u32, pub update_special:u32, pub pir:u32, pub sprg8:u64, pub sprg9:u64, pub csrr0:u64, pub dsrr0:u64, pub mcsrr0:u64, pub csrr1:u32, pub dsrr1:u32, pub mcsrr1:u32, pub esr:u32, pub dear:u64, pub ivpr:u64, pub mcivpr:u64, pub mcsr:u64, pub tsr:u32, pub tcr:u32, pub decar:u32, pub dec:u32, pub tb:u64, pub dbsr:u32, pub dbcr:[u32;3], pub iac:[u32;4], pub dac:[u32;2], pub dvc:[u32;2], pub num_iac:u8, pub num_dac:u8, pub num_dvc:u8, pub pad:u8, pub epr:u32, pub vrsave:u32, pub epcr:u32, pub mas0:u32, pub mas1:u32, pub mas2:u64, pub mas7_3:u64, pub mas4:u32, pub mas6:u32, pub ivor_low:[u32;16], pub ivor_high:[u32;18], pub mmucfg:u32, pub eptcfg:u32, pub tlbcfg:[u32;4], pub tlbps:[u32;4], pub eplc:u32, pub epsc:u32 }
#[repr(C)] pub union kvm_sregs_u { pub s:kvm_sregs_s, pub e:kvm_sregs_e, pub pad:[u8;1020] }
#[repr(C)] pub struct kvm_sregs { pub pvr:u32, pub u:kvm_sregs_u }
#[repr(C)] pub struct kvm_fpu { pub fpr:[u64;32] }

pub const KVMPPC_DEBUG_NONE:u32=0x0; pub const KVMPPC_DEBUG_BREAKPOINT:u32=1<<1; pub const KVMPPC_DEBUG_WATCH_WRITE:u32=1<<2; pub const KVMPPC_DEBUG_WATCH_READ:u32=1<<3;
#[repr(C)] pub struct kvm_debug_exit_arch { pub address:u64, pub status:u32, pub reserved:u32 }
#[repr(C)] pub struct kvm_guest_debug_bp { pub addr:u64, pub type_:u32, pub reserved:u32 }
#[repr(C)] pub struct kvm_guest_debug_arch { pub bp:[kvm_guest_debug_bp;16] }
pub const KVM_GUESTDBG_USE_SW_BP:u32=0x00010000; pub const KVM_GUESTDBG_USE_HW_BP:u32=0x00020000;
#[repr(C)] pub struct kvm_sync_regs {}
pub const KVM_INTERRUPT_SET:u32=!0u32-0; pub const KVM_INTERRUPT_UNSET:u32=!0u32-1; pub const KVM_INTERRUPT_SET_LEVEL:u32=!0u32-2;
pub const KVM_CPU_440:u32=1; pub const KVM_CPU_E500V2:u32=2; pub const KVM_CPU_3S_32:u32=3; pub const KVM_CPU_3S_64:u32=4; pub const KVM_CPU_E500MC:u32=5;
#[repr(C)] pub struct kvm_create_spapr_tce { pub liobn:u64, pub window_size:u32 }
#[repr(C)] pub struct kvm_create_spapr_tce_64 { pub liobn:u64, pub page_shift:u32, pub flags:u32, pub offset:u64, pub size:u64 }
#[repr(C)] pub struct kvm_allocate_rma { pub rma_size:u64 }
#[repr(C)] pub struct kvm_rtas_token_args { pub name:[i8;120], pub token:u64 }
#[repr(C)] pub struct kvm_book3e_206_tlb_entry { pub mas8:u32, pub mas1:u32, pub mas2:u64, pub mas7_3:u64 }
#[repr(C)] pub struct kvm_book3e_206_tlb_params { pub tlb_sizes:[u32;4], pub tlb_ways:[u32;4], pub reserved:[u32;8] }
#[repr(C)] pub struct kvm_get_htab_fd { pub flags:u64, pub start_index:u64, pub reserved:[u64;2] }
pub const KVM_GET_HTAB_BOLTED_ONLY:u64=0x1; pub const KVM_GET_HTAB_WRITE:u64=0x2;
#[repr(C)] pub struct kvm_get_htab_header { pub index:u32, pub n_valid:u16, pub n_invalid:u16 }
#[repr(C)] pub struct kvm_ppc_mmuv3_cfg { pub flags:u64, pub process_table:u64 }
pub const KVM_PPC_MMUV3_RADIX:u64=1; pub const KVM_PPC_MMUV3_GTSE:u64=2;
#[repr(C)] pub struct kvm_ppc_radix_geom { pub page_shift:u8, pub level_bits:[u8;4], pub pad:[u8;3] }
#[repr(C)] pub struct kvm_ppc_rmmu_info { pub geometries:[kvm_ppc_radix_geom;8], pub ap_encodings:[u32;8] }
#[repr(C)] pub struct kvm_ppc_cpu_char { pub character:u64, pub behaviour:u64, pub character_mask:u64, pub behaviour_mask:u64 }
#[repr(C)] pub struct kvm_ppc_compat_caps { pub size:u64, pub flags:u64, pub compat_capabilities:u64 }
pub const KVM_PPC_COMPAT_CAPS_SIZE_VER0:u32=24; pub const KVM_PPC_COMPAT_CAP_POWER9:u64=1<<62; pub const KVM_PPC_COMPAT_CAP_POWER10:u64=1<<61; pub const KVM_PPC_COMPAT_CAP_POWER11:u64=1<<60; pub const KVM_PPC_COMPAT_BITMASK:u64=KVM_PPC_COMPAT_CAP_POWER9|KVM_PPC_COMPAT_CAP_POWER10|KVM_PPC_COMPAT_CAP_POWER11;
pub const KVM_PPC_CPU_CHAR_SPEC_BAR_ORI31:u64=1<<63; pub const KVM_PPC_CPU_CHAR_BCCTRL_SERIALISED:u64=1<<62; pub const KVM_PPC_CPU_CHAR_L1D_FLUSH_ORI30:u64=1<<61; pub const KVM_PPC_CPU_CHAR_L1D_FLUSH_TRIG2:u64=1<<60; pub const KVM_PPC_CPU_CHAR_L1D_THREAD_PRIV:u64=1<<59; pub const KVM_PPC_CPU_CHAR_BR_HINT_HONOURED:u64=1<<58; pub const KVM_PPC_CPU_CHAR_MTTRIG_THR_RECONF:u64=1<<57; pub const KVM_PPC_CPU_CHAR_COUNT_CACHE_DIS:u64=1<<56; pub const KVM_PPC_CPU_CHAR_BCCTR_FLUSH_ASSIST:u64=1<<54;
pub const KVM_PPC_CPU_BEHAV_FAVOUR_SECURITY:u64=1<<63; pub const KVM_PPC_CPU_BEHAV_L1D_FLUSH_PR:u64=1<<62; pub const KVM_PPC_CPU_BEHAV_BNDS_CHK_SPEC_BAR:u64=1<<61; pub const KVM_PPC_CPU_BEHAV_FLUSH_COUNT_CACHE:u64=1<<58;

// KVM_REG_PPC, KVM_REG_SIZE_U32/U64/U128 are supplied by linux/kvm.h.
pub const KVM_REG_PPC_ICP_STATE:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x8c;
pub const KVM_REG_PPC_ICP_CPPR_SHIFT:u32=56; pub const KVM_REG_PPC_ICP_CPPR_MASK:u32=0xff; pub const KVM_REG_PPC_ICP_XISR_SHIFT:u32=32; pub const KVM_REG_PPC_ICP_XISR_MASK:u32=0xffffff; pub const KVM_REG_PPC_ICP_MFRR_SHIFT:u32=24; pub const KVM_REG_PPC_ICP_MFRR_MASK:u32=0xff; pub const KVM_REG_PPC_ICP_PPRI_SHIFT:u32=16; pub const KVM_REG_PPC_ICP_PPRI_MASK:u32=0xff;
pub const KVM_REG_PPC_VP_STATE:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x8d;
pub const KVM_DEV_MPIC_GRP_MISC:u32=1; pub const KVM_DEV_MPIC_BASE_ADDR:u32=0; pub const KVM_DEV_MPIC_GRP_REGISTER:u32=2; pub const KVM_DEV_MPIC_GRP_IRQ_ACTIVE:u32=3;

macro_rules! ppc_reg { ($name:ident,$size:ident,$n:expr) => { pub const $name:u64=KVM_REG_PPC|KVM_REG_SIZE_$size|$n; }; }
ppc_reg!(KVM_REG_PPC_HIOR,U64,0x1); ppc_reg!(KVM_REG_PPC_IAC1,U64,0x2); ppc_reg!(KVM_REG_PPC_IAC2,U64,0x3); ppc_reg!(KVM_REG_PPC_IAC3,U64,0x4); ppc_reg!(KVM_REG_PPC_IAC4,U64,0x5); ppc_reg!(KVM_REG_PPC_DAC1,U64,0x6); ppc_reg!(KVM_REG_PPC_DAC2,U64,0x7); ppc_reg!(KVM_REG_PPC_DABR,U64,0x8); ppc_reg!(KVM_REG_PPC_DSCR,U64,0x9); ppc_reg!(KVM_REG_PPC_PURR,U64,0xa); ppc_reg!(KVM_REG_PPC_SPURR,U64,0xb); ppc_reg!(KVM_REG_PPC_DAR,U64,0xc); ppc_reg!(KVM_REG_PPC_DSISR,U32,0xd); ppc_reg!(KVM_REG_PPC_AMR,U64,0xe); ppc_reg!(KVM_REG_PPC_UAMOR,U64,0xf);
ppc_reg!(KVM_REG_PPC_MMCR0,U64,0x10); ppc_reg!(KVM_REG_PPC_MMCR1,U64,0x11); ppc_reg!(KVM_REG_PPC_MMCRA,U64,0x12); ppc_reg!(KVM_REG_PPC_MMCR2,U64,0x13); ppc_reg!(KVM_REG_PPC_MMCRS,U64,0x14); ppc_reg!(KVM_REG_PPC_SIAR,U64,0x15); ppc_reg!(KVM_REG_PPC_SDAR,U64,0x16); ppc_reg!(KVM_REG_PPC_SIER,U64,0x17);
// Parameterized C macros retained as Rust macros; token-pasting sequences are represented explicitly below.
pub const KVM_REG_PPC_PMC1:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x18; pub const KVM_REG_PPC_PMC2:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x19; pub const KVM_REG_PPC_PMC3:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1a; pub const KVM_REG_PPC_PMC4:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1b; pub const KVM_REG_PPC_PMC5:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1c; pub const KVM_REG_PPC_PMC6:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1d; pub const KVM_REG_PPC_PMC7:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1e; pub const KVM_REG_PPC_PMC8:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x1f;
pub const KVM_REG_PPC_FPR0:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x20; pub const KVM_REG_PPC_FPR31:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x3f; pub const KVM_REG_PPC_VR0:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x40; pub const KVM_REG_PPC_VR31:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x5f; pub const KVM_REG_PPC_VSR0:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x60; pub const KVM_REG_PPC_VSR31:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x7f;
#[macro_export] macro_rules! KVM_REG_PPC_FPR { ($n:expr)=>{ $crate::KVM_REG_PPC_FPR0 + ($n as u64) }; } #[macro_export] macro_rules! KVM_REG_PPC_VR { ($n:expr)=>{ $crate::KVM_REG_PPC_VR0 + ($n as u64) }; } #[macro_export] macro_rules! KVM_REG_PPC_VSR { ($n:expr)=>{ $crate::KVM_REG_PPC_VSR0 + ($n as u64) }; }
pub const KVM_REG_PPC_FPSCR:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x80; pub const KVM_REG_PPC_VSCR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x81; pub const KVM_REG_PPC_VPA_ADDR:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x82; pub const KVM_REG_PPC_VPA_SLB:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x83; pub const KVM_REG_PPC_VPA_DTL:u64=KVM_REG_PPC|KVM_REG_SIZE_U128|0x84;
pub const KVM_REG_PPC_EPCR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x85; pub const KVM_REG_PPC_EPR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x86; pub const KVM_REG_PPC_OR_TSR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x87; pub const KVM_REG_PPC_CLEAR_TSR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x88; pub const KVM_REG_PPC_TCR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x89; pub const KVM_REG_PPC_TSR:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x8a; pub const KVM_REG_PPC_DEBUG_INST:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x8b; pub const KVM_REG_PPC_MAS0:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x8c; pub const KVM_REG_PPC_MAS1:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x8d; pub const KVM_REG_PPC_MAS2:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x8e; pub const KVM_REG_PPC_MAS7_3:u64=KVM_REG_PPC|KVM_REG_SIZE_U64|0x8f; pub const KVM_REG_PPC_MAS4:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x90; pub const KVM_REG_PPC_MAS6:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x91; pub const KVM_REG_PPC_MMUCFG:u64=KVM_REG_PPC|KVM_REG_SIZE_U32|0x92;
pub const KVM_REG_PPC_TM:u64=KVM_REG_PPC|0x80000000;

#[repr(C)] pub struct kvm_ppc_xive_eq { pub flags:u32, pub qshift:u32, pub qaddr:u64, pub qtoggle:u32, pub qindex:u32, pub pad:[u8;40] }
pub const KVM_XIVE_EQ_ALWAYS_NOTIFY:u32=1; pub const KVM_XIVE_TIMA_PAGE_OFFSET:u32=0; pub const KVM_XIVE_ESB_PAGE_OFFSET:u32=4; pub const KVM_PPC_PVINFO_FLAGS_EV_IDLE:u32=1<<0;
#[repr(C)] pub struct kvm_ppc_pvinfo { pub flags:u32, pub hcall:[u32;4], pub pad:[u8;108] }
pub const KVM_PPC_PAGE_SIZES_MAX_SZ:usize=8;
#[repr(C)] pub struct kvm_ppc_one_page_size { pub page_shift:u32, pub pte_enc:u32 }
#[repr(C)] pub struct kvm_ppc_one_seg_page_size { pub page_shift:u32, pub slb_enc:u32, pub enc:[kvm_ppc_one_page_size;8] }
pub const KVM_PPC_PAGE_SIZES_REAL:u32=1; pub const KVM_PPC_1T_SEGMENTS:u32=2; pub const KVM_PPC_NO_HASH:u32=4;
#[repr(C)] pub struct kvm_ppc_smmu_info { pub flags:u64, pub slb_size:u32, pub data_keys:u16, pub instr_keys:u16, pub sps:[kvm_ppc_one_seg_page_size;8] }
#[repr(C)] pub struct kvm_ppc_resize_hpt { pub flags:u64, pub shift:u32, pub pad:u32 }
pub const KVM_DEV_XICS_GRP_SOURCES:u32=1; pub const KVM_DEV_XICS_GRP_CTRL:u32=2; pub const KVM_DEV_XICS_NR_SERVERS:u32=1; pub const KVM_XICS_DESTINATION_SHIFT:u32=0; pub const KVM_XICS_DESTINATION_MASK:u64=0xffffffff; pub const KVM_XICS_PRIORITY_SHIFT:u32=32; pub const KVM_XICS_PRIORITY_MASK:u64=0xff; pub const KVM_XICS_LEVEL_SENSITIVE:u64=1<<40; pub const KVM_XICS_MASKED:u64=1<<41; pub const KVM_XICS_PENDING:u64=1<<42; pub const KVM_XICS_PRESENTED:u64=1<<43; pub const KVM_XICS_QUEUED:u64=1<<44;
pub const KVM_DEV_XIVE_GRP_CTRL:u32=1; pub const KVM_DEV_XIVE_RESET:u32=1; pub const KVM_DEV_XIVE_EQ_SYNC:u32=2; pub const KVM_DEV_XIVE_NR_SERVERS:u32=3; pub const KVM_DEV_XIVE_GRP_SOURCE:u32=2; pub const KVM_DEV_XIVE_GRP_SOURCE_CONFIG:u32=3; pub const KVM_DEV_XIVE_GRP_EQ_CONFIG:u32=4; pub const KVM_DEV_XIVE_GRP_SOURCE_SYNC:u32=5; pub const KVM_XIVE_LEVEL_SENSITIVE:u64=1; pub const KVM_XIVE_LEVEL_ASSERTED:u64=1<<1; pub const KVM_XIVE_EQ_PRIORITY_SHIFT:u32=0; pub const KVM_XIVE_EQ_PRIORITY_MASK:u64=7; pub const KVM_XIVE_EQ_SERVER_SHIFT:u32=3; pub const KVM_XIVE_EQ_SERVER_MASK:u64=0xfffffff8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
