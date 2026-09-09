/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and include directives have no Rust equivalent.

// For those 68K CPUs that support 64bit multiply define umul_ppmm()
// for the common muldi3 libgcc helper function (in lib/muldi3.c).
// CPUs that don't have it (like the original 68000 and ColdFire)
// will fallback to using the C-coded version of umul_ppmm().
//
// This declaration is conditional on !CONFIG_CPU_HAS_NO_MULDIV64 in the
// original header.  The build configuration should retain that condition
// when exposing this item.
#[inline]
pub unsafe fn umul_ppmm(w1: *mut u32, w0: *mut u32, u: u32, v: u32) {
    let product = (u as u64).wrapping_mul(v as u64);
    let __w0 = product as u32;
    let __w1 = (product >> 32) as u32;

    // Corresponds to the original m68k instruction:
    // __asm__("mulu%.l %3,%1:%0" : "=d" (__w0), "=d" (__w1)
    //                              : "%0" (__u), "dmi" (__v));
    *w0 = __w0;
    *w1 = __w1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
