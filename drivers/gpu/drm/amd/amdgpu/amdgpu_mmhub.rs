/*
 * Copyright (C) 2023  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern "C" {
    fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
}

pub unsafe fn amdgpu_mmhub_ras_sw_init(adev: *mut amdgpu_device) -> i32 {
    let mut err: i32;
    let ras: *mut amdgpu_mmhub_ras;

    if (*adev).mmhub.ras.is_null() {
        return 0;
    }

    ras = (*adev).mmhub.ras;
    err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 {
        dev_err!((*adev).dev, "Failed to register mmhub ras block!\n");
        return err;
    }

    strcpy(
        (*ras).ras_block.ras_comm.name.as_mut_ptr() as *mut core::ffi::c_char,
        b"mmhub\0".as_ptr() as *const core::ffi::c_char,
    );
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__MMHUB;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).mmhub.ras_if = &mut (*ras).ras_block.ras_comm;

    /* mmhub ras follows amdgpu_ras_block_late_init_default for late init */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
