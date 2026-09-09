/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2023 Loongson Technology Corporation Limited */
/* C header dependencies are supplied by the surrounding kernel translation. */

#[macro_export] macro_rules! KVM_GET_IOC_CSR_IDX { ($id:expr) => (($id & KVM_CSR_IDX_MASK) >> LOONGARCH_REG_SHIFT) }
#[macro_export] macro_rules! KVM_GET_IOC_CPUCFG_IDX { ($id:expr) => (($id & KVM_CPUCFG_IDX_MASK) >> LOONGARCH_REG_SHIFT) }

pub const KVM_MAX_VCPUS: usize = 256;
pub const KVM_MAX_CPUCFG_REGS: usize = 21;
pub const KVM_HALT_POLL_NS_DEFAULT: usize = 500000;
pub const KVM_REQ_TLB_FLUSH_GPA: usize = KVM_ARCH_REQ(0);
pub const KVM_REQ_STEAL_UPDATE: usize = KVM_ARCH_REQ(1);
pub const KVM_REQ_PMU: usize = KVM_ARCH_REQ(2);
pub const KVM_REQ_FPU_LOAD: usize = KVM_ARCH_REQ(3);
pub const KVM_REQ_LBT_LOAD: usize = KVM_ARCH_REQ(4);
pub const KVM_GUESTDBG_SW_BP_MASK: usize = KVM_GUESTDBG_ENABLE | KVM_GUESTDBG_USE_SW_BP;
pub const KVM_GUESTDBG_VALID_MASK: usize = KVM_GUESTDBG_ENABLE | KVM_GUESTDBG_USE_SW_BP | KVM_GUESTDBG_SINGLESTEP;
pub const KVM_DIRTY_LOG_MANUAL_CAPS: usize = KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE | KVM_DIRTY_LOG_INITIALLY_SET;

#[repr(C)]
pub struct kvm_vm_stat { pub generic: kvm_vm_stat_generic, pub pages: u64, pub hugepages: u64 }

#[repr(C)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub int_exits: u64, pub idle_exits: u64, pub cpucfg_exits: u64,
    pub signal_exits: u64, pub hypercall_exits: u64,
    pub ipi_read_exits: u64, pub ipi_write_exits: u64,
    pub eiointc_read_exits: u64, pub eiointc_write_exits: u64,
    pub pch_pic_read_exits: u64, pub pch_pic_write_exits: u64,
}

pub const KVM_MEM_HUGEPAGE_CAPABLE: libc::c_ulong = 1 << 0;
pub const KVM_MEM_HUGEPAGE_INCAPABLE: libc::c_ulong = 1 << 1;
#[repr(C)] pub struct kvm_arch_memory_slot { pub flags: libc::c_ulong }

pub const HOST_MAX_PMNUM: usize = 16;
#[repr(C)] pub struct kvm_context {
    pub vpid_cache: libc::c_ulong, pub last_vcpu: *mut kvm_vcpu,
    pub perf_ctrl: [u64; HOST_MAX_PMNUM], pub perf_cntr: [u64; HOST_MAX_PMNUM],
}
#[repr(C)] pub struct kvm_world_switch {
    pub exc_entry: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub enter_guest: Option<unsafe extern "C" fn(*mut kvm_run, *mut kvm_vcpu) -> libc::c_int>,
}
pub const MAX_PGTABLE_LEVELS: usize = 4;
pub const KVM_MAX_PHYID: usize = 256;
#[repr(C)] pub struct kvm_phyid_info { pub vcpu: *mut kvm_vcpu, pub enabled: bool }
#[repr(C)] pub struct kvm_phyid_map { pub max_phyid: libc::c_int, pub phys_map: [kvm_phyid_info; KVM_MAX_PHYID] }

