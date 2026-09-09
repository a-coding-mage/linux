/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux RISC-V KVM UAPI header. */

// Dependencies supplied by the surrounding UAPI translation.

pub const __KVM_HAVE_IRQ_LINE: bool = true;
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u64 = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u64 = 64;
pub const KVM_INTERRUPT_SET: u32 = u32::MAX;
pub const KVM_INTERRUPT_UNSET: u32 = u32::MAX - 1;
pub const KVM_EXIT_FAIL_ENTRY_NO_VSFILE: u64 = 1u64 << 0;

#[repr(C)] pub struct kvm_regs {}
#[repr(C)] pub struct kvm_fpu {}
#[repr(C)] pub struct kvm_debug_exit_arch {}
#[repr(C)] pub struct kvm_guest_debug_arch {}
#[repr(C)] pub struct kvm_sync_regs {}
#[repr(C)] pub struct kvm_sregs {}

#[repr(C)]
pub struct kvm_riscv_config {
    pub isa: usize, pub zicbom_block_size: usize, pub mvendorid: usize,
    pub marchid: usize, pub mimpid: usize, pub zicboz_block_size: usize,
    pub satp_mode: usize, pub zicbop_block_size: usize,
}

#[repr(C)] pub struct kvm_riscv_core { pub regs: user_regs_struct, pub mode: usize }
pub const KVM_RISCV_MODE_S: u32 = 1;
pub const KVM_RISCV_MODE_U: u32 = 0;

