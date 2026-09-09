/* Rust translation of dcn31_clk_mgr.c. External types, constants, and functions
 * are supplied by the surrounding driver translation unit. */

const REG_CLK1_CLK_PLL_REQ: u32 = 0x0237;
const REG_CLK1_CLK_PLL_REQ_BASE_IDX: u32 = 0;
const CLK1_CLK_PLL_REQ_FB_MULT_INT_SHIFT: u32 = 0x0;
const CLK1_CLK_PLL_REQ_PLL_SPINE_DIV_SHIFT: u32 = 0xc;
const CLK1_CLK_PLL_REQ_FB_MULT_FRAC_SHIFT: u32 = 0x10;
const CLK1_CLK_PLL_REQ_FB_MULT_INT_MASK: u32 = 0x000001ff;
const CLK1_CLK_PLL_REQ_PLL_SPINE_DIV_MASK: u32 = 0x0000f000;
const CLK1_CLK_PLL_REQ_FB_MULT_FRAC_MASK: u32 = 0xffff0000;

static mut DCN31_BW_PARAMS: clk_bw_params = clk_bw_params {
    vram_type: Ddr4MemType,
    num_channels: 1,
    clk_table: clk_table { num_entries: 4, ..clk_table::ZERO },
    ..clk_bw_params::ZERO
};

static mut DDR5_WM_TABLE: wm_table = wm_table { entries: [
    wm_entry { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0, valid: true },
    wm_entry { wm_inst: WM_B, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0, valid: true },
    wm_entry { wm_inst: WM_C, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0, valid: true },
    wm_entry { wm_inst: WM_D, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.72, sr_exit_time_us: 9.0, sr_enter_plus_exit_time_us: 11.0, valid: true },
], ..wm_table::ZERO };

static mut LPDDR5_WM_TABLE: wm_table = wm_table { entries: [
    wm_entry { wm_inst: WM_A, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true },
    wm_entry { wm_inst: WM_B, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true },
    wm_entry { wm_inst: WM_C, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true },
    wm_entry { wm_inst: WM_D, wm_type: WM_TYPE_PSTATE_CHG, pstate_latency_us: 11.65333, sr_exit_time_us: 11.5, sr_enter_plus_exit_time_us: 14.5, valid: true },
], ..wm_table::ZERO };

static mut DUMMY_CLOCKS: DpmClocks_t = DpmClocks_t::ZERO;
static mut DUMMY_WMS: dcn31_watermarks = dcn31_watermarks::ZERO;

unsafe fn dcn31_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut display_count = 0;
    let mut tmds_present = false;
    for i in 0..(*context).stream_count {
        let stream = *(*context).streams.add(i as usize);
        if (*stream).signal == SIGNAL_TYPE_HDMI_TYPE_A || (*stream).signal == SIGNAL_TYPE_DVI_SINGLE_LINK || (*stream).signal == SIGNAL_TYPE_DVI_DUAL_LINK { tmds_present = true; }
        if dc_is_dp_signal((*stream).signal) && !(*stream).dpms_off { display_count += 1; }
        if dc_is_hdmi_frl_signal((*stream).signal) { display_count += 1; }
    }
    for i in 0..(*dc).link_count {
        let link = *(*dc).links.add(i as usize);
        if !(*link).link_enc.is_null() && (*(*link).link_enc).funcs.is_dig_enabled.is_some() && ((*(*(*link).link_enc).funcs).is_dig_enabled.unwrap())((*link).link_enc) { display_count += 1; }
    }
    if display_count == 0 && tmds_present { display_count = 1; }
    display_count
}

unsafe fn dcn31_disable_otg_wa(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, disable: bool) {
    let dc = (*(*clk_mgr_base).ctx).dc;
    for i in 0..(*(*dc).res_pool).pipe_count {
        let pipe = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        if !pipe.top_pipe.is_null() || !pipe.prev_odm_pipe.is_null() { continue; }
        if !pipe.stream.is_null() && ((*pipe.stream).dpms_off || dc_is_virtual_signal((*pipe.stream).signal)) {
            if disable { ((*(*pipe.stream_res.tg).funcs).immediate_disable_crtc.unwrap())(pipe.stream_res.tg); reset_sync_context_for_pipe(dc, context, i); }
            else { ((*(*pipe.stream_res.tg).funcs).enable_crtc.unwrap())(pipe.stream_res.tg); }
        }
    }
}

