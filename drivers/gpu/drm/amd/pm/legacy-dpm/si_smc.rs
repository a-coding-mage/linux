/*
 * Copyright 2011 Advanced Micro Devices, Inc.
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

// Linux firmware and driver declarations are supplied by the surrounding crate.

unsafe fn si_set_smc_sram_address(adev: *mut amdgpu_device, smc_address: u32, limit: u32) -> i32 {
    if smc_address & 3 != 0 { return -EINVAL; }
    if smc_address.wrapping_add(3) > limit { return -EINVAL; }
    WREG32(mmSMC_IND_INDEX_0, smc_address);
    WREG32_P(mmSMC_IND_ACCESS_CNTL, 0, !SMC_IND_ACCESS_CNTL__AUTO_INCREMENT_IND_0_MASK);
    0
}

pub unsafe fn amdgpu_si_copy_bytes_to_smc(adev: *mut amdgpu_device, smc_start_address: u32,
    mut src: *const u8, mut byte_count: u32, limit: u32) -> i32 {
    let mut flags: c_ulong = 0;
    let mut ret: i32 = 0;
    let mut data: u32;
    let mut original_data: u32;
    let mut addr: u32 = smc_start_address;
    let mut extra_shift: u32;
    if smc_start_address & 3 != 0 { return -EINVAL; }
    if smc_start_address.wrapping_add(byte_count) > limit { return -EINVAL; }
    spin_lock_irqsave(&(*adev).reg.smc.lock, &mut flags);
    while byte_count >= 4 {
        // SMC address space is BE
        data = ((*src.add(0) as u32) << 24) | ((*src.add(1) as u32) << 16) |
               ((*src.add(2) as u32) << 8) | *src.add(3) as u32;
        ret = si_set_smc_sram_address(adev, addr, limit);
        if ret != 0 { break; }
        WREG32(mmSMC_IND_DATA_0, data);
        src = src.add(4); byte_count -= 4; addr += 4;
    }
    if ret == 0 && byte_count > 0 {
        data = 0;
        ret = si_set_smc_sram_address(adev, addr, limit);
        if ret == 0 {
            original_data = RREG32(mmSMC_IND_DATA_0);
            extra_shift = 8 * (4 - byte_count);
            while byte_count > 0 {
                data = (data << 8) + *src as u32;
                src = src.add(1); byte_count -= 1;
            }
            data <<= extra_shift;
            data |= original_data & !(((!0u32) << extra_shift));
            ret = si_set_smc_sram_address(adev, addr, limit);
            if ret == 0 { WREG32(mmSMC_IND_DATA_0, data); }
        }
    }
    spin_unlock_irqrestore(&(*adev).reg.smc.lock, flags);
    ret
}

pub unsafe fn amdgpu_si_start_smc(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SMC(SMC_SYSCON_RESET_CNTL); tmp &= !RST_REG;
    WREG32_SMC(SMC_SYSCON_RESET_CNTL, tmp);
}

pub unsafe fn amdgpu_si_reset_smc(adev: *mut amdgpu_device) {
    let mut tmp: u32;
    RREG32(mmCB_CGTT_SCLK_CTRL); RREG32(mmCB_CGTT_SCLK_CTRL);
    RREG32(mmCB_CGTT_SCLK_CTRL); RREG32(mmCB_CGTT_SCLK_CTRL);
    tmp = RREG32_SMC(SMC_SYSCON_RESET_CNTL) | RST_REG;
    WREG32_SMC(SMC_SYSCON_RESET_CNTL, tmp);
}

pub unsafe fn amdgpu_si_program_jump_on_start(adev: *mut amdgpu_device) -> i32 {
    static DATA: [u8; 4] = [0x0E, 0x00, 0x40, 0x40];
    amdgpu_si_copy_bytes_to_smc(adev, 0, DATA.as_ptr(), 4, DATA.len() as u32 + 1)
}

pub unsafe fn amdgpu_si_smc_clock(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    if enable { tmp &= !CK_DISABLE; } else { tmp |= CK_DISABLE; }
    WREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0, tmp);
}

pub unsafe fn amdgpu_si_is_smc_running(adev: *mut amdgpu_device) -> bool {
    let rst = RREG32_SMC(SMC_SYSCON_RESET_CNTL); let clk = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    !(rst & RST_REG != 0) && !(clk & CK_DISABLE != 0)
}

pub unsafe fn amdgpu_si_send_msg_to_smc(adev: *mut amdgpu_device, msg: PPSMC_Msg) -> PPSMC_Result {
    let usec_timeout: i32 = match msg {
        PPSMC_MSG_NoForcedLevel | PPSMC_MSG_SetEnabledLevels | PPSMC_MSG_SetForcedLevels |
        PPSMC_MSG_DisableULV | PPSMC_MSG_SwitchToSwState => 1000000,
        _ => 200000,
    };
    if !amdgpu_si_is_smc_running(adev) { return PPSMC_Result_Failed; }
    WREG32(mmSMC_MESSAGE_0, msg);
    for _ in 0..usec_timeout { if RREG32(mmSMC_RESP_0) != 0 { break; } udelay(1); }
    let tmp = RREG32(mmSMC_RESP_0);
    if tmp == 0 { drm_warn(adev_to_drm(adev), "%s timeout on message: %x (SMC_SCRATCH0: %x)\n", __func__, msg, RREG32(mmSMC_SCRATCH0)); }
    tmp as PPSMC_Result
}

pub unsafe fn amdgpu_si_wait_for_smc_inactive(adev: *mut amdgpu_device) -> PPSMC_Result {
    if !amdgpu_si_is_smc_running(adev) { return PPSMC_Result_OK; }
    for _ in 0..(*adev).usec_timeout { if RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0) & CKEN == 0 { break; } udelay(1); }
    PPSMC_Result_OK
}

pub unsafe fn amdgpu_si_load_smc_ucode(adev: *mut amdgpu_device, limit: u32) -> i32 {
    if (*adev).pm.fw.is_null() { return -EINVAL; }
    let hdr = (*adev).pm.fw.data as *const smc_firmware_header_v1_0;
    amdgpu_ucode_print_smc_hdr(&(*hdr).header);
    (*adev).pm.fw_version = le32_to_cpu((*hdr).header.ucode_version);
    let start = le32_to_cpu((*hdr).ucode_start_addr); let mut size = le32_to_cpu((*hdr).header.ucode_size_bytes);
    let mut src = (*adev).pm.fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize);
    if size & 3 != 0 { return -EINVAL; }
    let mut flags: c_ulong = 0; spin_lock_irqsave(&(*adev).reg.smc.lock, &mut flags);
    WREG32(mmSMC_IND_INDEX_0, start); WREG32_P(mmSMC_IND_ACCESS_CNTL, SMC_IND_ACCESS_CNTL__AUTO_INCREMENT_IND_0_MASK, !SMC_IND_ACCESS_CNTL__AUTO_INCREMENT_IND_0_MASK);
    while size >= 4 { let data = ((*src.add(0) as u32)<<24)|((*src.add(1) as u32)<<16)|((*src.add(2) as u32)<<8)|*src.add(3) as u32; WREG32(mmSMC_IND_DATA_0, data); src=src.add(4); size-=4; }
    WREG32_P(mmSMC_IND_ACCESS_CNTL, 0, !SMC_IND_ACCESS_CNTL__AUTO_INCREMENT_IND_0_MASK); spin_unlock_irqrestore(&(*adev).reg.smc.lock, flags); 0
}

pub unsafe fn amdgpu_si_read_smc_sram_dword(adev: *mut amdgpu_device, smc_address: u32, value: *mut u32, limit: u32) -> i32 {
    let mut flags: c_ulong=0; spin_lock_irqsave(&(*adev).reg.smc.lock,&mut flags); let ret=si_set_smc_sram_address(adev,smc_address,limit); if ret==0 {*value=RREG32(mmSMC_IND_DATA_0);} spin_unlock_irqrestore(&(*adev).reg.smc.lock,flags); ret
}

pub unsafe fn amdgpu_si_write_smc_sram_dword(adev: *mut amdgpu_device, smc_address: u32, value: u32, limit: u32) -> i32 {
    let mut flags: c_ulong=0; spin_lock_irqsave(&(*adev).reg.smc.lock,&mut flags); let ret=si_set_smc_sram_address(adev,smc_address,limit); if ret==0 {WREG32(mmSMC_IND_DATA_0,value);} spin_unlock_irqrestore(&(*adev).reg.smc.lock,flags); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
