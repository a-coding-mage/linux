// SPDX-License-Identifier: GPL-2.0
/*
 * This code is used on x86_64 to create page table identity mappings on
 * demand by building up a new set of page tables (or appending to the
 * existing ones), and then switching over to them when ready.
 *
 * Copyright (C) 2015-2016  Yinghai Lu
 * Copyright (C)      2016  Kees Cook
 */

// No MITIGATION_PAGE_TABLE_ISOLATION support needed either.

// Dependencies supplied by the surrounding kernel translation.

/* These actually do the work of building the kernel identity maps. */
// The original source includes linux/pgtable.h, asm/cmpxchg.h,
// asm/trap_pf.h, asm/trapnr.h, asm/init.h, and ../../mm/ident_map.c.
// Use the static base for this part of the boot process.

extern "C" {
    fn get_cmd_line_ptr() -> c_ulong;
    fn debug_putstr(s: *const c_char);
    fn debug_putaddr(value: c_ulong);
    fn error(s: *const c_char) -> !;
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn read_cr3_pa() -> c_ulong;
    fn write_cr3(value: c_ulong);
    fn sev_prep_identity_maps(value: c_ulong);
    fn snp_check_features();
    fn native_read_cr2() -> c_ulong;
    fn sev_es_check_ghcb_fault(address: c_ulong) -> bool;
    fn snp_set_page_shared(address: c_ulong);
    fn snp_set_page_private(address: c_ulong);
    fn clflush(address: *mut c_char);
}

/* Used by PAGE_KERN* macros. */
static mut __default_kernel_pte_mask: pteval_t = !0;

#[repr(C)]
struct alloc_pgt_data {
    pgt_buf: *mut c_uchar,
    pgt_buf_size: c_ulong,
    pgt_buf_offset: c_ulong,
}

/*
 * Allocates space for a page table entry, using struct alloc_pgt_data
 * above. Besides the local callers, this is used as the allocation
 * callback in mapping_info below.
 */
unsafe extern "C" fn alloc_pgt_page(context: *mut c_void) -> *mut c_void {
    let pages = &mut *(context as *mut alloc_pgt_data);

    /* Validate there is space available for a new page. */
    if pages.pgt_buf_offset >= pages.pgt_buf_size {
        debug_putstr(c"out of pgt_buf in ident_map_64.c!?\n".as_ptr());
        debug_putaddr(pages.pgt_buf_offset);
        debug_putaddr(pages.pgt_buf_size);
        return core::ptr::null_mut();
    }

    /* Consumed more tables than expected? */
    if pages.pgt_buf_offset == BOOT_PGT_SIZE_WARN {
        debug_putstr(c"pgt_buf running low in ident_map_64.c\n".as_ptr());
        debug_putstr(c"Need to raise BOOT_PGT_SIZE?\n".as_ptr());
        debug_putaddr(pages.pgt_buf_offset);
        debug_putaddr(pages.pgt_buf_size);
    }

    let entry = pages.pgt_buf.add(pages.pgt_buf_offset as usize);
    pages.pgt_buf_offset += PAGE_SIZE;
    entry as *mut c_void
}

/* Used to track our allocated page tables. */
static mut pgt_data: alloc_pgt_data = alloc_pgt_data {
    pgt_buf: core::ptr::null_mut(), pgt_buf_size: 0, pgt_buf_offset: 0,
};

/* The top level page table entry pointer. */
static mut top_level_pgt: c_ulong = 0;

static mut physical_mask: phys_addr_t = (1u64 << __PHYSICAL_MASK_SHIFT) - 1;

/* Mapping information structure passed to kernel_ident_mapping_init(). */
static mut mapping_info: x86_mapping_info = unsafe { core::mem::zeroed() };

/* Adds the specified range to the identity mappings. */
#[no_mangle]
pub unsafe extern "C" fn kernel_add_identity_map(mut start: c_ulong, mut end: c_ulong) {
    start = round_down(start, PMD_SIZE);
    end = round_up(end, PMD_SIZE);
    if start >= end { return; }

    let ret = kernel_ident_mapping_init(&mut mapping_info, top_level_pgt as *mut pgd_t, start, end);
    if ret != 0 { error(c"Error: kernel_ident_mapping_init() failed\n".as_ptr()); }
}

