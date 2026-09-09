/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/proc-fns.h
 *
 *  Copyright (C) 1997-1999 Russell King
 *  Copyright (C) 2000 Deep Blue Solutions Ltd
 */

/* __KERNEL__ */
/* Dependencies: asm/glue-proc.h, asm/page.h */

#[cfg(not(feature = "assembly"))]
pub struct MmStruct;

/* Don't change this structure - ASM code relies on it. */
#[repr(C)]
pub struct Processor {
    pub _data_abort: Option<unsafe extern "C" fn(pc: ::core::ffi::c_ulong)>,
    pub _prefetch_abort: Option<unsafe extern "C" fn(lr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong>,
    pub _proc_init: Option<unsafe extern "C" fn()>,
    pub check_bugs: Option<unsafe extern "C" fn()>,
    pub _proc_fin: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn(addr: ::core::ffi::c_ulong, hvc: bool) -> !>,
    pub _do_idle: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub dcache_clean_area: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_int)>,
    pub switch_mm: Option<unsafe extern "C" fn(pgd_phys: PhysAddr, mm: *mut MmStruct)>,
    #[cfg(feature = "arm_lpae")]
    pub set_pte_ext: Option<unsafe extern "C" fn(ptep: *mut PteT, pte: PteT)>,
    #[cfg(not(feature = "arm_lpae"))]
    pub set_pte_ext: Option<unsafe extern "C" fn(ptep: *mut PteT, pte: PteT, ext: ::core::ffi::c_uint)>,
    pub suspend_size: ::core::ffi::c_uint,
    pub do_suspend: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
    pub do_resume: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
}

/* Types supplied by asm/page.h. */
pub type PhysAddr = ::core::ffi::c_ulong;
#[repr(C)]
pub struct PteT { _private: [u8; 0] }
#[repr(C)]
pub struct PgdT { _private: [u8; 0] }

#[cfg(not(feature = "multi_cpu"))]
#[inline]
pub unsafe fn init_proc_vtable(_p: *const Processor) {}

unsafe extern "C" {
    pub fn cpu_proc_init();
    pub fn cpu_proc_fin();
    pub fn cpu_do_idle() -> ::core::ffi::c_int;
    pub fn cpu_dcache_clean_area(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_int);
    pub fn cpu_do_switch_mm(pgd_phys: PhysAddr, mm: *mut MmStruct);
    #[cfg(feature = "arm_lpae")]
    pub fn cpu_set_pte_ext(ptep: *mut PteT, pte: PteT);
    #[cfg(not(feature = "arm_lpae"))]
    pub fn cpu_set_pte_ext(ptep: *mut PteT, pte: PteT, ext: ::core::ffi::c_uint);
    pub fn cpu_reset(addr: ::core::ffi::c_ulong, hvc: bool) -> !;
    pub fn cpu_do_suspend(arg: *mut ::core::ffi::c_void);
    pub fn cpu_do_resume(arg: *mut ::core::ffi::c_void);
}

#[cfg(feature = "multi_cpu")]
unsafe extern "C" {
    pub static mut processor: Processor;
    pub static mut cpu_vtable: [*mut Processor; 0];
}

#[cfg(feature = "multi_cpu")]
#[inline]
pub unsafe fn init_proc_vtable(p: *const Processor) {
    #[cfg(all(feature = "big_little", feature = "harden_branch_predictor"))]
    {
        /* smp_processor_id(), WARN_ON_ONCE(), and the per-CPU table are external dependencies. */
        let _ = p;
        // The C implementation copies *p into the current CPU's table and checks
        // dcache_clean_area and set_pte_ext equality with cpu_vtable[0].
    }
    #[cfg(not(all(feature = "big_little", feature = "harden_branch_predictor")))]
    {
        processor = *p;
    }
}

unsafe extern "C" {
    pub fn cpu_resume();
}

/* CONFIG_MMU */
#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn cpu_switch_mm(pgd: *mut PgdT, mm: *mut MmStruct) {
    cpu_do_switch_mm(virt_to_phys(pgd), mm);
}

#[cfg(feature = "mmu")]
#[inline]
unsafe fn virt_to_phys(pgd: *mut PgdT) -> PhysAddr { pgd as PhysAddr }

#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn cpu_get_pgd() -> *mut PgdT {
    /* ARM CP15 inline assembly is preserved as an explicit external dependency. */
    unimplemented!("cpu_get_pgd requires ARM CP15 assembly")
}

#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn cpu_get_ttbcr() -> ::core::ffi::c_uint {
    unimplemented!("cpu_get_ttbcr requires ARM CP15 assembly")
}

#[cfg(feature = "mmu")]
#[inline]
pub unsafe fn cpu_set_ttbcr(_ttbcr: ::core::ffi::c_uint) {
    unimplemented!("cpu_set_ttbcr requires ARM CP15 assembly")
}

#[cfg(not(feature = "mmu"))]
#[inline]
pub unsafe fn cpu_switch_mm(_pgd: *mut PgdT, _mm: *mut MmStruct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
