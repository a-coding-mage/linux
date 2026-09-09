// SPDX-License-Identifier: GPL-2.0-only
/*
 * Re-map IO memory to kernel address space so that we can access it.
 * This is needed for high PCI addresses that aren't mapped in the
 * 640k-1MB IO memory area on PC's
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[repr(C)]
struct IoremapDesc {
    flags: c_uint,
}

fn ioremap_change_attr(vaddr: c_ulong, size: c_ulong, pcm: page_cache_mode) -> c_int {
    let nrpages = size >> PAGE_SHIFT;
    let err;
    match pcm {
        _PAGE_CACHE_MODE_UC => { err = _set_memory_uc(vaddr, nrpages); }
        _PAGE_CACHE_MODE_WC => { err = _set_memory_wc(vaddr, nrpages); }
        _PAGE_CACHE_MODE_WT => { err = _set_memory_wt(vaddr, nrpages); }
        _PAGE_CACHE_MODE_WB => { err = _set_memory_wb(vaddr, nrpages); }
        _ => { err = _set_memory_uc(vaddr, nrpages); }
    }
    err
}

unsafe fn __ioremap_check_ram(res: *mut resource) -> c_uint {
    let mut start_pfn: c_ulong;
    let mut stop_pfn: c_ulong;
    let mut pfn: c_ulong;
    if ((*res).flags & IORESOURCE_SYSTEM_RAM) != IORESOURCE_SYSTEM_RAM { return 0; }
    start_pfn = ((*res).start + PAGE_SIZE - 1) >> PAGE_SHIFT;
    stop_pfn = ((*res).end + 1) >> PAGE_SHIFT;
    if stop_pfn > start_pfn {
        for_each_valid_pfn!(pfn, start_pfn, stop_pfn) {
            if !PageReserved(pfn_to_page(pfn)) { return IORES_MAP_SYSTEM_RAM; }
        }
    }
    0
}

unsafe fn __ioremap_check_encrypted(res: *mut resource) -> c_uint {
    if !cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { return 0; }
    match (*res).desc {
        IORES_DESC_NONE | IORES_DESC_RESERVED => {}
        _ => return IORES_MAP_ENCRYPTED,
    }
    0
}

unsafe fn __ioremap_check_other(addr: resource_size_t, desc: *mut IoremapDesc) {
    if !cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { return; }
    if x86_platform.hyper.is_private_mmio(addr) {
        (*desc).flags |= IORES_MAP_ENCRYPTED;
        return;
    }
    if !IS_ENABLED(CONFIG_EFI) { return; }
    if efi_mem_type(addr) == EFI_RUNTIME_SERVICES_DATA ||
       (efi_mem_type(addr) == EFI_BOOT_SERVICES_DATA &&
        (efi_mem_attributes(addr) & EFI_MEMORY_RUNTIME) != 0) {
        (*desc).flags |= IORES_MAP_ENCRYPTED;
    }
}

unsafe extern "C" fn __ioremap_collect_map_flags(res: *mut resource, arg: *mut c_void) -> c_int {
    let desc = arg as *mut IoremapDesc;
    if (*desc).flags & IORES_MAP_SYSTEM_RAM == 0 { (*desc).flags |= __ioremap_check_ram(res); }
    if (*desc).flags & IORES_MAP_ENCRYPTED == 0 { (*desc).flags |= __ioremap_check_encrypted(res); }
    (((*desc).flags & (IORES_MAP_SYSTEM_RAM | IORES_MAP_ENCRYPTED)) ==
     (IORES_MAP_SYSTEM_RAM | IORES_MAP_ENCRYPTED)) as c_int
}

unsafe fn __ioremap_check_mem(addr: resource_size_t, size: c_ulong, desc: *mut IoremapDesc) {
    let start = addr as u64;
    let end = start + size as u64 - 1;
    core::ptr::write_bytes(desc, 0, 1);
    walk_mem_res(start, end, desc as *mut c_void, __ioremap_collect_map_flags);
    __ioremap_check_other(addr, desc);
}

unsafe fn __ioremap_caller(mut phys_addr: resource_size_t, mut size: c_ulong,
    mut pcm: page_cache_mode, caller: *mut c_void, encrypted: bool) -> *mut c_void {
    let mut offset: c_ulong;
    let mut vaddr: c_ulong;
    let last_addr = phys_addr.wrapping_add(size).wrapping_sub(1);
    let unaligned_phys_addr = phys_addr;
    let unaligned_size = size;
    let mut io_desc = IoremapDesc { flags: 0 };
    let mut new_pcm: page_cache_mode;
    let mut prot: pgprot_t;
    let retval: c_int;
    if size == 0 || last_addr < phys_addr { return core::ptr::null_mut(); }
    if !phys_addr_valid(phys_addr) {
        printk!(KERN_WARNING, "ioremap: invalid physical address %llx\n", phys_addr as u64);
        WARN_ON_ONCE!(1);
        return core::ptr::null_mut();
    }
    __ioremap_check_mem(phys_addr, size, &mut io_desc);
    if io_desc.flags & IORES_MAP_SYSTEM_RAM != 0 {
        WARN_ONCE!(1, "ioremap on RAM at %pa - %pa\n", &phys_addr, &last_addr);
        return core::ptr::null_mut();
    }
    offset = phys_addr & !PAGE_MASK;
    phys_addr &= PAGE_MASK;
    size = PAGE_ALIGN(last_addr + 1) - phys_addr;
    phys_addr &= PHYSICAL_PAGE_MASK;
    retval = memtype_reserve(phys_addr, phys_addr as u64 + size as u64, pcm, &mut new_pcm);
    if retval != 0 { printk!(KERN_ERR, "ioremap memtype_reserve failed %d\n", retval); return core::ptr::null_mut(); }
    if pcm != new_pcm {
        if !is_new_memtype_allowed(phys_addr, size, pcm, new_pcm) {
            printk!(KERN_ERR, "ioremap error for 0x%llx-0x%llx, requested 0x%x, got 0x%x\n", phys_addr, phys_addr + size, pcm, new_pcm);
            memtype_free(phys_addr, phys_addr + size); return core::ptr::null_mut();
        }
        pcm = new_pcm;
    }
    prot = PAGE_KERNEL_IO;
    prot = if (io_desc.flags & IORES_MAP_ENCRYPTED) != 0 || encrypted { pgprot_encrypted(prot) } else { pgprot_decrypted(prot) };
    match pcm {
        _PAGE_CACHE_MODE_UC => { prot = __pgprot(pgprot_val(prot) | cachemode2protval(_PAGE_CACHE_MODE_UC)); }
        _PAGE_CACHE_MODE_UC_MINUS => { prot = __pgprot(pgprot_val(prot) | cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS)); }
        _PAGE_CACHE_MODE_WC => { prot = __pgprot(pgprot_val(prot) | cachemode2protval(_PAGE_CACHE_MODE_WC)); }
        _PAGE_CACHE_MODE_WT => { prot = __pgprot(pgprot_val(prot) | cachemode2protval(_PAGE_CACHE_MODE_WT)); }
        _PAGE_CACHE_MODE_WB => {}
        _ => { prot = __pgprot(pgprot_val(prot) | cachemode2protval(_PAGE_CACHE_MODE_UC)); }
    }
    let area = get_vm_area_caller(size, VM_IOREMAP, caller);
    if area.is_null() { memtype_free(phys_addr, phys_addr + size); return core::ptr::null_mut(); }
    (*area).phys_addr = phys_addr;
    vaddr = (*area).addr as c_ulong;
    if memtype_kernel_map_sync(phys_addr, size, pcm) != 0 || ioremap_page_range(vaddr, vaddr + size, phys_addr, prot) != 0 {
        free_vm_area(area); memtype_free(phys_addr, phys_addr + size); return core::ptr::null_mut();
    }
    let ret_addr = (vaddr + offset) as *mut c_void;
    mmiotrace_ioremap(unaligned_phys_addr, unaligned_size, ret_addr);
    if iomem_map_sanity_check(unaligned_phys_addr, unaligned_size) != 0 { pr_warn!("caller %pS mapping multiple BARs\n", caller); }
    ret_addr
}

pub unsafe fn ioremap(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void {
    __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_UC_MINUS, __builtin_return_address(0), false)
}
pub unsafe fn ioremap_uc(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void { __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_UC, __builtin_return_address(0), false) }
pub unsafe fn ioremap_wc(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void { __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WC, __builtin_return_address(0), false) }
pub unsafe fn ioremap_wt(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void { __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WT, __builtin_return_address(0), false) }
pub unsafe fn ioremap_encrypted(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void { __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WB, __builtin_return_address(0), true) }
pub unsafe fn ioremap_cache(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void { __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WB, __builtin_return_address(0), false) }
pub unsafe fn ioremap_prot(phys_addr: resource_size_t, size: c_ulong, prot: pgprot_t) -> *mut c_void { __ioremap_caller(phys_addr, size, pgprot2cachemode(prot), __builtin_return_address(0), false) }

pub unsafe fn iounmap(mut addr: *mut c_void) {
    if WARN_ON_ONCE!(!is_ioremap_addr(addr)) { return; }
    if addr as c_ulong >= phys_to_virt(ISA_START_ADDRESS) && addr as c_ulong < phys_to_virt(ISA_END_ADDRESS) { WARN!(1, "iounmap() called for ISA range not obtained using ioremap()\n"); return; }
    mmiotrace_iounmap(addr);
    addr = ((addr as c_ulong) & PAGE_MASK) as *mut c_void;
    let p = find_vm_area(addr);
    if p.is_null() { printk!(KERN_ERR, "iounmap: bad address %p\n", addr); dump_stack(); return; }
    kmsan_iounmap_page_range(addr as c_ulong, addr as c_ulong + get_vm_area_size(p));
    memtype_free((*p).phys_addr, (*p).phys_addr + get_vm_area_size(p));
    let o = remove_vm_area(addr);
    BUG_ON!(p != o || o.is_null());
    kfree(p as *mut c_void);
}

pub unsafe fn arch_memremap_wb(phys_addr: phys_addr_t, size: usize, flags: c_ulong) -> *mut c_void {
    if (flags & MEMREMAP_DEC) != 0 || cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { ioremap_cache(phys_addr, size as c_ulong) } else { ioremap_encrypted(phys_addr, size as c_ulong) }
}

pub unsafe fn xlate_dev_mem_ptr(phys: phys_addr_t) -> *mut c_void {
    let start = phys & PAGE_MASK; let offset = phys & !PAGE_MASK;
    let mut vaddr = memremap(start, PAGE_SIZE, MEMREMAP_WB);
    if !vaddr.is_null() { vaddr = (vaddr as c_ulong + offset) as *mut c_void; }
    vaddr
}
pub unsafe fn unxlate_dev_mem_ptr(_phys: phys_addr_t, addr: *mut c_void) { memunmap((addr as c_ulong & PAGE_MASK) as *mut c_void); }

// CONFIG_AMD_MEM_ENCRYPT-dependent helpers are preserved below.
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn memremap_should_map_decrypted(phys_addr: resource_size_t, size: c_ulong) -> bool {
    let is_pmem = region_intersects(phys_addr, size, IORESOURCE_MEM, IORES_DESC_PERSISTENT_MEMORY);
    if is_pmem != REGION_DISJOINT { return true; }
    if efi_enabled(EFI_BOOT) && efi_mem_type(phys_addr) == EFI_RESERVED_TYPE && (efi_mem_attributes(phys_addr) & EFI_MEMORY_NV) != 0 { return true; }
    match e820__get_entry_type(phys_addr, phys_addr + size - 1) {
        E820_TYPE_RESERVED | E820_TYPE_ACPI | E820_TYPE_NVS | E820_TYPE_UNUSABLE => { if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { return false; } true }
        E820_TYPE_PRAM => true,
        _ => false,
    }
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn memremap_is_efi_data(phys_addr: resource_size_t) -> bool {
    if !efi_enabled(EFI_BOOT) { return false; }
    let mut paddr = ((boot_params.efi_info.efi_memmap_hi as u64) << 32) | boot_params.efi_info.efi_memmap as u64;
    if phys_addr == paddr { return true; }
    paddr = ((boot_params.efi_info.efi_systab_hi as u64) << 32) | boot_params.efi_info.efi_systab as u64;
    if phys_addr == paddr || efi_is_table_address(phys_addr) { return true; }
    matches!(efi_mem_type(phys_addr), EFI_BOOT_SERVICES_DATA | EFI_RUNTIME_SERVICES_DATA)
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn __memremap_is_setup_data(phys_addr: resource_size_t, early: bool) -> bool {
    let setup_data_sz = core::mem::size_of::<setup_data>() as c_uint;
    let mut paddr = boot_params.hdr.setup_data;
    while paddr != 0 {
        if phys_addr == paddr { return true; }
        let mut data = if early { early_memremap_decrypted(paddr, setup_data_sz as c_ulong) } else { memremap(paddr, setup_data_sz as c_ulong, MEMREMAP_WB | MEMREMAP_DEC) } as *mut setup_data;
        if data.is_null() { pr_warn!("failed to remap setup_data entry\n"); return false; }
        let mut size = setup_data_sz;
        let paddr_next = (*data).next; let mut len = (*data).len;
        if phys_addr > paddr && phys_addr < paddr + setup_data_sz as u64 + len as u64 { if early { early_memunmap(data as *mut c_void, setup_data_sz as c_ulong); } else { memunmap(data as *mut c_void); } return true; }
        if (*data).type_ == SETUP_INDIRECT { size += len; data = if early { early_memunmap(data as *mut c_void, setup_data_sz as c_ulong); early_memremap_decrypted(paddr, size as c_ulong) } else { memunmap(data as *mut c_void); memremap(paddr, size as c_ulong, MEMREMAP_WB | MEMREMAP_DEC) } as *mut setup_data; if data.is_null() { pr_warn!("failed to remap indirect setup_data\n"); return false; } let indirect = (*data).data as *mut setup_indirect; if (*indirect).type_ != SETUP_INDIRECT { paddr = (*indirect).addr; len = (*indirect).len; } }
        if early { early_memunmap(data as *mut c_void, size as c_ulong); } else { memunmap(data as *mut c_void); }
        if phys_addr > paddr && phys_addr < paddr + len as u64 { return true; }
        paddr = paddr_next;
    }
    false
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn memremap_is_setup_data(p: resource_size_t) -> bool { __memremap_is_setup_data(p, false) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
unsafe fn early_memremap_is_setup_data(p: resource_size_t) -> bool { __memremap_is_setup_data(p, true) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn arch_memremap_can_ram_remap(p: resource_size_t, size: c_ulong, flags: c_ulong) -> bool { if !cc_platform_has(CC_ATTR_MEM_ENCRYPT) || flags & MEMREMAP_ENC != 0 { return true; } if flags & MEMREMAP_DEC != 0 { return false; } if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) && (memremap_is_setup_data(p) || memremap_is_efi_data(p)) { return false; } !memremap_should_map_decrypted(p, size) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn early_memremap_pgprot_adjust(p: resource_size_t, size: c_ulong, prot: pgprot_t) -> pgprot_t { if !cc_platform_has(CC_ATTR_MEM_ENCRYPT) { return prot; } let mut enc = true; if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) && (early_memremap_is_setup_data(p) || memremap_is_efi_data(p)) { enc = false; } if enc && memremap_should_map_decrypted(p, size) { enc = false; } if enc { pgprot_encrypted(prot) } else { pgprot_decrypted(prot) } }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn phys_mem_access_encrypted(p: c_ulong, size: c_ulong) -> bool { arch_memremap_can_ram_remap(p, size, 0) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn early_memremap_encrypted(p: resource_size_t, s: c_ulong) -> *mut c_void { early_memremap_prot(p, s, __PAGE_KERNEL_ENC) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn early_memremap_encrypted_wp(p: resource_size_t, s: c_ulong) -> *mut c_void { if !x86_has_pat_wp() { core::ptr::null_mut() } else { early_memremap_prot(p, s, __PAGE_KERNEL_ENC_WP) } }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn early_memremap_decrypted(p: resource_size_t, s: c_ulong) -> *mut c_void { early_memremap_prot(p, s, __PAGE_KERNEL_NOENC) }
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
pub unsafe fn early_memremap_decrypted_wp(p: resource_size_t, s: c_ulong) -> *mut c_void { if !x86_has_pat_wp() { core::ptr::null_mut() } else { early_memremap_prot(p, s, __PAGE_KERNEL_NOENC_WP) } }

static mut BM_PTE: [pte_t; PAGE_SIZE / core::mem::size_of::<pte_t>()] = [unsafe { core::mem::zeroed() }; PAGE_SIZE / core::mem::size_of::<pte_t>()];
unsafe fn early_ioremap_pmd(addr: c_ulong) -> *mut pmd_t { let base = __va(read_cr3_pa()) as *mut pgd_t; let pgd = base.add(pgd_index(addr)); let p4d = p4d_offset(pgd, addr); let pud = pud_offset(p4d, addr); pmd_offset(pud, addr) }
unsafe fn early_ioremap_pte(addr: c_ulong) -> *mut pte_t { &mut BM_PTE[pte_index(addr)] }
pub unsafe fn is_early_ioremap_ptep(ptep: *mut pte_t) -> bool { ptep >= &mut BM_PTE[0] && ptep < &mut BM_PTE[BM_PTE.len()] }
pub unsafe fn early_ioremap_init() { let pmd = early_ioremap_pmd(fix_to_virt(FIX_BTMAP_BEGIN)); core::ptr::write_bytes(BM_PTE.as_mut_ptr(), 0, BM_PTE.len()); pmd_populate_kernel(&init_mm, pmd, BM_PTE.as_mut_ptr()); if pmd != early_ioremap_pmd(fix_to_virt(FIX_BTMAP_END)) { WARN_ON!(1); printk!(KERN_WARNING, "pmd %p != %p\n", pmd, early_ioremap_pmd(fix_to_virt(FIX_BTMAP_END))); printk!(KERN_WARNING, "fix_to_virt(FIX_BTMAP_BEGIN): %08lx\n", fix_to_virt(FIX_BTMAP_BEGIN)); printk!(KERN_WARNING, "fix_to_virt(FIX_BTMAP_END):   %08lx\n", fix_to_virt(FIX_BTMAP_END)); printk!(KERN_WARNING, "FIX_BTMAP_END:       %d\n", FIX_BTMAP_END); printk!(KERN_WARNING, "FIX_BTMAP_BEGIN:     %d\n", FIX_BTMAP_BEGIN); } }
pub unsafe fn __early_set_fixmap(idx: fixed_addresses, phys: phys_addr_t, mut flags: pgprot_t) { let addr = __fix_to_virt(idx); if idx >= __end_of_fixed_addresses { BUG!(); return; } let pte = early_ioremap_pte(addr); pgprot_val(flags) &= __supported_pte_mask; if pgprot_val(flags) != 0 { set_pte(pte, pfn_pte(phys >> PAGE_SHIFT, flags)); } else { pte_clear(&init_mm, addr, pte); } flush_tlb_one_kernel(addr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
