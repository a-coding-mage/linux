/*
 * Nios2 TLB handling
 *
 * Copyright (C) 2009, Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux and architecture headers are supplied by the surrounding translation.

const TLB_INDEX_MASK: usize = (((1usize << (cpuinfo.tlb_ptr_sz - cpuinfo.tlb_num_ways_log2)) - 1) << PAGE_SHIFT);

unsafe fn get_misc_and_pid(misc: *mut c_ulong, pid: *mut c_ulong) {
    *misc = RDCTL(CTL_TLBMISC);
    *misc &= TLBMISC_PID | TLBMISC_WAY;
    *pid = *misc & TLBMISC_PID;
}

unsafe fn pteaddr_invalid(addr: c_ulong) -> c_ulong {
    ((addr | 0xC0000000usize as c_ulong) >> PAGE_SHIFT) << 2
}

unsafe fn replace_tlb_one_pid(addr: c_ulong, mmu_pid: c_ulong, tlbacc: c_ulong) {
    let mut way: c_uint;
    let mut org_misc: c_ulong = 0;
    let mut pid_misc: c_ulong = 0;
    get_misc_and_pid(&mut org_misc, &mut pid_misc);
    WRCTL(CTL_PTEADDR, (addr >> PAGE_SHIFT) << 2);
    way = 0;
    while way < cpuinfo.tlb_num_ways {
        let mut tlbmisc: c_ulong = TLBMISC_RD | ((way as c_ulong) << TLBMISC_WAY_SHIFT);
        WRCTL(CTL_TLBMISC, tlbmisc);
        let pteaddr = RDCTL(CTL_PTEADDR);
        if ((pteaddr >> 2) & 0xfffff) != (addr >> PAGE_SHIFT) { way += 1; continue; }
        tlbmisc = RDCTL(CTL_TLBMISC);
        let pid = (tlbmisc >> TLBMISC_PID_SHIFT) & TLBMISC_PID_MASK;
        if pid != mmu_pid { way += 1; continue; }
        tlbmisc = (mmu_pid << TLBMISC_PID_SHIFT) | TLBMISC_WE |
            ((way as c_ulong) << TLBMISC_WAY_SHIFT);
        WRCTL(CTL_TLBMISC, tlbmisc);
        if tlbacc == 0 { WRCTL(CTL_PTEADDR, pteaddr_invalid(addr)); }
        WRCTL(CTL_TLBACC, tlbacc);
        break;
    }
    WRCTL(CTL_TLBMISC, org_misc);
}

unsafe fn flush_tlb_one_pid(addr: c_ulong, mmu_pid: c_ulong) {
    pr_debug!("Flush tlb-entry for vaddr=%#lx\n", addr);
    replace_tlb_one_pid(addr, mmu_pid, 0);
}

unsafe fn reload_tlb_one_pid(addr: c_ulong, mmu_pid: c_ulong, pte: pte_t) {
    pr_debug!("Reload tlb-entry for vaddr=%#lx\n", addr);
    replace_tlb_one_pid(addr, mmu_pid, pte_val(pte));
}

pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, mut start: c_ulong, end: c_ulong) {
    let mmu_pid = get_pid_from_context(&mut (*(*vma).vm_mm).context);
    while start < end { flush_tlb_one_pid(start, mmu_pid); start += PAGE_SIZE; }
}

pub unsafe fn reload_tlb_page(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) {
    let mmu_pid = get_pid_from_context(&mut (*(*vma).vm_mm).context);
    reload_tlb_one_pid(addr, mmu_pid, pte);
}

unsafe fn flush_tlb_one(addr: c_ulong) {
    let mut org_misc: c_ulong = 0;
    let mut pid_misc: c_ulong = 0;
    pr_debug!("Flush tlb-entry for vaddr=%#lx\n", addr);
    get_misc_and_pid(&mut org_misc, &mut pid_misc);
    WRCTL(CTL_PTEADDR, (addr >> PAGE_SHIFT) << 2);
    let mut way: c_uint = 0;
    while way < cpuinfo.tlb_num_ways {
        let mut tlbmisc = TLBMISC_RD | ((way as c_ulong) << TLBMISC_WAY_SHIFT);
        WRCTL(CTL_TLBMISC, tlbmisc);
        let pteaddr = RDCTL(CTL_PTEADDR);
        if ((pteaddr >> 2) & 0xfffff) == (addr >> PAGE_SHIFT) {
            tlbmisc = RDCTL(CTL_TLBMISC);
            pr_debug!("Flush entry by writing way=%dl pid=%ld\n", way, (tlbmisc >> TLBMISC_PID_SHIFT) & TLBMISC_PID_MASK);
            tlbmisc = TLBMISC_WE | ((way as c_ulong) << TLBMISC_WAY_SHIFT) | (tlbmisc & TLBMISC_PID);
            WRCTL(CTL_TLBMISC, tlbmisc); WRCTL(CTL_PTEADDR, pteaddr_invalid(addr)); WRCTL(CTL_TLBACC, 0);
        }
        way += 1;
    }
    WRCTL(CTL_TLBMISC, org_misc);
}

pub unsafe fn flush_tlb_kernel_range(mut start: c_ulong, end: c_ulong) { while start < end { flush_tlb_one(start); start += PAGE_SIZE; } }

pub unsafe fn dump_tlb_line(line: c_ulong) {
    let mut org_misc: c_ulong;
    pr_debug!("dump tlb-entries for line=%#lx (addr %08lx)\n", line, line << (PAGE_SHIFT + cpuinfo.tlb_num_ways_log2));
    org_misc = RDCTL(CTL_TLBMISC) & (TLBMISC_PID | TLBMISC_WAY);
    WRCTL(CTL_PTEADDR, line << 2);
    let mut way: c_uint = 0;
    while way < cpuinfo.tlb_num_ways {
        WRCTL(CTL_TLBMISC, TLBMISC_RD | ((way as c_ulong) << TLBMISC_WAY_SHIFT));
        let pteaddr = RDCTL(CTL_PTEADDR); let tlbmisc = RDCTL(CTL_TLBMISC); let tlbacc = RDCTL(CTL_TLBACC);
        if (tlbacc << PAGE_SHIFT) != 0 {
            pr_debug!("-- way:%02x vpn:0x%08lx phys:0x%08lx pid:0x%02lx flags:%c%c%c%c%c\n", way, pteaddr << (PAGE_SHIFT-2), tlbacc << PAGE_SHIFT, (tlbmisc >> TLBMISC_PID_SHIFT) & TLBMISC_PID_MASK, if tlbacc & _PAGE_READ != 0 {'r'} else {'-'}, if tlbacc & _PAGE_WRITE != 0 {'w'} else {'-'}, if tlbacc & _PAGE_EXEC != 0 {'x'} else {'-'}, if tlbacc & _PAGE_GLOBAL != 0 {'g'} else {'-'}, if tlbacc & _PAGE_CACHED != 0 {'c'} else {'-'});
        }
        way += 1;
    }
    WRCTL(CTL_TLBMISC, org_misc);
}

pub unsafe fn dump_tlb() { let mut i: c_uint = 0; while i < cpuinfo.tlb_num_lines { dump_tlb_line(i as c_ulong); i += 1; } }

pub unsafe fn flush_tlb_pid(mmu_pid: c_ulong) {
    let mut addr: c_ulong = 0; let mut org_misc = 0; let mut pid_misc = 0;
    get_misc_and_pid(&mut org_misc, &mut pid_misc);
    let mut line: c_uint = 0;
    while line < cpuinfo.tlb_num_lines {
        WRCTL(CTL_PTEADDR, pteaddr_invalid(addr)); let mut way: c_uint = 0;
        while way < cpuinfo.tlb_num_ways {
            let mut tlbmisc = TLBMISC_RD | ((way as c_ulong) << TLBMISC_WAY_SHIFT); WRCTL(CTL_TLBMISC, tlbmisc); tlbmisc = RDCTL(CTL_TLBMISC);
            let pid = (tlbmisc >> TLBMISC_PID_SHIFT) & TLBMISC_PID_MASK;
            if pid == mmu_pid { WRCTL(CTL_TLBMISC, TLBMISC_WE | ((way as c_ulong) << TLBMISC_WAY_SHIFT) | (pid << TLBMISC_PID_SHIFT)); WRCTL(CTL_TLBACC, 0); }
            way += 1;
        }
        addr += PAGE_SIZE; line += 1;
    }
    WRCTL(CTL_TLBMISC, org_misc);
}

pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    if (*current).mm == mm { let mmu_pid = get_pid_from_context(&mut (*mm).context); flush_tlb_pid(mmu_pid); }
    else { core::ptr::write_bytes(&mut (*mm).context as *mut mm_context_t, 0, 1); }
}

pub unsafe fn flush_tlb_all() {
    let mut addr: c_ulong = 0; let mut org_misc = 0; let mut pid_misc = 0; get_misc_and_pid(&mut org_misc, &mut pid_misc);
    let mut line: c_uint = 0;
    while line < cpuinfo.tlb_num_lines { WRCTL(CTL_PTEADDR, pteaddr_invalid(addr)); let mut way: c_uint = 0; while way < cpuinfo.tlb_num_ways { WRCTL(CTL_TLBMISC, TLBMISC_WE | ((way as c_ulong) << TLBMISC_WAY_SHIFT) | ((way as c_ulong) << TLBMISC_PID_SHIFT)); WRCTL(CTL_TLBACC, 0); way += 1; } addr += PAGE_SIZE; line += 1; }
    WRCTL(CTL_TLBMISC, org_misc);
}

pub unsafe fn set_mmu_pid(pid: c_ulong) { let mut tlbmisc = RDCTL(CTL_TLBMISC) & TLBMISC_WAY; tlbmisc |= (pid & TLBMISC_PID_MASK) << TLBMISC_PID_SHIFT; WRCTL(CTL_TLBMISC, tlbmisc); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
