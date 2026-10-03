// SPDX-License-Identifier: GPL-2.0
/*
 * On-demand x86-64 compressed-kernel identity mappings.
 * Copyright (C) 2015-2016 Yinghai Lu
 * Copyright (C) 2016 Kees Cook
 *
 * The original C/H/S sources remain the authority. Native generated bindings
 * provide layouts/constants only; this owner implements the allocation, table
 * construction, page attributes, faults and CPU operations in Rust.
 */
#![allow(non_camel_case_types, non_upper_case_globals)]
use crate::bindings;
use crate::boot_ident_map_bindings as abi;
use abi::{p4d_t, pgd_t, pmd_t, pt_regs, pte_t, pud_t, x86_mapping_info};
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr::{addr_of_mut, null_mut, read_unaligned, write_volatile};

#[path = "../../mm/ident_map.rs"]
mod ident_map;
use ident_map::kernel_ident_mapping_init;

const PAGE_SHIFT: u32 = abi::LUPOS_IDENT_PAGE_SHIFT as u32;
const PAGE_SIZE: c_ulong = abi::LUPOS_IDENT_PAGE_SIZE as c_ulong;
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const PMD_SHIFT: u32 = abi::LUPOS_IDENT_PMD_SHIFT as u32;
const PMD_SIZE: c_ulong = abi::LUPOS_IDENT_PMD_SIZE as c_ulong;
const PMD_MASK: c_ulong = !(PMD_SIZE - 1);
const PUD_SHIFT: u32 = abi::LUPOS_IDENT_PUD_SHIFT as u32;
const PUD_SIZE: c_ulong = abi::LUPOS_IDENT_PUD_SIZE as c_ulong;
const PUD_MASK: c_ulong = !(PUD_SIZE - 1);
const P4D_SHIFT: u32 = abi::LUPOS_IDENT_P4D_SHIFT as u32;
const P4D_SIZE: c_ulong = abi::LUPOS_IDENT_P4D_SIZE as c_ulong;
const PTRS_PER_PTE: usize = abi::LUPOS_IDENT_PTRS_PER_PTE as usize;
const PTRS_PER_PMD: usize = abi::LUPOS_IDENT_PTRS_PER_PMD as usize;
const PTRS_PER_PUD: usize = abi::LUPOS_IDENT_PTRS_PER_PUD as usize;
const PTRS_PER_PGD: usize = abi::LUPOS_IDENT_PTRS_PER_PGD as usize;
const INITIAL_PHYSICAL_MASK: u64 = (1u64 << abi::LUPOS_IDENT_PHYSICAL_MASK_SHIFT) - 1;
const _PAGE_PRESENT: c_ulong = abi::LUPOS_IDENT_PAGE_PRESENT as c_ulong;
const _PAGE_PROTNONE: c_ulong = abi::LUPOS_IDENT_PAGE_PROTNONE as c_ulong;
const _PAGE_PSE: c_ulong = abi::LUPOS_IDENT_PAGE_PSE as c_ulong;
const _PAGE_NOPTISHADOW: c_ulong = abi::LUPOS_IDENT_PAGE_NOPTISHADOW as c_ulong;
const BOOT_INIT_PGT_SIZE: c_ulong = abi::LUPOS_IDENT_BOOT_INIT_PGT_SIZE as c_ulong;
const BOOT_PGT_SIZE: c_ulong = abi::LUPOS_IDENT_BOOT_PGT_SIZE as c_ulong;
const BOOT_PGT_SIZE_WARN: c_ulong = abi::LUPOS_IDENT_BOOT_PGT_SIZE_WARN as c_ulong;
const ENOMEM: c_int = bindings::ENOMEM as c_int;

/* Nonzero .data initializers are also needed before startup clears .bss. */
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub(crate) static mut __default_kernel_pte_mask: c_ulong = !0;
#[no_mangle]
pub(crate) static mut physical_mask: abi::phys_addr_t = INITIAL_PHYSICAL_MASK;

