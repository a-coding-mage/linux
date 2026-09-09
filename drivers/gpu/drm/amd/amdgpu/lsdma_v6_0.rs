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

unsafe fn lsdma_v6_0_wait_pio_status(adev: *mut amdgpu_device) -> i32 {
    amdgpu_lsdma_wait_for(
        adev,
        SOC15_REG_OFFSET!(LSDMA, 0, regLSDMA_PIO_STATUS),
        LSDMA_PIO_STATUS__PIO_IDLE_MASK | LSDMA_PIO_STATUS__PIO_FIFO_EMPTY_MASK,
        LSDMA_PIO_STATUS__PIO_IDLE_MASK | LSDMA_PIO_STATUS__PIO_FIFO_EMPTY_MASK,
    )
}

unsafe fn lsdma_v6_0_copy_mem(
    adev: *mut amdgpu_device,
    src_addr: u64,
    dst_addr: u64,
    size: u64,
) -> i32 {
    let mut tmp: u32;

    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_SRC_ADDR_LO, lower_32_bits(src_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_SRC_ADDR_HI, upper_32_bits(src_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_DST_ADDR_LO, lower_32_bits(dst_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_DST_ADDR_HI, upper_32_bits(dst_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_CONTROL, 0x0);

    tmp = RREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_COMMAND);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, BYTE_COUNT, size);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, SRC_LOCATION, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, DST_LOCATION, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, SRC_ADDR_INC, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, DST_ADDR_INC, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, OVERLAP_DISABLE, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, CONSTANT_FILL, 0);
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_COMMAND, tmp);

    let ret = lsdma_v6_0_wait_pio_status(adev);
    if ret != 0 {
        dev_err!((*adev).dev, "LSDMA PIO failed to copy memory!\n");
    }
    ret
}

unsafe fn lsdma_v6_0_fill_mem(
    adev: *mut amdgpu_device,
    dst_addr: u64,
    data: u32,
    size: u64,
) -> i32 {
    let mut tmp: u32;

    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_CONSTFILL_DATA, data);
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_DST_ADDR_LO, lower_32_bits(dst_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_DST_ADDR_HI, upper_32_bits(dst_addr));
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_CONTROL, 0x0);

    tmp = RREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_COMMAND);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, BYTE_COUNT, size);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, SRC_LOCATION, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, DST_LOCATION, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, SRC_ADDR_INC, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, DST_ADDR_INC, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, OVERLAP_DISABLE, 0);
    tmp = REG_SET_FIELD!(tmp, LSDMA_PIO_COMMAND, CONSTANT_FILL, 1);
    WREG32_SOC15!(LSDMA, 0, regLSDMA_PIO_COMMAND, tmp);

    let ret = lsdma_v6_0_wait_pio_status(adev);
    if ret != 0 {
        dev_err!((*adev).dev, "LSDMA PIO failed to fill memory!\n");
    }
    ret
}

unsafe fn lsdma_v6_0_update_memory_power_gating(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    let mut tmp = RREG32_SOC15!(LSDMA, 0, regLSDMA_MEM_POWER_CTRL);
    tmp = REG_SET_FIELD!(tmp, LSDMA_MEM_POWER_CTRL, MEM_POWER_CTRL_EN, 0);
    WREG32_SOC15!(LSDMA, 0, regLSDMA_MEM_POWER_CTRL, tmp);
    tmp = REG_SET_FIELD!(tmp, LSDMA_MEM_POWER_CTRL, MEM_POWER_CTRL_EN, enable);
    WREG32_SOC15!(LSDMA, 0, regLSDMA_MEM_POWER_CTRL, tmp);
}

const lsdma_v6_0_funcs: amdgpu_lsdma_funcs = amdgpu_lsdma_funcs {
    copy_mem: Some(lsdma_v6_0_copy_mem),
    fill_mem: Some(lsdma_v6_0_fill_mem),
    update_memory_power_gating: Some(lsdma_v6_0_update_memory_power_gating),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
