/* SPDX-License-Identifier: GPL-2.0 */
/* definition for kernel virtual machines on s390 */

// C header dependencies are supplied by the surrounding translation unit.

pub const KVM_MAX_VCPUS: usize = 255;
pub const KVM_INTERNAL_MEM_SLOTS: usize = 1;
pub const KVM_S390_MANAGES_S390_GUEST: usize = 1;
pub const KVM_NR_IRQCHIPS: usize = 1;
pub const KVM_IRQCHIP_NUM_PINS: usize = 1;
pub const KVM_HALT_POLL_NS_DEFAULT: usize = 50000;

pub const KVM_REQ_ENABLE_IBS: usize = KVM_ARCH_REQ(0);
pub const KVM_REQ_DISABLE_IBS: usize = KVM_ARCH_REQ(1);
pub const KVM_REQ_ICPT_OPEREXC: usize = KVM_ARCH_REQ(2);
pub const KVM_REQ_START_MIGRATION: usize = KVM_ARCH_REQ(3);
pub const KVM_REQ_STOP_MIGRATION: usize = KVM_ARCH_REQ(4);
pub const KVM_REQ_VSIE_RESTART: usize = KVM_ARCH_REQ(5);
pub const KVM_REQ_REFRESH_GUEST_PREFIX: usize = KVM_ARCH_REQ_FLAGS(6, KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP);

#[repr(C)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub exit_userspace: u64, pub exit_null: u64, pub exit_external_request: u64,
    pub exit_io_request: u64, pub exit_external_interrupt: u64, pub exit_stop_request: u64,
    pub exit_validity: u64, pub exit_instruction: u64, pub exit_pei: u64, pub halt_no_poll_steal: u64,
    pub instruction_lctl: u64, pub instruction_lctlg: u64, pub instruction_stctl: u64, pub instruction_stctg: u64,
    pub exit_program_interruption: u64, pub exit_instr_and_program: u64, pub exit_operation_exception: u64,
    pub deliver_ckc: u64, pub deliver_cputm: u64, pub deliver_external_call: u64, pub deliver_emergency_signal: u64,
    pub deliver_service_signal: u64, pub deliver_virtio: u64, pub deliver_stop_signal: u64, pub deliver_prefix_signal: u64,
    pub deliver_restart_signal: u64, pub deliver_program: u64, pub deliver_io: u64, pub deliver_machine_check: u64,
    pub exit_wait_state: u64, pub inject_ckc: u64, pub inject_cputm: u64, pub inject_external_call: u64,
    pub inject_emergency: u64, pub inject_mchk: u64, pub inject_pfault_init: u64, pub inject_program: u64,
    pub inject_restart: u64, pub inject_set_prefix: u64, pub inject_stop_signal: u64,
    pub instruction_epsw: u64, pub instruction_gs: u64, pub instruction_io_other: u64, pub instruction_lpsw: u64,
    pub instruction_lpswe: u64, pub instruction_lpswey: u64, pub instruction_pfmf: u64, pub instruction_ptff: u64,
    pub instruction_sck: u64, pub instruction_sckpf: u64, pub instruction_stidp: u64, pub instruction_spx: u64,
    pub instruction_stpx: u64, pub instruction_stap: u64, pub instruction_iske: u64, pub instruction_ri: u64,
    pub instruction_rrbe: u64, pub instruction_sske: u64, pub instruction_ipte_interlock: u64, pub instruction_stsi: u64,
    pub instruction_stfl: u64, pub instruction_tb: u64, pub instruction_tpi: u64, pub instruction_tprot: u64,
    pub instruction_tsch: u64, pub instruction_sie: u64, pub instruction_essa: u64, pub instruction_sthyi: u64,
    pub instruction_sigp_sense: u64, pub instruction_sigp_sense_running: u64, pub instruction_sigp_external_call: u64,
    pub instruction_sigp_emergency: u64, pub instruction_sigp_cond_emergency: u64, pub instruction_sigp_start: u64,
    pub instruction_sigp_stop: u64, pub instruction_sigp_stop_store_status: u64, pub instruction_sigp_store_status: u64,
    pub instruction_sigp_store_adtl_status: u64, pub instruction_sigp_arch: u64, pub instruction_sigp_prefix: u64,
    pub instruction_sigp_restart: u64, pub instruction_sigp_init_cpu_reset: u64, pub instruction_sigp_cpu_reset: u64,
    pub instruction_sigp_unknown: u64, pub instruction_diagnose_10: u64, pub instruction_diagnose_44: u64,
    pub instruction_diagnose_9c: u64, pub diag_9c_ignored: u64, pub diag_9c_forward: u64,
    pub instruction_diagnose_258: u64, pub instruction_diagnose_308: u64, pub instruction_diagnose_500: u64,
    pub instruction_diagnose_other: u64, pub pfault_sync: u64, pub signal_exits: u64,
}

