// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

/* Dependencies supplied by the corresponding Linux/C-SKY headers are external. */

/*
 * One C-SKY MMU TLB entry contain two PFN/page entry, ie:
 * 1VPN -> 2PFN
 */
const TLB_ENTRY_SIZE: usize = PAGE_SIZE * 2;
const TLB_ENTRY_SIZE_MASK: usize = PAGE_MASK << 1;

pub unsafe fn flush_tlb_all() {
    tlb_invalid_all();
}

pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        core::arch::asm!(
            "tlbi.asids {0}",
            "sync.i",
            in(reg) cpu_asid(mm),
            options(nostack)
        );
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        tlb_invalid_all();
    }
}

/* MMU operation regs only could invalid tlb entry in jtlb and
 * we need change asid field to invalid I-utlb & D-utlb. */
#[cfg(not(CONFIG_CPU_HAS_TLBI))]
unsafe fn restore_asid_inv_utlb(oldpid: u32, newpid: u32) {
    if oldpid == newpid {
        write_mmu_entryhi(oldpid.wrapping_add(1));
    }
    write_mmu_entryhi(oldpid);
}

pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    mut start: usize,
    mut end: usize,
) {
    let newpid = cpu_asid((*vma).vm_mm);

    start &= TLB_ENTRY_SIZE_MASK;
    end = (end.wrapping_add(TLB_ENTRY_SIZE - 1)) & TLB_ENTRY_SIZE_MASK;

    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        while start < end {
            core::arch::asm!(
                "tlbi.vas {0}",
                in(reg) (start | newpid),
                options(nostack)
            );
            start = start.wrapping_add(2 * PAGE_SIZE);
        }
        core::arch::asm!("sync.i", options(nostack));
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        let oldpid = read_mmu_entryhi() & ASID_MASK;
        while start < end {
            write_mmu_entryhi(start | newpid);
            start = start.wrapping_add(2 * PAGE_SIZE);
            tlb_probe();
            let idx = read_mmu_index();
            if idx >= 0 {
                tlb_invalid_indexed();
            }
        }
        restore_asid_inv_utlb(oldpid, newpid);
        local_irq_restore(flags);
    }
}

pub unsafe fn flush_tlb_kernel_range(mut start: usize, mut end: usize) {
    start &= TLB_ENTRY_SIZE_MASK;
    end = (end.wrapping_add(TLB_ENTRY_SIZE - 1)) & TLB_ENTRY_SIZE_MASK;

    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        while start < end {
            core::arch::asm!("tlbi.vaas {0}", in(reg) start, options(nostack));
            start = start.wrapping_add(2 * PAGE_SIZE);
        }
        core::arch::asm!("sync.i", options(nostack));
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        let oldpid = read_mmu_entryhi() & ASID_MASK;
        while start < end {
            write_mmu_entryhi(start | oldpid);
            start = start.wrapping_add(2 * PAGE_SIZE);
            tlb_probe();
            let idx = read_mmu_index();
            if idx >= 0 {
                tlb_invalid_indexed();
            }
        }
        restore_asid_inv_utlb(oldpid, oldpid);
        local_irq_restore(flags);
    }
}

pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, mut addr: usize) {
    let newpid = cpu_asid((*vma).vm_mm);
    addr &= TLB_ENTRY_SIZE_MASK;

    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        core::arch::asm!(
            "tlbi.vas {0}",
            "sync.i",
            in(reg) (addr | newpid),
            options(nostack)
        );
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        let oldpid = read_mmu_entryhi() & ASID_MASK;
        write_mmu_entryhi(addr | newpid);
        tlb_probe();
        let idx = read_mmu_index();
        if idx >= 0 {
            tlb_invalid_indexed();
        }
        restore_asid_inv_utlb(oldpid, newpid);
        local_irq_restore(flags);
    }
}

pub unsafe fn flush_tlb_one(mut addr: usize) {
    addr &= TLB_ENTRY_SIZE_MASK;

    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        core::arch::asm!(
            "tlbi.vaas {0}",
            "sync.i",
            in(reg) addr,
            options(nostack)
        );
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        let mut flags: usize = 0;
        local_irq_save(&mut flags);
        let oldpid = read_mmu_entryhi() & ASID_MASK;
        write_mmu_entryhi(addr | oldpid);
        tlb_probe();
        let idx = read_mmu_index();
        if idx >= 0 {
            tlb_invalid_indexed();
        }
        restore_asid_inv_utlb(oldpid, oldpid);
        local_irq_restore(flags);
    }
}

// EXPORT_SYMBOL(flush_tlb_one);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