pub unsafe fn dcn31_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc = (*(*clk_mgr_base).ctx).dc;
    let mut update_dppclk = false; let mut update_dispclk = false; let mut dpp_clock_lowered = false;
    if (*dc).work_arounds.skip_clock_update { return; }
    if safe_to_lower {
        if new_clocks.zstate_support != DCN_ZSTATE_SUPPORT_DISALLOW && new_clocks.zstate_support != (*clk_mgr_base).clks.zstate_support { dcn31_smu_set_zstate_support(clk_mgr, new_clocks.zstate_support); (*clk_mgr_base).clks.zstate_support = new_clocks.zstate_support; }
        if (*clk_mgr_base).clks.dtbclk_en && !new_clocks.dtbclk_en { dcn31_smu_set_dtbclk(clk_mgr, false); (*clk_mgr_base).clks.dtbclk_en = new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER && dcn31_get_active_display_cnt_wa(dc, context) == 0 { let mut idle: display_idle_optimization_u = core::mem::zeroed(); idle.idle_info.df_request_disabled=1; idle.idle_info.phy_ref_clk_off=1; idle.idle_info.s0i2_rdy=1; dcn31_smu_set_display_idle_optimization(clk_mgr, idle.data); (*clk_mgr_base).clks.pwr_state=DCN_PWR_STATE_LOW_POWER; }
    } else {
        if new_clocks.zstate_support == DCN_ZSTATE_SUPPORT_DISALLOW && new_clocks.zstate_support != (*clk_mgr_base).clks.zstate_support { dcn31_smu_set_zstate_support(clk_mgr, DCN_ZSTATE_SUPPORT_DISALLOW); (*clk_mgr_base).clks.zstate_support=new_clocks.zstate_support; }
        if !(*clk_mgr_base).clks.dtbclk_en && new_clocks.dtbclk_en { dcn31_smu_set_dtbclk(clk_mgr, true); (*clk_mgr_base).clks.dtbclk_en=new_clocks.dtbclk_en; }
        if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_MISSION_MODE { let idle: display_idle_optimization_u=core::mem::zeroed(); dcn31_smu_set_display_idle_optimization(clk_mgr, idle.data); (*clk_mgr_base).clks.pwr_state=DCN_PWR_STATE_MISSION_MODE; }
    }
    if should_set_clock(safe_to_lower,new_clocks.dcfclk_khz,(*clk_mgr_base).clks.dcfclk_khz) { (*clk_mgr_base).clks.dcfclk_khz=new_clocks.dcfclk_khz; dcn31_smu_set_hard_min_dcfclk(clk_mgr,(*clk_mgr_base).clks.dcfclk_khz); }
    if should_set_clock(safe_to_lower,new_clocks.dcfclk_deep_sleep_khz,(*clk_mgr_base).clks.dcfclk_deep_sleep_khz) { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz=new_clocks.dcfclk_deep_sleep_khz; dcn31_smu_set_min_deep_sleep_dcfclk(clk_mgr,(*clk_mgr_base).clks.dcfclk_deep_sleep_khz); }
    if new_clocks.dppclk_khz < 100000 { new_clocks.dppclk_khz=100000; }
    if should_set_clock(safe_to_lower,new_clocks.dppclk_khz,(*clk_mgr_base).clks.dppclk_khz) { if (*clk_mgr_base).clks.dppclk_khz > new_clocks.dppclk_khz { dpp_clock_lowered=true; } (*clk_mgr_base).clks.dppclk_khz=new_clocks.dppclk_khz; update_dppclk=true; }
    if should_set_clock(safe_to_lower,new_clocks.dispclk_khz,(*clk_mgr_base).clks.dispclk_khz) { dcn31_disable_otg_wa(clk_mgr_base,context,true); (*clk_mgr_base).clks.dispclk_khz=new_clocks.dispclk_khz; dcn31_smu_set_dispclk(clk_mgr,(*clk_mgr_base).clks.dispclk_khz); dcn31_disable_otg_wa(clk_mgr_base,context,false); update_dispclk=true; }
    if dpp_clock_lowered { dcn20_update_clocks_update_dpp_dto(clk_mgr,context,safe_to_lower); dcn31_smu_set_dppclk(clk_mgr,(*clk_mgr_base).clks.dppclk_khz); } else { if update_dppclk || update_dispclk { dcn31_smu_set_dppclk(clk_mgr,(*clk_mgr_base).clks.dppclk_khz); } if new_clocks.dppclk_khz >= (*(*dc).current_state).bw_ctx.bw.dcn.clk.dppclk_khz { dcn20_update_clocks_update_dpp_dto(clk_mgr,context,safe_to_lower); } }
    cmd.notify_clocks.header.type_=DMUB_CMD__CLK_MGR; cmd.notify_clocks.header.sub_type=DMUB_CMD__CLK_MGR_NOTIFY_CLOCKS; cmd.notify_clocks.clocks.dcfclk_khz=(*clk_mgr_base).clks.dcfclk_khz; cmd.notify_clocks.clocks.dcfclk_deep_sleep_khz=(*clk_mgr_base).clks.dcfclk_deep_sleep_khz; cmd.notify_clocks.clocks.dispclk_khz=(*clk_mgr_base).clks.dispclk_khz; cmd.notify_clocks.clocks.dppclk_khz=(*clk_mgr_base).clks.dppclk_khz; dc_wake_and_execute_dmub_cmd((*(*clk_mgr_base).ctx),&mut cmd,DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dcn31_init_clocks(clk_mgr: *mut clk_mgr) { let ref_dtbclk=(*clk_mgr).clks.ref_dtbclk_khz; core::ptr::write_bytes(&mut (*clk_mgr).clks as *mut dc_clocks as *mut u8,0,core::mem::size_of::<dc_clocks>()); (*clk_mgr).clks.ref_dtbclk_khz=ref_dtbclk; (*clk_mgr).clks.p_state_change_support=true; (*clk_mgr).clks.prev_p_state_change_support=true; (*clk_mgr).clks.pwr_state=DCN_PWR_STATE_UNKNOWN; (*clk_mgr).clks.zstate_support=DCN_ZSTATE_SUPPORT_UNKNOWN; }
pub unsafe fn dcn31_are_clock_states_equal(a:*mut dc_clocks,b:*mut dc_clocks)->bool { (*a).dispclk_khz==(*b).dispclk_khz && (*a).dppclk_khz==(*b).dppclk_khz && (*a).dcfclk_khz==(*b).dcfclk_khz && (*a).dcfclk_deep_sleep_khz==(*b).dcfclk_deep_sleep_khz && (*a).zstate_support==(*b).zstate_support && (*a).dtbclk_en==(*b).dtbclk_en }
pub unsafe fn dcn31_get_dtb_ref_freq_khz(clk_mgr_base:*mut clk_mgr)->i32 { (*clk_mgr_base).clks.ref_dtbclk_khz }

unsafe fn dcn31_set_low_power_state(clk_mgr_base:*mut clk_mgr) { let clk_mgr=TO_CLK_MGR_INTERNAL(clk_mgr_base); let dc=(*(*clk_mgr_base).ctx).dc; if (*clk_mgr_base).clks.pwr_state != DCN_PWR_STATE_LOW_POWER && dcn31_get_active_display_cnt_wa(dc,(*dc).current_state)==0 { let mut idle:display_idle_optimization_u=core::mem::zeroed(); idle.idle_info.df_request_disabled=1; idle.idle_info.phy_ref_clk_off=1; idle.idle_info.s0i2_rdy=1; dcn31_smu_set_display_idle_optimization(clk_mgr,idle.data); (*clk_mgr_base).clks.pwr_state=DCN_PWR_STATE_LOW_POWER; } }

unsafe fn find_max_clk_value(clocks:*const u32,num_clocks:u32)->u32 { let mut max=0; for i in 0..num_clocks { if *clocks.add(i as usize)>max {max=*clocks.add(i as usize);} } max }
unsafe fn find_clk_for_voltage(clock_table:*const DpmClocks_t,clocks:*const u32,voltage:u32)->u32 { let mut max_voltage=0; let mut clock=0; for i in 0..NUM_SOC_VOLTAGE_LEVELS { let v=(*clock_table).SocVoltage[i]; if v==voltage{return *clocks.add(i);} else if v>=max_voltage && v<voltage {max_voltage=v;clock=*clocks.add(i);} } ASSERT(clock!=0);clock }

unsafe fn dcn31_build_watermark_ranges(bw_params:*mut clk_bw_params,table:*mut dcn31_watermarks) { let mut n=0usize; for i in 0..WM_SET_COUNT { if !(*bw_params).wm_table.entries[i].valid {continue;} let row=&mut (*table).WatermarkRow[WM_DCFCLK][n]; row.WmSetting=(*bw_params).wm_table.entries[i].wm_inst as u8; row.WmType=(*bw_params).wm_table.entries[i].wm_type as u8; row.MinClock=0;row.MaxClock=0xffff; if row.WmType==WM_TYPE_PSTATE_CHG as u8 { row.MinMclk=if i==0 {0} else {((*bw_params).clk_table.entries[i-1].dcfclk_mhz+1) as u16}; row.MaxMclk=(*bw_params).clk_table.entries[i].dcfclk_mhz as u16; } else { row.MinClock=0;row.MaxClock=0xffff;(*table).WatermarkRow[WM_DCFCLK][n-1].MaxClock=0xffff; } n+=1; } ASSERT(n!=0); (*table).WatermarkRow[WM_DCFCLK][0].MinMclk=0;(*table).WatermarkRow[WM_DCFCLK][0].MinClock=0;(*table).WatermarkRow[WM_DCFCLK][n-1].MaxMclk=0xffff;(*table).WatermarkRow[WM_DCFCLK][n-1].MaxClock=0xffff; let row=&mut (*table).WatermarkRow[WM_SOCCLK][0];row.WmSetting=WM_A;row.MinClock=0;row.MaxClock=0xffff;row.MinMclk=0;row.MaxMclk=0xffff; }

unsafe fn dcn31_notify_wm_ranges(base:*mut clk_mgr) { let c=TO_CLK_MGR_INTERNAL(base); let d=TO_CLK_MGR_DCN31(c); let table=(*d).smu_wm_set.wm_set; if (*c).smu_ver==0 || table.is_null() || (*d).smu_wm_set.mc_address.quad_part==0{return;} core::ptr::write_bytes(table as *mut u8,0,core::mem::size_of::<dcn31_watermarks>()); dcn31_build_watermark_ranges((*base).bw_params,table); dcn31_smu_set_dram_addr_high(c,(*d).smu_wm_set.mc_address.high_part);dcn31_smu_set_dram_addr_low(c,(*d).smu_wm_set.mc_address.low_part);dcn31_smu_transfer_wm_table_dram_2_smu(c); }

unsafe fn dcn31_clk_mgr_helper_populate_bw_params(c:*mut clk_mgr_internal,bios:*mut integrated_info,table:*const DpmClocks_t) { let bw=(*c).base.bw_params; let mut j=-1; for i in (0..NUM_DF_PSTATE_LEVELS).rev() {if (*table).DfPstateTable[i].FClk!=0 {j=i as i32;break;}} if j<0 {ASSERT(false);return;} (*bw).clk_table.num_entries=(j+1) as u32; let maxd=find_max_clk_value((*table).DispClocks.as_ptr(),(*table).NumDispClkLevelsEnabled);let maxp=find_max_clk_value((*table).DppClocks.as_ptr(),(*table).NumDispClkLevelsEnabled); for e in 0..(*bw).clk_table.num_entries as usize {let p=&(*table).DfPstateTable[(j as usize)-e];let q=&mut (*bw).clk_table.entries[e];q.fclk_mhz=p.FClk;q.memclk_mhz=p.MemClk;q.voltage=p.Voltage;q.wck_ratio=match p.WckRatio {WCK_RATIO_1_2=>2,WCK_RATIO_1_4=>4,_=>1};q.dcfclk_mhz=find_clk_for_voltage(table,(*table).DcfClocks.as_ptr(),p.Voltage);q.socclk_mhz=find_clk_for_voltage(table,(*table).SocClocks.as_ptr(),p.Voltage);q.dispclk_mhz=maxd;q.dppclk_mhz=maxp;} (*bw).vram_type=(*bios).memory_type;(*bw).dram_channel_width_bytes=if (*bios).memory_type==0x22 {8}else{4};(*bw).num_channels=if (*bios).ma_channel_number!=0 {(*bios).ma_channel_number}else{4}; }

// Function table and constructor are intentionally kept as external-driver-facing declarations.
extern "C" { pub fn dcn31_clk_mgr_construct(ctx:*mut dc_context,clk_mgr:*mut clk_mgr_dcn31,pp_smu:*mut pp_smu_funcs,dccg:*mut dccg); pub fn dcn31_clk_mgr_destroy(clk_mgr_int:*mut clk_mgr_internal); }

unsafe fn get_vco_frequency_from_reg(clk_mgr:*mut clk_mgr_internal)->i32 { let mut frac=0u32;let mut intv=0u32; REG_GET!(CLK1_CLK_PLL_REQ,FbMult_frac,&mut frac);REG_GET!(CLK1_CLK_PLL_REQ,FbMult_int,&mut intv);let mut pll=dc_fixpt_from_int(intv);pll.value |= frac<<16;pll=dc_fixpt_mul_int(pll,(*clk_mgr).dfs_ref_freq_khz);dc_fixpt_floor(pll) }
unsafe fn dcn31_enable_pme_wa(base:*mut clk_mgr) { dcn31_smu_enable_pme_wa(TO_CLK_MGR_INTERNAL(base)); }
unsafe fn dcn31_get_dpm_table_from_smu(clk_mgr:*mut clk_mgr_internal,smu:*mut dcn31_smu_dpm_clks) { if (*clk_mgr).smu_ver==0 || (*smu).dpm_clks.is_null() || (*smu).mc_address.quad_part==0{return;} core::ptr::write_bytes((*smu).dpm_clks as *mut u8,0,core::mem::size_of::<DpmClocks_t>());dcn31_smu_set_dram_addr_high(clk_mgr,(*smu).mc_address.high_part);dcn31_smu_set_dram_addr_low(clk_mgr,(*smu).mc_address.low_part);dcn31_smu_transfer_dpm_table_smu_2_dram(clk_mgr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