#[repr(i32)]
pub enum irq_types {
    IRQ_PEND_SET_PREFIX = 0, IRQ_PEND_RESTART, IRQ_PEND_SIGP_STOP,
    IRQ_PEND_IO_ISC_7, IRQ_PEND_IO_ISC_6, IRQ_PEND_IO_ISC_5, IRQ_PEND_IO_ISC_4,
    IRQ_PEND_IO_ISC_3, IRQ_PEND_IO_ISC_2, IRQ_PEND_IO_ISC_1, IRQ_PEND_IO_ISC_0,
    IRQ_PEND_VIRTIO, IRQ_PEND_PFAULT_DONE, IRQ_PEND_PFAULT_INIT, IRQ_PEND_EXT_HOST,
    IRQ_PEND_EXT_SERVICE, IRQ_PEND_EXT_SERVICE_EV, IRQ_PEND_EXT_TIMING, IRQ_PEND_EXT_CPU_TIMER,
    IRQ_PEND_EXT_CLOCK_COMP, IRQ_PEND_EXT_EXTERNAL, IRQ_PEND_EXT_EMERGENCY, IRQ_PEND_EXT_MALFUNC,
    IRQ_PEND_EXT_IRQ_KEY, IRQ_PEND_MCHK_REP, IRQ_PEND_PROG, IRQ_PEND_SVC, IRQ_PEND_MCHK_EX, IRQ_PEND_COUNT,
}

pub const KVM_S390_MAX_VIRTIO_IRQS: usize = 87381;
pub const MCHK_EXTD_BIT: usize = 58; pub const MCHK_DEGR_BIT: usize = 56; pub const MCHK_WARN_BIT: usize = 55;
pub const MCHK_REP_MASK: u64 = (1u64 << MCHK_DEGR_BIT) | (1u64 << MCHK_EXTD_BIT) | (1u64 << MCHK_WARN_BIT);
pub const MCHK_SD_BIT: usize = 63; pub const MCHK_PD_BIT: usize = 62;
pub const MCHK_EX_MASK: u64 = (1u64 << MCHK_SD_BIT) | (1u64 << MCHK_PD_BIT);

