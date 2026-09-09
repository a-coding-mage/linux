/*
 * Copyright 2013 Advanced Micro Devices, Inc.
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
 * Authors: Alex Deucher
 */

pub unsafe fn amdgpu_kv_notify_message_to_smu(adev: *mut amdgpu_device, id: u32) -> i32 {
    let mut tmp: u32 = 0;

    WREG32!(mmSMC_MESSAGE_0, id & SMC_MESSAGE_0__SMC_MSG_MASK);

    for _i in 0..(*adev).usec_timeout {
        if (RREG32!(mmSMC_RESP_0) & SMC_RESP_0__SMC_RESP_MASK) != 0 {
            break;
        }
        udelay(1);
    }
    tmp = RREG32!(mmSMC_RESP_0) & SMC_RESP_0__SMC_RESP_MASK;

    if tmp != 1 {
        if tmp == 0xFF {
            return -EINVAL;
        } else if tmp == 0xFE {
            return -EINVAL;
        }
    }

    0
}

pub unsafe fn amdgpu_kv_dpm_get_enable_mask(
    adev: *mut amdgpu_device,
    enable_mask: *mut u32,
) -> i32 {
    let ret = amdgpu_kv_notify_message_to_smu(adev, PPSMC_MSG_SCLKDPM_GetEnabledMask);

    if ret == 0 {
        *enable_mask = RREG32_SMC!(ixSMC_SYSCON_MSG_ARG_0);
    }

    ret
}

pub unsafe fn amdgpu_kv_send_msg_to_smc_with_parameter(
    adev: *mut amdgpu_device,
    msg: PPSMC_Msg,
    parameter: u32,
) -> i32 {
    WREG32!(mmSMC_MSG_ARG_0, parameter);

    amdgpu_kv_notify_message_to_smu(adev, msg)
}

unsafe fn kv_set_smc_sram_address(
    adev: *mut amdgpu_device,
    smc_address: u32,
    limit: u32,
) -> i32 {
    if smc_address & 3 != 0 {
        return -EINVAL;
    }
    if smc_address.wrapping_add(3) > limit {
        return -EINVAL;
    }

    WREG32!(mmSMC_IND_INDEX_0, smc_address);
    WREG32_P!(
        mmSMC_IND_ACCESS_CNTL,
        0,
        !SMC_IND_ACCESS_CNTL__AUTO_INCREMENT_IND_0_MASK
    );

    0
}

pub unsafe fn amdgpu_kv_read_smc_sram_dword(
    adev: *mut amdgpu_device,
    smc_address: u32,
    value: *mut u32,
    limit: u32,
) -> i32 {
    let ret = kv_set_smc_sram_address(adev, smc_address, limit);
    if ret != 0 {
        return ret;
    }

    *value = RREG32!(mmSMC_IND_DATA_0);
    0
}

pub unsafe fn amdgpu_kv_smc_dpm_enable(adev: *mut amdgpu_device, enable: bool) -> i32 {
    if enable {
        amdgpu_kv_notify_message_to_smu(adev, PPSMC_MSG_DPM_Enable)
    } else {
        amdgpu_kv_notify_message_to_smu(adev, PPSMC_MSG_DPM_Disable)
    }
}

pub unsafe fn amdgpu_kv_smc_bapm_enable(adev: *mut amdgpu_device, enable: bool) -> i32 {
    if enable {
        amdgpu_kv_notify_message_to_smu(adev, PPSMC_MSG_EnableBAPM)
    } else {
        amdgpu_kv_notify_message_to_smu(adev, PPSMC_MSG_DisableBAPM)
    }
}

pub unsafe fn amdgpu_kv_copy_bytes_to_smc(
    adev: *mut amdgpu_device,
    smc_start_address: u32,
    mut src: *const u8,
    mut byte_count: u32,
    limit: u32,
) -> i32 {
    let (mut data, mut original_data, mut addr, mut extra_shift, mut t_byte, mut count, mut mask):
        (u32, u32, u32, u32, u32, u32, u32);

    if smc_start_address.wrapping_add(byte_count) > limit {
        return -EINVAL;
    }

    addr = smc_start_address;
    t_byte = addr & 3;

    /* RMW for the initial bytes */
    if t_byte != 0 {
        addr = addr.wrapping_sub(t_byte);

        let ret = kv_set_smc_sram_address(adev, addr, limit);
        if ret != 0 {
            return ret;
        }

        original_data = RREG32!(mmSMC_IND_DATA_0);

        data = 0;
        mask = 0;
        count = 4;
        while count > 0 {
            if t_byte > 0 {
                mask = (mask << 8) | 0xff;
                t_byte -= 1;
            } else if byte_count > 0 {
                data = (data << 8) + *src;
                src = src.add(1);
                byte_count -= 1;
                mask <<= 8;
            } else {
                data <<= 8;
                mask = (mask << 8) | 0xff;
            }
            count -= 1;
        }

        data |= original_data & mask;

        let ret = kv_set_smc_sram_address(adev, addr, limit);
        if ret != 0 {
            return ret;
        }

        WREG32!(mmSMC_IND_DATA_0, data);

        addr = addr.wrapping_add(4);
    }

    while byte_count >= 4 {
        /* SMC address space is BE */
        data = ((*src.add(0) as u32) << 24)
            + ((*src.add(1) as u32) << 16)
            + ((*src.add(2) as u32) << 8)
            + (*src.add(3) as u32);

        let ret = kv_set_smc_sram_address(adev, addr, limit);
        if ret != 0 {
            return ret;
        }

        WREG32!(mmSMC_IND_DATA_0, data);

        src = src.add(4);
        byte_count -= 4;
        addr = addr.wrapping_add(4);
    }

    /* RMW for the final bytes */
    if byte_count > 0 {
        data = 0;

        let ret = kv_set_smc_sram_address(adev, addr, limit);
        if ret != 0 {
            return ret;
        }

        original_data = RREG32!(mmSMC_IND_DATA_0);

        extra_shift = 8 * (4 - byte_count);

        while byte_count > 0 {
            /* SMC address space is BE */
            data = (data << 8) + *src;
            src = src.add(1);
            byte_count -= 1;
        }

        data <<= extra_shift;

        data |= original_data & !((!0u32).wrapping_shl(extra_shift));

        let ret = kv_set_smc_sram_address(adev, addr, limit);
        if ret != 0 {
            return ret;
        }

        WREG32!(mmSMC_IND_DATA_0, data);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
