/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PowerPC atomic bit operations; translated from the C header. */

/* These names are supplied by the surrounding kernel translation. */
// BITS_PER_LONG, BIT_MASK, BIT_WORD, barriers, __clear_bit, ilog2, and the
// generic bit-operation declarations are external dependencies.

#[inline]
pub const fn ppc_bitlshift(be: usize, bits_per_long: usize) -> usize {
    bits_per_long - 1 - be
}

#[inline]
pub fn ppc_bit(bit: usize, bits_per_long: usize) -> usize {
    1usize << ppc_bitlshift(bit, bits_per_long)
}

#[inline]
pub fn ppc_bitmask(bs: usize, be: usize, bits_per_long: usize) -> usize {
    (ppc_bit(bs, bits_per_long).wrapping_sub(ppc_bit(be, bits_per_long)))
        | ppc_bit(bs, bits_per_long)
}

#[inline]
pub fn ppc_bitextract(bits: usize, ppc_bit_number: usize, dst_bit: usize,
                      bits_per_long: usize) -> usize {
    ((bits >> ppc_bitlshift(ppc_bit_number, bits_per_long)) & 1) << dst_bit
}

#[inline]
pub const fn ppc_bitlshift32(be: usize) -> usize { 32 - 1 - be }
#[inline]
pub fn ppc_bit32(bit: usize) -> usize { 1usize << ppc_bitlshift32(bit) }
#[inline]
pub fn ppc_bitmask32(bs: usize, be: usize) -> usize {
    ppc_bit32(bs).wrapping_sub(ppc_bit32(be)) | ppc_bit32(bs)
}

#[inline]
pub const fn ppc_bitlshift8(be: usize) -> usize { 8 - 1 - be }
#[inline]
pub fn ppc_bit8(bit: usize) -> usize { 1usize << ppc_bitlshift8(bit) }
#[inline]
pub fn ppc_bitmask8(bs: usize, be: usize) -> usize {
    ppc_bit8(bs).wrapping_sub(ppc_bit8(be)) | ppc_bit8(bs)
}

#[inline(always)]
pub fn is_rlwinm_mask_valid(mut x: usize) -> bool {
    if x == 0 { return false; }
    if x & 1 != 0 { x = !x; }
    x = x.wrapping_add(x & x.wrapping_neg());
    (x & x.wrapping_sub(1)) == 0
}

/* The following operations require the PowerPC load-reserve/store-conditional
 * instruction sequences and barrier macros from the original kernel. */
#[inline]
pub unsafe fn set_bits(_mask: usize, _p: *mut usize) {
    // TODO: PPC_LLARX/or/PPC_STLCX retry loop.
    core::hint::spin_loop();
}
#[inline]
pub unsafe fn change_bits(_mask: usize, _p: *mut usize) {
    // TODO: PPC_LLARX/xor/PPC_STLCX retry loop.
    core::hint::spin_loop();
}
#[inline]
pub unsafe fn clear_bits(_mask: usize, _p: *mut usize) {
    // TODO: PPC_LLARX/andc/PPC_STLCX retry loop.
    core::hint::spin_loop();
}
#[inline]
pub unsafe fn clear_bits_unlock(_mask: usize, _p: *mut usize) {
    // TODO: PPC_RELEASE_BARRIER plus PPC_LLARX/andc/PPC_STLCX retry loop.
    core::hint::spin_loop();
}

#[inline]
pub unsafe fn arch_set_bit(nr: i32, addr: *mut usize, bits_per_long: usize,
                           bit_mask: unsafe fn(usize) -> usize,
                           bit_word: unsafe fn(usize) -> usize) {
    set_bits(bit_mask(nr as usize), addr.add(bit_word(nr as usize)));
    let _ = bits_per_long;
}
#[inline]
pub unsafe fn arch_clear_bit(nr: i32, addr: *mut usize, bit_mask: unsafe fn(usize) -> usize,
                             bit_word: unsafe fn(usize) -> usize) {
    clear_bits(bit_mask(nr as usize), addr.add(bit_word(nr as usize)));
}
#[inline]
pub unsafe fn arch_clear_bit_unlock(nr: i32, addr: *mut usize, bit_mask: unsafe fn(usize) -> usize,
                                    bit_word: unsafe fn(usize) -> usize) {
    clear_bits_unlock(bit_mask(nr as usize), addr.add(bit_word(nr as usize)));
}
#[inline]
pub unsafe fn arch_change_bit(nr: i32, addr: *mut usize, bit_mask: unsafe fn(usize) -> usize,
                              bit_word: unsafe fn(usize) -> usize) {
    change_bits(bit_mask(nr as usize), addr.add(bit_word(nr as usize)));
}

#[inline]
pub const fn fls(x: u32) -> i32 { if x == 0 { 0 } else { 32 - x.leading_zeros() as i32 } }

#[cfg(target_pointer_width = "64")]
#[inline]
pub const fn fls64(x: u64) -> i32 { if x == 0 { 0 } else { 64 - x.leading_zeros() as i32 } }

/* Architecture-specific hweight declarations from the original header. */
extern "C" {
    pub fn __arch_hweight8(w: u32) -> u32;
    pub fn __arch_hweight16(w: u32) -> u32;
    pub fn __arch_hweight32(w: u32) -> u32;
    pub fn __arch_hweight64(w: u64) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
