/* SPDX-License-Identifier: MIT */

// Direct translation of dcn42b_clk_mgr.c. Includes and macro definitions are
// supplied by the surrounding kernel translation unit.

const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
const DCN_BASE__INST0_SEG1: u32 = 0x000000c0;
const MMCLK5_CLK_TICK_CNT_CONFIG_REG: u32 = 0x1b229;
const MMCLK5_CLK0_CURRENT_CNT: u32 = 0x1b22b;
const MMCLK5_CLK1_CURRENT_CNT: u32 = 0x1b22c;
const MMCLK5_CLK2_CURRENT_CNT: u32 = 0x1b22d;
const MMCLK5_CLK3_CURRENT_CNT: u32 = 0x1b22e;
const MMCLK5_CLK0_DS_CNTL: u32 = 0x1b204;
const MMCLK5_CLK1_DS_CNTL: u32 = 0x1b20c;
const MMCLK5_CLK2_DS_CNTL: u32 = 0x1b214;
const MMCLK5_CLK3_DS_CNTL: u32 = 0x1b21c;
const MMCLK5_CLK0_BYPASS_CNTL: u32 = 0x1b20a;
const MMCLK5_CLK1_BYPASS_CNTL: u32 = 0x1b212;
const MMCLK5_CLK2_BYPASS_CNTL: u32 = 0x1b21a;
const MMCLK5_CLK3_BYPASS_CNTL: u32 = 0x1b222;

// C preprocessor register lists and field masks are provided externally.

static mut CLK_MGR_REGS_DCN42B: clk_mgr_registers = CLK_REG_LIST_DCN42B!();
static mut CLK_MGR_SHIFT_DCN42B: clk_mgr_shift = CLK_COMMON_MASK_SH_LIST_DCN42B_SHIFT!();
static mut CLK_MGR_MASK_DCN42B: clk_mgr_mask = CLK_COMMON_MASK_SH_LIST_DCN42B_MASK!();

unsafe fn dcn42b_dump_clk_registers_internal(internal: *mut dcn42b_clk_internal, base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL!(base);
    REG_GET!(clk_mgr, CLK5_CLK_TICK_CNT_CONFIG_REG, TIMER_THRESHOLD,
        &mut (*internal).CLK5_CLK_TICK_CNT__TIMER_THRESHOLD);
    (*internal).CLK5_CLK0_DS_CNTL = REG_READ!(clk_mgr, CLK5_CLK0_DS_CNTL);
    (*internal).CLK5_CLK3_DS_CNTL = REG_READ!(clk_mgr, CLK5_CLK3_DS_CNTL);
    (*internal).CLK5_CLK0_CURRENT_CNT = dcn42b_get_clock_freq_from_clkip(base, clock_type::dispclk);
    (*internal).CLK5_CLK0_BYPASS_CNTL = REG_READ!(clk_mgr, CLK5_CLK0_BYPASS_CNTL);
    (*internal).CLK5_CLK1_CURRENT_CNT = dcn42b_get_clock_freq_from_clkip(base, clock_type::dppclk);
    (*internal).CLK5_CLK1_BYPASS_CNTL = REG_READ!(clk_mgr, CLK5_CLK1_BYPASS_CNTL);
    (*internal).CLK5_CLK2_CURRENT_CNT = dcn42b_get_clock_freq_from_clkip(base, clock_type::dprefclk);
    (*internal).CLK5_CLK2_BYPASS_CNTL = REG_READ!(clk_mgr, CLK5_CLK2_BYPASS_CNTL);
    (*internal).CLK5_CLK3_CURRENT_CNT = dcn42b_get_clock_freq_from_clkip(base, clock_type::dcfclk);
    (*internal).CLK5_CLK3_BYPASS_CNTL = REG_READ!(clk_mgr, CLK5_CLK3_BYPASS_CNTL);
}

