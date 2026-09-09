// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines for doing kexec-based kdump.
 *
 * Copyright (C) 2005, IBM Corp.
 *
 * Created by: Michael Ellerman
 */

// C dependencies:
// linux/crash_dump.h, linux/io.h, linux/memblock.h, linux/of.h,
// asm/text-patching.h, asm/kdump.h, asm/firmware.h, linux/uio.h,
// asm/rtas.h, asm/inst.h, asm/fadump.h

#[cfg(feature = "debug")]
macro_rules! DBG {
    ($($arg:tt)*) => { udbg_printf(format_args!($($arg)*)); };
}
#[cfg(not(feature = "debug"))]
macro_rules! DBG {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "config_nonstatic_kernel"))]
pub unsafe fn reserve_kdump_trampoline() {
    memblock_reserve(0, KDUMP_RESERVE_LIMIT);
}

#[cfg(not(feature = "config_nonstatic_kernel"))]
unsafe fn create_trampoline(addr: c_ulong) {
    let p = addr as *mut u32;

    /* The maximum range of a single instruction branch, is the current
     * instruction's address + (32 MB - 4) bytes. For the trampoline we
     * need to branch to current address + 32 MB. So we insert a nop at
     * the trampoline address, then the next instruction (+ 4 bytes)
     * does a branch to (32 MB - 4). The net effect is that when we
     * branch to "addr" we jump to ("addr" + 32 MB). Although it requires
     * two instructions it doesn't require any registers.
     */
    patch_instruction(p, ppc_inst(PPC_RAW_NOP()));
    patch_branch(p.add(1), addr.wrapping_add(PHYSICAL_START), 0);
}

#[cfg(not(feature = "config_nonstatic_kernel"))]
pub unsafe fn setup_kdump_trampoline() {
    let mut i: c_ulong;

    DBG!(" -> setup_kdump_trampoline()\n");

    i = KDUMP_TRAMPOLINE_START;
    while i < KDUMP_TRAMPOLINE_END {
        create_trampoline(i);
        i = i.wrapping_add(8);
    }

    #[cfg(feature = "config_ppc_pseries")]
    {
        create_trampoline(__pa(system_reset_fwnmi).wrapping_sub(PHYSICAL_START));
        create_trampoline(__pa(machine_check_fwnmi).wrapping_sub(PHYSICAL_START));
    }

    DBG!(" <- setup_kdump_trampoline()\n");
}

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: c_ulong,
    mut csize: usize,
    offset: c_ulong,
) -> isize {
    let vaddr: *mut c_void;
    let paddr: phys_addr_t;

    if csize == 0 {
        return 0;
    }

    csize = core::cmp::min(csize, PAGE_SIZE);
    paddr = pfn << PAGE_SHIFT;

    if memblock_is_region_memory(paddr, csize) {
        vaddr = __va(paddr);
        csize = copy_to_iter(vaddr.add(offset as usize), csize, iter);
    } else {
        vaddr = ioremap_cache(paddr, PAGE_SIZE);
        csize = copy_to_iter(vaddr.add(offset as usize), csize, iter);
        iounmap(vaddr);
    }

    csize as isize
}

/*
 * Return true only when kexec based kernel dump capturing method is used.
 * This ensures all restritions applied for kdump case are not automatically
 * applied for fadump case.
 */
pub unsafe fn is_kdump_kernel() -> bool {
    !is_fadump_active() && elfcorehdr_addr != ELFCORE_ADDR_MAX
}
// EXPORT_SYMBOL_GPL(is_kdump_kernel);

#[cfg(feature = "config_ppc_rtas")]
pub unsafe fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong) {
    let mut addr: c_ulong;
    let mut rtas_start: u32 = 0;
    let mut rtas_end: u32 = 0;

    let basep = of_get_property(rtas.dev, b"linux,rtas-base\0".as_ptr() as *const c_char, core::ptr::null_mut());
    let sizep = of_get_property(rtas.dev, b"rtas-size\0".as_ptr() as *const c_char, core::ptr::null_mut());

    if !basep.is_null() && !sizep.is_null() {
        rtas_start = be32_to_cpup(basep);
        rtas_end = rtas_start.wrapping_add(be32_to_cpup(sizep));
    }

    addr = begin;
    while addr < end {
        /* Does this page overlap with the RTAS region? */
        if addr <= rtas_end as c_ulong
            && addr.wrapping_add(PAGE_SIZE) > rtas_start as c_ulong
        {
            addr = addr.wrapping_add(PAGE_SIZE);
            continue;
        }

        free_reserved_page(pfn_to_page(addr >> PAGE_SHIFT));
        addr = addr.wrapping_add(PAGE_SIZE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
