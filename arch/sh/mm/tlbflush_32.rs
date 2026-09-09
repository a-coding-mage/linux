/*
 * TLB flushing operations for SH with an MMU.
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2003  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, mut page: c_ulong) {
    let cpu: c_uint = smp_processor_id();

    if !(*vma).vm_mm.is_null() && cpu_context(cpu, (*vma).vm_mm) != NO_CONTEXT {
        let mut flags: c_ulong;
        let asid: c_ulong;
        let mut saved_asid: c_ulong = MMU_NO_ASID;

        asid = cpu_asid(cpu, (*vma).vm_mm);
        page &= PAGE_MASK;

        local_irq_save(&mut flags);
        if (*vma).vm_mm != (*current).mm {
            saved_asid = get_asid();
            set_asid(asid);
        }
        local_flush_tlb_one(asid, page);
        if saved_asid != MMU_NO_ASID {
            set_asid(saved_asid);
        }
        local_irq_restore(flags);
    }
}

pub unsafe fn local_flush_tlb_range(
    vma: *mut vm_area_struct,
    mut start: c_ulong,
    mut end: c_ulong,
) {
    let mm: *mut mm_struct = (*vma).vm_mm;
    let cpu: c_uint = smp_processor_id();

    if cpu_context(cpu, mm) != NO_CONTEXT {
        let mut flags: c_ulong;
        let size: c_int;

        local_irq_save(&mut flags);
        size = ((end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT) as c_int;
        if size > (MMU_NTLB_ENTRIES / 4) as c_int {
            /* Too many TLB to flush */
            *cpu_context_ptr(cpu, mm) = NO_CONTEXT;
            if mm == (*current).mm {
                activate_context(mm, cpu);
            }
        } else {
            let asid: c_ulong;
            let mut saved_asid: c_ulong = MMU_NO_ASID;

            asid = cpu_asid(cpu, mm);
            start &= PAGE_MASK;
            end += PAGE_SIZE - 1;
            end &= PAGE_MASK;
            if mm != (*current).mm {
                saved_asid = get_asid();
                set_asid(asid);
            }
            while start < end {
                local_flush_tlb_one(asid, start);
                start += PAGE_SIZE;
            }
            if saved_asid != MMU_NO_ASID {
                set_asid(saved_asid);
            }
        }
        local_irq_restore(flags);
    }
}

pub unsafe fn local_flush_tlb_kernel_range(mut start: c_ulong, mut end: c_ulong) {
    let cpu: c_uint = smp_processor_id();
    let mut flags: c_ulong;
    let size: c_int;

    local_irq_save(&mut flags);
    size = ((end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT) as c_int;
    if size > (MMU_NTLB_ENTRIES / 4) as c_int {
        /* Too many TLB to flush */
        local_flush_tlb_all();
    } else {
        let asid: c_ulong;
        let saved_asid: c_ulong = get_asid();

        asid = cpu_asid(cpu, &init_mm as *const mm_struct as *mut mm_struct);
        start &= PAGE_MASK;
        end += PAGE_SIZE - 1;
        end &= PAGE_MASK;
        set_asid(asid);
        while start < end {
            local_flush_tlb_one(asid, start);
            start += PAGE_SIZE;
        }
        set_asid(saved_asid);
    }
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    let cpu: c_uint = smp_processor_id();

    /* Invalidate all TLB of this process. */
    /* Instead of invalidating each TLB, we get new MMU context. */
    if cpu_context(cpu, mm) != NO_CONTEXT {
        let mut flags: c_ulong;

        local_irq_save(&mut flags);
        *cpu_context_ptr(cpu, mm) = NO_CONTEXT;
        if mm == (*current).mm {
            activate_context(mm, cpu);
        }
        local_irq_restore(flags);
    }
}

pub unsafe fn __flush_tlb_global() {
    let mut flags: c_ulong;

    local_irq_save(&mut flags);

    /*
     * This is the most destructive of the TLB flushing options,
     * and will tear down all of the UTLB/ITLB mappings, including
     * wired entries.
     */
    __raw_writel(__raw_readl(MMUCR) | MMUCR_TI, MMUCR);

    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
