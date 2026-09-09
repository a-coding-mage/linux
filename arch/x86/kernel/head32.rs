// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/i386/kernel/head32.c -- prepare to run common code
 *
 *  Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
 *  Copyright (C) 2007 Eric Biederman <ebiederm@xmission.com>
 */

// Declarations supplied by the corresponding Linux architecture headers.
extern "C" {
    fn idt_setup_early_handler();
    fn load_ucode_bsp();
    fn cr4_init_shadow();
    fn sanitize_boot_params(params: *mut boot_params);
    fn x86_early_init_platform_quirks();
    fn x86_intel_mid_early_setup();
    fn x86_ce4100_early_setup();
    fn start_kernel() -> !;
    fn i386_reserve_resources();
    fn setup_ioapic_ids_from_mpc();
}

#[repr(C)]
pub struct pte_t { pub pte: usize }
#[repr(C)]
pub struct pgd_t { pub pgd: usize }
#[repr(C)]
pub struct pmd_t { pub pmd: usize }

#[repr(C)]
pub struct boot_params_hdr {
    pub hardware_subarch: u32,
    pub ramdisk_image: u32,
    pub ramdisk_size: u32,
}
#[repr(C)]
pub struct boot_params { pub hdr: boot_params_hdr }

extern "C" {
    static mut boot_params: boot_params;
    static mut max_pfn_mapped: usize;
    static mut _end: u8;
    static mut __brk_base: u8;
    static mut _brk_end: u8;
    static mut initial_page_table: [pgd_t; 0];
    static mut initial_pg_pmd: [pmd_t; 0];
    static mut x86_init: X86Init;
}

#[repr(C)]
pub struct X86Resources { pub reserve_resources: Option<unsafe extern "C" fn()> }
#[repr(C)]
pub struct X86Mpparse { pub setup_ioapic_ids: Option<unsafe extern "C" fn()> }
#[repr(C)]
pub struct X86Init { pub resources: X86Resources, pub mpparse: X86Mpparse }

const PAGE_OFFSET: usize = 0;
const PGDIR_SHIFT: usize = 22;
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const PTRS_PER_PTE: usize = 1024;
const PTE_PFN_MASK: usize = !((1 << PAGE_SHIFT) - 1);
const PTE_IDENT_ATTR: usize = 0;
const PDE_IDENT_ATTR: usize = 0;
const LOWMEM_PAGES: usize = 0;
const X86_SUBARCH_INTEL_MID: u32 = 1;
const X86_SUBARCH_CE4100: u32 = 2;

#[inline]
unsafe fn __pa_nodebug<T>(p: *const T) -> usize { p as usize }
#[inline]
unsafe fn page_table_size(_: usize) -> usize { 0 }
#[inline]
unsafe fn pfn_align(v: usize) -> usize { (v + PAGE_SIZE - 1) & PAGE_MASK }

#[cfg(feature = "CONFIG_MICROCODE_INITRD32")]
static mut initrd_start_early: usize = 0;
#[cfg(feature = "CONFIG_MICROCODE_INITRD32")]
static mut initrd_pl2p_start: *mut pte_t = core::ptr::null_mut();
#[cfg(feature = "CONFIG_MICROCODE_INITRD32")]
static mut initrd_pl2p_end: *mut pte_t = core::ptr::null_mut();

unsafe fn i386_default_early_setup() {
    (*core::ptr::addr_of_mut!(x86_init)).resources.reserve_resources =
        Some(i386_reserve_resources);
    (*core::ptr::addr_of_mut!(x86_init)).mpparse.setup_ioapic_ids =
        Some(setup_ioapic_ids_from_mpc);
}

#[cfg(feature = "CONFIG_MICROCODE_INITRD32")]
unsafe fn zap_early_initrd_mapping() {
    let mut pl2p = initrd_pl2p_start;
    while pl2p < initrd_pl2p_end {
        (*pl2p).pte = 0;
        if !cfg!(feature = "CONFIG_X86_PAE") {
            (*pl2p.add(PAGE_OFFSET >> PGDIR_SHIFT)).pte = 0;
        }
        pl2p = pl2p.add(1);
    }
}
#[cfg(not(feature = "CONFIG_MICROCODE_INITRD32"))]
#[inline]
unsafe fn zap_early_initrd_mapping() {}

pub unsafe extern "C" fn i386_start_kernel() -> ! {
    idt_setup_early_handler();
    load_ucode_bsp();
    zap_early_initrd_mapping();
    cr4_init_shadow();
    sanitize_boot_params(core::ptr::addr_of_mut!(boot_params));
    x86_early_init_platform_quirks();
    match boot_params.hdr.hardware_subarch {
        X86_SUBARCH_INTEL_MID => x86_intel_mid_early_setup(),
        X86_SUBARCH_CE4100 => x86_ce4100_early_setup(),
        _ => i386_default_early_setup(),
    }
    start_kernel()
}

unsafe fn init_map(mut pte: pte_t, ptep: &mut *mut pte_t, pl2p: &mut *mut usize,
                   limit: usize) -> pte_t {
    while (pte.pte & PTE_PFN_MASK) < limit {
        let pl2 = *ptep as usize | PDE_IDENT_ATTR;
        *pl2p = pl2;
        if !cfg!(feature = "CONFIG_X86_PAE") {
            *(*pl2p).add(PAGE_OFFSET >> PGDIR_SHIFT) = pl2;
        }
        for _ in 0..PTRS_PER_PTE {
            (**ptep).pte = pte.pte;
            pte.pte = pte.pte.wrapping_add(PAGE_SIZE);
            *ptep = (*ptep).add(1);
        }
        *pl2p = (*pl2p).add(1);
    }
    pte
}

pub unsafe extern "C" fn mk_early_pgtbl_32() {
    let limit = __pa_nodebug(core::ptr::addr_of!(_end))
        .wrapping_add(page_table_size(LOWMEM_PAGES) << PAGE_SHIFT);
    let mut ptep = __pa_nodebug(core::ptr::addr_of!(__brk_base)) as *mut pte_t;
    let mut pl2p = __pa_nodebug(core::ptr::addr_of!(initial_page_table)) as *mut usize;
    let mut pte = init_map(pte_t { pte: PTE_IDENT_ATTR }, &mut ptep, &mut pl2p, limit);
    let ptr = __pa_nodebug(core::ptr::addr_of!(max_pfn_mapped)) as *mut usize;
    *ptr = (pte.pte & PTE_PFN_MASK) >> PAGE_SHIFT;
    let ptr = __pa_nodebug(core::ptr::addr_of!(_brk_end)) as *mut usize;
    *ptr = ptep as usize + PAGE_OFFSET;

    #[cfg(feature = "CONFIG_MICROCODE_INITRD32")]
    {
        let params = core::ptr::addr_of!(boot_params);
        if (*params).hdr.ramdisk_size == 0 || (*params).hdr.ramdisk_image == 0 {
            return;
        }
        initrd_start_early = (pte.pte & PTE_PFN_MASK) + PAGE_OFFSET;
        initrd_start_early = initrd_start_early.wrapping_add(
            (*params).hdr.ramdisk_image as usize & !PAGE_MASK);
        initrd_pl2p_start = pl2p as *mut pte_t;
        let limit = (*params).hdr.ramdisk_image as usize;
        pte.pte = PTE_IDENT_ATTR | pfn_align(limit);
        let limit = (*params).hdr.ramdisk_image as usize
            + (*params).hdr.ramdisk_size as usize;
        init_map(pte, &mut ptep, &mut pl2p, limit);
        initrd_pl2p_end = pl2p as *mut pte_t;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
