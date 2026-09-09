// SPDX-License-Identifier: GPL-2.0
// Translated from init.c; declarations supplied by kernel dependencies are external.

extern "C" {
    static mut real_mode_header: *mut real_mode_header;
    static mut trampoline_cr4_features: *mut u32;
    static mut trampoline_pgd_entry: pgd_t;
}

pub unsafe extern "C" fn load_trampoline_pgtable() {
    #[cfg(CONFIG_X86_32)]
    {
        load_cr3(initial_page_table);
    }
    #[cfg(not(CONFIG_X86_32))]
    {
        // This function is called before exiting to real-mode and that will
        // fail with CR4.PCIDE still set.
        if boot_cpu_has(X86_FEATURE_PCID) {
            cr4_clear_bits(X86_CR4_PCIDE);
        }

        write_cr3((*real_mode_header).trampoline_pgd);
    }

    // The CR3 write above will not flush global TLB entries.
    // Stale, global entries from previous page tables may still be present.
    // Flush those stale entries.
    //
    // This ensures that memory accessed while running with trampoline_pgd is
    // actually mapped into trampoline_pgd.
    __flush_tlb_all();
}

pub unsafe extern "C" fn reserve_real_mode() {
    let limit: phys_addr_t = x86_init.resources.realmode_limit;
    let size: usize = real_mode_size_needed();

    if size == 0 {
        return;
    }

    WARN_ON(slab_is_available());

    let mem: phys_addr_t = memblock_phys_alloc_range(size, PAGE_SIZE, 0, limit);
    if mem == 0 {
        pr_info("No memory below %pa for the real-mode trampoline\\n", &limit);
    } else {
        set_real_mode_mem(mem);
    }

    // Unconditionally reserve the entire first 1M, see comment in setup_arch().
    memblock_reserve(0, SZ_1M);
    memblock_clear_kho_scratch(0, SZ_1M);
}

unsafe fn sme_sev_setup_real_mode(th: *mut trampoline_header) {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    {
        if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
            (*th).flags |= TH_FLAGS_SME_ACTIVE;
        }

        if cc_platform_has(CC_ATTR_GUEST_STATE_ENCRYPT) {
            // Skip the call to verify_cpu() in secondary_startup_64 as it will
            // cause #VC exceptions when the AP can't handle them yet.
            (*th).start = secondary_startup_64_no_verify as u64;

            if sev_es_setup_ap_jump_table(real_mode_header) != 0 {
                panic("Failed to get/update SEV-ES AP Jump Table");
            }
        }
    }
}

unsafe fn setup_real_mode() {
    let mut real_mode_seg: u16;
    let mut rel: *const u32;
    let mut count: u32;
    let base = real_mode_header as *mut u8;
    let phys_base: usize;
    let trampoline_header: *mut trampoline_header;
    let size: usize = PAGE_ALIGN(real_mode_blob_end as usize - real_mode_blob as usize);

    #[cfg(not(CONFIG_X86_32))]
    let mut efer: u64;
    #[cfg(not(CONFIG_X86_32))]
    let mut i: i32;

    // If SME is active, the trampoline area will need to be in decrypted memory
    // in order to bring up other processors successfully. This is not needed for SEV.
    if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
        set_memory_decrypted(base as usize, size >> PAGE_SHIFT);
    }

    memcpy(base as *mut _, real_mode_blob as *const _, size);
    phys_base = __pa(base as usize);
    real_mode_seg = (phys_base >> 4) as u16;
    rel = real_mode_relocs;

    // 16-bit segment relocations.
    count = *rel;
    rel = rel.add(1);
    while count != 0 {
        let seg = (base.add(*rel as usize)) as *mut u16;
        *seg = real_mode_seg;
        rel = rel.add(1);
        count -= 1;
    }

    // 32-bit linear relocations.
    count = *rel;
    rel = rel.add(1);
    while count != 0 {
        let ptr = (base.add(*rel as usize)) as *mut u32;
        *ptr = (*ptr).wrapping_add(phys_base as u32);
        rel = rel.add(1);
        count -= 1;
    }

    // Must be performed *after* relocation.
    trampoline_header = __va((*real_mode_header).trampoline_header) as *mut trampoline_header;

    #[cfg(CONFIG_X86_32)]
    {
        (*trampoline_header).start = __pa_symbol(startup_32_smp);
        (*trampoline_header).gdt_limit = __BOOT_DS + 7;
        (*trampoline_header).gdt_base = __pa_symbol(boot_gdt);
    }
    #[cfg(not(CONFIG_X86_32))]
    {
        // Some AMD processors will #GP(0) if EFER.LMA is set in WRMSR.
        rdmsrq(MSR_EFER, &mut efer);
        (*trampoline_header).efer = efer & !EFER_LMA;
        (*trampoline_header).start = secondary_startup_64 as u64;
        trampoline_cr4_features = &mut (*trampoline_header).cr4;
        *trampoline_cr4_features = mmu_cr4_features;
        (*trampoline_header).flags = 0;
        trampoline_lock = &mut (*trampoline_header).lock;
        *trampoline_lock = 0;

        let trampoline_pgd = __va((*real_mode_header).trampoline_pgd) as *mut u64;
        // Map the real mode stub as virtual == physical.
        *trampoline_pgd = trampoline_pgd_entry.pgd;

        // Include the entirety of the kernel mapping into the trampoline PGD.
        i = pgd_index(__PAGE_OFFSET);
        while i < PTRS_PER_PGD {
            *trampoline_pgd.add(i as usize) = init_top_pgt[i as usize].pgd;
            i += 1;
        }
    }

    sme_sev_setup_real_mode(trampoline_header);
}

unsafe fn set_real_mode_permissions() {
    let base = real_mode_header as *mut u8;
    let size = PAGE_ALIGN(real_mode_blob_end as usize - real_mode_blob as usize);
    let ro_size = PAGE_ALIGN((*real_mode_header).ro_end as usize) - __pa(base as usize);
    let text_size = PAGE_ALIGN((*real_mode_header).ro_end as usize) - (*real_mode_header).text_start as usize;
    let text_start = __va((*real_mode_header).text_start) as usize;
    set_memory_nx(base as usize, size >> PAGE_SHIFT);
    set_memory_ro(base as usize, ro_size >> PAGE_SHIFT);
    set_memory_x(text_start, text_size >> PAGE_SHIFT);
}

pub unsafe extern "C" fn init_real_mode() {
    if real_mode_header.is_null() {
        panic("Real mode trampoline was not allocated");
    }
    setup_real_mode();
    set_real_mode_permissions();
}

unsafe fn do_init_real_mode() -> i32 {
    x86_platform.realmode_init();
    0
}

early_initcall!(do_init_real_mode);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