#[repr(C)]
pub struct kvm_riscv_csr {
    pub sstatus: usize, pub sie: usize, pub stvec: usize, pub sscratch: usize,
    pub sepc: usize, pub scause: usize, pub stval: usize, pub sip: usize,
    pub satp: usize, pub scounteren: usize, pub senvcfg: usize,
}
#[repr(C)] pub struct kvm_riscv_aia_csr { pub siselect: usize, pub iprio1: usize, pub iprio2: usize, pub sieh: usize, pub siph: usize, pub iprio1h: usize, pub iprio2h: usize }
#[repr(C)] pub struct kvm_riscv_smstateen_csr { pub sstateen0: usize }
#[repr(C)] pub struct kvm_riscv_zicfiss_csr { pub ssp: usize }
#[repr(C)] pub struct kvm_riscv_timer { pub frequency: u64, pub time: u64, pub compare: u64, pub state: u64 }
pub const KVM_RISCV_TIMER_STATE_OFF: u32 = 0;
pub const KVM_RISCV_TIMER_STATE_ON: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KVM_RISCV_ISA_EXT_ID {
    KVM_RISCV_ISA_EXT_A = 0, KVM_RISCV_ISA_EXT_C, KVM_RISCV_ISA_EXT_D, KVM_RISCV_ISA_EXT_F,
    KVM_RISCV_ISA_EXT_H, KVM_RISCV_ISA_EXT_I, KVM_RISCV_ISA_EXT_M, KVM_RISCV_ISA_EXT_SVPBMT,
    KVM_RISCV_ISA_EXT_SSTC, KVM_RISCV_ISA_EXT_SVINVAL, KVM_RISCV_ISA_EXT_ZIHINTPAUSE,
    KVM_RISCV_ISA_EXT_ZICBOM, KVM_RISCV_ISA_EXT_ZICBOZ, KVM_RISCV_ISA_EXT_ZBB,
    KVM_RISCV_ISA_EXT_SSAIA, KVM_RISCV_ISA_EXT_V, KVM_RISCV_ISA_EXT_SVNAPOT,
    KVM_RISCV_ISA_EXT_ZBA, KVM_RISCV_ISA_EXT_ZBS, KVM_RISCV_ISA_EXT_ZICNTR,
    KVM_RISCV_ISA_EXT_ZICSR, KVM_RISCV_ISA_EXT_ZIFENCEI, KVM_RISCV_ISA_EXT_ZIHPM,
    KVM_RISCV_ISA_EXT_SMSTATEEN, KVM_RISCV_ISA_EXT_ZICOND, KVM_RISCV_ISA_EXT_ZBC,
    KVM_RISCV_ISA_EXT_ZBKB, KVM_RISCV_ISA_EXT_ZBKC, KVM_RISCV_ISA_EXT_ZBKX,
    KVM_RISCV_ISA_EXT_ZKND, KVM_RISCV_ISA_EXT_ZKNE, KVM_RISCV_ISA_EXT_ZKNH,
    KVM_RISCV_ISA_EXT_ZKR, KVM_RISCV_ISA_EXT_ZKSED, KVM_RISCV_ISA_EXT_ZKSH,
    KVM_RISCV_ISA_EXT_ZKT, KVM_RISCV_ISA_EXT_ZVBB, KVM_RISCV_ISA_EXT_ZVBC,
    KVM_RISCV_ISA_EXT_ZVKB, KVM_RISCV_ISA_EXT_ZVKG, KVM_RISCV_ISA_EXT_ZVKNED,
    KVM_RISCV_ISA_EXT_ZVKNHA, KVM_RISCV_ISA_EXT_ZVKNHB, KVM_RISCV_ISA_EXT_ZVKSED,
    KVM_RISCV_ISA_EXT_ZVKSH, KVM_RISCV_ISA_EXT_ZVKT, KVM_RISCV_ISA_EXT_ZFH,
    KVM_RISCV_ISA_EXT_ZFHMIN, KVM_RISCV_ISA_EXT_ZIHINTNTL, KVM_RISCV_ISA_EXT_ZVFH,
    KVM_RISCV_ISA_EXT_ZVFHMIN, KVM_RISCV_ISA_EXT_ZFA, KVM_RISCV_ISA_EXT_ZTSO,
    KVM_RISCV_ISA_EXT_ZACAS, KVM_RISCV_ISA_EXT_SSCOFPMF, KVM_RISCV_ISA_EXT_ZIMOP,
    KVM_RISCV_ISA_EXT_ZCA, KVM_RISCV_ISA_EXT_ZCB, KVM_RISCV_ISA_EXT_ZCD,
    KVM_RISCV_ISA_EXT_ZCF, KVM_RISCV_ISA_EXT_ZCMOP, KVM_RISCV_ISA_EXT_ZAWRS,
    KVM_RISCV_ISA_EXT_SMNPM, KVM_RISCV_ISA_EXT_SSNPM, KVM_RISCV_ISA_EXT_SVADE,
    KVM_RISCV_ISA_EXT_SVADU, KVM_RISCV_ISA_EXT_SVVPTC, KVM_RISCV_ISA_EXT_ZABHA,
    KVM_RISCV_ISA_EXT_ZICCRSE, KVM_RISCV_ISA_EXT_ZAAMO, KVM_RISCV_ISA_EXT_ZALRSC,
    KVM_RISCV_ISA_EXT_ZICBOP, KVM_RISCV_ISA_EXT_ZFBFMIN, KVM_RISCV_ISA_EXT_ZVFBFMIN,
    KVM_RISCV_ISA_EXT_ZVFBFWMA, KVM_RISCV_ISA_EXT_ZCLSD, KVM_RISCV_ISA_EXT_ZILSD,
    KVM_RISCV_ISA_EXT_ZALASR, KVM_RISCV_ISA_EXT_ZICFILP, KVM_RISCV_ISA_EXT_ZICFISS,
    KVM_RISCV_ISA_EXT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KVM_RISCV_SBI_EXT_ID { KVM_RISCV_SBI_EXT_V01 = 0, KVM_RISCV_SBI_EXT_TIME, KVM_RISCV_SBI_EXT_IPI, KVM_RISCV_SBI_EXT_RFENCE, KVM_RISCV_SBI_EXT_SRST, KVM_RISCV_SBI_EXT_HSM, KVM_RISCV_SBI_EXT_PMU, KVM_RISCV_SBI_EXT_EXPERIMENTAL, KVM_RISCV_SBI_EXT_VENDOR, KVM_RISCV_SBI_EXT_DBCN, KVM_RISCV_SBI_EXT_STA, KVM_RISCV_SBI_EXT_SUSP, KVM_RISCV_SBI_EXT_FWFT, KVM_RISCV_SBI_EXT_MPXY, KVM_RISCV_SBI_EXT_MAX }

#[repr(C)] pub struct kvm_riscv_sbi_sta { pub shmem_lo: usize, pub shmem_hi: usize }
#[repr(C)] pub struct kvm_riscv_sbi_fwft_feature { pub enable: usize, pub flags: usize, pub value: usize }
#[repr(C)] pub struct kvm_riscv_sbi_fwft { pub misaligned_deleg: kvm_riscv_sbi_fwft_feature, pub pointer_masking: kvm_riscv_sbi_fwft_feature, pub pte_ad_hw_updating: kvm_riscv_sbi_fwft_feature, pub landing_pad: kvm_riscv_sbi_fwft_feature, pub shadow_stack: kvm_riscv_sbi_fwft_feature }

pub const KVM_REG_RISCV_TYPE_MASK: u64 = 0x00000000FF000000;
pub const KVM_REG_RISCV_TYPE_SHIFT: u32 = 24;
pub const KVM_REG_RISCV_SUBTYPE_MASK: u64 = 0x0000000000FF0000;
pub const KVM_REG_RISCV_SUBTYPE_SHIFT: u32 = 16;
pub const KVM_REG_RISCV_CONFIG: u64 = 0x01 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_CORE: u64 = 0x02 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_CSR: u64 = 0x03 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_CSR_GENERAL: u64 = 0x0 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_CSR_AIA: u64 = 0x1 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_CSR_SMSTATEEN: u64 = 0x2 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_CSR_ZICFISS: u64 = 0x3 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_TIMER: u64 = 0x04 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_FP_F: u64 = 0x05 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_FP_D: u64 = 0x06 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_ISA_EXT: u64 = 0x07 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_ISA_SINGLE: u64 = 0x0 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_ISA_MULTI_EN: u64 = 0x1 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_ISA_MULTI_DIS: u64 = 0x2 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_EXT: u64 = 0x08 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_SINGLE: u64 = 0x0 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_MULTI_EN: u64 = 0x1 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_MULTI_DIS: u64 = 0x2 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_VECTOR: u64 = 0x09 << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_STATE: u64 = 0x0a << KVM_REG_RISCV_TYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_STA: u64 = 0x0 << KVM_REG_RISCV_SUBTYPE_SHIFT;
pub const KVM_REG_RISCV_SBI_FWFT: u64 = 0x1 << KVM_REG_RISCV_SUBTYPE_SHIFT;

// The following local macros preserve the C offsetof-based register-index API.
#[macro_export] macro_rules! KVM_REG_RISCV_CONFIG_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_config, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_CORE_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_core, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_CSR_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_csr, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_CSR_AIA_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_aia_csr, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_CSR_SMSTATEEN_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_smstateen_csr, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_CSR_ZICFISS_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_zicfiss_csr, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_TIMER_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_timer, $name) / ::core::mem::size_of::<u64>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_FP_F_REG { ($name:ident) => { ::core::mem::offset_of!(__riscv_f_ext_state, $name) / ::core::mem::size_of::<u32>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_FP_D_REG { ($name:ident) => { ::core::mem::offset_of!(__riscv_d_ext_state, $name) / ::core::mem::size_of::<u64>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_VECTOR_CSR_REG { ($name:ident) => { ::core::mem::offset_of!(__riscv_v_ext_state, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_VECTOR_REG { ($n:expr) => { ($n) + ::core::mem::size_of::<__riscv_v_ext_state>() / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_SBI_STA_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_sbi_sta, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_SBI_FWFT_REG { ($name:ident) => { ::core::mem::offset_of!(kvm_riscv_sbi_fwft, $name) / ::core::mem::size_of::<usize>() }; }
#[macro_export] macro_rules! KVM_REG_RISCV_ISA_MULTI_REG { ($id:expr) => { ($id) / usize::BITS as usize }; }
#[macro_export] macro_rules! KVM_REG_RISCV_ISA_MULTI_MASK { ($id:expr) => { 1usize << (($id) % usize::BITS as usize) }; }
#[macro_export] macro_rules! KVM_REG_RISCV_SBI_MULTI_REG { ($id:expr) => { ($id) / usize::BITS as usize }; }
#[macro_export] macro_rules! KVM_REG_RISCV_SBI_MULTI_MASK { ($id:expr) => { 1usize << (($id) % usize::BITS as usize) }; }

pub const KVM_DEV_RISCV_APLIC_ALIGN: u32 = 0x1000; pub const KVM_DEV_RISCV_APLIC_SIZE: u32 = 0x4000; pub const KVM_DEV_RISCV_APLIC_MAX_HARTS: u32 = 0x4000; pub const KVM_DEV_RISCV_IMSIC_ALIGN: u32 = 0x1000; pub const KVM_DEV_RISCV_IMSIC_SIZE: u32 = 0x1000;
pub const KVM_DEV_RISCV_AIA_GRP_CONFIG: u32 = 0; pub const KVM_DEV_RISCV_AIA_CONFIG_MODE: u32 = 0; pub const KVM_DEV_RISCV_AIA_CONFIG_IDS: u32 = 1; pub const KVM_DEV_RISCV_AIA_CONFIG_SRCS: u32 = 2; pub const KVM_DEV_RISCV_AIA_CONFIG_GROUP_BITS: u32 = 3; pub const KVM_DEV_RISCV_AIA_CONFIG_GROUP_SHIFT: u32 = 4; pub const KVM_DEV_RISCV_AIA_CONFIG_HART_BITS: u32 = 5; pub const KVM_DEV_RISCV_AIA_CONFIG_GUEST_BITS: u32 = 6;
pub const KVM_DEV_RISCV_AIA_MODE_EMUL: u32 = 0; pub const KVM_DEV_RISCV_AIA_MODE_HWACCEL: u32 = 1; pub const KVM_DEV_RISCV_AIA_MODE_AUTO: u32 = 2;
pub const KVM_DEV_RISCV_AIA_IDS_MIN: u32 = 63; pub const KVM_DEV_RISCV_AIA_IDS_MAX: u32 = 2048; pub const KVM_DEV_RISCV_AIA_SRCS_MAX: u32 = 1024; pub const KVM_DEV_RISCV_AIA_GROUP_BITS_MAX: u32 = 8; pub const KVM_DEV_RISCV_AIA_GROUP_SHIFT_MIN: u32 = 24; pub const KVM_DEV_RISCV_AIA_GROUP_SHIFT_MAX: u32 = 56; pub const KVM_DEV_RISCV_AIA_HART_BITS_MAX: u32 = 16; pub const KVM_DEV_RISCV_AIA_GUEST_BITS_MAX: u32 = 8;
pub const KVM_DEV_RISCV_AIA_GRP_ADDR: u32 = 1; pub const KVM_DEV_RISCV_AIA_ADDR_APLIC: u32 = 0; pub const KVM_DEV_RISCV_AIA_GRP_CTRL: u32 = 2; pub const KVM_DEV_RISCV_AIA_CTRL_INIT: u32 = 0; pub const KVM_DEV_RISCV_AIA_GRP_APLIC: u32 = 3; pub const KVM_DEV_RISCV_AIA_GRP_IMSIC: u32 = 4; pub const KVM_DEV_RISCV_AIA_IMSIC_ISEL_BITS: u32 = 12; pub const KVM_DEV_RISCV_AIA_IMSIC_ISEL_MASK: u32 = (1u32 << 12) - 1; pub const KVM_NR_IRQCHIPS: u32 = 1;
#[macro_export] macro_rules! KVM_DEV_RISCV_AIA_ADDR_IMSIC { ($vcpu:expr) => { 1 + ($vcpu) }; }
#[macro_export] macro_rules! KVM_DEV_RISCV_AIA_ADDR_MAX { () => { 1 + KVM_DEV_RISCV_APLIC_MAX_HARTS }; }
#[macro_export] macro_rules! KVM_DEV_RISCV_AIA_IMSIC_MKATTR { ($vcpu:expr, $isel:expr) => { (($vcpu) << KVM_DEV_RISCV_AIA_IMSIC_ISEL_BITS) | (($isel) & KVM_DEV_RISCV_AIA_IMSIC_ISEL_MASK) }; }
#[macro_export] macro_rules! KVM_DEV_RISCV_AIA_IMSIC_GET_ISEL { ($attr:expr) => { ($attr) & KVM_DEV_RISCV_AIA_IMSIC_ISEL_MASK }; }
#[macro_export] macro_rules! KVM_DEV_RISCV_AIA_IMSIC_GET_VCPU { ($attr:expr) => { ($attr) >> KVM_DEV_RISCV_AIA_IMSIC_ISEL_BITS }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