pub const FIRQ_LIST_IO_ISC_0: usize = 0; pub const FIRQ_LIST_IO_ISC_1: usize = 1;
pub const FIRQ_LIST_IO_ISC_2: usize = 2; pub const FIRQ_LIST_IO_ISC_3: usize = 3;
pub const FIRQ_LIST_IO_ISC_4: usize = 4; pub const FIRQ_LIST_IO_ISC_5: usize = 5;
pub const FIRQ_LIST_IO_ISC_6: usize = 6; pub const FIRQ_LIST_IO_ISC_7: usize = 7;
pub const FIRQ_LIST_PFAULT: usize = 8; pub const FIRQ_LIST_VIRTIO: usize = 9; pub const FIRQ_LIST_COUNT: usize = 10;
pub const FIRQ_CNTR_IO: usize = 0; pub const FIRQ_CNTR_SERVICE: usize = 1; pub const FIRQ_CNTR_VIRTIO: usize = 2;
pub const FIRQ_CNTR_PFAULT: usize = 3; pub const FIRQ_MAX_COUNT: usize = 4;
pub const KVM_S390_AIS_MODE_ALL: i32 = 0; pub const KVM_S390_AIS_MODE_SINGLE: i32 = 1;
pub const KVM_GUESTDBG_EXIT_PENDING: u32 = 0x10000000;
pub const ASYNC_PF_PER_VCPU: usize = 64;
pub const SIE64_RETURN_NORMAL: i32 = 0; pub const SIE64_RETURN_MCCK: i32 = 1;
pub const IRQ_PEND_EXT_MASK: u64 = 0; // dependent enum mask; preserve as a build-time expression in the C source
pub const IRQ_PEND_IO_MASK: u64 = 0; pub const IRQ_PEND_MCHK_MASK: u64 = 0; pub const IRQ_PEND_EXT_II_MASK: u64 = 0;
pub const KVM_GUESTDBG_VALID_MASK: u32 = KVM_GUESTDBG_ENABLE | KVM_GUESTDBG_SINGLESTEP | KVM_GUESTDBG_USE_HW_BP | KVM_GUESTDBG_EXIT_PENDING;
#[inline] pub unsafe fn guestdbg_enabled(vcpu: *const kvm_vcpu) -> bool { ((*vcpu).guest_debug & KVM_GUESTDBG_ENABLE) != 0 }
#[inline] pub unsafe fn guestdbg_sstep_enabled(vcpu: *const kvm_vcpu) -> bool { ((*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP) != 0 }
#[inline] pub unsafe fn guestdbg_hw_bp_enabled(vcpu: *const kvm_vcpu) -> bool { ((*vcpu).guest_debug & KVM_GUESTDBG_USE_HW_BP) != 0 }
#[inline] pub unsafe fn guestdbg_exit_pending(vcpu: *const kvm_vcpu) -> bool { guestdbg_enabled(vcpu) && ((*vcpu).guest_debug & KVM_GUESTDBG_EXIT_PENDING) != 0 }

#[repr(C)] pub struct kvm_s390_interrupt_info { pub list: list_head, pub type_: u64, pub payload: kvm_s390_irq_payload_union }
#[repr(C)] pub union kvm_s390_irq_payload_union { pub io: kvm_s390_io_info, pub ext: kvm_s390_ext_info, pub pgm: kvm_s390_pgm_info, pub emerg: kvm_s390_emerg_info, pub extcall: kvm_s390_extcall_info, pub prefix: kvm_s390_prefix_info, pub stop: kvm_s390_stop_info, pub mchk: kvm_s390_mchk_info }
#[repr(C)] pub struct kvm_s390_irq_payload { pub io: kvm_s390_io_info, pub ext: kvm_s390_ext_info, pub pgm: kvm_s390_pgm_info, pub emerg: kvm_s390_emerg_info, pub extcall: kvm_s390_extcall_info, pub prefix: kvm_s390_prefix_info, pub stop: kvm_s390_stop_info, pub mchk: kvm_s390_mchk_info }
#[repr(C)] pub struct kvm_s390_local_interrupt { pub lock: spinlock_t, pub sigp_emerg_pending: [u64; (KVM_MAX_VCPUS + 63) / 64], pub irq: kvm_s390_irq_payload, pub pending_irqs: usize }
#[repr(C)] pub struct kvm_s390_float_interrupt { pub pending_irqs: usize, pub masked_irqs: usize, pub lock: spinlock_t, pub lists: [list_head; FIRQ_LIST_COUNT], pub counters: [i32; FIRQ_MAX_COUNT], pub mchk: kvm_s390_mchk_info, pub srv_signal: kvm_s390_ext_info, pub last_sleep_cpu: i32, pub ais_lock: spinlock_t, pub simm: u8, pub nimm: u8 }
#[repr(C)] pub struct kvm_hw_wp_info_arch { pub addr: usize, pub phys_addr: usize, pub len: i32, pub old_data: *mut i8 }
#[repr(C)] pub struct kvm_hw_bp_info_arch { pub addr: usize, pub len: i32 }
#[repr(C)] pub struct kvm_guestdbg_info_arch { pub cr0: usize, pub cr9: usize, pub cr10: usize, pub cr11: usize, pub hw_bp_info: *mut kvm_hw_bp_info_arch, pub hw_wp_info: *mut kvm_hw_wp_info_arch, pub nr_hw_bp: i32, pub nr_hw_wp: i32, pub last_bp: usize }
#[repr(C)] pub struct kvm_s390_pv_vcpu { pub handle: u64, pub stor_base: usize }

