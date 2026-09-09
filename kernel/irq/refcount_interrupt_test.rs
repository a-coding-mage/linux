// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for refcounted interrupt enable/disables.
 */

// C dependencies: <kunit/test.h>, <linux/interrupt_rc.h>

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const core::ffi::c_char,
    pub test_cases: *mut kunit_case,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut kunit)>,
}

unsafe extern "C" {
    fn local_interrupt_disable();
    fn local_interrupt_enable();
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn irqs_disabled() -> bool;
}

type c_ulong = core::ffi::c_ulong;

// Equivalent to KUNIT_EXPECT_FALSE(test, irqs_disabled()).
macro_rules! test_irq_on {
    ($test:expr) => {
        let _ = $test;
        debug_assert!(!unsafe { irqs_disabled() });
    };
}

// Equivalent to KUNIT_EXPECT_TRUE(test, irqs_disabled()).
macro_rules! test_irq_off {
    ($test:expr) => {
        let _ = $test;
        debug_assert!(unsafe { irqs_disabled() });
    };
}

/* ===== Test cases ===== */
unsafe extern "C" fn test_single_irq_change(test: *mut kunit) {
    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_enable();
}

unsafe extern "C" fn test_nested_irq_change(test: *mut kunit) {
    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_disable();
    test_irq_off!(test);

    local_interrupt_enable();
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_on!(test);
}

unsafe extern "C" fn test_multiple_irq_change(test: *mut kunit) {
    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_disable();
    test_irq_off!(test);

    local_interrupt_enable();
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_on!(test);

    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_on!(test);
}

unsafe extern "C" fn test_irq_save(test: *mut kunit) {
    let mut flags: c_ulong = 0;

    local_irq_save(&mut flags);
    test_irq_off!(test);
    local_interrupt_disable();
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_off!(test);
    local_irq_restore(flags);
    test_irq_on!(test);

    local_interrupt_disable();
    test_irq_off!(test);
    local_irq_save(&mut flags);
    test_irq_off!(test);
    local_irq_restore(flags);
    test_irq_off!(test);
    local_interrupt_enable();
    test_irq_on!(test);
}

// KUNIT_CASE(test_*) entries, followed by the required zero sentinel.
static mut test_cases: [Option<unsafe extern "C" fn(*mut kunit)>; 5] = [
    Some(test_single_irq_change),
    Some(test_nested_irq_change),
    Some(test_multiple_irq_change),
    Some(test_irq_save),
    None,
];

/* init and exit are the same. */
unsafe extern "C" fn test_init(test: *mut kunit) -> core::ffi::c_int {
    test_irq_on!(test);
    0
}

unsafe extern "C" fn test_exit(test: *mut kunit) {
    test_irq_on!(test);
}

static mut refcount_interrupt_test_suite: kunit_suite = kunit_suite {
    name: b"refcount_interrupt\0".as_ptr() as *const core::ffi::c_char,
    test_cases: core::ptr::null_mut(),
    init: Some(test_init),
    exit: Some(test_exit),
};

// kunit_test_suite(refcount_interrupt_test_suite);
// MODULE_AUTHOR("Lyude Paul <lyude@redhat.com>");
// MODULE_DESCRIPTION("Refcounted interrupt unit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
