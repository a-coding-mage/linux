// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel/UML translation.

#[repr(C)]
pub struct vm_ops {
    pub mm_idp: *mut mm_id,
    pub mmap: Option<unsafe extern "C" fn(
        *mut mm_id,
        c_ulong,
        c_ulong,
        c_int,
        c_int,
        c_ulonglong,
    ) -> c_int>,
    pub unmap: Option<unsafe extern "C" fn(*mut mm_id, c_ulong, c_ulong) -> c_int>,
}

unsafe extern "C" fn kern_map(
    _mm_idp: *mut mm_id,
    virt: c_ulong,
    len: c_ulong,
    prot: c_int,
    phys_fd: c_int,
    offset: c_ulonglong,
) -> c_int {
    os_map_memory(
        virt as *mut c_void,
        phys_fd,
        offset,
        len,
        prot & UM_PROT_READ,
        prot & UM_PROT_WRITE,
        prot & UM_PROT_EXEC,
    )
}

unsafe extern "C" fn kern_unmap(_mm_idp: *mut mm_id, virt: c_ulong, len: c_ulong) -> c_int {
    os_unmap_memory(virt as *mut c_void, len)
}

pub unsafe extern "C" fn report_enomem() {
    printk(
        KERN_ERR,
        "UML ran out of memory on the host side! This can happen due to a memory limitation or vm.max_map_count has been reached.\n",
    );
}

unsafe fn update_pte_range(
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    ops: *mut vm_ops,
) -> c_int {
    let mut pte = pte_offset_kernel(pmd, addr);
    let mut ret: c_int = 0;
    loop {
        if pte_needsync(*pte) {
            if pte_present(*pte) {
                let mut offset: c_ulonglong = 0;
                let phys = pte_val(*pte) & PAGE_MASK;
                let fd = phys_mapping(phys, &mut offset);
                let mut r = pte_read(*pte);
                let mut w = pte_write(*pte);
                let x = pte_exec(*pte);
                if !pte_young(*pte) {
                    r = 0;
                    w = 0;
                } else if !pte_dirty(*pte) {
                    w = 0;
                }
                let prot = (if r != 0 { UM_PROT_READ } else { 0 })
                    | (if w != 0 { UM_PROT_WRITE } else { 0 })
                    | (if x != 0 { UM_PROT_EXEC } else { 0 });
                ret = ((*ops).mmap.unwrap())(
                    (*ops).mm_idp,
                    addr,
                    PAGE_SIZE,
                    prot,
                    fd,
                    offset,
                );
            } else {
                ret = ((*ops).unmap.unwrap())((*ops).mm_idp, addr, PAGE_SIZE);
            }
            *pte = pte_mkuptodate(*pte);
        }
        pte = pte.add(1);
        addr = addr.wrapping_add(PAGE_SIZE);
        if !(addr < end && ret == 0) {
            break;
        }
    }
    ret
}

unsafe fn update_pmd_range(
    pud: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    ops: *mut vm_ops,
) -> c_int {
    let mut pmd = pmd_offset(pud, addr);
    let mut ret: c_int = 0;
    loop {
        let next = pmd_addr_end(addr, end);
        if !pmd_present(*pmd) {
            if pmd_needsync(*pmd) {
                ret = ((*ops).unmap.unwrap())((*ops).mm_idp, addr, next - addr);
                pmd_mkuptodate(*pmd);
            }
        } else {
            ret = update_pte_range(pmd, addr, next, ops);
        }
        pmd = pmd.add(1);
        addr = next;
        if !(addr < end && ret == 0) {
            break;
        }
    }
    ret
}

unsafe fn update_pud_range(
    p4d: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
    ops: *mut vm_ops,
) -> c_int {
    let mut pud = pud_offset(p4d, addr);
    let mut ret: c_int = 0;
    loop {
        let next = pud_addr_end(addr, end);
        if !pud_present(*pud) {
            if pud_needsync(*pud) {
                ret = ((*ops).unmap.unwrap())((*ops).mm_idp, addr, next - addr);
                pud_mkuptodate(*pud);
            }
        } else {
            ret = update_pmd_range(pud, addr, next, ops);
        }
        pud = pud.add(1);
        addr = next;
        if !(addr < end && ret == 0) {
            break;
        }
    }
    ret
}

unsafe fn update_p4d_range(
    pgd: *mut pgd_t,
    mut addr: c_ulong,
    end: c_ulong,
    ops: *mut vm_ops,
) -> c_int {
    let mut p4d = p4d_offset(pgd, addr);
    let mut ret: c_int = 0;
    loop {
        let next = p4d_addr_end(addr, end);
        if !p4d_present(*p4d) {
            if p4d_needsync(*p4d) {
                ret = ((*ops).unmap.unwrap())((*ops).mm_idp, addr, next - addr);
                p4d_mkuptodate(*p4d);
            }
        } else {
            ret = update_pud_range(p4d, addr, next, ops);
        }
        p4d = p4d.add(1);
        addr = next;
        if !(addr < end && ret == 0) {
            break;
        }
    }
    ret
}

pub unsafe extern "C" fn um_tlb_sync(mm: *mut mm_struct) -> c_int {
    // C guard(spinlock_irqsave) calls acquire the page-table and sync-TLB locks here.
    let mut ops = vm_ops {
        mm_idp: &mut (*mm).context.id,
        mmap: None,
        unmap: None,
    };
    if (*mm).context.sync_tlb_range_to == 0 {
        return 0;
    }
    if mm == &mut init_mm {
        ops.mmap = Some(kern_map);
        ops.unmap = Some(kern_unmap);
    } else {
        ops.mmap = Some(map);
        ops.unmap = Some(unmap);
    }
    let mut addr = (*mm).context.sync_tlb_range_from;
    let mut pgd = pgd_offset(mm, addr);
    let mut ret: c_int = 0;
    loop {
        let next = pgd_addr_end(addr, (*mm).context.sync_tlb_range_to);
        if !pgd_present(*pgd) {
            if pgd_needsync(*pgd) {
                ret = (ops.unmap.unwrap())(ops.mm_idp, addr, next - addr);
                pgd_mkuptodate(*pgd);
            }
        } else {
            ret = update_p4d_range(pgd, addr, next, &mut ops);
        }
        pgd = pgd.add(1);
        addr = next;
        if !(addr < (*mm).context.sync_tlb_range_to && ret == 0) {
            break;
        }
    }
    if ret == -ENOMEM {
        report_enomem();
    }
    (*mm).context.sync_tlb_range_from = 0;
    (*mm).context.sync_tlb_range_to = 0;
    ret
}

pub unsafe extern "C" fn flush_tlb_all() {
    /* Don't bother flushing if this address space is about to be destroyed. */
    if atomic_read(&mut (*(*current).mm).mm_users) == 0 {
        return;
    }
    flush_tlb_mm((*current).mm);
}

pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    let mut vma: *mut vm_area_struct;
    let mut vmi = VmaIterator::new(mm, 0);
    while let Some(next_vma) = vmi.next() {
        vma = next_vma;
        um_tlb_mark_sync(mm, (*vma).vm_start, (*vma).vm_end);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
