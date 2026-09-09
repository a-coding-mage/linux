/*
 * Copyright 2008 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 * Copyright 2009 Christian König.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

// Dependencies supplied by the surrounding kernel/DRM translation.

static AMDGPU_AFMT_PREDEFINED_ACR: [amdgpu_afmt_acr; 10] = [
    //        32kHz          44.1kHz       48kHz
    // Clock      N     CTS      N     CTS      N     CTS
    amdgpu_afmt_acr { clock: 25175,  n_32khz: 4096, cts_32khz: 25175, n_44_1khz: 28224, cts_44_1khz: 125875, n_48khz: 6144, cts_48khz: 25175 }, // 25,20/1.001 MHz
    amdgpu_afmt_acr { clock: 25200,  n_32khz: 4096, cts_32khz: 25200, n_44_1khz: 6272,  cts_44_1khz: 28000,  n_48khz: 6144, cts_48khz: 25200 }, // 25.20 MHz
    amdgpu_afmt_acr { clock: 27000,  n_32khz: 4096, cts_32khz: 27000, n_44_1khz: 6272,  cts_44_1khz: 30000,  n_48khz: 6144, cts_48khz: 27000 }, // 27.00 MHz
    amdgpu_afmt_acr { clock: 27027,  n_32khz: 4096, cts_32khz: 27027, n_44_1khz: 6272,  cts_44_1khz: 30030,  n_48khz: 6144, cts_48khz: 27027 }, // 27.00*1.001 MHz
    amdgpu_afmt_acr { clock: 54000,  n_32khz: 4096, cts_32khz: 54000, n_44_1khz: 6272, cts_44_1khz: 60000,  n_48khz: 6144, cts_48khz: 54000 }, // 54.00 MHz
    amdgpu_afmt_acr { clock: 54054,  n_32khz: 4096, cts_32khz: 54054, n_44_1khz: 6272, cts_44_1khz: 60060,  n_48khz: 6144, cts_48khz: 54054 }, // 54.00*1.001 MHz
    amdgpu_afmt_acr { clock: 74176,  n_32khz: 4096, cts_32khz: 74176, n_44_1khz: 5733, cts_44_1khz: 75335,  n_48khz: 6144, cts_48khz: 74176 }, // 74.25/1.001 MHz
    amdgpu_afmt_acr { clock: 74250,  n_32khz: 4096, cts_32khz: 74250, n_44_1khz: 6272, cts_44_1khz: 82500,  n_48khz: 6144, cts_48khz: 74250 }, // 74.25 MHz
    amdgpu_afmt_acr { clock: 148352, n_32khz: 4096, cts_32khz: 148352, n_44_1khz: 5733, cts_44_1khz: 150670, n_48khz: 6144, cts_48khz: 148352 }, // 148.50/1.001 MHz
    amdgpu_afmt_acr { clock: 148500, n_32khz: 4096, cts_32khz: 148500, n_44_1khz: 6272, cts_44_1khz: 165000, n_48khz: 6144, cts_48khz: 148500 }, // 148.50 MHz
];

/* calculate CTS and N values if they are not found in the table */
unsafe fn amdgpu_afmt_calc_cts(clock: u32, cts: *mut i32, n: *mut i32, freq: i32) {
    let mut n_value: i32;
    let mut cts_value: i32;
    let mut div: u64;
    let mut mul: u64;

    /* Safe, but overly large values */
    n_value = 128 * freq;
    cts_value = (clock * 1000) as i32;

    /* Smallest valid fraction */
    div = gcd(n_value as u64, cts_value as u64);

    n_value /= div as i32;
    cts_value /= div as i32;

    /*
     * The optimal N is 128*freq/1000. Calculate the closest larger
     * value that doesn't truncate any bits.
     */
    mul = (((128 * freq / 1000) + (n_value - 1)) / n_value) as u64;

    n_value = (n_value as u64 * mul) as i32;
    cts_value = (cts_value as u64 * mul) as i32;

    /* Check that we are in spec (not always possible) */
    if n_value < (128 * freq / 1500) {
        pr_warn!("Calculated ACR N value is too small. You may experience audio problems.\n");
    }
    if n_value > (128 * freq / 300) {
        pr_warn!("Calculated ACR N value is too large. You may experience audio problems.\n");
    }

    *n = n_value;
    *cts = cts_value;

    drm_debug!("Calculated ACR timing N={} CTS={} for frequency {}\n", *n, *cts, freq);
}

fn amdgpu_afmt_acr(clock: u32) -> amdgpu_afmt_acr {
    let mut res: amdgpu_afmt_acr;

    /* Precalculated values for common clocks */
    for acr in AMDGPU_AFMT_PREDEFINED_ACR.iter() {
        if acr.clock == clock {
            return *acr;
        }
    }

    /* And odd clocks get manually calculated */
    unsafe {
        amdgpu_afmt_calc_cts(clock, &mut res.cts_32khz, &mut res.n_32khz, 32000);
        amdgpu_afmt_calc_cts(clock, &mut res.cts_44_1khz, &mut res.n_44_1khz, 44100);
        amdgpu_afmt_calc_cts(clock, &mut res.cts_48khz, &mut res.n_48khz, 48000);
    }
    res.clock = clock;

    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
