/* SPDX-License-Identifier: GPL-2.0 */
/*
 * uClinux flat-format executables
 *
 * Copyright (C) 2005 John Williams <jwilliams@itee.uq.edu.au>
 */

// Dependency supplied by the Linux unaligned-access facilities.

/*
 * Microblaze works a little differently from other arches, because
 * of the MICROBLAZE_64 reloc type. Here, a 32 bit address is split
 * over two instructions, an 'imm' instruction which provides the top
 * 16 bits, then the instruction "proper" which provides the low 16
 * bits.
 */

/*
 * Crack open a symbol reference and extract the address to be
 * relocated. rp is a potentially unaligned pointer to the
 * reference
 */

#[inline]
pub unsafe fn flat_get_addr_from_rp(
    rp: *mut u32,
    relval: u32,
    _flags: u32,
    addr: *mut u32,
) -> i32 {
    let p: *mut u32 = rp;

    /* Is it a split 64/32 reference? */
    if relval & 0x8000_0000 != 0 {
        /* Grab the two halves of the reference */
        let val_hi: u32;
        let val_lo: u32;

        val_hi = get_unaligned(p);
        val_lo = get_unaligned(p.add(1));

        /* Crack the address out */
        *addr = ((val_hi & 0xffff) << 16).wrapping_add(val_lo & 0xffff);
    } else {
        /* Get the address straight out */
        *addr = get_unaligned(p);
    }

    0
}

/*
 * Insert an address into the symbol reference at rp. rp is potentially
 * unaligned.
 */

#[inline]
pub unsafe fn flat_put_addr_at_rp(
    rp: *mut u32,
    addr: u32,
    relval: u32,
) -> i32 {
    let p: *mut u32 = rp;
    /* Is this a split 64/32 reloc? */
    if relval & 0x8000_0000 != 0 {
        /* Get the two "halves" */
        let mut val_hi: usize = get_unaligned(p) as usize;
        let mut val_lo: usize = get_unaligned(p.add(1)) as usize;

        /* insert the address */
        val_hi = (val_hi & 0xffff_0000) | (addr >> 16) as usize;
        val_lo = (val_lo & 0xffff_0000) | (addr & 0xffff) as usize;

        /* store the two halves back into memory */
        put_unaligned(val_hi as u32, p);
        put_unaligned(val_lo as u32, p.add(1));
    } else {
        /* Put it straight in, no messing around */
        put_unaligned(addr, p);
    }
    0
}

#[inline]
pub const fn flat_get_relocate_addr(rel: u32) -> u32 {
    rel & 0x7fff_ffff
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
