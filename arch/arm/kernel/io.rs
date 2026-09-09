// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    static mut __io_lock: raw_spinlock_t;

    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn readl_relaxed(reg: *const c_void) -> u32;
    fn writel_relaxed(value: u32, reg: *mut c_void);
    fn writel(value: u32, reg: *mut c_void);
    fn readb(from: *const u8) -> u8;
    fn writeb(value: u8, to: *mut u8);
}

type c_ulong = usize;

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

/*
 * Generic atomic MMIO modify.
 *
 * Allows thread-safe access to registers shared by unrelated subsystems.
 * The access is protected by a single MMIO-wide lock.
 */
pub unsafe fn atomic_io_modify_relaxed(reg: *mut c_void, mask: u32, set: u32) {
    let mut flags: c_ulong = 0;
    let mut value: u32;

    raw_spin_lock_irqsave(&mut __io_lock, &mut flags);
    value = readl_relaxed(reg as *const c_void) & !mask;
    value |= set & mask;
    writel_relaxed(value, reg);
    raw_spin_unlock_irqrestore(&mut __io_lock, flags);
}

pub unsafe fn atomic_io_modify(reg: *mut c_void, mask: u32, set: u32) {
    let mut flags: c_ulong = 0;
    let mut value: u32;

    raw_spin_lock_irqsave(&mut __io_lock, &mut flags);
    value = readl_relaxed(reg as *const c_void) & !mask;
    value |= set & mask;
    writel(value, reg);
    raw_spin_unlock_irqrestore(&mut __io_lock, flags);
}

/*
 * Copy data from IO memory space to "real" memory space.
 * This needs to be optimized.
 */
pub unsafe fn _memcpy_fromio(
    to: *mut c_void,
    mut from: *const u8,
    mut count: usize,
) {
    let mut t = to as *mut u8;
    while count != 0 {
        count -= 1;
        *t = readb(from);
        t = t.add(1);
        from = from.add(1);
    }
}

/*
 * Copy data from "real" memory space to IO memory space.
 * This needs to be optimized.
 */
pub unsafe fn _memcpy_toio(
    mut to: *mut u8,
    from: *const c_void,
    mut count: usize,
) {
    let mut f = from as *const u8;
    while count != 0 {
        count -= 1;
        writeb(*f, to);
        f = f.add(1);
        to = to.add(1);
    }
}

/*
 * "memset" on IO memory space.
 * This needs to be optimized.
 */
pub unsafe fn _memset_io(mut dst: *mut u8, c: i32, mut count: usize) {
    while count != 0 {
        count -= 1;
        writeb(c as u8, dst);
        dst = dst.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
