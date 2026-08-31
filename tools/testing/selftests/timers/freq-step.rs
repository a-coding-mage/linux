// SPDX-License-Identifier: GPL-2.0-only
/*
 * This test checks the response of the system clock to frequency
 * steps made with adjtimex(). The frequency error and stability of
 * the CLOCK_MONOTONIC clock relative to the CLOCK_MONOTONIC_RAW clock
 * is measured in two intervals following the step. The test fails if
 * values from the second interval exceed specified limits.
 *
 * Copyright (C) Miroslav Lichvar <mlichvar@redhat.com>  2017
 */

use core::ffi::{c_char, c_double, c_int, c_long};

const SAMPLES: usize = 100;
const SAMPLE_READINGS: c_int = 10;
const MEAN_SAMPLE_INTERVAL: c_double = 0.1;
const STEP_INTERVAL: c_double = 1.0;
const MAX_PRECISION: c_double = 500e-9;
const MAX_FREQ_ERROR: c_double = 0.02e-6;
const MAX_STDDEV: c_double = 50e-9;

// Fallback from the original C source for systems where ADJ_SETOFFSET is absent.
const ADJ_SETOFFSET: c_int = 0x0100;

const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_MONOTONIC_RAW: c_int = 4;
const ADJ_FREQUENCY: c_int = 0x0002;
const ADJ_TICK: c_int = 0x4000;
const _SC_CLK_TCK: c_int = 2;

type time_t = c_long;
type suseconds_t = c_long;

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct timex {
    modes: c_int,
    offset: c_long,
    freq: c_long,
    maxerror: c_long,
    esterror: c_long,
    status: c_int,
    constant: c_long,
    precision: c_long,
    tolerance: c_long,
    time: timeval,
    tick: c_long,
    ppsfreq: c_long,
    jitter: c_long,
    shift: c_int,
    stabil: c_long,
    jitcnt: c_long,
    calcnt: c_long,
    errcnt: c_long,
    stbcnt: c_long,
    tai: c_int,
    __unused: [c_int; 11],
}

#[repr(C)]
struct sample {
    offset: c_double,
    time: c_double,
}

static mut mono_raw_base: time_t = 0;
static mut mono_base: time_t = 0;
static mut user_hz: c_long = 0;
static mut precision: c_double = 0.0;
static mut mono_freq_offset: c_double = 0.0;

unsafe extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn adjtimex(buf: *mut timex) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn usleep(usec: u32) -> c_int;
    fn rand() -> c_int;
    fn srand(seed: u32);
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
    fn ksft_exit_skip(format: *const c_char, ...) -> !;
}

unsafe fn diff_timespec(ts1: *mut timespec, ts2: *mut timespec) -> c_double {
    unsafe {
        ((*ts1).tv_sec - (*ts2).tv_sec) as c_double
            + ((*ts1).tv_nsec - (*ts2).tv_nsec) as c_double / 1e9
    }
}

unsafe fn get_sample(sample: *mut sample) -> c_double {
    let mut delay: c_double;
    let mut mindelay: c_double = 0.0;
    let mut ts1 = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts2 = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts3 = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: c_int = 0;

    unsafe {
        while i < SAMPLE_READINGS {
            clock_gettime(CLOCK_MONOTONIC_RAW, &mut ts1);
            clock_gettime(CLOCK_MONOTONIC, &mut ts2);
            clock_gettime(CLOCK_MONOTONIC_RAW, &mut ts3);

            ts1.tv_sec -= mono_raw_base;
            ts2.tv_sec -= mono_base;
            ts3.tv_sec -= mono_raw_base;

            delay = diff_timespec(&mut ts3, &mut ts1);
            if delay <= 1e-9 {
                i -= 1;
                i += 1;
                continue;
            }

            if i == 0 || delay < mindelay {
                (*sample).offset = diff_timespec(&mut ts2, &mut ts1);
                (*sample).offset -= delay / 2.0;
                (*sample).time = ts1.tv_sec as c_double + ts1.tv_nsec as c_double / 1e9;
                mindelay = delay;
            }

            i += 1;
        }
    }

    mindelay
}

