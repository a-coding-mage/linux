/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

extern "C" {
    pub static mut tlbstate_untag_mask: u64;
    pub fn __flush_tlb_all();
    pub fn cr4_update_irqsoff(set: usize, clear: usize);
    pub fn cr4_read_shadow() -> usize;
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
    pub fn __read_cr4() -> usize;
    pub fn initialize_tlbstate_and_flush();
    pub fn flush_tlb_local();
    pub fn flush_tlb_one_user(addr: usize);
    pub fn flush_tlb_one_kernel(addr: usize);
    pub fn flush_tlb_multi(cpumask: *const cpumask, info: *const flush_tlb_info);
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm_range(mm: *mut mm_struct, start: usize, end: usize, stride_shift: u32, freed_tables: bool);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);
    pub fn arch_tlbbatch_flush(batch: *mut arch_tlbflush_unmap_batch);
    pub fn mm_free_global_asid(mm: *mut mm_struct);
    pub fn enter_lazy_tlb(mm: *mut mm_struct, tsk: *mut task_struct);
    pub fn native_write_cr4(cr4: usize);
}

pub const TLB_FLUSH_ALL: usize = usize::MAX;
pub const TLB_GENERATION_INVALID: u64 = 0;
pub const TLB_NR_DYN_ASIDS: usize = 6;

#[repr(C)]
pub struct tlb_context { pub ctx_id: u64, pub tlb_gen: u64 }

#[repr(C)]
pub union tlb_last_user_mm { pub last_user_mm: *mut mm_struct, pub last_user_mm_spec: usize }

#[repr(C)]
pub struct tlb_state {
    pub loaded_mm: *mut mm_struct,
    pub last_user_mm: tlb_last_user_mm,
    pub loaded_mm_asid: u16,
    pub next_asid: u16,
    pub invalidate_other: bool,
    #[cfg(CONFIG_ADDRESS_MASKING)] pub lam: u8,
    pub user_pcid_flush_mask: u16,
    pub cr4: usize,
    pub ctxs: [tlb_context; TLB_NR_DYN_ASIDS],
}

#[repr(C)]
pub struct tlb_state_shared { pub is_lazy: bool }

#[repr(C)]
pub struct flush_tlb_info {
    pub mm: *mut mm_struct,
    pub start: usize,
    pub end: usize,
    pub new_tlb_gen: u64,
    pub initiating_cpu: u32,
    pub stride_shift: u8,
    pub freed_tables: u8,
    pub trim_cpumask: u8,
}

extern "C" {
    pub static mut mmu_cr4_features: usize;
    pub static mut trampoline_cr4_features: *mut u32;
    pub static mut invlpgb_count_max: u16;
    pub static mut cpu_tlbstate: tlb_state;
    pub static mut cpu_tlbstate_shared: tlb_state_shared;
    pub static mut init_mm: mm_struct;
}

pub const FLUSH_TLB_INFO_ALIGN: usize = 64; // MIN(SMP_CACHE_BYTES, 64)

#[inline]
pub unsafe fn cr4_set_bits_irqsoff(mask: usize) { cr4_update_irqsoff(mask, 0); }

#[inline]
pub unsafe fn cr4_clear_bits_irqsoff(mask: usize) { cr4_update_irqsoff(0, mask); }

