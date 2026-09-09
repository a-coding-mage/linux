// SPDX-License-Identifier: GPL-2.0-only
/*
 * String functions optimized for hardware which doesn't
 * handle unaligned memory accesses efficiently.
 *
 * Copyright (C) 2021 Matteo Croce
 */

use core::ffi::c_void;

/* Minimum size for a word copy to be convenient */
const BYTES_LONG: usize = core::mem::size_of::<usize>();
const WORD_MASK: usize = BYTES_LONG - 1;
const MIN_THRESHOLD: usize = BYTES_LONG * 2;

/* convenience union to avoid cast between different pointer types */
#[repr(C)]
union Types {
    as_u8: *mut u8,
    as_ulong: *mut usize,
    as_uptr: usize,
}

#[repr(C)]
union ConstTypes {
    as_u8: *const u8,
    as_ulong: *const usize,
    as_uptr: usize,
}

pub unsafe fn memcpy(dest: *mut c_void, src: *const c_void, mut count: usize) -> *mut c_void {
    let mut s = ConstTypes { as_u8: src as *const u8 };
    let mut d = Types { as_u8: dest as *mut u8 };
    let mut distance: isize = 0;

    if count < MIN_THRESHOLD {
        return copy_remainder(dest, &mut d, &mut s, count);
    }

    /* Copy a byte at time until destination is aligned. */
    while (unsafe { d.as_uptr } & WORD_MASK) != 0 {
        unsafe { *d.as_u8 = *s.as_u8; d.as_u8 = d.as_u8.add(1); s.as_u8 = s.as_u8.add(1); }
        count -= 1;
    }

    distance = (unsafe { s.as_uptr } & WORD_MASK) as isize;

    if distance != 0 {
        let mut last: usize;
        let mut next: usize;

        /*
         * s is distance bytes ahead of d, and d just reached
         * the alignment boundary. Move s backward to word align it
         * and shift data to compensate for distance, in order to do
         * word-by-word copy.
         */
        unsafe { s.as_u8 = s.as_u8.offset(-distance); }

        next = unsafe { *s.as_ulong };
        while count >= BYTES_LONG {
            last = next;
            next = unsafe { *s.as_ulong.add(1) };

            unsafe {
                *d.as_ulong = (last >> ((distance as usize) * 8)) |
                    (next << ((BYTES_LONG - distance as usize) * 8));
                d.as_ulong = d.as_ulong.add(1);
                s.as_ulong = s.as_ulong.add(1);
            }
            count -= BYTES_LONG;
        }

        /* Restore s with the original offset. */
        unsafe { s.as_u8 = s.as_u8.offset(distance); }
    } else {
        /*
         * If the source and dest lower bits are the same, do a simple
         * 32/64 bit wide copy.
         */
        while count >= BYTES_LONG {
            unsafe { *d.as_ulong = *s.as_ulong; d.as_ulong = d.as_ulong.add(1); s.as_ulong = s.as_ulong.add(1); }
            count -= BYTES_LONG;
        }
    }

    copy_remainder(dest, &mut d, &mut s, count)
}

unsafe fn copy_remainder(
    dest: *mut c_void,
    d: &mut Types,
    s: &mut ConstTypes,
    mut count: usize,
) -> *mut c_void {
    while count != 0 {
        *d.as_u8 = *s.as_u8;
        d.as_u8 = d.as_u8.add(1);
        s.as_u8 = s.as_u8.add(1);
        count -= 1;
    }
    dest
}

/*
 * Simply check if the buffer overlaps an call memcpy() in case,
 * otherwise do a simple one byte at time backward copy.
 */
pub unsafe fn memmove(dest: *mut c_void, src: *const c_void, mut count: usize) -> *mut c_void {
    let d = dest as usize;
    let s = src as usize;
    if d < s || s.wrapping_add(count) <= d {
        return memcpy(dest, src, count);
    }

    if d > s {
        let mut source = (src as *const u8).add(count);
        let mut temporary = (dest as *mut u8).add(count);

        while count != 0 {
            temporary = temporary.sub(1);
            source = source.sub(1);
            *temporary = *source;
            count -= 1;
        }
    }
    dest
}

pub unsafe fn memset(s: *mut c_void, c: i32, mut count: usize) -> *mut c_void {
    let mut dest = Types { as_u8: s as *mut u8 };

    if count >= MIN_THRESHOLD {
        let mut cu = c as usize;

        /* Compose an ulong with 'c' repeated 4/8 times */
        cu |= cu << 8;
        cu |= cu << 16;
        /* Suppress warning on 32 bit machines */
        cu |= (cu << 16) << 16;

        while count != 0 && (unsafe { dest.as_uptr } & WORD_MASK) != 0 {
            unsafe { *dest.as_u8 = c as u8; dest.as_u8 = dest.as_u8.add(1); }
            count -= 1;
        }

        /* Copy using the largest size allowed */
        while count >= BYTES_LONG {
            unsafe { *dest.as_ulong = cu; dest.as_ulong = dest.as_ulong.add(1); }
            count -= BYTES_LONG;
        }
    }

    /* copy the remainder */
    while count != 0 {
        unsafe { *dest.as_u8 = c as u8; dest.as_u8 = dest.as_u8.add(1); }
        count -= 1;
    }

    s
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
