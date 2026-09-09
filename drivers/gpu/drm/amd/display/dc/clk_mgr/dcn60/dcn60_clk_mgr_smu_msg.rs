// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding DCN60 driver are intentionally not
// implemented here: dcn60_clk_mgr_smu_msg.h, clk_mgr_internal.h, reg_helper.h,
// dalsmc.h, dcn401/dcn401_smu14_driver_if.h, and logger_types.h.

const MM_DAL_MSG_REG: u32 = 0x162A2;
const MM_DAL_RESP_REG: u32 = 0x162A3;
const MM_DAL_ARG_REG: u32 = 0x162A4;
const MM_DAL_ARG_REG_0: u32 = 0x162A4;
const MM_DAL_ARG_REG_1: u32 = 0x162A5;
const MM_DAL_ARG_REG_2: u32 = 0x162A6;
const MM_DAL_ARG_REG_3: u32 = 0x162A7;

// External types, constants, register helpers, logging, and tracing are
// provided by the translated driver dependencies.

unsafe fn dcn60_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
    total_delay_us: *mut u32,
) -> u32 {
    let mut reg: u32 = 0;

    if !total_delay_us.is_null() { *total_delay_us = 0; }

    loop {
        reg = REG_READ(MM_DAL_RESP_REG);
        if reg != 0 { break; }

        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        if !total_delay_us.is_null() { *total_delay_us += delay_us; }

        if max_retries == 0 { break; }
        max_retries -= 1;
    }
    let _ = clk_mgr;
    reg
}

unsafe fn dcn60_smu_send_msg_with_args(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    args: DALSMC_args_t,
    param_out: *mut u32,
    total_delay_us: *mut u32,
) -> bool {
    let mut delay1_us = 0;
    let mut delay2_us = 0;
    if !total_delay_us.is_null() { *total_delay_us = 0; }

    dcn60_smu_wait_for_response(clk_mgr, 10, 200000, if !total_delay_us.is_null() { &mut delay1_us } else { core::ptr::null_mut() });
    smu_print!("SMU msg 0x{:x} enter: arg0=0x{:08x} arg1=0x{:08x} arg2=0x{:08x} arg3=0x{:08x}\n", msg_id, args.Reg0, args.Reg1, args.Reg2, args.Reg3);
    TRACE_SMU_MSG_ENTER(msg_id, args.Reg0, (*clk_mgr).base.ctx);
    REG_WRITE(MM_DAL_RESP_REG, 0);
    REG_WRITE(MM_DAL_ARG_REG_0, args.Reg0);
    REG_WRITE(MM_DAL_ARG_REG_1, args.Reg1);
    REG_WRITE(MM_DAL_ARG_REG_2, args.Reg2);
    REG_WRITE(MM_DAL_ARG_REG_3, args.Reg3);
    REG_WRITE(MM_DAL_MSG_REG, msg_id);

    if dcn60_smu_wait_for_response(clk_mgr, 10, 200000, if !total_delay_us.is_null() { &mut delay2_us } else { core::ptr::null_mut() }) == DALSMC_Result_OK {
        if !param_out.is_null() { *param_out = REG_READ(MM_DAL_ARG_REG_0); }
        if !total_delay_us.is_null() { *total_delay_us = delay1_us + delay2_us; }
        smu_print!("SMU msg 0x{:x} exit: ok resp=0x{:08x}\n", msg_id, if !param_out.is_null() { *param_out } else { 0 });
        TRACE_SMU_MSG_EXIT(true, if !param_out.is_null() { *param_out } else { 0 }, (*clk_mgr).base.ctx);
        return true;
    }
    if !total_delay_us.is_null() { *total_delay_us = delay1_us + 2000000; }
    smu_print!("SMU msg 0x{:x} exit: failed\n", msg_id);
    TRACE_SMU_MSG_EXIT(false, 0, (*clk_mgr).base.ctx);
    false
}

