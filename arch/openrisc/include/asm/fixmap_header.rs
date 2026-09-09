/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

/* Why exactly do we need 2 empty pages between the top of the fixed
 * addresses and the top of virtual memory?  Something is using that
 * memory space but not sure what right now... If you find it, leave
 * a comment here.
 */
pub const FIXADDR_TOP: usize = (-(2isize) * PAGE_SIZE as isize) as usize;

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/bug.h, asm/page.h

#[repr(C)]
pub enum fixed_addresses {
    FIX_EARLYCON_MEM_BASE,
    FIX_TEXT_POKE0,
    __end_of_fixed_addresses,
}

pub const FIXADDR_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
/* FIXADDR_BOTTOM might be a better name here... */
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;
pub const FIXMAP_PAGE_IO: _ = PAGE_KERNEL_NOCACHE;

pub extern "C" fn __set_fixmap(
    idx: fixed_addresses,
    phys: phys_addr_t,
    flags: pgprot_t,
);

// Contents supplied by asm-generic/fixmap.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
