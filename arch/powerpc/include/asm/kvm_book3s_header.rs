/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of asm/kvm_book3s.h. External kernel types and symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

#[repr(C)]
pub struct kvmppc_bat { pub raw: u64, pub bepi: u32, pub bepi_mask: u32, pub brpn: u32, pub wimg: u8, pub pp: u8, pub vs: bool, pub vp: bool }
#[repr(C)] pub struct kvmppc_sid_map { pub guest_vsid: u64, pub guest_esid: u64, pub host_vsid: u64, pub valid: bool }

pub const SID_MAP_BITS: usize = 9;
pub const SID_MAP_NUM: usize = 1usize << SID_MAP_BITS;
pub const SID_MAP_MASK: usize = SID_MAP_NUM - 1;
#[cfg(CONFIG_PPC_BOOK3S_64)] pub const SID_CONTEXTS: usize = 1;
#[cfg(not(CONFIG_PPC_BOOK3S_64))] pub const SID_CONTEXTS: usize = 128;
#[cfg(not(CONFIG_PPC_BOOK3S_64))] pub const VSID_POOL_SIZE: usize = SID_CONTEXTS * 16;

#[repr(C)] pub struct hpte_cache {
    pub list_pte: hlist_node, pub list_pte_long: hlist_node, pub list_vpte: hlist_node,
    pub list_vpte_long: hlist_node,
    #[cfg(CONFIG_PPC_BOOK3S_64)] pub list_vpte_64k: hlist_node,
    pub rcu_head: rcu_head, pub host_vpn: u64, pub pfn: u64, pub slot: ulong,
    pub pte: kvmppc_pte, pub pagesize: i32,
}

#[repr(C)] pub struct kvmppc_vcore {
    pub n_runnable: i32, pub num_threads: i32, pub entry_exit_map: i32, pub napping_threads: i32,
    pub first_vcpuid: i32, pub pcpu: u16, pub last_cpu: u16, pub vcore_state: u8, pub in_guest: u8,
    pub runnable_threads: [*mut kvm_vcpu; MAX_SMT_THREADS], pub preempt_list: list_head,
    pub lock: spinlock_t, pub wait: rcuwait, pub stoltb_lock: spinlock_t, pub stolen_tb: u64,
    pub preempt_tb: u64, pub runner: *mut kvm_vcpu, pub kvm: *mut kvm, pub tb_offset: u64,
    pub tb_offset_applied: u64, pub lpcr: ulong, pub arch_compat: u32, pub pcr: ulong,
    pub dpdes: ulong, pub vtb: ulong, pub conferring_threads: ulong, pub halt_poll_ns: u32,
    pub online_count: atomic_t,
}

#[repr(C)] pub struct kvmppc_vcpu_book3s {
    pub sid_map: [kvmppc_sid_map; SID_MAP_NUM], pub slb_shadow: [(u64,u64); 64], pub slb_shadow_max: u8,
    pub ibat: [kvmppc_bat; 8], pub dbat: [kvmppc_bat; 8], pub hid: [u64; 6], pub gqr: [u64; 8],
    pub sdr1: u64, pub hior: u64, pub msr_mask: u64, pub vtb: u64,
    #[cfg(CONFIG_PPC_BOOK3S_32)] pub vsid_pool: [u32; VSID_POOL_SIZE],
    #[cfg(CONFIG_PPC_BOOK3S_32)] pub vsid_next: u32,
    #[cfg(not(CONFIG_PPC_BOOK3S_32))] pub proto_vsid_first: u64,
    #[cfg(not(CONFIG_PPC_BOOK3S_32))] pub proto_vsid_max: u64,
    #[cfg(not(CONFIG_PPC_BOOK3S_32))] pub proto_vsid_next: u64,
    pub context_id: [i32; SID_CONTEXTS], pub hior_explicit: bool,
    pub hpte_hash_pte: [hlist_head; HPTEG_HASH_NUM_PTE], pub hpte_hash_pte_long: [hlist_head; HPTEG_HASH_NUM_PTE_LONG],
    pub hpte_hash_vpte: [hlist_head; HPTEG_HASH_NUM_VPTE], pub hpte_hash_vpte_long: [hlist_head; HPTEG_HASH_NUM_VPTE_LONG],
    #[cfg(CONFIG_PPC_BOOK3S_64)] pub hpte_hash_vpte_64k: [hlist_head; HPTEG_HASH_NUM_VPTE_64K],
    pub hpte_cache_count: i32, pub mmu_lock: spinlock_t,
}

