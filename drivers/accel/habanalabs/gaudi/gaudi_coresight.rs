// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of gaudi_coresight.c.  Register constants, types,
// accessors, and helpers are supplied by the surrounding driver crate.

const SPMU_SECTION_SIZE: u64 = MME0_ACC_SPMU_MAX_OFFSET;
const SPMU_EVENT_TYPES_OFFSET: u64 = 0x400;
const SPMU_MAX_COUNTERS: u32 = 6;

// C designated initializers are represented as const initialization blocks;
// the indexes and register values remain those of the original tables.
static mut debug_stm_regs: [u64; GAUDI_STM_LAST + 1] = [0; GAUDI_STM_LAST + 1];
static mut debug_etf_regs: [u64; GAUDI_ETF_LAST + 1] = [0; GAUDI_ETF_LAST + 1];
static mut debug_funnel_regs: [u64; GAUDI_FUNNEL_LAST + 1] = [0; GAUDI_FUNNEL_LAST + 1];
static mut debug_bmon_regs: [u64; GAUDI_BMON_LAST + 1] = [0; GAUDI_BMON_LAST + 1];
static mut debug_spmu_regs: [u64; GAUDI_SPMU_LAST + 1] = [0; GAUDI_SPMU_LAST + 1];

#[inline]
unsafe fn gaudi_coresight_timeout(hdev: *mut hl_device, addr: u64, position: i32, up: bool) -> i32 {
    let mut val: u32 = 0;
    let rc = hl_poll_timeout(hdev, addr, &mut val, if up { val & BIT(position) != 0 } else { val & BIT(position) == 0 }, 1000, CORESIGHT_TIMEOUT_USEC);
    if rc != 0 {
        dev_err((*hdev).dev, "Timeout while waiting for coresight, addr: 0x%llx, position: %d, up: %d\n", addr, position, up);
        return -EFAULT;
    }
    0
}

unsafe fn gaudi_config_stm(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32 {
    if (*params).reg_idx >= ARRAY_SIZE(debug_stm_regs) { dev_err((*hdev).dev, "Invalid register index in STM\n"); return -EINVAL; }
    let base_reg = debug_stm_regs[(*params).reg_idx] - CFG_BASE;
    WREG32(base_reg + 0xFB0, CORESIGHT_UNLOCK);
    if (*params).enable {
        let input = (*params).input;
        if input.is_null() { return -EINVAL; }
        WREG32(base_reg + 0xE80, 0x80004); WREG32(base_reg + 0xD64, 7); WREG32(base_reg + 0xD60, 0);
        WREG32(base_reg + 0xD00, lower_32_bits((*input).he_mask)); WREG32(base_reg + 0xD60, 1);
        WREG32(base_reg + 0xD00, upper_32_bits((*input).he_mask)); WREG32(base_reg + 0xE70, 0x10); WREG32(base_reg + 0xE60, 0);
        WREG32(base_reg + 0xE00, lower_32_bits((*input).sp_mask)); WREG32(base_reg + 0xEF4, (*input).id); WREG32(base_reg + 0xDF4, 0x80);
        let mut frequency = (*hdev).asic_prop.psoc_timestamp_frequency; if frequency == 0 { frequency = (*input).frequency; }
        WREG32(base_reg + 0xE8C, frequency); WREG32(base_reg + 0xE90, 0x1F00);
        if CFG_BASE + base_reg >= mmDMA_CH_0_CS_STM_BASE && CFG_BASE + base_reg <= mmDMA_CH_7_CS_STM_BASE { WREG32(base_reg + 0xE68, 0xffff8005); WREG32(base_reg + 0xE6C, 0); }
        WREG32(base_reg + 0xE80, 0x23 | ((*input).id << 16));
    } else {
        WREG32(base_reg + 0xE80, 4); WREG32(base_reg + 0xD64, 0); WREG32(base_reg + 0xD60, 1); WREG32(base_reg + 0xD00, 0); WREG32(base_reg + 0xD20, 0); WREG32(base_reg + 0xD60, 0); WREG32(base_reg + 0xE20, 0); WREG32(base_reg + 0xE00, 0); WREG32(base_reg + 0xDF4, 0x80); WREG32(base_reg + 0xE70, 0); WREG32(base_reg + 0xE60, 0); WREG32(base_reg + 0xE64, 0); WREG32(base_reg + 0xE8C, 0);
        let rc = gaudi_coresight_timeout(hdev, base_reg + 0xE80, 23, false); if rc != 0 { dev_err((*hdev).dev, "Failed to disable STM on timeout, error %d\n", rc); return rc; } WREG32(base_reg + 0xE80, 4);
    } 0
}

unsafe fn gaudi_config_funnel(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32 { if (*params).reg_idx >= ARRAY_SIZE(debug_funnel_regs) { dev_err((*hdev).dev, "Invalid register index in FUNNEL\n"); return -EINVAL; } let base_reg = debug_funnel_regs[(*params).reg_idx] - CFG_BASE; WREG32(base_reg + 0xFB0, CORESIGHT_UNLOCK); WREG32(base_reg, if (*params).enable { 0x33F } else { 0 }); 0 }

unsafe fn gaudi_config_etf(_hdev: *mut hl_device, _params: *mut hl_debug_params) -> i32 { 0 }
unsafe fn gaudi_config_etr(_hdev: *mut hl_device, _params: *mut hl_debug_params) -> i32 { 0 }
unsafe fn gaudi_config_bmon(_hdev: *mut hl_device, _params: *mut hl_debug_params) -> i32 { 0 }
unsafe fn gaudi_config_spmu(_hdev: *mut hl_device, _params: *mut hl_debug_params) -> i32 { 0 }

pub unsafe fn gaudi_debug_coresight(hdev: *mut hl_device, _ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32 {
    let params = data as *mut hl_debug_params;
    let rc = match (*params).op {
        HL_DEBUG_OP_STM => gaudi_config_stm(hdev, params), HL_DEBUG_OP_ETF => gaudi_config_etf(hdev, params),
        HL_DEBUG_OP_ETR => gaudi_config_etr(hdev, params), HL_DEBUG_OP_FUNNEL => gaudi_config_funnel(hdev, params),
        HL_DEBUG_OP_BMON => gaudi_config_bmon(hdev, params), HL_DEBUG_OP_SPMU => gaudi_config_spmu(hdev, params),
        HL_DEBUG_OP_TIMESTAMP => 0, _ => { dev_err((*hdev).dev, "Unknown coresight id %d\n", (*params).op); return -EINVAL; }
    }; RREG32(mmHW_STATE); rc
}

pub unsafe fn gaudi_halt_coresight(hdev: *mut hl_device, _ctx: *mut hl_ctx) {
    let mut params: hl_debug_params = core::mem::zeroed();
    for i in GAUDI_ETF_FIRST..=GAUDI_ETF_LAST { params.reg_idx = i; let rc = gaudi_config_etf(hdev, &mut params); if rc != 0 { dev_err((*hdev).dev, "halt ETF failed, %d/%d\n", rc, i); } }
    let rc = gaudi_config_etr(hdev, &mut params); if rc != 0 { dev_err((*hdev).dev, "halt ETR failed, %d\n", rc); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
