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
 * Copyright (C) 2001, 2004, 2007  Maciej W. Rozycki
 */

// Symbols supplied by the included Linux/MIPS headers are external dependencies.

static mut icache_size: ::core::primitive::c_ulong = 0;
static mut dcache_size: ::core::primitive::c_ulong = 0;
static mut icache_lsize: ::core::primitive::c_ulong = 0;
static mut dcache_lsize: ::core::primitive::c_ulong = 0;

pub unsafe fn r3k_cache_size(ca_flags: ::core::primitive::c_ulong) -> ::core::primitive::c_ulong {
    let mut flags: ::core::primitive::c_ulong;
    let mut status: ::core::primitive::c_ulong;
    let mut dummy: ::core::primitive::c_ulong;
    let mut size: ::core::primitive::c_ulong;
    let p = KSEG0 as *mut ::core::primitive::c_ulong;

    flags = read_c0_status();
    write_c0_status((ca_flags | flags) & !ST0_IEC);
    core::ptr::write_volatile(p, 0xa5a55a5a);
    dummy = core::ptr::read_volatile(p);
    status = read_c0_status();
    if dummy != 0xa5a55a5a || (status & ST0_CM) != 0 {
        size = 0;
    } else {
        size = 128;
        while size <= 0x40000 {
            core::ptr::write_volatile(p.add(size as usize), 0);
            size <<= 1;
        }
        core::ptr::write_volatile(p, !0);
        size = 128;
        while size <= 0x40000 && core::ptr::read_volatile(p.add(size as usize)) == 0 {
            size <<= 1;
        }
        if size > 0x40000 { size = 0; }
    }
    write_c0_status(flags);
    size * core::mem::size_of::<::core::primitive::c_ulong>() as ::core::primitive::c_ulong
}

pub unsafe fn r3k_cache_lsize(ca_flags: ::core::primitive::c_ulong) -> ::core::primitive::c_ulong {
    let flags = read_c0_status();
    let p = KSEG0 as *mut ::core::primitive::c_ulong;
    write_c0_status((ca_flags | flags) & !ST0_IEC);
    for i in 0..128 { core::ptr::write_volatile(p.add(i), 0); }
    core::ptr::write_volatile(p as *mut u8, 0);
    let mut lsize = 1;
    while lsize < 128 {
        let _ = core::ptr::read_volatile(p.add(lsize as usize));
        let status = read_c0_status();
        if (status & ST0_CM) == 0 { break; }
        lsize <<= 1;
    }
    let mut i = 0;
    while i < 128 { core::ptr::write_volatile(p.add(i), 0); i += lsize; }
    write_c0_status(flags);
    lsize * core::mem::size_of::<::core::primitive::c_ulong>() as ::core::primitive::c_ulong
}

unsafe fn r3k_probe_cache() {
    dcache_size = r3k_cache_size(ST0_ISC);
    if dcache_size != 0 { dcache_lsize = r3k_cache_lsize(ST0_ISC); }
    icache_size = r3k_cache_size(ST0_ISC | ST0_SWC);
    if icache_size != 0 { icache_lsize = r3k_cache_lsize(ST0_ISC | ST0_SWC); }
}

unsafe fn r3k_flush_icache_range(mut start: ::core::primitive::c_ulong, end: ::core::primitive::c_ulong) {
    let mut size = end.wrapping_sub(start);
    if size > icache_size || KSEGX(start) != KSEG0 { start = KSEG0; size = icache_size; }
    let mut p = start as *mut u8;
    let flags = read_c0_status();
    write_c0_status((ST0_ISC | ST0_SWC | flags) & !ST0_IEC);
    let mut i = 0;
    while i < size {
        // Original MIPS inline assembly invalidates the 32 byte cache-line stores at p.
        core::arch::asm!("sb $0, 0x000({0})", "sb $0, 0x004({0})", "sb $0, 0x008({0})", "sb $0, 0x00c({0})", "sb $0, 0x010({0})", "sb $0, 0x014({0})", "sb $0, 0x018({0})", "sb $0, 0x01c({0})", "sb $0, 0x020({0})", "sb $0, 0x024({0})", "sb $0, 0x028({0})", "sb $0, 0x02c({0})", "sb $0, 0x030({0})", "sb $0, 0x034({0})", "sb $0, 0x038({0})", "sb $0, 0x03c({0})", "sb $0, 0x040({0})", "sb $0, 0x044({0})", "sb $0, 0x048({0})", "sb $0, 0x04c({0})", "sb $0, 0x050({0})", "sb $0, 0x054({0})", "sb $0, 0x058({0})", "sb $0, 0x05c({0})", "sb $0, 0x060({0})", "sb $0, 0x064({0})", "sb $0, 0x068({0})", "sb $0, 0x06c({0})", "sb $0, 0x070({0})", "sb $0, 0x074({0})", "sb $0, 0x078({0})", "sb $0, 0x07c({0})", in(reg) p);
        p = p.add(0x080); i += 0x080;
    }
    write_c0_status(flags);
}

