/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct cgs_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cgs_read_register(device: *mut cgs_device, reg: u32) -> u32;
}

// Linux errno dependency supplied by the surrounding kernel translation.
const ENODEV: i32 = 19;

const ACP_MODE_I2S: u32 = 0;
const ACP_MODE_AZ: u32 = 1;

const MM_ACP_AZALIA_I2S_SELECT: u32 = 0x51d4;

pub unsafe fn amd_acp_hw_init(
    cgs_device: *mut cgs_device,
    acp_version_major: u32,
    acp_version_minor: u32,
) -> i32 {
    let mut acp_mode: u32 = ACP_MODE_I2S;

    if (acp_version_major == 2) && (acp_version_minor == 2) {
        acp_mode = unsafe { cgs_read_register(cgs_device, MM_ACP_AZALIA_I2S_SELECT) };
    }

    if acp_mode != ACP_MODE_I2S {
        return -ENODEV;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
