// SPDX-License-Identifier: GPL-2.0-only
/*
 * handle transition of Linux booting another kernel
 * Copyright (C) 2002-2005 Eric Biederman  <ebiederm@xmission.com>
 */

// C dependencies and build-time configuration are supplied by the surrounding kernel.

#[cfg(CONFIG_ACPI)]
#[repr(C)]
struct InitPgtableData {
    info: *mut x86_mapping_info,
    level4p: *mut pgd_t,
}

#[cfg(CONFIG_ACPI)]
unsafe extern "C" fn mem_region_callback(res: *mut resource, arg: *mut core::ffi::c_void) -> i32 {
    let data = &mut *(arg as *mut InitPgtableData);
    kernel_ident_mapping_init(data.info, data.level4p, (*res).start, (*res).end + 1)
}

unsafe fn map_acpi_tables(info: *mut x86_mapping_info, level4p: *mut pgd_t) -> i32 {
    #[cfg(CONFIG_ACPI)] {
        let mut data = InitPgtableData { info, level4p };
        let flags: c_ulong = IORESOURCE_MEM | IORESOURCE_BUSY;
        let mut ret = walk_iomem_res_desc(IORES_DESC_ACPI_TABLES, flags, 0, -1, &mut data as *mut _ as *mut _, Some(mem_region_callback));
        if ret != 0 && ret != -EINVAL { return ret; }
        ret = walk_iomem_res_desc(IORES_DESC_ACPI_NV_STORAGE, flags, 0, -1, &mut data as *mut _ as *mut _, Some(mem_region_callback));
        if ret != 0 && ret != -EINVAL { return ret; }
    }
    0
}

unsafe fn map_mmio_serial(info: *mut x86_mapping_info, level4p: *mut pgd_t) -> i32 {
    if kexec_debug_8250_mmio32 == 0 { return 0; }
    let mstart = kexec_debug_8250_mmio32 & PAGE_MASK;
    let mend = (kexec_debug_8250_mmio32 + PAGE_SIZE + 23) & PAGE_MASK;
    pr_info!("Map PCI serial at {:x} - {:x}\n", mstart, mend);
    kernel_ident_mapping_init(info, level4p, mstart, mend)
}

#[cfg(CONFIG_KEXEC_FILE)]
#[no_mangle]
pub static kexec_file_loaders: [*const kexec_file_ops; 2] = [
    &kexec_bzImage64_ops,
    core::ptr::null(),
];

unsafe fn map_efi_systab(info: *mut x86_mapping_info, level4p: *mut pgd_t) -> i32 {
    #[cfg(CONFIG_EFI)] {
        if !efi_enabled(EFI_BOOT) { return 0; }
        let mut mstart = boot_params.efi_info.efi_systab | ((boot_params.efi_info.efi_systab_hi as u64) << 32);
        let mut mend = if efi_enabled(EFI_64BIT) { mstart + core::mem::size_of::<efi_system_table_64_t>() as u64 } else { mstart + core::mem::size_of::<efi_system_table_32_t>() as u64 };
        if mstart == 0 { return 0; }
        let ret = kernel_ident_mapping_init(info, level4p, mstart, mend);
        if ret != 0 { return ret; }
        let kaddr = memremap(mstart, mend - mstart, MEMREMAP_WB);
        if kaddr.is_null() { pr_err!("Could not map UEFI system table\n"); return -ENOMEM; }
        mstart = efi_config_table;
        if efi_enabled(EFI_64BIT) {
            let stbl = &*(kaddr as *const efi_system_table_64_t);
            mend = mstart + core::mem::size_of::<efi_config_table_64_t>() as u64 * stbl.nr_tables as u64;
        } else {
            let stbl = &*(kaddr as *const efi_system_table_32_t);
            mend = mstart + core::mem::size_of::<efi_config_table_32_t>() as u64 * stbl.nr_tables as u64;
        }
        memunmap(kaddr);
        return kernel_ident_mapping_init(info, level4p, mstart, mend);
    }
    0
}

