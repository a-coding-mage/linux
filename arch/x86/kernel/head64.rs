// SPDX-License-Identifier: GPL-2.0
/*
 *  prepare to run common code
 *
 *  Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
 */

// cpu_feature_enabled() cannot be used this early.
// USE_EARLY_PGTABLE_L5

// Dependencies are supplied by the surrounding kernel translation unit.

extern "C" {
    static mut early_dynamic_pgts: [[pmd_t; PTRS_PER_PMD]; EARLY_DYNAMIC_PAGE_TABLES];
    static mut early_top_pgt: [pgd_t; PTRS_PER_PGD];
    static mut init_top_pgt: [pgd_t; PTRS_PER_PGD];
    static mut next_early_pgt: c_uint;
    static mut early_pmd_flags: pmdval_t;
    static mut __pgtable_l5_enabled: c_uint;
    static mut pgdir_shift: c_uint;
    static mut ptrs_per_p4d: c_uint;
    static mut page_offset_base: c_ulong;
    static mut vmalloc_base: c_ulong;
    static mut vmemmap_base: c_ulong;
}

// Wipe all early page tables except for the kernel symbol map
unsafe fn reset_early_page_tables() {
    core::ptr::write_bytes(early_top_pgt.as_mut_ptr(), 0, PTRS_PER_PGD - 1);
    next_early_pgt = 0;
    write_cr3(__sme_pa_nodebug(early_top_pgt.as_mut_ptr()));
}

// Create a new PMD entry
unsafe fn __early_make_pgtable(address: c_ulong, pmd: pmdval_t) -> bool {
    let physaddr = address.wrapping_sub(__PAGE_OFFSET);
    if physaddr >= MAXMEM || read_cr3_pa() != __pa_nodebug(early_top_pgt.as_mut_ptr()) {
        return false;
    }

    loop {
        let pgd_p = &mut early_top_pgt[pgd_index(address)].pgd;
        let pgd = *pgd_p;
        let p4d_p: *mut p4dval_t;

        /* The use of __START_KERNEL_map rather than __PAGE_OFFSET here is
         * critical -- __PAGE_OFFSET would point us back into the dynamic
         * range and we might end up looping forever... */
        if !pgtable_l5_enabled() {
            p4d_p = pgd_p as *mut _;
        } else if pgd != 0 {
            p4d_p = ((pgd & PTE_PFN_MASK).wrapping_add(__START_KERNEL_map).wrapping_sub(phys_base)) as *mut _;
        } else {
            if next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES as c_uint {
                reset_early_page_tables();
                continue;
            }
            p4d_p = early_dynamic_pgts[next_early_pgt as usize].as_mut_ptr() as *mut _;
            core::ptr::write_bytes(p4d_p, 0, PTRS_PER_P4D);
            *pgd_p = p4d_p as pgdval_t - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
            next_early_pgt += 1;
        }
        let p4d_p = p4d_p.add(p4d_index(address));
        let pud_p: *mut pudval_t;
        if *p4d_p != 0 {
            pud_p = (((*p4d_p) & PTE_PFN_MASK).wrapping_add(__START_KERNEL_map).wrapping_sub(phys_base)) as *mut _;
        } else {
            if next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES as c_uint { reset_early_page_tables(); continue; }
            pud_p = early_dynamic_pgts[next_early_pgt as usize].as_mut_ptr() as *mut _;
            core::ptr::write_bytes(pud_p, 0, PTRS_PER_PUD);
            *p4d_p = pud_p as p4dval_t - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
            next_early_pgt += 1;
        }
        let pud_p = pud_p.add(pud_index(address));
        let pmd_p: *mut pmdval_t;
        if *pud_p != 0 {
            pmd_p = (((*pud_p) & PTE_PFN_MASK).wrapping_add(__START_KERNEL_map).wrapping_sub(phys_base)) as *mut _;
        } else {
            if next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES as c_uint { reset_early_page_tables(); continue; }
            pmd_p = early_dynamic_pgts[next_early_pgt as usize].as_mut_ptr() as *mut _;
            core::ptr::write_bytes(pmd_p, 0, PTRS_PER_PMD);
            *pud_p = pmd_p as pudval_t - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
            next_early_pgt += 1;
        }
        *pmd_p.add(pmd_index(address)) = pmd;
        return true;
    }
}

