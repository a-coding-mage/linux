/* jhash.h: Jenkins hash support.
 *
 * Copyright (C) 2006. Bob Jenkins (bob_jenkins@burtleburtle.net)
 *
 * https://burtleburtle.net/bob/hash/
 *
 * These are the credits from Bob's sources:
 *
 * lookup3.c, by Bob Jenkins, May 2006, Public Domain.
 *
 * These are functions for producing 32-bit hashes for hash table lookup.
 * hashword(), hashlittle(), hashlittle2(), hashbig(), mix(), and final()
 * are externally useful functions.  Routines to test the hash are included
 * if SELF_TEST is defined.  You can use this free for any purpose.  It's in
 * the public domain.  It has no warranty.
 *
 * Copyright (C) 2009-2010 Jozsef Kadlecsik (kadlec@blackhole.kfki.hu)
 *
 * I've modified Bob's hash to be useful in the Linux kernel, and
 * any bugs present are my fault.
 * Jozsef
 */

/* Dependencies in the C header:
 * #include <linux/bitops.h>
 * #include <linux/unaligned/packed_struct.h>
 */

/* Best hash sizes are of power of two */
#[inline]
pub const fn jhash_size(n: u32) -> u32 {
    (1u32).wrapping_shl(n)
}

/* Mask the hash value, i.e (value & jhash_mask(n)) instead of (value % n) */
#[inline]
pub const fn jhash_mask(n: u32) -> u32 {
    jhash_size(n).wrapping_sub(1)
}

/* __jhash_mix -- mix 3 32-bit values reversibly. */
#[inline]
fn __jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = (*a).wrapping_sub(*c);
    *a ^= (*c).rotate_left(4);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= (*a).rotate_left(6);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= (*b).rotate_left(8);
    *b = (*b).wrapping_add(*a);
    *a = (*a).wrapping_sub(*c);
    *a ^= (*c).rotate_left(16);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= (*a).rotate_left(19);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= (*b).rotate_left(4);
    *b = (*b).wrapping_add(*a);
}

/* __jhash_final - final mixing of 3 32-bit values (a,b,c) into c */
#[inline]
fn __jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b;
    *c = (*c).wrapping_sub((*b).rotate_left(14));
    *a ^= *c;
    *a = (*a).wrapping_sub((*c).rotate_left(11));
    *b ^= *a;
    *b = (*b).wrapping_sub((*a).rotate_left(25));
    *c ^= *b;
    *c = (*c).wrapping_sub((*b).rotate_left(16));
    *a ^= *c;
    *a = (*a).wrapping_sub((*c).rotate_left(4));
    *b ^= *a;
    *b = (*b).wrapping_sub((*a).rotate_left(14));
    *c ^= *b;
    *c = (*c).wrapping_sub((*b).rotate_left(24));
}

/* An arbitrary initial parameter */
pub const JHASH_INITVAL: u32 = 0xdeadbeef;

#[inline]
unsafe fn __get_unaligned_cpu32(p: *const u8) -> u32 {
    unsafe { core::ptr::read_unaligned(p as *const u32) }
}

/* jhash - hash an arbitrary key
 * @k: sequence of bytes as key
 * @length: the length of the key
 * @initval: the previous hash, or an arbitray value
 *
 * The generic version, hashes an arbitrary sequence of bytes.
 * No alignment or length assumptions are made about the input key.
 *
 * Returns the hash value of the key. The result depends on endianness.
 */
