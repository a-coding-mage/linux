/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of arm64/include/asm/kvm_mmu.h.
 * C includes, preprocessor conditions, and assembly-only macros are retained
 * below as dependency/conditional notes where they have no direct Rust item.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/page.h, asm/memory.h, asm/mmu.h, asm/cpufeature.h, linux/pgtable.h,
// asm/pgalloc.h, asm/cache.h, asm/cacheflush.h, asm/mmu_context.h,
// asm/kvm_emulate.h, asm/kvm_host.h, asm/kvm_nested.h, asm/kvm_pgtable.h,
// asm/stage2_pgtable.h.

extern "C" {
    pub fn kvm_update_va_mask(alt: *mut alt_instr, origptr: *mut __le32,
                              updptr: *mut __le32, nr_inst: c_int);
    pub fn kvm_compute_layout();
    pub fn kvm_hyp_va_bits() -> u32;
    pub fn kvm_apply_hyp_relocations();
    pub static mut __hyp_va_bits: u32;
    pub fn kvm_share_hyp(from: *mut c_void, to: *mut c_void) -> c_int;
    pub fn kvm_unshare_hyp(from: *mut c_void, to: *mut c_void);
    pub fn create_hyp_mappings(from: *mut c_void, to: *mut c_void, prot: kvm_pgtable_prot) -> c_int;
    pub fn __create_hyp_mappings(start: c_ulong, size: c_ulong, phys: c_ulong, prot: kvm_pgtable_prot) -> c_int;
    pub fn hyp_alloc_private_va_range(size: usize, haddr: *mut c_ulong) -> c_int;
    pub fn create_hyp_io_mappings(phys_addr: phys_addr_t, size: usize,
                                  kaddr: *mut *mut c_void, haddr: *mut *mut c_void) -> c_int;
    pub fn create_hyp_exec_mappings(phys_addr: phys_addr_t, size: usize, haddr: *mut *mut c_void) -> c_int;
    pub fn create_hyp_stack(phys_addr: phys_addr_t, haddr: *mut c_ulong) -> c_int;
    pub fn free_hyp_pgds();
    pub fn kvm_stage2_unmap_range(mmu: *mut kvm_s2_mmu, start: phys_addr_t, size: u64, may_block: bool);
    pub fn kvm_stage2_flush_range(mmu: *mut kvm_s2_mmu, addr: phys_addr_t, end: phys_addr_t);
    pub fn kvm_stage2_wp_range(mmu: *mut kvm_s2_mmu, addr: phys_addr_t, end: phys_addr_t);
    pub fn stage2_unmap_vm(kvm: *mut kvm);
    pub fn kvm_init_stage2_mmu(kvm: *mut kvm, mmu: *mut kvm_s2_mmu, typ: c_ulong) -> c_int;
    pub fn kvm_uninit_stage2_mmu(kvm: *mut kvm);
    pub fn kvm_free_stage2_pgd(mmu: *mut kvm_s2_mmu);
    pub fn kvm_phys_addr_ioremap(kvm: *mut kvm, guest_ipa: phys_addr_t, pa: phys_addr_t,
                                 size: c_ulong, writable: bool) -> c_int;
    pub fn kvm_handle_guest_sea(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_handle_guest_abort(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_mmu_get_httbr() -> phys_addr_t;
    pub fn kvm_get_idmap_vector() -> phys_addr_t;
    pub fn kvm_mmu_init(hyp_va_bits: u32) -> c_int;
    pub fn kvm_set_way_flush(vcpu: *mut kvm_vcpu);
    pub fn kvm_toggle_cache(vcpu: *mut kvm_vcpu, was_enabled: bool);
    pub fn kvm_s2_ptdump_create_debugfs(kvm: *mut kvm);
    pub fn kvm_nested_s2_ptdump_create_debugfs(mmu: *mut kvm_s2_mmu);
    pub fn kvm_nested_s2_ptdump_remove_debugfs(mmu: *mut kvm_s2_mmu);
}

pub const KVM_PHYS_SHIFT: u32 = 40;

#[inline(always)]
pub unsafe fn __kern_hyp_va(mut v: c_ulong) -> c_ulong {
    // __KVM_VHE_HYPERVISOR__ omits the alternative assembly sequence.
    v
}

#[inline(always)]
pub unsafe fn kern_hyp_va<T>(v: T) -> T {
    core::mem::transmute_copy(&__kern_hyp_va(v as c_ulong))
}

#[inline(always)]
pub unsafe fn __kvm_vector_slot2addr(base: *mut c_void, slot: arm64_hyp_spectre_vector) -> *mut c_void {
    let idx = slot as c_int - (slot != HYP_VECTOR_DIRECT) as c_int;
    (base as *mut u8).offset((idx as isize) * (SZ_2K as isize)) as *mut c_void
}

#[inline(always)]
pub unsafe fn vcpu_has_cache_enabled(vcpu: *mut kvm_vcpu) -> bool {
    let cache_bits = SCTLR_ELx_M | SCTLR_ELx_C;
    let reg = if vcpu_is_el2(vcpu) { SCTLR_EL2 } else { SCTLR_EL1 };
    (vcpu_read_sys_reg(vcpu, reg) & cache_bits) == cache_bits
}

#[inline(always)]
pub unsafe fn __clean_dcache_guest_page(va: *mut c_void, size: usize) {
    if cpus_have_final_cap(ARM64_HAS_STAGE2_FWB) { return; }
    dcache_clean_inval_poc(va as c_ulong, va as c_ulong + size as c_ulong);
}

#[inline(always)]
pub unsafe fn __invalidate_icache_max_range() -> usize {
    let ctr: u64 = 0; // Filled by the ARM64 alternative instruction sequence.
    let iminline = (SYS_FIELD_GET(CTR_EL0, IminLine, ctr) + 2) as usize;
    (MAX_DVM_OPS as usize) << iminline
}

#[inline(always)]
pub unsafe fn __invalidate_icache_guest_page(va: *mut c_void, size: usize) {
    if icache_is_aliasing() || size > __invalidate_icache_max_range() {
        icache_inval_all_pou();
    } else {
        icache_inval_pou(va as c_ulong, va as c_ulong + size as c_ulong);
    }
}

#[inline(always)]
pub unsafe fn kvm_get_vmid_bits() -> c_uint {
    get_vmid_bits(read_sanitised_ftr_reg(SYS_ID_AA64MMFR1_EL1))
}

#[inline(always)]
pub unsafe fn kvm_read_guest_lock(kvm: *mut kvm, gpa: gpa_t, data: *mut c_void, len: c_ulong) -> c_int {
    let idx = srcu_read_lock(&mut (*kvm).srcu);
    let ret = kvm_read_guest(kvm, gpa, data, len);
    srcu_read_unlock(&mut (*kvm).srcu, idx);
    ret
}

#[inline(always)]
pub unsafe fn kvm_write_guest_lock(kvm: *mut kvm, gpa: gpa_t, data: *const c_void, len: c_ulong) -> c_int {
    let idx = srcu_read_lock(&mut (*kvm).srcu);
    let ret = kvm_write_guest(kvm, gpa, data, len);
    srcu_read_unlock(&mut (*kvm).srcu, idx);
    ret
}

#[inline(always)]
pub unsafe fn get_vmid(vttbr: u64) -> u64 {
    (vttbr & VTTBR_VMID_MASK(kvm_get_vmid_bits())) >> VTTBR_VMID_SHIFT
}

#[inline(always)]
pub unsafe fn kvm_get_vttbr(mmu: *mut kvm_s2_mmu) -> u64 {
    let vmid = &mut (*mmu).vmid;
    let baddr = (*mmu).pgd_phys;
    let mut vmid_field = atomic64_read(&mut vmid.id) << VTTBR_VMID_SHIFT;
    vmid_field &= VTTBR_VMID_MASK(kvm_arm_vmid_bits);
    phys_to_ttbr(baddr) | vmid_field | if system_supports_cnp() { VTTBR_CNP_BIT } else { 0 }
}

#[inline(always)]
pub unsafe fn __load_stage2(mmu: *mut kvm_s2_mmu) {
    write_sysreg((*mmu).vtcr, vtcr_el2);
    write_sysreg(kvm_get_vttbr(mmu), vttbr_el2);
    // ARM errata 1165522 and 1530923: the C alternative emits nop or isb.
}

#[inline(always)]
pub unsafe fn kvm_s2_mmu_to_kvm(mmu: *mut kvm_s2_mmu) -> *mut kvm {
    container_of((*mmu).arch, kvm, arch)
}

#[inline(always)]
pub unsafe fn kvm_fault_lock(kvm: *mut kvm) {
    if is_protected_kvm_enabled() { write_lock(&mut (*kvm).mmu_lock); }
    else { read_lock(&mut (*kvm).mmu_lock); }
}

#[inline(always)]
pub unsafe fn kvm_fault_unlock(kvm: *mut kvm) {
    if is_protected_kvm_enabled() { write_unlock(&mut (*kvm).mmu_lock); }
    else { read_unlock(&mut (*kvm).mmu_lock); }
}

#[inline(always)]
pub unsafe fn kvm_s2_mmu_valid(mmu: *mut kvm_s2_mmu) -> bool { (*mmu).tlb_vttbr & VTTBR_CNP_BIT == 0 }

#[inline(always)]
pub unsafe fn kvm_is_nested_s2_mmu(kvm: *mut kvm, mmu: *mut kvm_s2_mmu) -> bool { &(*kvm).arch.mmu != mmu }

#[inline(always)]
pub unsafe fn kvm_supports_cacheable_pfnmap() -> bool {
    cpus_have_final_cap(ARM64_HAS_STAGE2_FWB) && cpus_have_final_cap(ARM64_HAS_CACHE_DIC)
}

// CONFIG_PTDUMP_STAGE2_DEBUGFS supplies the three debugfs declarations above;
// when disabled, their C inline bodies are empty.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
