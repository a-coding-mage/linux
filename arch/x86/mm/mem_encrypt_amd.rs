// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2016-2024 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// Linux and x86 dependencies supplied by the surrounding kernel translation.

/* Since SME related variables are set early, they reside in .data. */
#[link_section = ".data"]
pub static mut sme_me_mask: u64 = 0;
#[link_section = ".data"]
pub static mut sev_status: u64 = 0;
#[link_section = ".data"]
pub static mut sev_check_data: u64 = 0;

/* Buffer used for early in-place encryption by BSP, no locking needed. */
static mut sme_early_buffer: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

/* SNP-specific copy which temporarily changes the page state. */
unsafe fn snp_memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void,
                     sz: usize, paddr: usize, decrypt: bool) {
    let npages = page_align(sz) >> PAGE_SHIFT;
    if decrypt {
        early_snp_set_memory_shared(__va(paddr) as usize, paddr, npages);
        memcpy(dst, src, sz);
        early_snp_set_memory_private(__va(paddr) as usize, paddr, npages);
    } else {
        memcpy(dst, src, sz);
    }
}

unsafe fn __sme_early_enc_dec(mut paddr: resource_size_t, mut size: usize, enc: bool) {
    if sme_me_mask == 0 { return; }
    wbinvd();
    while size != 0 {
        let len = core::cmp::min(core::mem::size_of_val(&sme_early_buffer), size);
        let src = if enc { early_memremap_decrypted_wp(paddr, len) }
                  else { early_memremap_encrypted_wp(paddr, len) };
        let dst = if enc { early_memremap_encrypted(paddr, len) }
                  else { early_memremap_decrypted(paddr, len) };
        BUG_ON(src.is_null() || dst.is_null());
        if cc_platform_has(CC_ATTR_GUEST_SEV_SNP) {
            snp_memcpy(sme_early_buffer.as_mut_ptr() as _, src, len, paddr, enc);
            snp_memcpy(dst, sme_early_buffer.as_ptr() as _, len, paddr, !enc);
        } else {
            memcpy(sme_early_buffer.as_mut_ptr() as _, src, len);
            memcpy(dst, sme_early_buffer.as_ptr() as _, len);
        }
        early_memunmap(dst, len);
        early_memunmap(src, len);
        paddr += len;
        size -= len;
    }
}

pub unsafe fn sme_early_encrypt(paddr: resource_size_t, size: usize) { __sme_early_enc_dec(paddr, size, true); }
pub unsafe fn sme_early_decrypt(paddr: resource_size_t, size: usize) { __sme_early_enc_dec(paddr, size, false); }

unsafe fn __sme_early_map_unmap_mem(mut vaddr: *mut core::ffi::c_void, mut size: usize, map: bool) {
    let mut paddr = vaddr as usize - __PAGE_OFFSET;
    let pmd_flags = __sme_clr(early_pmd_flags);
    loop {
        let pmd = if map { (paddr & PMD_MASK) + pmd_flags } else { 0 };
        __early_make_pgtable(vaddr as usize, pmd);
        vaddr = (vaddr as usize + PMD_SIZE) as _;
        paddr += PMD_SIZE;
        size = if size <= PMD_SIZE { 0 } else { size - PMD_SIZE };
        if size == 0 { break; }
    }
    flush_tlb_local();
}

pub unsafe fn sme_unmap_bootdata(real_mode_data: *mut i8) {
    if !cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { return; }
    let boot_data = real_mode_data as *mut boot_params;
    let cmdline_paddr = (*boot_data).hdr.cmd_line_ptr as u64 |
        ((*boot_data).ext_cmd_line_ptr as u64) << 32;
    __sme_early_map_unmap_mem(real_mode_data as _, core::mem::size_of::<boot_params>(), false);
    if cmdline_paddr != 0 { __sme_early_map_unmap_mem(__va(cmdline_paddr as usize), COMMAND_LINE_SIZE, false); }
}

