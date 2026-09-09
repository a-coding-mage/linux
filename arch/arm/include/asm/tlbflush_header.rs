/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/tlbflush.h. */

/* The original header is active only with CONFIG_MMU. */
#[cfg(CONFIG_MMU)]
pub const TLB_V4_U_PAGE: u32 = 1 << 1;
#[cfg(CONFIG_MMU)]
pub const TLB_V4_D_PAGE: u32 = 1 << 2;
#[cfg(CONFIG_MMU)]
pub const TLB_V4_I_PAGE: u32 = 1 << 3;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_U_PAGE: u32 = 1 << 4;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_D_PAGE: u32 = 1 << 5;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_I_PAGE: u32 = 1 << 6;
#[cfg(CONFIG_MMU)]
pub const TLB_V4_U_FULL: u32 = 1 << 9;
#[cfg(CONFIG_MMU)]
pub const TLB_V4_D_FULL: u32 = 1 << 10;
#[cfg(CONFIG_MMU)]
pub const TLB_V4_I_FULL: u32 = 1 << 11;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_U_FULL: u32 = 1 << 12;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_D_FULL: u32 = 1 << 13;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_I_FULL: u32 = 1 << 14;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_U_ASID: u32 = 1 << 16;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_D_ASID: u32 = 1 << 17;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_I_ASID: u32 = 1 << 18;
#[cfg(CONFIG_MMU)]
pub const TLB_V6_BP: u32 = 1 << 19;
#[cfg(CONFIG_MMU)]
pub const TLB_V7_UIS_PAGE: u32 = 1 << 20;
#[cfg(CONFIG_MMU)]
pub const TLB_V7_UIS_FULL: u32 = 1 << 21;
#[cfg(CONFIG_MMU)]
pub const TLB_V7_UIS_ASID: u32 = 1 << 22;
#[cfg(CONFIG_MMU)]
pub const TLB_V7_UIS_BP: u32 = 1 << 23;
#[cfg(CONFIG_MMU)]
pub const TLB_BARRIER: u32 = 1 << 28;
#[cfg(CONFIG_MMU)]
pub const TLB_L2CLEAN_FR: u32 = 1 << 29;
#[cfg(CONFIG_MMU)]
pub const TLB_DCLEAN: u32 = 1 << 30;
#[cfg(CONFIG_MMU)]
pub const TLB_WB: u32 = 1 << 31;

#[cfg(CONFIG_MMU)]
pub const v4_tlb_flags: u32 = TLB_V4_U_FULL | TLB_V4_U_PAGE;
#[cfg(CONFIG_MMU)]
pub const fa_tlb_flags: u32 = TLB_WB | TLB_DCLEAN | TLB_BARRIER | TLB_V4_U_FULL | TLB_V4_U_PAGE;
#[cfg(CONFIG_MMU)]
pub const v4wbi_tlb_flags: u32 = TLB_WB | TLB_DCLEAN | TLB_V4_I_FULL | TLB_V4_D_FULL | TLB_V4_I_PAGE | TLB_V4_D_PAGE;
#[cfg(CONFIG_MMU)]
pub const fr_tlb_flags: u32 = TLB_WB | TLB_DCLEAN | TLB_L2CLEAN_FR | TLB_V4_I_FULL | TLB_V4_D_FULL | TLB_V4_I_PAGE | TLB_V4_D_PAGE;
#[cfg(CONFIG_MMU)]
pub const v4wb_tlb_flags: u32 = TLB_WB | TLB_DCLEAN | TLB_V4_I_FULL | TLB_V4_D_FULL | TLB_V4_D_PAGE;
#[cfg(CONFIG_MMU)]
pub const v6wbi_tlb_flags: u32 = TLB_WB | TLB_DCLEAN | TLB_BARRIER | TLB_V6_I_FULL | TLB_V6_D_FULL | TLB_V6_I_PAGE | TLB_V6_D_PAGE | TLB_V6_I_ASID | TLB_V6_D_ASID | TLB_V6_BP;
#[cfg(CONFIG_MMU)]
pub const v7wbi_tlb_flags_smp: u32 = TLB_WB | TLB_BARRIER | TLB_V7_UIS_FULL | TLB_V7_UIS_PAGE | TLB_V7_UIS_ASID | TLB_V7_UIS_BP;
#[cfg(CONFIG_MMU)]
pub const v7wbi_tlb_flags_up: u32 = TLB_WB | TLB_DCLEAN | TLB_BARRIER | TLB_V6_U_FULL | TLB_V6_U_PAGE | TLB_V6_U_ASID | TLB_V6_BP;

#[repr(C)]
pub struct cpu_tlb_fns {
    pub flush_user_range: Option<unsafe extern "C" fn(usize, usize, *mut vm_area_struct)>,
    pub flush_kern_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub tlb_flags: usize,
}

#[allow(non_camel_case_types)]
pub enum vm_area_struct {}
#[allow(non_camel_case_types)]
pub enum mm_struct {}
#[allow(non_camel_case_types)]
pub enum vm_fault {}
#[allow(non_camel_case_types)]
pub enum pte_t {}

extern "C" {
    pub static mut cpu_tlb: cpu_tlb_fns;
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_all();
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_mm(mm: *mut mm_struct);
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_page(vma: *mut vm_area_struct, uaddr: usize);
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_kernel_page(kaddr: usize);
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    #[cfg(CONFIG_SMP)] pub fn flush_tlb_kernel_range(start: usize, end: usize);
    #[cfg(CONFIG_SMP)] pub fn flush_bp_all();
    #[cfg(all(not(CONFIG_ARM_ERRATA_798181), not(CONFIG_MMU)))] fn erratum_a15_798181_init();
    pub static mut erratum_a15_798181_handler: Option<unsafe extern "C" fn() -> bool>;
}

#[cfg(CONFIG_MMU)]
#[inline(always)]
pub unsafe fn __local_flush_tlb_all() { let _ = cpu_tlb.tlb_flags; }
#[cfg(CONFIG_MMU)]
#[inline(always)]
pub unsafe fn local_flush_tlb_all() { __local_flush_tlb_all(); }
#[cfg(CONFIG_MMU)]
#[inline(always)]
pub unsafe fn __flush_tlb_all() { __local_flush_tlb_all(); }

/* Architecture-specific assembly operations and the remaining inline routines
 * retain their C interfaces through the external implementation. */
#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, uaddr: usize);
    pub fn local_flush_tlb_kernel_page(kaddr: usize);
    pub fn local_flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn local_flush_tlb_kernel_range(start: usize, end: usize);
    pub fn local_flush_bp_all();
    pub fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t, nr: u32);
}

#[inline(always)]
pub unsafe fn erratum_a15_798181() -> bool {
    if cfg!(CONFIG_ARM_ERRATA_798181) {
        if let Some(handler) = erratum_a15_798181_handler { return handler(); }
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