#[inline]
pub unsafe fn jhash(key: *const core::ffi::c_void, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut k = key as *const u8;

    /* Set up the internal state */
    c = JHASH_INITVAL.wrapping_add(length).wrapping_add(initval);
    b = c;
    a = b;

    /* All but the last block: affect some 32 bits of (a,b,c) */
    while length > 12 {
        a = a.wrapping_add(unsafe { __get_unaligned_cpu32(k) });
        b = b.wrapping_add(unsafe { __get_unaligned_cpu32(unsafe { k.add(4) }) });
        c = c.wrapping_add(unsafe { __get_unaligned_cpu32(unsafe { k.add(8) }) });
        __jhash_mix(&mut a, &mut b, &mut c);
        length = length.wrapping_sub(12);
        k = unsafe { k.add(12) };
    }
    /* Last block: affect all 32 bits of (c) */
    /* All the case statements fall through */
    match length {
        12 => {
            c = c.wrapping_add((unsafe { *k.add(11) } as u32) << 24);
            c = c.wrapping_add((unsafe { *k.add(10) } as u32) << 16);
            c = c.wrapping_add((unsafe { *k.add(9) } as u32) << 8);
            c = c.wrapping_add(unsafe { *k.add(8) } as u32);
            b = b.wrapping_add((unsafe { *k.add(7) } as u32) << 24);
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        11 => {
            c = c.wrapping_add((unsafe { *k.add(10) } as u32) << 16);
            c = c.wrapping_add((unsafe { *k.add(9) } as u32) << 8);
            c = c.wrapping_add(unsafe { *k.add(8) } as u32);
            b = b.wrapping_add((unsafe { *k.add(7) } as u32) << 24);
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        10 => {
            c = c.wrapping_add((unsafe { *k.add(9) } as u32) << 8);
            c = c.wrapping_add(unsafe { *k.add(8) } as u32);
            b = b.wrapping_add((unsafe { *k.add(7) } as u32) << 24);
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        9 => {
            c = c.wrapping_add(unsafe { *k.add(8) } as u32);
            b = b.wrapping_add((unsafe { *k.add(7) } as u32) << 24);
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        8 => {
            b = b.wrapping_add((unsafe { *k.add(7) } as u32) << 24);
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        7 => {
            b = b.wrapping_add((unsafe { *k.add(6) } as u32) << 16);
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        6 => {
            b = b.wrapping_add((unsafe { *k.add(5) } as u32) << 8);
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        5 => {
            b = b.wrapping_add(unsafe { *k.add(4) } as u32);
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        4 => {
            a = a.wrapping_add((unsafe { *k.add(3) } as u32) << 24);
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        3 => {
            a = a.wrapping_add((unsafe { *k.add(2) } as u32) << 16);
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        2 => {
            a = a.wrapping_add((unsafe { *k.add(1) } as u32) << 8);
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        1 => {
            a = a.wrapping_add(unsafe { *k.add(0) } as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        0 => {
            /* Nothing left to add */
        }
        _ => {}
    }

    c
}

/* jhash2 - hash an array of u32's
 * @k: the key which must be an array of u32's
 * @length: the number of u32's in the key
 * @initval: the previous hash, or an arbitray value
 *
 * Returns the hash value of the key.
 */
#[inline]
pub unsafe fn jhash2(mut k: *const u32, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    /* Set up the internal state */
    c = JHASH_INITVAL
        .wrapping_add(length.wrapping_shl(2))
        .wrapping_add(initval);
    b = c;
    a = b;

    /* Handle most of the key */
    while length > 3 {
        a = a.wrapping_add(unsafe { *k.add(0) });
        b = b.wrapping_add(unsafe { *k.add(1) });
        c = c.wrapping_add(unsafe { *k.add(2) });
        __jhash_mix(&mut a, &mut b, &mut c);
        length = length.wrapping_sub(3);
        k = unsafe { k.add(3) };
    }

    /* Handle the last 3 u32's: all the case statements fall through */
    match length {
        3 => {
            c = c.wrapping_add(unsafe { *k.add(2) });
            b = b.wrapping_add(unsafe { *k.add(1) });
            a = a.wrapping_add(unsafe { *k.add(0) });
            __jhash_final(&mut a, &mut b, &mut c);
        }
        2 => {
            b = b.wrapping_add(unsafe { *k.add(1) });
            a = a.wrapping_add(unsafe { *k.add(0) });
            __jhash_final(&mut a, &mut b, &mut c);
        }
        1 => {
            a = a.wrapping_add(unsafe { *k.add(0) });
            __jhash_final(&mut a, &mut b, &mut c);
        }
        0 => {
            /* Nothing left to add */
        }
        _ => {}
    }

    c
}

/* __jhash_nwords - hash exactly 3, 2 or 1 word(s) */
#[inline]
pub fn __jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);

    __jhash_final(&mut a, &mut b, &mut c);

    c
}

#[inline]
pub fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32 {
    __jhash_nwords(
        a,
        b,
        c,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(3 << 2),
    )
}

#[inline]
pub fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 {
    __jhash_nwords(
        a,
        b,
        0,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2),
    )
}

#[inline]
pub fn jhash_1word(a: u32, initval: u32) -> u32 {
    __jhash_nwords(
        a,
        0,
        0,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(1 << 2),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
