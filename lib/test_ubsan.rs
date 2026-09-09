// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel build environment are intentionally
// left as external symbols.

type TestUbsanFp = unsafe extern "C" fn();

macro_rules! ubsan_test {
    ($config:ident) => {
        unsafe { pr_info(concat!("%s (%s=%s)\n")); }
    };
    ($config:ident, $msg:literal) => {
        unsafe { pr_info(concat!("%s ", $msg, "%s(%s=%s)\n")); }
    };
}

extern "C" {
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

unsafe fn test_ubsan_add_overflow() {
    let mut val: i32 = i32::MAX;

    ubsan_test!(CONFIG_UBSAN_INTEGER_WRAP);
    val = val.wrapping_add(2);
}

unsafe fn test_ubsan_sub_overflow() {
    let mut val: i32 = i32::MIN;
    let val2: i32 = 2;

    ubsan_test!(CONFIG_UBSAN_INTEGER_WRAP);
    val = val.wrapping_sub(val2);
}

unsafe fn test_ubsan_mul_overflow() {
    let mut val: i32 = i32::MAX / 2;

    ubsan_test!(CONFIG_UBSAN_INTEGER_WRAP);
    val = val.wrapping_mul(3);
}

unsafe fn test_ubsan_negate_overflow() {
    let mut val: i32 = i32::MIN;

    ubsan_test!(CONFIG_UBSAN_INTEGER_WRAP);
    val = val.wrapping_neg();
}

unsafe fn test_ubsan_divrem_overflow() {
    let mut val: i32 = 16;
    let val2: i32 = 0;

    ubsan_test!(CONFIG_UBSAN_DIV_ZERO);
    val = val / val2;
}

unsafe fn test_ubsan_truncate_signed() {
    let val: i64 = i64::MAX;
    let mut val2: i32 = 0;

    ubsan_test!(CONFIG_UBSAN_INTEGER_WRAP);
    val2 = val as i32;
}

unsafe fn test_ubsan_shift_out_of_bounds() {
    let neg: i32 = -1;
    let wrap: i32 = 4;
    let mut val1: i32 = 10;
    let mut val2: i32 = i32::MAX;

    ubsan_test!(CONFIG_UBSAN_SHIFT, "negative exponent");
    val1 = val1.wrapping_shl(neg as u32);

    ubsan_test!(CONFIG_UBSAN_SHIFT, "left overflow");
    val2 = val2.wrapping_shl(wrap as u32);
}

#[repr(C)]
struct TestUbsanData {
    above: [i8; 4],
    arr: [i32; 4],
    below: [i8; 4],
}

unsafe fn test_ubsan_out_of_bounds() {
    let i: isize = 4;
    let j: isize = 4;
    let k: isize = -1;
    let mut data = TestUbsanData {
        above: [0; 4],
        arr: [0; 4],
        below: [0; 4],
    };

    ubsan_test!(CONFIG_UBSAN_BOUNDS, "above");
    *data.arr.as_mut_ptr().offset(j) = i as i32;

    ubsan_test!(CONFIG_UBSAN_BOUNDS, "below");
    *data.arr.as_mut_ptr().offset(k) = i as i32;
}

#[repr(i32)]
enum UbsanTestEnum {
    UBSAN_TEST_ZERO = 0,
    UBSAN_TEST_ONE,
    UBSAN_TEST_MAX,
}

unsafe fn test_ubsan_load_invalid_value() {
    let mut val: bool = false;
    let mut val2: bool = false;
    let mut eval = UbsanTestEnum::UBSAN_TEST_ZERO;
    let mut eval2 = UbsanTestEnum::UBSAN_TEST_ZERO;
    let c: u8 = 0xff;

    ubsan_test!(CONFIG_UBSAN_BOOL, "bool");
    *(core::ptr::addr_of_mut!(val) as *mut u8) = c;
    val2 = val;

    ubsan_test!(CONFIG_UBSAN_ENUM, "enum");
    *(core::ptr::addr_of_mut!(eval) as *mut u8) = c;
    eval2 = eval;
}

unsafe fn test_ubsan_misaligned_access() {
    let mut arr: [u8; 5] = [1, 2, 3, 4, 5];
    let val: i32 = 6;

    ubsan_test!(CONFIG_UBSAN_ALIGNMENT);
    *(arr.as_mut_ptr().add(1) as *mut i32) = val;
}

static TEST_UBSAN_ARRAY: [TestUbsanFp; 9] = [
    test_ubsan_add_overflow,
    test_ubsan_sub_overflow,
    test_ubsan_mul_overflow,
    test_ubsan_negate_overflow,
    test_ubsan_truncate_signed,
    test_ubsan_shift_out_of_bounds,
    test_ubsan_out_of_bounds,
    test_ubsan_load_invalid_value,
    test_ubsan_misaligned_access,
];

// Excluded because they Oops the module.
#[used]
static SKIP_UBSAN_ARRAY: [TestUbsanFp; 1] = [test_ubsan_divrem_overflow];

unsafe fn test_ubsan_init() -> i32 {
    let mut i: usize = 0;

    while i < TEST_UBSAN_ARRAY.len() {
        (TEST_UBSAN_ARRAY[i])();
        i += 1;
    }

    0
}

unsafe fn test_ubsan_exit() {
    // do nothing
}

// module_init(test_ubsan_init);
// module_exit(test_ubsan_exit);
// MODULE_AUTHOR("Jinbum Park <jinb.park7@gmail.com>");
// MODULE_DESCRIPTION("UBSAN unit test");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
