/* SPDX-License-Identifier: GPL-2.0-only */

// Header guard omitted in Rust: _KMEMLEAK_H.

#[inline]
pub fn kmemleak_free_part_phys(phys: phys_addr_t, size: size_t) {
}

#[inline]
pub fn kmemleak_alloc_phys(phys: phys_addr_t, size: size_t, gfp: gfp_t) {
}

#[inline]
pub fn dump_stack() {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