#[repr(C)] pub struct kvm_arch {
    pub pgd: *mut kvm_pte_t, pub gpa_size: libc::c_ulong,
    pub invalid_ptes: [libc::c_ulong; MAX_PGTABLE_LEVELS], pub pte_shifts: [libc::c_uint; MAX_PGTABLE_LEVELS],
    pub root_level: libc::c_uint, pub phyid_map_lock: spinlock_t, pub phyid_map: *mut kvm_phyid_map,
    pub pv_features: libc::c_ulong, pub kvm_features: libc::c_ulong, pub time_offset: i64,
    pub vmcs: *mut kvm_context, pub ipi: *mut loongarch_ipi, pub dmsintc: *mut loongarch_dmsintc,
    pub eiointc: *mut loongarch_eiointc, pub pch_pic: *mut loongarch_pch_pic,
}
pub const CSR_MAX_NUMS: usize = 0x800;
#[repr(C)] pub struct loongarch_csrs { pub csrs: [libc::c_ulong; CSR_MAX_NUMS] }
pub const RESUME_HOST: libc::c_int = 0; pub const RESUME_GUEST: libc::c_int = 1;
#[repr(C)] pub enum emulation_result { EMULATE_DONE, EMULATE_DO_MMIO, EMULATE_DO_IOCSR, EMULATE_FAIL, EMULATE_EXCEPT }
pub const KVM_LARCH_FPU: u32 = 0x1 << 0; pub const KVM_LARCH_LBT: u32 = 0x1 << 1;
pub const KVM_LARCH_PMU: u32 = 0x1 << 2; pub const KVM_LARCH_SWCSR_LATEST: u32 = 0x1 << 3; pub const KVM_LARCH_HWCSR_USABLE: u32 = 0x1 << 4;
pub const LOONGARCH_PV_FEAT_UPDATED: u64 = 1u64 << 63;
pub const LOONGARCH_PV_FEAT_MASK: u64 = (1u64 << KVM_FEATURE_IPI) | (1u64 << KVM_FEATURE_PREEMPT) | (1u64 << KVM_FEATURE_STEAL_TIME) | (1u64 << KVM_FEATURE_USER_HCALL) | (1u64 << KVM_FEATURE_VIRT_EXTIOI);

#[repr(C)] pub struct kvm_vcpu_arch {
    pub host_eentry: libc::c_ulong, pub guest_eentry: libc::c_ulong,
    pub handle_exit: Option<unsafe extern "C" fn(*mut kvm_run, *mut kvm_vcpu) -> libc::c_int>,
    pub kvm_pgd: libc::c_ulong, pub host_sp: libc::c_ulong, pub host_tp: libc::c_ulong, pub host_pgd: libc::c_ulong,
    pub badi: libc::c_ulong, pub badv: libc::c_ulong, pub host_ecfg: libc::c_ulong, pub host_estat: libc::c_ulong, pub host_percpu: libc::c_ulong,
    pub gprs: [libc::c_ulong; 32], pub pc: libc::c_ulong, pub aux_inuse: libc::c_uint,
    pub fpu: loongarch_fpu, pub lbt: loongarch_lbt, pub csr: *mut loongarch_csrs, pub max_pmu_csrid: libc::c_int, pub io_gpr: u32, pub count_ctl: u32, pub swtimer: hrtimer,
    pub irq_pending: libc::c_ulong, pub irq_clear: libc::c_ulong, pub exception_pending: libc::c_ulong, pub esubcode: libc::c_uint,
    pub mmu_page_cache: kvm_mmu_memory_cache, pub vpid: u64, pub flush_gpa: gpa_t, pub timer_mhz: u64, pub expire: ktime_t,
    pub last_sched_cpu: libc::c_int, pub mp_state: kvm_mp_state, pub ipi_state: ipi_state, pub dmsintc_state: dmsintc_state,
    pub cpucfg: [u32; KVM_MAX_CPUCFG_REGS],
    pub st: kvm_vcpu_arch_st,
}
#[repr(C)] pub struct kvm_vcpu_arch_st { pub guest_addr: u64, pub last_steal: u64, pub cache: gfn_to_hva_cache, pub preempted: u8 }

