// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the linear_ranges helper.
 *
 * Copyright (C) 2020, ROHM Semiconductors.
 * Author: Matti Vaittinen <matti.vaittien@fi.rohmeurope.com>
 */

/* Kernel and KUnit declarations are supplied by the surrounding build. */

/* First things first. I deeply dislike unit-tests. I have seen all the hell
 * breaking loose when people who think the unit tests are "the silver bullet"
 * to kill bugs get to decide how a company should implement testing strategy...
 *
 * Believe me, it may get _really_ ridiculous. It is tempting to think that
 * walking through all the possible execution branches will nail down 100% of
 * bugs. This may lead to ideas about demands to get certain % of "test
 * coverage" - measured as line coverage. And that is one of the worst things
 * you can do.
 *
 * Ask people to provide line coverage and they do. I've seen clever tools
 * which generate test cases to test the existing functions - and by default
 * these tools expect code to be correct and just generate checks which are
 * passing when ran against current code-base. Run this generator and you'll get
 * tests that do not test code is correct but just verify nothing changes.
 * Problem is that testing working code is pointless. And if it is not
 * working, your test must not assume it is working. You won't catch any bugs
 * by such tests. What you can do is to generate a huge amount of tests.
 * Especially if you were are asked to proivde 100% line-coverage x_x. So what
 * does these tests - which are not finding any bugs now - do?
 *
 * They add inertia to every future development. I think it was Terry Pratchet
 * who wrote someone having same impact as thick syrup has to chronometre.
 * Excessive amount of unit-tests have this effect to development. If you do
 * actually find _any_ bug from code in such environment and try fixing it...
 * ...chances are you also need to fix the test cases. In sunny day you fix one
 * test. But I've done refactoring which resulted 500+ broken tests (which had
 * really zero value other than proving to managers that we do do "quality")...
 *
 * After this being said - there are situations where UTs can be handy. If you
 * have algorithms which take some input and should produce output - then you
 * can implement few, carefully selected simple UT-cases which test this. I've
 * previously used this for example for netlink and device-tree data parsing
 * functions. Feed some data examples to functions and verify the output is as
 * expected. I am not covering all the cases but I will see the logic should be
 * working.
 *
 * Here we also do some minor testing. I don't want to go through all branches
 * or test more or less obvious things - but I want to see the main logic is
 * working. And I definitely don't want to add 500+ test cases that break when
 * some simple fix is done x_x. So - let's only add few, well selected tests
 * which ensure as much logic is good as possible.
 */

