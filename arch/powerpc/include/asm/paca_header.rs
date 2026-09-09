/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translation of the PowerPC PACA header. */

#[cfg(target_pointer_width = "64")]
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub static mut local_paca: *mut paca_struct;
}

#[cfg(all(feature = "debug_preempt", feature = "smp"))]
extern "C" {
    pub fn debug_smp_processor_id() -> u32;
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn get_paca() -> *mut paca_struct {
    #[cfg(all(feature = "debug_preempt", feature = "smp"))]
    { let _ = debug_smp_processor_id(); }
    local_paca
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn get_slb_shadow() -> *mut slb_shadow {
    (*get_paca()).slb_shadow_ptr
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct paca_struct {
    #[cfg(feature = "ppc_pseries")]
    pub lppaca_ptr: *mut lppaca,
    #[cfg(target_endian = "big")]
    pub lock_token: u16,
    #[cfg(target_endian = "big")]
    pub paca_index: u16,
    #[cfg(target_endian = "little")]
    pub paca_index: u16,
    #[cfg(target_endian = "little")]
    pub lock_token: u16,
    #[cfg(not(feature = "ppc_kernel_pcrel"))]
    pub kernel_toc: u64,
    pub kernelbase: u64,
    pub kernel_msr: u64,
    pub emergency_sp: *mut c_void,
    pub data_offset: u64,
    pub hw_cpu_id: i16,
    pub cpu_start: u8,
    pub kexec_state: u8,
    #[cfg(feature = "ppc_book3s_64")]
    #[cfg(feature = "ppc_64s_hash_mmu")]
    pub slb_shadow_ptr: *mut slb_shadow,
    #[cfg(feature = "ppc_book3s_64")]
    pub dispatch_log: *mut dtl_entry,
    #[cfg(feature = "ppc_book3s_64")]
    pub dispatch_log_end: *mut dtl_entry,
    pub dscr_default: u64,
    #[cfg(feature = "ppc_book3s_64")]
    pub exgen: [u64; EX_SIZE],
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub vmalloc_sllp: u16,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub slb_cache_ptr: u8,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub stab_rr: u8,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu", feature = "debug_vm"))]
    pub in_kernel_slb_handler: u8,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub slb_used_bitmap: u32,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub slb_kern_bitmap: u32,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub slb_cache: [u32; SLB_CACHE_ENTRIES],
    #[cfg(feature = "ppc_book3e_64")]
    pub exgen_3e: [u64; 8],
    #[cfg(feature = "ppc_book3e_64")]
    pub pgd: *mut pgd_t,
    #[cfg(feature = "ppc_book3e_64")]
    pub kernel_pgd: *mut pgd_t,
    #[cfg(feature = "ppc_book3e_64")]
    pub tcd_ptr: *mut tlb_core_data,
    #[cfg(feature = "ppc_book3e_64")]
    pub extlb: [[u64; EX_TLB_SIZE / core::mem::size_of::<u64>()]; 12],
    #[cfg(feature = "ppc_book3e_64")]
    pub exmc_3e: [u64; 8],
    #[cfg(feature = "ppc_book3e_64")]
    pub excrit: [u64; 8],
    #[cfg(feature = "ppc_book3e_64")]
    pub exdbg: [u64; 8],
    #[cfg(feature = "ppc_book3e_64")]
    pub mc_kstack: *mut c_void,
    #[cfg(feature = "ppc_book3e_64")]
    pub crit_kstack: *mut c_void,
    #[cfg(feature = "ppc_book3e_64")]
    pub dbg_kstack: *mut c_void,
    #[cfg(feature = "ppc_book3e_64")]
    pub tcd: tlb_core_data,
    #[cfg(feature = "ppc_64s_hash_mmu")]
    pub mm_ctx_low_slices_psize: [u8; BITS_PER_LONG / BITS_PER_BYTE],
    #[cfg(feature = "ppc_64s_hash_mmu")]
    pub mm_ctx_high_slices_psize: [u8; SLICE_ARRAY_SIZE],
    pub __current: *mut task_struct,
    pub kstack: u64,
    pub saved_r1: u64,
    pub saved_msr: u64,
    pub exit_save_r1: u64,
    #[cfg(feature = "ppc_book3e_64")]
    pub trap_save: u16,
    #[cfg(feature = "ppc_book3s_64")]
    pub hsrr_valid: u8,
    #[cfg(feature = "ppc_book3s_64")]
    pub srr_valid: u8,
    pub irq_soft_mask: u8,
    pub irq_happened: u8,
    pub irq_work_pending: u8,
    #[cfg(feature = "kvm_book3s_hv_possible")]
    pub pmcregs_in_use: u8,
    pub sprg_vdso: u64,
    #[cfg(feature = "ppc_transactional_mem")]
    pub tm_scratch: u64,
    #[cfg(feature = "ppc_powernv")]
    pub idle_lock: usize,
    #[cfg(feature = "ppc_powernv")]
    pub idle_state: usize,
    #[cfg(feature = "ppc_powernv")]
    pub idle_fields: paca_idle_fields,
    #[cfg(feature = "ppc_book3s_64")]
    pub exnmi: [u64; EX_SIZE],
    #[cfg(feature = "ppc_book3s_64")]
    pub exmc: [u64; EX_SIZE],
    #[cfg(feature = "ppc_book3s_64")]
    pub nmi_emergency_sp: *mut c_void,
    #[cfg(feature = "ppc_book3s_64")]
    pub mc_emergency_sp: *mut c_void,
    #[cfg(feature = "ppc_book3s_64")]
    pub in_nmi: u16,
    #[cfg(feature = "ppc_book3s_64")]
    pub in_mce: u16,
    #[cfg(feature = "ppc_book3s_64")]
    pub hmi_event_available: u8,
    #[cfg(feature = "ppc_book3s_64")]
    pub hmi_p9_special_emu: u8,
    #[cfg(feature = "ppc_book3s_64")]
    pub hmi_irqs: u32,
    pub ftrace_enabled: u8,
    pub accounting: cpu_accounting_data,
    pub dtl_ridx: u64,
    pub dtl_curr: *mut dtl_entry,
    #[cfg(all(feature = "kvm_book3s_handler", feature = "kvm_book3s_pr_possible"))]
    pub shadow_vcpu: kvmppc_book3s_shadow_vcpu,
    #[cfg(feature = "kvm_book3s_handler")]
    pub kvm_hstate: kvmppc_host_state,
    #[cfg(all(feature = "kvm_book3s_handler", feature = "kvm_book3s_hv_possible"))]
    pub sibling_subcore_state: *mut sibling_subcore_state,
    #[cfg(feature = "ppc_book3s_64")]
    pub exrfi: [u64; EX_SIZE],
    #[cfg(feature = "ppc_book3s_64")]
    pub rfi_flush_fallback_area: *mut c_void,
    #[cfg(feature = "ppc_book3s_64")]
    pub l1d_flush_size: u64,
    #[cfg(feature = "ppc_pseries")]
    pub mce_data_buf: *mut u8,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub mce_faulty_slbs: *mut slb_entry,
    #[cfg(all(feature = "ppc_book3s_64", feature = "ppc_64s_hash_mmu"))]
    pub slb_save_cache_ptr: u16,
    #[cfg(feature = "stackprotector")]
    pub canary: usize,
    #[cfg(feature = "mmiowb")]
    pub mmiowb_state: mmiowb_state,
    #[cfg(feature = "ppc_book3s_64")]
    pub mce_info: *mut mce_info,
    #[cfg(feature = "ppc_book3s_64")]
    pub mce_pending_irq_work: u8,
}

#[cfg(feature = "ppc_powernv")]
#[repr(C)]
pub union paca_idle_fields {
    pub p7_p8: paca_idle_p7_p8,
    pub p9: paca_idle_p9,
}
#[cfg(feature = "ppc_powernv")]
#[repr(C)] pub struct paca_idle_p7_p8 { pub thread_idle_state: u8, pub subcore_sibling_mask: u8 }
#[cfg(feature = "ppc_powernv")]
#[repr(C)] pub struct paca_idle_p9 {
    #[cfg(feature = "kvm_book3s_hv_possible")] pub requested_psscr: u64,
    #[cfg(feature = "kvm_book3s_hv_possible")] pub dont_stop: atomic_t,
}

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub fn copy_mm_to_paca(mm: *mut mm_struct);
    pub static mut paca_ptrs: *mut *mut paca_struct;
    pub fn initialise_paca(new_paca: *mut paca_struct, cpu: i32);
    pub fn setup_paca(new_paca: *mut paca_struct);
    pub fn allocate_paca_ptrs();
    pub fn allocate_paca(cpu: i32);
    pub fn free_unused_pacas();
}

#[cfg(not(target_pointer_width = "64"))]
#[inline] pub fn allocate_paca(_cpu: i32) {}
#[cfg(not(target_pointer_width = "64"))]
#[inline] pub fn free_unused_pacas() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