unsafe fn early_make_pgtable(address: c_ulong) -> bool {
    let physaddr = address.wrapping_sub(__PAGE_OFFSET);
    __early_make_pgtable(address, (physaddr & PMD_MASK) + early_pmd_flags)
}

pub unsafe fn do_early_exception(regs: *mut pt_regs, trapnr: c_int) {
    if trapnr == X86_TRAP_PF && early_make_pgtable(native_read_cr2()) { return; }
    if IS_ENABLED_CONFIG_AMD_MEM_ENCRYPT && trapnr == X86_TRAP_VC && handle_vc_boot_ghcb(regs) { return; }
    if trapnr == X86_TRAP_VE && tdx_early_handle_ve(regs) { return; }
    early_fixup_exception(regs, trapnr);
}

// Don't add a printk in there. printk relies on the PDA which is not initialized yet.
pub unsafe fn clear_bss() {
    core::ptr::write_bytes(__bss_start as *mut u8, 0, __bss_stop as usize - __bss_start as usize);
    core::ptr::write_bytes(__brk_base as *mut u8, 0, __brk_limit as usize - __brk_base as usize);
}

unsafe fn get_cmd_line_ptr() -> c_ulong {
    boot_params.hdr.cmd_line_ptr as c_ulong | ((boot_params.ext_cmd_line_ptr as u64) << 32)
}

unsafe fn copy_bootdata(real_mode_data: *mut c_char) {
    sme_map_bootdata(real_mode_data);
    core::ptr::copy_nonoverlapping(real_mode_data as *const u8, &mut boot_params as *mut _ as *mut u8, core::mem::size_of_val(&boot_params));
    sanitize_boot_params(&mut boot_params);
    let p = get_cmd_line_ptr();
    if p != 0 { core::ptr::copy_nonoverlapping(__va(p) as *const u8, boot_command_line.as_mut_ptr() as *mut u8, COMMAND_LINE_SIZE); }
    sme_unmap_bootdata(real_mode_data);
}

pub unsafe fn x86_64_start_kernel(real_mode_data: *mut c_char) -> ! {
    // Build-time sanity checks are intentionally preserved as comments; they produce no code.
    cr4_init_shadow();
    reset_early_page_tables();
    if pgtable_l5_enabled() { page_offset_base = __PAGE_OFFSET_BASE_L5; vmalloc_base = __VMALLOC_BASE_L5; vmemmap_base = __VMEMMAP_BASE_L5; }
    clear_bss();
    clear_page(init_top_pgt.as_mut_ptr());
    sme_early_init();
    kasan_early_init();
    __native_tlb_flush_global(this_cpu_read(cpu_tlbstate.cr4));
    idt_setup_early_handler();
    tdx_early_init();
    copy_bootdata(__va(real_mode_data as c_ulong));
    load_ucode_bsp();
    init_top_pgt[511] = early_top_pgt[511];
    x86_64_start_reservations(real_mode_data)
}

pub unsafe fn x86_64_start_reservations(real_mode_data: *mut c_char) -> ! {
    if boot_params.hdr.version == 0 { copy_bootdata(__va(real_mode_data as c_ulong)); }
    x86_early_init_platform_quirks();
    match boot_params.hdr.hardware_subarch { X86_SUBARCH_INTEL_MID => x86_intel_mid_early_setup(), _ => {} }
    start_kernel()
}

pub unsafe fn early_setup_idt() {
    let mut handler: *mut c_void = core::ptr::null_mut();
    if IS_ENABLED_CONFIG_AMD_MEM_ENCRYPT { setup_ghcb(); handler = vc_boot_ghcb as *mut c_void; }
    __pi_startup_64_load_idt(handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