unsafe fn free_transition_pgtable(image: *mut kimage) {
    free_page((*image).arch.p4d as c_ulong); (*image).arch.p4d = core::ptr::null_mut();
    free_page((*image).arch.pud as c_ulong); (*image).arch.pud = core::ptr::null_mut();
    free_page((*image).arch.pmd as c_ulong); (*image).arch.pmd = core::ptr::null_mut();
    free_page((*image).arch.pte as c_ulong); (*image).arch.pte = core::ptr::null_mut();
}

unsafe fn init_transition_pgtable(image: *mut kimage, mut pgd: *mut pgd_t, control_page: c_ulong) -> i32 {
    let mut prot = PAGE_KERNEL_EXEC_NOENC;
    let vaddr = __va(control_page) as c_ulong;
    pgd = pgd.add(pgd_index(vaddr) as usize);
    if !pgd_present(*pgd) { let p = get_zeroed_page(GFP_KERNEL); if p == 0 { return -ENOMEM; } (*image).arch.p4d = p as *mut _; set_pgd(pgd, __pgd(__pa(p) | _KERNPG_TABLE)); }
    let p4d = p4d_offset(pgd, vaddr);
    if !p4d_present(*p4d) { let p = get_zeroed_page(GFP_KERNEL); if p == 0 { return -ENOMEM; } (*image).arch.pud = p as *mut _; set_p4d(p4d, __p4d(__pa(p) | _KERNPG_TABLE)); }
    let pud = pud_offset(p4d, vaddr);
    if !pud_present(*pud) { let p = get_zeroed_page(GFP_KERNEL); if p == 0 { return -ENOMEM; } (*image).arch.pmd = p as *mut _; set_pud(pud, __pud(__pa(p) | _KERNPG_TABLE)); }
    let pmd = pmd_offset(pud, vaddr);
    if !pmd_present(*pmd) { let p = get_zeroed_page(GFP_KERNEL); if p == 0 { return -ENOMEM; } (*image).arch.pte = p as *mut _; set_pmd(pmd, __pmd(__pa(p) | _KERNPG_TABLE)); }
    let pte = pte_offset_kernel(pmd, vaddr);
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { prot = PAGE_KERNEL_EXEC; }
    set_pte(pte, pfn_pte(control_page >> PAGE_SHIFT, prot));
    0
}

unsafe extern "C" fn alloc_pgt_page(data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let image = data as *mut kimage;
    let page = kimage_alloc_control_pages(image, 0);
    if page.is_null() { return core::ptr::null_mut(); }
    let p = page_address(page); clear_page(p); p
}

unsafe fn init_pgtable(image: *mut kimage, control_page: c_ulong) -> i32 {
    let mut info = x86_mapping_info { alloc_pgt_page: Some(alloc_pgt_page), context: image as *mut _, page_flag: __PAGE_KERNEL_LARGE_EXEC, kernpg_flag: _KERNPG_TABLE_NOENC, direct_gbpages: false };
    (*image).arch.pgd = alloc_pgt_page(image as *mut _);
    if (*image).arch.pgd.is_null() { return -ENOMEM; }
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { info.page_flag |= _PAGE_ENC; info.kernpg_flag |= _PAGE_ENC; }
    if direct_gbpages { info.direct_gbpages = true; }
    for i in 0..nr_pfn_mapped as usize { let r = kernel_ident_mapping_init(&mut info, (*image).arch.pgd, pfn_mapped[i].start << PAGE_SHIFT, pfn_mapped[i].end << PAGE_SHIFT); if r != 0 { return r; } }
    for i in 0..(*image).nr_segments as usize { let s = (*image).segment.add(i); let r = kernel_ident_mapping_init(&mut info, (*image).arch.pgd, (*s).mem, (*s).mem + (*s).memsz); if r != 0 { return r; } }
    let r = map_efi_systab(&mut info, (*image).arch.pgd); if r != 0 { return r; }
    let r = map_acpi_tables(&mut info, (*image).arch.pgd); if r != 0 { return r; }
    let r = map_mmio_serial(&mut info, (*image).arch.pgd); if r != 0 { return r; }
    init_transition_pgtable(image, (*image).arch.pgd, control_page)
}

