// SPDX-License-Identifier: GPL-2.0-only
/* tnum: tracked (or tristate) numbers
 *
 * A tnum tracks knowledge about the bits of a value.  Each bit can be either
 * known (0 or 1), or unknown (x).  Arithmetic operations on tnums will
 * propagate the unknown bits such that the tnum result represents all the
 * possible results for possible values of the operands.
 */

#[repr(C)]
pub struct tnum {
    pub value: u64,
    pub mask: u64,
}

unsafe extern "C" {
    fn fls64(x: u64) -> u8;
}

const fn tnum_make(value: u64, mask: u64) -> tnum {
    tnum { value, mask }
}

/* A completely unknown value */
pub static tnum_unknown: tnum = tnum_make(0, u64::MAX);

pub fn tnum_const(value: u64) -> tnum {
    tnum_make(value, 0)
}

pub fn tnum_range(min: u64, max: u64) -> tnum {
    let chi = min ^ max;
    let bits: u8 = unsafe { fls64(chi) };

    /* special case, needed because 1ULL << 64 is undefined */
    if bits > 63 {
        return tnum_unknown;
    }
    /* e.g. if chi = 4, bits = 3, delta = (1<<3) - 1 = 7.
     * if chi = 0, bits = 0, delta = (1<<0) - 1 = 0, so we return
     *  constant min (since min == max).
     */
    let delta = (1u64 << bits) - 1;
    tnum_make(min & !delta, delta)
}

pub fn tnum_lshift(a: tnum, shift: u8) -> tnum {
    tnum_make(a.value.wrapping_shl(shift as u32), a.mask.wrapping_shl(shift as u32))
}

pub fn tnum_rshift(a: tnum, shift: u8) -> tnum {
    tnum_make(a.value.wrapping_shr(shift as u32), a.mask.wrapping_shr(shift as u32))
}

pub fn tnum_arshift(a: tnum, min_shift: u8, insn_bitness: u8) -> tnum {
    /* if a.value is negative, arithmetic shifting by minimum shift
     * will have larger negative offset compared to more shifting.
     * If a.value is nonnegative, arithmetic shifting by minimum shift
     * will have larger positive offset compare to more shifting.
     */
    if insn_bitness == 32 {
        tnum_make(
            ((a.value as u32 as i32) >> min_shift) as u32 as u64,
            ((a.mask as u32 as i32) >> min_shift) as u32 as u64,
        )
    } else {
        tnum_make((a.value as i64 >> min_shift) as u64, (a.mask as i64 >> min_shift) as u64)
    }
}

pub fn tnum_add(a: tnum, b: tnum) -> tnum {
    let sm = a.mask.wrapping_add(b.mask);
    let sv = a.value.wrapping_add(b.value);
    let sigma = sm.wrapping_add(sv);
    let chi = sigma ^ sv;
    let mu = chi | a.mask | b.mask;
    tnum_make(sv & !mu, mu)
}

pub fn tnum_sub(a: tnum, b: tnum) -> tnum {
    let dv = a.value.wrapping_sub(b.value);
    let alpha = dv.wrapping_add(a.mask);
    let beta = dv.wrapping_sub(b.mask);
    let chi = alpha ^ beta;
    let mu = chi | a.mask | b.mask;
    tnum_make(dv & !mu, mu)
}

pub fn tnum_neg(a: tnum) -> tnum {
    tnum_sub(tnum_make(0, 0), a)
}

pub fn tnum_and(a: tnum, b: tnum) -> tnum {
    let alpha = a.value | a.mask;
    let beta = b.value | b.mask;
    let v = a.value & b.value;
    tnum_make(v, alpha & beta & !v)
}

pub fn tnum_or(a: tnum, b: tnum) -> tnum {
    let v = a.value | b.value;
    let mu = a.mask | b.mask;
    tnum_make(v, mu & !v)
}

pub fn tnum_xor(a: tnum, b: tnum) -> tnum {
    let v = a.value ^ b.value;
    let mu = a.mask | b.mask;
    tnum_make(v & !mu, mu)
}

/* Perform long multiplication, iterating through the bits in a using rshift:
 * - if LSB(a) is a known 0, keep current accumulator
 * - if LSB(a) is a known 1, add b to current accumulator
 * - if LSB(a) is unknown, take a union of the above cases.
 */
