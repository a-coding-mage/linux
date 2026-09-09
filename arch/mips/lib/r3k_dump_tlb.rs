// SPDX-License-Identifier: GPL-2.0
/*
 * Dump R3000 TLB for debugging purposes.
 *
 * Copyright (C) 1994, 1995 by Waldorf Electronics, written by Ralf Baechle.
 * Copyright (C) 1999 by Silicon Graphics, Inc.
 * Copyright (C) 1999 by Harald Koerfgen
 */

// The declarations below are supplied by the surrounding kernel translation.
unsafe extern "C" {
    fn read_c0_index() -> u32;
    fn read_c0_entryhi() -> libc::c_ulong;
    fn read_c0_entrylo0() -> libc::c_ulong;
    fn write_c0_index(value: i32);
    fn write_c0_entryhi(value: libc::c_ulong);
    fn cpu_asid_mask(cpu: *const CurrentCpuData) -> libc::c_ulong;
    static current_cpu_data: CurrentCpuData;
    fn printk(format: *const libc::c_char, ...);
    fn pr_info(format: *const libc::c_char, ...);
    fn pr_cont(format: *const libc::c_char, ...);
}

#[repr(C)]
pub struct CurrentCpuData {
    pub tlbsize: i32,
}

pub unsafe fn dump_tlb_regs() {
    pr_info(b"Index    : %0x\n\0".as_ptr() as *const libc::c_char,
            read_c0_index());
    pr_info(b"EntryHi  : %0lx\n\0".as_ptr() as *const libc::c_char,
            read_c0_entryhi());
    pr_info(b"EntryLo  : %0lx\n\0".as_ptr() as *const libc::c_char,
            read_c0_entrylo0());
}

unsafe fn dump_tlb(first: i32, last: i32) {
    let mut i: i32;
    let mut asid: libc::c_uint;
    let mut entryhi: libc::c_ulong;
    let mut entrylo0: libc::c_ulong;
    let asid_mask: libc::c_ulong;

    asid_mask = cpu_asid_mask(&current_cpu_data);
    asid = (read_c0_entryhi() & asid_mask) as libc::c_uint;

    i = first;
    while i <= last {
        write_c0_index(i << 8);
        core::arch::asm!(
            "tlbr",
            "nop",
            options(nostack, preserves_flags)
        );
        entryhi = read_c0_entryhi();
        entrylo0 = read_c0_entrylo0();

        /* Unused entries have a virtual address of KSEG0.  */
        if (entryhi & PAGE_MASK) != KSEG0
            && ((entrylo0 & R3K_ENTRYLO_G) != 0
                || (entryhi & asid_mask) == asid as libc::c_ulong)
        {
            /*
             * Only print entries in use
             */
            printk(b"Index: %2d \0".as_ptr() as *const libc::c_char, i);

            pr_cont(
                b"va=%08lx asid=%08lx  [pa=%06lx n=%d d=%d v=%d g=%d]\0"
                    .as_ptr() as *const libc::c_char,
                entryhi & PAGE_MASK,
                entryhi & asid_mask,
                entrylo0 & PAGE_MASK,
                if (entrylo0 & R3K_ENTRYLO_N) != 0 { 1 } else { 0 },
                if (entrylo0 & R3K_ENTRYLO_D) != 0 { 1 } else { 0 },
                if (entrylo0 & R3K_ENTRYLO_V) != 0 { 1 } else { 0 },
                if (entrylo0 & R3K_ENTRYLO_G) != 0 { 1 } else { 0 },
            );
        }
        i += 1;
    }
    printk(b"\n\0".as_ptr() as *const libc::c_char);

    write_c0_entryhi(asid as libc::c_ulong);
}

pub unsafe fn dump_tlb_all() {
    dump_tlb(0, current_cpu_data.tlbsize - 1);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
