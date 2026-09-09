/* Translated from kvm_host.h. Included kernel dependencies are external. */

macro_rules! MIPS_CP0_32 { ($r:expr, $s:expr) => { KVM_REG_MIPS_CP0 | KVM_REG_SIZE_U32 | (8 * ($r) + ($s)) }; }
macro_rules! MIPS_CP0_64 { ($r:expr, $s:expr) => { KVM_REG_MIPS_CP0 | KVM_REG_SIZE_U64 | (8 * ($r) + ($s)) }; }

pub const KVM_REG_MIPS_CP0_INDEX: u64 = MIPS_CP0_32!(0, 0);
pub const KVM_REG_MIPS_CP0_ENTRYLO0: u64 = MIPS_CP0_64!(2, 0);
pub const KVM_REG_MIPS_CP0_ENTRYLO1: u64 = MIPS_CP0_64!(3, 0);
pub const KVM_REG_MIPS_CP0_CONTEXT: u64 = MIPS_CP0_64!(4, 0);
pub const KVM_REG_MIPS_CP0_CONTEXTCONFIG: u64 = MIPS_CP0_32!(4, 1);
pub const KVM_REG_MIPS_CP0_USERLOCAL: u64 = MIPS_CP0_64!(4, 2);
pub const KVM_REG_MIPS_CP0_XCONTEXTCONFIG: u64 = MIPS_CP0_64!(4, 3);
pub const KVM_REG_MIPS_CP0_PAGEMASK: u64 = MIPS_CP0_32!(5, 0);
pub const KVM_REG_MIPS_CP0_PAGEGRAIN: u64 = MIPS_CP0_32!(5, 1);
pub const KVM_REG_MIPS_CP0_SEGCTL0: u64 = MIPS_CP0_64!(5, 2);
pub const KVM_REG_MIPS_CP0_SEGCTL1: u64 = MIPS_CP0_64!(5, 3);
pub const KVM_REG_MIPS_CP0_SEGCTL2: u64 = MIPS_CP0_64!(5, 4);
pub const KVM_REG_MIPS_CP0_PWBASE: u64 = MIPS_CP0_64!(5, 5);
pub const KVM_REG_MIPS_CP0_PWFIELD: u64 = MIPS_CP0_64!(5, 6);
pub const KVM_REG_MIPS_CP0_PWSIZE: u64 = MIPS_CP0_64!(5, 7);
pub const KVM_REG_MIPS_CP0_WIRED: u64 = MIPS_CP0_32!(6, 0);
pub const KVM_REG_MIPS_CP0_PWCTL: u64 = MIPS_CP0_32!(6, 6);
pub const KVM_REG_MIPS_CP0_HWRENA: u64 = MIPS_CP0_32!(7, 0);
pub const KVM_REG_MIPS_CP0_BADVADDR: u64 = MIPS_CP0_64!(8, 0);
pub const KVM_REG_MIPS_CP0_BADINSTR: u64 = MIPS_CP0_32!(8, 1);
pub const KVM_REG_MIPS_CP0_BADINSTRP: u64 = MIPS_CP0_32!(8, 2);
pub const KVM_REG_MIPS_CP0_COUNT: u64 = MIPS_CP0_32!(9, 0);
pub const KVM_REG_MIPS_CP0_ENTRYHI: u64 = MIPS_CP0_64!(10, 0);
pub const KVM_REG_MIPS_CP0_COMPARE: u64 = MIPS_CP0_32!(11, 0);
pub const KVM_REG_MIPS_CP0_STATUS: u64 = MIPS_CP0_32!(12, 0);
pub const KVM_REG_MIPS_CP0_INTCTL: u64 = MIPS_CP0_32!(12, 1);
pub const KVM_REG_MIPS_CP0_CAUSE: u64 = MIPS_CP0_32!(13, 0);
pub const KVM_REG_MIPS_CP0_EPC: u64 = MIPS_CP0_64!(14, 0);
pub const KVM_REG_MIPS_CP0_PRID: u64 = MIPS_CP0_32!(15, 0);
pub const KVM_REG_MIPS_CP0_EBASE: u64 = MIPS_CP0_64!(15, 1);
pub const KVM_REG_MIPS_CP0_CONFIG: u64 = MIPS_CP0_32!(16, 0);
pub const KVM_REG_MIPS_CP0_CONFIG1: u64 = MIPS_CP0_32!(16, 1);
pub const KVM_REG_MIPS_CP0_CONFIG2: u64 = MIPS_CP0_32!(16, 2);
pub const KVM_REG_MIPS_CP0_CONFIG3: u64 = MIPS_CP0_32!(16, 3);
pub const KVM_REG_MIPS_CP0_CONFIG4: u64 = MIPS_CP0_32!(16, 4);
pub const KVM_REG_MIPS_CP0_CONFIG5: u64 = MIPS_CP0_32!(16, 5);
pub const KVM_REG_MIPS_CP0_CONFIG6: u64 = MIPS_CP0_32!(16, 6);
pub const KVM_REG_MIPS_CP0_CONFIG7: u64 = MIPS_CP0_32!(16, 7);
pub const KVM_REG_MIPS_CP0_MAARI: u64 = MIPS_CP0_64!(17, 2);
pub const KVM_REG_MIPS_CP0_XCONTEXT: u64 = MIPS_CP0_64!(20, 0);
pub const KVM_REG_MIPS_CP0_DIAG: u64 = MIPS_CP0_32!(22, 0);
pub const KVM_REG_MIPS_CP0_ERROREPC: u64 = MIPS_CP0_64!(30, 0);
pub const KVM_REG_MIPS_CP0_KSCRATCH1: u64 = MIPS_CP0_64!(31, 2);
pub const KVM_REG_MIPS_CP0_KSCRATCH2: u64 = MIPS_CP0_64!(31, 3);
pub const KVM_REG_MIPS_CP0_KSCRATCH3: u64 = MIPS_CP0_64!(31, 4);
pub const KVM_REG_MIPS_CP0_KSCRATCH4: u64 = MIPS_CP0_64!(31, 5);
pub const KVM_REG_MIPS_CP0_KSCRATCH5: u64 = MIPS_CP0_64!(31, 6);
pub const KVM_REG_MIPS_CP0_KSCRATCH6: u64 = MIPS_CP0_64!(31, 7);

