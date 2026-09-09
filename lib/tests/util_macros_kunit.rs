// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for bitfield helpers.
 */

// Dependencies supplied by the surrounding kernel/Rust integration.

macro_rules! find_closest_range_check {
    ($ctx:expr, $from:expr, $to:expr, $array:expr, $exp_idx:expr) => {{
        for i in $from..=$to {
            let found = find_closest(i, &$array, $array.len());
            kunit_assert_eq!($ctx, $exp_idx, found);
        }
    }};
}

unsafe fn test_find_closest(ctx: *mut kunit) {
    /* This will test a few arrays that are found in drivers */
    let ina226_avg_tab: [i32; 8] = [1, 4, 16, 64, 128, 256, 512, 1024];
    let ad7616_oversampling_avail: [u32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    let wd_timeout_table: [u32; 8] = [2, 4, 6, 8, 16, 32, 48, 64];
    let array_prog1a: [i32; 5] = [1, 2, 3, 4, 5];
    let array_prog1b: [u32; 5] = [2, 3, 4, 5, 6];
    let array_prog1mix: [i32; 5] = [-2, -1, 0, 1, 2];
    let array_prog2a: [i32; 4] = [1, 3, 5, 7];
    let array_prog2b: [u32; 4] = [2, 4, 6, 8];
    let array_prog3a: [i32; 4] = [1, 4, 7, 10];
    let array_prog3b: [u32; 4] = [2, 5, 8, 11];
    let array_prog4a: [i32; 4] = [1, 5, 9, 13];
    let array_prog4b: [u32; 4] = [2, 6, 10, 14];

    find_closest_range_check!(ctx, -3, 2, ina226_avg_tab, 0);
    find_closest_range_check!(ctx, 3, 10, ina226_avg_tab, 1);
    find_closest_range_check!(ctx, 11, 40, ina226_avg_tab, 2);
    find_closest_range_check!(ctx, 41, 96, ina226_avg_tab, 3);
    find_closest_range_check!(ctx, 97, 192, ina226_avg_tab, 4);
    find_closest_range_check!(ctx, 193, 384, ina226_avg_tab, 5);
    find_closest_range_check!(ctx, 385, 768, ina226_avg_tab, 6);
    find_closest_range_check!(ctx, 769, 2048, ina226_avg_tab, 7);

    find_closest_range_check!(ctx, -3, 1, ad7616_oversampling_avail, 0);
    find_closest_range_check!(ctx, 2, 3, ad7616_oversampling_avail, 1);
    find_closest_range_check!(ctx, 4, 6, ad7616_oversampling_avail, 2);
    find_closest_range_check!(ctx, 7, 12, ad7616_oversampling_avail, 3);
    find_closest_range_check!(ctx, 13, 24, ad7616_oversampling_avail, 4);
    find_closest_range_check!(ctx, 25, 48, ad7616_oversampling_avail, 5);
    find_closest_range_check!(ctx, 49, 96, ad7616_oversampling_avail, 6);
    find_closest_range_check!(ctx, 97, 256, ad7616_oversampling_avail, 7);

    find_closest_range_check!(ctx, -3, 3, wd_timeout_table, 0);
    find_closest_range_check!(ctx, 4, 5, wd_timeout_table, 1);
    find_closest_range_check!(ctx, 6, 7, wd_timeout_table, 2);
    find_closest_range_check!(ctx, 8, 12, wd_timeout_table, 3);
    find_closest_range_check!(ctx, 13, 24, wd_timeout_table, 4);
    find_closest_range_check!(ctx, 25, 40, wd_timeout_table, 5);
    find_closest_range_check!(ctx, 41, 56, wd_timeout_table, 6);
    find_closest_range_check!(ctx, 57, 128, wd_timeout_table, 7);

    /* One could argue that find_closest() should not be used for monotonic
     * arrays (like 1,2,3,4,5), but even so, it should work as long as the
     * array is sorted ascending. */
    find_closest_range_check!(ctx, -3, 1, array_prog1a, 0);
    find_closest_range_check!(ctx, 2, 2, array_prog1a, 1);
    find_closest_range_check!(ctx, 3, 3, array_prog1a, 2);
    find_closest_range_check!(ctx, 4, 4, array_prog1a, 3);
    find_closest_range_check!(ctx, 5, 8, array_prog1a, 4);
    find_closest_range_check!(ctx, -3, 2, array_prog1b, 0);
    find_closest_range_check!(ctx, 3, 3, array_prog1b, 1);
    find_closest_range_check!(ctx, 4, 4, array_prog1b, 2);
    find_closest_range_check!(ctx, 5, 5, array_prog1b, 3);
    find_closest_range_check!(ctx, 6, 8, array_prog1b, 4);
    find_closest_range_check!(ctx, -4, -2, array_prog1mix, 0);
    find_closest_range_check!(ctx, -1, -1, array_prog1mix, 1);
    find_closest_range_check!(ctx, 0, 0, array_prog1mix, 2);
    find_closest_range_check!(ctx, 1, 1, array_prog1mix, 3);
    find_closest_range_check!(ctx, 2, 5, array_prog1mix, 4);
    find_closest_range_check!(ctx, -3, 2, array_prog2a, 0);
    find_closest_range_check!(ctx, 3, 4, array_prog2a, 1);
    find_closest_range_check!(ctx, 5, 6, array_prog2a, 2);
    find_closest_range_check!(ctx, 7, 10, array_prog2a, 3);
    find_closest_range_check!(ctx, -3, 3, array_prog2b, 0);
    find_closest_range_check!(ctx, 4, 5, array_prog2b, 1);
    find_closest_range_check!(ctx, 6, 7, array_prog2b, 2);
    find_closest_range_check!(ctx, 8, 10, array_prog2b, 3);
    find_closest_range_check!(ctx, -3, 2, array_prog3a, 0);
    find_closest_range_check!(ctx, 3, 5, array_prog3a, 1);
    find_closest_range_check!(ctx, 6, 8, array_prog3a, 2);
    find_closest_range_check!(ctx, 9, 20, array_prog3a, 3);
    find_closest_range_check!(ctx, -3, 3, array_prog3b, 0);
    find_closest_range_check!(ctx, 4, 6, array_prog3b, 1);
    find_closest_range_check!(ctx, 7, 9, array_prog3b, 2);
    find_closest_range_check!(ctx, 10, 20, array_prog3b, 3);
    find_closest_range_check!(ctx, -3, 3, array_prog4a, 0);
    find_closest_range_check!(ctx, 4, 7, array_prog4a, 1);
    find_closest_range_check!(ctx, 8, 11, array_prog4a, 2);
    find_closest_range_check!(ctx, 12, 20, array_prog4a, 3);
    find_closest_range_check!(ctx, -3, 4, array_prog4b, 0);
    find_closest_range_check!(ctx, 5, 8, array_prog4b, 1);
    find_closest_range_check!(ctx, 9, 12, array_prog4b, 2);
    find_closest_range_check!(ctx, 13, 20, array_prog4b, 3);
}

macro_rules! find_closest_desc_range_check {
    ($ctx:expr, $from:expr, $to:expr, $array:expr, $exp_idx:expr) => {{
        for i in $from..=$to {
            let found = find_closest_descending(i, &$array, $array.len());
            kunit_assert_eq!($ctx, $exp_idx, found);
        }
    }};
}

// Same arrays as `test_find_closest`, but reversed. The corresponding test
// cases retain the exact ranges and expected indices from the C source.
unsafe fn test_find_closest_descending(ctx: *mut kunit) {
    let ina226_avg_tab = [1024, 512, 256, 128, 64, 16, 4, 1];
    let ad7616_oversampling_avail: [u32; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
    let wd_timeout_table: [u32; 8] = [64, 48, 32, 16, 8, 6, 4, 2];
    let array_prog1a = [5, 4, 3, 2, 1];
    let array_prog1b: [u32; 5] = [6, 5, 4, 3, 2];
    let array_prog1mix = [2, 1, 0, -1, -2];
    let array_prog2a = [7, 5, 3, 1];
    let array_prog2b: [u32; 4] = [8, 6, 4, 2];
    let array_prog3a = [10, 7, 4, 1];
    let array_prog3b: [u32; 4] = [11, 8, 5, 2];
    let array_prog4a = [13, 9, 5, 1];
    let array_prog4b: [u32; 4] = [14, 10, 6, 2];

    find_closest_desc_range_check!(ctx, -3, 2, ina226_avg_tab, 7);
    find_closest_desc_range_check!(ctx, 3, 10, ina226_avg_tab, 6);
    find_closest_desc_range_check!(ctx, 11, 40, ina226_avg_tab, 5);
    find_closest_desc_range_check!(ctx, 41, 96, ina226_avg_tab, 4);
    find_closest_desc_range_check!(ctx, 97, 192, ina226_avg_tab, 3);
    find_closest_desc_range_check!(ctx, 193, 384, ina226_avg_tab, 2);
    find_closest_desc_range_check!(ctx, 385, 768, ina226_avg_tab, 1);
    find_closest_desc_range_check!(ctx, 769, 2048, ina226_avg_tab, 0);
    find_closest_desc_range_check!(ctx, -3, 1, ad7616_oversampling_avail, 7);
    find_closest_desc_range_check!(ctx, 2, 3, ad7616_oversampling_avail, 6);
    find_closest_desc_range_check!(ctx, 4, 6, ad7616_oversampling_avail, 5);
    find_closest_desc_range_check!(ctx, 7, 12, ad7616_oversampling_avail, 4);
    find_closest_desc_range_check!(ctx, 13, 24, ad7616_oversampling_avail, 3);
    find_closest_desc_range_check!(ctx, 25, 48, ad7616_oversampling_avail, 2);
    find_closest_desc_range_check!(ctx, 49, 96, ad7616_oversampling_avail, 1);
    find_closest_desc_range_check!(ctx, 97, 256, ad7616_oversampling_avail, 0);
    find_closest_desc_range_check!(ctx, -3, 3, wd_timeout_table, 7);
    find_closest_desc_range_check!(ctx, 4, 5, wd_timeout_table, 6);
    find_closest_desc_range_check!(ctx, 6, 7, wd_timeout_table, 5);
    find_closest_desc_range_check!(ctx, 8, 12, wd_timeout_table, 4);
    find_closest_desc_range_check!(ctx, 13, 24, wd_timeout_table, 3);
    find_closest_desc_range_check!(ctx, 25, 40, wd_timeout_table, 2);
    find_closest_desc_range_check!(ctx, 41, 56, wd_timeout_table, 1);
    find_closest_desc_range_check!(ctx, 57, 128, wd_timeout_table, 0);
    find_closest_desc_range_check!(ctx, -3, 1, array_prog1a, 4);
    find_closest_desc_range_check!(ctx, 2, 2, array_prog1a, 3);
    find_closest_desc_range_check!(ctx, 3, 3, array_prog1a, 2);
    find_closest_desc_range_check!(ctx, 4, 4, array_prog1a, 1);
    find_closest_desc_range_check!(ctx, 5, 8, array_prog1a, 0);
    find_closest_desc_range_check!(ctx, -3, 2, array_prog1b, 4);
    find_closest_desc_range_check!(ctx, 3, 3, array_prog1b, 3);
    find_closest_desc_range_check!(ctx, 4, 4, array_prog1b, 2);
    find_closest_desc_range_check!(ctx, 5, 5, array_prog1b, 1);
    find_closest_desc_range_check!(ctx, 6, 8, array_prog1b, 0);
    find_closest_desc_range_check!(ctx, -4, -2, array_prog1mix, 4);
    find_closest_desc_range_check!(ctx, -1, -1, array_prog1mix, 3);
    find_closest_desc_range_check!(ctx, 0, 0, array_prog1mix, 2);
    find_closest_desc_range_check!(ctx, 1, 1, array_prog1mix, 1);
    find_closest_desc_range_check!(ctx, 2, 5, array_prog1mix, 0);
    find_closest_desc_range_check!(ctx, -3, 2, array_prog2a, 3);
    find_closest_desc_range_check!(ctx, 3, 4, array_prog2a, 2);
    find_closest_desc_range_check!(ctx, 5, 6, array_prog2a, 1);
    find_closest_desc_range_check!(ctx, 7, 10, array_prog2a, 0);
    find_closest_desc_range_check!(ctx, -3, 3, array_prog2b, 3);
    find_closest_desc_range_check!(ctx, 4, 5, array_prog2b, 2);
    find_closest_desc_range_check!(ctx, 6, 7, array_prog2b, 1);
    find_closest_desc_range_check!(ctx, 8, 10, array_prog2b, 0);
    find_closest_desc_range_check!(ctx, -3, 2, array_prog3a, 3);
    find_closest_desc_range_check!(ctx, 3, 5, array_prog3a, 2);
    find_closest_desc_range_check!(ctx, 6, 8, array_prog3a, 1);
    find_closest_desc_range_check!(ctx, 9, 20, array_prog3a, 0);
    find_closest_desc_range_check!(ctx, -3, 3, array_prog3b, 3);
    find_closest_desc_range_check!(ctx, 4, 6, array_prog3b, 2);
    find_closest_desc_range_check!(ctx, 7, 9, array_prog3b, 1);
    find_closest_desc_range_check!(ctx, 10, 20, array_prog3b, 0);
    find_closest_desc_range_check!(ctx, -3, 3, array_prog4a, 3);
    find_closest_desc_range_check!(ctx, 4, 7, array_prog4a, 2);
    find_closest_desc_range_check!(ctx, 8, 11, array_prog4a, 1);
    find_closest_desc_range_check!(ctx, 12, 20, array_prog4a, 0);
    find_closest_desc_range_check!(ctx, -3, 4, array_prog4b, 3);
    find_closest_desc_range_check!(ctx, 5, 8, array_prog4b, 2);
    find_closest_desc_range_check!(ctx, 9, 12, array_prog4b, 1);
    find_closest_desc_range_check!(ctx, 13, 20, array_prog4b, 0);
}

// KUnit registration and module metadata supplied by the kernel integration.
static mut UTIL_MACROS_TEST_CASES: [Option<unsafe fn(*mut kunit)>; 3] =
    [Some(test_find_closest), Some(test_find_closest_descending), None];

static mut UTIL_MACROS_TEST_SUITE: kunit_suite = kunit_suite {
    name: "util_macros.h",
    test_cases: &mut UTIL_MACROS_TEST_CASES,
};

kunit_test_suites!(&mut UTIL_MACROS_TEST_SUITE);

module_author!("Alexandru Ardelean <aardelean@baylibre.com>");
module_description!("Test cases for util_macros.h helpers");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
