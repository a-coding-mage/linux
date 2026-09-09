// SPDX-License-Identifier: GPL-2.0
/* Translated from linux/arch/sparc/mm/leon_m.c. */

// External kernel definitions supplied by the surrounding translation unit.
extern "C" {
    static mut srmmu_swprobe_trace: ::core::ffi::c_int;
    fn _pfn_valid(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn srmmu_get_context() -> ::core::ffi::c_uint;
    fn printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn flush_tlb_mm(mm: *mut ::core::ffi::c_void);
    static mut srmmu_name: *const ::core::ffi::c_char;
    static mut sparc32_cachetlb_ops: *mut sparc32_cachetlb_ops;
    static mut poke_srmmu: Option<unsafe extern "C" fn()>;
}

#[repr(C)]
pub struct vm_area_struct { pub vm_flags: ::core::ffi::c_ulong }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub struct leon3_cacheregs { pub ccr: ::core::ffi::c_ulong, pub iccr: ::core::ffi::c_ulong, pub dccr: ::core::ffi::c_ulong }
#[repr(C)]
pub struct sparc32_cachetlb_ops {
    pub cache_all: Option<unsafe extern "C" fn()>,
    pub cache_mm: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub cache_page: Option<unsafe extern "C" fn(*mut vm_area_struct, ::core::ffi::c_ulong)>,
    pub cache_range: Option<unsafe extern "C" fn(*mut vm_area_struct, ::core::ffi::c_ulong, ::core::ffi::c_ulong)>,
    pub tlb_all: Option<unsafe extern "C" fn()>,
    pub tlb_mm: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub tlb_page: Option<unsafe extern "C" fn(*mut vm_area_struct, ::core::ffi::c_ulong)>,
    pub tlb_range: Option<unsafe extern "C" fn(*mut vm_area_struct, ::core::ffi::c_ulong, ::core::ffi::c_ulong)>,
    pub page_to_ram: Option<unsafe extern "C" fn(::core::ffi::c_ulong)>,
    pub sig_insns: Option<unsafe extern "C" fn(*mut mm_struct, ::core::ffi::c_ulong)>,
    pub page_for_dma: Option<unsafe extern "C" fn(::core::ffi::c_ulong)>,
}

pub static mut leon_flush_during_switch: ::core::ffi::c_int = 1;

// Constants/macros are provided by asm/asi.h, asm/leon.h, asm/tlbflush.h, and mm_32.h.
extern "C" { fn leon_get_ctable_ptr() -> ::core::ffi::c_ulong; }

#[inline]
pub unsafe extern "C" fn leon_swprobe(vaddr: ::core::ffi::c_ulong, paddr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let ctxtbl = leon_get_ctable_ptr() as u32;
    if ctxtbl == 0 || _pfn_valid((ctxtbl as u64 >> 12) as _) == 0 { return 0; }
    let ctx = srmmu_get_context();
    let mut pgd = (ctxtbl.wrapping_add(ctx.wrapping_mul(4))) as *const u32;
    let mut lvl: u32;
    let mut pte: u32;
    let mut val = pgd.read_volatile();
    if (val & SRMMU_ET_MASK) == SRMMU_ET_PTE { lvl = 3; pte = val; }
    else {
        if (val & SRMMU_ET_MASK) != SRMMU_ET_PTD { return 0; }
        let ptr = (((val & SRMMU_PTD_PMASK) << 4).wrapping_add((((vaddr >> LEON_PGD_SH) as u32 & LEON_PGD_M) * 4))) as *const u32;
        if _pfn_valid((ptr as usize as u64 >> 12) as _) == 0 { return 0; }
        val = ptr.read_volatile();
        if (val & SRMMU_ET_MASK) == SRMMU_ET_PTE { lvl = 2; pte = val; }
        else {
            if (val & SRMMU_ET_MASK) != SRMMU_ET_PTD { return 0; }
            let ptr = (((val & SRMMU_PTD_PMASK) << 4).wrapping_add((((vaddr >> LEON_PMD_SH) as u32 & LEON_PMD_M) * 4))) as *const u32;
            if _pfn_valid((ptr as usize as u64 >> 12) as _) == 0 { return 0; }
            val = ptr.read_volatile();
            if (val & SRMMU_ET_MASK) == SRMMU_ET_PTE { lvl = 1; pte = val; }
            else {
                if (val & SRMMU_ET_MASK) != SRMMU_ET_PTD { return 0; }
                let ptr = (((val & SRMMU_PTD_PMASK) << 4).wrapping_add((((vaddr >> LEON_PTE_SH) as u32 & LEON_PTE_M) * 4))) as *const u32;
                if _pfn_valid((ptr as usize as u64 >> 12) as _) == 0 { return 0; }
                val = ptr.read_volatile();
                if (val & SRMMU_ET_MASK) != SRMMU_ET_PTE { return 0; }
                lvl = 0; pte = val;
            }
        }
    }
    let paddr_calc = match lvl { 0 => (vaddr & !((!0u64) << LEON_PTE_SH)) | (((pte & !0xff) as u64) << 4), 1 => (vaddr & !((!0u64) << LEON_PMD_SH)) | (((pte & !0xff) as u64) << 4), 2 => (vaddr & !((!0u64) << LEON_PGD_SH)) | (((pte & !0xff) as u64) << 4), _ => vaddr };
    if !paddr.is_null() { paddr.write(paddr_calc); }
    pte as u64
}

pub unsafe extern "C" fn leon_flush_icache_all() { core::arch::asm!("flush"); }
pub unsafe extern "C" fn leon_flush_dcache_all() { core::arch::asm!("sta %g0, [%g0] 0"); }
pub unsafe extern "C" fn leon_flush_pcache_all(vma: *mut vm_area_struct, _page: ::core::ffi::c_ulong) { if (*vma).vm_flags & VM_EXEC != 0 { leon_flush_icache_all(); } leon_flush_dcache_all(); }
pub unsafe extern "C" fn leon_flush_cache_all() { leon_flush_icache_all(); leon_flush_dcache_all(); }
pub unsafe extern "C" fn leon_flush_tlb_all() { leon_flush_cache_all(); core::arch::asm!("sta %g0, [%0] 0", in(reg) 0x400usize); }

pub unsafe extern "C" fn leon3_getCacheRegs(regs: *mut leon3_cacheregs) { if !regs.is_null() { (*regs).ccr = 0; (*regs).iccr = 0; (*regs).dccr = 0; } }
pub unsafe extern "C" fn leon_flush_needed() -> ::core::ffi::c_int { -1 }
pub unsafe extern "C" fn leon_switch_mm() { flush_tlb_mm(core::ptr::null_mut()); if leon_flush_during_switch != 0 { leon_flush_cache_all(); } }
unsafe extern "C" fn leon_flush_cache_mm(_: *mut mm_struct) { leon_flush_cache_all(); }
unsafe extern "C" fn leon_flush_cache_page(v: *mut vm_area_struct, p: ::core::ffi::c_ulong) { leon_flush_pcache_all(v, p); }
unsafe extern "C" fn leon_flush_cache_range(_: *mut vm_area_struct, _: ::core::ffi::c_ulong, _: ::core::ffi::c_ulong) { leon_flush_cache_all(); }
unsafe extern "C" fn leon_flush_tlb_mm(_: *mut mm_struct) { leon_flush_tlb_all(); }
unsafe extern "C" fn leon_flush_tlb_page(_: *mut vm_area_struct, _: ::core::ffi::c_ulong) { leon_flush_tlb_all(); }
unsafe extern "C" fn leon_flush_tlb_range(_: *mut vm_area_struct, _: ::core::ffi::c_ulong, _: ::core::ffi::c_ulong) { leon_flush_tlb_all(); }
unsafe extern "C" fn leon_flush_page_to_ram(_: ::core::ffi::c_ulong) { leon_flush_cache_all(); }
unsafe extern "C" fn leon_flush_sig_insns(_: *mut mm_struct, _: ::core::ffi::c_ulong) { leon_flush_cache_all(); }
unsafe extern "C" fn leon_flush_page_for_dma(_: ::core::ffi::c_ulong) { leon_flush_dcache_all(); }
pub unsafe extern "C" fn poke_leonsparc() {}

pub unsafe extern "C" fn init_leon() { srmmu_name = b"LEON\0".as_ptr() as _; leon_flush_during_switch = leon_flush_needed(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
