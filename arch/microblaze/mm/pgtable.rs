/*
 *  This file contains the routines setting up the linux page tables.
 *
 * Copyright (C) 2008 Michal Simek
 * Copyright (C) 2008 PetaLogix
 *
 *    Copyright (C) 2007 Xilinx, Inc.  All rights reserved.
 *
 *  Derived from arch/ppc/mm/pgtable.c:
 *    -- paulus
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *  Amiga/APUS changes by Jesper Skov (jskov@cygnus.co.uk).
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 *
 *  This file is subject to the terms and conditions of the GNU General
 *  Public License.  See the file COPYING in the main directory of this
 *  archive for more details.
 */

pub static mut ioremap_base: ::core::ffi::c_ulong = 0;
#[no_mangle]
pub static mut ioremap_bot: ::core::ffi::c_ulong = 0;

unsafe fn __ioremap(addr: phys_addr_t, mut size: ::core::ffi::c_ulong,
                    mut flags: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    let (mut v, mut i): (::core::ffi::c_ulong, ::core::ffi::c_ulong);
    let mut p: phys_addr_t;
    let mut err: i32;

    /* Choose an address to map it to. */
    p = addr & PAGE_MASK;
    size = PAGE_ALIGN(addr + size) - p;

    if mem_init_done != 0 && p >= memory_start && p < virt_to_phys(high_memory)
        && !(p >= __virt_to_phys(__bss_stop as phys_addr_t)
            && p < __virt_to_phys(__bss_stop as phys_addr_t)) {
        pr_warn("__ioremap(): phys addr is RAM lr %ps\n", p, 0);
        return ::core::ptr::null_mut();
    }

    if size == 0 { return ::core::ptr::null_mut(); }

    if mem_init_done != 0 {
        let area = get_vm_area(size, VM_IOREMAP);
        if area.is_null() { return ::core::ptr::null_mut(); }
        v = (*area).addr as ::core::ffi::c_ulong;
    } else {
        ioremap_bot -= size;
        v = ioremap_bot;
    }

    if (flags & _PAGE_PRESENT) == 0 { flags |= _PAGE_KERNEL; }
    if (flags & _PAGE_NO_CACHE) != 0 { flags |= _PAGE_GUARDED; }

    err = 0;
    i = 0;
    while i < size && err == 0 {
        err = map_page(v + i, p + i, flags as i32);
        i += PAGE_SIZE;
    }
    if err != 0 {
        if mem_init_done != 0 { vfree(v as *mut ::core::ffi::c_void); }
        return ::core::ptr::null_mut();
    }
    (v + ((addr as ::core::ffi::c_ulong) & !PAGE_MASK)) as *mut ::core::ffi::c_void
}

pub unsafe fn ioremap(addr: phys_addr_t, size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    __ioremap(addr, size, _PAGE_NO_CACHE)
}

pub unsafe fn iounmap(addr: *mut ::core::ffi::c_void) {
    if (addr as ::core::ffi::c_ulong) > (high_memory as ::core::ffi::c_ulong)
        && (addr as ::core::ffi::c_ulong) < ioremap_bot {
        vfree((PAGE_MASK & addr as ::core::ffi::c_ulong) as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn map_page(va: ::core::ffi::c_ulong, pa: phys_addr_t, flags: i32) -> i32 {
    let p4d = p4d_offset(pgd_offset_k(va), va);
    let pud = pud_offset(p4d, va);
    let pd = pmd_offset(pud, va);
    let pg = pte_alloc_kernel(pd, va);
    let mut err: i32 = -ENOMEM;
    if !pg.is_null() {
        err = 0;
        set_pte_at(&init_mm, va, pg, pfn_pte(pa >> PAGE_SHIFT, __pgprot(flags)));
        if unlikely(mem_init_done != 0) { _tlbie(va); }
    }
    err
}

pub unsafe fn mapin_ram() {
    let mut v = CONFIG_KERNEL_START;
    let mut p = memory_start;
    let mut s = 0;
    while s < lowmem_size {
        let mut f = _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_SHARED | _PAGE_HWEXEC;
        if !is_kernel_text(v) { f |= _PAGE_WRENABLE; }
        else { f |= _PAGE_USER; }
        map_page(v, p, f as i32);
        v += PAGE_SIZE; p += PAGE_SIZE; s += PAGE_SIZE;
    }
}

#[inline]
fn is_power_of_2(x: ::core::ffi::c_ulong) -> bool { x != 0 && (x & (x - 1)) == 0 }

unsafe fn get_pteptr(mm: *mut mm_struct, addr: ::core::ffi::c_ulong,
                     ptep: *mut *mut pte_t) -> i32 {
    let pgd = pgd_offset(mm, addr & PAGE_MASK);
    let mut retval = 0;
    if !pgd.is_null() {
        let p4d = p4d_offset(pgd, addr & PAGE_MASK);
        let pud = pud_offset(p4d, addr & PAGE_MASK);
        let pmd = pmd_offset(pud, addr & PAGE_MASK);
        if pmd_present(*pmd) {
            let pte = pte_offset_kernel(pmd, addr & PAGE_MASK);
            if !pte.is_null() { retval = 1; *ptep = pte; }
        }
    }
    retval
}

pub unsafe fn iopa(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mm = if addr < TASK_SIZE { (*current).mm } else { &mut init_mm };
    let mut pte: *mut pte_t = ::core::ptr::null_mut();
    let mut pa = 0;
    if get_pteptr(mm, addr, &mut pte) != 0 { pa = (pte_val(*pte) & PAGE_MASK) | (addr & !PAGE_MASK); }
    pa
}

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    if mem_init_done != 0 { return __pte_alloc_one_kernel(mm); }
    memblock_alloc_try_nid(PAGE_SIZE, PAGE_SIZE, MEMBLOCK_LOW_LIMIT,
                           memory_start + kernel_tlb, NUMA_NO_NODE)
}

pub unsafe fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, flags: pgprot_t) {
    let address = __fix_to_virt(idx);
    if (idx as usize) >= __end_of_fixed_addresses as usize { BUG(); }
    map_page(address, phys, pgprot_val(flags) as i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