pub fn tnum_mul(mut a: tnum, mut b: tnum) -> tnum {
    let mut acc = tnum_make(0, 0);
    while a.value != 0 || a.mask != 0 {
        if a.value & 1 != 0 {
            acc = tnum_add(acc, b);
        } else if a.mask & 1 != 0 {
            acc = tnum_union(acc, tnum_add(acc, b));
        }
        a = tnum_rshift(a, 1);
        b = tnum_lshift(b, 1);
    }
    acc
}

pub fn tnum_overlap(a: tnum, b: tnum) -> bool {
    let mu = !a.mask & !b.mask;
    (a.value & mu) == (b.value & mu)
}

/* Note that if a and b disagree - i.e. one has a 'known 1' where the other has
 * a 'known 0' - this will return a 'known 1' for that bit.
 */
pub fn tnum_intersect(a: tnum, b: tnum) -> tnum {
    let v = a.value | b.value;
    let mu = a.mask & b.mask;
    tnum_make(v & !mu, mu)
}

/* Returns a tnum with the uncertainty from both a and b, and in addition, new
 * uncertainty at any position that a and b disagree. This represents a
 * superset of the union of the concrete sets of both a and b. Despite the
 * overapproximation, it is optimal.
 */
pub fn tnum_union(a: tnum, b: tnum) -> tnum {
    let v = a.value & b.value;
    let mu = (a.value ^ b.value) | a.mask | b.mask;
    tnum_make(v & !mu, mu)
}

pub fn tnum_cast(mut a: tnum, size: u8) -> tnum {
    let bits = (size as u32) * 8;
    let mask = (1u64 << bits).wrapping_sub(1);
    a.value &= mask;
    a.mask &= mask;
    a
}

pub fn tnum_is_aligned(a: tnum, size: u64) -> bool {
    if size == 0 { return true; }
    ((a.value | a.mask) & (size - 1)) == 0
}

pub fn tnum_in(mut a: tnum, mut b: tnum) -> bool {
    if b.mask & !a.mask != 0 { return false; }
    b.value &= !a.mask;
    a.value == b.value
}

pub unsafe fn tnum_sbin(str_: *mut u8, size: usize, mut a: tnum) -> i32 {
    let mut n = 64usize;
    while n != 0 {
        if n < size {
            *str_.add(n - 1) = if a.mask & 1 != 0 { b'x' } else if a.value & 1 != 0 { b'1' } else { b'0' };
        }
        a.mask >>= 1;
        a.value >>= 1;
        n -= 1;
    }
    *str_.add(core::cmp::min(size - 1, 64)) = 0;
    64
}

pub fn tnum_subreg(a: tnum) -> tnum { tnum_cast(a, 4) }

pub fn tnum_clear_subreg(a: tnum) -> tnum { tnum_lshift(tnum_rshift(a, 32), 32) }

pub fn tnum_with_subreg(reg: tnum, subreg: tnum) -> tnum {
    tnum_or(tnum_clear_subreg(reg), tnum_subreg(subreg))
}

pub fn tnum_const_subreg(a: tnum, value: u32) -> tnum {
    tnum_with_subreg(a, tnum_const(value as u64))
}

pub fn tnum_bswap16(a: tnum) -> tnum {
    tnum_make((a.value & 0xFFFF).swap_bytes() >> 48, (a.mask & 0xFFFF).swap_bytes() >> 48)
}

pub fn tnum_bswap32(a: tnum) -> tnum {
    tnum_make((a.value & 0xFFFFFFFF).swap_bytes() >> 32, (a.mask & 0xFFFFFFFF).swap_bytes() >> 32)
}

pub fn tnum_bswap64(a: tnum) -> tnum {
    tnum_make(a.value.swap_bytes(), a.mask.swap_bytes())
}

pub fn tnum_step(t: tnum, z: u64) -> u64 {
    let tmax = t.value | t.mask;
    if z >= tmax { return tmax; }
    if z < t.value { return t.value; }
    let d = z - t.value;
    let carry_mask = (1u64 << unsafe { fls64(d & !t.mask) }).wrapping_sub(1);
    let filled = d | carry_mask | !t.mask;
    let inc = filled.wrapping_add(1) & t.mask;
    t.value | inc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