unsafe fn r3k_flush_dcache_range(mut start: ::core::primitive::c_ulong, end: ::core::primitive::c_ulong) {
    let mut size = end.wrapping_sub(start);
    if size > dcache_size || KSEGX(start) != KSEG0 { start = KSEG0; size = dcache_size; }
    let mut p = start as *mut u8;
    let flags = read_c0_status();
    write_c0_status((ST0_ISC | flags) & !ST0_IEC);
    let mut i = 0;
    while i < size {
        core::arch::asm!("sb $0, 0x000({0})", "sb $0, 0x004({0})", "sb $0, 0x008({0})", "sb $0, 0x00c({0})", "sb $0, 0x010({0})", "sb $0, 0x014({0})", "sb $0, 0x018({0})", "sb $0, 0x01c({0})", "sb $0, 0x020({0})", "sb $0, 0x024({0})", "sb $0, 0x028({0})", "sb $0, 0x02c({0})", "sb $0, 0x030({0})", "sb $0, 0x034({0})", "sb $0, 0x038({0})", "sb $0, 0x03c({0})", "sb $0, 0x040({0})", "sb $0, 0x044({0})", "sb $0, 0x048({0})", "sb $0, 0x04c({0})", "sb $0, 0x050({0})", "sb $0, 0x054({0})", "sb $0, 0x058({0})", "sb $0, 0x05c({0})", "sb $0, 0x060({0})", "sb $0, 0x064({0})", "sb $0, 0x068({0})", "sb $0, 0x06c({0})", "sb $0, 0x070({0})", "sb $0, 0x074({0})", "sb $0, 0x078({0})", "sb $0, 0x07c({0})", in(reg) p);
        p = p.add(0x080); i += 0x080;
    }
    write_c0_status(flags);
}

#[inline] unsafe fn r3k_flush_cache_all() {}
#[inline] unsafe fn r3k___flush_cache_all() { r3k_flush_dcache_range(KSEG0, KSEG0 + dcache_size); r3k_flush_icache_range(KSEG0, KSEG0 + icache_size); }
unsafe fn r3k_flush_cache_mm(_mm: *mut mm_struct) {}
unsafe fn r3k_flush_cache_range(_vma: *mut vm_area_struct, _start: ::core::primitive::c_ulong, _end: ::core::primitive::c_ulong) {}

unsafe fn r3k_flush_cache_page(vma: *mut vm_area_struct, addr: ::core::primitive::c_ulong, pfn: ::core::primitive::c_ulong) {
    let kaddr = KSEG0ADDR(pfn << PAGE_SHIFT);
    let exec = (*vma).vm_flags & VM_EXEC;
    let mm = (*vma).vm_mm;
    pr_debug!("cpage[{:#08x},{:#08x}]\n", cpu_context(smp_processor_id(), mm), addr);
    if cpu_context(smp_processor_id(), mm) == 0 { return; }
    let pmdp = pmd_off(mm, addr);
    let ptep = pte_offset_kernel(pmdp, addr);
    if (pte_val(*ptep) & _PAGE_PRESENT) == 0 { return; }
    r3k_flush_dcache_range(kaddr, kaddr + PAGE_SIZE);
    if exec != 0 { r3k_flush_icache_range(kaddr, kaddr + PAGE_SIZE); }
}

unsafe fn r3k_flush_data_cache_page(_addr: ::core::primitive::c_ulong) {}
unsafe fn r3k_flush_kernel_vmap_range(_vaddr: ::core::primitive::c_ulong, _size: i32) { BUG!(); }
unsafe fn r3k_dma_cache_wback_inv(start: ::core::primitive::c_ulong, size: ::core::primitive::c_ulong) { BUG_ON!(size == 0); iob(); r3k_flush_dcache_range(start, start + size); }

pub unsafe fn r3k_cache_init() {
    unsafe extern "C" { fn build_clear_page(); fn build_copy_page(); }
    r3k_probe_cache();
    flush_cache_all = Some(r3k_flush_cache_all);
    __flush_cache_all = Some(r3k___flush_cache_all);
    flush_cache_mm = Some(r3k_flush_cache_mm);
    flush_cache_range = Some(r3k_flush_cache_range);
    flush_cache_page = Some(r3k_flush_cache_page);
    flush_icache_range = Some(r3k_flush_icache_range);
    local_flush_icache_range = Some(r3k_flush_icache_range);
    __flush_icache_user_range = Some(r3k_flush_icache_range);
    __local_flush_icache_user_range = Some(r3k_flush_icache_range);
    __flush_kernel_vmap_range = Some(r3k_flush_kernel_vmap_range);
    flush_data_cache_page = Some(r3k_flush_data_cache_page);
    _dma_cache_wback_inv = Some(r3k_dma_cache_wback_inv);
    _dma_cache_wback = Some(r3k_dma_cache_wback_inv);
    _dma_cache_inv = Some(r3k_dma_cache_wback_inv);
    pr_info!("Primary instruction cache {}kB, linesize {} bytes.\n", icache_size >> 10, icache_lsize);
    pr_info!("Primary data cache {}kB, linesize {} bytes.\n", dcache_size >> 10, dcache_lsize);
    build_clear_page(); build_copy_page();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
