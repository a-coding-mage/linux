// Generated from testing/selftests/bpf/libarena/selftests/test_bitmap.bpf.c
// Dependencies from:
// #include <libarena/common.h>
// #include <libarena/asan.h>
// #include <libarena/bitmap.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = u32;
type size_t = usize;

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

extern "C" {
    static zero: u32;
    static can_loop: bool;
    static BITS_PER_LONG_LONG: size_t;

    fn BITS_TO_LONG_LONGS(nr: size_t) -> size_t;
    fn BIT_MASK(nr: size_t) -> u64;

    fn bmp_alloc(nbits: size_t) -> *mut arena_bitmap;
    fn bmp_free(bmp: *mut arena_bitmap);
    fn bmp_empty(nbits: size_t, bmp: *mut arena_bitmap) -> bool;
    fn __bmp_set_bit(nr: size_t, bmp: *mut arena_bitmap);
    fn __bmp_clear_bit(nr: size_t, bmp: *mut arena_bitmap);
    fn bmp_test_bit(nr: size_t, bmp: *mut arena_bitmap) -> bool;
    fn bmp_test_and_clear_bit(nr: size_t, bmp: *mut arena_bitmap) -> bool;
    fn bmp_test_and_set_bit(nr: size_t, bmp: *mut arena_bitmap) -> bool;
    fn bmp_and(
        nbits: size_t,
        dst: *mut arena_bitmap,
        src1: *mut arena_bitmap,
        src2: *mut arena_bitmap,
    );
    fn bmp_or(
        nbits: size_t,
        dst: *mut arena_bitmap,
        src1: *mut arena_bitmap,
        src2: *mut arena_bitmap,
    );
    fn bmp_subset(nbits: size_t, src1: *mut arena_bitmap, src2: *mut arena_bitmap) -> bool;
    fn bmp_intersects(nbits: size_t, src1: *mut arena_bitmap, src2: *mut arena_bitmap) -> bool;
    fn bmp_copy(nbits: size_t, dst: *mut arena_bitmap, src: *mut arena_bitmap);
}

#[repr(C)]
pub struct arena_bitmap {
    pub bits: [u64; 0],
}

unsafe fn TEST_BITS() -> size_t {
    2usize.wrapping_mul(BITS_PER_LONG_LONG)
}

unsafe fn TEST_WORDS() -> size_t {
    BITS_TO_LONG_LONGS(TEST_BITS())
}

unsafe fn MID_BIT() -> size_t {
    BITS_PER_LONG_LONG.wrapping_add(1)
}

unsafe fn LAST_BIT() -> size_t {
    TEST_BITS().wrapping_sub(1)
}