pub const KVM_MAX_VCPUS: usize = 16;
pub const KVM_HALT_POLL_NS_DEFAULT: u64 = 500000;
pub static mut GUESTID_MASK: usize = 0;
pub static mut GUESTID_FIRST_VERSION: usize = 0;
pub static mut GUESTID_VERSION_MASK: usize = 0;
pub const KVM_INVALID_ADDR: usize = 0xdeadbeef;
pub const KVM_HVA_ERR_BAD: usize = (!0usize);
pub const KVM_HVA_ERR_RO_BAD: usize = (!1usize);

#[inline] pub unsafe fn kvm_is_error_hva(addr: usize) -> bool { IS_ERR_VALUE(addr) }

#[repr(C)] pub struct kvm_vm_stat { pub generic: kvm_vm_stat_generic }
#[repr(C)] pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub wait_exits: u64, pub cache_exits: u64, pub signal_exits: u64, pub int_exits: u64,
    pub cop_unusable_exits: u64, pub tlbmod_exits: u64, pub tlbmiss_ld_exits: u64,
    pub tlbmiss_st_exits: u64, pub addrerr_st_exits: u64, pub addrerr_ld_exits: u64,
    pub syscall_exits: u64, pub resvd_inst_exits: u64, pub break_inst_exits: u64,
    pub trap_inst_exits: u64, pub msa_fpe_exits: u64, pub fpe_exits: u64,
    pub msa_disabled_exits: u64, pub flush_dcache_exits: u64, pub vz_gpsi_exits: u64,
    pub vz_gsfc_exits: u64, pub vz_hc_exits: u64, pub vz_grr_exits: u64, pub vz_gva_exits: u64,
    pub vz_ghfc_exits: u64, pub vz_gpa_exits: u64, pub vz_resvd_exits: u64,
    // CONFIG_CPU_LOONGSON64: vz_cpucfg_exits: u64
}
#[repr(C)] pub struct kvm_arch_memory_slot {}

#[repr(C)] pub struct kvm_arch {
    pub gpa_mm: mm_struct,
    pub asid_flush_mask: cpumask_t,
    // CONFIG_CPU_LOONGSON64: pub ipi: loongson_kvm_ipi,
}
pub const N_MIPS_COPROC_REGS: usize = 32;
pub const N_MIPS_COPROC_SEL: usize = 8;
#[repr(C)] pub struct mips_coproc { pub reg: [[usize; N_MIPS_COPROC_SEL]; N_MIPS_COPROC_REGS] }

