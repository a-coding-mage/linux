/*
 * Robert Jenkin's hash function.
 * https://burtleburtle.net/bob/hash/evahash.html
 * This is in the public domain.
 */

#[inline]
fn mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 13;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= *a << 8;
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 13;
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 12;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= *a << 16;
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 5;
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 3;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= *a << 10;
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 15;
}

pub unsafe fn ceph_str_hash_rjenkins(str_: *const i8, length: u32) -> u32 {
    let mut k = str_ as *const u8;
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut len: u32;

    len = length;
    a = 0x9e3779b9;
    b = a;
    c = 0;

    while len >= 12 {
        a = a.wrapping_add((*k.add(0) as u32)
            .wrapping_add((*k.add(1) as u32) << 8)
            .wrapping_add((*k.add(2) as u32) << 16)
            .wrapping_add((*k.add(3) as u32) << 24));
        b = b.wrapping_add((*k.add(4) as u32)
            .wrapping_add((*k.add(5) as u32) << 8)
            .wrapping_add((*k.add(6) as u32) << 16)
            .wrapping_add((*k.add(7) as u32) << 24));
        c = c.wrapping_add((*k.add(8) as u32)
            .wrapping_add((*k.add(9) as u32) << 8)
            .wrapping_add((*k.add(10) as u32) << 16)
            .wrapping_add((*k.add(11) as u32) << 24));
        mix(&mut a, &mut b, &mut c);
        k = k.add(12);
        len -= 12;
    }

    c = c.wrapping_add(length);
    match len {
        11 => { c = c.wrapping_add((*k.add(10) as u32) << 24); c = c.wrapping_add((*k.add(9) as u32) << 16); c = c.wrapping_add((*k.add(8) as u32) << 8); b = b.wrapping_add((*k.add(7) as u32) << 24); b = b.wrapping_add((*k.add(6) as u32) << 16); b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        10 => { c = c.wrapping_add((*k.add(9) as u32) << 16); c = c.wrapping_add((*k.add(8) as u32) << 8); b = b.wrapping_add((*k.add(7) as u32) << 24); b = b.wrapping_add((*k.add(6) as u32) << 16); b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        9 => { c = c.wrapping_add((*k.add(8) as u32) << 8); b = b.wrapping_add((*k.add(7) as u32) << 24); b = b.wrapping_add((*k.add(6) as u32) << 16); b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        8 => { b = b.wrapping_add((*k.add(7) as u32) << 24); b = b.wrapping_add((*k.add(6) as u32) << 16); b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        7 => { b = b.wrapping_add((*k.add(6) as u32) << 16); b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        6 => { b = b.wrapping_add((*k.add(5) as u32) << 8); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        5 => { b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        4 => { a = a.wrapping_add((*k.add(3) as u32) << 24); a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        3 => { a = a.wrapping_add((*k.add(2) as u32) << 16); a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        2 => { a = a.wrapping_add((*k.add(1) as u32) << 8); a = a.wrapping_add(*k.add(0) as u32); }
        1 => { a = a.wrapping_add(*k.add(0) as u32); }
        _ => {}
    }
    mix(&mut a, &mut b, &mut c);
    c
}

pub unsafe fn ceph_str_hash_linux(mut str_: *const i8, mut length: u32) -> u32 {
    let mut hash: usize = 0;
    while length != 0 {
        let c = *str_ as u8 as usize;
        str_ = str_.add(1);
        hash = hash.wrapping_add(c << 4).wrapping_add(c >> 4).wrapping_mul(11);
        length -= 1;
    }
    hash as u32
}

pub unsafe fn ceph_str_hash(type_: i32, s: *const i8, len: u32) -> u32 {
    if type_ == CEPH_STR_HASH_LINUX {
        ceph_str_hash_linux(s, len)
    } else if type_ == CEPH_STR_HASH_RJENKINS {
        ceph_str_hash_rjenkins(s, len)
    } else {
        u32::MAX
    }
}

pub unsafe fn ceph_str_hash_name(type_: i32) -> *const i8 {
    if type_ == CEPH_STR_HASH_LINUX {
        b"linux\0".as_ptr() as *const i8
    } else if type_ == CEPH_STR_HASH_RJENKINS {
        b"rjenkins\0".as_ptr() as *const i8
    } else {
        b"unknown\0".as_ptr() as *const i8
    }
}

// CEPH_STR_HASH_LINUX and CEPH_STR_HASH_RJENKINS are supplied by the Ceph headers.
extern "C" {
    static CEPH_STR_HASH_LINUX: i32;
    static CEPH_STR_HASH_RJENKINS: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