unsafe fn test_bmp_setall(bmp: *mut arena_bitmap) {
    let mut i: u32 = core::ptr::read_volatile(&zero);

    while (i as size_t) < TEST_WORDS() && core::ptr::read_volatile(&can_loop) {
        let bits = (*bmp).bits.as_mut_ptr();
        core::ptr::write(bits.add(i as size_t), !0u64);
        i = i.wrapping_add(1);
    }
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_alloc_free() -> i32 {
    let bmp: *mut arena_bitmap;

    bmp = bmp_alloc(TEST_BITS());
    if bmp.is_null() {
        return -ENOMEM;
    }

    if !bmp_empty(TEST_BITS(), bmp) {
        goto_err_alloc_free(bmp);
        return -EINVAL;
    }

    __bmp_set_bit(LAST_BIT(), bmp);
    if !bmp_test_bit(LAST_BIT(), bmp) {
        goto_err_alloc_free(bmp);
        return -EINVAL;
    }

    __bmp_clear_bit(LAST_BIT(), bmp);
    if bmp_test_bit(LAST_BIT(), bmp) {
        goto_err_alloc_free(bmp);
        return -EINVAL;
    }

    bmp_free(bmp);
    0
}

unsafe fn goto_err_alloc_free(bmp: *mut arena_bitmap) {
    bmp_free(bmp);
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_bit_ops() -> i32 {
    let bmp: *mut arena_bitmap;

    bmp = bmp_alloc(TEST_BITS());
    if bmp.is_null() {
        return -ENOMEM;
    }

    __bmp_set_bit(0, bmp);
    if !bmp_test_bit(0, bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    __bmp_set_bit(MID_BIT(), bmp);
    if !bmp_test_bit(MID_BIT(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    __bmp_set_bit(LAST_BIT(), bmp);
    if !bmp_test_bit(LAST_BIT(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if bmp_test_bit(MID_BIT().wrapping_sub(1), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    __bmp_clear_bit(MID_BIT(), bmp);
    if bmp_test_bit(MID_BIT(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !bmp_test_bit(0, bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !bmp_test_bit(LAST_BIT(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    __bmp_clear_bit(0, bmp);
    __bmp_clear_bit(LAST_BIT(), bmp);
    if !bmp_empty(TEST_BITS(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    let bits = (*bmp).bits.as_ptr();
    if *bits.add(0) != 0 {
        bmp_free(bmp);
        return -EINVAL;
    }

    if *bits.add(1) != 0 {
        bmp_free(bmp);
        return -EINVAL;
    }

    bmp_free(bmp);
    0
}

unsafe fn test_bitmap_test_and_clear_single(bmp: *mut arena_bitmap, ind: size_t) -> bool {
    if bmp_test_and_clear_bit(ind, bmp) {
        return false;
    }

    __bmp_set_bit(ind, bmp);

    if !bmp_test_and_clear_bit(ind, bmp) {
        return false;
    }

    if bmp_test_bit(ind, bmp) {
        return false;
    }

    if bmp_test_and_clear_bit(ind, bmp) {
        return false;
    }

    true
}

unsafe fn test_bitmap_test_and_set_single(bmp: *mut arena_bitmap, ind: size_t) -> bool {
    if bmp_test_and_set_bit(ind, bmp) {
        return false;
    }

    if !bmp_test_and_set_bit(ind, bmp) {
        return false;
    }

    if !bmp_test_bit(ind, bmp) {
        return false;
    }

    __bmp_clear_bit(ind, bmp);

    if bmp_test_and_set_bit(ind, bmp) {
        return false;
    }

    true
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_test_and_clear_bit() -> i32 {
    let bmp: *mut arena_bitmap;

    bmp = bmp_alloc(TEST_BITS());
    if bmp.is_null() {
        return -ENOMEM;
    }

    if !test_bitmap_test_and_clear_single(bmp, 0) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !test_bitmap_test_and_clear_single(bmp, MID_BIT()) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !test_bitmap_test_and_clear_single(bmp, LAST_BIT()) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !bmp_empty(TEST_BITS(), bmp) {
        bmp_free(bmp);
        return -EINVAL;
    }

    bmp_free(bmp);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_test_and_set_bit() -> i32 {
    let bmp: *mut arena_bitmap;

    bmp = bmp_alloc(TEST_BITS());
    if bmp.is_null() {
        return -ENOMEM;
    }

    if !test_bitmap_test_and_set_single(bmp, 0) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !test_bitmap_test_and_set_single(bmp, MID_BIT()) {
        bmp_free(bmp);
        return -EINVAL;
    }

    if !test_bitmap_test_and_set_single(bmp, LAST_BIT()) {
        bmp_free(bmp);
        return -EINVAL;
    }

    bmp_free(bmp);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_and() -> i32 {
    let mut src1: *mut arena_bitmap = core::ptr::null_mut();
    let mut src2: *mut arena_bitmap = core::ptr::null_mut();
    let mut dst: *mut arena_bitmap = core::ptr::null_mut();

    src1 = bmp_alloc(TEST_BITS());
    src2 = bmp_alloc(TEST_BITS());
    dst = bmp_alloc(TEST_BITS());
    if src1.is_null() || src2.is_null() || dst.is_null() {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    test_bmp_setall(dst);

    __bmp_set_bit(0, src1);
    __bmp_set_bit(MID_BIT(), src1);
    __bmp_set_bit(LAST_BIT(), src1);

    __bmp_set_bit(MID_BIT(), src2);
    __bmp_set_bit(LAST_BIT(), src2);

    bmp_and(TEST_BITS(), dst, src1, src2);

    if bmp_test_bit(0, dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if !bmp_test_bit(MID_BIT(), dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if !bmp_test_bit(LAST_BIT(), dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    let dst_bits = (*dst).bits.as_ptr();
    if *dst_bits.add(0) != 0 {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if *dst_bits.add(1) != (BIT_MASK(MID_BIT()) | BIT_MASK(LAST_BIT())) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    bmp_free(src1);
    bmp_free(src2);
    bmp_free(dst);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_or() -> i32 {
    let mut src1: *mut arena_bitmap = core::ptr::null_mut();
    let mut src2: *mut arena_bitmap = core::ptr::null_mut();
    let mut dst: *mut arena_bitmap = core::ptr::null_mut();

    src1 = bmp_alloc(TEST_BITS());
    src2 = bmp_alloc(TEST_BITS());
    dst = bmp_alloc(TEST_BITS());
    if src1.is_null() || src2.is_null() || dst.is_null() {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    test_bmp_setall(dst);

    __bmp_set_bit(0, src1);
    __bmp_set_bit(LAST_BIT(), src1);

    __bmp_set_bit(MID_BIT(), src2);
    __bmp_set_bit(LAST_BIT(), src2);

    bmp_or(TEST_BITS(), dst, src1, src2);

    if !bmp_test_bit(0, dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if !bmp_test_bit(MID_BIT(), dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if !bmp_test_bit(LAST_BIT(), dst) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    let dst_bits = (*dst).bits.as_ptr();
    if *dst_bits.add(0) != BIT_MASK(0) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }
    if *dst_bits.add(1) != (BIT_MASK(MID_BIT()) | BIT_MASK(LAST_BIT())) {
        bmp_free(src1);
        bmp_free(src2);
        bmp_free(dst);
        return -EINVAL;
    }

    bmp_free(src1);
    bmp_free(src2);
    bmp_free(dst);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_subset() -> i32 {
    let mut big: *mut arena_bitmap = core::ptr::null_mut();
    let mut small: *mut arena_bitmap = core::ptr::null_mut();

    big = bmp_alloc(TEST_BITS());
    small = bmp_alloc(TEST_BITS());
    if big.is_null() || small.is_null() {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    if !bmp_subset(TEST_BITS(), big, small) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    __bmp_set_bit(0, small);
    if bmp_subset(TEST_BITS(), big, small) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    __bmp_set_bit(0, big);
    if !bmp_subset(TEST_BITS(), big, small) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    __bmp_set_bit(LAST_BIT(), small);
    if bmp_subset(TEST_BITS(), big, small) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    __bmp_set_bit(LAST_BIT(), big);
    __bmp_set_bit(MID_BIT(), big);
    if !bmp_subset(TEST_BITS(), big, small) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    if bmp_subset(TEST_BITS(), small, big) {
        bmp_free(big);
        bmp_free(small);
        return -EINVAL;
    }

    bmp_free(big);
    bmp_free(small);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_intersects() -> i32 {
    let mut arg1: *mut arena_bitmap = core::ptr::null_mut();
    let mut arg2: *mut arena_bitmap = core::ptr::null_mut();

    arg1 = bmp_alloc(TEST_BITS());
    arg2 = bmp_alloc(TEST_BITS());
    if arg1.is_null() || arg2.is_null() {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    if bmp_intersects(TEST_BITS(), arg1, arg2) {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    __bmp_set_bit(0, arg1);
    __bmp_set_bit(MID_BIT(), arg2);
    if bmp_intersects(TEST_BITS(), arg1, arg2) {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    __bmp_set_bit(LAST_BIT(), arg1);
    __bmp_set_bit(LAST_BIT(), arg2);
    if !bmp_intersects(TEST_BITS(), arg1, arg2) {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    bmp_free(arg1);
    bmp_free(arg2);
    0
}

// SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_bitmap_copy() -> i32 {
    let mut arg1: *mut arena_bitmap = core::ptr::null_mut();
    let mut arg2: *mut arena_bitmap = core::ptr::null_mut();

    arg1 = bmp_alloc(TEST_BITS());
    arg2 = bmp_alloc(TEST_BITS());
    if arg1.is_null() || arg2.is_null() {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    __bmp_set_bit(0, arg1);
    __bmp_set_bit(MID_BIT(), arg1);

    /* Make sure those get overwritten. */
    __bmp_set_bit(1, arg2);
    __bmp_set_bit(MID_BIT().wrapping_add(2), arg2);

    bmp_copy(TEST_BITS(), arg2, arg1);

    /* Bitmaps are equal if a subset of each other. */
    if !bmp_subset(TEST_BITS(), arg1, arg2) || !bmp_subset(TEST_BITS(), arg2, arg1) {
        bmp_free(arg1);
        bmp_free(arg2);
        return -EINVAL;
    }

    bmp_free(arg1);
    bmp_free(arg2);
    0
}
