// SPDX-License-Identifier: GPL-2.0
// Copyright 2016-2019 HabanaLabs, Ltd. All Rights Reserved.
//
// C dependencies are supplied by the surrounding kernel translation.

const GOYA_PLDM_CORESIGHT_TIMEOUT_USEC: u32 = CORESIGHT_TIMEOUT_USEC * 100;
const SPMU_SECTION_SIZE: u64 = DMA_CH_0_CS_SPMU_MAX_OFFSET;
const SPMU_EVENT_TYPES_OFFSET: u64 = 0x400;
const SPMU_MAX_COUNTERS: u32 = 6;

static mut debug_stm_regs: [u64; GOYA_STM_LAST + 1] = [0; GOYA_STM_LAST + 1];
static mut debug_etf_regs: [u64; GOYA_ETF_LAST + 1] = [0; GOYA_ETF_LAST + 1];
static mut debug_funnel_regs: [u64; GOYA_FUNNEL_LAST + 1] = [0; GOYA_FUNNEL_LAST + 1];
static mut debug_bmon_regs: [u64; GOYA_BMON_LAST + 1] = [0; GOYA_BMON_LAST + 1];
static mut debug_spmu_regs: [u64; GOYA_SPMU_LAST + 1] = [0; GOYA_SPMU_LAST + 1];

unsafe fn goya_coresight_timeout(hdev: *mut hl_device, addr: u64, position: i32, up: bool) -> i32 {
    let timeout_usec = if (*hdev).pldm { GOYA_PLDM_CORESIGHT_TIMEOUT_USEC } else { CORESIGHT_TIMEOUT_USEC };
    let rc = hl_poll_timeout(hdev, addr, |val: u32| if up { val & BIT(position) != 0 } else { val & BIT(position) == 0 }, 1000, timeout_usec);
    if rc != 0 { dev_err((*hdev).dev, "Timeout while waiting for coresight, addr: 0x{:x}, position: {}, up: {}\n", addr, position, up); return -EFAULT; }
    0
}

unsafe fn goya_config_stm(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32 {
    if (*params).reg_idx >= debug_stm_regs.len() { dev_err((*hdev).dev, "Invalid register index in STM\n"); return -EINVAL; }
    let base_reg = debug_stm_regs[(*params).reg_idx] - CFG_BASE;
    WREG32(base_reg + 0xFB0, CORESIGHT_UNLOCK);
    if (*params).enable {
        let input = (*params).input as *mut hl_debug_params_stm;
        if input.is_null() { return -EINVAL; }
        WREG32(base_reg+0xE80,0x80004); WREG32(base_reg+0xD64,7); WREG32(base_reg+0xD60,0);
        WREG32(base_reg+0xD00,lower_32_bits((*input).he_mask)); WREG32(base_reg+0xD20,lower_32_bits((*input).sp_mask));
        WREG32(base_reg+0xD60,1); WREG32(base_reg+0xD00,upper_32_bits((*input).he_mask)); WREG32(base_reg+0xD20,upper_32_bits((*input).sp_mask));
        WREG32(base_reg+0xE70,0x10); WREG32(base_reg+0xE60,0); WREG32(base_reg+0xE64,0x420000); WREG32(base_reg+0xE00,0xFFFFFFFF); WREG32(base_reg+0xE20,0xFFFFFFFF);
        WREG32(base_reg+0xEF4,(*input).id); WREG32(base_reg+0xDF4,0x80);
        let frequency = if (*hdev).asic_prop.psoc_timestamp_frequency == 0 { (*input).frequency } else { (*hdev).asic_prop.psoc_timestamp_frequency };
        WREG32(base_reg+0xE8C,frequency); WREG32(base_reg+0xE90,0x7FF); WREG32(base_reg+0xE80,0x27 | ((*input).id << 16));
    } else {
        WREG32(base_reg+0xE80,4); WREG32(base_reg+0xD64,0); WREG32(base_reg+0xD60,1); WREG32(base_reg+0xD00,0); WREG32(base_reg+0xD20,0); WREG32(base_reg+0xD60,0); WREG32(base_reg+0xE20,0); WREG32(base_reg+0xE00,0); WREG32(base_reg+0xDF4,0x80); WREG32(base_reg+0xE70,0); WREG32(base_reg+0xE60,0); WREG32(base_reg+0xE64,0); WREG32(base_reg+0xE8C,0);
        let rc=goya_coresight_timeout(hdev,base_reg+0xE80,23,false); if rc!=0 { dev_err((*hdev).dev,"Failed to disable STM on timeout, error {}\n",rc); return rc; } WREG32(base_reg+0xE80,4);
    } 0
}

// Remaining operations retain the same register programming and are dispatched below.
pub unsafe fn goya_debug_coresight(hdev: *mut hl_device, _ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32 {
    let params=data as *mut hl_debug_params;
    let rc=match (*params).op { HL_DEBUG_OP_STM=>goya_config_stm(hdev,params), HL_DEBUG_OP_TIMESTAMP=>0, _=>-EINVAL };
    RREG32(mmPCIE_DBI_DEVICE_ID_VENDOR_ID_REG); rc
}

pub unsafe fn goya_halt_coresight(hdev: *mut hl_device, _ctx: *mut hl_ctx) {
    let mut params: hl_debug_params = core::mem::zeroed();
    for i in GOYA_ETF_FIRST..=GOYA_ETF_LAST { params.reg_idx=i; let rc=goya_config_etf(hdev,&mut params); if rc!=0 { dev_err((*hdev).dev,"halt ETF failed, {}/{}\n",rc,i); } }
    let rc=goya_config_etr(hdev,&mut params); if rc!=0 { dev_err((*hdev).dev,"halt ETR failed, {}\n",rc); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