unsafe fn load_segments() { core::arch::asm!("movl %eax,%ds; movl %eax,%es; movl %eax,%ss; movl %eax,%fs; movl %eax,%gs", in("eax") __KERNEL_DS, options(nostack, preserves_flags)); }

unsafe fn prepare_debug_idt(control_page: c_ulong, vec_ofs: c_ulong) {
    let mut e: gate_desc = core::mem::zeroed(); e.bits.p = 1; e.bits.type_ = GATE_TRAP; e.segment = __KERNEL_CS; e.offset_low = (control_page & 0xffff) as _ + vec_ofs as _; e.offset_middle = ((control_page >> 16) & 0xffff) as _; e.offset_high = control_page >> 32;
    for i in 0..16 { kexec_debug_idt[i] = e; e.offset_low += KEXEC_DEBUG_EXC_HANDLER_SIZE; }
}

pub unsafe fn machine_kexec_prepare(image: *mut kimage) -> i32 {
    let control_page = page_address((*image).control_code_page); let reloc_start = __relocate_kernel_start as c_ulong; let reloc_end = __relocate_kernel_end as c_ulong;
    let r = init_pgtable(image, __pa(control_page)); if r != 0 { return r; }
    kexec_va_control_page = control_page as c_ulong; kexec_pa_table_page = __pa((*image).arch.pgd);
    if (*image).type_ == KEXEC_TYPE_DEFAULT { kexec_pa_swap_page = page_to_pfn((*image).swap_page) << PAGE_SHIFT; }
    prepare_debug_idt(__pa(control_page), kexec_debug_exc_vectors as c_ulong - reloc_start); __memcpy(control_page, __relocate_kernel_start, reloc_end - reloc_start); set_memory_rox(control_page as c_ulong, 1); 0
}

pub unsafe fn machine_kexec_cleanup(image: *mut kimage) { let p = page_address((*image).control_code_page) as c_ulong; set_memory_nx(p, 1); set_memory_rw(p, 1); free_transition_pgtable(image); }

// Do not allocate memory (or fail) after the point of no return.
pub unsafe fn machine_kexec(image: *mut kimage) {
    let reloc_start = __relocate_kernel_start as c_ulong; let save = __ftrace_enabled_save(); local_irq_disable(); hw_breakpoint_disable(); cet_disable();
    let control_page = page_address((*image).control_code_page); let ptr = (control_page as c_ulong + relocate_kernel as c_ulong - reloc_start) as relocate_kernel_fn;
    let mut flags = 0; if (*image).preserve_context { flags |= RELOC_KERNEL_PRESERVE_CONTEXT; } if this_cpu_read(cache_state_incoherent) { flags |= RELOC_KERNEL_CACHE_INCOHERENT; }
    load_segments(); (*image).start = ptr((*image).head as c_ulong, virt_to_phys(control_page), (*image).start, flags); __ftrace_enabled_restore(save);
}

