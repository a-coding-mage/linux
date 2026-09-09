/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux ARM64 UAPI KVM header. */

pub const KVM_SPSR_EL1: u32 = 0;
pub const KVM_SPSR_SVC: u32 = KVM_SPSR_EL1;
pub const KVM_SPSR_ABT: u32 = 1;
pub const KVM_SPSR_UND: u32 = 2;
pub const KVM_SPSR_IRQ: u32 = 3;
pub const KVM_SPSR_FIQ: u32 = 4;
pub const KVM_NR_SPSR: usize = 5;

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u32 = 64;

#[repr(C)]
pub struct kvm_regs {
    pub regs: user_pt_regs,
    pub sp_el1: __u64,
    pub elr_el1: __u64,
    pub spsr: [__u64; KVM_NR_SPSR],
    pub fp_regs: user_fpsimd_state,
}

pub const KVM_ARM_TARGET_AEM_V8: u32 = 0;
pub const KVM_ARM_TARGET_FOUNDATION_V8: u32 = 1;
pub const KVM_ARM_TARGET_CORTEX_A57: u32 = 2;
pub const KVM_ARM_TARGET_XGENE_POTENZA: u32 = 3;
pub const KVM_ARM_TARGET_CORTEX_A53: u32 = 4;
pub const KVM_ARM_TARGET_GENERIC_V8: u32 = 5;
pub const KVM_ARM_NUM_TARGETS: u32 = 6;

pub const KVM_ARM_DEVICE_TYPE_SHIFT: u32 = 0;
pub const KVM_ARM_DEVICE_TYPE_MASK: u32 = __GENMASK(KVM_ARM_DEVICE_TYPE_SHIFT + 15, KVM_ARM_DEVICE_TYPE_SHIFT);
pub const KVM_ARM_DEVICE_ID_SHIFT: u32 = 16;
pub const KVM_ARM_DEVICE_ID_MASK: u32 = __GENMASK(KVM_ARM_DEVICE_ID_SHIFT + 15, KVM_ARM_DEVICE_ID_SHIFT);
pub const KVM_ARM_DEVICE_VGIC_V2: u32 = 0;
pub const KVM_VGIC_V2_ADDR_TYPE_DIST: u32 = 0;
pub const KVM_VGIC_V2_ADDR_TYPE_CPU: u32 = 1;
pub const KVM_VGIC_V2_DIST_SIZE: u32 = 0x1000;
pub const KVM_VGIC_V2_CPU_SIZE: u32 = 0x2000;
pub const KVM_VGIC_V3_ADDR_TYPE_DIST: u32 = 2;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST: u32 = 3;
pub const KVM_VGIC_ITS_ADDR_TYPE: u32 = 4;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION: u32 = 5;
pub const KVM_VGIC_V3_DIST_SIZE: usize = SZ_64K;
pub const KVM_VGIC_V3_REDIST_SIZE: usize = 2 * SZ_64K;
pub const KVM_VGIC_V3_ITS_SIZE: usize = 2 * SZ_64K;

pub const KVM_ARM_VCPU_POWER_OFF: u32 = 0;
pub const KVM_ARM_VCPU_EL1_32BIT: u32 = 1;
pub const KVM_ARM_VCPU_PSCI_0_2: u32 = 2;
pub const KVM_ARM_VCPU_PMU_V3: u32 = 3;
pub const KVM_ARM_VCPU_SVE: u32 = 4;
pub const KVM_ARM_VCPU_PTRAUTH_ADDRESS: u32 = 5;
pub const KVM_ARM_VCPU_PTRAUTH_GENERIC: u32 = 6;
pub const KVM_ARM_VCPU_HAS_EL2: u32 = 7;
pub const KVM_ARM_VCPU_HAS_EL2_E2H0: u32 = 8;
pub const KVM_ARM_VCPU_PMU_V3_STRICT: u32 = 9;