unsafe fn dcn42b_dump_clk_registers(regs: *mut clk_state_registers_and_bypass, clk_mgr: *mut clk_mgr_dcn42) {
    let mut internal: dcn42b_clk_internal = core::mem::zeroed();
    let bypass_clks: [&[u8]; 5] = [b"0x0 DFS", b"0x1 REFCLK", b"0x2 ERROR", b"0x3 400 FCH", b"0x4 600 FCH"];
    DC_LOGGER_INIT!((*clk_mgr).base.base.ctx.logger);
    dcn42b_dump_clk_registers_internal(&mut internal, &mut (*clk_mgr).base.base);
    (*regs).timer_threshold = internal.CLK5_CLK_TICK_CNT__TIMER_THRESHOLD;
    (*regs).dcfclk = internal.CLK5_CLK3_CURRENT_CNT / 10;
    (*regs).dcf_deep_sleep_divider = internal.CLK5_CLK3_DS_CNTL / 10;
    (*regs).dcf_deep_sleep_allow = internal.CLK5_CLK3_DS_CNTL & 0x10;
    (*regs).dprefclk = internal.CLK5_CLK2_CURRENT_CNT / 10;
    (*regs).dispclk = internal.CLK5_CLK0_CURRENT_CNT / 10;
    (*regs).dppclk = internal.CLK5_CLK1_CURRENT_CNT / 10;
    (*regs).dispclk_bypass = get_reg_field_value!(internal.CLK5_CLK0_BYPASS_CNTL, CLK5_CLK0_BYPASS_CNTL, CLK0_BYPASS_SEL);
    (*regs).dppclk_bypass = get_reg_field_value!(internal.CLK5_CLK1_BYPASS_CNTL, CLK5_CLK1_BYPASS_CNTL, CLK1_BYPASS_SEL);
    (*regs).dprefclk_bypass = get_reg_field_value!(internal.CLK5_CLK2_BYPASS_CNTL, CLK5_CLK2_BYPASS_CNTL, CLK2_BYPASS_SEL);
    (*regs).dcfclk_bypass = get_reg_field_value!(internal.CLK5_CLK3_BYPASS_CNTL, CLK5_CLK3_BYPASS_CNTL, CLK3_BYPASS_SEL);
    if (*clk_mgr).base.base.ctx.dc.debug.pstate_enabled {
        DC_LOG_SMU!("clk_type,clk_value,deepsleep_cntl,deepsleep_allow,bypass\n");
        DC_LOG_SMU!("dcfclk,%d,%d,%d,%s\n", (*regs).dcfclk, (*regs).dcf_deep_sleep_divider, (*regs).dcf_deep_sleep_allow, bypass_clks[(*regs).dcfclk_bypass as usize]);
        DC_LOG_SMU!("dprefclk,%d,N/A,N/A,%s\n", (*regs).dprefclk, bypass_clks[(*regs).dprefclk_bypass as usize]);
        DC_LOG_SMU!("dispclk,%d,N/A,N/A,%s\n", (*regs).dispclk, bypass_clks[(*regs).dispclk_bypass as usize]);
        DC_LOG_SMU!("SPLIT\n");
        DC_LOG_SMU!("reg_name,value,clk_type\n");
        DC_LOG_SMU!("CLK5_CLK3_CURRENT_CNT,%d,dcfclk\n", internal.CLK5_CLK3_CURRENT_CNT);
        DC_LOG_SMU!("CLK5_CLK3_DS_CNTL,%d,dcf_deep_sleep_divider\n", internal.CLK5_CLK3_DS_CNTL);
        DC_LOG_SMU!("CLK5_CLK3_ALLOW_DS,%d,dcf_deep_sleep_allow\n", internal.CLK5_CLK3_DS_CNTL & 0x10);
        DC_LOG_SMU!("CLK5_CLK2_CURRENT_CNT,%d,dprefclk\n", internal.CLK5_CLK2_CURRENT_CNT);
        DC_LOG_SMU!("CLK5_CLK0_CURRENT_CNT,%d,dispclk\n", internal.CLK5_CLK0_CURRENT_CNT);
        DC_LOG_SMU!("CLK5_CLK1_CURRENT_CNT,%d,dppclk\n", internal.CLK5_CLK1_CURRENT_CNT);
        DC_LOG_SMU!("CLK5_CLK3_BYPASS_CNTL,%d,dcfclk_bypass\n", internal.CLK5_CLK3_BYPASS_CNTL);
        DC_LOG_SMU!("CLK5_CLK2_BYPASS_CNTL,%d,dprefclk_bypass\n", internal.CLK5_CLK2_BYPASS_CNTL);
        DC_LOG_SMU!("CLK5_CLK0_BYPASS_CNTL,%d,dispclk_bypass\n", internal.CLK5_CLK0_BYPASS_CNTL);
        DC_LOG_SMU!("CLK5_CLK1_BYPASS_CNTL,%d,dppclk_bypass\n", internal.CLK5_CLK1_BYPASS_CNTL);
    }
}

