/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * based on arch/m68k/mm/init.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux and architecture headers from the original implementation provide
// the kernel types, constants, functions, and macros referenced below.

extern "C" {
    static mut pgd_current: *mut pgd_t;
    static mut max_low_pfn: ::core::ffi::c_ulong;
    static mut empty_zero_page: ::core::ffi::c_uchar;

    fn pagetable_init();
    fn flush_dcache_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn flush_tlb_all();
    fn get_zeroed_page(gfp: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn memcpy(dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, n: usize) -> *mut ::core::ffi::c_void;
    fn flush_icache_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn virt_to_page(addr: ::core::ffi::c_ulong) -> *mut page;
    fn mmap_write_lock(mm: *mut mm_struct);
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        vm_flags: ::core::ffi::c_ulong,
        spec: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn is_err(ptr: *mut vm_area_struct) -> bool;
    fn ptr_err(ptr: *mut vm_area_struct) -> ::core::ffi::c_long;
}

#[repr(C)]
pub struct pgd_t;
#[repr(C)]
pub struct pte_t;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct mm_struct;
#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: ::core::ffi::c_ulong,
}
#[repr(C)]
pub struct linux_binprm;
#[repr(C)]
pub struct pgprot_t(::core::ffi::c_ulong);
#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const ::core::ffi::c_char,
    pub pages: *mut *mut page,
}

const PTRS_PER_PGD: usize = 1;
const PTRS_PER_PTE: usize = 1;

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut ::core::ffi::c_ulong) {
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
}

/*
 * paging_init() continues the virtual memory environment setup which
 * was begun by the code in arch/head.S.
 * The parameters are pointers to where to stick the starting and ending
 * addresses of available kernel virtual memory.
 */
pub unsafe fn paging_init() {
    pagetable_init();
    pgd_current = swapper_pg_dir.as_mut_ptr();

    flush_dcache_range(
        (&raw mut empty_zero_page) as *mut _ as ::core::ffi::c_ulong,
        (&raw mut empty_zero_page) as *mut _ as ::core::ffi::c_ulong + PAGE_SIZE,
    );
}

pub unsafe fn mmu_init() {
    flush_tlb_all();
}

pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [pgd_t; PTRS_PER_PGD];
pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE] = [pte_t; PTRS_PER_PTE];
static mut kuser_page: [*mut page; 1] = [::core::ptr::null_mut(); 1];
static mut vdso_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const ::core::ffi::c_char,
    pages: &raw mut kuser_page,
};

unsafe fn alloc_kuser_page() -> ::core::ffi::c_int {
    unsafe extern "C" {
        static __kuser_helper_start: ::core::ffi::c_char;
        static __kuser_helper_end: ::core::ffi::c_char;
    }
    let kuser_sz = (&raw const __kuser_helper_end as usize)
        .wrapping_sub(&raw const __kuser_helper_start as usize);
    let vpage = get_zeroed_page(GFP_ATOMIC);
    if vpage == 0 {
        return -ENOMEM;
    }

    /* Copy kuser helpers */
    memcpy(
        vpage as *mut ::core::ffi::c_void,
        &raw const __kuser_helper_start as *const ::core::ffi::c_void,
        kuser_sz,
    );

    flush_icache_range(vpage, vpage + KUSER_SIZE);
    kuser_page[0] = virt_to_page(vpage);

    0
}

pub unsafe fn arch_setup_additional_pages(
    bprm: *mut linux_binprm,
    uses_interp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mm = current_mm();
    mmap_write_lock(mm);

    /* Map kuser helpers to user space address */
    let vma = _install_special_mapping(
        mm,
        KUSER_BASE,
        KUSER_SIZE,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYEXEC,
        &raw mut vdso_mapping,
    );

    mmap_write_unlock(mm);

    if is_err(vma) { ptr_err(vma) as ::core::ffi::c_int } else { 0 }
}

pub unsafe fn arch_vma_name(vma: *mut vm_area_struct) -> *const ::core::ffi::c_char {
    if (*vma).vm_start == KUSER_BASE {
        b"[kuser]\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        ::core::ptr::null()
    }
}

static protection_map: [pgprot_t; 16] = [
    MKP(0, 0, 0), MKP(0, 0, 1), MKP(0, 0, 0), MKP(0, 0, 1),
    MKP(1, 0, 0), MKP(1, 0, 1), MKP(1, 0, 0), MKP(1, 0, 1),
    MKP(0, 0, 0), MKP(0, 0, 1), MKP(0, 1, 0), MKP(0, 1, 1),
    MKP(1, 0, 0), MKP(1, 0, 1), MKP(1, 1, 0), MKP(1, 1, 1),
];

// DECLARE_VM_GET_PAGE_PROT

// The following block is compiled only when CONFIG_EXECMEM is enabled.
#[cfg(CONFIG_EXECMEM)]
static mut execmem_info: execmem_info_t = execmem_info_t { ranges: [execmem_range_t { start: 0, end: 0, pgprot: PAGE_KERNEL_EXEC, alignment: 0 }; 1] };

#[cfg(CONFIG_EXECMEM)]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info_t {
    execmem_info.ranges[EXECMEM_DEFAULT] = execmem_range_t {
        start: MODULES_VADDR,
        end: MODULES_END,
        pgprot: PAGE_KERNEL_EXEC,
        alignment: 1,
    };
    &raw mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
