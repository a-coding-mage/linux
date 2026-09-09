// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 */

// Dependencies supplied by the surrounding amdgpu translation.

/*
 * amdgpu_wb_*()
 * Writeback is the method by which the GPU updates special pages in memory
 * with the status of certain GPU events (fences, ring pointers,etc.).
 */

/**
 * amdgpu_wb_fini - Disable Writeback and free memory
 *
 * @adev: amdgpu_device pointer
 *
 * Disables Writeback and frees the Writeback memory (all asics).
 * Used at driver shutdown.
 */
pub unsafe fn amdgpu_wb_fini(adev: *mut amdgpu_device) {
    if !(*adev).wb.wb_obj.is_null() {
        amdgpu_bo_free_kernel(
            &mut (*adev).wb.wb_obj,
            &mut (*adev).wb.gpu_addr,
            &mut (*adev).wb.wb as *mut _ as *mut *mut core::ffi::c_void,
        );
        (*adev).wb.wb_obj = core::ptr::null_mut();
    }
}

/**
 * amdgpu_wb_init - Init Writeback driver info and allocate memory
 *
 * @adev: amdgpu_device pointer
 *
 * Initializes writeback and allocates writeback memory (all asics).
 * Used at driver startup.
 * Returns 0 on success or an -error on failure.
 */
pub unsafe fn amdgpu_wb_init(adev: *mut amdgpu_device) -> i32 {
    let mut r: i32;

    if (*adev).wb.wb_obj.is_null() {
        // AMDGPU_MAX_WB * sizeof(uint32_t) * 8 = AMDGPU_MAX_WB 256bit slots
        r = amdgpu_bo_create_kernel(
            adev,
            AMDGPU_MAX_WB * core::mem::size_of::<u32>() * 8,
            PAGE_SIZE,
            AMDGPU_GEM_DOMAIN_GTT,
            &mut (*adev).wb.wb_obj,
            &mut (*adev).wb.gpu_addr,
            &mut (*adev).wb.wb as *mut _ as *mut *mut core::ffi::c_void,
        );
        if r != 0 {
            dev_warn((*adev).dev, "({}) create WB bo failed\n", r);
            return r;
        }

        (*adev).wb.num_wb = AMDGPU_MAX_WB;
        core::ptr::write_bytes(
            &mut (*adev).wb.used as *mut _,
            0,
            core::mem::size_of_val(&(*adev).wb.used),
        );

        // clear wb memory
        core::ptr::write_bytes(
            (*adev).wb.wb as *mut u8,
            0,
            AMDGPU_MAX_WB * core::mem::size_of::<u32>() * 8,
        );
    }

    0
}

/**
 * amdgpu_wb_get - Allocate a wb entry
 *
 * @adev: amdgpu_device pointer
 * @wb: wb index
 *
 * Allocate a wb slot for use by the driver (all asics).
 * Returns 0 on success or an -EINVAL on failure.
 */
pub unsafe fn amdgpu_wb_get(adev: *mut amdgpu_device, wb: *mut u32) -> i32 {
    let mut flags: c_ulong;

    spin_lock_irqsave(&mut (*adev).wb.lock, &mut flags);
    let offset = find_first_zero_bit((*adev).wb.used, (*adev).wb.num_wb);
    if offset < (*adev).wb.num_wb {
        __set_bit(offset, (*adev).wb.used);
        spin_unlock_irqrestore(&mut (*adev).wb.lock, flags);
        *wb = offset << 3; // convert to dw offset
        0
    } else {
        spin_unlock_irqrestore(&mut (*adev).wb.lock, flags);
        -EINVAL
    }
}

/**
 * amdgpu_wb_free - Free a wb entry
 *
 * @adev: amdgpu_device pointer
 * @wb: wb index
 *
 * Free a wb slot allocated for use by the driver (all asics)
 */
pub unsafe fn amdgpu_wb_free(adev: *mut amdgpu_device, mut wb: u32) {
    let mut flags: c_ulong;

    wb >>= 3;
    spin_lock_irqsave(&mut (*adev).wb.lock, &mut flags);
    if wb < (*adev).wb.num_wb {
        __clear_bit(wb, (*adev).wb.used);
    }
    spin_unlock_irqrestore(&mut (*adev).wb.lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