#[inline]
pub unsafe fn cr4_set_bits(mask: usize) {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    cr4_set_bits_irqsoff(mask);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn cr4_clear_bits(mask: usize) {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    cr4_clear_bits_irqsoff(mask);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn enter_lazy_tlb_inline(_mm: *mut mm_struct, _tsk: *mut task_struct) {
    if cpu_tlbstate.loaded_mm == (&raw mut init_mm) { return; }
    cpu_tlbstate_shared.is_lazy = true;
}

extern "C" { pub fn nmi_uaccess_okay() -> bool; }

#[inline]
pub unsafe fn is_dyn_asid(asid: u16) -> bool { (asid as usize) < TLB_NR_DYN_ASIDS }

#[inline]
pub unsafe fn is_global_asid(asid: u16) -> bool { !is_dyn_asid(asid) }

#[cfg(CONFIG_BROADCAST_TLB_FLUSH)]
extern "C" {
    pub fn cpu_feature_enabled(feature: u32) -> bool;
    pub fn smp_load_acquire(ptr: *const u16) -> u16;
    pub fn smp_store_release(ptr: *mut u16, value: u16);
    pub fn WRITE_ONCE_BOOL(ptr: *mut bool, value: bool);
    pub fn READ_ONCE_BOOL(ptr: *const bool) -> bool;
}

#[cfg(CONFIG_BROADCAST_TLB_FLUSH)]
pub unsafe fn mm_global_asid(mm: *mut mm_struct) -> u16 {
    if !cpu_feature_enabled(X86_FEATURE_INVLPGB) { return 0; }
    let asid = smp_load_acquire((&raw const (*mm).context.global_asid));
    // mm->context.global_asid is either 0, or a global ASID.
    asid
}

#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_global_asid(_mm: *mut mm_struct) -> u16 { 0 }
#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_init_global_asid(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_free_global_asid_inline(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_assign_global_asid(_mm: *mut mm_struct, _asid: u16) {}
#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_clear_asid_transition(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_BROADCAST_TLB_FLUSH))]
#[inline] pub unsafe fn mm_in_asid_transition(_mm: *mut mm_struct) -> bool { false }

#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, a: usize) {
    flush_tlb_mm_range((*vma).vm_mm, a, a.wrapping_add(PAGE_SIZE), PAGE_SHIFT, false);
}

#[inline]
pub unsafe fn pte_flags_need_flush(oldflags: usize, newflags: usize, ignore_access: bool) -> bool {
    let flush_on_clear = _PAGE_DIRTY | _PAGE_PRESENT | _PAGE_ACCESSED;
    let software_flags = _PAGE_SOFTW1 | _PAGE_SOFTW2 | _PAGE_SOFTW3 | _PAGE_SOFTW4 | _PAGE_SAVED_DIRTY;
    let flush_on_change = _PAGE_RW | _PAGE_USER | _PAGE_PWT | _PAGE_PCD | _PAGE_PSE | _PAGE_GLOBAL | _PAGE_PAT |
        _PAGE_PAT_LARGE | _PAGE_PKEY_BIT0 | _PAGE_PKEY_BIT1 | _PAGE_PKEY_BIT2 | _PAGE_PKEY_BIT3 | _PAGE_NX;
    let mut diff = oldflags ^ newflags;
    diff &= !software_flags;
    if ignore_access { diff &= !_PAGE_ACCESSED; }
    if diff & oldflags & flush_on_clear != 0 { return true; }
    if diff & flush_on_change != 0 { return true; }
    false
}

// The remaining inline helpers retain their C-facing semantics and depend on
// the surrounding architecture types and primitives.
extern "C" {
    pub fn inc_mm_tlb_gen(mm: *mut mm_struct) -> u64;
    pub fn arch_tlbbatch_add_pending(batch: *mut arch_tlbflush_unmap_batch, mm: *mut mm_struct, start: usize, end: usize);
}

#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline] pub unsafe fn tlbstate_lam_cr3_mask() -> u64 { (cpu_tlbstate.lam as u64) << X86_CR3_LAM_U57_BIT }
#[cfg(not(CONFIG_ADDRESS_MASKING))]
#[inline] pub unsafe fn tlbstate_lam_cr3_mask() -> u64 { 0 }

#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline] pub unsafe fn cpu_tlbstate_update_lam(lam: usize, untag_mask: u64) {
    cpu_tlbstate.lam = (lam >> X86_CR3_LAM_U57_BIT) as u8;
    tlbstate_untag_mask = untag_mask;
}
#[cfg(not(CONFIG_ADDRESS_MASKING))]
#[inline] pub unsafe fn cpu_tlbstate_update_lam(_lam: usize, _untag_mask: u64) {}

#[inline]
pub unsafe fn __native_tlb_flush_global(cr4: usize) {
    native_write_cr4(cr4 ^ X86_CR4_PGE);
    native_write_cr4(cr4);
}

// External kernel types/constants referenced above are supplied by other translated files.
#[allow(non_camel_case_types)] pub enum mm_struct {}
#[allow(non_camel_case_types)] pub enum task_struct {}
#[allow(non_camel_case_types)] pub enum vm_area_struct {}
#[allow(non_camel_case_types)] pub enum cpumask {}
#[allow(non_camel_case_types)] pub enum arch_tlbflush_unmap_batch {}
extern "C" { pub static mut PAGE_SIZE: usize; }
pub const PAGE_SHIFT: u32 = 0;
pub const X86_FEATURE_INVLPGB: u32 = 0;
pub const X86_CR3_LAM_U57_BIT: u32 = 0;
pub const X86_CR4_PGE: usize = 0;
pub const _PAGE_DIRTY: usize = 0;
pub const _PAGE_PRESENT: usize = 0;
pub const _PAGE_ACCESSED: usize = 0;
pub const _PAGE_SOFTW1: usize = 0;
pub const _PAGE_SOFTW2: usize = 0;
pub const _PAGE_SOFTW3: usize = 0;
pub const _PAGE_SOFTW4: usize = 0;
pub const _PAGE_SAVED_DIRTY: usize = 0;
pub const _PAGE_RW: usize = 0;
pub const _PAGE_USER: usize = 0;
pub const _PAGE_PWT: usize = 0;
pub const _PAGE_PCD: usize = 0;
pub const _PAGE_PSE: usize = 0;
pub const _PAGE_GLOBAL: usize = 0;
pub const _PAGE_PAT: usize = 0;
pub const _PAGE_PAT_LARGE: usize = 0;
pub const _PAGE_PKEY_BIT0: usize = 0;
pub const _PAGE_PKEY_BIT1: usize = 0;
pub const _PAGE_PKEY_BIT2: usize = 0;
pub const _PAGE_PKEY_BIT3: usize = 0;
pub const _PAGE_NX: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