#[repr(C)] pub struct kvm_vcpu_init { pub target: __u32, pub features: [__u32; 7] }
#[repr(C)] pub struct kvm_sregs {}
#[repr(C)] pub struct kvm_fpu {}
pub const KVM_ARM_MAX_DBG_REGS: usize = 16;
#[repr(C)] pub struct kvm_guest_debug_arch { pub dbg_bcr: [__u64;16], pub dbg_bvr: [__u64;16], pub dbg_wcr: [__u64;16], pub dbg_wvr: [__u64;16] }
pub const KVM_DEBUG_ARCH_HSR_HIGH_VALID: u32 = 1 << 0;
#[repr(C)] pub struct kvm_debug_exit_arch { pub hsr: __u32, pub hsr_high: __u32, pub far: __u64 }
pub const KVM_GUESTDBG_USE_SW_BP: u32 = 1 << 16;
pub const KVM_GUESTDBG_USE_HW: u32 = 1 << 17;
#[repr(C)] pub struct kvm_sync_regs { pub device_irq_level: __u64 }
pub const KVM_ARM_DEV_EL1_VTIMER: u64 = 1<<0; pub const KVM_ARM_DEV_EL1_PTIMER: u64 = 1<<1; pub const KVM_ARM_DEV_PMU: u64 = 1<<2;
#[repr(C)] pub struct kvm_pmu_event_filter { pub base_event: __u16, pub nevents: __u16, pub action: __u8, pub pad: [__u8;3] }
pub const KVM_PMU_EVENT_ALLOW: u8 = 0; pub const KVM_PMU_EVENT_DENY: u8 = 1;
#[repr(C)] pub struct kvm_vcpu_events { pub exception: kvm_vcpu_events_exception, pub reserved: [__u32;12] }
#[repr(C)] pub struct kvm_vcpu_events_exception { pub serror_pending: __u8, pub serror_has_esr: __u8, pub ext_dabt_pending: __u8, pub pad: [__u8;5], pub serror_esr: __u64 }
#[repr(C)] pub struct kvm_arm_copy_mte_tags { pub guest_ipa: __u64, pub length: __u64, pub addr: *mut core::ffi::c_void, pub flags: __u64, pub reserved: [__u64;2] }
#[repr(C)] pub struct kvm_arm_counter_offset { pub counter_offset: __u64, pub reserved: __u64 }
pub const KVM_ARM_TAGS_TO_GUEST: u32 = 0; pub const KVM_ARM_TAGS_FROM_GUEST: u32 = 1;

