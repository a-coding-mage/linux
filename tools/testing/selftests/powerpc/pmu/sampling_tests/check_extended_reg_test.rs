// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024, Kajol Jain, IBM Corp.
 */

/* Dependencies from:
 * ../event.h
 * misc.h
 * utils.h
 */

unsafe extern "C" {
    static PPC_FEATURE2_ARCH_3_00: u64;
    static mut platform_extended_mask: u64;

    fn have_hwcap2(feature: u64) -> i32;
    fn check_for_generic_compat_pmu() -> i32;
    fn perf_get_platform_reg_mask() -> u64;
    fn check_extended_regs_support() -> i32;
    fn test_harness(test: Option<unsafe extern "C" fn() -> i32>, name: *const i8) -> i32;

    /*
     * C preprocessor macros supplied externally. They are represented here as
     * declarations to preserve the source-level dependency and call ordering.
     */
    fn SKIP_IF(cond: i32);
    fn FAIL_IF(cond: i32);
}

/*
 * A perf sampling test to check extended
 * reg support.
 */
unsafe extern "C" fn check_extended_reg_test() -> i32 {
    /* Check for platform support for the test */
    unsafe {
        SKIP_IF((have_hwcap2(PPC_FEATURE2_ARCH_3_00) == 0) as i32);
    }

    /* Skip for Generic compat PMU */
    unsafe {
        SKIP_IF(check_for_generic_compat_pmu());
    }

    /* Check if platform supports extended regs */
    unsafe {
        platform_extended_mask = perf_get_platform_reg_mask();
        FAIL_IF(check_extended_regs_support());
    }

    0
}

fn main() {
    let name = b"check_extended_reg_test\0";

    unsafe {
        std::process::exit(test_harness(
            Some(check_extended_reg_test),
            name.as_ptr() as *const i8,
        ));
    }
}
