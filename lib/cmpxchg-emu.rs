// SPDX-License-Identifier: GPL-2.0+
/*
 * Emulated 1-byte cmpxchg operation for architectures lacking direct
 * support for this size.  This is implemented in terms of 4-byte cmpxchg
 * operations.
 *
 * Copyright (C) 2024 Paul E. McKenney.
 */

#[repr(C)]
pub union U8_32 {
    pub b: [u8; 4],
    pub w: u32,
}

extern "C" {
    fn instrument_atomic_read_write(p: *mut u8, size: usize);
    fn cmpxchg(p: *mut u32, old: u32, new: u32) -> u32;
}

/* Emulate one-byte cmpxchg() in terms of 4-byte cmpxchg. */
#[no_mangle]
pub unsafe extern "C" fn cmpxchg_emu_u8(
    p: *mut u8,
    old: usize,
    new: usize,
) -> usize {
    let p32 = (p as usize & !0x3) as *mut u32;
    let i = p as usize & 0x3;
    let mut old32: U8_32;
    let mut new32: U8_32;
    let mut ret: u32;

    // READ_ONCE(*p32)
    ret = core::ptr::read_volatile(p32);
    loop {
        old32 = U8_32 { w: ret };
        if (*old32.b.get_unchecked(i) as usize) != old {
            return *old32.b.get_unchecked(i) as usize;
        }
        new32 = U8_32 { w: old32.w };
        *new32.b.get_unchecked_mut(i) = new as u8;
        instrument_atomic_read_write(p, 1);
        // data_race(cmpxchg(p32, old32.w, new32.w)); // Overridden above.
        ret = cmpxchg(p32, old32.w, new32.w);
        if ret == old32.w {
            break;
        }
    }
    old
}

// EXPORT_SYMBOL_GPL(cmpxchg_emu_u8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
