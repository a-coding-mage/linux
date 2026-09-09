/* SPDX-License-Identifier: GPL-2.0-only */

// Linux kernel headers supplied by other translation units.

pub fn do_fortify_tests();

pub const __BUF_SMALL: usize = 16;
pub const __BUF_LARGE: usize = 32;

#[repr(C)]
pub struct fortify_object {
    pub a: i32,
    pub buf: [i8; __BUF_SMALL],
    pub c: i32,
}

pub const LITERAL_SMALL: &[u8; 16] = b"AAAAAAAAAAAAAAA\0";
pub const LITERAL_LARGE: &[u8; 32] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\0";

pub static small_src: [i8; __BUF_SMALL] = [
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, 0,
];
pub static large_src: [i8; __BUF_LARGE] = [
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, b'A' as i8,
    b'A' as i8, b'A' as i8, b'A' as i8, 0,
];

pub static mut small: [i8; __BUF_SMALL] = [0; __BUF_SMALL];
pub static mut large: [i8; __BUF_LARGE] = [0; __BUF_LARGE];
pub static mut instance: fortify_object = fortify_object {
    a: 0,
    buf: [0; __BUF_SMALL],
    c: 0,
};
pub static mut size: usize = 0;

pub unsafe fn do_fortify_tests() {
    /* Normal initializations. */
    core::ptr::write_bytes(
        &raw mut instance as *mut fortify_object as *mut u8,
        0x32,
        core::mem::size_of::<fortify_object>(),
    );
    core::ptr::write_bytes(small.as_mut_ptr() as *mut u8, 0xA5, core::mem::size_of_val(&small));
    core::ptr::write_bytes(large.as_mut_ptr() as *mut u8, 0x5A, core::mem::size_of_val(&large));

    // TEST; is an external macro supplied by the including translation unit.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
