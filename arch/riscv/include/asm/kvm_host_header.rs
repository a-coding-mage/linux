/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const KVM_MAX_VCPUS: u32 = 1024;
pub const KVM_HALT_POLL_NS_DEFAULT: u32 = 500000;
pub const KVM_VCPU_MAX_FEATURES: u32 = 0;
pub const KVM_IRQCHIP_NUM_PINS: u32 = 1024;

pub const KVM_REQ_SLEEP: _ = KVM_ARCH_REQ_FLAGS(0, KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP);
pub const KVM_REQ_VCPU_RESET: _ = KVM_ARCH_REQ(1);
pub const KVM_REQ_UPDATE_HGATP: _ = KVM_ARCH_REQ(2);
pub const KVM_REQ_FENCE_I: _ = KVM_ARCH_REQ_FLAGS(3, KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP);
pub const KVM_REQ_HFENCE_VVMA_ALL: _ = KVM_ARCH_REQ_FLAGS(4, KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP);
pub const KVM_REQ_HFENCE: _ = KVM_ARCH_REQ_FLAGS(5, KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP);
pub const KVM_REQ_STEAL_UPDATE: _ = KVM_ARCH_REQ(6);

// Build-time feature markers from the original header.
pub const __KVM_HAVE_ARCH_FLUSH_REMOTE_TLBS_RANGE: bool = true;
pub const KVM_HAVE_MMU_RWLOCK: bool = true;
pub const KVM_DIRTY_LOG_MANUAL_CAPS: _ =
    KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE | KVM_DIRTY_LOG_INITIALLY_SET;

#[repr(C)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
}

#[repr(C)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub ecall_exit_stat: u64,
    pub wfi_exit_stat: u64,
    pub wrs_exit_stat: u64,
    pub mmio_exit_user: u64,
    pub mmio_exit_kernel: u64,
    pub csr_exit_user: u64,
    pub csr_exit_kernel: u64,
    pub signal_exits: u64,
    pub exits: u64,
    pub instr_illegal_exits: u64,
    pub load_misaligned_exits: u64,
    pub store_misaligned_exits: u64,
    pub load_access_exits: u64,
    pub store_access_exits: u64,
}

#[repr(C)]
pub struct kvm_arch_memory_slot {}

#[repr(C)]
pub struct kvm_arch {
    pub vmid: kvm_vmid,
    pub pgd: *mut pgd_t,
    pub pgd_phys: phys_addr_t,
    pub pgd_levels: c_ulong,
    pub pgd_split_page_cache: kvm_mmu_memory_cache,
    pub timer: kvm_guest_timer,
    pub aia: kvm_aia,
    pub mp_state_reset: bool,
}

#[repr(C)]
pub struct kvm_cpu_trap {
    pub sepc: c_ulong, pub scause: c_ulong, pub stval: c_ulong,
    pub htval: c_ulong, pub htinst: c_ulong,
}

#[repr(C)]
pub struct kvm_cpu_context {
    pub zero: c_ulong, pub ra: c_ulong, pub sp: c_ulong, pub gp: c_ulong, pub tp: c_ulong,
    pub t0: c_ulong, pub t1: c_ulong, pub t2: c_ulong, pub s0: c_ulong, pub s1: c_ulong,
    pub a0: c_ulong, pub a1: c_ulong, pub a2: c_ulong, pub a3: c_ulong, pub a4: c_ulong,
    pub a5: c_ulong, pub a6: c_ulong, pub a7: c_ulong, pub s2: c_ulong, pub s3: c_ulong,
    pub s4: c_ulong, pub s5: c_ulong, pub s6: c_ulong, pub s7: c_ulong, pub s8: c_ulong,
    pub s9: c_ulong, pub s10: c_ulong, pub s11: c_ulong, pub t3: c_ulong, pub t4: c_ulong,
    pub t5: c_ulong, pub t6: c_ulong, pub sepc: c_ulong, pub sstatus: c_ulong, pub hstatus: c_ulong,
    pub fp: __riscv_fp_state, pub vector: __riscv_v_ext_state,
}

#[repr(C)]
pub struct kvm_vcpu_csr {
    pub vsstatus: c_ulong, pub vsie: c_ulong, pub vstvec: c_ulong, pub vsscratch: c_ulong,
    pub vsepc: c_ulong, pub vscause: c_ulong, pub vstval: c_ulong, pub hvip: c_ulong,
    pub vsatp: c_ulong, pub scounteren: c_ulong, pub senvcfg: c_ulong,
}
#[repr(C)] pub struct kvm_vcpu_smstateen_csr { pub sstateen0: c_ulong }
#[repr(C)] pub struct kvm_vcpu_zicfiss_csr { pub ssp: c_ulong }
#[repr(C)] pub struct kvm_vcpu_reset_state { pub lock: spinlock_t, pub pc: c_ulong, pub a1: c_ulong }

pub const KVM_RISCV_VCPU_NR_IRQS: usize = 64;

