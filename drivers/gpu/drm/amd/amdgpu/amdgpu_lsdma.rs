/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit:
// amdgpu.h, amdgpu_lsdma.h

const AMDGPU_LSDMA_MAX_SIZE: u64 = 0x2000000u64;

pub unsafe fn amdgpu_lsdma_wait_for(
    adev: *mut amdgpu_device,
    reg_index: u32,
    reg_val: u32,
    mask: u32,
) -> i32 {
    let mut val: u32;
    let mut i: i32;

    i = 0;
    while i < (*adev).usec_timeout {
        val = RREG32(adev, reg_index);
        if (val & mask) == reg_val {
            return 0;
        }
        udelay(1);
        i += 1;
    }

    -ETIME
}

pub unsafe fn amdgpu_lsdma_copy_mem(
    adev: *mut amdgpu_device,
    mut src_addr: u64,
    mut dst_addr: u64,
    mut mem_size: u64,
) -> i32 {
    let mut ret: i32;

    if mem_size == 0 {
        return -EINVAL;
    }

    while mem_size > 0 {
        let current_copy_size: u64 = if mem_size < AMDGPU_LSDMA_MAX_SIZE {
            mem_size
        } else {
            AMDGPU_LSDMA_MAX_SIZE
        };

        ret = ((*(*adev).lsdma.funcs).copy_mem)(
            adev,
            src_addr,
            dst_addr,
            current_copy_size,
        );
        if ret != 0 {
            return ret;
        }
        src_addr += current_copy_size;
        dst_addr += current_copy_size;
        mem_size -= current_copy_size;
    }

    0
}

pub unsafe fn amdgpu_lsdma_fill_mem(
    adev: *mut amdgpu_device,
    mut dst_addr: u64,
    data: u32,
    mut mem_size: u64,
) -> i32 {
    let mut ret: i32;

    if mem_size == 0 {
        return -EINVAL;
    }

    while mem_size > 0 {
        let current_fill_size: u64 = if mem_size < AMDGPU_LSDMA_MAX_SIZE {
            mem_size
        } else {
            AMDGPU_LSDMA_MAX_SIZE
        };

        ret = ((*(*adev).lsdma.funcs).fill_mem)(
            adev,
            dst_addr,
            data,
            current_fill_size,
        );
        if ret != 0 {
            return ret;
        }
        dst_addr += current_fill_size;
        mem_size -= current_fill_size;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