pub unsafe fn sme_map_bootdata(real_mode_data: *mut i8) {
    if !cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { return; }
    __sme_early_map_unmap_mem(real_mode_data as _, core::mem::size_of::<boot_params>(), true);
    let boot_data = real_mode_data as *mut boot_params;
    let cmdline_paddr = (*boot_data).hdr.cmd_line_ptr as u64 |
        ((*boot_data).ext_cmd_line_ptr as u64) << 32;
    if cmdline_paddr != 0 { __sme_early_map_unmap_mem(__va(cmdline_paddr as usize), COMMAND_LINE_SIZE, true); }
}

unsafe fn pg_level_to_pfn(level: i32, kpte: *mut pte_t, ret_prot: *mut pgprot_t) -> usize {
    let (pfn, prot) = match level {
        PG_LEVEL_4K => (pte_pfn(*kpte), pte_pgprot(*kpte)),
        PG_LEVEL_2M => (pmd_pfn(*(kpte as *mut pmd_t)), pmd_pgprot(*(kpte as *mut pmd_t))),
        PG_LEVEL_1G => (pud_pfn(*(kpte as *mut pud_t)), pud_pgprot(*(kpte as *mut pud_t))),
        _ => { WARN_ONCE(true, "Invalid level for kpte\n"); return 0; }
    };
    if !ret_prot.is_null() { *ret_prot = prot; }
    pfn
}

unsafe fn amd_enc_tlb_flush_required(_enc: bool) -> bool { true }
unsafe fn amd_enc_cache_flush_required() -> bool { !cpu_feature_enabled(X86_FEATURE_SME_COHERENT) }

unsafe fn enc_dec_hypercall(mut vaddr: usize, size: usize, enc: bool) {
    #[cfg(CONFIG_PARAVIRT)]
    {
        let end = vaddr + size;
        while vaddr < end {
            let mut level = 0; let kpte = lookup_address(vaddr, &mut level);
            if kpte.is_null() || pte_none(*kpte) { WARN_ONCE(true, "kpte lookup for vaddr\n"); return; }
            let pfn = pg_level_to_pfn(level, kpte, core::ptr::null_mut());
            if pfn == 0 { continue; }
            let psize = page_level_size(level); let pmask = page_level_mask(level);
            notify_page_enc_status_changed(pfn, psize >> PAGE_SHIFT, enc);
            vaddr = (vaddr & pmask) + psize;
        }
    }
}

unsafe fn amd_enc_status_change_prepare(vaddr: usize, npages: i32, enc: bool) -> i32 {
    if cc_platform_has(CC_ATTR_GUEST_SEV_SNP) && !enc { snp_set_memory_shared(vaddr, npages); }
    0
}

unsafe fn amd_enc_status_change_finish(vaddr: usize, npages: i32, enc: bool) -> i32 {
    if cc_platform_has(CC_ATTR_GUEST_SEV_SNP) && enc { snp_set_memory_private(vaddr, npages); }
    if !cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) { enc_dec_hypercall(vaddr, (npages << PAGE_SHIFT) as usize, enc); }
    0
}

pub unsafe fn prepare_pte_enc(d: *mut pte_enc_desc) -> i32 {
    let mut old_prot = core::mem::zeroed();
    (*d).pfn = pg_level_to_pfn((*d).pte_level, (*d).kpte, &mut old_prot);
    if (*d).pfn == 0 { return 1; }
    (*d).new_pgprot = old_prot;
    if (*d).encrypt { pgprot_val((*d).new_pgprot) |= _PAGE_ENC; }
    else { pgprot_val((*d).new_pgprot) &= !_PAGE_ENC; }
    if pgprot_val(old_prot) == pgprot_val((*d).new_pgprot) { return 1; }
    (*d).pa = (*d).pfn << PAGE_SHIFT; (*d).size = page_level_size((*d).pte_level);
    if (*d).va != 0 { clflush_cache_range((*d).va as _, (*d).size); }
    else { clflush_cache_range(__va((*d).pa), (*d).size); }
    0
}

pub unsafe fn set_pte_enc_mask(kpte: *mut pte_t, pfn: usize, new_prot: pgprot_t) {
    let new_pte = pfn_pte(pfn, new_prot); set_pte_atomic(kpte, new_pte);
}