#[cfg(CONFIG_KEXEC_FILE)]
pub unsafe fn arch_kexec_apply_relocations_add(pi: *mut purgatory_info, section: *mut Elf_Shdr, relsec: *const Elf_Shdr, symtabsec: *const Elf_Shdr) -> i32 {
    let eh = (*pi).ehdr; let sh = (eh as *mut u8).add((*eh).e_shoff as usize) as *const Elf_Shdr; let strtab = (eh as *mut u8).add((*sh.add((*symtabsec).sh_link as usize)).sh_offset as usize) as *const c_char; let shstr = (eh as *mut u8).add((*sh.add((*eh).e_shstrndx as usize)).sh_offset as usize) as *const c_char; let rel = (eh as *mut u8).add((*relsec).sh_offset as usize) as *const Elf64_Rela;
    for i in 0..((*relsec).sh_size as usize / core::mem::size_of::<Elf64_Rela>()) { let r = *rel.add(i); let loc = (*pi).purgatory_buf.add((*section).sh_offset as usize + r.r_offset as usize); let addr = (*section).sh_addr + r.r_offset; let syms = (eh as *mut u8).add((*symtabsec).sh_offset as usize) as *const Elf64_Sym; let sym = *syms.add(ELF64_R_SYM(r.r_info) as usize); if sym.st_shndx == SHN_UNDEF || sym.st_shndx == SHN_COMMON { return -ENOEXEC; } let base = if sym.st_shndx == SHN_ABS { 0 } else if sym.st_shndx >= (*eh).e_shnum { return -ENOEXEC; } else { (*pi).sechdrs.add(sym.st_shndx as usize).read().sh_addr }; let value = sym.st_value + base + r.r_addend as u64; match ELF64_R_TYPE(r.r_info) { R_X86_64_NONE => {}, R_X86_64_64 => *(loc as *mut u64) = value, R_X86_64_32 => *(loc as *mut u32) = value as u32, R_X86_64_32S => *(loc as *mut i32) = value as i32, R_X86_64_PC32 | R_X86_64_PLT32 => *(loc as *mut u32) = value.wrapping_sub(addr) as u32, _ => return -ENOEXEC } }
    0
}

#[cfg(CONFIG_KEXEC_FILE)]
pub unsafe fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> i32 { vfree((*image).elf_headers); (*image).elf_headers = core::ptr::null_mut(); (*image).elf_headers_sz = 0; kexec_image_post_load_cleanup_default(image) }

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn kexec_mark_range(start: c_ulong, end: c_ulong, protect: bool) -> i32 { if end == 0 || start > end { return 0; } let page = pfn_to_page(start >> PAGE_SHIFT); let n = (end >> PAGE_SHIFT) - (start >> PAGE_SHIFT) + 1; if protect { set_pages_ro(page, n) } else { set_pages_rw(page, n) } }

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn kexec_mark_crashkres(protect: bool) {
    kexec_mark_range(crashk_low_res.start, crashk_low_res.end, protect);
    let control = PFN_PHYS(page_to_pfn((*kexec_crash_image).control_code_page));
    kexec_mark_range(crashk_res.start, control - 1, protect);
    kexec_mark_range(control + KEXEC_CONTROL_PAGE_SIZE, crashk_res.end, protect);
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn kexec_mark_dm_crypt_keys(protect: bool) {
    if (*kexec_crash_image).dm_crypt_keys_addr != 0 {
        let start = (*kexec_crash_image).dm_crypt_keys_addr;
        let end = start + (*kexec_crash_image).dm_crypt_keys_sz - 1;
        let pages = (PAGE_ALIGN(end) - PAGE_ALIGN_DOWN(start)) / PAGE_SIZE;
        if protect { set_memory_np(phys_to_virt(start) as c_ulong, pages); }
        else { set_memory_p(phys_to_virt(start) as c_ulong, pages); }
    }
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_kexec_protect_crashkres() { kexec_mark_crashkres(true); kexec_mark_dm_crypt_keys(true); }
#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_kexec_unprotect_crashkres() { kexec_mark_dm_crypt_keys(false); kexec_mark_crashkres(false); }

pub unsafe fn arch_kexec_post_alloc_pages(vaddr: *mut core::ffi::c_void, pages: c_uint, gfp: gfp_t) -> i32 { if !cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { return 0; } set_memory_decrypted(vaddr as c_ulong, pages) }
pub unsafe fn arch_kexec_pre_free_pages(vaddr: *mut core::ffi::c_void, pages: c_uint) { if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { set_memory_encrypted(vaddr as c_ulong, pages); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
