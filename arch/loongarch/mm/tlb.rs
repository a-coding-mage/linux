// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub unsafe fn local_flush_tlb_all() {
    invtlb_all(INVTLB_CURRENT_ALL, 0, 0);
}

pub unsafe fn local_flush_tlb_user() {
    invtlb_all(INVTLB_CURRENT_GFALSE, 0, 0);
}

pub unsafe fn local_flush_tlb_kernel() {
    invtlb_all(INVTLB_CURRENT_GTRUE, 0, 0);
}

/* All entries common to a mm share an asid. To effectively flush these entries, we just bump the asid. */
pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    let cpu: i32;
    preempt_disable();
    cpu = smp_processor_id();
    if asid_valid(mm, cpu) {
        drop_mmu_context(mm, cpu);
    } else {
        cpumask_clear_cpu(cpu, mm_cpumask(mm));
    }
    preempt_enable();
}

pub unsafe fn local_flush_tlb_range(vma: *mut vm_area_struct, mut start: usize, mut end: usize) {
    let mm = (*vma).vm_mm;
    let cpu = smp_processor_id();
    if asid_valid(mm, cpu) {
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        start = round_down(start, PAGE_SIZE << 1);
        end = round_up(end, PAGE_SIZE << 1);
        let size = (end - start) >> (PAGE_SHIFT + 1);
        if size <= if current_cpu_data.tlbsizestlbsets != 0 {
            current_cpu_data.tlbsize / 8
        } else {
            current_cpu_data.tlbsize / 2
        } {
            let asid = cpu_asid(cpu, mm);
            while start < end {
                invtlb(INVTLB_ADDR_GFALSE_AND_ASID, asid, start);
                start += PAGE_SIZE << 1;
            }
        } else {
            drop_mmu_context(mm, cpu);
        }
        local_irq_restore(flags);
    } else {
        cpumask_clear_cpu(cpu, mm_cpumask(mm));
    }
}

pub unsafe fn local_flush_tlb_kernel_range(mut start: usize, mut end: usize) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let mut size = (end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT;
    size = (size + 1) >> 1;
    if size <= if current_cpu_data.tlbsizestlbsets != 0 { current_cpu_data.tlbsize / 8 } else { current_cpu_data.tlbsize / 2 } {
        start &= PAGE_MASK << 1;
        end += (PAGE_SIZE << 1) - 1;
        end &= PAGE_MASK << 1;
        while start < end {
            invtlb_addr(INVTLB_ADDR_GTRUE_OR_ASID, 0, start);
            start += PAGE_SIZE << 1;
        }
    } else {
        local_flush_tlb_kernel();
    }
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, mut page: usize) {
    let cpu = smp_processor_id();
    if asid_valid((*vma).vm_mm, cpu) {
        let newpid = cpu_asid(cpu, (*vma).vm_mm);
        page &= PAGE_MASK << 1;
        invtlb(INVTLB_ADDR_GFALSE_AND_ASID, newpid, page);
    } else {
        cpumask_clear_cpu(cpu, mm_cpumask((*vma).vm_mm));
    }
}

/* This is only used for pages with the global bit set, so we don't care much about the ASID. */
pub unsafe fn local_flush_tlb_one(mut page: usize) {
    page &= PAGE_MASK << 1;
    invtlb_addr(INVTLB_ADDR_GTRUE_OR_ASID, 0, page);
}

unsafe fn __update_hugetlb(vma: *mut vm_area_struct, mut address: usize, ptep: *mut pte_t) {
    // CONFIG_HUGETLB_PAGE conditional from the C source.
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        let idx: i32;
        let lo: usize;
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        address &= PAGE_MASK << 1;
        write_csr_entryhi(address);
        tlb_probe();
        idx = read_csr_tlbidx();
        write_csr_pagesize(PS_HUGE_SIZE);
        lo = pmd_to_entrylo(pte_val(*ptep));
        write_csr_entrylo0(lo);
        write_csr_entrylo1(lo + (HPAGE_SIZE >> 1));
        if idx < 0 { tlb_write_random(); } else { tlb_write_indexed(); }
        write_csr_pagesize(PS_DEFAULT_SIZE);
        local_irq_restore(flags);
    }
}