pub const MIPS_CP0_TLB_INDEX: usize = 0; pub const MIPS_CP0_TLB_RANDOM: usize = 1;
pub const MIPS_CP0_TLB_LOW: usize = 2; pub const MIPS_CP0_TLB_LO0: usize = 2; pub const MIPS_CP0_TLB_LO1: usize = 3;
pub const MIPS_CP0_TLB_CONTEXT: usize = 4; pub const MIPS_CP0_TLB_PG_MASK: usize = 5; pub const MIPS_CP0_TLB_WIRED: usize = 6;
pub const MIPS_CP0_HWRENA: usize = 7; pub const MIPS_CP0_BAD_VADDR: usize = 8; pub const MIPS_CP0_COUNT: usize = 9;
pub const MIPS_CP0_TLB_HI: usize = 10; pub const MIPS_CP0_COMPARE: usize = 11; pub const MIPS_CP0_STATUS: usize = 12;
pub const MIPS_CP0_CAUSE: usize = 13; pub const MIPS_CP0_EXC_PC: usize = 14; pub const MIPS_CP0_PRID: usize = 15;
pub const MIPS_CP0_CONFIG: usize = 16; pub const MIPS_CP0_LLADDR: usize = 17; pub const MIPS_CP0_WATCH_LO: usize = 18;
pub const MIPS_CP0_WATCH_HI: usize = 19; pub const MIPS_CP0_TLB_XCONTEXT: usize = 20; pub const MIPS_CP0_DIAG: usize = 22;
pub const MIPS_CP0_ECC: usize = 26; pub const MIPS_CP0_CACHE_ERR: usize = 27; pub const MIPS_CP0_TAG_LO: usize = 28;
pub const MIPS_CP0_TAG_HI: usize = 29; pub const MIPS_CP0_ERROR_PC: usize = 30; pub const MIPS_CP0_DEBUG: usize = 23;
pub const MIPS_CP0_DEPC: usize = 24; pub const MIPS_CP0_PERFCNT: usize = 25; pub const MIPS_CP0_ERRCTL: usize = 26;
pub const MIPS_CP0_DATA_LO: usize = 28; pub const MIPS_CP0_DATA_HI: usize = 29; pub const MIPS_CP0_DESAVE: usize = 31;
pub const MIPS_CP0_CONFIG_SEL: usize = 0; pub const MIPS_CP0_CONFIG1_SEL: usize = 1; pub const MIPS_CP0_CONFIG2_SEL: usize = 2;
pub const MIPS_CP0_CONFIG3_SEL: usize = 3; pub const MIPS_CP0_CONFIG4_SEL: usize = 4; pub const MIPS_CP0_CONFIG5_SEL: usize = 5;
pub const MIPS_CP0_GUESTCTL2: usize = 10; pub const MIPS_CP0_GUESTCTL2_SEL: usize = 5;
pub const MIPS_CP0_GTOFFSET: usize = 12; pub const MIPS_CP0_GTOFFSET_SEL: usize = 7;

pub const RESUME_FLAG_DR: u32 = 1 << 0; pub const RESUME_FLAG_HOST: u32 = 1 << 1;
pub const RESUME_GUEST: u32 = 0; pub const RESUME_GUEST_DR: u32 = RESUME_FLAG_DR; pub const RESUME_HOST: u32 = RESUME_FLAG_HOST;
#[repr(C)] pub enum emulation_result { EMULATE_DONE, EMULATE_DO_MMIO, EMULATE_FAIL, EMULATE_WAIT, EMULATE_PRIV_FAIL, EMULATE_EXCEPT, EMULATE_HYPERCALL }

#[cfg(target_pointer_width = "64")] pub const VPN2_MASK: usize = GENMASK(cpu_vmbits - 1, 13);
#[cfg(not(target_pointer_width = "64"))] pub const VPN2_MASK: usize = 0xffffe000;
pub const KVM_MIPS_AUX_FPU: u32 = 0x1; pub const KVM_MIPS_AUX_MSA: u32 = 0x2;
#[repr(C)] pub struct kvm_mips_tlb { pub tlb_mask: isize, pub tlb_hi: isize, pub tlb_lo: [isize; 2] }