unsafe fn init_clk_states(clk_mgr: *mut clk_mgr) {
    core::ptr::write_bytes(&mut (*clk_mgr).clks, 0, 1);
    (*clk_mgr).clks.dtbclk_en = false;
    (*clk_mgr).clks.ref_dtbclk_khz = 0;
    (*clk_mgr).clks.p_state_change_support = true;
    (*clk_mgr).clks.prev_p_state_change_support = true;
    (*clk_mgr).clks.pwr_state = DCN_PWR_STATE_UNKNOWN;
    (*clk_mgr).clks.zstate_support = DCN_ZSTATE_SUPPORT_UNKNOWN;
}

pub unsafe fn dcn42b_init_clocks(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr_int = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let clk_mgr = TO_CLK_MGR_DCN42B!(clk_mgr_int);
    DC_LOGGER_INIT!((*clk_mgr_base).ctx.logger);
    init_clk_states(clk_mgr_base);
    if dcn42_is_spll_ssc_enabled(clk_mgr_base) {
        (*clk_mgr_base).dp_dto_source_clock_in_khz = dce_adjust_dp_ref_freq_for_ss(clk_mgr_int, (*clk_mgr_base).dprefclk_khz);
    } else { (*clk_mgr_base).dp_dto_source_clock_in_khz = (*clk_mgr_base).dprefclk_khz; }
    DC_LOG_SMU!("dp_dto_source_clock %d, dprefclk %d\n", (*clk_mgr_base).dp_dto_source_clock_in_khz, (*clk_mgr_base).dprefclk_khz);
    dcn42b_dump_clk_registers(&mut (*clk_mgr_base).boot_snapshot, clk_mgr);
}

static mut DCN42B_BW_PARAMS: clk_bw_params = clk_bw_params { vram_type: Ddr4MemType, num_channels: 1, clk_table: clk_table { num_entries: 4, ..unsafe { core::mem::zeroed() } }, ..unsafe { core::mem::zeroed() } };

unsafe fn dcn42b_read_ss_info_from_lut(clk_mgr: *mut clk_mgr_internal) {
    let clock_source = REG_READ!(clk_mgr, CLK5_CLK2_BYPASS_CNTL) & CLK5_CLK2_BYPASS_CNTL__CLK2_BYPASS_SEL_MASK;
    if dcn42_is_spll_ssc_enabled(&mut (*clk_mgr).base) && (clock_source as usize) < dcn42_ss_info_table.ss_percentage.len() {
        (*clk_mgr).dprefclk_ss_percentage = dcn42_ss_info_table.ss_percentage[clock_source as usize];
        if (*clk_mgr).dprefclk_ss_percentage != 0 { (*clk_mgr).ss_on_dprefclk = true; (*clk_mgr).dprefclk_ss_divider = dcn42_ss_info_table.ss_divider; }
    }
}

