// SPDX-License-Identifier: GPL-2.0

use std::os::raw::{c_char, c_int, c_ulong, c_uint};

// C dependencies removed from executable Rust:
// stdio.h, errno.h, stdlib.h, string.h, math.h
// helpers/helpers.h, cpufreq.h, acpi_cppc.h

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn print_speed(speed: i64, no_rounding: c_int);
    fn acpi_cppc_get_data(cpu: c_uint, which: c_int) -> i64;

    fn _(msgid: *const c_char) -> *const c_char;

    static NOMINAL_PERF: c_int;
    static NOMINAL_FREQ: c_int;
    static LOWEST_PERF: c_int;
    static LOWEST_FREQ: c_int;
    static LOWEST_NONLINEAR_PERF: c_int;
    static HIGHEST_PERF: c_int;
}

fn cppc_to_frequency(perf: c_ulong, slope: f32, intercept: f32) -> i64 {
    (slope * perf as f32 + intercept).round() as i64
}

#[no_mangle]
pub unsafe extern "C" fn cppc_show_perf_and_freq(cpu: c_uint, no_rounding: c_int) {
    let nominal: i64 = unsafe { acpi_cppc_get_data(cpu, NOMINAL_PERF) };
    let nominal_freq: i64 = unsafe { acpi_cppc_get_data(cpu, NOMINAL_FREQ) * 1000 };
    let lowest: i64 = unsafe { acpi_cppc_get_data(cpu, LOWEST_PERF) };
    let lowest_freq: i64 = unsafe { acpi_cppc_get_data(cpu, LOWEST_FREQ) * 1000 };
    let non_linear: c_ulong = unsafe { acpi_cppc_get_data(cpu, LOWEST_NONLINEAR_PERF) as c_ulong };
    let highest: c_ulong = unsafe { acpi_cppc_get_data(cpu, HIGHEST_PERF) as c_ulong };
    let slope: f32;
    let intercept: f32;

    /* do the optional freq fields look invalid? */
    if nominal_freq == 0 || lowest_freq == 0 || nominal == lowest {
        return;
    }

    slope = (nominal_freq - lowest_freq) as f32 / (nominal - lowest) as f32;
    intercept = lowest_freq as f32 - slope * lowest as f32;

    unsafe {
        printf(_(b"  CPPC limits:\n\0".as_ptr() as *const c_char));
        printf(
            _(b"    Highest Performance: %lu. Maximum Frequency: \0".as_ptr() as *const c_char),
            highest,
        );
        /*
         * If boost isn't active, the cpuinfo_max doesn't indicate real max
         * frequency.
         */
        print_speed(cppc_to_frequency(highest, slope, intercept), no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            _(b"    Nominal Performance: %lu. Nominal Frequency: \0".as_ptr() as *const c_char),
            acpi_cppc_get_data(cpu, NOMINAL_PERF) as c_ulong,
        );
        print_speed(nominal_freq, no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            _(
                b"    Lowest Non-linear Performance: %lu. Lowest Non-linear Frequency: \0"
                    .as_ptr() as *const c_char,
            ),
            non_linear,
        );
        print_speed(cppc_to_frequency(non_linear, slope, intercept), no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            _(b"    Lowest Performance: %lu. Lowest Frequency: \0".as_ptr() as *const c_char),
            acpi_cppc_get_data(cpu, LOWEST_PERF) as c_ulong,
        );
        print_speed(lowest_freq, no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);
    }
}
