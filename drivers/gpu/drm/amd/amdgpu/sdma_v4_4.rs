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

const SDMA1_REG_OFFSET: u32 = 0x600;
const SDMA2_REG_OFFSET: u32 = 0x1cda0;
const SDMA3_REG_OFFSET: u32 = 0x1d1a0;
const SDMA4_REG_OFFSET: u32 = 0x1d5a0;

/* helper function that allow only use sdma0 register offset
 * to calculate register offset for all the sdma instances */
unsafe fn sdma_v4_4_get_reg_offset(
    adev: *mut amdgpu_device,
    instance: u32,
    offset: u32,
) -> u32 {
    let sdma_base = (*adev).reg_offset[SDMA0_HWIP][0][0];

    match instance {
        0 => sdma_base + offset,
        1 => sdma_base + SDMA1_REG_OFFSET + offset,
        2 => sdma_base + SDMA2_REG_OFFSET + offset,
        3 => sdma_base + SDMA3_REG_OFFSET + offset,
        4 => sdma_base + SDMA4_REG_OFFSET + offset,
        _ => 0,
    }
}

static sdma_v4_4_ras_fields: [soc15_ras_field_entry; 27] = [
    { name: "SDMA_MBANK_DATA_BUF0_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF0_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF1_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF1_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF2_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF2_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF3_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF3_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF4_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF4_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF5_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF5_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF6_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF6_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF7_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF7_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF8_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF8_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF9_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF9_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF10_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF10_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF11_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF11_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF12_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF12_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF13_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF13_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF14_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF14_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MBANK_DATA_BUF15_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER, SDMA_MBANK_DATA_BUF15_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_UCODE_BUF_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_UCODE_BUF_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_RB_CMD_BUF_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_RB_CMD_BUF_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_IB_CMD_BUF_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_IB_CMD_BUF_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_UTCL1_RD_FIFO_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_UTCL1_RD_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_UTCL1_RDBST_FIFO_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_UTCL1_RDBST_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_UTCL1_WR_FIFO_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_UTCL1_WR_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_DATA_LUT_FIFO_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_DATA_LUT_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_SPLIT_DATA_BUF_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_SPLIT_DATA_BUF_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MC_WR_ADDR_FIFO_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_MC_WR_ADDR_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
    { name: "SDMA_MC_RDRET_BUF_SED", reg_offset: SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_EDC_COUNTER2), sec_count_mask: SOC15_REG_FIELD!(SDMA0_EDC_COUNTER2, SDMA_MC_WR_ADDR_FIFO_SED), sec_count_shift: 0, ue_count_mask: 0, ue_count_shift: 0 },
];

unsafe fn sdma_v4_4_get_ras_error_count(adev: *mut amdgpu_device, reg_offset: u32, value: u32, instance: u32, sec_count: *mut u32) {
    let mut sec_cnt: u32;
    for i in 0..sdma_v4_4_ras_fields.len() {
        if sdma_v4_4_ras_fields[i].reg_offset != reg_offset { continue; }
        sec_cnt = (value & sdma_v4_4_ras_fields[i].sec_count_mask) >> sdma_v4_4_ras_fields[i].sec_count_shift;
        if sec_cnt != 0 {
            dev_info!((*adev).dev, "Detected {} in SDMA{}, SED {}\n", sdma_v4_4_ras_fields[i].name, instance, sec_cnt);
            *sec_count += sec_cnt;
        }
    }
}

unsafe fn sdma_v4_4_query_ras_error_count_by_instance(adev: *mut amdgpu_device, instance: u32, ras_error_status: *mut core::ffi::c_void) -> i32 {
    let err_data = ras_error_status as *mut ras_err_data;
    let mut sec_count: u32 = 0;
    let mut reg_offset = sdma_v4_4_get_reg_offset(adev, instance, regSDMA0_EDC_COUNTER);
    let mut reg_value = RREG32!(reg_offset);
    if reg_value != 0 { sdma_v4_4_get_ras_error_count(adev, regSDMA0_EDC_COUNTER, reg_value, instance, &mut sec_count); }
    reg_offset = sdma_v4_4_get_reg_offset(adev, instance, regSDMA0_EDC_COUNTER2);
    reg_value = RREG32!(reg_offset);
    if reg_value != 0 { sdma_v4_4_get_ras_error_count(adev, regSDMA0_EDC_COUNTER2, reg_value, instance, &mut sec_count); }
    (*err_data).ue_count += sec_count;
    (*err_data).ce_count = 0;
    0
}

unsafe fn sdma_v4_4_reset_ras_error_count(adev: *mut amdgpu_device) {
    if amdgpu_ras_is_supported(adev, AMDGPU_RAS_BLOCK__SDMA) {
        for i in 0..(*adev).sdma.num_instances {
            let reg_offset = sdma_v4_4_get_reg_offset(adev, i, regSDMA0_EDC_COUNTER);
            WREG32!(reg_offset, 0);
            let reg_offset = sdma_v4_4_get_reg_offset(adev, i, regSDMA0_EDC_COUNTER2);
            WREG32!(reg_offset, 0);
        }
    }
}

unsafe fn sdma_v4_4_query_ras_error_count(adev: *mut amdgpu_device, ras_error_status: *mut core::ffi::c_void) {
    for i in 0..(*adev).sdma.num_instances {
        if sdma_v4_4_query_ras_error_count_by_instance(adev, i, ras_error_status) != 0 {
            dev_err!((*adev).dev, "Query ras error count failed in SDMA{}\n", i);
            return;
        }
    }
}

const sdma_v4_4_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(sdma_v4_4_query_ras_error_count),
    reset_ras_error_count: Some(sdma_v4_4_reset_ras_error_count),
};

static mut sdma_v4_4_ras: amdgpu_sdma_ras = amdgpu_sdma_ras {
    ras_block: amdgpu_ras_block {
        hw_ops: &sdma_v4_4_ras_hw_ops,
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