#[repr(C)]
struct alloc_pgt_data {
    pgt_buf: *mut u8,
    pgt_buf_size: c_ulong,
    pgt_buf_offset: c_ulong,
}
static mut pgt_data: alloc_pgt_data = alloc_pgt_data {
    pgt_buf: null_mut(),
    pgt_buf_size: 0,
    pgt_buf_offset: 0,
};
static mut top_level_pgt: c_ulong = 0;
/* Relocation requires the callback and context to be assigned at run time. */
static mut mapping_info: x86_mapping_info = x86_mapping_info {
    alloc_pgt_page: None,
    free_pgt_page: None,
    context: null_mut(),
    page_flag: 0,
    offset: 0,
    direct_gbpages: false,
    kernpg_flag: 0,
};

#[inline]
unsafe fn encryption_mask() -> c_ulong {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    unsafe {
        abi::sme_me_mask as c_ulong
    }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    {
        0
    }
}
#[inline]
unsafe fn configured_physical_mask() -> c_ulong {
    #[cfg(CONFIG_DYNAMIC_PHYSICAL_MASK)]
    unsafe {
        physical_mask as c_ulong
    }
    #[cfg(not(CONFIG_DYNAMIC_PHYSICAL_MASK))]
    {
        INITIAL_PHYSICAL_MASK as c_ulong
    }
}
#[inline]
unsafe fn kernpg_table() -> c_ulong {
    unsafe { abi::LUPOS_IDENT_KERNPG_TABLE_NOENC as c_ulong | encryption_mask() }
}
#[inline]
unsafe fn pgtable_l5_enabled() -> bool {
    unsafe { abi::__pgtable_l5_enabled != 0 }
}
#[inline]
unsafe fn ptrs_per_p4d() -> usize {
    unsafe { abi::ptrs_per_p4d as usize }
}
#[inline]
unsafe fn pte_pfn_mask() -> c_ulong {
    unsafe { configured_physical_mask() & PAGE_MASK }
}
#[inline]
unsafe fn pmd_pfn_mask(value: pmd_t) -> c_ulong {
    unsafe {
        configured_physical_mask()
            & if value.pmd & _PAGE_PSE != 0 {
                PMD_MASK
            } else {
                PAGE_MASK
            }
    }
}
#[inline]
unsafe fn pud_pfn_mask(value: pud_t) -> c_ulong {
    unsafe {
        configured_physical_mask()
            & if value.pud & _PAGE_PSE != 0 {
                PUD_MASK
            } else {
                PAGE_MASK
            }
    }
}
#[inline]
unsafe fn pgd_present(value: pgd_t) -> bool {
    unsafe { !pgtable_l5_enabled() || (value.pgd & !pte_pfn_mask() & _PAGE_PRESENT) != 0 }
}
#[inline]
unsafe fn p4d_present(value: p4d_t) -> bool {
    unsafe { value.p4d & !pte_pfn_mask() & _PAGE_PRESENT != 0 }
}
#[inline]
unsafe fn pud_present(value: pud_t) -> bool {
    unsafe { value.pud & !pud_pfn_mask(value) & _PAGE_PRESENT != 0 }
}
#[inline]
unsafe fn pmd_present(value: pmd_t) -> bool {
    unsafe { value.pmd & !pmd_pfn_mask(value) & (_PAGE_PRESENT | _PAGE_PROTNONE | _PAGE_PSE) != 0 }
}
#[inline]
unsafe fn pmd_leaf(value: pmd_t) -> bool {
    unsafe { value.pmd & !pmd_pfn_mask(value) & _PAGE_PSE != 0 }
}
#[inline]
fn pud_leaf(value: pud_t) -> bool {
    value.pud & _PAGE_PSE != 0
}
#[inline]
fn pte_index(address: c_ulong) -> usize {
    (address >> PAGE_SHIFT) as usize & (PTRS_PER_PTE - 1)
}
#[inline]
fn pmd_index(address: c_ulong) -> usize {
    (address >> PMD_SHIFT) as usize & (PTRS_PER_PMD - 1)
}
#[inline]
fn pud_index(address: c_ulong) -> usize {
    (address >> PUD_SHIFT) as usize & (PTRS_PER_PUD - 1)
}
#[inline]
unsafe fn p4d_index(address: c_ulong) -> usize {
    unsafe { (address >> P4D_SHIFT) as usize & (ptrs_per_p4d() - 1) }
}
#[inline]
unsafe fn pgd_index(address: c_ulong) -> usize {
    unsafe { (address >> abi::pgdir_shift) as usize & (PTRS_PER_PGD - 1) }
}
/* misc.h's early-boot __pa and __va are identity conversions. */
#[inline]
fn __pa(address: *mut c_void) -> c_ulong {
    address as c_ulong
}
#[inline]
unsafe fn p4d_offset(pgd: *mut pgd_t, address: c_ulong) -> *mut p4d_t {
    unsafe {
        if !pgtable_l5_enabled() {
            return pgd.cast();
        }
        (((*pgd).pgd & pte_pfn_mask()) as *mut p4d_t).add(p4d_index(address))
    }
}
#[inline]
unsafe fn pud_offset(p4d: *mut p4d_t, address: c_ulong) -> *mut pud_t {
    unsafe { (((*p4d).p4d & pte_pfn_mask()) as *mut pud_t).add(pud_index(address)) }
}
#[inline]
unsafe fn pmd_offset(pud: *mut pud_t, address: c_ulong) -> *mut pmd_t {
    unsafe { (((*pud).pud & pud_pfn_mask(*pud)) as *mut pmd_t).add(pmd_index(address)) }
}
#[inline]
unsafe fn pte_offset_kernel(pmd: *mut pmd_t, address: c_ulong) -> *mut pte_t {
    unsafe { (((*pmd).pmd & pmd_pfn_mask(*pmd)) as *mut pte_t).add(pte_index(address)) }
}
#[inline]
fn __pte(value: c_ulong) -> pte_t {
    pte_t { pte: value }
}
#[inline]
fn __pmd(value: c_ulong) -> pmd_t {
    pmd_t { pmd: value }
}
#[inline]
fn __pud(value: c_ulong) -> pud_t {
    pud_t { pud: value }
}
#[inline]
fn __p4d(value: c_ulong) -> p4d_t {
    p4d_t { p4d: value }
}
#[inline]
fn __pgd(value: c_ulong) -> pgd_t {
    pgd_t { pgd: value }
}
/* Native WRITE_ONCE setters; PTI is disabled in this compressed owner. */
#[inline]
unsafe fn set_pte(entry: *mut pte_t, value: pte_t) {
    unsafe {
        write_volatile(entry, value);
    }
}
#[inline]
unsafe fn set_pmd(entry: *mut pmd_t, value: pmd_t) {
    unsafe {
        write_volatile(entry, value);
    }
}
#[inline]
unsafe fn set_pud(entry: *mut pud_t, value: pud_t) {
    unsafe {
        write_volatile(entry, value);
    }
}
#[inline]
unsafe fn set_p4d(entry: *mut p4d_t, value: p4d_t) {
    unsafe {
        write_volatile(entry, value);
    }
}
#[inline]
unsafe fn set_pgd(entry: *mut pgd_t, value: pgd_t) {
    unsafe {
        write_volatile(entry, value);
    }
}
/* Preserve unsigned wrap at the address-space end in pgtable.h's macros. */
#[inline]
fn addr_end(address: c_ulong, end: c_ulong, size: c_ulong) -> c_ulong {
    let boundary = address.wrapping_add(size) & !(size - 1);
    if boundary.wrapping_sub(1) < end.wrapping_sub(1) {
        boundary
    } else {
        end
    }
}
#[inline]
fn pud_addr_end(address: c_ulong, end: c_ulong) -> c_ulong {
    addr_end(address, end, PUD_SIZE)
}
#[inline]
fn p4d_addr_end(address: c_ulong, end: c_ulong) -> c_ulong {
    addr_end(address, end, P4D_SIZE)
}
#[inline]
unsafe fn pgd_addr_end(address: c_ulong, end: c_ulong) -> c_ulong {
    unsafe { addr_end(address, end, 1u64 << abi::pgdir_shift) }
}