#[repr(C)] pub struct kvm_vcpu_arch {
    pub sie_block: *mut kvm_s390_sie_block, pub vsie_block: *mut kvm_s390_sie_block,
    pub host_acrs: [u32; NUM_ACRS], pub host_gscb: *mut gs_cb, pub local_int: kvm_s390_local_interrupt,
    pub ckc_timer: hrtimer, pub pgm: kvm_s390_pgm_info, pub gmap: *mut gmap, pub guestdbg: kvm_guestdbg_info_arch,
    pub pfault_token: usize, pub pfault_select: usize, pub pfault_compare: usize, pub cputm_enabled: bool,
    pub cputm_seqcount: seqcount_t, pub cputm_start: u64, pub gs_enabled: bool, pub skey_enabled: bool,
    pub acrs_loaded: bool, pub initialized: bool, pub pv: kvm_s390_pv_vcpu, pub diag318_info: diag318_info,
    pub mc: *mut kvm_s390_mmu_cache,
}

#[repr(C)] pub struct kvm_vm_stat { pub generic: kvm_vm_stat_generic, pub inject_io: u64, pub io_390_adapter_map: u64, pub io_390_adapter_unmap: u64, pub io_390_inatomic: u64, pub io_flic_inject_airq: u64, pub io_set_adapter_int: u64, pub io_390_inatomic_no_inject: u64, pub inject_float_mchk: u64, pub inject_pfault_done: u64, pub inject_service_signal: u64, pub inject_virtio: u64, pub aen_forward: u64, pub gmap_shadow_create: u64, pub gmap_shadow_reuse: u64, pub gmap_shadow_r1_entry: u64, pub gmap_shadow_r2_entry: u64, pub gmap_shadow_r3_entry: u64, pub gmap_shadow_sg_entry: u64, pub gmap_shadow_pg_entry: u64 }
#[repr(C)] pub struct kvm_arch_memory_slot;
#[repr(C)] pub struct s390_map_info { pub list: list_head, pub guest_addr: u64, pub addr: u64, pub page: *mut page, pub pinned: bool }
#[repr(C)] pub struct s390_io_adapter { pub id: u32, pub isc: i32, pub maskable: bool, pub masked: bool, pub swap: bool, pub suppressible: bool, pub maps_lock: spinlock_t, pub maps: list_head, pub nr_maps: u32 }
pub const MAX_S390_IO_ADAPTERS: usize = (MAX_ISC + 1) * 8; pub const MAX_S390_ADAPTER_MAPS: usize = 256;
pub const S390_ARCH_FAC_LIST_SIZE_BYTE: usize = 1 << 11; pub const S390_ARCH_FAC_LIST_SIZE_U64: usize = S390_ARCH_FAC_LIST_SIZE_BYTE / 8; pub const S390_ARCH_FAC_MASK_SIZE_BYTE: usize = S390_ARCH_FAC_LIST_SIZE_BYTE; pub const S390_ARCH_FAC_MASK_SIZE_U64: usize = S390_ARCH_FAC_MASK_SIZE_BYTE / 8;
#[repr(C)] pub struct kvm_s390_cpu_model { pub fac_mask: [u64; S390_ARCH_FAC_MASK_SIZE_U64], pub subfuncs: kvm_s390_vm_cpu_subfunc, pub fac_list: *mut u64, pub cpuid: u64, pub ibc: u16, pub uv_feat_guest: kvm_s390_vm_cpu_uv_feat }
pub const S390_ARCH_FAC_FORMAT_2: usize = 2;
#[repr(C)] pub union kvm_s390_flcb2_header { pub bytes: [u8; 8], pub header_val: u64 }
#[repr(C)] pub struct kvm_s390_flcb2 { pub header: kvm_s390_flcb2_header, pub facilities: [u64; S390_ARCH_FAC_LIST_SIZE_U64] }
pub type crypto_hook = unsafe extern "C" fn(*mut kvm_vcpu) -> i32;
#[repr(C)] pub struct kvm_s390_crypto { pub crycb: *mut kvm_s390_crypto_cb, pub pqap_hook_rwsem: rw_semaphore, pub pqap_hook: *mut crypto_hook, pub crycbd: u32, pub aes_kw: u8, pub dea_kw: u8, pub apie: u8 }
pub const APCB0_MASK_SIZE: usize = 1; #[repr(C)] pub struct kvm_s390_apcb0 { pub apm: [u64; 1], pub aqm: [u64; 1], pub adm: [u64; 1], pub reserved18: u64 }
pub const APCB1_MASK_SIZE: usize = 4; #[repr(C)] pub struct kvm_s390_apcb1 { pub apm: [u64; 4], pub aqm: [u64; 4], pub adm: [u64; 4], pub reserved60: [u64; 4] }
#[repr(C)] pub struct kvm_s390_crypto_cb { pub apcb0: kvm_s390_apcb0, pub reserved20: [u8; 0x28], pub dea_wrapping_key_mask: [u8; 24], pub aes_wrapping_key_mask: [u8; 32], pub apcb1: kvm_s390_apcb1 }