#[repr(C)] pub struct kvm_vcpu_arch {
    pub guest_ebase: *mut core::ffi::c_void, pub vcpu_run: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub host_stack: usize, pub host_gp: usize, pub host_pgd: usize, pub host_entryhi: usize,
    pub host_cp0_badvaddr: usize, pub host_cp0_epc: usize, pub host_cp0_cause: u32, pub host_cp0_guestctl0: u32,
    pub host_cp0_badinstr: u32, pub host_cp0_badinstrp: u32, pub gprs: [usize; 32], pub hi: usize, pub lo: usize, pub pc: usize,
    pub fpu: mips_fpu_struct, pub aux_inuse: u32, pub cop0: mips_coproc, pub io_pc: usize, pub io_gpr: u32,
    pub comparecount_timer: hrtimer, pub count_ctl: u32, pub count_bias: u32, pub count_hz: u32, pub count_dyn_bias: i64,
    pub count_resume: ktime_t, pub count_period: u64, pub pending_exceptions: usize, pub pending_exceptions_clr: usize,
    pub mmu_page_cache: kvm_mmu_memory_cache, pub vzguestid: [u32; NR_CPUS], pub wired_tlb: *mut kvm_mips_tlb,
    pub wired_tlb_limit: u32, pub wired_tlb_used: u32, pub maar: [usize; 6], pub last_sched_cpu: i32,
    pub last_exec_cpu: i32, pub wait: i32, pub fpu_enabled: u8, pub msa_enabled: u8,
}

