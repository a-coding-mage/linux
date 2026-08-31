// SPDX-License-Identifier: GPL-2.0

// C header guard omitted in Rust.

#[cfg(target_endian = "little")]
pub unsafe fn load_byte<T>(src: *const T, b: usize, _s: usize) -> u64 {
    let ptr = src as *const u8;
    let value = unsafe { core::ptr::read_volatile(ptr.add(b)) };

    (value as u64) << (8 * b)
}

#[cfg(target_endian = "little")]
pub unsafe fn load_word<T>(src: *const T, w: usize, _s: usize) -> u64 {
    let ptr = src as *const u16;
    let value = unsafe { core::ptr::read_volatile(ptr.add(w)) };

    (value as u64) << (16 * w)
}

#[cfg(target_endian = "big")]
pub unsafe fn load_byte<T>(src: *const T, b: usize, s: usize) -> u64 {
    let ptr = src as *const u8;
    let index = b + (core::mem::size_of::<T>() - s);
    let value = unsafe { core::ptr::read_volatile(ptr.add(index)) };

    (value as u64) << (8 * (s - b - 1))
}

#[cfg(target_endian = "big")]
pub unsafe fn load_word<T>(src: *const T, w: usize, s: usize) -> u64 {
    let ptr = src as *const u16;
    let value = unsafe { core::ptr::read_volatile(ptr.add(w)) };

    (value as u64) << (16 * ((s / 2) - w - 1))
}

#[cfg(not(any(target_endian = "little", target_endian = "big")))]
compile_error!("Fix your compiler's __BYTE_ORDER__?!");
