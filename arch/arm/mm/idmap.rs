// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation unit.

/*
 * Note: accesses outside of the kernel image and the identity map area
 * are not supported on any CPU using the idmap tables as its current
 * page tables.
 */
pub static mut idmap_pgd: *mut pgd_t = core::ptr::null_mut();
pub static mut arch_phys_to_idmap_offset: i64 = 0;

#[cfg(CONFIG_ARM_LPAE)]
unsafe fn idmap_add_pmd(pud: *mut pud_t, mut addr: c_ulong, end: c_ulong,
                        prot: c_ulong) {
    let mut pmd: *mut pmd_t;
    let mut next: c_ulong;

    if pud_none_or_clear_bad(pud) || (pud_val(*pud) & L_PGD_SWAPPER) != 0 {
        pmd = pmd_alloc_one(&mut init_mm, addr);
        if pmd.is_null() {
            pr_warn("Failed to allocate identity pmd.\n");
            return;
        }
        /*
         * Copy the original PMD to ensure that the PMD entries for
         * the kernel image are preserved.
         */
        if !pud_none(*pud) {
            memcpy(pmd as *mut c_void, pmd_offset(pud, 0) as *const c_void,
                   PTRS_PER_PMD * core::mem::size_of::<pmd_t>());
        }
        pud_populate(&mut init_mm, pud, pmd);
        pmd = pmd.add(pmd_index(addr) as usize);
    } else {
        pmd = pmd_offset(pud, addr);
    }

    loop {
        next = pmd_addr_end(addr, end);
        *pmd = __pmd((addr & PMD_MASK) | prot);
        flush_pmd_entry(pmd);
        pmd = pmd.add(1);
        addr = next;
        if addr == end { break; }
    }
}

#[cfg(not(CONFIG_ARM_LPAE))]
unsafe fn idmap_add_pmd(pud: *mut pud_t, mut addr: c_ulong, _end: c_ulong,
                        prot: c_ulong) {
    let pmd = pmd_offset(pud, addr);

    addr = (addr & PMD_MASK) | prot;
    *pmd = __pmd(addr);
    *pmd.add(1) = __pmd(addr + SECTION_SIZE);
    flush_pmd_entry(pmd);
}

unsafe fn idmap_add_pud(pgd: *mut pgd_t, mut addr: c_ulong, end: c_ulong,
                        prot: c_ulong) {
    let p4d = p4d_offset(pgd, addr);
    let mut pud = pud_offset(p4d, addr);
    let mut next: c_ulong;

    loop {
        next = pud_addr_end(addr, end);
        idmap_add_pmd(pud, addr, next, prot);
        pud = pud.add(1);
        addr = next;
        if addr == end { break; }
    }
}

unsafe fn identity_mapping_add(pgd: *mut pgd_t, text_start: *const c_char,
                               text_end: *const c_char, mut prot: c_ulong) {
    let (mut addr, end): (c_ulong, c_ulong);
    let mut next: c_ulong;

    #[cfg(CONFIG_XIP_KERNEL)]
    {
        addr = (text_start as phys_addr_t) - XIP_VIRT_ADDR(CONFIG_XIP_PHYS_ADDR)
            + CONFIG_XIP_PHYS_ADDR;
        end = (text_end as phys_addr_t) - XIP_VIRT_ADDR(CONFIG_XIP_PHYS_ADDR)
            + CONFIG_XIP_PHYS_ADDR;
    }
    #[cfg(not(CONFIG_XIP_KERNEL))]
    {
        addr = virt_to_idmap(text_start);
        end = virt_to_idmap(text_end);
    }
    pr_info("Setting up static identity map for 0x%lx - 0x%lx\n", addr, end);

    prot |= PMD_TYPE_SECT | PMD_SECT_AP_WRITE | PMD_SECT_AF;

    if cpu_architecture() <= CPU_ARCH_ARMv5TEJ && !cpu_is_xscale_family() {
        prot |= PMD_BIT4;
    }

    let mut pgd = pgd.add(pgd_index(addr) as usize);
    loop {
        next = pgd_addr_end(addr, end);
        idmap_add_pud(pgd, addr, next, prot);
        pgd = pgd.add(1);
        addr = next;
        if addr == end { break; }
    }
}

pub static mut __idmap_text_start: [c_char; 0] = [];
pub static mut __idmap_text_end: [c_char; 0] = [];

unsafe fn init_static_idmap() -> c_int {
    idmap_pgd = pgd_alloc(&mut init_mm);
    if idmap_pgd.is_null() { return -ENOMEM; }

    identity_mapping_add(idmap_pgd, __idmap_text_start.as_ptr(),
                         __idmap_text_end.as_ptr(), 0);

    /* Flush L1 for the hardware to see this page table content */
    if (elf_hwcap & HWCAP_LPAE) == 0 {
        flush_cache_louis();
    }

    0
}

// early_initcall(init_static_idmap);

/*
 * In order to soft-boot, we need to switch to a 1:1 mapping for the
 * cpu_reset functions. This will then ensure that we have predictable
 * results when turning off the mmu.
 */
pub unsafe fn setup_mm_for_reboot() {
    /*
     * With CONFIG_CPU_TTBR0_PAN enabled, TTBCR.EPD0 is set whenever
     * user-space access is disabled in order to block TTBR0 page-table
     * walks.  The identity mapping lives at low (user-space) virtual
     * addresses and can only be reached via TTBR0, so we must re-enable
     * those walks before switching page tables.  On non-PAN kernels this
     * is a no-op.
     */
    if IS_ENABLED(CONFIG_CPU_TTBR0_PAN) {
        uaccess_save_and_enable();
    }

    /* Switch to the identity mapping. */
    cpu_switch_mm(idmap_pgd, &mut init_mm);
    local_flush_bp_all();

    #[cfg(CONFIG_CPU_HAS_ASID)]
    {
        /*
         * We don't have a clean ASID for the identity mapping, which
         * may clash with virtual addresses of the previous page tables
         * and therefore potentially in the TLB.
         */
        local_flush_tlb_all();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
