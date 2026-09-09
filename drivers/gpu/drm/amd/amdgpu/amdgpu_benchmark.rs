/*
 * Copyright 2009 Jerome Glisse.
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
 * Authors: Jerome Glisse
 */

// Dependencies supplied by the surrounding kernel/amdgpu translation unit.

const AMDGPU_BENCHMARK_ITERATIONS: i32 = 1024;
const AMDGPU_BENCHMARK_COMMON_MODES_N: usize = 17;

unsafe fn amdgpu_benchmark_do_move(
    adev: *mut amdgpu_device,
    size: u32,
    saddr: u64,
    daddr: u64,
    n: i32,
    time_ms: *mut s64,
) -> i32 {
    let stime: ktime_t;
    let mut etime: ktime_t;
    let mut fence: *mut dma_fence = core::ptr::null_mut();
    let mut i: i32;
    let mut r: i32;

    mutex_lock(&mut (*(*adev).mman.default_entity).lock);
    stime = ktime_get();
    i = 0;
    while i < n {
        r = amdgpu_copy_buffer(
            adev,
            &mut (*adev).mman.default_entity,
            saddr,
            daddr,
            size,
            core::ptr::null_mut(),
            &mut fence,
            false,
            0,
        );
        if r != 0 {
            break;
        }
        r = dma_fence_wait(fence, false);
        dma_fence_put(fence);
        if r != 0 {
            break;
        }
        i += 1;
    }

    mutex_unlock(&mut (*(*adev).mman.default_entity).lock);
    etime = ktime_get();
    *time_ms = ktime_ms_delta(etime, stime);

    r
}

unsafe fn amdgpu_benchmark_log_results(
    adev: *mut amdgpu_device,
    n: i32,
    size: u32,
    time_ms: s64,
    sdomain: u32,
    ddomain: u32,
    kind: *mut i8,
) {
    let mut throughput: s64 = (n as s64) * ((size >> 10) as s64);

    throughput = div64_s64(throughput, time_ms);

    dev_info(
        (*adev).dev,
        b" %s %u bo moves of %u kB from %d to %d in %lld ms, throughput: %lld Mb/s or %lld MB/s\n\0".as_ptr() as *const i8,
        kind,
        n,
        size >> 10,
        sdomain,
        ddomain,
        time_ms,
        throughput * 8,
        throughput,
    );
}

unsafe fn amdgpu_benchmark_move(
    adev: *mut amdgpu_device,
    size: u32,
    sdomain: u32,
    ddomain: u32,
) -> i32 {
    let mut dobj: *mut amdgpu_bo = core::ptr::null_mut();
    let mut sobj: *mut amdgpu_bo = core::ptr::null_mut();
    let mut saddr: u64 = 0;
    let mut daddr: u64 = 0;
    let mut time_ms: s64 = 0;
    let n: i32 = AMDGPU_BENCHMARK_ITERATIONS;
    let mut r: i32;

    r = amdgpu_bo_create_kernel(adev, size, PAGE_SIZE, sdomain, &mut sobj, &mut saddr, core::ptr::null_mut());
    if r != 0 {
        goto_out_cleanup!(r, adev, sobj, dobj, saddr, daddr);
    }
    r = amdgpu_bo_create_kernel(adev, size, PAGE_SIZE, ddomain, &mut dobj, &mut daddr, core::ptr::null_mut());
    if r != 0 {
        goto_out_cleanup!(r, adev, sobj, dobj, saddr, daddr);
    }

    if !(*adev).mman.buffer_funcs.is_null() {
        r = amdgpu_benchmark_do_move(adev, size, saddr, daddr, n, &mut time_ms);
        if r != 0 {
            goto_out_cleanup!(r, adev, sobj, dobj, saddr, daddr);
        } else {
            amdgpu_benchmark_log_results(adev, n, size, time_ms, sdomain, ddomain, b"dma\0".as_ptr() as *mut i8);
        }
    }

    // Check error value now. The value can be overwritten when clean up.
    if r < 0 {
        dev_info((*adev).dev, b"Error while benchmarking BO move.\n\0".as_ptr() as *const i8);
    }
    if !sobj.is_null() { amdgpu_bo_free_kernel(&mut sobj, &mut saddr, core::ptr::null_mut()); }
    if !dobj.is_null() { amdgpu_bo_free_kernel(&mut dobj, &mut daddr, core::ptr::null_mut()); }
    r
}

// The macro preserves the C goto cleanup path; it is supplied by the translation environment.
unsafe fn amdgpu_benchmark(adev: *mut amdgpu_device, test_number: i32) -> i32 {
    let common_modes: [i32; AMDGPU_BENCHMARK_COMMON_MODES_N] = [
        640 * 480 * 4, 720 * 480 * 4, 800 * 600 * 4, 848 * 480 * 4,
        1024 * 768 * 4, 1152 * 768 * 4, 1280 * 720 * 4, 1280 * 800 * 4,
        1280 * 854 * 4, 1280 * 960 * 4, 1280 * 1024 * 4, 1440 * 900 * 4,
        1400 * 1050 * 4, 1680 * 1050 * 4, 1600 * 1200 * 4, 1920 * 1080 * 4,
        1920 * 1200 * 4,
    ];
    let mut r: i32 = 0;
    mutex_lock(&mut (*adev).benchmark_mutex);
    match test_number {
        1 => { dev_info((*adev).dev, b"benchmark test: %d (simple test, VRAM to GTT and GTT to VRAM)\n\0".as_ptr() as *const i8, test_number); r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_GTT, AMDGPU_GEM_DOMAIN_VRAM); if r == 0 { r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_GTT); } }
        2 => { dev_info((*adev).dev, b"benchmark test: %d (simple test, VRAM to VRAM)\n\0".as_ptr() as *const i8, test_number); r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_VRAM); }
        3..=5 => { let (msg, s, d) = match test_number { 3 => (b"GTT to VRAM", AMDGPU_GEM_DOMAIN_GTT, AMDGPU_GEM_DOMAIN_VRAM), 4 => (b"VRAM to GTT", AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_GTT), _ => (b"VRAM to VRAM", AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_VRAM) }; dev_info((*adev).dev, b"benchmark test: %d (%s, buffer size sweep, powers of 2)\n\0".as_ptr() as *const i8, test_number, msg.as_ptr()); let mut i = 1; while i <= 16384 { r = amdgpu_benchmark_move(adev, i * AMDGPU_GPU_PAGE_SIZE, s, d); if r != 0 { break; } i <<= 1; } }
        6..=8 => { let (s, d) = match test_number { 6 => (AMDGPU_GEM_DOMAIN_GTT, AMDGPU_GEM_DOMAIN_VRAM), 7 => (AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_GTT), _ => (AMDGPU_GEM_DOMAIN_VRAM, AMDGPU_GEM_DOMAIN_VRAM) }; let mut i = 0; while i < AMDGPU_BENCHMARK_COMMON_MODES_N { r = amdgpu_benchmark_move(adev, common_modes[i] as u32, s, d); if r != 0 { break; } i += 1; } }
        _ => { dev_info((*adev).dev, b"Unknown benchmark %d\n\0".as_ptr() as *const i8, test_number); r = -EINVAL; }
    }
    mutex_unlock(&mut (*adev).benchmark_mutex);
    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
