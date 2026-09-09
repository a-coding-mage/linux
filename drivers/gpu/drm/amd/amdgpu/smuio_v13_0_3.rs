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

// Dependencies supplied by the surrounding AMDGPU translation.

const PKG_TYPE_MASK: u32 = 0x00000003;

/**
 * smuio_v13_0_3_get_die_id - query die id from FCH.
 *
 * @adev: amdgpu device pointer
 *
 * Returns die id
 */
unsafe fn smuio_v13_0_3_get_die_id(adev: *mut amdgpu_device) -> u32 {
    let data: u32;
    let die_id: u32;

    data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    die_id = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, DIE_ID);

    die_id
}

/**
 * smuio_v13_0_3_get_socket_id - query socket id from FCH
 *
 * @adev: amdgpu device pointer
 *
 * Returns socket id
 */
unsafe fn smuio_v13_0_3_get_socket_id(adev: *mut amdgpu_device) -> u32 {
    let data: u32;
    let socket_id: u32;

    data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    socket_id = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, SOCKET_ID);

    socket_id
}

/**
 * smuio_v13_0_3_get_pkg_type - query package type set by MP1/bootcode
 *
 * @adev: amdgpu device pointer
 *
 * Returns package type
 */
unsafe fn smuio_v13_0_3_get_pkg_type(adev: *mut amdgpu_device) -> amdgpu_pkg_type {
    let pkg_type: amdgpu_pkg_type;
    let mut data: u32;

    data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, PKG_TYPE);
    /* pkg_type[4:0]
     *
     * bit 1 == 1 APU form factor
     *
     * b0100 - b1111 - Reserved
     */
    pkg_type = match data & PKG_TYPE_MASK {
        0x0 => AMDGPU_PKG_TYPE_CEM,
        0x1 => AMDGPU_PKG_TYPE_OAM,
        0x2 => AMDGPU_PKG_TYPE_APU,
        _ => AMDGPU_PKG_TYPE_UNKNOWN,
    };

    pkg_type
}

pub const smuio_v13_0_3_funcs: amdgpu_smuio_funcs = amdgpu_smuio_funcs {
    get_die_id: Some(smuio_v13_0_3_get_die_id),
    get_socket_id: Some(smuio_v13_0_3_get_socket_id),
    get_pkg_type: Some(smuio_v13_0_3_get_pkg_type),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
