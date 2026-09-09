/* SPDX-License-Identifier: GPL-2.0 */

// The asm-generic non-atomic bitops dependency is supplied by the surrounding build.

#[inline]
pub unsafe fn __bo_cas(p: *mut u32, old: u32, mut new: u32) -> u32 {
    // SH-specific inline assembly, translated directly from the source.
    core::arch::asm!(
        "cas.l {old}, {new}, @r0",
        old = in(reg) old,
        new = inout(reg) new,
        in("r0") p,
        options(nostack)
    );
    new
}

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old | mask) == old {
            break;
        }
    }
}

#[inline]
pub unsafe fn clear_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old & !mask) == old {
            break;
        }
    }
}

#[inline]
pub unsafe fn change_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old ^ mask) == old {
            break;
        }
    }
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old | mask) == old {
            break;
        }
    }

    ((old & mask) != 0) as i32
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old & !mask) == old {
            break;
        }
    }

    ((old & mask) != 0) as i32
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let mask: u32;
    let mut old: u32;
    let a = addr as *mut u32;
    let a = a.offset((nr >> 5) as isize);
    mask = 1u32 << ((nr & 0x1f) as u32);

    loop {
        old = core::ptr::read_volatile(a);
        if __bo_cas(a, old, old ^ mask) == old {
            break;
        }
    }

    ((old & mask) != 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
