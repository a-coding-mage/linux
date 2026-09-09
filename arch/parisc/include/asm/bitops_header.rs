/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the Linux/compiler, asm, and generic bitops headers. */

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let word = BIT_WORD(nr) as isize;
    let p = addr.offset(word);
    _atomic_spin_lock_irqsave(p, &mut flags);
    *p |= mask;
    _atomic_spin_unlock_irqrestore(p, flags);
}

#[inline]
pub unsafe fn clear_bit(nr: i32, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let p = addr.offset(BIT_WORD(nr) as isize);
    _atomic_spin_lock_irqsave(p, &mut flags);
    *p &= !mask;
    _atomic_spin_unlock_irqrestore(p, flags);
}

#[inline]
pub unsafe fn change_bit(nr: i32, addr: *mut libc::c_ulong) {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let p = addr.offset(BIT_WORD(nr) as isize);
    _atomic_spin_lock_irqsave(p, &mut flags);
    *p ^= mask;
    _atomic_spin_unlock_irqrestore(p, flags);
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut libc::c_ulong) -> i32 {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let p = addr.offset(BIT_WORD(nr) as isize);
    _atomic_spin_lock_irqsave(p, &mut flags);
    let old = *p;
    let set = if old & mask != 0 { 1 } else { 0 };
    if set == 0 { *p = old | mask; }
    _atomic_spin_unlock_irqrestore(p, flags);
    set
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut libc::c_ulong) -> i32 {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let p = addr.offset(BIT_WORD(nr) as isize);
    _atomic_spin_lock_irqsave(p, &mut flags);
    let old = *p;
    let set = if old & mask != 0 { 1 } else { 0 };
    if set != 0 { *p = old & !mask; }
    _atomic_spin_unlock_irqrestore(p, flags);
    set
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut libc::c_ulong) -> i32 {
    let mask: libc::c_ulong = BIT_MASK(nr);
    let mut flags: libc::c_ulong = 0;
    let p = addr.offset(BIT_WORD(nr) as isize);
    _atomic_spin_lock_irqsave(p, &mut flags);
    let oldbit = *p;
    *p = oldbit ^ mask;
    _atomic_spin_unlock_irqrestore(p, flags);
    if oldbit & mask != 0 { 1 } else { 0 }
}

/* Generic non-atomic bit operations are supplied by asm-generic/bitops/non-atomic.h. */

/** Find first bit in word; undefined if no bit is set. */
#[inline]
pub const fn __ffs(x: libc::c_ulong) -> libc::c_ulong {
    x.trailing_zeros() as libc::c_ulong
}

/* Generic ffz operations are supplied by asm-generic/bitops/ffz.h. */

#[inline]
pub const fn ffs(x: i32) -> i32 {
    if x != 0 { (__ffs(x as libc::c_ulong) + 1) as i32 } else { 0 }
}

#[inline]
pub const fn fls(x: u32) -> i32 {
    if x == 0 { 0 } else { (32 - x.leading_zeros()) as i32 }
}

/* Generic __fls, fls64, hweight, lock, sched, le, and ext2 atomic-setbit
 * operations are supplied by their corresponding asm-generic headers.
 */

extern "C" {
    fn BIT_MASK(nr: i32) -> libc::c_ulong;
    fn BIT_WORD(nr: i32) -> libc::c_ulong;
    fn _atomic_spin_lock_irqsave(addr: *mut libc::c_ulong, flags: *mut libc::c_ulong);
    fn _atomic_spin_unlock_irqrestore(addr: *mut libc::c_ulong, flags: libc::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
