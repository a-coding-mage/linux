/* SPDX-License-Identifier: GPL-2.0-only */

// Generic definitions for bit operations, should not be used in regular code
// directly.
//
// The original header depends on <linux/bits.h> and <asm/barrier.h>, and is
// intended to be included through <linux/bitops.h>.

#[inline(always)]
unsafe fn generic___set_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);

    *p |= mask;
}

#[inline(always)]
unsafe fn generic___clear_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);

    *p &= !mask;
}

#[inline(always)]
unsafe fn generic___change_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);

    *p ^= mask;
}

#[inline(always)]
unsafe fn generic___test_and_set_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) -> bool {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);
    let old = *p;

    *p = old | mask;
    (old & mask) != 0
}

#[inline(always)]
unsafe fn generic___test_and_clear_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) -> bool {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);
    let old = *p;

    *p = old & !mask;
    (old & mask) != 0
}

// WARNING: non atomic and it can be reordered!
#[inline(always)]
unsafe fn generic___test_and_change_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) -> bool {
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let p = addr.add((nr / BITS_PER_LONG) as usize);
    let old = *p;

    *p = old ^ mask;
    (old & mask) != 0
}

#[inline(always)]
unsafe fn generic_test_bit(nr: libc::c_ulong, addr: *const libc::c_ulong) -> bool {
    /*
     * Unlike the bitops with the '__' prefix above, this one *is* atomic,
     * so `volatile` must always stay here with no cast-aways. See
     * `Documentation/atomic_bitops.txt` for the details.
     */
    1 as libc::c_ulong & (*addr.add((nr / BITS_PER_LONG) as usize) >> (nr & (BITS_PER_LONG - 1))) != 0
}

#[inline(always)]
unsafe fn generic_test_bit_acquire(nr: libc::c_ulong, addr: *const libc::c_ulong) -> bool {
    let p = addr.add((nr / BITS_PER_LONG) as usize);
    1 as libc::c_ulong & smp_load_acquire(p) >> (nr & (BITS_PER_LONG - 1)) != 0
}

// const_*() definitions provide good compile-time optimizations when the
// passed arguments can be resolved at compile time.
#[inline(always)]
unsafe fn const_test_bit(nr: libc::c_ulong, addr: *const libc::c_ulong) -> bool {
    let p = addr.add((nr / BITS_PER_LONG) as usize);
    let mask: libc::c_ulong = 1 as libc::c_ulong << (nr & (BITS_PER_LONG - 1));
    let val = *p;

    (val & mask) != 0
}

// The following C macro aliases are represented by the corresponding Rust
// functions: const___set_bit = generic___set_bit,
// const___clear_bit = generic___clear_bit,
// const___change_bit = generic___change_bit,
// const___test_and_set_bit = generic___test_and_set_bit,
// const___test_and_clear_bit = generic___test_and_clear_bit,
// const___test_and_change_bit = generic___test_and_change_bit,
// const_test_bit_acquire = generic_test_bit_acquire.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
