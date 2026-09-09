/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

unsafe fn smuio_v11_0_get_rom_index_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET(SMUIO, 0, mmROM_INDEX)
}

unsafe fn smuio_v11_0_get_rom_data_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET(SMUIO, 0, mmROM_DATA)
}

unsafe fn smuio_v11_0_update_rom_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let (mut def, mut data): (u32, u32);

    /* enable/disable ROM CG is not supported on APU */
    if (*adev).flags & AMD_IS_APU != 0 {
        return;
    }

    if (*adev).cg_flags & AMD_CG_SUPPORT_ROM_MGCG == 0 {
        return;
    }

    def = RREG32_SOC15(SMUIO, 0, mmCGTT_ROM_CLK_CTRL0);
    data = def;

    if enable {
        data &= !(CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK |
            CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK);
    } else {
        data |= CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK |
            CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK;
    }

    if def != data {
        WREG32_SOC15(SMUIO, 0, mmCGTT_ROM_CLK_CTRL0, data);
    }
}

unsafe fn smuio_v11_0_get_clock_gating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data: u32;

    /* CGTT_ROM_CLK_CTRL0 is not available for APU */
    if (*adev).flags & AMD_IS_APU != 0 {
        return;
    }

    data = RREG32_SOC15(SMUIO, 0, mmCGTT_ROM_CLK_CTRL0);
    if data & CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK == 0 {
        *flags |= AMD_CG_SUPPORT_ROM_MGCG as u64;
    }
}

pub static smuio_v11_0_funcs: amdgpu_smuio_funcs = amdgpu_smuio_funcs {
    get_rom_index_offset: Some(smuio_v11_0_get_rom_index_offset),
    get_rom_data_offset: Some(smuio_v11_0_get_rom_data_offset),
    update_rom_clock_gating: Some(smuio_v11_0_update_rom_clock_gating),
    get_clock_gating_state: Some(smuio_v11_0_get_clock_gating_state),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
