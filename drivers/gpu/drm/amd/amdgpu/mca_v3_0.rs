/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
// Dependencies supplied by the surrounding translation unit:
// amdgpu_ras.h, amdgpu.h, amdgpu_mca.h

const smnMCMP0_STATUST0: u32 = 0x03830408;
const smnMCMP1_STATUST0: u32 = 0x03b30408;
const smnMCMPIO_STATUST0: u32 = 0x0c930408;

unsafe extern "C" fn mca_v3_0_mp0_query_ras_error_count(
    adev: *mut amdgpu_device,
    ras_error_status: *mut core::ffi::c_void,
) {
    amdgpu_mca_query_ras_error_count(adev, smnMCMP0_STATUST0, ras_error_status);
}

unsafe extern "C" fn mca_v3_0_ras_block_match(
    block_obj: *mut amdgpu_ras_block_object,
    block: amdgpu_ras_block,
    sub_block_index: u32,
) -> i32 {
    if block_obj.is_null() {
        return -EINVAL;
    }

    unsafe {
        if ((*block_obj).ras_comm.block == block)
            && ((*block_obj).ras_comm.sub_block_index == sub_block_index)
        {
            return 0;
        }
    }

    -EINVAL
}

static mca_v3_0_mp0_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(mca_v3_0_mp0_query_ras_error_count),
    query_ras_error_address: None,
};

static mut mca_v3_0_mp0_ras: amdgpu_mca_ras_block = amdgpu_mca_ras_block {
    ras_block: amdgpu_ras_block_object {
        hw_ops: &raw const mca_v3_0_mp0_hw_ops,
        ras_block_match: Some(mca_v3_0_ras_block_match),
    },
};

unsafe extern "C" fn mca_v3_0_mp1_query_ras_error_count(
    adev: *mut amdgpu_device,
    ras_error_status: *mut core::ffi::c_void,
) {
    amdgpu_mca_query_ras_error_count(adev, smnMCMP1_STATUST0, ras_error_status);
}

static mca_v3_0_mp1_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(mca_v3_0_mp1_query_ras_error_count),
    query_ras_error_address: None,
};

static mut mca_v3_0_mp1_ras: amdgpu_mca_ras_block = amdgpu_mca_ras_block {
    ras_block: amdgpu_ras_block_object {
        hw_ops: &raw const mca_v3_0_mp1_hw_ops,
        ras_block_match: Some(mca_v3_0_ras_block_match),
    },
};

unsafe extern "C" fn mca_v3_0_mpio_query_ras_error_count(
    adev: *mut amdgpu_device,
    ras_error_status: *mut core::ffi::c_void,
) {
    amdgpu_mca_query_ras_error_count(adev, smnMCMPIO_STATUST0, ras_error_status);
}

static mca_v3_0_mpio_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(mca_v3_0_mpio_query_ras_error_count),
    query_ras_error_address: None,
};

static mut mca_v3_0_mpio_ras: amdgpu_mca_ras_block = amdgpu_mca_ras_block {
    ras_block: amdgpu_ras_block_object {
        hw_ops: &raw const mca_v3_0_mpio_hw_ops,
        ras_block_match: Some(mca_v3_0_ras_block_match),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