#[repr(C)]
pub struct linear_range {
    pub min: u32,
    pub min_sel: u32,
    pub max_sel: u32,
    pub step: u32,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    fn linear_range_get_value_array(
        range: *const linear_range,
        num_ranges: usize,
        selector: u32,
        value: *mut u32,
    ) -> i32;
    fn linear_range_get_selector_high(
        range: *const linear_range,
        value: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
    fn linear_range_values_in_range_array(range: *const linear_range, num_ranges: usize) -> i32;
    fn linear_range_get_selector_low_array(
        range: *const linear_range,
        num_ranges: usize,
        value: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
}

pub const RANGE1_MIN: u32 = 10;
pub const RANGE1_MIN_SEL: u32 = 2;
pub const RANGE1_STEP: u32 = 10;
pub static RANGE1_SELS: [u32; 5] = [2, 3, 4, 5, 6];
pub static RANGE1_VALS: [u32; 5] = [10, 20, 30, 40, 50];

pub const RANGE2_MIN: u32 = 100;
pub const RANGE2_MIN_SEL: u32 = 7;
pub const RANGE2_STEP: u32 = 50;
pub static RANGE2_SELS: [u32; 4] = [7, 8, 9, 10];
pub static RANGE2_VALS: [u32; 4] = [100, 150, 200, 250];

pub const RANGE1_NUM_VALS: usize = RANGE1_VALS.len();
pub const RANGE2_NUM_VALS: usize = RANGE2_VALS.len();
pub const RANGE_NUM_VALS: usize = RANGE1_NUM_VALS + RANGE2_NUM_VALS;
pub const RANGE1_MAX_SEL: u32 = RANGE1_MIN_SEL + RANGE1_NUM_VALS as u32 - 1;
pub const RANGE1_MAX_VAL: u32 = RANGE1_VALS[RANGE1_NUM_VALS - 1];
pub const RANGE2_MAX_SEL: u32 = RANGE2_MIN_SEL + RANGE2_NUM_VALS as u32 - 1;
pub const RANGE2_MAX_VAL: u32 = RANGE2_VALS[RANGE2_NUM_VALS - 1];
pub const SMALLEST_SEL: u32 = RANGE1_MIN_SEL;
pub const SMALLEST_VAL: u32 = RANGE1_MIN;

pub static mut TESTR: [linear_range; 2] = [
    linear_range { min: RANGE1_MIN, min_sel: RANGE1_MIN_SEL, max_sel: RANGE1_MAX_SEL, step: RANGE1_STEP },
    linear_range { min: RANGE2_MIN, min_sel: RANGE2_MIN_SEL, max_sel: RANGE2_MAX_SEL, step: RANGE2_STEP },
];

pub unsafe fn range_test_get_value(test: *mut kunit) {
    let mut val: u32 = 0;
    for i in 0..RANGE1_NUM_VALS {
        let sel = RANGE1_SELS[i];
        let ret = linear_range_get_value_array(TESTR.as_ptr(), 2, sel, &mut val);
        KUNIT_EXPECT_EQ!(test, 0, ret);
        KUNIT_EXPECT_EQ!(test, val, RANGE1_VALS[i]);
    }
    for i in 0..RANGE2_NUM_VALS {
        let sel = RANGE2_SELS[i];
        let ret = linear_range_get_value_array(TESTR.as_ptr(), 2, sel, &mut val);
        KUNIT_EXPECT_EQ!(test, 0, ret);
        KUNIT_EXPECT_EQ!(test, val, RANGE2_VALS[i]);
    }
    let ret = linear_range_get_value_array(TESTR.as_ptr(), 2, RANGE2_SELS[RANGE2_NUM_VALS - 1] + 1, &mut val);
    KUNIT_EXPECT_NE!(test, 0, ret);
}

pub unsafe fn range_test_get_selector_high(test: *mut kunit) {
    let mut sel = 0;
    let mut found = false;
    for i in 0..RANGE1_NUM_VALS {
        let ret = linear_range_get_selector_high(TESTR.as_ptr(), RANGE1_VALS[i], &mut sel, &mut found);
        KUNIT_EXPECT_EQ!(test, 0, ret);
        KUNIT_EXPECT_EQ!(test, sel, RANGE1_SELS[i]);
        KUNIT_EXPECT_TRUE!(test, found);
    }
    let ret = linear_range_get_selector_high(TESTR.as_ptr(), RANGE1_MAX_VAL + 1, &mut sel, &mut found);
    KUNIT_EXPECT_LE!(test, ret, 0);
    let ret = linear_range_get_selector_high(TESTR.as_ptr(), RANGE1_MIN - 1, &mut sel, &mut found);
    KUNIT_EXPECT_EQ!(test, 0, ret);
    KUNIT_EXPECT_FALSE!(test, found);
    KUNIT_EXPECT_EQ!(test, sel, RANGE1_SELS[0]);
}

pub unsafe fn range_test_get_value_amount(test: *mut kunit) {
    let ret = linear_range_values_in_range_array(TESTR.as_ptr(), 2);
    KUNIT_EXPECT_EQ!(test, RANGE_NUM_VALS as i32, ret);
}

pub unsafe fn range_test_get_selector_low(test: *mut kunit) {
    let mut sel = 0;
    let mut found = false;
    for i in 0..RANGE1_NUM_VALS {
        let ret = linear_range_get_selector_low_array(TESTR.as_ptr(), 2, RANGE1_VALS[i], &mut sel, &mut found);
        KUNIT_EXPECT_EQ!(test, 0, ret);
        KUNIT_EXPECT_EQ!(test, sel, RANGE1_SELS[i]);
        KUNIT_EXPECT_TRUE!(test, found);
    }
    for i in 0..RANGE2_NUM_VALS {
        let ret = linear_range_get_selector_low_array(TESTR.as_ptr(), 2, RANGE2_VALS[i], &mut sel, &mut found);
        KUNIT_EXPECT_EQ!(test, 0, ret);
        KUNIT_EXPECT_EQ!(test, sel, RANGE2_SELS[i]);
        KUNIT_EXPECT_TRUE!(test, found);
    }
    /* Seek value greater than range max: return Ok but set found false. */
    let ret = linear_range_get_selector_low_array(TESTR.as_ptr(), 2, RANGE2_VALS[RANGE2_NUM_VALS - 1] + 1, &mut sel, &mut found);
    KUNIT_EXPECT_EQ!(test, 0, ret);
    KUNIT_EXPECT_EQ!(test, sel, RANGE2_SELS[RANGE2_NUM_VALS - 1]);
    KUNIT_EXPECT_FALSE!(test, found);
}

// KUnit case array, suite registration, and module metadata are provided by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
