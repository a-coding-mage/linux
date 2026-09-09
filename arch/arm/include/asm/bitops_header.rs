/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM bit operations header translated from C.
 * bit 0 is the LSB of an "unsigned long" quantity.
 * This code is intended for kernel use only.
 */

/* Dependencies supplied by the surrounding kernel translation:
 * BIT_MASK, BIT_WORD, raw_local_irq_save, raw_local_irq_restore, and the
 * generic bit-operation declarations/items.
 */

#[inline]
pub unsafe fn ____atomic_set_bit(bit: u32, mut p: *mut usize) {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let value = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, value | mask);
    raw_local_irq_restore(flags);
}

#[inline]
pub unsafe fn ____atomic_clear_bit(bit: u32, mut p: *mut usize) {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let value = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, value & !mask);
    raw_local_irq_restore(flags);
}

#[inline]
pub unsafe fn ____atomic_change_bit(bit: u32, mut p: *mut usize) {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let value = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, value ^ mask);
    raw_local_irq_restore(flags);
}

#[inline]
pub unsafe fn ____atomic_test_and_set_bit(bit: u32, mut p: *mut usize) -> i32 {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let res = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, res | mask);
    raw_local_irq_restore(flags);
    if res & mask != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn ____atomic_test_and_clear_bit(bit: u32, mut p: *mut usize) -> i32 {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let res = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, res & !mask);
    raw_local_irq_restore(flags);
    if res & mask != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn ____atomic_test_and_change_bit(bit: u32, mut p: *mut usize) -> i32 {
    let mut flags: usize = 0;
    let mask: usize = BIT_MASK(bit);
    p = p.add(BIT_WORD(bit));
    raw_local_irq_save(&mut flags);
    let res = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, res ^ mask);
    raw_local_irq_restore(flags);
    if res & mask != 0 { 1 } else { 0 }
}

extern "C" {
    pub fn _set_bit(nr: i32, p: *mut usize);
    pub fn _clear_bit(nr: i32, p: *mut usize);
    pub fn _change_bit(nr: i32, p: *mut usize);
    pub fn _test_and_set_bit(nr: i32, p: *mut usize) -> i32;
    pub fn _test_and_clear_bit(nr: i32, p: *mut usize) -> i32;
    pub fn _test_and_change_bit(nr: i32, p: *mut usize) -> i32;

    pub fn _find_first_zero_bit_le(p: *const usize, size: usize) -> usize;
    pub fn _find_next_zero_bit_le(p: *const usize, size: usize, offset: usize) -> usize;
    pub fn _find_first_bit_le(p: *const usize, size: usize) -> usize;
    pub fn _find_next_bit_le(p: *const usize, size: usize, offset: usize) -> usize;
    pub fn _find_first_zero_bit_be(p: *const usize, size: usize) -> usize;
    pub fn _find_next_zero_bit_be(p: *const usize, size: usize, offset: usize) -> usize;
    pub fn _find_first_bit_be(p: *const usize, size: usize) -> usize;
    pub fn _find_next_bit_be(p: *const usize, size: usize, offset: usize) -> usize;
}

/* The C ATOMIC_BITOP macro selects the inline operation for constant bit
 * numbers and the assembly operation otherwise; CONFIG_SMP selects the
 * assembly operation unconditionally.  The build system supplies that
 * condition. */

#[macro_export]
macro_rules! set_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_set_bit($nr as i32, $p) } }; }
#[macro_export]
macro_rules! clear_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_clear_bit($nr as i32, $p) } }; }
#[macro_export]
macro_rules! change_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_change_bit($nr as i32, $p) } }; }
#[macro_export]
macro_rules! test_and_set_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_test_and_set_bit($nr as i32, $p) } }; }
#[macro_export]
macro_rules! test_and_clear_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_test_and_clear_bit($nr as i32, $p) } }; }
#[macro_export]
macro_rules! test_and_change_bit { ($nr:expr, $p:expr) => { unsafe { $crate::_test_and_change_bit($nr as i32, $p) } }; }

/* __ARMEB__ selects the big-endian variants; otherwise the little-endian
 * variants are selected.  The default macro definitions below represent the
 * non-__ARMEB__ branch; a target configuration may replace them with the
 * corresponding *_be functions. */
#[macro_export]
macro_rules! find_first_zero_bit { ($p:expr, $sz:expr) => { unsafe { $crate::_find_first_zero_bit_le($p, $sz) } }; }
#[macro_export]
macro_rules! find_next_zero_bit { ($p:expr, $sz:expr, $off:expr) => { unsafe { $crate::_find_next_zero_bit_le($p, $sz, $off) } }; }
#[macro_export]
macro_rules! find_first_bit { ($p:expr, $sz:expr) => { unsafe { $crate::_find_first_bit_le($p, $sz) } }; }
#[macro_export]
macro_rules! find_next_bit { ($p:expr, $sz:expr, $off:expr) => { unsafe { $crate::_find_next_bit_le($p, $sz, $off) } }; }

/* Generic bitops headers provide the remaining declarations (including
 * non-atomic operations, ffz, fls, ffs, fls64, sched, hweight, lock, le,
 * and ext2-atomic-setbit).  The __LINUX_BITOPS_H and __KERNEL__ header
 * conditions from the source are build-time conditions not expressible from
 * this file alone. */

#[inline]
pub unsafe fn find_first_zero_bit_le(p: *const core::ffi::c_void, size: u32) -> i32 {
    _find_first_zero_bit_le(p as *const usize, size as usize) as i32
}

#[inline]
pub unsafe fn find_next_zero_bit_le(p: *const core::ffi::c_void, size: i32, offset: i32) -> i32 {
    _find_next_zero_bit_le(p as *const usize, size as usize, offset as usize) as i32
}

#[inline]
pub unsafe fn find_next_bit_le(p: *const core::ffi::c_void, size: i32, offset: i32) -> i32 {
    _find_next_bit_le(p as *const usize, size as usize, offset as usize) as i32
}

/* In the C header these helpers and their self-aliasing macros exist only
 * when __ARMEB__ is defined. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