pub const KVM_REG_ARM_COPROC_MASK: u64 = 0x000000000FFF0000; pub const KVM_REG_ARM_COPROC_SHIFT: u32 = 16;
pub const KVM_REG_ARM_CORE: u64 = 0x0010u64 << KVM_REG_ARM_COPROC_SHIFT;
#[macro_export] macro_rules! KVM_REG_ARM_CORE_REG { ($name:ident) => { core::mem::offset_of!(kvm_regs, $name) as u64 / core::mem::size_of::<__u32>() as u64 }; }
pub const KVM_REG_ARM_DEMUX: u64 = 0x0011u64 << KVM_REG_ARM_COPROC_SHIFT; pub const KVM_REG_ARM_DEMUX_ID_MASK:u64=0xff00; pub const KVM_REG_ARM_DEMUX_ID_SHIFT:u32=8; pub const KVM_REG_ARM_DEMUX_ID_CCSIDR:u64=0; pub const KVM_REG_ARM_DEMUX_VAL_MASK:u64=0xff; pub const KVM_REG_ARM_DEMUX_VAL_SHIFT:u32=0;
pub const KVM_REG_ARM64_SYSREG:u64=0x0013u64<<KVM_REG_ARM_COPROC_SHIFT; pub const KVM_REG_ARM64_SYSREG_OP0_MASK:u64=0xc000; pub const KVM_REG_ARM64_SYSREG_OP0_SHIFT:u32=14; pub const KVM_REG_ARM64_SYSREG_OP1_MASK:u64=0x3800; pub const KVM_REG_ARM64_SYSREG_OP1_SHIFT:u32=11; pub const KVM_REG_ARM64_SYSREG_CRN_MASK:u64=0x780; pub const KVM_REG_ARM64_SYSREG_CRN_SHIFT:u32=7; pub const KVM_REG_ARM64_SYSREG_CRM_MASK:u64=0x78; pub const KVM_REG_ARM64_SYSREG_CRM_SHIFT:u32=3; pub const KVM_REG_ARM64_SYSREG_OP2_MASK:u64=7; pub const KVM_REG_ARM64_SYSREG_OP2_SHIFT:u32=0;
#[inline] pub const fn arm64_sys_reg_shift_mask(x:u64, shift:u32, mask:u64)->u64 {(x<<shift)&mask}
#[macro_export] macro_rules! ARM64_SYS_REG { ($op0:expr,$op1:expr,$crn:expr,$crm:expr,$op2:expr) => { (KVM_REG_ARM64 | KVM_REG_ARM64_SYSREG | (($op0 as u64)<<14 & 0xc000) | (($op1 as u64)<<11 & 0x3800) | (($crn as u64)<<7 & 0x780) | (($crm as u64)<<3 & 0x78) | (($op2 as u64)&7) | KVM_REG_SIZE_U64) }; }
pub const KVM_REG_ARM_PTIMER_CTL:u64=ARM64_SYS_REG!(3,3,14,2,1); pub const KVM_REG_ARM_PTIMER_CVAL:u64=ARM64_SYS_REG!(3,3,14,2,2); pub const KVM_REG_ARM_PTIMER_CNT:u64=ARM64_SYS_REG!(3,3,14,0,1); pub const KVM_REG_ARM_TIMER_CTL:u64=ARM64_SYS_REG!(3,3,14,3,1); pub const KVM_REG_ARM_TIMER_CVAL:u64=ARM64_SYS_REG!(3,3,14,0,2); pub const KVM_REG_ARM_TIMER_CNT:u64=ARM64_SYS_REG!(3,3,14,3,2);
pub const KVM_REG_ARM_FW:u64=0x0014u64<<KVM_REG_ARM_COPROC_SHIFT;
#[inline] pub const fn kvm_reg_arm_fw_reg(r:u64)->u64 { KVM_REG_ARM64|KVM_REG_SIZE_U64|KVM_REG_ARM_FW|(r&0xffff) }
pub const KVM_REG_ARM_PSCI_VERSION:u64=kvm_reg_arm_fw_reg(0); pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1:u64=kvm_reg_arm_fw_reg(1); pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL:u32=0; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL:u32=1; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED:u32=2;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2:u64=kvm_reg_arm_fw_reg(2); pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL:u32=0; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_UNKNOWN:u32=1; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_AVAIL:u32=2; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED:u32=3; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_ENABLED:u32=1<<4;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3:u64=kvm_reg_arm_fw_reg(3); pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_AVAIL:u32=0; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_AVAIL:u32=1; pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_REQUIRED:u32=2;
pub const KVM_REG_ARM64_SVE:u64=0x15u64<<KVM_REG_ARM_COPROC_SHIFT; pub const KVM_REG_ARM64_SVE_ZREG_BASE:u64=0; pub const KVM_REG_ARM64_SVE_PREG_BASE:u64=0x400; pub const KVM_REG_ARM64_SVE_FFR_BASE:u64=0x600; pub const KVM_ARM64_SVE_NUM_ZREGS:u64=__SVE_NUM_ZREGS; pub const KVM_ARM64_SVE_NUM_PREGS:u64=__SVE_NUM_PREGS; pub const KVM_ARM64_SVE_MAX_SLICES:u64=32;
#[inline] pub const fn kvm_reg_arm64_sve_zreg(n:u64,i:u64)->u64 { KVM_REG_ARM64|KVM_REG_ARM64_SVE|KVM_REG_ARM64_SVE_ZREG_BASE|KVM_REG_SIZE_U2048|((n&(KVM_ARM64_SVE_NUM_ZREGS-1))<<5)|(i&(KVM_ARM64_SVE_MAX_SLICES-1)) }
#[inline] pub const fn kvm_reg_arm64_sve_preg(n:u64,i:u64)->u64 { KVM_REG_ARM64|KVM_REG_ARM64_SVE|KVM_REG_ARM64_SVE_PREG_BASE|KVM_REG_SIZE_U256|((n&(KVM_ARM64_SVE_NUM_PREGS-1))<<5)|(i&(KVM_ARM64_SVE_MAX_SLICES-1)) }
#[inline] pub const fn kvm_reg_arm64_sve_ffr(i:u64)->u64 { KVM_REG_ARM64|KVM_REG_ARM64_SVE|KVM_REG_ARM64_SVE_FFR_BASE|KVM_REG_SIZE_U256|(i&(KVM_ARM64_SVE_MAX_SLICES-1)) }
pub const KVM_ARM64_SVE_VQ_MIN:u32=__SVE_VQ_MIN; pub const KVM_ARM64_SVE_VQ_MAX:u32=__SVE_VQ_MAX; pub const KVM_REG_ARM64_SVE_VLS:u64=KVM_REG_ARM64|KVM_REG_ARM64_SVE|KVM_REG_SIZE_U512|0xffff; pub const KVM_ARM64_SVE_VLS_WORDS:u32=(KVM_ARM64_SVE_VQ_MAX-KVM_ARM64_SVE_VQ_MIN)/64+1;
pub const KVM_REG_ARM_FW_FEAT_BMAP:u64=0x0016u64<<KVM_REG_ARM_COPROC_SHIFT; #[inline] pub const fn kvm_reg_arm_fw_feat_bmap_reg(r:u64)->u64 { KVM_REG_ARM64|KVM_REG_SIZE_U64|KVM_REG_ARM_FW_FEAT_BMAP|(r&0xffff) } pub const KVM_REG_ARM_STD_BMAP:u64=kvm_reg_arm_fw_feat_bmap_reg(0); pub const KVM_REG_ARM_STD_BIT_TRNG_V1_0:u32=0; pub const KVM_REG_ARM_STD_HYP_BMAP:u64=kvm_reg_arm_fw_feat_bmap_reg(1); pub const KVM_REG_ARM_STD_HYP_BIT_PV_TIME:u32=0; pub const KVM_REG_ARM_VENDOR_HYP_BMAP:u64=kvm_reg_arm_fw_feat_bmap_reg(2); pub const KVM_REG_ARM_VENDOR_HYP_BIT_FUNC_FEAT:u32=0; pub const KVM_REG_ARM_VENDOR_HYP_BIT_PTP:u32=1; pub const KVM_REG_ARM_VENDOR_HYP_BMAP_2:u64=kvm_reg_arm_fw_feat_bmap_reg(3); pub const KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_VER:u32=0; pub const KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_CPUS:u32=1;