unsafe fn dcn60_smu_get_hard_min_status(clk_mgr: *mut clk_mgr_internal, no_timeout: *mut bool, total_delay_us: *mut u32) -> u32 {
    let args = DALSMC_args_t::default();
    let mut response = 0;
    *no_timeout = dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_ReturnHardMinStatus, args, &mut response, total_delay_us);
    smu_print!("SMU Get hard min status: no_timeout {} delay {} us clk bits {:x}\n", *no_timeout, *total_delay_us, response);
    response
}

unsafe fn dcn60_smu_wait_hard_min_status(clk_mgr: *mut clk_mgr_internal, ppclk: u32) -> bool {
    let max_delay_us = 1000000;
    let hardmin_status_mask = 1u32 << ppclk;
    let mut total_delay_us = 0;
    let mut hardmin_done = false;
    while !hardmin_done && total_delay_us < max_delay_us {
        if total_delay_us > 0 { udelay(500); total_delay_us += 500; smu_print!("SMU Wait hard min status for {} us\n", total_delay_us); }
        let mut read_total_delay_us = 0;
        let mut no_timeout = false;
        let hardmin_status = dcn60_smu_get_hard_min_status(clk_mgr, &mut no_timeout, &mut read_total_delay_us);
        total_delay_us += read_total_delay_us;
        hardmin_done = (hardmin_status & hardmin_status_mask) != 0;
    }
    hardmin_done
}

pub unsafe fn dcn60_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> u32 {
    let mut arg = DALSMC_SetHardMinByFreq_arg_t::default();
    let mut response = 0;
    smu_print!("SMU Set hard min by freq: clk = {}, freq_mhz = {} MHz\n", clk, freq_mhz);
    arg.FreqKhz = (freq_mhz as u32) * 1000;
    arg.Ppclk = clk;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_SetHardMinByFreq, arg.Args, &mut response, core::ptr::null_mut());
    let hard_min_done = dcn60_smu_wait_hard_min_status(clk_mgr, clk);
    smu_print!("SMU Frequency set = {} KHz hard_min_done {}\n", response, hard_min_done);
    response
}

pub unsafe fn dcn60_smu_set_stutter_efficiency(clk_mgr: *mut clk_mgr_internal, base_efficiency: u8, low_power_efficiency: u8) {
    let mut arg = DALSMC_SetStutterEfficiency_arg_t::default();
    smu_print!("SMU Set stutter efficiencies: base(LP1) = {} percent, low power(LP2) = {} percent\n", base_efficiency, low_power_efficiency);
    arg.BaseEfficiencyPct = base_efficiency; arg.LowPowerEfficiencyPct = low_power_efficiency;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_SetStutterEfficiency, arg.Args, core::ptr::null_mut(), core::ptr::null_mut());
}

pub unsafe fn dcn60_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32) {
    let mut arg = DALSMC_SetMinDeepSleepDcfclk_arg_t::default();
    smu_print!("SMU Set min deep sleep dcfclk: freq_mhz = {} MHz\n", freq_mhz);
    arg.MinDcfclkMhz = freq_mhz;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_SetMinDeepSleepDcfclk, arg.Args, core::ptr::null_mut(), core::ptr::null_mut());
}

pub unsafe fn dcn60_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal) {
    let args = DALSMC_args_t::default();
    smu_print!("SMU Set PME workaround (BacoAudioD3PME)\n");
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_BacoAudioD3PME, args, core::ptr::null_mut(), core::ptr::null_mut());
}

pub unsafe fn dcn60_smu_indicate_pstate_status(clk_mgr: *mut clk_mgr_internal, allow_fclk: bool, allow_uclk: bool, wait_resp: bool, drr_enable: bool, alt_ch_enable: bool) {
    let mut arg = DALSMC_IndicatePstateStatus_arg_t::default();
    smu_print!("SMU Indicate pstate status: allow_fclk={} allow_uclk={} wait_resp={} drr_enable={} alt_ch_enable={}\n", allow_fclk, allow_uclk, wait_resp, drr_enable, alt_ch_enable);
    arg.AllowFclk = allow_fclk as u32; arg.AllowUclk = allow_uclk as u32; arg.WaitResp = wait_resp as u32; arg.DrrEnable = drr_enable as u32; arg.AltCh = alt_ch_enable as u32;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_IndicatePstateStatus, arg.Args, core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe fn dcn60_smu_transfer_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal, table_id: u32, dram_addr: i64) -> bool {
    let mut arg = DALSMC_TransferTable_arg_t::default();
    smu_print!("SMU TransferTableSmu2Dram: table_id=0x{:x} addr=0x{:08x}_{:08x}\n", table_id, (dram_addr >> 32) as u32, (dram_addr & 0xFFFFFFFF) as u32);
    arg.TableId = table_id; arg.AddrLow = (dram_addr & 0xFFFFFFFF) as u32; arg.AddrHigh = (dram_addr >> 32) as u32;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_TransferTableSmu2Dram, arg.Args, core::ptr::null_mut(), core::ptr::null_mut())
}

