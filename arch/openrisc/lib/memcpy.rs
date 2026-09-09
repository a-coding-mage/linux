// SPDX-License-Identifier: GPL-2.0
/*
 * arch/openrisc/lib/memcpy.c
 *
 * Optimized memory copy routines for openrisc.  These are mostly copied
 * from ohter sources but slightly entended based on ideas discuassed in
 * #openrisc.
 *
 * The word unroll implementation is an extension to the arm byte
 * unrolled implementation, but using word copies (if things are
 * properly aligned)
 *
 * The great arm loop unroll algorithm can be found at:
 *  arch/arm/boot/compressed/string.c
 */

// CONFIG_OR1K_1200 selects the loop-unrolled implementation.
#[cfg(CONFIG_OR1K_1200)]
pub unsafe fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, mut n: usize) -> *mut core::ffi::c_void {
    let mut i: usize = 0;
    let mut d: *mut u8;
    let mut s: *const u8;
    let mut dest_w = dest as *mut u32;
    let mut src_w = src as *const u32;

    /* If both source and dest are word aligned copy words */
    if (dest_w as usize & 3) == 0 && (src_w as usize & 3) == 0 {
        /* Copy 32 bytes per loop */
        for i_ref in (0..(n >> 5)).rev() {
            i = i_ref;
            let _ = i;
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
        }

        if n & (1 << 4) != 0 {
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
        }
        if n & (1 << 3) != 0 {
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
        }
        if n & (1 << 2) != 0 {
            *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1);
        }
        d = dest_w as *mut u8;
        s = src_w as *const u8;
    } else {
        d = dest_w as *mut u8;
        s = src_w as *const u8;
        for _ in 0..(n >> 3) {
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
        }
        if n & (1 << 2) != 0 {
            for _ in 0..4 { *d = *s; d = d.add(1); s = s.add(1); }
        }
    }
    if n & (1 << 1) != 0 { *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1); }
    if n & 1 != 0 { *d = *s; }
    dest
}

#[cfg(not(CONFIG_OR1K_1200))]
pub unsafe fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, mut n: usize) -> *mut core::ffi::c_void {
    let mut dest_w = dest as *mut u32;
    let mut src_w = src as *const u32;
    if (dest_w as usize & 3) == 0 && (src_w as usize & 3) == 0 {
        while n >= 4 { *dest_w = *src_w; dest_w = dest_w.add(1); src_w = src_w.add(1); n -= 4; }
    }
    let mut d = dest_w as *mut u8;
    let mut s = src_w as *const u8;
    while n >= 1 { *d = *s; d = d.add(1); s = s.add(1); n -= 1; }
    dest
}

// EXPORT_SYMBOL(memcpy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
