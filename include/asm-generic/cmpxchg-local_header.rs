/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn wrong_size_cmpxchg(ptr: *mut core::ffi::c_void) -> !;
    fn raw_local_irq_save(flags: *mut usize);
    fn raw_local_irq_restore(flags: usize);
}

/*
 * Generic version of __cmpxchg_local (disables interrupts). Takes an unsigned
 * long parameter, supporting various types of architectures.
 */
#[inline]
pub unsafe fn __generic_cmpxchg_local(
    ptr: *mut core::ffi::c_void,
    old: usize,
    new: usize,
    size: core::ffi::c_int,
) -> usize {
    let mut flags: usize = 0;
    let prev: usize;

    /*
     * Sanity checking, compile-time.
     */
    if size == 8 && core::mem::size_of::<usize>() != 8 {
        wrong_size_cmpxchg(ptr);
    }

    raw_local_irq_save(&mut flags as *mut usize);
    match size {
        1 => {
            prev = *(ptr as *mut u8) as usize;
            if prev == (old & 0xffu) {
                *(ptr as *mut u8) = (new & 0xffu) as u8;
            }
        }
        2 => {
            prev = *(ptr as *mut u16) as usize;
            if prev == (old & 0xffffu) {
                *(ptr as *mut u16) = (new & 0xffffu) as u16;
            }
        }
        4 => {
            prev = *(ptr as *mut u32) as usize;
            if prev == (old & 0xffffffffu) {
                *(ptr as *mut u32) = (new & 0xffffffffu) as u32;
            }
        }
        8 => {
            prev = *(ptr as *mut u64) as usize;
            if prev == old {
                *(ptr as *mut u64) = new as u64;
            }
        }
        _ => {
            wrong_size_cmpxchg(ptr);
        }
    }
    raw_local_irq_restore(flags);
    prev
}

/*
 * Generic version of __cmpxchg64_local. Takes an u64 parameter.
 */
#[inline]
pub unsafe fn __generic_cmpxchg64_local(
    ptr: *mut core::ffi::c_void,
    old: u64,
    new: u64,
) -> u64 {
    let mut flags: usize = 0;
    let prev: u64;

    raw_local_irq_save(&mut flags as *mut usize);
    prev = *(ptr as *mut u64);
    if prev == old {
        *(ptr as *mut u64) = new;
    }
    raw_local_irq_restore(flags);
    prev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
