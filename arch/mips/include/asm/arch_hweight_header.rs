/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 */

/* Preserved build-time condition: ARCH_HAS_USABLE_BUILTIN_POPCOUNT. */
#[cfg(ARCH_HAS_USABLE_BUILTIN_POPCOUNT)]
#[inline]
fn __arch_hweight32(w: u32) -> u32 {
    w.count_ones()
}

#[cfg(ARCH_HAS_USABLE_BUILTIN_POPCOUNT)]
#[inline]
fn __arch_hweight16(w: u32) -> u32 {
    (w & 0xffff).count_ones()
}

#[cfg(ARCH_HAS_USABLE_BUILTIN_POPCOUNT)]
#[inline]
fn __arch_hweight8(w: u32) -> u32 {
    (w & 0xff).count_ones()
}

#[cfg(ARCH_HAS_USABLE_BUILTIN_POPCOUNT)]
#[inline]
fn __arch_hweight64(w: u64) -> u64 {
    w.count_ones() as u64
}

/* Otherwise, use the declarations supplied by asm-generic/bitops/arch_hweight.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