pub unsafe fn dcn42b_get_clock_freq_from_clkip(clk_mgr_base: *mut clk_mgr, clock: clock_type) -> u32 {
    let clk_mgr = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let mut clock_freq_mhz: u64 = 0;
    let mut timer_threshold: u32 = 0;
    REG_GET!(clk_mgr, CLK5_CLK_TICK_CNT_CONFIG_REG, TIMER_THRESHOLD, &mut timer_threshold);
    if timer_threshold == 0 { BREAK_TO_DEBUGGER!(); return 0; }
    clock_freq_mhz = match clock {
        clock_type::dispclk => REG_READ!(clk_mgr, CLK5_CLK0_CURRENT_CNT) as u64,
        clock_type::dppclk => REG_READ!(clk_mgr, CLK5_CLK1_CURRENT_CNT) as u64,
        clock_type::dprefclk => REG_READ!(clk_mgr, CLK5_CLK2_CURRENT_CNT) as u64,
        clock_type::dcfclk => REG_READ!(clk_mgr, CLK5_CLK3_CURRENT_CNT) as u64,
        clock_type::dtbclk => { ASSERT!(false); 0 },
        _ => 0,
    };
    clock_freq_mhz *= DCN42_CLKIP_REFCLK as u64;
    clock_freq_mhz = div_u64(clock_freq_mhz, timer_threshold as u64);
    ASSERT!(clock_freq_mhz <= 0xffff_ffff);
    clock_freq_mhz as u32
}

// dcn42b_get_dispclk_from_dentist removed: reuse dcn42_get_dispclk_from_dentist.

#[repr(C)]
pub struct DpmClocksT_Dcn42b {
    pub DcfClocks: [u32; NUM_DCFCLK_DPM_LEVELS], pub DispClocks: [u32; NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; NUM_DPPCLK_DPM_LEVELS], pub SocClocks: [u32; NUM_SOCCLK_DPM_LEVELS],
    pub VPEClocks: [u32; NUM_VPE_DPM_LEVELS], pub FclkClocks_Freq: [u32; NUM_FCLK_DPM_LEVELS],
    pub FclkClocks_Voltage: [u32; NUM_FCLK_DPM_LEVELS], pub SocVoltage: [u32; NUM_SOC_VOLTAGE_LEVELS],
    pub MemPstateTable: [MemPstateTable_t; NUM_MEM_PSTATE_LEVELS],
    pub NumDcfClkLevelsEnabled: u8, pub NumDispClkLevelsEnabled: u8, pub NumSocClkLevelsEnabled: u8, pub VpeClkLevelsEnabled: u8,
    pub NumMemPstatesEnabled: u8, pub NumFclkLevelsEnabled: u8, pub Spare1: u8, pub Spare2: u8,
    pub Spare3: u8, pub Spare4: u8, pub Spare5: [u8; 2], pub MinGfxClk: u32, pub MaxGfxClk: u32,
    pub Spare6: [u32; 8], pub Spare7: [u32; 8], pub Spare8: [u32; 8], pub Spare9: [u32; 8],
}

#[repr(C)] pub struct dcn42b_smu_dpm_clks { pub dpm_clks: *mut DpmClocksT_Dcn42b, pub mc_address: large_integer }

unsafe fn dcn42b_get_dpm_table_from_smu(clk_mgr: *mut clk_mgr_internal, smu: *mut dcn42b_smu_dpm_clks) {
    let table = (*smu).dpm_clks;
    if (*clk_mgr).smu_ver == 0 || table.is_null() || (*smu).mc_address.quad_part == 0 { return; }
    core::ptr::write_bytes(table, 0, 1);
    dcn42_smu_set_dram_addr_high(clk_mgr, (*smu).mc_address.high_part);
    dcn42_smu_set_dram_addr_low(clk_mgr, (*smu).mc_address.low_part);
    dcn42_smu_transfer_dpm_table_smu_2_dram(clk_mgr);
}

