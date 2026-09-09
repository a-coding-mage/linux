/* jhash.h: Jenkins hash support.
 *
 * Translated from the Linux kernel header. The original implementation is
 * public domain, with Linux kernel modifications by Jozsef Kadlecsik.
 */

/* Best hash sizes are of power of two. */
#[inline]
pub const fn jhash_size(n: u32) -> u32 {
    1u32.wrapping_shl(n)
}

/* Mask the hash value, i.e. (value & jhash_mask(n)) instead of (value % n). */
#[inline]
pub const fn jhash_mask(n: u32) -> u32 {
    jhash_size(n).wrapping_sub(1)
}

#[inline]
fn __jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c); *a ^= c.rotate_left(4); *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= a.rotate_left(6); *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= b.rotate_left(8); *b = b.wrapping_add(*a);
    *a = a.wrapping_sub(*c); *a ^= c.rotate_left(16); *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= a.rotate_left(19); *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= b.rotate_left(4); *b = b.wrapping_add(*a);
}

#[inline]
fn __jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(14));
    *a ^= *c; *a = a.wrapping_sub(c.rotate_left(11));
    *b ^= *a; *b = b.wrapping_sub(a.rotate_left(25));
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(16));
    *a ^= *c; *a = a.wrapping_sub(c.rotate_left(4));
    *b ^= *a; *b = b.wrapping_sub(a.rotate_left(14));
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(24));
}

/* An arbitrary initial parameter. */
pub const JHASH_INITVAL: u32 = 0xdeadbeef;

#[inline]
pub unsafe fn jhash(key: *const core::ffi::c_void, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut k = key as *const u8;

    a = JHASH_INITVAL.wrapping_add(length).wrapping_add(initval);
    b = a;
    c = a;

    while length > 12 {
        a = a.wrapping_add(core::ptr::read_unaligned(k as *const u32));
        b = b.wrapping_add(core::ptr::read_unaligned(k.add(4) as *const u32));
        c = c.wrapping_add(core::ptr::read_unaligned(k.add(8) as *const u32));
        __jhash_mix(&mut a, &mut b, &mut c);
        length -= 12;
        k = k.add(12);
    }

    match length {
        12 => { c = c.wrapping_add(((*k.add(11)) as u32) << 24); c = c.wrapping_add(((*k.add(10)) as u32) << 16); c = c.wrapping_add(((*k.add(9)) as u32) << 8); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7)) as u32) << 24); b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        11 => { c = c.wrapping_add(((*k.add(10)) as u32) << 16); c = c.wrapping_add(((*k.add(9)) as u32) << 8); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7)) as u32) << 24); b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        10 => { c = c.wrapping_add(((*k.add(9)) as u32) << 8); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7)) as u32) << 24); b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        9 => { c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7)) as u32) << 24); b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        8 => { b = b.wrapping_add(((*k.add(7)) as u32) << 24); b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        7 => { b = b.wrapping_add(((*k.add(6)) as u32) << 16); b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        6 => { b = b.wrapping_add(((*k.add(5)) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        5 => { b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        4 => { a = a.wrapping_add(((*k.add(3)) as u32) << 24); a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        3 => { a = a.wrapping_add(((*k.add(2)) as u32) << 16); a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        2 => { a = a.wrapping_add(((*k.add(1)) as u32) << 8); a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        1 => { a = a.wrapping_add(*k as u32); __jhash_final(&mut a, &mut b, &mut c); }
        0 => {}
        _ => unreachable!(),
    }
    c
}

#[inline]
pub unsafe fn jhash2(mut k: *const u32, mut length: u32, initval: u32) -> u32 {
    let mut a = JHASH_INITVAL.wrapping_add(length << 2).wrapping_add(initval);
    let mut b = a;
    let mut c = a;
    while length > 3 { a = a.wrapping_add(*k); b = b.wrapping_add(*k.add(1)); c = c.wrapping_add(*k.add(2)); __jhash_mix(&mut a, &mut b, &mut c); length -= 3; k = k.add(3); }
    match length { 3 => { c = c.wrapping_add(*k.add(2)); b = b.wrapping_add(*k.add(1)); a = a.wrapping_add(*k); __jhash_final(&mut a, &mut b, &mut c); }, 2 => { b = b.wrapping_add(*k.add(1)); a = a.wrapping_add(*k); __jhash_final(&mut a, &mut b, &mut c); }, 1 => { a = a.wrapping_add(*k); __jhash_final(&mut a, &mut b, &mut c); }, 0 => {}, _ => unreachable!() }
    c
}

#[inline]
fn __jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval); b = b.wrapping_add(initval); c = c.wrapping_add(initval); __jhash_final(&mut a, &mut b, &mut c); c
}

#[inline] pub fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32 { __jhash_nwords(a, b, c, initval.wrapping_add(JHASH_INITVAL).wrapping_add(3 << 2)) }
#[inline] pub fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 { __jhash_nwords(a, b, 0, initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2)) }
#[inline] pub fn jhash_1word(a: u32, initval: u32) -> u32 { __jhash_nwords(a, 0, 0, initval.wrapping_add(JHASH_INITVAL).wrapping_add(1 << 2)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
