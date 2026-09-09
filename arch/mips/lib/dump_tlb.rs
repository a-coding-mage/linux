// SPDX-License-Identifier: GPL-2.0
/*
 * Dump R4x00 TLB for debugging purposes.
 *
 * Copyright (C) 1994, 1995 by Waldorf Electronics, written by Ralf Baechle.
 * Copyright (C) 1999 by Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn dump_tlb_regs() {
    let field: i32 = (2 * core::mem::size_of::<usize>()) as i32;

    pr_info!("Index    : %0x\n", read_c0_index());
    pr_info!("PageMask : %0x\n", read_c0_pagemask());
    if cpu_has_guestid {
        pr_info!("GuestCtl1: %0x\n", read_c0_guestctl1());
    }
    pr_info!("EntryHi  : %0*lx\n", field, read_c0_entryhi());
    pr_info!("EntryLo0 : %0*lx\n", field, read_c0_entrylo0());
    pr_info!("EntryLo1 : %0*lx\n", field, read_c0_entrylo1());
    pr_info!("Wired    : %0x\n", read_c0_wired());
    match current_cpu_type() {
        CPU_R10000 | CPU_R12000 | CPU_R14000 | CPU_R16000 => {
            pr_info!("FrameMask: %0x\n", read_c0_framemask());
        }
        _ => {}
    }
    if cpu_has_small_pages || cpu_has_rixi || cpu_has_xpa {
        pr_info!("PageGrain: %0x\n", read_c0_pagegrain());
    }
    if cpu_has_htw {
        pr_info!("PWField  : %0*lx\n", field, read_c0_pwfield());
        pr_info!("PWSize   : %0*lx\n", field, read_c0_pwsize());
        pr_info!("PWCtl    : %0x\n", read_c0_pwctl());
    }
}

#[inline]
fn msk2str(mask: u32) -> &'static str {
    match mask {
        PM_4K => "4kb",
        PM_16K => "16kb",
        PM_64K => "64kb",
        PM_256K => "256kb",
        // CONFIG_CPU_CAVIUM_OCTEON conditional cases.
        PM_8K => "8kb",
        PM_32K => "32kb",
        PM_128K => "128kb",
        PM_512K => "512kb",
        PM_2M => "2Mb",
        PM_8M => "8Mb",
        PM_32M => "32Mb",
        _ => "",
    }
}

unsafe fn dump_tlb(first: i32, last: i32) {
    let mut s_entryhi: usize;
    let mut entryhi: usize;
    let asid: usize;
    let mut mmid: usize;
    let mut entrylo0: u64;
    let mut entrylo1: u64;
    let mut pa: u64;
    let s_index: u32;
    let s_pagemask: u32;
    let mut s_guestctl1: u32 = 0;
    let mut pagemask: u32;
    let mut guestctl1: u32 = 0;
    let mut c0: u32;
    let mut c1: u32;
    let asidmask: usize = cpu_asid_mask(&current_cpu_data);
    let asidwidth: i32 = (ilog2(asidmask) + 1 + 3) / 4;
    let s_mmid: usize;
    let xpa: bool;
    let pwidth: i32;
    let vwidth: i32;

    // CONFIG_32BIT selects the alternate widths and page-grain test.
    xpa = cpu_has_xpa && (read_c0_pagegrain() & PG_ELPA) != 0;
    pwidth = if xpa { 11 } else { 8 };
    vwidth = 8;

    s_pagemask = read_c0_pagemask();
    s_entryhi = read_c0_entryhi();
    s_index = read_c0_index();

    if cpu_has_mmid {
        s_mmid = read_c0_memorymapid();
        asid = s_mmid;
    } else {
        s_mmid = 0;
        asid = s_entryhi & asidmask;
    }

    if cpu_has_guestid {
        s_guestctl1 = read_c0_guestctl1();
    }

    for i in first..=last {
        write_c0_index(i);
        mtc0_tlbr_hazard();
        tlb_read();
        tlb_read_hazard();
        pagemask = read_c0_pagemask();
        entryhi = read_c0_entryhi();
        entrylo0 = read_c0_entrylo0() as u64;
        entrylo1 = read_c0_entrylo1() as u64;

        mmid = if cpu_has_mmid { read_c0_memorymapid() } else { entryhi & asidmask };
        if cpu_has_guestid { guestctl1 = read_c0_guestctl1(); }

        // EHINV bit marks entire entry as invalid
        if cpu_has_tlbinv && (entryhi & MIPS_ENTRYHI_EHINV) != 0 { continue; }
        // Prior to tlbinv, unused entries have a virtual address of CKSEG0.
        if (entryhi & !0x1ffffusize) == CKSEG0 { continue; }
        // ASID takes effect in absence of G (global) bit.
        if ((entrylo0 | entrylo1) & ENTRYLO_G as u64) == 0 && mmid != asid { continue; }

        printk!("Index: %2d pgmask=%s ", i, msk2str(pagemask));
        c0 = ((entrylo0 & ENTRYLO_C as u64) >> ENTRYLO_C_SHIFT) as u32;
        c1 = ((entrylo1 & ENTRYLO_C as u64) >> ENTRYLO_C_SHIFT) as u32;
        pr_cont!("va=%0*lx asid=%0*lx", vwidth, entryhi & !0x1fffusize, asidwidth, mmid);
        if cpu_has_guestid { pr_cont!(" gid=%02lx", (guestctl1 & MIPS_GCTL1_RID) >> MIPS_GCTL1_RID_SHIFT); }
        pa = entrylo0 & !(MIPS_ENTRYLO_RI | MIPS_ENTRYLO_XI) as u64;
        if xpa { pa |= (readx_c0_entrylo0() as u64) << 30; }
        pa = (pa << 6) & PAGE_MASK as u64;
        pr_cont!("\n\t[");
        if cpu_has_rixi { pr_cont!("ri=%d xi=%d ", (entrylo0 & MIPS_ENTRYLO_RI as u64 != 0) as i32, (entrylo0 & MIPS_ENTRYLO_XI as u64 != 0) as i32); }
        pr_cont!("pa=%0*llx c=%d d=%d v=%d g=%d] [", pwidth, pa, c0, (entrylo0 & ENTRYLO_D as u64 != 0) as i32, (entrylo0 & ENTRYLO_V as u64 != 0) as i32, (entrylo0 & ENTRYLO_G as u64 != 0) as i32);
        pa = entrylo1 & !(MIPS_ENTRYLO_RI | MIPS_ENTRYLO_XI) as u64;
        if xpa { pa |= (readx_c0_entrylo1() as u64) << 30; }
        pa = (pa << 6) & PAGE_MASK as u64;
        if cpu_has_rixi { pr_cont!("ri=%d xi=%d ", (entrylo1 & MIPS_ENTRYLO_RI as u64 != 0) as i32, (entrylo1 & MIPS_ENTRYLO_XI as u64 != 0) as i32); }
        pr_cont!("pa=%0*llx c=%d d=%d v=%d g=%d]\n", pwidth, pa, c1, (entrylo1 & ENTRYLO_D as u64 != 0) as i32, (entrylo1 & ENTRYLO_V as u64 != 0) as i32, (entrylo1 & ENTRYLO_G as u64 != 0) as i32);
    }
    printk!("\n");
    write_c0_entryhi(s_entryhi);
    write_c0_index(s_index);
    write_c0_pagemask(s_pagemask);
    if cpu_has_guestid { write_c0_guestctl1(s_guestctl1); }
}

pub unsafe fn dump_tlb_all() {
    dump_tlb(0, current_cpu_data.tlbsize - 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