unsafe fn reset_ntp_error() {
    let mut txc = timex {
        modes: 0,
        offset: 0,
        freq: 0,
        maxerror: 0,
        esterror: 0,
        status: 0,
        constant: 0,
        precision: 0,
        tolerance: 0,
        time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        tick: 0,
        ppsfreq: 0,
        jitter: 0,
        shift: 0,
        stabil: 0,
        jitcnt: 0,
        calcnt: 0,
        errcnt: 0,
        stbcnt: 0,
        tai: 0,
        __unused: [0; 11],
    };

    txc.modes = ADJ_SETOFFSET;
    txc.time.tv_sec = 0;
    txc.time.tv_usec = 0;

    unsafe {
        if adjtimex(&mut txc) < 0 {
            perror(c"[FAIL] adjtimex".as_ptr());
            ksft_exit_fail();
        }
    }
}

unsafe fn set_frequency(freq: c_double) {
    let mut txc = timex {
        modes: 0,
        offset: 0,
        freq: 0,
        maxerror: 0,
        esterror: 0,
        status: 0,
        constant: 0,
        precision: 0,
        tolerance: 0,
        time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        tick: 0,
        ppsfreq: 0,
        jitter: 0,
        shift: 0,
        stabil: 0,
        jitcnt: 0,
        calcnt: 0,
        errcnt: 0,
        stbcnt: 0,
        tai: 0,
        __unused: [0; 11],
    };
    let tick_offset: c_int;

    unsafe {
        tick_offset = (1e6 * freq / user_hz as c_double) as c_int;

        txc.modes = ADJ_TICK | ADJ_FREQUENCY;
        txc.tick = 1000000 / user_hz + tick_offset as c_long;
        txc.freq = ((1e6 * freq - user_hz as c_double * tick_offset as c_double)
            * (1 << 16) as c_double) as c_long;

        if adjtimex(&mut txc) < 0 {
            perror(c"[FAIL] adjtimex".as_ptr());
            ksft_exit_fail();
        }
    }
}

unsafe fn regress(
    samples: *mut sample,
    n: c_int,
    intercept: *mut c_double,
    slope: *mut c_double,
    r_stddev: *mut c_double,
    r_max: *mut c_double,
) {
    let mut x: c_double;
    let mut y: c_double;
    let mut r: c_double;
    let mut x_sum: c_double;
    let mut y_sum: c_double;
    let mut xy_sum: c_double;
    let mut x2_sum: c_double;
    let mut r2_sum: c_double;
    let mut i: c_int;

    x_sum = 0.0;
    y_sum = 0.0;
    xy_sum = 0.0;
    x2_sum = 0.0;

    i = 0;
    unsafe {
        while i < n {
            x = (*samples.add(i as usize)).time;
            y = (*samples.add(i as usize)).offset;

            x_sum += x;
            y_sum += y;
            xy_sum += x * y;
            x2_sum += x * x;

            i += 1;
        }

        *slope = (xy_sum - x_sum * y_sum / n as c_double)
            / (x2_sum - x_sum * x_sum / n as c_double);
        *intercept = (y_sum - *slope * x_sum) / n as c_double;

        *r_max = 0.0;
        r2_sum = 0.0;

        i = 0;
        while i < n {
            x = (*samples.add(i as usize)).time;
            y = (*samples.add(i as usize)).offset;
            r = (x * *slope + *intercept - y).abs();
            if *r_max < r {
                *r_max = r;
            }
            r2_sum += r * r;

            i += 1;
        }

        *r_stddev = (r2_sum / n as c_double).sqrt();
    }
}

