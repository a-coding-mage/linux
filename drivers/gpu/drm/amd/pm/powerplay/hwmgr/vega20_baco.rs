/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

static CLEAN_BACO_TBL: [soc15_baco_cmd_entry; 2] = [
    soc15_baco_cmd_entry { cmd: CMD_WRITE, reg: SOC15_REG_ENTRY(NBIF, 0, mmBIOS_SCRATCH_6), or_mask: 0, and_mask: 0, value: 0, delay: 0 },
    soc15_baco_cmd_entry { cmd: CMD_WRITE, reg: SOC15_REG_ENTRY(NBIF, 0, mmBIOS_SCRATCH_7), or_mask: 0, and_mask: 0, value: 0, delay: 0 },
];

pub unsafe fn vega20_get_bamaco_support(hwmgr: *mut pp_hwmgr) -> i32 {
    let adev = (*hwmgr).adev as *mut amdgpu_device;
    let mut reg: u32;

    if !phm_cap_enabled((*hwmgr).platform_descriptor.platformCaps, PHM_PlatformCaps_BACO) {
        return 0;
    }

    if ((RREG32(adev, 0x17569) & 0x20000000) >> 29) == 0x1 {
        reg = RREG32_SOC15(adev, NBIF, 0, mmRCC_BIF_STRAP0);

        if reg & RCC_BIF_STRAP0__STRAP_PX_CAPABLE_MASK != 0 {
            return BACO_SUPPORT;
        }
    }

    0
}

pub unsafe fn vega20_baco_get_state(
    hwmgr: *mut pp_hwmgr,
    state: *mut BACO_STATE,
) -> i32 {
    let adev = (*hwmgr).adev as *mut amdgpu_device;
    let reg = RREG32_SOC15(adev, NBIF, 0, mmBACO_CNTL);

    if reg & BACO_CNTL__BACO_MODE_MASK != 0 {
        // gfx has already entered BACO state
        *state = BACO_STATE_IN;
    } else {
        *state = BACO_STATE_OUT;
    }
    0
}

pub unsafe fn vega20_baco_set_state(hwmgr: *mut pp_hwmgr, state: BACO_STATE) -> i32 {
    let adev = (*hwmgr).adev as *mut amdgpu_device;
    let ras = amdgpu_ras_get_context(adev);
    let mut cur_state: BACO_STATE = BACO_STATE_OUT;
    let mut data: u32;

    vega20_baco_get_state(hwmgr, &mut cur_state);

    if cur_state == state {
        // aisc already in the target state
        return 0;
    }

    if state == BACO_STATE_IN {
        if ras.is_null() || !(*adev).ras_enabled {
            data = RREG32_SOC15(adev, THM, 0, mmTHM_BACO_CNTL);
            data |= 0x80000000;
            WREG32_SOC15(adev, THM, 0, mmTHM_BACO_CNTL, data);

            if smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_MSG_EnterBaco, 0, core::ptr::null_mut()) != 0 {
                return -EINVAL;
            }
        } else if smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_MSG_EnterBaco, 1, core::ptr::null_mut()) != 0 {
            return -EINVAL;
        }
    } else if state == BACO_STATE_OUT {
        if smum_send_msg_to_smc(hwmgr, PPSMC_MSG_ExitBaco, core::ptr::null_mut()) != 0 {
            return -EINVAL;
        }
        if soc15_baco_program_registers(hwmgr, CLEAN_BACO_TBL.as_ptr(), CLEAN_BACO_TBL.len()) == 0 {
            return -EINVAL;
        }
    }

    0
}

pub unsafe fn vega20_baco_apply_vdci_flush_workaround(hwmgr: *mut pp_hwmgr) -> i32 {
    let ret = vega20_set_pptable_driver_address(hwmgr);
    if ret != 0 {
        return ret;
    }

    smum_send_msg_to_smc(hwmgr, PPSMC_MSG_BacoWorkAroundFlushVDCI, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
