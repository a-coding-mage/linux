/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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
 */

// Dependencies supplied by the surrounding AMDGPU translation.
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_smuio_funcs {
    pub get_gpu_clock_counter:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
}

unsafe extern "C" {
    fn preempt_disable();
    fn preempt_enable();

    // RREG32_SOC15 is a C macro; this declaration represents its register-read
    // operation for the direct Rust translation.
    fn RREG32_SOC15(
        adev: *mut amdgpu_device,
        block: u32,
        instance: u32,
        register: u32,
    ) -> u32;
}

// Register-block and register constants are provided by the generated headers.
unsafe extern "C" {
    static SMUIO: u32;
    static regGOLDEN_TSC_COUNT_UPPER: u32;
    static regGOLDEN_TSC_COUNT_LOWER: u32;
}

unsafe extern "C" fn smuio_v15_0_0_get_gpu_clock_counter(
    adev: *mut amdgpu_device,
) -> u64 {
    let clock: u64;
    let mut clock_counter_lo: u64;
    let clock_counter_hi_pre: u64;
    let clock_counter_hi_after: u64;

    unsafe {
        preempt_disable();
        clock_counter_hi_pre =
            RREG32_SOC15(adev, SMUIO, 0, regGOLDEN_TSC_COUNT_UPPER) as u64;
        clock_counter_lo =
            RREG32_SOC15(adev, SMUIO, 0, regGOLDEN_TSC_COUNT_LOWER) as u64;
        /* the clock counter may be udpated during polling the counters */
        clock_counter_hi_after =
            RREG32_SOC15(adev, SMUIO, 0, regGOLDEN_TSC_COUNT_UPPER) as u64;
        if clock_counter_hi_pre != clock_counter_hi_after {
            clock_counter_lo =
                RREG32_SOC15(adev, SMUIO, 0, regGOLDEN_TSC_COUNT_LOWER) as u64;
        }
        preempt_enable();
    }

    clock = clock_counter_lo | (clock_counter_hi_after << 32u64);

    clock
}

pub static smuio_v15_0_0_funcs: amdgpu_smuio_funcs = amdgpu_smuio_funcs {
    get_gpu_clock_counter: Some(smuio_v15_0_0_get_gpu_clock_counter),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