/* Locates and clears a region for a new top level page table. */
#[no_mangle]
pub unsafe extern "C" fn initialize_identity_maps(rmode: *mut setup_data) {
    let mut cmdline: c_ulong;
    physical_mask &= !sme_me_mask;

    mapping_info.alloc_pgt_page = Some(alloc_pgt_page);
    mapping_info.context = &mut pgt_data as *mut _ as *mut c_void;
    mapping_info.page_flag = __PAGE_KERNEL_LARGE_EXEC | sme_me_mask;
    mapping_info.kernpg_flag = _KERNPG_TABLE;
    pgt_data.pgt_buf_offset = 0;

    top_level_pgt = read_cr3_pa();
    if p4d_offset(top_level_pgt as *mut pgd_t, 0) == _pgtable as *mut p4d_t {
        pgt_data.pgt_buf = _pgtable.add(BOOT_INIT_PGT_SIZE as usize);
        pgt_data.pgt_buf_size = BOOT_PGT_SIZE - BOOT_INIT_PGT_SIZE;
        memset(pgt_data.pgt_buf as *mut c_void, 0, pgt_data.pgt_buf_size as usize);
    } else {
        pgt_data.pgt_buf = _pgtable;
        pgt_data.pgt_buf_size = BOOT_PGT_SIZE;
        memset(pgt_data.pgt_buf as *mut c_void, 0, pgt_data.pgt_buf_size as usize);
        top_level_pgt = alloc_pgt_page(&mut pgt_data as *mut _ as *mut c_void) as c_ulong;
    }

    kernel_add_identity_map(_head as c_ulong, _end as c_ulong);
    boot_params_ptr = rmode;
    kernel_add_identity_map(boot_params_ptr as c_ulong, boot_params_ptr.add(1) as c_ulong);
    cmdline = get_cmd_line_ptr();
    kernel_add_identity_map(cmdline, cmdline + COMMAND_LINE_SIZE);

    let mut sd = (*boot_params_ptr).hdr.setup_data as *mut setup_data;
    while !sd.is_null() {
        let sd_addr = sd as c_ulong;
        kernel_add_identity_map(sd_addr, sd_addr + core::mem::size_of::<setup_data>() as c_ulong + (*sd).len);
        sd = (*sd).next as *mut setup_data;
    }
    sev_prep_identity_maps(top_level_pgt);
    write_cr3(top_level_pgt);
    snp_check_features();
}

unsafe fn split_large_pmd(info: *mut x86_mapping_info, pmdp: *mut pmd_t, address: c_ulong) -> *mut pte_t {
    let pte = ((*info).alloc_pgt_page.unwrap())((*info).context) as *mut pte_t;
    if pte.is_null() { return core::ptr::null_mut(); }
    let mut addr = address & PMD_MASK;
    let flags = (*info).page_flag & !_PAGE_PSE;
    for i in 0..PTRS_PER_PMD { set_pte(pte.add(i as usize), __pte(addr | flags)); addr += PAGE_SIZE; }
    set_pmd(pmdp, __pmd(pte as c_ulong | (*info).kernpg_flag));
    write_cr3(top_level_pgt);
    pte.add(pte_index(address) as usize)
}

unsafe fn clflush_page(address: c_ulong) {
    let start = (address & PAGE_MASK) as *mut c_char;
    let end = start.add(PAGE_SIZE as usize);
    core::arch::asm!("mfence", options(nostack, preserves_flags));
    let mut cl = start;
    while cl != end { clflush(cl); cl = cl.add(64); }
}

unsafe fn set_clr_page_flags(info: *mut x86_mapping_info, address: c_ulong, set: pteval_t, clr: pteval_t) -> c_int {
    let pgdp = top_level_pgt as *mut pgd_t;
    core::ptr::read_volatile(address as *const c_ulong);
    let pmdp = pmd_offset(pud_offset(p4d_offset(pgdp, address), address), address);
    let ptep = if pmd_leaf(*pmdp) { split_large_pmd(info, pmdp, address) } else { pte_offset_kernel(pmdp, address) };
    if ptep.is_null() { return -ENOMEM; }
    if (set | clr) & _PAGE_ENC != 0 {
        clflush_page(address);
        if clr != 0 { snp_set_page_shared(__pa(address & PAGE_MASK)); }
    }
    let mut pte = *ptep;
    pte = pte_set_flags(pte, set);
    pte = pte_clear_flags(pte, clr);
    set_pte(ptep, pte);
    if set & _PAGE_ENC != 0 { snp_set_page_private(__pa(address & PAGE_MASK)); }
    write_cr3(top_level_pgt);
    0
}

pub unsafe extern "C" fn set_page_decrypted(address: c_ulong) -> c_int { set_clr_page_flags(&mut mapping_info, address, 0, _PAGE_ENC) }
pub unsafe extern "C" fn set_page_encrypted(address: c_ulong) -> c_int { set_clr_page_flags(&mut mapping_info, address, _PAGE_ENC, 0) }
pub unsafe extern "C" fn set_page_non_present(address: c_ulong) -> c_int { set_clr_page_flags(&mut mapping_info, address, 0, _PAGE_PRESENT) }

unsafe fn do_pf_error(msg: *const c_char, error_code: c_ulong, address: c_ulong, ip: c_ulong) {
    error_putstr(msg); error_putstr(c"\nError Code: ".as_ptr()); error_puthex(error_code);
    error_putstr(c"\nCR2: 0x".as_ptr()); error_puthex(address);
    error_putstr(c"\nRIP relative to _head: 0x".as_ptr()); error_puthex(ip - _head as c_ulong);
    error_putstr(c"\n".as_ptr()); error(c"Stopping.\n".as_ptr());
}

pub unsafe extern "C" fn do_boot_page_fault(regs: *mut pt_regs, error_code: c_ulong) {
    let mut address = native_read_cr2();
    let ghcb_fault = sev_es_check_ghcb_fault(address);
    address &= PMD_MASK;
    let end = address + PMD_SIZE;
    if error_code & (X86_PF_PROT | X86_PF_USER | X86_PF_RSVD) != 0 {
        do_pf_error(c"Unexpected page-fault:".as_ptr(), error_code, address, (*regs).ip);
    } else if ghcb_fault { do_pf_error(c"Page-fault on GHCB page:".as_ptr(), error_code, address, (*regs).ip); }
    kernel_add_identity_map(address, end);
}

pub unsafe extern "C" fn do_boot_nmi_trap(_regs: *mut pt_regs, _error_code: c_ulong) { spurious_nmi_count += 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