unsafe fn dcn60_smu_transfer_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal, table_id: u32, dram_addr: i64) -> bool {
    let mut arg = DALSMC_TransferTable_arg_t::default();
    smu_print!("SMU TransferTableDram2Smu: table_id=0x{:x} addr=0x{:08x}_{:08x}\n", table_id, (dram_addr >> 32) as u32, (dram_addr & 0xFFFFFFFF) as u32);
    arg.TableId = table_id; arg.AddrLow = (dram_addr & 0xFFFFFFFF) as u32; arg.AddrHigh = (dram_addr >> 32) as u32;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_TransferTableDram2Smu, arg.Args, core::ptr::null_mut(), core::ptr::null_mut())
}

pub unsafe fn dcn60_smu_set_soc_utm_table(clk_mgr: *mut clk_mgr_internal, dram_addr: i64) -> bool { dcn60_smu_transfer_table_dram_2_smu(clk_mgr, TABLE_SOC_UTM, dram_addr) }

pub unsafe fn dcn60_smu_get_dal_init_table(clk_mgr: *mut clk_mgr_internal, init_table: *mut *const DalInitTable_t) -> bool {
    if !dcn60_smu_transfer_table_smu_2_dram(clk_mgr, TABLE_DAL_INIT, (*clk_mgr).dal_init_table_addr) { return false; }
    *init_table = (*clk_mgr).dal_init_table as *const DalInitTable_t;
    true
}

pub unsafe fn dcn60_smu_update_utm_qos_request(clk_mgr: *mut clk_mgr_internal, latency_sop_index: u32, nominal_bandwidth_kbps: u32, urgent_bandwidth_kbps: u32, lsdma_bandwidth_kbps: u32) -> bool {
    let mut arg = DALSMC_UpdateUTMQoSRequest_arg_t::default();
    smu_print!("SMU UpdateUTMQoSRequest: sop_idx={} nominal={} urgent={} lsdma={} KBps\n", latency_sop_index, nominal_bandwidth_kbps, urgent_bandwidth_kbps, lsdma_bandwidth_kbps);
    arg.LatencySopIndex = latency_sop_index; arg.NominalBandwidthKBps = nominal_bandwidth_kbps; arg.UrgentBandwidthKBps = urgent_bandwidth_kbps; arg.LsdmaBandwidthKBps = lsdma_bandwidth_kbps;
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_UpdateUTMQoSRequest, arg.Args, core::ptr::null_mut(), core::ptr::null_mut())
}

pub unsafe fn dcn60_smu_get_msg_header_version(clk_mgr: *mut clk_mgr_internal, version: *mut u32) -> bool { dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_GetMsgHeaderVersion, DALSMC_args_t::default(), version, core::ptr::null_mut()) }

pub unsafe fn dcn60_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, is_idle: bool) {
    let mut arg = DALSMC_SetDisplayIdleOptimizations_arg_t::default();
    arg.DfRequestDisabled = is_idle as u32; arg.PhyRefClkOff = is_idle as u32; arg.S0i2Rdy = is_idle as u32;
    smu_print!("SMU SetDisplayIdleOptimizations: DfRequestDisabled={} PhyRefClkOff={} S0i2Rdy={}\n", arg.DfRequestDisabled, arg.PhyRefClkOff, arg.S0i2Rdy);
    dcn60_smu_send_msg_with_args(clk_mgr, DALSMC_MSG_SetDisplayIdleOptimizations, arg.Args, core::ptr::null_mut(), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
