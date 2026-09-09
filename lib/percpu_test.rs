// SPDX-License-Identifier: GPL-2.0-only
// Translated from the Linux kernel percpu test implementation.

use core::ffi::c_char;

// These symbols/macros are supplied by the kernel environment.
extern "C" {
    fn pr_info(fmt: *const c_char, ...);
    fn WARN(condition: bool, fmt: *const c_char, ...);
}

static mut LONG_COUNTER: isize = 0;
static mut ULONG_COUNTER: usize = 0;

macro_rules! check {
    ($native:expr, $pcp:expr, $expected:expr) => {{
        let native = $native;
        let expected = $expected;
        unsafe {
            WARN(
                native != expected,
                b"raw %ld (0x%lx) != expected %lld (0x%llx)\0".as_ptr() as *const c_char,
                native,
                native,
                expected as i64,
                expected as i64,
            );
            let pcp = $pcp;
            WARN(
                pcp != expected,
                b"pcp %ld (0x%lx) != expected %lld (0x%llx)\0".as_ptr() as *const c_char,
                pcp,
                pcp,
                expected as i64,
                expected as i64,
            );
        }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn percpu_test_init() -> i32 {
    // volatile prevents compiler from optimizing its uses, otherwise the
    // +ul_one/-ul_one below would be replaced with inc/dec instructions.
    let mut ui_one: u32 = 1;
    let mut ull: u64 = 0;
    let mut ul: usize = 0;
    let mut l: isize = 0;

    pr_info(b"percpu test start\n\0".as_ptr() as *const c_char);

    // preempt_disable();

    l = l.wrapping_add((-1isize));
    LONG_COUNTER = LONG_COUNTER.wrapping_add(-1isize);
    check!(l, LONG_COUNTER, -1isize);

    l = l.wrapping_add(1);
    LONG_COUNTER = LONG_COUNTER.wrapping_add(1);
    check!(l, LONG_COUNTER, 0isize);

    ul = 0;
    ULONG_COUNTER = 0;

    ul = ul.wrapping_add(1usize);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_add(1usize);
    check!(ul, ULONG_COUNTER, 1usize);

    ul = ul.wrapping_add((-1isize) as usize);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_add((-1isize) as usize);
    check!(ul, ULONG_COUNTER, 0usize);

    ul = ul.wrapping_add((-(1isize)) as usize);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_add((-(1isize)) as usize);
    check!(ul, ULONG_COUNTER, (-1isize) as usize);

    ul = 0;
    ULONG_COUNTER = 0;

    ul = ul.wrapping_sub(1);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_sub(1);
    check!(ul, ULONG_COUNTER, (-1isize) as usize);
    check!(ul, ULONG_COUNTER, usize::MAX);

    l = l.wrapping_sub(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_sub(ui_one as isize);
    check!(l, LONG_COUNTER, 0xffffffffisize);

    l = l.wrapping_add(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_add(ui_one as isize);
    check!(l, LONG_COUNTER, 0x1_0000_0000isize);

    l = 0;
    LONG_COUNTER = 0;

    l = l.wrapping_sub(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_sub(ui_one as isize);
    check!(l, LONG_COUNTER, -1isize);

    l = 0;
    LONG_COUNTER = 0;

    l = l.wrapping_add(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_add(ui_one as isize);
    check!(l, LONG_COUNTER, 1isize);

    l = l.wrapping_sub(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_sub(ui_one as isize);
    check!(l, LONG_COUNTER, 0x1_0000_0000isize);

    l = 0;
    LONG_COUNTER = 0;

    l = l.wrapping_sub(ui_one as isize);
    LONG_COUNTER = LONG_COUNTER.wrapping_sub(ui_one as isize);
    check!(l, LONG_COUNTER, -1isize);
    check!(l, LONG_COUNTER, usize::MAX);

    ul = 0;
    ULONG_COUNTER = 0;

    ul = ul.wrapping_add(ui_one as usize);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_add(ui_one as usize);
    check!(ul, ULONG_COUNTER, 1usize);

    ul = 0;
    ULONG_COUNTER = 0;

    ul = ul.wrapping_sub(ui_one as usize);
    ULONG_COUNTER = ULONG_COUNTER.wrapping_sub(ui_one as usize);
    check!(ul, ULONG_COUNTER, (-1isize) as usize);
    check!(ul, ULONG_COUNTER, usize::MAX);

    ul = 0;
    ull = 0;
    ULONG_COUNTER = 0;

    ull = ull.wrapping_add(u32::MAX as u64);
    ul = ull as usize;
    ULONG_COUNTER = ULONG_COUNTER.wrapping_add(ull as usize);
    check!(ul, ULONG_COUNTER, u32::MAX as usize);

    ul = 3;
    ULONG_COUNTER = 3;

    ul = ULONG_COUNTER.wrapping_sub(ui_one as usize);
    ULONG_COUNTER = ul;
    check!(ul, ULONG_COUNTER, 2usize);

    ul = ULONG_COUNTER.wrapping_sub(ui_one as usize);
    ULONG_COUNTER = ul;
    check!(ul, ULONG_COUNTER, 1usize);

    // preempt_enable();

    pr_info(b"percpu test done\n\0".as_ptr() as *const c_char);
    -11 // -EAGAIN: fail will directly unload the module
}

#[no_mangle]
pub unsafe extern "C" fn percpu_test_exit() {}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Greg Thelen");
// MODULE_DESCRIPTION("percpu operations test");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
