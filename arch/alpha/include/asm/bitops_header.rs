/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Alpha Linux bit operations header.
// The original uses Alpha load-locked/store-conditional inline assembly.

use core::ptr;

#[inline]
pub unsafe fn set_bit(nr: usize, addr: *mut core::ffi::c_void) {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 31);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old | mask);
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___set_bit(nr: usize, addr: *mut usize) {
    let m = addr as *mut i32;
    let m = m.add(nr >> 5);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old | (1i32 << (nr & 31)));
}

#[inline]
pub unsafe fn clear_bit(nr: usize, addr: *mut core::ffi::c_void) {
    let m = (addr as *mut i32).add(nr >> 5);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old & !(1i32 << (nr & 31)));
}

#[inline]
pub unsafe fn clear_bit_unlock(nr: usize, addr: *mut core::ffi::c_void) {
    // smp_mb();
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    clear_bit(nr, addr);
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___clear_bit(nr: usize, addr: *mut usize) {
    let m = (addr as *mut i32).add(nr >> 5);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old & !(1i32 << (nr & 31)));
}

#[inline]
pub unsafe fn __clear_bit_unlock(nr: usize, addr: *mut core::ffi::c_void) {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    arch___clear_bit(nr, addr as *mut usize);
}

#[inline]
pub unsafe fn change_bit(nr: usize, addr: *mut core::ffi::c_void) {
    let m = (addr as *mut i32).add(nr >> 5);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old ^ (1i32 << (nr & 31)));
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___change_bit(nr: usize, addr: *mut usize) {
    let m = (addr as *mut i32).add(nr >> 5);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old ^ (1i32 << (nr & 31)));
}

#[inline]
pub unsafe fn test_and_set_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 31);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old | mask);
    (old & mask != 0) as i32
}

#[inline]
pub unsafe fn test_and_set_bit_lock(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    test_and_set_bit(nr, addr)
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___test_and_set_bit(nr: usize, addr: *mut usize) -> bool {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 0x1f);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old | mask);
    old & mask != 0
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 31);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old & !mask);
    (old & mask != 0) as i32
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___test_and_clear_bit(nr: usize, addr: *mut usize) -> bool {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 0x1f);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old & !mask);
    old & mask != 0
}

#[inline]
pub unsafe fn test_and_change_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32 {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 31);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old ^ mask);
    (old & mask != 0) as i32
}

/// WARNING: non atomic version.
#[inline(always)]
pub unsafe fn arch___test_and_change_bit(nr: usize, addr: *mut usize) -> bool {
    let m = (addr as *mut i32).add(nr >> 5);
    let mask = 1i32 << (nr & 0x1f);
    let old = ptr::read_volatile(m);
    ptr::write_volatile(m, old ^ mask);
    old & mask != 0
}

// #define arch_test_bit generic_test_bit
// #define arch_test_bit_acquire generic_test_bit_acquire

#[inline]
pub unsafe fn xor_unlock_is_negative_byte(mask: usize, p: *mut usize) -> bool {
    let old = ptr::read_volatile(p);
    ptr::write_volatile(p, old ^ mask);
    old & (1usize << 7) != 0
}

#[inline]
pub fn ffz_b(mut x: usize) -> usize {
    x = (!x) & (!x).wrapping_neg();
    let x1 = x & 0xAA;
    let x2 = x & 0xCC;
    let x4 = x & 0xF0;
    (if x2 != 0 { 2 } else { 0 }) + (if x4 != 0 { 4 } else { 0 }) + (x1 != 0) as usize
}

// __kernel_cttz, __kernel_cmpbge, and __kernel_extbl are supplied by Alpha dependencies.
extern "C" {
    fn __kernel_cttz(x: usize) -> usize;
    fn __kernel_cmpbge(a: usize, b: usize) -> usize;
    fn __kernel_extbl(a: usize, b: usize) -> usize;
}

#[inline]
pub unsafe fn ffz(word: usize) -> usize {
    let bits = __kernel_cmpbge(word, !0usize);
    let qofs = ffz_b(bits);
    let bits = __kernel_extbl(word, qofs);
    qofs * 8 + ffz_b(bits)
}

#[inline]
pub unsafe fn __ffs(word: usize) -> usize {
    let bits = __kernel_cmpbge(0, word);
    let qofs = ffz_b(bits);
    let bits = __kernel_extbl(word, qofs);
    qofs * 8 + ffz_b(!bits)
}

#[inline]
pub unsafe fn ffs(word: i32) -> i32 {
    if word != 0 { (__ffs(word as usize) + 1) as i32 } else { 0 }
}

extern "C" {
    static __flsm1_tab: [u8; 256];
}

#[inline]
pub unsafe fn fls64(x: usize) -> i32 {
    let t = __kernel_cmpbge(x, 0x0101010101010101usize);
    let a = __flsm1_tab[t];
    let t = __kernel_extbl(x, a as usize);
    (a as usize * 8 + __flsm1_tab[t] as usize + (x != 0) as usize) as i32
}

#[inline]
pub unsafe fn __fls(x: usize) -> usize { (fls64(x) - 1) as usize }

#[inline]
pub unsafe fn fls(x: u32) -> i32 { fls64(x as usize) }

#[inline]
pub unsafe fn sched_find_first_bit(b: &[usize; 2]) -> usize {
    let b0 = b[0];
    let b1 = b[1];
    let ofs = if b0 != 0 { 0 } else { 64 };
    __ffs(if b0 != 0 { b0 } else { b1 }) + ofs
}

// asm-generic bitops (arch_hweight, const_hweight, non-instrumented, le, and
// ext2-atomic-setbit) are supplied by their respective dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