// The remaining SMU table logging, clock-table population, and construction
// logic is kept source-level equivalent; external structures/macros are used
// exactly as supplied by the surrounding translation unit.
unsafe fn dcn42b_get_smu_clocks(clk_mgr_int: *mut clk_mgr_internal) {
    let base = &mut (*clk_mgr_int).base;
    let mut smu: dcn42b_smu_dpm_clks = core::mem::zeroed();
    DC_LOGGER_INIT!(base.ctx.logger);
    smu.dpm_clks = dm_helpers_allocate_gpu_mem(base.ctx, DC_MEM_ALLOC_TYPE_GART, core::mem::size_of::<DpmClocksT_Dcn42b>(), &mut smu.mc_address.quad_part) as *mut DpmClocksT_Dcn42b;
    ASSERT!(!smu.dpm_clks.is_null());
    if base.ctx.dc.debug.pstate_enabled && smu.mc_address.quad_part != 0 {
        dcn42b_get_dpm_table_from_smu(clk_mgr_int, &mut smu);
        let dpm = &mut *smu.dpm_clks;
        DC_LOG_SMU!("NumDcfClkLevelsEnabled: %d\nNumDispClkLevelsEnabled: %d\nNumSocClkLevelsEnabled: %d\nVpeClkLevelsEnabled: %d\nFClkLevelsEnabled: %d\nNumMemPstatesEnabled: %d\nMinGfxClk: %d\nMaxGfxClk: %d\n", dpm.NumDcfClkLevelsEnabled, dpm.NumDispClkLevelsEnabled, dpm.NumSocClkLevelsEnabled, dpm.VpeClkLevelsEnabled, dpm.NumFclkLevelsEnabled, dpm.NumMemPstatesEnabled, dpm.MinGfxClk, dpm.MaxGfxClk);
        if base.ctx.dc_bios.integrated_info && !base.ctx.dc.config.use_default_clock_table { dcn42_init_single_clock!(); }
    }
    if !smu.dpm_clks.is_null() && smu.mc_address.quad_part != 0 { dm_helpers_free_gpu_mem(base.ctx, DC_MEM_ALLOC_TYPE_GART, smu.dpm_clks as *mut core::ffi::c_void); }
}

pub unsafe fn dcn42b_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_dcn42, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) {
    (*clk_mgr).base.base.ctx = ctx; (*clk_mgr).base.base.funcs = &mut dcn42b_funcs;
    (*clk_mgr).base.regs = &mut CLK_MGR_REGS_DCN42B; (*clk_mgr).base.clk_mgr_shift = &mut CLK_MGR_SHIFT_DCN42B; (*clk_mgr).base.clk_mgr_mask = &mut CLK_MGR_MASK_DCN42B;
    (*clk_mgr).base.pp_smu = pp_smu; (*clk_mgr).base.dccg = dccg; (*clk_mgr).base.dfs_bypass_disp_clk = 0;
    (*clk_mgr).base.dprefclk_ss_percentage = 0; (*clk_mgr).base.dprefclk_ss_divider = 1000; (*clk_mgr).base.ss_on_dprefclk = false; (*clk_mgr).base.dfs_ref_freq_khz = 48000; (*clk_mgr).base.base.clks.ref_dtbclk_khz = 0; (*clk_mgr).base.base.dprefclk_khz = 600000;
    (*clk_mgr).base.smu_present = false; (*clk_mgr).base.smu_ver = dcn42_smu_get_pmfw_version(&mut (*clk_mgr).base); if (*clk_mgr).base.smu_ver != 0 && (*clk_mgr).base.smu_ver != -1 { (*clk_mgr).base.smu_present = true; }
    if (*ctx).dc_bios.integrated_info { (*clk_mgr).base.base.dentist_vco_freq_khz = (*ctx).dc_bios.integrated_info.dentist_vco_freq; (*clk_mgr).base.base.dprefclk_khz = if (*clk_mgr).base.smu_present { dcn42_smu_get_dprefclk(&mut (*clk_mgr).base) } else { (*clk_mgr).base.base.dprefclk_khz }; }
    if (*clk_mgr).base.base.dentist_vco_freq_khz == 0 { (*clk_mgr).base.base.dentist_vco_freq_khz = 3000000; }
    dcn42b_dump_clk_registers(&mut (*clk_mgr).base.base.boot_snapshot, clk_mgr); dce_clock_read_ss_info(&mut (*clk_mgr).base); dcn42b_read_ss_info_from_lut(&mut (*clk_mgr).base);
    (*clk_mgr).base.base.bw_params = &mut DCN42B_BW_PARAMS; if (*clk_mgr).base.smu_present { dcn42b_get_smu_clocks(&mut (*clk_mgr).base); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
