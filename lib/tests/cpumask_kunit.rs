// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for cpumask.
 *
 * Author: Sander Vanheule <sander@svanheule.net>
 */

// Dependencies supplied by the kernel and KUnit headers:
// kunit/test.h, linux/cpu.h, linux/cpumask.h

macro_rules! mask_msg {
    ($m:expr) => {
        format!(
            "{} contains {}CPUs %*pbl",
            stringify!($m),
            if cpumask_weight($m) != 0 { "" } else { "no " },
        )
    };
}

macro_rules! expect_for_each_cpu_eq {
    ($test:expr, $mask:expr) => {{
        let m = $mask;
        let mask_weight = cpumask_weight(m);
        let mut iter = 0;
        for_each_cpu!(cpu, m, {
            let _ = cpu;
            iter += 1;
        });
        kunit_expect_eq_msg!($test, mask_weight, iter, mask_msg!($mask));
    }};
}

macro_rules! expect_for_each_cpu_op_eq {
    ($test:expr, $op:ident, $mask1:expr, $mask2:expr) => {{
        let m1 = $mask1;
        let m2 = $mask2;
        let mut iter = 0;
        cpumask_$op!(&mut MASK_TMP, m1, m2);
        let weight = cpumask_weight(&MASK_TMP);
        for_each_cpu_$op!(cpu, $mask1, $mask2, {
            let _ = cpu;
            iter += 1;
        });
        kunit_expect_eq!($test, weight, iter);
    }};
}

macro_rules! expect_for_each_cpu_wrap_eq {
    ($test:expr, $mask:expr) => {{
        let m = $mask;
        let mask_weight = cpumask_weight(m);
        let mut iter = 0;
        for_each_cpu_wrap!(cpu, m, nr_cpu_ids / 2, {
            let _ = cpu;
            iter += 1;
        });
        kunit_expect_eq_msg!($test, mask_weight, iter, mask_msg!($mask));
    }};
}

macro_rules! expect_for_each_cpu_builtin_eq {
    ($test:expr, $name:ident) => {{
        let mask_weight = num_$name##_cpus();
        let mut iter = 0;
        for_each_$name##_cpu!(cpu, {
            let _ = cpu;
            iter += 1;
        });
        kunit_expect_eq_msg!($test, mask_weight, iter, mask_msg!(cpu_$name##_mask));
    }};
}

static mut MASK_EMPTY: cpumask_t = cpumask_t::default();
static mut MASK_ALL: cpumask_t = cpumask_t::default();
static mut MASK_TMP: cpumask_t = cpumask_t::default();

unsafe fn test_cpumask_weight(test: *mut kunit) {
    kunit_expect_true_msg!(test, cpumask_empty(&MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_true_msg!(test, cpumask_full(&MASK_ALL), mask_msg!(&MASK_ALL));

    kunit_expect_eq_msg!(test, 0, cpumask_weight(&MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_eq_msg!(test, nr_cpu_ids, cpumask_weight(cpu_possible_mask), mask_msg!(cpu_possible_mask));
    kunit_expect_eq_msg!(test, nr_cpu_ids, cpumask_weight(&MASK_ALL), mask_msg!(&MASK_ALL));
}

unsafe fn test_cpumask_first(test: *mut kunit) {
    kunit_expect_le_msg!(test, nr_cpu_ids, cpumask_first(&MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_eq_msg!(test, 0, cpumask_first(cpu_possible_mask), mask_msg!(cpu_possible_mask));

    kunit_expect_eq_msg!(test, 0, cpumask_first_zero(&MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_le_msg!(test, nr_cpu_ids, cpumask_first_zero(cpu_possible_mask), mask_msg!(cpu_possible_mask));
}

unsafe fn test_cpumask_last(test: *mut kunit) {
    kunit_expect_le_msg!(test, nr_cpumask_bits, cpumask_last(&MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_eq_msg!(test, nr_cpu_ids - 1, cpumask_last(cpu_possible_mask), mask_msg!(cpu_possible_mask));
}

unsafe fn test_cpumask_next(test: *mut kunit) {
    kunit_expect_eq_msg!(test, 0, cpumask_next_zero(-1, &MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_le_msg!(test, nr_cpu_ids, cpumask_next_zero(-1, cpu_possible_mask), mask_msg!(cpu_possible_mask));
    kunit_expect_le_msg!(test, nr_cpu_ids, cpumask_next(-1, &MASK_EMPTY), mask_msg!(&MASK_EMPTY));
    kunit_expect_eq_msg!(test, 0, cpumask_next(-1, cpu_possible_mask), mask_msg!(cpu_possible_mask));
}

unsafe fn test_cpumask_iterators(test: *mut kunit) {
    expect_for_each_cpu_eq!(test, &MASK_EMPTY);
    expect_for_each_cpu_wrap_eq!(test, &MASK_EMPTY);
    expect_for_each_cpu_op_eq!(test, and, &MASK_EMPTY, &MASK_EMPTY);
    expect_for_each_cpu_op_eq!(test, and, cpu_possible_mask, &MASK_EMPTY);
    expect_for_each_cpu_op_eq!(test, andnot, &MASK_EMPTY, &MASK_EMPTY);
    expect_for_each_cpu_eq!(test, cpu_possible_mask);
    expect_for_each_cpu_wrap_eq!(test, cpu_possible_mask);
    expect_for_each_cpu_op_eq!(test, and, cpu_possible_mask, cpu_possible_mask);
    expect_for_each_cpu_op_eq!(test, andnot, cpu_possible_mask, &MASK_EMPTY);
}

unsafe fn test_cpumask_iterators_builtin(test: *mut kunit) {
    expect_for_each_cpu_builtin_eq!(test, possible);
    // Ensure the dynamic masks are stable while running the tests
    cpu_hotplug_disable();
    expect_for_each_cpu_builtin_eq!(test, online);
    expect_for_each_cpu_builtin_eq!(test, present);
    cpu_hotplug_enable();
}

unsafe fn test_cpumask_init(_test: *mut kunit) -> i32 {
    cpumask_clear(&mut MASK_EMPTY);
    cpumask_setall(&mut MASK_ALL);
    0
}

static mut TEST_CPUMASK_CASES: [kunit_case; 7] = [
    kunit_case!(test_cpumask_weight),
    kunit_case!(test_cpumask_first),
    kunit_case!(test_cpumask_last),
    kunit_case!(test_cpumask_next),
    kunit_case!(test_cpumask_iterators),
    kunit_case!(test_cpumask_iterators_builtin),
    kunit_case!(),
];

static mut TEST_CPUMASK_SUITE: kunit_suite = kunit_suite {
    name: "cpumask",
    init: Some(test_cpumask_init),
    test_cases: TEST_CPUMASK_CASES.as_ptr(),
};

kunit_test_suite!(TEST_CPUMASK_SUITE);

module_description!("KUnit tests for cpumask");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
