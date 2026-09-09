/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2025 Google LLC */

/*
 * Template translated from crc-clmul-template.h.  The includer must provide
 * crc_t, LSB_CRC, BITS_PER_LONG, crc_clmul_consts, and endian load helpers.
 */

const CRC_BITS: usize = 8 * core::mem::size_of::<crc_t>();

#[inline]
unsafe fn clmul(a: usize, b: usize) -> usize {
    let res: usize;
    core::arch::asm!(".option push\n.option arch,+zbc\nclmul {0}, {1}, {2}\n.option pop", out(reg) res, in(reg) a, in(reg) b);
    res
}

#[inline]
unsafe fn clmulh(a: usize, b: usize) -> usize {
    let res: usize;
    core::arch::asm!(".option push\n.option arch,+zbc\nclmulh {0}, {1}, {2}\n.option pop", out(reg) res, in(reg) a, in(reg) b);
    res
}

#[inline]
unsafe fn clmulr(a: usize, b: usize) -> usize {
    let res: usize;
    core::arch::asm!(".option push\n.option arch,+zbc\nclmulr {0}, {1}, {2}\n.option pop", out(reg) res, in(reg) a, in(reg) b);
    res
}

/* crc_load_long() loads one aligned unsigned long in the CRC bit order. */
#[inline]
unsafe fn crc_load_long(x: *const u8) -> usize {
    /* CONFIG_64BIT and LSB_CRC select the corresponding endian load. */
    let mut value = 0usize;
    for i in 0..core::mem::size_of::<usize>() {
        let byte = *x.add(i) as usize;
        if LSB_CRC { value |= byte << (8 * i); }
        else { value |= byte << (8 * (core::mem::size_of::<usize>() - 1 - i)); }
    }
    value
}

#[inline]
fn crc_clmul_prep(crc: crc_t, msgpoly: usize) -> usize {
    if LSB_CRC { msgpoly ^ crc as usize }
    else { msgpoly ^ ((crc as usize) << (usize::BITS as usize - CRC_BITS)) }
}

#[inline]
unsafe fn crc_clmul_long(msgpoly: usize, consts: *const crc_clmul_consts) -> crc_t {
    let tmp = if LSB_CRC {
        clmul(msgpoly, (*consts).barrett_reduction_const_1)
    } else {
        clmulr(msgpoly, (*consts).barrett_reduction_const_1)
    };
    let result = if LSB_CRC {
        clmulr(tmp, (*consts).barrett_reduction_const_2)
    } else {
        clmul(tmp, (*consts).barrett_reduction_const_2)
    };
    result as crc_t
}

#[inline]
unsafe fn crc_clmul_update_long(crc: crc_t, msgpoly: usize, consts: *const crc_clmul_consts) -> crc_t {
    crc_clmul_long(crc_clmul_prep(crc, msgpoly), consts)
}

#[inline]
unsafe fn crc_clmul_update_partial(crc: crc_t, p: *const u8, len: usize, consts: *const crc_clmul_consts) -> crc_t {
    let mut msgpoly: usize;
    if LSB_CRC {
        msgpoly = (*p as usize) << (usize::BITS as usize - 8);
        for i in 1..len { msgpoly = (msgpoly >> 8) ^ ((*p.add(i) as usize) << (usize::BITS as usize - 8)); }
    } else {
        msgpoly = *p as usize;
        for i in 1..len { msgpoly = (msgpoly << 8) ^ *p.add(i) as usize; }
    }
    if len >= core::mem::size_of::<crc_t>() {
        if LSB_CRC { msgpoly ^= (crc as usize) << (usize::BITS as usize - 8 * len); }
        else { msgpoly ^= (crc as usize) << (8 * len - CRC_BITS); }
        return crc_clmul_long(msgpoly, consts);
    }
    if LSB_CRC {
        msgpoly ^= (crc as usize) << (usize::BITS as usize - 8 * len);
        crc_clmul_long(msgpoly, consts) ^ ((crc as usize >> (8 * len)) as crc_t)
    } else {
        msgpoly ^= (crc as usize) >> (CRC_BITS - 8 * len);
        crc_clmul_long(msgpoly, consts) ^ ((crc as usize << (8 * len)) as crc_t)
    }
}

#[inline]
unsafe fn crc_clmul(mut crc: crc_t, mut p: *const u8, mut len: usize, consts: *const crc_clmul_consts) -> crc_t {
    /* This implementation assumes that the CRC fits in an unsigned long. */
    let mut align = (p as usize) % core::mem::size_of::<usize>();
    if align != 0 && len != 0 {
        align = core::cmp::min(core::mem::size_of::<usize>() - align, len);
        crc = crc_clmul_update_partial(crc, p, align, consts);
        p = p.add(align); len -= align;
    }
    if len >= 4 * core::mem::size_of::<usize>() {
        let mut m0 = crc_clmul_prep(crc, crc_load_long(p));
        let mut m1 = crc_load_long(p.add(core::mem::size_of::<usize>()));
        p = p.add(2 * core::mem::size_of::<usize>()); len -= 2 * core::mem::size_of::<usize>();
        loop {
            let p0 = clmulh(m0, (*consts).fold_across_2_longs_const_hi);
            let p1 = clmul(m0, (*consts).fold_across_2_longs_const_hi);
            let p2 = clmulh(m1, (*consts).fold_across_2_longs_const_lo);
            let p3 = clmul(m1, (*consts).fold_across_2_longs_const_lo);
            m0 = if LSB_CRC { p1 ^ p3 } else { p0 ^ p2 } ^ crc_load_long(p);
            m1 = if LSB_CRC { p0 ^ p2 } else { p1 ^ p3 } ^ crc_load_long(p.add(core::mem::size_of::<usize>()));
            p = p.add(2 * core::mem::size_of::<usize>()); len -= 2 * core::mem::size_of::<usize>();
            if len < 2 * core::mem::size_of::<usize>() { break; }
        }
        crc = crc_clmul_long(m0, consts); crc = crc_clmul_update_long(crc, m1, consts);
    }
    while len >= core::mem::size_of::<usize>() { crc = crc_clmul_update_long(crc, crc_load_long(p), consts); p = p.add(core::mem::size_of::<usize>()); len -= core::mem::size_of::<usize>(); }
    if len != 0 { crc = crc_clmul_update_partial(crc, p, len, consts); }
    crc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
