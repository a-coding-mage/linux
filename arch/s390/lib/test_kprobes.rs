// SPDX-License-Identifier: GPL-2.0+

// Kernel and KUnit declarations are supplied by the surrounding build.

use core::ffi::c_char;
use core::ptr;

#[repr(C)]
pub struct kprobe {
    pub offset: i32,
    pub addr: *mut core::ffi::c_void,
    pub symbol_name: *const c_char,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    fn register_kprobe(kp: *mut kprobe) -> i32;
    fn unregister_kprobe(kp: *mut kprobe);
    fn kunit_expect_eq(test: *mut kunit, left: i32, right: i32);

    static kprobes_target_odd_offs: i32;
    static kprobes_target_in_insn4_offs: i32;
    static kprobes_target_in_insn6_lo_offs: i32;
    static kprobes_target_in_insn6_hi_offs: i32;
}

static mut kp: kprobe = kprobe {
    offset: 0,
    addr: ptr::null_mut(),
    symbol_name: ptr::null(),
};

unsafe fn setup_kprobe(
    _test: *mut kunit,
    kp: *mut kprobe,
    symbol: *const c_char,
    offset: i32,
) {
    (*kp).offset = offset;
    (*kp).addr = ptr::null_mut();
    (*kp).symbol_name = symbol;
}

unsafe fn test_kprobe_offset(
    test: *mut kunit,
    kp: *mut kprobe,
    target: *const c_char,
    offset: i32,
) {
    let mut ret: i32;

    setup_kprobe(test, kp, target, 0);
    ret = register_kprobe(kp);
    if ret == 0 {
        unregister_kprobe(kp);
    }
    kunit_expect_eq(test, 0, ret);
    setup_kprobe(test, kp, target, offset);
    ret = register_kprobe(kp);
    kunit_expect_eq(test, -22, ret); // -EINVAL
    if ret == 0 {
        unregister_kprobe(kp);
    }
}

unsafe extern "C" fn test_kprobe_odd(test: *mut kunit) {
    test_kprobe_offset(
        test,
        &raw mut kp,
        c"kprobes_target_odd".as_ptr(),
        kprobes_target_odd_offs,
    );
}

unsafe extern "C" fn test_kprobe_in_insn4(test: *mut kunit) {
    test_kprobe_offset(
        test,
        &raw mut kp,
        c"kprobes_target_in_insn4".as_ptr(),
        kprobes_target_in_insn4_offs,
    );
}

unsafe extern "C" fn test_kprobe_in_insn6_lo(test: *mut kunit) {
    test_kprobe_offset(
        test,
        &raw mut kp,
        c"kprobes_target_in_insn6_lo".as_ptr(),
        kprobes_target_in_insn6_lo_offs,
    );
}

unsafe extern "C" fn test_kprobe_in_insn6_hi(test: *mut kunit) {
    test_kprobe_offset(
        test,
        &raw mut kp,
        c"kprobes_target_in_insn6_hi".as_ptr(),
        kprobes_target_in_insn6_hi_offs,
    );
}

// Equivalent KUnit case table:
// KUNIT_CASE(test_kprobe_odd),
// KUNIT_CASE(test_kprobe_in_insn4),
// KUNIT_CASE(test_kprobe_in_insn6_lo),
// KUNIT_CASE(test_kprobe_in_insn6_hi),
// {}

// Equivalent suite declaration:
// .name = "kprobes_test_s390",
// .test_cases = kprobes_testcases,

// kunit_test_suites(&kprobes_test_suite);
// MODULE_DESCRIPTION("KUnit tests for kprobes");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