#[repr(C)] pub struct kvm_s390_gisa { pub words: [u64; 4] }
#[repr(C)] pub struct kvm_s390_gib { pub alert_list_origin: u32, pub reserved01: u32, pub nisc: u8, pub reserved03: [u8; 3], pub reserved04: [u32; 5] }
#[repr(C)] pub struct sie_page2 { pub fac_list: [u64; S390_ARCH_FAC_LIST_SIZE_U64], pub crycb: kvm_s390_crypto_cb, pub gisa: kvm_s390_gisa, pub kvm: *mut kvm, pub reserved928: [u8; 0x6d8] }
pub struct vsie_page;
#[repr(C)] pub struct kvm_s390_vsie { pub mutex: mutex, pub addr_to_page: radix_tree_root, pub page_count: i32, pub next: i32, pub pages: [*mut vsie_page; KVM_MAX_VCPUS] }
#[repr(C)] pub struct kvm_s390_gisa_iam { pub mask: u8, pub ref_lock: spinlock_t, pub ref_count: [u32; MAX_ISC + 1] }
#[repr(C)] pub struct kvm_s390_gisa_interrupt { pub origin: *mut kvm_s390_gisa, pub alert: kvm_s390_gisa_iam, pub timer: hrtimer, pub expires: u64, pub kicked_mask: [u64; (KVM_MAX_VCPUS + 63) / 64] }
#[repr(C)] pub struct kvm_s390_pv { pub handle: u64, pub guest_len: u64, pub stor_base: usize, pub stor_var: *mut core::ffi::c_void, pub dumping: bool, pub set_aside: *mut core::ffi::c_void, pub need_cleanup: list_head, pub mmu_notifier: mmu_notifier, pub import_lock: mutex }
pub struct kvm_s390_mmu_cache;
#[repr(C)] pub struct kvm_arch { pub sca: *mut esca_block, pub dbf: *mut debug_info_t, pub float_int: kvm_s390_float_interrupt, pub flic: *mut kvm_device, pub gmap: *mut gmap, pub mem_limit: usize, pub css_support: i32, pub use_irqchip: i32, pub use_cmma: i32, pub use_pfmfi: i32, pub use_skf: i32, pub use_zpci_interp: i32, pub user_cpu_state_ctrl: i32, pub user_sigp: i32, pub user_stsi: i32, pub user_instr0: i32, pub user_operexec: i32, pub allow_vsie_esamode: i32, pub adapters: [*mut s390_io_adapter; MAX_S390_IO_ADAPTERS], pub ipte_wq: wait_queue_head_t, pub ipte_lock_count: i32, pub ipte_mutex: mutex, pub start_stop_lock: spinlock_t, pub sie_page2: *mut sie_page2, pub model: kvm_s390_cpu_model, pub crypto: kvm_s390_crypto, pub vsie: kvm_s390_vsie, pub epdx: u8, pub epoch: u64, pub migration_mode: i32, pub cmma_dirty_pages: atomic64_t, pub cpu_feat: [u64; (KVM_S390_VM_CPU_FEAT_NR_BITS + 63) / 64], pub idle_mask: [u64; (KVM_MAX_VCPUS + 63) / 64], pub gisa_int: kvm_s390_gisa_interrupt, pub pv: kvm_s390_pv, pub kzdev_list: list_head, pub kzdev_list_lock: spinlock_t, pub mc: *mut kvm_s390_mmu_cache }
pub const KVM_HVA_ERR_BAD: usize = usize::MAX; pub const KVM_HVA_ERR_RO_BAD: usize = usize::MAX - 1;
#[inline] pub unsafe fn kvm_is_error_hva(addr: usize) -> bool { IS_ERR_VALUE(addr) }
#[repr(C)] pub struct kvm_arch_async_pf { pub pfault_token: usize }
extern "C" { pub fn kvm_arch_can_dequeue_async_page_present(vcpu: *mut kvm_vcpu) -> bool; pub fn kvm_arch_async_page_ready(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf); pub fn kvm_arch_async_page_not_present(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf) -> bool; pub fn kvm_arch_async_page_present(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf); pub fn kvm_arch_crypto_clear_masks(kvm: *mut kvm); pub fn kvm_arch_crypto_set_masks(kvm: *mut kvm, apm: *mut usize, aqm: *mut usize, adm: *mut usize); pub fn __sie64a(sie_block_phys: phys_addr_t, sie_block: *mut kvm_s390_sie_block, rsa: *mut u64, gasce: usize) -> i32; pub static mut sie_exit: i8; pub fn kvm_s390_pv_is_protected(kvm: *mut kvm) -> bool; pub fn kvm_s390_pv_cpu_is_protected(vcpu: *mut kvm_vcpu) -> bool; pub fn kvm_s390_enter_exit_sie(scb: *mut kvm_s390_sie_block, gprs: *mut u64, gasce: usize) -> i32; pub fn kvm_s390_gisc_register(kvm: *mut kvm, gisc: u32) -> i32; pub fn kvm_s390_gisc_unregister(kvm: *mut kvm, gisc: u32) -> i32; pub fn kvm_s390_is_gpa_in_memslot(kvm: *mut kvm, gpa: gpa_t) -> bool; pub fn kvm_arch_free_vm(kvm: *mut kvm) }
#[inline] pub unsafe fn sie64a(sie_block: *mut kvm_s390_sie_block, rsa: *mut u64, gasce: usize) -> i32 { __sie64a(virt_to_phys(sie_block), sie_block, rsa, gasce) }
#[repr(C)] pub struct zpci_kvm_hook { pub kvm_register: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut kvm) -> i32>, pub kvm_unregister: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
extern "C" { pub static mut zpci_kvm_hook: zpci_kvm_hook; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
