// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

// C dependencies:
// #include <bpf_atomic.h>
// #include <libarena/common.h>
// #include <libarena/asan.h>
// #include <libarena/bitmap.h>

type u64 = u64;
type u32 = u32;

const TEST_BITMAP_THREADS: u64 = 2;
const TEST_BITMAP_BITS: u32 = 2 * BITS_PER_LONG_LONG;
const TEST_BITMAP_SYNC_SPINS: u64 = BPF_MAX_LOOPS;
const TEST_BITMAP_ITERS: u32 = 10 * 1000 * 1000;

extern "C" {
    static zero: u32;
    static can_loop: bool;

    static BITS_PER_LONG_LONG: u32;
    static BPF_MAX_LOOPS: u64;

    static EINTR: i32;
    static ETIMEDOUT: i32;
    static EINVAL: i32;
    static EOPNOTSUPP: i32;
    static ENOMEM: i32;

    fn arena_subprog_init();
    fn bmp_alloc(bits: u32) -> *mut arena_bitmap;
    fn bmp_free(bitmap: *mut arena_bitmap);
    fn bmp_test_and_clear_bit(bit: u32, bitmap: *mut arena_bitmap) -> bool;
    fn bmp_test_and_set_bit(bit: u32, bitmap: *mut arena_bitmap) -> bool;
    fn bmp_test_bit(bit: u32, bitmap: *mut arena_bitmap) -> bool;
    fn bmp_set_bit(bit: u32, bitmap: *mut arena_bitmap);
    fn bmp_clear_bit(bit: u32, bitmap: *mut arena_bitmap);
    fn smp_load_acquire(p: *const u64) -> u64;
}

#[repr(C)]
pub struct arena_bitmap {
    _private: [u8; 0],
}

static mut bitmap: *mut arena_bitmap = core::ptr::null_mut();
static mut started: u64 = 0;
static mut test_abort: bool = false;

/*
 * The test needs cmpxchg atomics on arena memory.
 *
 * C condition:
 * defined(ENABLE_ATOMICS_TESTS) &&
 * (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) ||
 *  defined(__TARGET_ARCH_s390) || defined(__TARGET_ARCH_powerpc) ||
 *  (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64))
 */
fn bitmap_tests_enabled() -> bool {
    cfg!(all(
        feature = "ENABLE_ATOMICS_TESTS",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "s390x",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "riscv64"
        )
    ))
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn bitmap_wait_for_start() -> i32 {
    let mut i: u64;

    core::sync::atomic::AtomicU64::from_ptr(core::ptr::addr_of_mut!(started)).fetch_add(
        1,
        core::sync::atomic::Ordering::SeqCst,
    );

    i = zero as u64;
    while i < TEST_BITMAP_SYNC_SPINS && can_loop {
        if core::ptr::read_volatile(core::ptr::addr_of!(test_abort)) {
            return -EINTR;
        }
        if smp_load_acquire(core::ptr::addr_of!(started)) >= TEST_BITMAP_THREADS {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    core::ptr::write_volatile(core::ptr::addr_of_mut!(test_abort), true);
    -ETIMEDOUT
}

/*
 * The test makes sure writes don't clobber each other by overwriting
 * the same word. One thread always writes on even bits, the other on
 * odds. Both should be able to operate on the bitmap oblivious of the
 * other's operations.
 */
#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn bitmap_test_bit_sequence(bit: u32) -> i32 {
    if bmp_test_and_clear_bit(bit, bitmap) {
        return -EINVAL;
    }

    if bmp_test_and_set_bit(bit, bitmap) {
        return -EINVAL;
    }
    if !bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    if !bmp_test_and_set_bit(bit, bitmap) {
        return -EINVAL;
    }
    if !bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    if !bmp_test_and_clear_bit(bit, bitmap) {
        return -EINVAL;
    }
    if bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    if bmp_test_and_clear_bit(bit, bitmap) {
        return -EINVAL;
    }

    bmp_set_bit(bit, bitmap);
    if !bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    bmp_clear_bit(bit, bitmap);
    if bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    bmp_set_bit(bit, bitmap);
    if !bmp_test_bit(bit, bitmap) {
        return -EINVAL;
    }

    0
}

unsafe fn bitmap_test_reset_single(parity: i32) {
    let mut bit: u32;

    bit = parity as u32;
    while bit < TEST_BITMAP_BITS && can_loop {
        bmp_clear_bit(bit, bitmap);
        bit = bit.wrapping_add(2);
    }
}

unsafe fn bitmap_test_common_single(parity: i32) -> i32 {
    let mut bit: u32;
    let mut ret: i32;

    bit = parity as u32;
    while bit < TEST_BITMAP_BITS && can_loop {
        if core::ptr::read_volatile(core::ptr::addr_of!(test_abort)) {
            return -EINTR;
        }

        ret = bitmap_test_bit_sequence(bit);
        if ret != 0 {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(test_abort), true);
            return ret;
        }
        bit = bit.wrapping_add(2);
    }

    0
}

unsafe fn bitmap_test_common(parity: i32) -> i32 {
    let mut ret: i32;
    let mut i: u32;

    arena_subprog_init();

    ret = bitmap_wait_for_start();
    if ret != 0 {
        return ret;
    }

    i = zero;
    while i < TEST_BITMAP_ITERS && can_loop {
        ret = bitmap_test_common_single(parity);
        if ret != 0 {
            return ret;
        }

        if core::ptr::read_volatile(core::ptr::addr_of!(test_abort)) {
            break;
        }

        bitmap_test_reset_single(parity);
        i = i.wrapping_add(1);
    }

    0
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn parallel_test_bitmap__enabled() -> i32 {
    if bitmap_tests_enabled() {
        0
    } else {
        -EOPNOTSUPP
    }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn parallel_test_bitmap__init() -> i32 {
    bitmap = bmp_alloc(TEST_BITMAP_BITS);
    if bitmap.is_null() {
        return -ENOMEM;
    }

    0
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn parallel_test_bitmap__fini() -> i32 {
    let ret: i32 = 0;

    if bitmap.is_null() {
        return -EINVAL;
    }

    bmp_free(bitmap);
    bitmap = core::ptr::null_mut();

    ret
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn parallel_test_bitmap__0() -> i32 {
    bitmap_test_common(0)
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn parallel_test_bitmap__1() -> i32 {
    bitmap_test_common(1)
}
