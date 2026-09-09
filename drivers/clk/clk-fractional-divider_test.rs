// SPDX-License-Identifier: GPL-2.0
/*
 * Kunit tests for clk fractional divider
 */

// Dependencies supplied by the kernel and by clk-fractional-divider.h are
// intentionally left as external interfaces.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_fractional_divider {
    pub hw: clk_hw,
    pub flags: u32,
    pub mwidth: u8,
    pub nwidth: u8,
}

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn clk_fractional_divider_general_approximation(
        hw: *mut clk_hw,
        rate: c_ulong,
        parent_rate: *mut c_ulong,
        m: *mut c_ulong,
        n: *mut c_ulong,
    );
}

type c_ulong = usize;

const GFP_KERNEL: u32 = 0;
const CLK_FRAC_DIVIDER_ZERO_BASED: u32 = 1;

/*
 * Test the maximum denominator case for fd clock without flags.
 *
 * Expect the highest possible denominator to be used in order to get as close as possible to the
 * requested rate.
 */
unsafe fn clk_fd_test_approximation_max_denominator(test: *mut kunit) {
    let fd = kunit_kzalloc(test, core::mem::size_of::<clk_fractional_divider>(), GFP_KERNEL)
        as *mut clk_fractional_divider;
    assert!(!fd.is_null());

    (*fd).mwidth = 3;
    (*fd).nwidth = 3;
    let max_n: c_ulong = 7;

    let rate: c_ulong = 240000000;
    let mut parent_rate = (max_n + 1) * rate; /* so that it exceeds the maximum divisor */
    let parent_rate_before = parent_rate;
    let mut m = 0;
    let mut n = 0;

    clk_fractional_divider_general_approximation(
        &mut (*fd).hw,
        rate,
        &mut parent_rate,
        &mut m,
        &mut n,
    );
    assert_eq!(parent_rate, parent_rate_before);
    assert_eq!(m, 1);
    assert_eq!(n, max_n);
}

/*
 * Test the maximum numerator case for fd clock without flags.
 *
 * Expect the highest possible numerator to be used in order to get as close as possible to the
 * requested rate.
 */
unsafe fn clk_fd_test_approximation_max_numerator(test: *mut kunit) {
    let fd = kunit_kzalloc(test, core::mem::size_of::<clk_fractional_divider>(), GFP_KERNEL)
        as *mut clk_fractional_divider;
    assert!(!fd.is_null());

    (*fd).mwidth = 3;
    let max_m: c_ulong = 7;
    (*fd).nwidth = 3;

    let rate: c_ulong = 240000000;
    let mut parent_rate = rate / (max_m + 1); /* so that it exceeds the maximum numerator */
    let parent_rate_before = parent_rate;
    let mut m = 0;
    let mut n = 0;

    clk_fractional_divider_general_approximation(
        &mut (*fd).hw,
        rate,
        &mut parent_rate,
        &mut m,
        &mut n,
    );
    assert_eq!(parent_rate, parent_rate_before);
    assert_eq!(m, max_m);
    assert_eq!(n, 1);
}

/*
 * Test the maximum denominator case for zero based fd clock.
 *
 * Expect the highest possible denominator to be used in order to get as close as possible to the
 * requested rate.
 */
unsafe fn clk_fd_test_approximation_max_denominator_zero_based(test: *mut kunit) {
    let fd = kunit_kzalloc(test, core::mem::size_of::<clk_fractional_divider>(), GFP_KERNEL)
        as *mut clk_fractional_divider;
    assert!(!fd.is_null());

    (*fd).flags = CLK_FRAC_DIVIDER_ZERO_BASED;
    (*fd).mwidth = 3;
    (*fd).nwidth = 3;
    let max_n: c_ulong = 8;

    let rate: c_ulong = 240000000;
    let mut parent_rate = (max_n + 1) * rate; /* so that it exceeds the maximum divisor */
    let parent_rate_before = parent_rate;
    let mut m = 0;
    let mut n = 0;

    clk_fractional_divider_general_approximation(
        &mut (*fd).hw,
        rate,
        &mut parent_rate,
        &mut m,
        &mut n,
    );
    assert_eq!(parent_rate, parent_rate_before);
    assert_eq!(m, 1);
    assert_eq!(n, max_n);
}

/*
 * Test the maximum numerator case for zero based fd clock.
 *
 * Expect the highest possible numerator to be used in order to get as close as possible to the
 * requested rate.
 */
unsafe fn clk_fd_test_approximation_max_numerator_zero_based(test: *mut kunit) {
    let fd = kunit_kzalloc(test, core::mem::size_of::<clk_fractional_divider>(), GFP_KERNEL)
        as *mut clk_fractional_divider;
    assert!(!fd.is_null());

    (*fd).flags = CLK_FRAC_DIVIDER_ZERO_BASED;
    (*fd).mwidth = 3;
    let max_m: c_ulong = 8;
    (*fd).nwidth = 3;

    let rate: c_ulong = 240000000;
    let mut parent_rate = rate / (max_m + 1); /* so that it exceeds the maximum numerator */
    let parent_rate_before = parent_rate;
    let mut m = 0;
    let mut n = 0;

    clk_fractional_divider_general_approximation(
        &mut (*fd).hw,
        rate,
        &mut parent_rate,
        &mut m,
        &mut n,
    );
    assert_eq!(parent_rate, parent_rate_before);
    assert_eq!(m, max_m);
    assert_eq!(n, 1);
}

// KUNIT_CASE entries and suite registration from the C source:
// clk_fd_approximation_test_cases = {
//     clk_fd_test_approximation_max_denominator,
//     clk_fd_test_approximation_max_numerator,
//     clk_fd_test_approximation_max_denominator_zero_based,
//     clk_fd_test_approximation_max_numerator_zero_based,
// };
// Suite name: "clk-fd-approximation".
// Kunit test suite registration is performed by the kernel build system.
// MODULE_DESCRIPTION("Kunit tests for clk fractional divider");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
