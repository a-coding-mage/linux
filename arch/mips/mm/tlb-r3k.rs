// SPDX-License-Identifier: GPL-2.0
/*
 * r2300.c: R2000 and R3000 specific mmu/cache code.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 *
 * with a lot of changes to make this thing work for R3000s
 * Tx39XX R4k style caches added. HK
 * Copyright (C) 1998, 1999, 2000 Harald Koerfgen
 * Copyright (C) 1998 Gleb Raiko & Vladimir Roganov
 * Copyright (C) 2002  Ralf Baechle
 * Copyright (C) 2002  Maciej W. Rozycki
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[repr(C)]
pub struct VmAreaStruct {
    pub vm_mm: *mut MmStruct,
}
#[repr(C)]
pub struct MmStruct;
#[repr(C)]
pub struct TaskStruct {
    pub active_mm: *mut MmStruct,
}
#[repr(C)]
pub struct CpuData {
    pub tlbsize: i32,
}
#[repr(transparent)]
pub struct PteT(pub usize);

extern "C" {
    static mut current: *mut TaskStruct;
    static mut current_cpu_data: CpuData;
    fn read_c0_entryhi() -> usize;
    fn write_c0_entryhi(value: usize);
    fn write_c0_entrylo0(value: usize);
    fn write_c0_index(value: i32);
    fn read_c0_index() -> i32;
    fn tlb_write_indexed();
    fn tlb_probe();
    fn tlb_write_random();
    fn build_tlb_refill_handler();
    fn cpu_asid_mask(cpu: *const CpuData) -> usize;
    fn smp_processor_id() -> i32;
    fn cpu_context(cpu: i32, mm: *mut MmStruct) -> usize;
    fn drop_mmu_context(mm: *mut MmStruct);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn pte_val(pte: PteT) -> usize;
}

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const KSEG0: usize = 0x8000_0000;

unsafe fn local_flush_tlb_from(mut entry: i32) {
    let old_ctx = read_c0_entryhi() & cpu_asid_mask(&current_cpu_data);
    write_c0_entrylo0(0);
    while entry < current_cpu_data.tlbsize {
        write_c0_index(entry << 8);
        write_c0_entryhi(((entry as usize) | 0x80000) << 12);
        entry += 1; // BARRIER
        tlb_write_indexed();
    }
    write_c0_entryhi(old_ctx);
}

pub unsafe fn local_flush_tlb_all() {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    local_flush_tlb_from(8);
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_range(vma: *mut VmAreaStruct, mut start: usize, mut end: usize) {
    let asid_mask = cpu_asid_mask(&current_cpu_data);
    let mm = (*vma).vm_mm;
    let cpu = smp_processor_id();

    if cpu_context(cpu, mm) != 0 {
        let mut flags = 0usize;
        local_irq_save(&mut flags);
        let size = (end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT;
        if size <= current_cpu_data.tlbsize as usize {
            let oldpid = read_c0_entryhi() & asid_mask;
            let newpid = cpu_context(cpu, mm) & asid_mask;
            start &= PAGE_MASK;
            end = (end + PAGE_SIZE - 1) & PAGE_MASK;
            while start < end {
                write_c0_entryhi(start | newpid);
                start += PAGE_SIZE; // BARRIER
                tlb_probe();
                let idx = read_c0_index();
                write_c0_entrylo0(0);
                write_c0_entryhi(KSEG0);
                if idx < 0 {
                    continue;
                }
                tlb_write_indexed();
            }
            write_c0_entryhi(oldpid);
        } else {
            drop_mmu_context(mm);
        }
        local_irq_restore(flags);
    }
}

pub unsafe fn local_flush_tlb_kernel_range(mut start: usize, mut end: usize) {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    let size = (end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT;
    if size <= current_cpu_data.tlbsize as usize {
        let pid = read_c0_entryhi();
        start &= PAGE_MASK;
        end = (end + PAGE_SIZE - 1) & PAGE_MASK;
        while start < end {
            write_c0_entryhi(start);
            start += PAGE_SIZE; // BARRIER
            tlb_probe();
            let idx = read_c0_index();
            write_c0_entrylo0(0);
            write_c0_entryhi(KSEG0);
            if idx < 0 {
                continue;
            }
            tlb_write_indexed();
        }
        write_c0_entryhi(pid);
    } else {
        local_flush_tlb_all();
    }
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_page(vma: *mut VmAreaStruct, mut page: usize) {
    let asid_mask = cpu_asid_mask(&current_cpu_data);
    let cpu = smp_processor_id();
    if cpu_context(cpu, (*vma).vm_mm) != 0 {
        let mut flags = 0usize;
        let newpid = cpu_context(cpu, (*vma).vm_mm) & asid_mask;
        page &= PAGE_MASK;
        local_irq_save(&mut flags);
        let oldpid = read_c0_entryhi() & asid_mask;
        write_c0_entryhi(page | newpid);
        tlb_probe();
        let idx = read_c0_index();
        write_c0_entrylo0(0);
        write_c0_entryhi(KSEG0);
        if idx >= 0 {
            tlb_write_indexed();
        }
        write_c0_entryhi(oldpid);
        local_irq_restore(flags);
    }
}

pub unsafe fn __update_tlb(vma: *mut VmAreaStruct, mut address: usize, pte: PteT) {
    let asid_mask = cpu_asid_mask(&current_cpu_data);
    let mut flags = 0usize;
    if (*current).active_mm != (*vma).vm_mm {
        return;
    }
    let pid = read_c0_entryhi() & asid_mask;
    local_irq_save(&mut flags);
    address &= PAGE_MASK;
    write_c0_entryhi(address | pid);
    tlb_probe();
    let idx = read_c0_index();
    write_c0_entrylo0(pte_val(pte));
    write_c0_entryhi(address | pid);
    if idx < 0 {
        tlb_write_random();
    } else {
        tlb_write_indexed();
    }
    write_c0_entryhi(pid);
    local_irq_restore(flags);
}

pub unsafe fn add_wired_entry(entrylo0: usize, _entrylo1: usize, entryhi: usize, _pagemask: usize) {
    let asid_mask = cpu_asid_mask(&current_cpu_data);
    let mut flags = 0usize;
    let mut old_ctx: usize;
    static mut wired: usize = 0;
    if wired < 8 {
        local_irq_save(&mut flags);
        old_ctx = read_c0_entryhi() & asid_mask;
        write_c0_entrylo0(entrylo0);
        write_c0_entryhi(entryhi);
        write_c0_index(wired as i32);
        wired += 1; // BARRIER
        tlb_write_indexed();
        write_c0_entryhi(old_ctx);
        local_flush_tlb_all();
        local_irq_restore(flags);
    }
}

pub unsafe fn tlb_init() {
    local_flush_tlb_from(0);
    build_tlb_refill_handler();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
