/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1994-1997, 99, 2000, 06, 07 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (c) 1999, 2000  Silicon Graphics, Inc.
 */

use core::ffi::c_ulong;

// Supplied by the architecture's IRQ flags implementation.
unsafe extern "C" {
    fn raw_local_irq_save(flags: *mut c_ulong);
    fn raw_local_irq_restore(flags: c_ulong);
}

// These values are supplied by the target architecture/build configuration.
const BITS_PER_LONG: usize = c_ulong::BITS as usize;

#[inline]
unsafe fn bit_word(nr: c_ulong) -> usize {
    (nr as usize) / BITS_PER_LONG
}

/**
 * __mips_set_bit - Atomically set a bit in memory.  This is called by
 * set_bit() if it cannot find a faster solution.
 * @nr: the bit to set
 * @addr: the address to start counting from
 */
pub unsafe fn __mips_set_bit(nr: c_ulong, addr: *mut c_ulong) {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    a.write_volatile(a.read_volatile() | mask);
    raw_local_irq_restore(flags);
}

/**
 * __mips_clear_bit - Clears a bit in memory.  This is called by clear_bit() if
 * it cannot find a faster solution.
 * @nr: Bit to clear
 * @addr: Address to start counting from
 */
pub unsafe fn __mips_clear_bit(nr: c_ulong, addr: *mut c_ulong) {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    a.write_volatile(a.read_volatile() & !mask);
    raw_local_irq_restore(flags);
}

/**
 * __mips_change_bit - Toggle a bit in memory.  This is called by change_bit()
 * if it cannot find a faster solution.
 * @nr: Bit to change
 * @addr: Address to start counting from
 */
pub unsafe fn __mips_change_bit(nr: c_ulong, addr: *mut c_ulong) {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    a.write_volatile(a.read_volatile() ^ mask);
    raw_local_irq_restore(flags);
}

/**
 * __mips_test_and_set_bit_lock - Set a bit and return its old value.  This is
 * called by test_and_set_bit_lock() if it cannot find a faster solution.
 * @nr: Bit to set
 * @addr: Address to count from
 */
pub unsafe fn __mips_test_and_set_bit_lock(nr: c_ulong, addr: *mut c_ulong) -> i32 {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    let res = (mask & a.read_volatile()) != 0;
    a.write_volatile(a.read_volatile() | mask);
    raw_local_irq_restore(flags);
    res as i32
}

/**
 * __mips_test_and_clear_bit - Clear a bit and return its old value.  This is
 * called by test_and_clear_bit() if it cannot find a faster solution.
 * @nr: Bit to clear
 * @addr: Address to count from
 */
pub unsafe fn __mips_test_and_clear_bit(nr: c_ulong, addr: *mut c_ulong) -> i32 {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    let res = (mask & a.read_volatile()) != 0;
    a.write_volatile(a.read_volatile() & !mask);
    raw_local_irq_restore(flags);
    res as i32
}

/**
 * __mips_test_and_change_bit - Change a bit and return its old value.  This is
 * called by test_and_change_bit() if it cannot find a faster solution.
 * @nr: Bit to change
 * @addr: Address to count from
 */
pub unsafe fn __mips_test_and_change_bit(nr: c_ulong, addr: *mut c_ulong) -> i32 {
    let a = addr.add(bit_word(nr));
    let bit = nr % (BITS_PER_LONG as c_ulong);
    let mask = 1 as c_ulong << bit;
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    let res = (mask & a.read_volatile()) != 0;
    a.write_volatile(a.read_volatile() ^ mask);
    raw_local_irq_restore(flags);
    res as i32
}

pub unsafe fn __mips_xor_is_negative_byte(mask: c_ulong, addr: *mut c_ulong) -> bool {
    let mut flags = 0 as c_ulong;
    raw_local_irq_save(&mut flags);
    let data = addr.read_volatile();
    addr.write_volatile(data ^ mask);
    raw_local_irq_restore(flags);
    (data & (1 as c_ulong << 7)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