/* The C accessor-generator macros are retained as Rust macro placeholders; their generated functions depend on external CP0 bindings. */
macro_rules! __BUILD_KVM_RW_SAVED { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SET_SAVED { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_ATOMIC_SAVED { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_RW_VZ { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SET_VZ { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SAVE_VZ { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_RW_WRAP { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SET_WRAP { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_RW_SW { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SET_SW { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_ATOMIC_SW { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_RW_HW { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_SET_HW { ($($tt:tt)*) => {} }
macro_rules! __BUILD_KVM_ATOMIC_HW { ($($tt:tt)*) => {} }

#[inline] pub unsafe fn kvm_mips_guest_can_have_fpu(vcpu: *mut kvm_vcpu_arch) -> bool { (!__builtin_constant_p(raw_cpu_has_fpu) || raw_cpu_has_fpu) && (*vcpu).fpu_enabled != 0 }
#[inline] pub unsafe fn kvm_mips_guest_has_fpu(vcpu: *mut kvm_vcpu_arch) -> bool { kvm_mips_guest_can_have_fpu(vcpu) && kvm_read_c0_guest_config1(&(*vcpu).cop0) & MIPS_CONF1_FP != 0 }
#[inline] pub unsafe fn kvm_mips_guest_can_have_msa(vcpu: *mut kvm_vcpu_arch) -> bool { (!__builtin_constant_p(cpu_has_msa) || cpu_has_msa) && (*vcpu).msa_enabled != 0 }
#[inline] pub unsafe fn kvm_mips_guest_has_msa(vcpu: *mut kvm_vcpu_arch) -> bool { kvm_mips_guest_can_have_msa(vcpu) && kvm_read_c0_guest_config3(&(*vcpu).cop0) & MIPS_CONF3_MSA != 0 }

#[repr(C)] pub struct kvm_mips_callbacks {
    pub handle_cop_unusable: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_tlb_mod: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_tlb_ld_miss: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_tlb_st_miss: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_addr_err_st: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_addr_err_ld: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_syscall: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_res_inst: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_break: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_trap: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_msa_fpe: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_fpe: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub handle_msa_disabled: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub handle_guest_exit: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub enable_virtualization_cpu: Option<unsafe extern "C" fn() -> i32>, pub disable_virtualization_cpu: Option<unsafe extern "C" fn()>,
    pub check_extension: Option<unsafe extern "C" fn(*mut kvm, isize) -> i32>, pub vcpu_init: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub vcpu_uninit: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub vcpu_setup: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    pub prepare_flush_shadow: Option<unsafe extern "C" fn(*mut kvm)>, pub gva_to_gpa: Option<unsafe extern "C" fn(gva_t) -> gpa_t>,
    pub queue_timer_int: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub dequeue_timer_int: Option<unsafe extern "C" fn(*mut kvm_vcpu)>,
    pub queue_io_int: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut kvm_mips_interrupt)>, pub dequeue_io_int: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut kvm_mips_interrupt)>,
    pub irq_deliver: Option<unsafe extern "C" fn(*mut kvm_vcpu, u32, u32) -> i32>, pub irq_clear: Option<unsafe extern "C" fn(*mut kvm_vcpu, u32, u32) -> i32>,
    pub num_regs: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> usize>, pub copy_reg_indices: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut u64) -> i32>,
    pub get_one_reg: Option<unsafe extern "C" fn(*mut kvm_vcpu, *const kvm_one_reg, *mut i64) -> i32>, pub set_one_reg: Option<unsafe extern "C" fn(*mut kvm_vcpu, *const kvm_one_reg, i64) -> i32>,
    pub vcpu_load: Option<unsafe extern "C" fn(*mut kvm_vcpu, i32) -> i32>, pub vcpu_put: Option<unsafe extern "C" fn(*mut kvm_vcpu, i32) -> i32>,
    pub vcpu_run: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>, pub vcpu_reenter: Option<unsafe extern "C" fn(*mut kvm_vcpu)>,
}
extern "C" { pub static kvm_mips_callbacks: *const kvm_mips_callbacks; pub fn kvm_mips_emulation_init() -> i32; pub fn kvm_arch_vcpu_dump_regs(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_mips_handle_exit(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_mips_entry_setup() -> i32; pub fn kvm_mips_build_vcpu_run(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void; pub fn kvm_mips_build_tlb_refill_exception(addr: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> *mut core::ffi::c_void; pub fn kvm_mips_build_exception(addr: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> *mut core::ffi::c_void; pub fn kvm_mips_build_exit(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void; }

#[inline] pub unsafe fn kvm_is_ifetch_fault(vcpu: *mut kvm_vcpu_arch) -> bool { let badvaddr = (*vcpu).host_cp0_badvaddr; let epc = msk_isa16_mode((*vcpu).pc); let cause = (*vcpu).host_cp0_cause; if epc == badvaddr { true } else if (cause & CAUSEF_BD) != 0 && badvaddr.wrapping_sub(epc) <= 4 { true } else { false } }

extern "C" {
    pub fn kvm_mips_complete_mmio_load(*mut kvm_vcpu) -> emulation_result;
    pub fn kvm_mips_read_count(*mut kvm_vcpu) -> u32; pub fn kvm_mips_write_count(*mut kvm_vcpu, u32); pub fn kvm_mips_write_compare(*mut kvm_vcpu, u32, bool); pub fn kvm_mips_init_count(*mut kvm_vcpu, usize); pub fn kvm_mips_set_count_ctl(*mut kvm_vcpu, i64) -> i32; pub fn kvm_mips_set_count_resume(*mut kvm_vcpu, i64) -> i32; pub fn kvm_mips_set_count_hz(*mut kvm_vcpu, i64) -> i32; pub fn kvm_mips_count_enable_cause(*mut kvm_vcpu); pub fn kvm_mips_count_disable_cause(*mut kvm_vcpu); pub fn kvm_mips_count_timeout(*mut kvm_vcpu) -> hrtimer_restart; pub fn kvm_mips_count_disabled(*mut kvm_vcpu) -> i32; pub fn kvm_mips_freeze_hrtimer(*mut kvm_vcpu, *mut u32) -> ktime_t; pub fn kvm_mips_restore_hrtimer(*mut kvm_vcpu, ktime_t, u32, i32) -> i32; pub fn kvm_vz_acquire_htimer(*mut kvm_vcpu); pub fn kvm_vz_lose_htimer(*mut kvm_vcpu); pub fn kvm_mips_emul_wait(*mut kvm_vcpu) -> emulation_result; pub fn kvm_mips_handle_hypcall(*mut kvm_vcpu) -> i32; pub fn kvm_mips_get_ramsize(*mut kvm) -> usize; pub fn kvm_vcpu_ioctl_interrupt(*mut kvm_vcpu, *mut kvm_mips_interrupt) -> i32;
}
pub const __KVM_HAVE_ARCH_FLUSH_REMOTE_TLBS: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