pub const KVM_ARM_VM_SMCCC_CTRL:u32=0; pub const KVM_ARM_VM_SMCCC_FILTER:u32=0;
pub const KVM_DEV_ARM_VGIC_GRP_ADDR:u32=0; pub const KVM_DEV_ARM_VGIC_GRP_DIST_REGS:u32=1; pub const KVM_DEV_ARM_VGIC_GRP_CPU_REGS:u32=2; pub const KVM_DEV_ARM_VGIC_CPUID_SHIFT:u32=32; pub const KVM_DEV_ARM_VGIC_CPUID_MASK:u64=0xffu64<<32; pub const KVM_DEV_ARM_VGIC_V3_MPIDR_SHIFT:u32=32; pub const KVM_DEV_ARM_VGIC_V3_MPIDR_MASK:u64=0xffffffffu64<<32; pub const KVM_DEV_ARM_VGIC_OFFSET_SHIFT:u32=0; pub const KVM_DEV_ARM_VGIC_OFFSET_MASK:u64=0xffffffff; pub const KVM_DEV_ARM_VGIC_SYSREG_INSTR_MASK:u32=0xffff; pub const KVM_DEV_ARM_VGIC_GRP_NR_IRQS:u32=3; pub const KVM_DEV_ARM_VGIC_GRP_CTRL:u32=4; pub const KVM_DEV_ARM_VGIC_GRP_REDIST_REGS:u32=5; pub const KVM_DEV_ARM_VGIC_GRP_CPU_SYSREGS:u32=6; pub const KVM_DEV_ARM_VGIC_GRP_LEVEL_INFO:u32=7; pub const KVM_DEV_ARM_VGIC_GRP_ITS_REGS:u32=8; pub const KVM_DEV_ARM_VGIC_GRP_MAINT_IRQ:u32=9; pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_SHIFT:u32=10; pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_MASK:u64=0x3fffffu64<<10; pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INTID_MASK:u32=0x3ff; pub const VGIC_LEVEL_INFO_LINE_LEVEL:u32=0;
pub const KVM_DEV_ARM_VGIC_CTRL_INIT:u32=0; pub const KVM_DEV_ARM_ITS_SAVE_TABLES:u32=1; pub const KVM_DEV_ARM_ITS_RESTORE_TABLES:u32=2; pub const KVM_DEV_ARM_VGIC_SAVE_PENDING_TABLES:u32=3; pub const KVM_DEV_ARM_ITS_CTRL_RESET:u32=4; pub const KVM_DEV_ARM_VGIC_USERSPACE_PPIS:u32=5;
pub const KVM_ARM_VCPU_PMU_V3_CTRL:u32=0; pub const KVM_ARM_VCPU_PMU_V3_IRQ:u32=0; pub const KVM_ARM_VCPU_PMU_V3_INIT:u32=1; pub const KVM_ARM_VCPU_PMU_V3_FILTER:u32=2; pub const KVM_ARM_VCPU_PMU_V3_SET_PMU:u32=3; pub const KVM_ARM_VCPU_PMU_V3_SET_NR_COUNTERS:u32=4; pub const KVM_ARM_VCPU_TIMER_CTRL:u32=1; pub const KVM_ARM_VCPU_TIMER_IRQ_VTIMER:u32=0; pub const KVM_ARM_VCPU_TIMER_IRQ_PTIMER:u32=1; pub const KVM_ARM_VCPU_TIMER_IRQ_HVTIMER:u32=2; pub const KVM_ARM_VCPU_TIMER_IRQ_HPTIMER:u32=3; pub const KVM_ARM_VCPU_PVTIME_CTRL:u32=2; pub const KVM_ARM_VCPU_PVTIME_IPA:u32=0;
pub const KVM_ARM_IRQ_VCPU2_SHIFT:u32=28; pub const KVM_ARM_IRQ_VCPU2_MASK:u32=0xf; pub const KVM_ARM_IRQ_TYPE_SHIFT:u32=24; pub const KVM_ARM_IRQ_TYPE_MASK:u32=0xf; pub const KVM_ARM_IRQ_VCPU_SHIFT:u32=16; pub const KVM_ARM_IRQ_VCPU_MASK:u32=0xff; pub const KVM_ARM_IRQ_NUM_SHIFT:u32=0; pub const KVM_ARM_IRQ_NUM_MASK:u32=0xffff; pub const KVM_ARM_IRQ_TYPE_CPU:u32=0; pub const KVM_ARM_IRQ_TYPE_SPI:u32=1; pub const KVM_ARM_IRQ_TYPE_PPI:u32=2; pub const KVM_ARM_IRQ_CPU_IRQ:u32=0; pub const KVM_ARM_IRQ_CPU_FIQ:u32=1; pub const KVM_NR_IRQCHIPS:u32=1;
pub const KVM_PSCI_FN_BASE:u32=0x95c1ba5e; #[inline] pub const fn KVM_PSCI_FN(n:u32)->u32 { KVM_PSCI_FN_BASE+n } pub const KVM_PSCI_FN_CPU_SUSPEND:u32=KVM_PSCI_FN(0); pub const KVM_PSCI_FN_CPU_OFF:u32=KVM_PSCI_FN(1); pub const KVM_PSCI_FN_CPU_ON:u32=KVM_PSCI_FN(2); pub const KVM_PSCI_FN_MIGRATE:u32=KVM_PSCI_FN(3); pub const KVM_PSCI_RET_SUCCESS:u32=PSCI_RET_SUCCESS; pub const KVM_PSCI_RET_NI:u32=PSCI_RET_NOT_SUPPORTED; pub const KVM_PSCI_RET_INVAL:u32=PSCI_RET_INVALID_PARAMS; pub const KVM_PSCI_RET_DENIED:u32=PSCI_RET_DENIED;
pub const KVM_SYSTEM_EVENT_RESET_FLAG_PSCI_RESET2:u64=1; pub const KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2:u64=1; pub const KVM_EXIT_FAIL_ENTRY_CPU_UNSUPPORTED:u64=1;
#[repr(u32)] pub enum kvm_smccc_filter_action { KVM_SMCCC_FILTER_HANDLE=0, KVM_SMCCC_FILTER_DENY=1, KVM_SMCCC_FILTER_FWD_TO_USER=2 }
#[repr(C)] pub struct kvm_smccc_filter { pub base:__u32, pub nr_functions:__u32, pub action:__u8, pub pad:[__u8;15] }
pub const KVM_HYPERCALL_EXIT_SMC:u32=1; pub const KVM_HYPERCALL_EXIT_16BIT:u32=2;
#[inline] pub const fn KVM_ARM_FEATURE_ID_RANGE_IDX(_op0:u64,op1:u64,_crn:u64,crm:u64,op2:u64)->u64 { let mut v=op1&3; if v==3 {v-=1;} (v<<6)|((crm&7)<<3)|(op2&7) }
pub const KVM_ARM_FEATURE_ID_RANGE:u32=0; pub const KVM_ARM_FEATURE_ID_RANGE_SIZE:u32=3*8*8;
#[repr(C)] pub struct reg_mask_range { pub addr:__u64, pub range:__u32, pub reserved:[__u32;13] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
