// SPDX-License-Identifier: GPL-2.0-only
/*
 * Test for find_*_bit functions.
 *
 * Copyright (c) 2017 Cavium.
 */

/*
 * find_bit functions are widely used in kernel, so the successful boot
 * is good enough test for correctness.
 *
 * This test is focused on performance of traversing bitmaps. Two typical
 * scenarios are reproduced:
 * - randomly filled bitmap with approximately equal number of set and
 *   cleared bits;
 * - sparse bitmap with few set bits at random positions.
 */

const BITMAP_LEN: usize = 4096 * 8 * 10;
const SPARSE: usize = 500;
const BITMAP_WORDS: usize = (BITMAP_LEN + usize::BITS as usize - 1) / usize::BITS as usize;

static mut BITMAP: [usize; BITMAP_WORDS] = [0; BITMAP_WORDS];
static mut BITMAP2: [usize; BITMAP_WORDS] = [0; BITMAP_WORDS];

type KtimeT = i64;

extern "C" {
    fn bitmap_copy(dst: *mut usize, src: *const usize, nbits: usize);
    fn ktime_get() -> KtimeT;
    fn find_first_bit(addr: *const usize, size: usize) -> usize;
    fn find_first_and_bit(addr1: *const usize, addr2: *const usize, size: usize) -> usize;
    fn find_next_bit(addr: *const usize, size: usize, offset: usize) -> usize;
    fn find_next_zero_bit(addr: *const usize, size: usize, offset: usize) -> usize;
    fn find_last_bit(addr: *const usize, size: usize) -> usize;
    fn find_nth_bit(addr: *const usize, size: usize, n: usize) -> usize;
    fn find_next_and_bit(addr1: *const usize, addr2: *const usize, size: usize, offset: usize) -> usize;
    fn bitmap_find_next_zero_area_off(addr: *mut usize, size: usize, start: usize, nr: usize, align_mask: usize, align_offset: usize) -> usize;
    fn bitmap_weight(addr: *const usize, nbits: usize) -> usize;
    fn get_random_bytes(buf: *mut core::ffi::c_void, nbytes: usize);
    fn get_random_u32_below(n: u32) -> u32;
    fn warn_on(condition: bool) -> bool;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

unsafe fn clear_bit(nr: usize, addr: *mut usize) {
    let word = nr / usize::BITS as usize;
    let bit = nr % usize::BITS as usize;
    *addr.add(word) &= !(1usize << bit);
}

unsafe fn set_bit(nr: usize, addr: *mut usize) {
    let word = nr / usize::BITS as usize;
    let bit = nr % usize::BITS as usize;
    *addr.add(word) |= 1usize << bit;
}

/*
 * This is Schlemiel the Painter's algorithm.
 */
unsafe fn test_find_first_bit(bitmap: *const usize, len: usize) -> i32 {
    static mut CP: [usize; BITMAP_WORDS] = [0; BITMAP_WORDS];
    let mut i: usize;
    let mut cnt: usize;
    let mut time: KtimeT;

    bitmap_copy(CP.as_mut_ptr(), bitmap, BITMAP_LEN);

    time = ktime_get();
    i = 0;
    cnt = 0;
    while i < len {
        i = find_first_bit(CP.as_ptr(), len);
        clear_bit(i, CP.as_mut_ptr());
        cnt += 1;
    }
    time = ktime_get() - time;
    pr_err(b"find_first_bit:     %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt);

    0
}

unsafe fn test_find_first_and_bit(bitmap: *const usize, bitmap2: *const usize, len: usize) -> i32 {
    static mut CP: [usize; BITMAP_WORDS] = [0; BITMAP_WORDS];
    let mut i: usize;
    let mut cnt: usize;
    let mut time: KtimeT;

    bitmap_copy(CP.as_mut_ptr(), bitmap, BITMAP_LEN);
    time = ktime_get();
    i = 0;
    cnt = 0;
    while i < len {
        i = find_first_and_bit(CP.as_ptr(), bitmap2, len);
        clear_bit(i, CP.as_mut_ptr());
        cnt += 1;
    }
    time = ktime_get() - time;
    pr_err(b"find_first_and_bit: %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt);
    0
}

unsafe fn test_find_next_bit(bitmap: *const usize, _len: usize) -> i32 {
    let mut i = 0usize;
    let mut cnt = 0usize;
    let mut time = ktime_get();
    while i < BITMAP_LEN { cnt += 1; i = find_next_bit(bitmap, BITMAP_LEN, i) + 1; }
    time = ktime_get() - time;
    pr_err(b"find_next_bit:      %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt);
    0
}

unsafe fn test_find_next_zero_bit(bitmap: *const usize, len: usize) -> i32 {
    let mut i = 0usize;
    let mut cnt = 0usize;
    let mut time = ktime_get();
    while i < BITMAP_LEN { cnt += 1; i = find_next_zero_bit(bitmap, len, i) + 1; }
    time = ktime_get() - time;
    pr_err(b"find_next_zero_bit: %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt);
    0
}

unsafe fn test_find_last_bit(bitmap: *const usize, mut len: usize) -> i32 {
    let mut cnt = 0usize;
    let mut time = ktime_get();
    loop { cnt += 1; let l = find_last_bit(bitmap, len); if l >= len { break; } len = l; if len == 0 { break; } }
    time = ktime_get() - time;
    pr_err(b"find_last_bit:      %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt);
    0
}

unsafe fn test_find_nth_bit(bitmap: *const usize, len: usize) -> i32 {
    let w = bitmap_weight(bitmap, len);
    let mut time = ktime_get();
    for n in 0..w { let l = find_nth_bit(bitmap, len, n); let _ = warn_on(l >= len); }
    time = ktime_get() - time;
    pr_err(b"find_nth_bit:       %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, w);
    0
}

unsafe fn test_find_next_and_bit(bitmap: *const usize, bitmap2: *const usize, _len: usize) -> i32 {
    let mut i = 0usize; let mut cnt = 0usize; let mut time = ktime_get();
    while i < BITMAP_LEN { cnt += 1; i = find_next_and_bit(bitmap, bitmap2, BITMAP_LEN, i + 1); }
    time = ktime_get() - time;
    pr_err(b"find_next_and_bit:  %18llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt); 0
}

unsafe fn test_bitmap_find_next_zero_area_off(bitmap: *mut usize, _len: usize) -> i32 {
    let mut i = 0usize; let mut cnt = 0usize; let mut time = ktime_get();
    while i < BITMAP_LEN { cnt += 1; i = bitmap_find_next_zero_area_off(bitmap, BITMAP_LEN, i, 8, 0, 0) + 1; }
    time = ktime_get() - time;
    pr_err(b"bitmap_find_next_zero_area_off:%7llu ns, %6ld iterations\0".as_ptr() as *const _, time, cnt); 0
}

unsafe fn find_bit_test() -> i32 {
    let mut nbits = BITMAP_LEN / SPARSE;
    pr_err(b"\nStart testing find_bit() with random-filled bitmap\0".as_ptr() as *const _);
    get_random_bytes(BITMAP.as_mut_ptr() as *mut _, core::mem::size_of_val(&BITMAP));
    get_random_bytes(BITMAP2.as_mut_ptr() as *mut _, core::mem::size_of_val(&BITMAP2));
    test_bitmap_find_next_zero_area_off(BITMAP.as_mut_ptr(), BITMAP_LEN);
    test_find_next_bit(BITMAP.as_ptr(), BITMAP_LEN); test_find_next_zero_bit(BITMAP.as_ptr(), BITMAP_LEN);
    test_find_last_bit(BITMAP.as_ptr(), BITMAP_LEN); test_find_nth_bit(BITMAP.as_ptr(), BITMAP_LEN / 10);
    test_find_first_bit(BITMAP.as_ptr(), BITMAP_LEN / 10); test_find_first_and_bit(BITMAP.as_ptr(), BITMAP2.as_ptr(), BITMAP_LEN / 2);
    test_find_next_and_bit(BITMAP.as_ptr(), BITMAP2.as_ptr(), BITMAP_LEN);
    pr_err(b"\nStart testing find_bit() with sparse bitmap\0".as_ptr() as *const _);
    for word in BITMAP.iter_mut() { *word = 0; } for word in BITMAP2.iter_mut() { *word = 0; }
    while nbits > 0 { nbits -= 1; set_bit(get_random_u32_below(BITMAP_LEN as u32) as usize, BITMAP.as_mut_ptr()); set_bit(get_random_u32_below(BITMAP_LEN as u32) as usize, BITMAP2.as_mut_ptr()); }
    test_bitmap_find_next_zero_area_off(BITMAP.as_mut_ptr(), BITMAP_LEN); test_find_next_bit(BITMAP.as_ptr(), BITMAP_LEN);
    test_find_next_zero_bit(BITMAP.as_ptr(), BITMAP_LEN); test_find_last_bit(BITMAP.as_ptr(), BITMAP_LEN);
    test_find_nth_bit(BITMAP.as_ptr(), BITMAP_LEN); test_find_first_bit(BITMAP.as_ptr(), BITMAP_LEN);
    test_find_first_and_bit(BITMAP.as_ptr(), BITMAP2.as_ptr(), BITMAP_LEN); test_find_next_and_bit(BITMAP.as_ptr(), BITMAP2.as_ptr(), BITMAP_LEN);
    -22
}

// module_init(find_bit_test);
// MODULE_DESCRIPTION("Test for find_*_bit functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