unsafe fn run_test(calibration: c_int, freq_base: c_double, freq_step: c_double) -> c_int {
    let mut samples = [const { sample {
        offset: 0.0,
        time: 0.0,
    } }; SAMPLES];
    let mut intercept: c_double = 0.0;
    let mut slope: c_double = 0.0;
    let mut stddev1: c_double = 0.0;
    let mut max1: c_double = 0.0;
    let mut stddev2: c_double = 0.0;
    let mut max2: c_double = 0.0;
    let freq_error1: c_double;
    let freq_error2: c_double;
    let mut i: c_int;

    unsafe {
        set_frequency(freq_base);

        i = 0;
        while i < 10 {
            usleep((1e6 * MEAN_SAMPLE_INTERVAL / 10.0) as u32);
            i += 1;
        }

        reset_ntp_error();

        set_frequency(freq_base + freq_step);

        i = 0;
        while i < 10 {
            usleep(((rand() % 2000000) as c_double * STEP_INTERVAL / 10.0) as u32);
            i += 1;
        }

        set_frequency(freq_base);

        i = 0;
        while i < SAMPLES as c_int {
            usleep(((rand() % 2000000) as c_double * MEAN_SAMPLE_INTERVAL) as u32);
            get_sample(&mut samples[i as usize]);
            i += 1;
        }

        if calibration != 0 {
            regress(
                samples.as_mut_ptr(),
                SAMPLES as c_int,
                &mut intercept,
                &mut slope,
                &mut stddev1,
                &mut max1,
            );
            mono_freq_offset = slope;
            printf(
                c"CLOCK_MONOTONIC_RAW frequency offset: %11.3f ppm\n".as_ptr(),
                1e6 * mono_freq_offset,
            );
            return 0;
        }

        regress(
            samples.as_mut_ptr(),
            (SAMPLES / 2) as c_int,
            &mut intercept,
            &mut slope,
            &mut stddev1,
            &mut max1,
        );
        freq_error1 = slope * (1.0 - mono_freq_offset) - mono_freq_offset - freq_base;

        regress(
            samples.as_mut_ptr().add(SAMPLES / 2),
            (SAMPLES / 2) as c_int,
            &mut intercept,
            &mut slope,
            &mut stddev2,
            &mut max2,
        );
        freq_error2 = slope * (1.0 - mono_freq_offset) - mono_freq_offset - freq_base;

        printf(
            c"%6.0f %+10.3f %6.0f %7.0f %+10.3f %6.0f %7.0f\t".as_ptr(),
            1e6 * freq_step,
            1e6 * freq_error1,
            1e9 * stddev1,
            1e9 * max1,
            1e6 * freq_error2,
            1e9 * stddev2,
            1e9 * max2,
        );

        if freq_error2.abs() > MAX_FREQ_ERROR || stddev2 > MAX_STDDEV {
            printf(c"[FAIL]\n".as_ptr());
            return 1;
        }

        printf(c"[OK]\n".as_ptr());
    }

    0
}

unsafe fn init_test() {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut sample = sample {
        offset: 0.0,
        time: 0.0,
    };

    unsafe {
        if clock_gettime(CLOCK_MONOTONIC_RAW, &mut ts) != 0 {
            perror(c"[FAIL] clock_gettime(CLOCK_MONOTONIC_RAW)".as_ptr());
            ksft_exit_fail();
        }

        mono_raw_base = ts.tv_sec;

        if clock_gettime(CLOCK_MONOTONIC, &mut ts) != 0 {
            perror(c"[FAIL] clock_gettime(CLOCK_MONOTONIC)".as_ptr());
            ksft_exit_fail();
        }

        mono_base = ts.tv_sec;

        user_hz = sysconf(_SC_CLK_TCK);

        precision = get_sample(&mut sample) / 2.0;
        printf(
            c"CLOCK_MONOTONIC_RAW+CLOCK_MONOTONIC precision: %.0f ns\t\t".as_ptr(),
            1e9 * precision,
        );

        if precision > MAX_PRECISION {
            ksft_exit_skip(
                c"precision: %.0f ns > MAX_PRECISION: %.0f ns\n".as_ptr(),
                1e9 * precision,
                1e9 * MAX_PRECISION,
            );
        }

        printf(c"[OK]\n".as_ptr());
        srand((ts.tv_sec ^ ts.tv_nsec) as u32);

        run_test(1, 0.0, 0.0);
    }
}

fn main() {
    let mut freq_base: c_double;
    let mut freq_step: c_double;
    let mut i: c_int;
    let mut j: c_int;
    let mut fails: c_int = 0;

    unsafe {
        init_test();

        printf(c"Checking response to frequency step:\n".as_ptr());
        printf(c"  Step           1st interval              2nd interval\n".as_ptr());
        printf(c"             Freq    Dev     Max       Freq    Dev     Max\n".as_ptr());

        i = 2;
        while i >= 0 {
            j = 0;
            while j < 5 {
                freq_base = (rand() % (1 << 24) - (1 << 23)) as c_double / 65536e6;
                freq_step = 10e-6 * (1 << (6 * i)) as c_double;
                fails += run_test(0, freq_base, freq_step);
                j += 1;
            }
            i -= 1;
        }

        set_frequency(0.0);

        if fails != 0 {
            ksft_exit_fail();
        }

        ksft_exit_pass();
    }
}
