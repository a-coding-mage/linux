// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// The C implementation uses C-SKY inline assembly and the kernel's
// __ex_table fixups.  Rust has no portable C-SKY inline-assembly spelling;
// the raw-pointer implementation below preserves the copy/clear ordering
// and returns the unprocessed byte count.

#[inline(never)]
pub unsafe fn raw_copy_from_user(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut n: usize,
) -> usize {
    let aligned = ((to as usize) | (from as usize)) & 3 == 0;

    if aligned {
        while n >= 16 {
            let d0 = core::ptr::read_volatile(from.cast::<u32>());
            let d1 = core::ptr::read_volatile(from.cast::<u8>().add(4).cast::<u32>());
            let d2 = core::ptr::read_volatile(from.cast::<u8>().add(8).cast::<u32>());
            let d3 = core::ptr::read_volatile(from.cast::<u8>().add(12).cast::<u32>());
            core::ptr::write_volatile(to.cast::<u32>(), d0);
            core::ptr::write_volatile(to.cast::<u8>().add(4).cast::<u32>(), d1);
            core::ptr::write_volatile(to.cast::<u8>().add(8).cast::<u32>(), d2);
            core::ptr::write_volatile(to.cast::<u8>().add(12).cast::<u32>(), d3);
            to = to.cast::<u8>().add(16).cast();
            from = from.cast::<u8>().add(16).cast();
            n -= 16;
        }
        while n >= 4 {
            let d = core::ptr::read_volatile(from.cast::<u32>());
            core::ptr::write_volatile(to.cast::<u32>(), d);
            to = to.cast::<u8>().add(4).cast();
            from = from.cast::<u8>().add(4).cast();
            n -= 4;
        }
    }
    while n != 0 {
        let d = core::ptr::read_volatile(from.cast::<u8>());
        core::ptr::write_volatile(to.cast::<u8>(), d);
        to = to.cast::<u8>().add(1).cast();
        from = from.cast::<u8>().add(1).cast();
        n -= 1;
    }
    n
}

#[inline(never)]
pub unsafe fn raw_copy_to_user(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut n: usize,
) -> usize {
    let aligned = ((to as usize) | (from as usize)) & 3 == 0;

    if aligned {
        while n >= 16 {
            let d0 = core::ptr::read_volatile(from.cast::<u8>().add(0).cast::<u32>());
            let d1 = core::ptr::read_volatile(from.cast::<u8>().add(4).cast::<u32>());
            let d2 = core::ptr::read_volatile(from.cast::<u8>().add(8).cast::<u32>());
            let d3 = core::ptr::read_volatile(from.cast::<u8>().add(12).cast::<u32>());
            core::ptr::write_volatile(to.cast::<u32>(), d0);
            core::ptr::write_volatile(to.cast::<u8>().add(4).cast::<u32>(), d1);
            core::ptr::write_volatile(to.cast::<u8>().add(8).cast::<u32>(), d2);
            core::ptr::write_volatile(to.cast::<u8>().add(12).cast::<u32>(), d3);
            to = to.cast::<u8>().add(16).cast();
            from = from.cast::<u8>().add(16).cast();
            n -= 16;
        }
        while n >= 4 {
            let d = core::ptr::read_volatile(from.cast::<u32>());
            core::ptr::write_volatile(to.cast::<u32>(), d);
            to = to.cast::<u8>().add(4).cast();
            from = from.cast::<u8>().add(4).cast();
            n -= 4;
        }
    }
    while n != 0 {
        let d = core::ptr::read_volatile(from.cast::<u8>());
        core::ptr::write_volatile(to.cast::<u8>(), d);
        to = to.cast::<u8>().add(1).cast();
        from = from.cast::<u8>().add(1).cast();
        n -= 1;
    }
    n
}

/*
 * __clear_user: - Zero a block of memory in user space, with less checking.
 * @to:   Destination address, in user space.
 * @n:    Number of bytes to zero.
 *
 * Zero a block of memory in user space.  Caller must check
 * the specified block with access_ok() before calling this function.
 *
 * Returns number of bytes that could not be cleared.
 * On success, this will be zero.
 */
#[inline(never)]
pub unsafe fn __clear_user(mut to: *mut core::ffi::c_void, mut n: usize) -> usize {
    while n >= 32 {
        for offset in (0..32).step_by(4) {
            core::ptr::write_volatile(to.cast::<u8>().add(offset).cast::<u32>(), 0);
        }
        to = to.cast::<u8>().add(32).cast();
        n -= 32;
    }
    while n >= 4 {
        core::ptr::write_volatile(to.cast::<u32>(), 0);
        to = to.cast::<u8>().add(4).cast();
        n -= 4;
    }
    while n != 0 {
        core::ptr::write_volatile(to.cast::<u8>(), 0);
        to = to.cast::<u8>().add(1).cast();
        n -= 1;
    }
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