pub const VSID_REAL: u64 = 0x07ffffffffc00000; pub const VSID_BAT: u64 = 0x07ffffffffb00000;
pub const VSID_64K: u64 = 0x0800000000000000; pub const VSID_1T: u64 = 0x1000000000000000;
pub const VSID_REAL_DR: u64 = 0x2000000000000000; pub const VSID_REAL_IR: u64 = 0x4000000000000000;
pub const VSID_PR: u64 = 0x8000000000000000;

extern "C" {
    pub fn kvmppc_mmu_pte_flush(vcpu:*mut kvm_vcpu, ea:ulong, ea_mask:ulong);
    pub fn kvmppc_mmu_pte_vflush(vcpu:*mut kvm_vcpu, vp:u64, vp_mask:u64);
    pub fn kvmppc_mmu_pte_pflush(vcpu:*mut kvm_vcpu, pa_start:ulong, pa_end:ulong);
    pub fn kvmppc_set_msr(vcpu:*mut kvm_vcpu, new_msr:u64);
    pub fn kvmppc_mmu_book3s_64_init(vcpu:*mut kvm_vcpu); pub fn kvmppc_mmu_book3s_32_init(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_mmu_book3s_hv_init(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_mmu_map_page(vcpu:*mut kvm_vcpu, pte:*mut kvmppc_pte, iswrite:bool)->i32;
    pub fn kvmppc_mmu_unmap_page(vcpu:*mut kvm_vcpu, pte:*mut kvmppc_pte);
    pub fn kvmppc_mmu_map_segment(vcpu:*mut kvm_vcpu, eaddr:ulong)->i32;
    pub fn kvmppc_mmu_flush_segment(vcpu:*mut kvm_vcpu, eaddr:ulong, seg_size:ulong);
    pub fn kvmppc_mmu_flush_segments(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_book3s_hv_page_fault(vcpu:*mut kvm_vcpu, addr:ulong, status:ulong)->i32;
    pub fn kvmppc_hv_find_lock_hpte(kvm:*mut kvm, eaddr:gva_t, slb_v:ulong, valid:ulong)->i64;
    pub fn kvmppc_hv_emulate_mmio(vcpu:*mut kvm_vcpu, gpa:ulong, ea:gva_t, is_store:i32)->i32;
    pub fn kvmppc_mmu_hpte_cache_map(vcpu:*mut kvm_vcpu, pte:*mut hpte_cache);
    pub fn kvmppc_mmu_hpte_cache_next(vcpu:*mut kvm_vcpu)->*mut hpte_cache;
    pub fn kvmppc_mmu_hpte_cache_free(pte:*mut hpte_cache); pub fn kvmppc_mmu_hpte_destroy(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_mmu_hpte_init(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_mmu_invalidate_pte(vcpu:*mut kvm_vcpu,pte:*mut hpte_cache);
    pub fn kvmppc_mmu_hpte_sysinit()->i32; pub fn kvmppc_mmu_hpte_sysexit(); pub fn kvmppc_mmu_hv_init()->i32;
    pub fn kvmppc_book3s_hcall_implemented(kvm:*mut kvm,hc:ulong)->i32;
}

/* The remaining declarations and inline accessors retain the C ABI through
 * opaque external types; conditional kernel-only dependencies stay conditional. */
extern "C" {
    pub fn kvmppc_book3s_radix_page_fault(vcpu:*mut kvm_vcpu, ea:ulong, dsisr:ulong)->i32;
    pub fn kvmppc_radix_init()->i32; pub fn kvmppc_radix_exit();
    pub fn kvmppc_hv_get_dirty_log_radix(kvm:*mut kvm, memslot:*mut kvm_memory_slot, map:*mut ulong)->i64;
    pub fn kvmppc_hv_get_dirty_log_hpt(kvm:*mut kvm, memslot:*mut kvm_memory_slot, map:*mut ulong)->i64;
    pub fn kvmppc_giveup_fac(vcpu:*mut kvm_vcpu, fac:ulong);
}

#[inline] pub unsafe fn is_kvmppc_resume_guest(r:i32)->bool { r == RESUME_GUEST || r == RESUME_GUEST_NV }
pub const OSI_SC_MAGIC_R3:u32 = 0x113724FA; pub const OSI_SC_MAGIC_R4:u32 = 0x77810F9B;
pub const INS_DCBZ:u32 = 0x7c0007ec; pub const INS_TW:u32 = 0x7fe00008;
pub const SPLIT_HACK_MASK:u32 = 0xff000000; pub const SPLIT_HACK_OFFS:u32 = 0xfb000000;

extern "C" {
    pub fn kvmppc_set_msr_hv(vcpu:*mut kvm_vcpu, msr:u64); pub fn kvmppc_inject_interrupt_hv(vcpu:*mut kvm_vcpu,vec:i32,flags:u64);
    pub fn kvmppc_read_intr()->i64; pub fn kvmhv_nested_init()->i64; pub fn kvmhv_nested_exit();
    pub fn kvmhv_vm_nested_init(kvm:*mut kvm); pub fn kvmhv_set_partition_table(vcpu:*mut kvm_vcpu)->i64;
    pub fn kvmhv_copy_tofrom_guest_nested(vcpu:*mut kvm_vcpu)->i64; pub fn kvmhv_flush_lpid(lpid:u64);
    pub fn kvmhv_set_ptbl_entry(lpid:u64,dw0:u64,dw1:u64); pub fn kvmhv_release_all_nested(kvm:*mut kvm);
    pub fn kvmhv_enter_nested_guest(vcpu:*mut kvm_vcpu)->i64; pub fn kvmhv_do_nested_tlbie(vcpu:*mut kvm_vcpu)->i64;
    pub fn kvmhv_run_single_vcpu(vcpu:*mut kvm_vcpu,time_limit:u64,lpcr:ulong)->i32;
    pub fn kvmhv_nested_page_fault(vcpu:*mut kvm_vcpu)->i64;
    pub fn __kvmhv_nestedv2_reload_ptregs(vcpu:*mut kvm_vcpu,regs:*mut pt_regs)->i32;
    pub fn __kvmhv_nestedv2_mark_dirty_ptregs(vcpu:*mut kvm_vcpu,regs:*mut pt_regs)->i32;
    pub fn __kvmhv_nestedv2_mark_dirty(vcpu:*mut kvm_vcpu,iden:u16)->i32;
    pub fn __kvmhv_nestedv2_cached_reload(vcpu:*mut kvm_vcpu,iden:u16)->i32;
    pub fn kvmppc_entry_trampoline(); pub fn kvmppc_hv_entry_trampoline();
    pub fn kvmppc_alignment_dsisr(vcpu:*mut kvm_vcpu,inst:u32)->u32; pub fn kvmppc_alignment_dar(vcpu:*mut kvm_vcpu,inst:u32)->ulong;
    pub fn kvmppc_h_logical_ci_load(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_h_logical_ci_store(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_h_logical_ci_load(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_get_msr(vcpu:*mut kvm_vcpu)->u64;
}

#[inline] pub unsafe fn kvmhv_is_nestedv2()->bool { false }
#[inline] pub unsafe fn kvmhv_is_nestedv1()->bool { false }
#[inline] pub unsafe fn kvmhv_nestedv2_reload_ptregs(v:*mut kvm_vcpu,r:*mut pt_regs)->i32 { if kvmhv_is_nestedv2(){__kvmhv_nestedv2_reload_ptregs(v,r)}else{0} }
#[inline] pub unsafe fn kvmhv_nestedv2_mark_dirty(v:*mut kvm_vcpu,i:u16)->i32 { if kvmhv_is_nestedv2(){__kvmhv_nestedv2_mark_dirty(v,i)}else{0} }
#[inline] pub unsafe fn kvmhv_nestedv2_cached_reload(v:*mut kvm_vcpu,i:u16)->i32 { if kvmhv_is_nestedv2(){__kvmhv_nestedv2_cached_reload(v,i)}else{0} }
#[inline] pub unsafe fn kvmppc_need_byteswap(v:*mut kvm_vcpu)->bool { (kvmppc_get_msr(v)&MSR_LE)!=(MSR_KERNEL&MSR_LE) }
#[inline] pub unsafe fn kvmppc_supports_magic_page(k:*mut kvm_vcpu)->bool { !is_kvmppc_hv_enabled((*k).kvm) }
extern "C" { pub fn is_kvmppc_hv_enabled(kvm:*mut kvm)->bool; }
#[inline] pub unsafe fn kvmppc_dec_expires_host_tb(v:*mut kvm_vcpu)->u64 { kvmppc_get_dec_expires(v)-kvmppc_get_tb_offset(v) }
extern "C" { pub fn kvmppc_get_dec_expires(v:*mut kvm_vcpu)->u64; pub fn kvmppc_get_tb_offset(v:*mut kvm_vcpu)->u64; }

#[inline] pub unsafe fn kvmppc_pack_vcpu_id(kvm:*mut kvm, id:u32)->u32 {
    let offsets:[u32;8]=[0,4,2,6,1,5,3,7]; let stride=(*kvm).arch.emul_smt_mode;
    let block=(id / KVM_MAX_VCPUS) * (MAX_SMT_THREADS / stride);
    if block >= MAX_SMT_THREADS { return 0; }
    let packed=(id % KVM_MAX_VCPUS)+offsets[block as usize]; if packed >= KVM_MAX_VCPUS { return 0; } packed
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