#[repr(C)]
pub struct kvm_vcpu_arch {
    pub ran_atleast_once: bool,
    pub last_exit_cpu: c_int,
    pub isa: [c_ulong; RISCV_ISA_EXT_MAX as usize / (core::mem::size_of::<c_ulong>() * 8)],
    pub mvendorid: c_ulong, pub marchid: c_ulong, pub mimpid: c_ulong,
    pub host_sscratch: c_ulong, pub host_stvec: c_ulong, pub host_scounteren: c_ulong,
    pub host_senvcfg: c_ulong, pub host_sstateen0: c_ulong,
    pub host_context: kvm_cpu_context, pub guest_context: kvm_cpu_context,
    pub guest_csr: kvm_vcpu_csr, pub smstateen_csr: kvm_vcpu_smstateen_csr,
    pub zicfiss_csr: kvm_vcpu_zicfiss_csr, pub reset_state: kvm_vcpu_reset_state,
    pub irqs_pending_lock: raw_spinlock_t,
    pub irqs_pending: [c_ulong; KVM_RISCV_VCPU_NR_IRQS / (core::mem::size_of::<c_ulong>() * 8)],
    pub irqs_pending_mask: [c_ulong; KVM_RISCV_VCPU_NR_IRQS / (core::mem::size_of::<c_ulong>() * 8)],
    pub timer: kvm_vcpu_timer, pub hfence_lock: spinlock_t,
    pub hfence_head: c_ulong, pub hfence_tail: c_ulong,
    pub hfence_queue: [kvm_riscv_hfence; KVM_RISCV_VCPU_MAX_HFENCE as usize],
    pub mmio_decode: kvm_mmio_decode, pub csr_decode: kvm_csr_decode,
    pub sbi_context: kvm_vcpu_sbi_context, pub aia_context: kvm_vcpu_aia,
    pub mmu_page_cache: kvm_mmu_memory_cache, pub mp_state: kvm_mp_state,
    pub mp_state_lock: spinlock_t, pub pause: bool, pub pmu_context: kvm_pmu,
    pub fwft_context: kvm_sbi_fwft, pub cfg: kvm_vcpu_config, pub csr_dirty: bool,
    pub sta: kvm_vcpu_arch_sta,
}

#[repr(C)] pub struct kvm_vcpu_arch_sta { pub shmem: gpa_t, pub last_steal: u64 }

#[inline]
pub unsafe fn kvm_arch_pmi_in_guest(vcpu: *mut kvm_vcpu) -> bool {
    IS_ENABLED(CONFIG_GUEST_PERF_EVENTS) && !vcpu.is_null()
}
#[inline] pub unsafe fn kvm_arch_vcpu_blocking(_vcpu: *mut kvm_vcpu) {}
#[inline] pub unsafe fn kvm_arch_vcpu_unblocking(_vcpu: *mut kvm_vcpu) {}

extern "C" {
    pub fn kvm_riscv_clear_former_vcpu();
    pub fn kvm_riscv_setup_default_irq_routing(kvm: *mut kvm, lines: u32) -> c_int;
    pub fn __kvm_riscv_unpriv_trap();
    pub fn kvm_riscv_vcpu_unpriv_read(vcpu: *mut kvm_vcpu, read_insn: bool, guest_addr: c_ulong, trap: *mut kvm_cpu_trap) -> c_ulong;
    pub fn kvm_riscv_vcpu_trap_redirect(vcpu: *mut kvm_vcpu, trap: *mut kvm_cpu_trap);
    pub fn kvm_riscv_vcpu_exit(vcpu: *mut kvm_vcpu, run: *mut kvm_run, trap: *mut kvm_cpu_trap) -> c_int;
    pub fn __kvm_riscv_switch_to(vcpu_arch: *mut kvm_vcpu_arch);
    pub fn kvm_riscv_vcpu_setup_isa(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_num_regs(vcpu: *mut kvm_vcpu) -> c_ulong;
    pub fn kvm_riscv_vcpu_copy_reg_indices(vcpu: *mut kvm_vcpu, uindices: *mut u64) -> c_int;
    pub fn kvm_riscv_vcpu_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
    pub fn kvm_riscv_vcpu_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
    pub fn kvm_riscv_vcpu_set_interrupt(vcpu: *mut kvm_vcpu, irq: c_uint) -> c_int;
    pub fn kvm_riscv_vcpu_unset_interrupt(vcpu: *mut kvm_vcpu, irq: c_uint) -> c_int;
    pub fn kvm_riscv_vcpu_flush_interrupts(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_sync_interrupts(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool;
    pub fn __kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu);
    pub fn __kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_stopped(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_riscv_vcpu_record_steal_time(vcpu: *mut kvm_vcpu);
}

// DECLARE_STATIC_KEY_FALSE(kvm_riscv_vsstage_tlb_no_gpa)
pub static mut kvm_riscv_vsstage_tlb_no_gpa: bool = false;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