#[inline]
unsafe fn read_cr3_pa() -> c_ulong {
    unsafe {
        let value: c_ulong;
        core::arch::asm!("mov {}, cr3", out(reg) value, options(nostack, preserves_flags));
        value & pte_pfn_mask() & !encryption_mask()
    }
}
#[inline]
unsafe fn write_cr3(value: c_ulong) {
    // The default asm memory clobber preserves native_write_cr3 ordering.
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));
    }
}

unsafe extern "C" fn alloc_pgt_page(context: *mut c_void) -> *mut c_void {
    unsafe {
        let pages = context.cast::<alloc_pgt_data>();
        if (*pages).pgt_buf_offset >= (*pages).pgt_buf_size {
            #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
            {
                bindings::__putstr(
                    c"out of pgt_buf in arch/x86/boot/compressed/ident_map_64.c!?\n".as_ptr(),
                );
                debug_pgt_space(pages);
            }
            return null_mut();
        }
        if (*pages).pgt_buf_offset == BOOT_PGT_SIZE_WARN {
            #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
            {
                bindings::__putstr(
                    c"pgt_buf running low in arch/x86/boot/compressed/ident_map_64.c\n".as_ptr(),
                );
                bindings::__putstr(c"Need to raise BOOT_PGT_SIZE?\n".as_ptr());
                debug_pgt_space(pages);
            }
        }
        let entry = (*pages).pgt_buf.add((*pages).pgt_buf_offset as usize);
        (*pages).pgt_buf_offset = (*pages).pgt_buf_offset.wrapping_add(PAGE_SIZE);
        entry.cast()
    }
}
#[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
unsafe fn debug_pgt_space(pages: *const alloc_pgt_data) {
    unsafe {
        bindings::__putstr(c"pages->pgt_buf_offset: 0x".as_ptr());
        bindings::__puthex((*pages).pgt_buf_offset);
        bindings::__putstr(c"\npages->pgt_buf_size: 0x".as_ptr());
        bindings::__puthex((*pages).pgt_buf_size);
        bindings::__putstr(c"\n".as_ptr());
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_add_identity_map(mut start: c_ulong, mut end: c_ulong) {
    unsafe {
        start &= PMD_MASK;
        end = end.wrapping_sub(1) | (PMD_SIZE - 1);
        end = end.wrapping_add(1);
        if start >= end {
            return;
        }
        let ret = kernel_ident_mapping_init(
            addr_of_mut!(mapping_info),
            top_level_pgt as *mut pgd_t,
            start,
            end,
        );
        if ret != 0 {
            crate::error::error(
                c"Error: kernel_ident_mapping_init() failed\n"
                    .as_ptr()
                    .cast_mut(),
            );
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn initialize_identity_maps(rmode: *mut c_void) {
    unsafe {
        physical_mask &= !(encryption_mask() as u64);
        let info = addr_of_mut!(mapping_info);
        let pages = addr_of_mut!(pgt_data);
        (*info).alloc_pgt_page = Some(alloc_pgt_page);
        (*info).context = pages.cast();
        (*info).page_flag = abi::LUPOS_IDENT_PAGE_KERNEL_LARGE_EXEC as c_ulong | encryption_mask();
        (*info).kernpg_flag = kernpg_table();
        (*pages).pgt_buf_offset = 0;
        top_level_pgt = read_cr3_pa();
        let pgtable = addr_of_mut!(abi::_pgtable).cast::<u8>();
        if p4d_offset(top_level_pgt as *mut pgd_t, 0) == pgtable.cast() {
            (*pages).pgt_buf = pgtable.add(BOOT_INIT_PGT_SIZE as usize);
            (*pages).pgt_buf_size = BOOT_PGT_SIZE - BOOT_INIT_PGT_SIZE;
            bindings::memset((*pages).pgt_buf.cast(), 0, (*pages).pgt_buf_size as usize);
        } else {
            (*pages).pgt_buf = pgtable;
            (*pages).pgt_buf_size = BOOT_PGT_SIZE;
            bindings::memset((*pages).pgt_buf.cast(), 0, (*pages).pgt_buf_size as usize);
            top_level_pgt = alloc_pgt_page(pages.cast()) as c_ulong;
        }
        kernel_add_identity_map(
            addr_of_mut!(abi::_head) as c_ulong,
            addr_of_mut!(abi::_end) as c_ulong,
        );
        bindings::boot_params_ptr = rmode.cast();
        let bp = bindings::boot_params_ptr;
        kernel_add_identity_map(bp as c_ulong, bp.add(1) as c_ulong);
        let cmdline = bindings::get_cmd_line_ptr();
        kernel_add_identity_map(
            cmdline,
            cmdline.wrapping_add(bindings::COMMAND_LINE_SIZE as c_ulong),
        );
        // Both native boot_params and setup_data are packed layouts.
        let mut sd = read_unaligned(core::ptr::addr_of!((*bp).hdr.setup_data))
            as *const bindings::setup_data;
        while !sd.is_null() {
            let sd_addr = sd as c_ulong;
            let len = read_unaligned(core::ptr::addr_of!((*sd).len)) as c_ulong;
            kernel_add_identity_map(
                sd_addr,
                sd_addr
                    .wrapping_add(core::mem::size_of::<bindings::setup_data>() as c_ulong)
                    .wrapping_add(len),
            );
            sd = read_unaligned(core::ptr::addr_of!((*sd).next)) as *const bindings::setup_data;
        }
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        abi::sev_prep_identity_maps(top_level_pgt);
        write_cr3(top_level_pgt);
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        abi::snp_check_features();
    }
}

unsafe fn split_large_pmd(
    info: *mut x86_mapping_info,
    pmdp: *mut pmd_t,
    address: c_ulong,
) -> *mut pte_t {
    unsafe {
        let pte = ((*info).alloc_pgt_page.unwrap())((*info).context).cast::<pte_t>();
        if pte.is_null() {
            return null_mut();
        }
        let mut page_address = address & PMD_MASK;
        let page_flags = (*info).page_flag & !_PAGE_PSE;
        for i in 0..PTRS_PER_PMD {
            set_pte(pte.add(i), __pte(page_address | page_flags));
            page_address = page_address.wrapping_add(PAGE_SIZE);
        }
        // The range may contain our own stack/code: never clear the PMD first.
        set_pmd(pmdp, __pmd(pte as c_ulong | (*info).kernpg_flag));
        write_cr3(top_level_pgt);
        pte.add(pte_index(address))
    }
}

unsafe fn clflush_page(address: c_ulong) {
    unsafe {
        // Do not query CPUID: a nested #VC cannot use the not-yet-ready GHCB.
        let start = address & PAGE_MASK;
        let end = start.wrapping_add(PAGE_SIZE);
        core::arch::asm!("mfence", options(nostack, preserves_flags));
        let mut cl = start;
        while cl != end {
            core::arch::asm!("clflush [{}]", in(reg) cl, options(nostack, preserves_flags));
            cl = cl.wrapping_add(64);
        }
    }
}

unsafe fn set_clr_page_flags(
    info: *mut x86_mapping_info,
    address: c_ulong,
    set: c_ulong,
    clr: c_ulong,
) -> c_int {
    unsafe {
        // Keep the original faulting memory access and r9 clobber. Calling
        // kernel_add_identity_map here would overwrite existing PTE mappings.
        core::arch::asm!("mov r9, [{address}]", address = in(reg) address, out("r9") _, options(nostack, preserves_flags));
        let pgdp = top_level_pgt as *mut pgd_t;
        let pmdp = pmd_offset(pud_offset(p4d_offset(pgdp, address), address), address);
        let ptep = if pmd_leaf(*pmdp) {
            split_large_pmd(info, pmdp, address)
        } else {
            pte_offset_kernel(pmdp, address)
        };
        if ptep.is_null() {
            return -ENOMEM;
        }
        if (set | clr) & encryption_mask() != 0 {
            clflush_page(address);
            if clr != 0 {
                #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
                abi::snp_set_page_shared(address & PAGE_MASK);
            }
        }
        let pte = __pte(((*ptep).pte | set) & !clr);
        set_pte(ptep, pte);
        // Private state transition must occur after publishing the new PTE.
        if set & encryption_mask() != 0 {
            #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
            abi::snp_set_page_private(address & PAGE_MASK);
        }
        write_cr3(top_level_pgt);
        0
    }
}
#[no_mangle]
pub(crate) unsafe extern "C" fn set_page_decrypted(address: c_ulong) -> c_int {
    unsafe { set_clr_page_flags(addr_of_mut!(mapping_info), address, 0, encryption_mask()) }
}
#[no_mangle]
pub(crate) unsafe extern "C" fn set_page_encrypted(address: c_ulong) -> c_int {
    unsafe { set_clr_page_flags(addr_of_mut!(mapping_info), address, encryption_mask(), 0) }
}
#[no_mangle]
pub(crate) unsafe extern "C" fn set_page_non_present(address: c_ulong) -> c_int {
    unsafe { set_clr_page_flags(addr_of_mut!(mapping_info), address, 0, _PAGE_PRESENT) }
}

unsafe fn do_pf_error(msg: *const c_char, error_code: c_ulong, address: c_ulong, ip: c_ulong) -> ! {
    unsafe {
        bindings::__putstr(msg);
        bindings::__putstr(c"\nError Code: ".as_ptr());
        bindings::__puthex(error_code);
        bindings::__putstr(c"\nCR2: 0x".as_ptr());
        bindings::__puthex(address);
        bindings::__putstr(c"\nRIP relative to _head: 0x".as_ptr());
        bindings::__puthex(ip.wrapping_sub(addr_of_mut!(abi::_head) as c_ulong));
        bindings::__putstr(c"\n".as_ptr());
        crate::error::error(c"Stopping.\n".as_ptr().cast_mut())
    }
}
#[no_mangle]
pub(crate) unsafe extern "C" fn do_boot_page_fault(regs: *mut pt_regs, error_code: c_ulong) {
    unsafe {
        let mut address: c_ulong;
        core::arch::asm!("mov {}, cr2", out(reg) address, options(nostack, preserves_flags));
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        let ghcb_fault = abi::sev_es_check_ghcb_fault(address);
        #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
        let ghcb_fault = false;
        address &= PMD_MASK;
        let end = address.wrapping_add(PMD_SIZE);
        if error_code & (abi::X86_PF_PROT | abi::X86_PF_USER | abi::X86_PF_RSVD) as c_ulong != 0 {
            do_pf_error(
                c"Unexpected page-fault:".as_ptr(),
                error_code,
                address,
                (*regs).ip,
            );
        } else if ghcb_fault {
            do_pf_error(
                c"Page-fault on GHCB page:".as_ptr(),
                error_code,
                address,
                (*regs).ip,
            );
        }
        kernel_add_identity_map(address, end);
    }
}
#[no_mangle]
pub(crate) unsafe extern "C" fn do_boot_nmi_trap(_regs: *mut pt_regs, _error_code: c_ulong) {
    unsafe {
        abi::spurious_nmi_count = abi::spurious_nmi_count.wrapping_add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
