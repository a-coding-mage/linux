/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Copyright 2008 Freescale Semiconductor Inc.
 *   Port to powerpc added by Kumar Gala
 *
 * Copyright 2011 Michal Simek <monstr@monstr.eu>
 * Copyright 2011 PetaLogix Qld Pty Ltd
 *   Port to Microblaze
 */

/* Translated from the non-assembler portion of the C header. */
/* Dependencies supplied by linux/kernel.h, asm/page.h, linux/threads.h,
 * asm/kmap_size.h, and asm-generic/fixmap.h remain external. */

pub const FIXADDR_TOP: usize = (-(PAGE_SIZE as isize)) as usize;

/*
 * Here we define all the compile-time 'special' virtual
 * addresses. The point is to have a constant address at
 * compile time, but to set the physical address only
 * in the boot process. We allocate these special addresses
 * from the end of virtual memory (0xfffff000) backwards.
 * Also this lets us do fail-safe vmalloc(), we
 * can guarantee that these special addresses and
 * vmalloc()-ed addresses never overlap.
 *
 * these 'compile-time allocated' memory buffers are
 * fixed-size 4k pages. (or larger if used with an increment
 * highger than 1) use fixmap_set(idx,phys) to associate
 * physical memory with fixmap indices.
 *
 * TLB entries of such buffers will not be flushed across
 * task switches.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fixed_addresses {
    FIX_HOLE,
    /* CONFIG_HIGHMEM: reserved pte's for temporary kernel mappings. */
    #[cfg(feature = "CONFIG_HIGHMEM")]
    FIX_KMAP_BEGIN,
    #[cfg(feature = "CONFIG_HIGHMEM")]
    FIX_KMAP_END = FIX_KMAP_BEGIN as isize
        + (KM_MAX_IDX as isize * num_possible_cpus() as isize)
        - 1,
    __end_of_fixed_addresses,
}

extern "C" {
    pub fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, flags: pgprot_t);
}

pub const __FIXADDR_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - __FIXADDR_SIZE;

pub const FIXMAP_PAGE_NOCACHE: pgprot_t = PAGE_KERNEL_CI;

/* Contents of asm-generic/fixmap.h are supplied by the corresponding dependency. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
