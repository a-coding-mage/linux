// SPDX-License-Identifier: GPL-2.0-or-later
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

// Dependencies from: stdio.h, unistd.h, math.h, config.h, system.h, benchmark.h

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config {
    pub output: *mut FILE,
    pub verbose: c_int,
    pub sleep: c_long,
    pub load: c_long,
    pub rounds: c_uint,
    pub cycles: c_uint,
    pub cpu: c_uint,
    pub governor: *const c_char,
    pub sleep_step: c_long,
    pub load_step: c_long,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static GAUGECOUNT: c_uint;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn get_time() -> i64;
    fn set_cpufreq_governor(governor: *const c_char, cpu: c_uint) -> c_int;

    // Macro/function-like dependencies supplied by included project headers.
    fn ROUNDS(rounds: c_uint);
    fn dprintf(format: *const c_char, ...) -> c_int;
}

unsafe fn show_progress(total_time: c_ulong, progress_time: c_ulong, config: *mut config) {
    /* Print out progress if we log into a file */
    if (*config).output != stdout {
        fprintf(
            stdout,
            c"Progress: %02lu %%\r".as_ptr(),
            (progress_time * 100) / total_time,
        );
        fflush(stdout);
    }
}

/**
 * compute how many rounds of calculation we should do
 * to get the given load time
 *
 * @param load aimed load time in µs
 *
 * @retval rounds of calculation
 **/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calculate_timespace(load: c_long, config: *mut config) -> c_uint {
    let mut i: c_int;
    let mut now: i64;
    let mut then: i64;
    let mut estimated: c_uint = GAUGECOUNT;
    let mut rounds: c_uint = 0;
    let mut timed: c_uint;

    if (*config).verbose != 0 {
        printf(c"calibrating load of %lius, please wait...\n".as_ptr(), load);
    }

    /* get the initial calculation time for a specific number of rounds */
    now = get_time();
    ROUNDS(estimated);
    then = get_time();

    timed = (then - now) as c_uint;

    /* approximation of the wanted load time by comparing with the
     * initial calculation time */
    i = 0;
    while i < 4 {
        rounds = (load * estimated as c_long / timed as c_long) as c_uint;
        dprintf(c"calibrating with %u rounds\n".as_ptr(), rounds);
        now = get_time();
        ROUNDS(rounds);
        then = get_time();

        timed = (then - now) as c_uint;
        estimated = rounds;
        i += 1;
    }
    if (*config).verbose != 0 {
        printf(c"calibration done\n".as_ptr());
    }

    estimated
}

/**
 * benchmark
 * generates a specific sleep an load time with the performance
 * governor and compares the used time for same calculations done
 * with the configured powersave governor
 *
 * @param config config values for the benchmark
 *
 **/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_benchmark(config: *mut config) {
    let mut _round: c_uint;
    let mut cycle: c_uint;
    let mut now: i64;
    let mut then: i64;
    let mut sleep_time: c_long = 0;
    let mut load_time: c_long = 0;
    let mut performance_time: c_long = 0;
    let mut powersave_time: c_long = 0;
    let calculations: c_uint;
    let mut total_time: c_ulong = 0;
    let mut progress_time: c_ulong = 0;

    sleep_time = (*config).sleep;
    load_time = (*config).load;

    /* For the progress bar */
    _round = 1;
    while _round <= (*config).rounds {
        total_time += (_round as c_ulong) * (((*config).sleep + (*config).load) as c_ulong);
        _round += 1;
    }
    total_time *= 2; /* powersave and performance cycles */

    _round = 0;
    while _round < (*config).rounds {
        performance_time = 0;
        powersave_time = 0;

        show_progress(total_time, progress_time, config);

        /* set the cpufreq governor to "performance" which disables
         * P-State switching. */
        if set_cpufreq_governor(c"performance".as_ptr(), (*config).cpu) != 0 {
            return;
        }

        /* calibrate the calculation time. the resulting calculation
         * _rounds should produce a load which matches the configured
         * load time */
        calculations = calculate_timespace(load_time, config);

        if (*config).verbose != 0 {
            printf(
                c"_round %i: doing %u cycles with %u calculations for %lius\n".as_ptr(),
                _round + 1,
                (*config).cycles,
                calculations,
                load_time,
            );
        }

        fprintf(
            (*config).output,
            c"%u %li %li ".as_ptr(),
            _round,
            load_time,
            sleep_time,
        );

        if (*config).verbose != 0 {
            printf(
                c"average: %lius, rps:%li\n".as_ptr(),
                load_time / calculations as c_long,
                1000000 * calculations as c_long / load_time,
            );
        }

        /* do some sleep/load cycles with the performance governor */
        cycle = 0;
        while cycle < (*config).cycles {
            now = get_time();
            usleep(sleep_time as c_uint);
            ROUNDS(calculations);
            then = get_time();
            performance_time += (then - now) as c_long - sleep_time;
            if (*config).verbose != 0 {
                printf(
                    c"performance cycle took %lius, sleep: %lius, load: %lius, rounds: %u\n"
                        .as_ptr(),
                    (then - now) as c_long,
                    sleep_time,
                    load_time,
                    calculations,
                );
            }
            cycle += 1;
        }
        fprintf(
            (*config).output,
            c"%li ".as_ptr(),
            performance_time / (*config).cycles as c_long,
        );

        progress_time += (sleep_time + load_time) as c_ulong;
        show_progress(total_time, progress_time, config);

        /* set the powersave governor which activates P-State switching
         * again */
        if set_cpufreq_governor((*config).governor, (*config).cpu) != 0 {
            return;
        }

        /* again, do some sleep/load cycles with the
         * powersave governor */
        cycle = 0;
        while cycle < (*config).cycles {
            now = get_time();
            usleep(sleep_time as c_uint);
            ROUNDS(calculations);
            then = get_time();
            powersave_time += (then - now) as c_long - sleep_time;
            if (*config).verbose != 0 {
                printf(
                    c"powersave cycle took %lius, sleep: %lius, load: %lius, rounds: %u\n"
                        .as_ptr(),
                    (then - now) as c_long,
                    sleep_time,
                    load_time,
                    calculations,
                );
            }
            cycle += 1;
        }

        progress_time += (sleep_time + load_time) as c_ulong;

        /* compare the average sleep/load cycles  */
        fprintf(
            (*config).output,
            c"%li ".as_ptr(),
            powersave_time / (*config).cycles as c_long,
        );
        fprintf(
            (*config).output,
            c"%.3f\n".as_ptr(),
            performance_time as f64 * 100.0 / powersave_time as f64,
        );
        fflush((*config).output);

        if (*config).verbose != 0 {
            printf(
                c"performance is at %.2f%%\n".as_ptr(),
                performance_time as f64 * 100.0 / powersave_time as f64,
            );
        }

        sleep_time += (*config).sleep_step;
        load_time += (*config).load_step;
        _round += 1;
    }
}
