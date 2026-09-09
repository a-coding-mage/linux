// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for clk_divider_bestdiv()
 */
// Dependencies supplied by the kernel test and clock-provider code.

use core::ffi::c_void;

const PARENT_RATE_1GHZ: c_ulong = GIGA;
const PARENT_RATE_2GHZ: c_ulong = 2 * GIGA;
const PARENT_RATE_4GHZ: c_ulong = 4 * GIGA;

const GIGA: c_ulong = 1_000_000_000;
type c_ulong = usize;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_div_table {
    pub val: u32,
    pub div: u32,
}

unsafe extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn clk_hw_register_fixed_rate(
        dev: *mut c_void,
        name: *const u8,
        parent_name: *const u8,
        flags: u32,
        rate: c_ulong,
    ) -> *mut clk_hw;
    fn clk_hw_unregister_fixed_rate(hw: *mut clk_hw);
    fn clk_hw_register_divider_table(
        dev: *mut c_void,
        name: *const u8,
        parent_name: *const u8,
        flags: u32,
        reg: *mut u32,
        shift: u8,
        width: u8,
        clk_divider_flags: u8,
        table: *const clk_div_table,
        lock: *mut c_void,
    ) -> *mut clk_hw;
    fn clk_hw_unregister_divider(hw: *mut clk_hw);
    fn clk_hw_register_mux(
        dev: *mut c_void,
        name: *const u8,
        parent_names: *const *const u8,
        num_parents: usize,
        flags: u32,
        reg: *mut u32,
        shift: u8,
        width: u8,
        mux_flags: u32,
        lock: *mut c_void,
    ) -> *mut clk_hw;
    fn clk_hw_unregister_mux(hw: *mut clk_hw);
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> i32;
}

unsafe extern "C" fn clk_hw_unregister_fixed_rate_wrapper(data: *mut c_void) {
    unsafe { clk_hw_unregister_fixed_rate(data.cast()) }
}

unsafe extern "C" fn clk_hw_unregister_divider_wrapper(data: *mut c_void) {
    unsafe { clk_hw_unregister_divider(data.cast()) }
}

unsafe extern "C" fn clk_hw_unregister_mux_wrapper(data: *mut c_void) {
    unsafe { clk_hw_unregister_mux(data.cast()) }
}

static BESTDIV_TABLE: [clk_div_table; 3] = [
    clk_div_table { val: 0, div: 2 },
    clk_div_table { val: 1, div: 4 },
    clk_div_table { val: 2, div: 8 },
];

/*
 * Test that clk_round_rate(clk, ULONG_MAX) returns the maximum achievable
 * rate for a divider clock.
 */
unsafe fn clk_divider_bestdiv_ulong_max_returns_max_rate(test: *mut kunit) {
    let mut fake_reg: *mut u32;
    let parent_hw: *mut clk_hw;
    let div_hw: *mut clk_hw;
    let rate: c_ulong;

    fake_reg = unsafe { kunit_kzalloc(test, core::mem::size_of::<u32>(), 0) }.cast();
    assert!(!fake_reg.is_null());

    parent_hw = unsafe {
        clk_hw_register_fixed_rate(
            core::ptr::null_mut(), b"bestdiv-parent\0".as_ptr(), core::ptr::null(), 0,
            PARENT_RATE_1GHZ,
        )
    };
    assert!(!parent_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_fixed_rate_wrapper, parent_hw.cast()) }, 0);

    div_hw = unsafe {
        clk_hw_register_divider_table(
            core::ptr::null_mut(), b"bestdiv-div\0".as_ptr(), b"bestdiv-parent\0".as_ptr(),
            1, fake_reg, 0, 2, 0, BESTDIV_TABLE.as_ptr(), core::ptr::null_mut(),
        )
    };
    assert!(!div_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_divider_wrapper, div_hw.cast()) }, 0);

    /* ULONG_MAX is the canonical way to probe the maximum rate a clock can produce. */
    rate = unsafe { clk_hw_round_rate(div_hw, c_ulong::MAX) };
    assert_eq!(rate, PARENT_RATE_1GHZ / 2);
}

/*
 * Test that clk_round_rate(clk, ULONG_MAX) returns the correct maximum rate
 * when a mux clock sits between a divider and its parent candidates.
 */
unsafe fn clk_divider_bestdiv_mux_ulong_max_returns_max_rate(test: *mut kunit) {
    let mux_parents: [*const u8; 2] = [
        b"bestdiv-mux-parent-a\0".as_ptr(),
        b"bestdiv-mux-parent-b\0".as_ptr(),
    ];
    let fake_reg_mux = unsafe { kunit_kzalloc(test, core::mem::size_of::<u32>(), 0) }.cast::<u32>();
    assert!(!fake_reg_mux.is_null());
    let fake_reg_div = unsafe { kunit_kzalloc(test, core::mem::size_of::<u32>(), 0) }.cast::<u32>();
    assert!(!fake_reg_div.is_null());

    let parent_a_hw = unsafe { clk_hw_register_fixed_rate(core::ptr::null_mut(), mux_parents[0], core::ptr::null(), 0, PARENT_RATE_4GHZ) };
    assert!(!parent_a_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_fixed_rate_wrapper, parent_a_hw.cast()) }, 0);
    let parent_b_hw = unsafe { clk_hw_register_fixed_rate(core::ptr::null_mut(), mux_parents[1], core::ptr::null(), 0, PARENT_RATE_2GHZ) };
    assert!(!parent_b_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_fixed_rate_wrapper, parent_b_hw.cast()) }, 0);

    let mux_hw = unsafe { clk_hw_register_mux(core::ptr::null_mut(), b"bestdiv-mux\0".as_ptr(), mux_parents.as_ptr(), mux_parents.len(), 1, fake_reg_mux, 0, 1, 0, core::ptr::null_mut()) };
    assert!(!mux_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_mux_wrapper, mux_hw.cast()) }, 0);
    let div_hw = unsafe { clk_hw_register_divider_table(core::ptr::null_mut(), b"bestdiv-mux-div\0".as_ptr(), b"bestdiv-mux\0".as_ptr(), 1, fake_reg_div, 0, 2, 0, BESTDIV_TABLE.as_ptr(), core::ptr::null_mut()) };
    assert!(!div_hw.is_null());
    assert_eq!(unsafe { kunit_add_action_or_reset(test, clk_hw_unregister_divider_wrapper, div_hw.cast()) }, 0);

    let rate = unsafe { clk_hw_round_rate(div_hw, c_ulong::MAX) };
    assert_eq!(rate, PARENT_RATE_4GHZ / 2);
}

// KUnit case and suite registration are supplied by the kernel test framework.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
