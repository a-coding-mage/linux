/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

/* load loop, this schould take about 1 to 2ms to complete */
macro_rules! ROUNDS {
    ($x:expr) => {{
        let mut rcnt: ::std::ffi::c_uint = 0;
        while rcnt < (($x).wrapping_mul(1000)) {
            let _ = (((rcnt as f64).powf(rcnt as f64)
                * ((rcnt.wrapping_mul(7230970)) as f64).sqrt()) as ::std::ffi::c_int
                ^ 7230716)
                ^ ((rcnt as f64).atan2(rcnt as f64) as ::std::ffi::c_int);
            rcnt = rcnt.wrapping_add(1);
        }
    }};
}

#[repr(C)]
pub struct config {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn start_benchmark(config: *mut config);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