pub unsafe fn __update_tlb(vma: *mut vm_area_struct, mut address: usize, mut ptep: *mut pte_t) {
    if cpu_has_ptw { return; }
    if current.active_mm != (*vma).vm_mm { return; }
    if pte_val(*ptep) & _PAGE_HUGE != 0 { __update_hugetlb(vma, address, ptep); return; }
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    if ptep as usize & std::mem::size_of::<pte_t>() != 0 { ptep = ptep.offset(-1); }
    address &= PAGE_MASK << 1;
    write_csr_entryhi(address);
    tlb_probe();
    let idx = read_csr_tlbidx();
    write_csr_pagesize(PS_DEFAULT_SIZE);
    write_csr_entrylo0(pte_val(*ptep));
    ptep = ptep.offset(1);
    write_csr_entrylo1(pte_val(*ptep));
    if idx < 0 { tlb_write_random(); } else { tlb_write_indexed(); }
    local_irq_restore(flags);
}

unsafe fn setup_ptwalker() {
    let mut pwctl0: usize;
    let mut pwctl1: usize;
    let pgd_i = PGDIR_SHIFT; let pgd_w = PAGE_SHIFT - 3;
    let mut pud_i = 0; let mut pud_w = 0; let mut pmd_i = 0; let mut pmd_w = 0;
    #[cfg(any())] { pud_i = PUD_SHIFT; pud_w = PAGE_SHIFT - 3; }
    #[cfg(any())] { pmd_i = PMD_SHIFT; pmd_w = PAGE_SHIFT - 3; }
    let pte_i = PAGE_SHIFT; let pte_w = PAGE_SHIFT - 3;
    pwctl0 = pte_i | pwctl_w(pte_w, 5) | pmd_i << 10 | pmd_w << 15 | pud_i << 20 | pud_w << 25;
    pwctl1 = pgd_i | pgd_w << 6;
    if cpu_has_ptw { pwctl1 |= CSR_PWCTL1_PTW; }
    csr_write(pwctl0, LOONGARCH_CSR_PWCTL0); csr_write(pwctl1, LOONGARCH_CSR_PWCTL1);
    csr_write(swapper_pg_dir as isize, LOONGARCH_CSR_PGDH); csr_write(invalid_pg_dir as isize, LOONGARCH_CSR_PGDL);
    csr_write(smp_processor_id() as isize, LOONGARCH_CSR_TMID);
}

unsafe fn output_pgtable_bits_defines() {
    pr_debug("#include <asm/asm.h>\n# include <asm/regdef.h>\n\n");
    pr_debug!("#define _PAGE_VALID_SHIFT %d\n", _PAGE_VALID_SHIFT);
    pr_debug!("#define _PAGE_DIRTY_SHIFT %d\n", _PAGE_DIRTY_SHIFT);
    pr_debug!("#define _PAGE_HUGE_SHIFT %d\n", _PAGE_HUGE_SHIFT);
    pr_debug!("#define _PAGE_GLOBAL_SHIFT %d\n", _PAGE_GLOBAL_SHIFT);
    pr_debug!("#define _PAGE_PRESENT_SHIFT %d\n", _PAGE_PRESENT_SHIFT);
    pr_debug!("#define _PAGE_WRITE_SHIFT %d\n", _PAGE_WRITE_SHIFT);
    pr_debug!("#define PFN_PTE_SHIFT %d\n", PFN_PTE_SHIFT);
    pr_debug!("\n");
}

pub unsafe fn setup_tlb_handler(cpu: i32) {
    setup_ptwalker(); local_flush_tlb_all();
    if cpu_has_ptw { exception_table[EXCCODE_TLBI] = handle_tlb_load_ptw; exception_table[EXCCODE_TLBL] = handle_tlb_load_ptw; exception_table[EXCCODE_TLBS] = handle_tlb_store_ptw; exception_table[EXCCODE_TLBM] = handle_tlb_modify_ptw; }
    if cpu == 0 {
        memcpy(tlbrentry as *mut _, handle_tlb_refill as *const _, 0x80);
        local_flush_icache_range(tlbrentry, tlbrentry + 0x80);
        for i in EXCCODE_TLBL..=EXCCODE_TLBPE { set_handler(i * VECSIZE, exception_table[i], VECSIZE); }
    } else {
        rcutree_report_cpu_starting(cpu);
        // CONFIG_NUMA && !CONFIG_PREEMPT_RT block preserved from the C source.
    }
}

pub unsafe fn tlb_init(cpu: i32) {
    write_csr_pagesize(PS_DEFAULT_SIZE); write_csr_stlbpgsize(PS_DEFAULT_SIZE); write_csr_tlbrefill_pagesize(PS_DEFAULT_SIZE);
    setup_tlb_handler(cpu); output_pgtable_bits_defines();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
