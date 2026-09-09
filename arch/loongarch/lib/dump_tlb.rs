// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1994, 1995 by Waldorf Electronics, written by Ralf Baechle.
 * Copyright (C) 1999 by Silicon Graphics, Inc.
 */

// Dependencies supplied by the kernel and architecture headers are intentionally
// left as external symbols for the surrounding translation unit.

extern "C" {
    fn read_csr_tlbidx() -> u32;
    fn read_csr_pagesize() -> u32;
    fn read_csr_entryhi() -> u64;
    fn read_csr_entrylo0() -> u64;
    fn read_csr_entrylo1() -> u64;
    fn read_csr_asid() -> u32;
    fn write_csr_index(index: u32);
    fn tlb_read();
    fn write_csr_entryhi(value: u64);
    fn write_csr_tlbidx(value: u32);
    fn write_csr_asid(value: u32);
    fn cpu_asid_mask(cpu: *const CpuData) -> u64;
    fn ilog2(value: u64) -> u32;
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn pr_cont(format: *const core::ffi::c_char, ...);
    static current_cpu_data: CpuData;
}

#[repr(C)]
pub struct CpuData {
    pub tlbsize: i32,
}

const CSR_TLBIDX_EHINV: u32 = 1 << 31;
const ENTRYLO_G: u64 = 1 << 6;
const ENTRYLO_C: u64 = 0x7 << 3;
const ENTRYLO_C_SHIFT: u32 = 3;
const ENTRYLO_NR: u64 = 1 << 61;
const ENTRYLO_NX: u64 = 1 << 62;
const ENTRYLO_D: u64 = 1 << 1;
const ENTRYLO_V: u64 = 1 << 0;
const ENTRYLO_PLV: u64 = 0x3 << 2;
const ENTRYLO_PLV_SHIFT: u32 = 2;
const PAGE_MASK: u64 = !0xfff;

pub unsafe fn dump_tlb_regs() {
    let field: i32 = 2 * core::mem::size_of::<usize>() as i32;

    pr_info(b"Index    : 0x%0x\0".as_ptr() as _, read_csr_tlbidx());
    pr_info(b"PageSize : 0x%0x\0".as_ptr() as _, read_csr_pagesize());
    pr_info(b"EntryHi  : 0x%0*lx\0".as_ptr() as _, field, read_csr_entryhi());
    pr_info(b"EntryLo0 : 0x%0*lx\0".as_ptr() as _, field, read_csr_entrylo0());
    pr_info(b"EntryLo1 : 0x%0*lx\0".as_ptr() as _, field, read_csr_entrylo1());
}

unsafe fn dump_tlb(first: i32, last: i32) {
    let mut s_entryhi: u64;
    let mut entryhi: u64;
    let mut asid: u64;
    let mut entrylo0: u64;
    let mut entrylo1: u64;
    let mut pa: u64;
    let mut index: u32;
    let s_index: u32;
    let s_asid: u32;
    let mut pagesize: u32;
    let mut c0: u32;
    let mut c1: u32;
    let asidmask = cpu_asid_mask(&current_cpu_data);
    let pwidth = 16;
    let vwidth = 16;
    let asidwidth = (ilog2(asidmask) + 1).div_ceil(4);

    s_entryhi = read_csr_entryhi();
    s_index = read_csr_tlbidx();
    s_asid = read_csr_asid();

    let mut i = first;
    while i <= last {
        write_csr_index(i as u32);
        tlb_read();
        pagesize = read_csr_pagesize();
        entryhi = read_csr_entryhi();
        entrylo0 = read_csr_entrylo0();
        entrylo1 = read_csr_entrylo1();
        index = read_csr_tlbidx();
        asid = read_csr_asid() as u64;

        // EHINV bit marks entire entry as invalid
        if index & CSR_TLBIDX_EHINV != 0 { i += 1; continue; }
        // ASID takes effect in absence of G (global) bit.
        if ((entrylo0 | entrylo1) & ENTRYLO_G) == 0 && asid != s_asid as u64 { i += 1; continue; }

        // Only print entries in use
        pr_info(b"Index: %4d pgsize=0x%x \0".as_ptr() as _, i, 1u32 << pagesize);
        c0 = ((entrylo0 & ENTRYLO_C) >> ENTRYLO_C_SHIFT) as u32;
        c1 = ((entrylo1 & ENTRYLO_C) >> ENTRYLO_C_SHIFT) as u32;
        pr_cont(b"va=0x%0*lx asid=0x%0*lx\0".as_ptr() as _, vwidth, entryhi & !0x1fff, asidwidth, asid & asidmask);

        // NR/NX are in awkward places, so mask them off separately
        pa = entrylo0 & !(ENTRYLO_NR | ENTRYLO_NX);
        pa &= PAGE_MASK;
        pr_cont(b"\n\t[\0".as_ptr() as _);
        pr_cont(b"nr=%d nx=%d \0".as_ptr() as _, (entrylo0 & ENTRYLO_NR != 0) as i32, (entrylo0 & ENTRYLO_NX != 0) as i32);
        pr_cont(b"pa=0x%0*llx c=%d d=%d v=%d g=%d plv=%lld] [\0".as_ptr() as _, pwidth, pa, c0, (entrylo0 & ENTRYLO_D != 0) as i32, (entrylo0 & ENTRYLO_V != 0) as i32, (entrylo0 & ENTRYLO_G != 0) as i32, (entrylo0 & ENTRYLO_PLV) >> ENTRYLO_PLV_SHIFT);
        pa = entrylo1 & !(ENTRYLO_NR | ENTRYLO_NX);
        pa &= PAGE_MASK;
        pr_cont(b"nr=%d nx=%d \0".as_ptr() as _, (entrylo1 & ENTRYLO_NR != 0) as i32, (entrylo1 & ENTRYLO_NX != 0) as i32);
        pr_cont(b"pa=0x%0*llx c=%d d=%d v=%d g=%d plv=%lld]\n\0".as_ptr() as _, pwidth, pa, c1, (entrylo1 & ENTRYLO_D != 0) as i32, (entrylo1 & ENTRYLO_V != 0) as i32, (entrylo1 & ENTRYLO_G != 0) as i32, (entrylo1 & ENTRYLO_PLV) >> ENTRYLO_PLV_SHIFT);
        i += 1;
    }
    pr_info(b"\n\0".as_ptr() as _);
    write_csr_entryhi(s_entryhi);
    write_csr_tlbidx(s_index);
    write_csr_asid(s_asid);
}

pub unsafe fn dump_tlb_all() {
    dump_tlb(0, current_cpu_data.tlbsize - 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