unsafe fn __set_clr_pte_enc(kpte: *mut pte_t, level: i32, enc: bool) {
    let mut d: pte_enc_desc = core::mem::zeroed(); d.kpte = kpte; d.pte_level = level; d.encrypt = enc;
    if prepare_pte_enc(&mut d) != 0 { return; }
    if enc { sme_early_encrypt(d.pa, d.size); } else {
        sme_early_decrypt(d.pa, d.size); early_snp_set_memory_shared(__va(d.pa) as usize, d.pa, 1);
    }
    set_pte_enc_mask(kpte, d.pfn, d.new_pgprot);
    if enc { early_snp_set_memory_private(__va(d.pa) as usize, d.pa, 1); }
}

unsafe fn early_set_memory_enc_dec(mut vaddr: usize, size: usize, enc: bool) -> i32 {
    let start = vaddr; let end = vaddr + size; let mut next = vaddr;
    while vaddr < end {
        let mut level = 0; let kpte = lookup_address(vaddr, &mut level);
        if kpte.is_null() || pte_none(*kpte) { __flush_tlb_all(); return 1; }
        if level == PG_LEVEL_4K { __set_clr_pte_enc(kpte, level, enc); next = (vaddr & PAGE_MASK) + PAGE_SIZE; vaddr = next; continue; }
        let psize = page_level_size(level); let pmask = page_level_mask(level);
        if vaddr == (vaddr & pmask) && end - vaddr >= psize { __set_clr_pte_enc(kpte, level, enc); next = (vaddr & pmask) + psize; vaddr = next; continue; }
        let split = if level == PG_LEVEL_2M { 0 } else { 1 << PG_LEVEL_2M };
        kernel_physical_mapping_change(__pa(vaddr & pmask), __pa((end & pmask) + psize), split);
        vaddr = next;
    }
    early_set_mem_enc_dec_hypercall(start, size, enc); __flush_tlb_all(); 0
}

pub unsafe fn early_set_memory_decrypted(vaddr: usize, size: usize) -> i32 { early_set_memory_enc_dec(vaddr, size, false) }
pub unsafe fn early_set_memory_encrypted(vaddr: usize, size: usize) -> i32 { early_set_memory_enc_dec(vaddr, size, true) }
pub unsafe fn early_set_mem_enc_dec_hypercall(vaddr: usize, size: usize, enc: bool) { enc_dec_hypercall(vaddr, size, enc); }

pub unsafe fn sme_early_init() {
    if sme_me_mask == 0 { return; }
    early_pmd_flags = __sme_set(early_pmd_flags); __supported_pte_mask = __sme_set(__supported_pte_mask);
    add_encrypt_protection_map();
    x86_platform.guest.enc_status_change_prepare = Some(amd_enc_status_change_prepare);
    x86_platform.guest.enc_status_change_finish = Some(amd_enc_status_change_finish);
    x86_platform.guest.enc_tlb_flush_required = Some(amd_enc_tlb_flush_required);
    x86_platform.guest.enc_cache_flush_required = Some(amd_enc_cache_flush_required);
    x86_platform.guest.enc_kexec_begin = Some(snp_kexec_begin); x86_platform.guest.enc_kexec_finish = Some(snp_kexec_finish);
    if sev_status & MSR_AMD64_SEV_ES_ENABLED != 0 { x86_cpuinit.parallel_bringup = false; }
    if sev_status & MSR_AMD64_SEV_ENABLED != 0 { ia32_disable(); }
    if sev_status & MSR_AMD64_SEV_SNP_ENABLED != 0 {
        x86_init.mpparse.find_mptable = x86_init_noop; x86_init.pci.init_irq = x86_init_noop; x86_init.resources.probe_roms = x86_init_noop;
        x86_init.resources.dmi_setup = snp_dmi_setup;
    }
    if sev_status & MSR_AMD64_SNP_SECURE_TSC != 0 { setup_force_cpu_cap(X86_FEATURE_TSC_RELIABLE); }
}

pub unsafe fn mem_encrypt_free_decrypted_mem() {
    let vaddr = __start_bss_decrypted_unused as usize; let end = __end_bss_decrypted as usize;
    let npages = (end - vaddr) >> PAGE_SHIFT;
    if sme_me_mask != 0 && set_memory_encrypted(vaddr, npages) != 0 { pr_warn("failed to free unused decrypted pages\n"); return; }
    free_init_pages("unused decrypted", vaddr, end);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