pub unsafe fn readl_sw_gcsr(csr: *mut loongarch_csrs, reg: libc::c_int) -> libc::c_ulong { (*csr).csrs[reg as usize] }
pub unsafe fn writel_sw_gcsr(csr: *mut loongarch_csrs, reg: libc::c_int, val: libc::c_ulong) { (*csr).csrs[reg as usize] = val; }
pub unsafe fn kvm_guest_has_msgint(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[1] & CPUCFG1_MSGINT != 0 }
pub unsafe fn kvm_guest_has_fpu(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[2] & CPUCFG2_FP != 0 }
pub unsafe fn kvm_guest_has_lsx(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[2] & CPUCFG2_LSX != 0 }
pub unsafe fn kvm_guest_has_lasx(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[2] & CPUCFG2_LASX != 0 }
pub unsafe fn kvm_guest_has_lbt(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[2] & (CPUCFG2_X86BT | CPUCFG2_ARMBT | CPUCFG2_MIPSBT) != 0 }
pub unsafe fn kvm_guest_has_pmu(arch: *mut kvm_vcpu_arch) -> bool { (*arch).cpucfg[6] & CPUCFG6_PMP != 0 }
pub unsafe fn kvm_get_pmu_num(arch: *mut kvm_vcpu_arch) -> libc::c_int { (((*arch).cpucfg[6] & CPUCFG6_PMNUM) >> CPUCFG6_PMNUM_SHIFT) as libc::c_int }
pub unsafe fn kvm_vm_support(arch: *mut kvm_arch, feature: libc::c_int) -> bool { (*arch).kvm_features & (1u64 << feature) != 0 }
pub unsafe fn update_pc(arch: *mut kvm_vcpu_arch) { (*arch).pc = (*arch).pc.wrapping_add(4); }
pub unsafe fn kvm_is_ifetch_fault(arch: *mut kvm_vcpu_arch) -> bool { (*arch).pc == (*arch).badv }
pub unsafe fn kvm_arch_hardware_unsetup() {} pub unsafe fn kvm_arch_memslots_updated(_: *mut kvm, _: u64) {}
pub unsafe fn kvm_arch_vcpu_blocking(_: *mut kvm_vcpu) {} pub unsafe fn kvm_arch_vcpu_unblocking(_: *mut kvm_vcpu) {} pub unsafe fn kvm_arch_vcpu_block_finish(_: *mut kvm_vcpu) {}
pub unsafe fn kvm_arch_free_memslot(_: *mut kvm, _: *mut kvm_memory_slot) {}

extern "C" {
    pub fn kvm_arch_pmi_in_guest(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_arch_vcpu_dump_regs(vcpu: *mut kvm_vcpu) -> libc::c_int;
    pub fn kvm_flush_tlb_all(); pub fn kvm_flush_tlb_gpa(vcpu: *mut kvm_vcpu, gpa: libc::c_ulong);
    pub fn kvm_handle_mm_fault(vcpu: *mut kvm_vcpu, badv: libc::c_ulong, write: bool, ecode: libc::c_int) -> libc::c_int;
    pub fn kvm_unmap_hva_range(kvm: *mut kvm, start: libc::c_ulong, end: libc::c_ulong, blockable: bool) -> libc::c_int;
    pub fn kvm_age_hva(kvm: *mut kvm, start: libc::c_ulong, end: libc::c_ulong) -> libc::c_int;
    pub fn kvm_test_age_hva(kvm: *mut kvm, hva: libc::c_ulong) -> libc::c_int;
    pub fn kvm_check_vpid(vcpu: *mut kvm_vcpu); pub fn kvm_swtimer_wakeup(timer: *mut hrtimer) -> hrtimer_restart;
    pub fn kvm_arch_flush_remote_tlbs_memslot(kvm: *mut kvm, memslot: *const kvm_memory_slot);
    pub fn kvm_init_vmcs(kvm: *mut kvm); pub fn kvm_exc_entry();
    pub fn kvm_enter_guest(run: *mut kvm_run, vcpu: *mut kvm_vcpu) -> libc::c_int;
    pub static mut vpid_mask: libc::c_ulong; pub static mut kvm_loongarch_ops: *mut kvm_world_switch;
    pub fn get_gcsr_flag(csr: libc::c_int) -> libc::c_int; pub fn set_hw_gcsr(csr_id: libc::c_int, val: libc::c_ulong);
}
pub const SW_GCSR: u32 = 1 << 0; pub const HW_GCSR: u32 = 1 << 1; pub const INVALID_GCSR: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
