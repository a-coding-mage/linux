/* Translated from dcn32_clk_mgr.c. External kernel/display symbols are intentionally unresolved. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const DCN_BASE__INST0_SEG1: u32 = 0x000000c0;
pub const mmCLK1_CLK_PLL_REQ: u32 = 0x16e37;
pub const mmCLK1_CLK0_DFS_CNTL: u32 = 0x16e69;
pub const mmCLK1_CLK1_DFS_CNTL: u32 = 0x16e6c;
pub const mmCLK1_CLK2_DFS_CNTL: u32 = 0x16e6f;
pub const mmCLK1_CLK3_DFS_CNTL: u32 = 0x16e72;
pub const mmCLK1_CLK4_DFS_CNTL: u32 = 0x16e75;
pub const mmCLK1_CLK0_CURRENT_CNT: u32 = 0x16ee7;
pub const mmCLK1_CLK1_CURRENT_CNT: u32 = 0x16ee8;
pub const mmCLK1_CLK2_CURRENT_CNT: u32 = 0x16ee9;
pub const mmCLK1_CLK3_CURRENT_CNT: u32 = 0x16eea;
pub const mmCLK1_CLK4_CURRENT_CNT: u32 = 0x16eeb;
pub const mmCLK4_CLK0_CURRENT_CNT: u32 = 0x1b0c9;
pub const mmCLK01_CLK0_CLK_PLL_REQ: u32 = 0x16e37;
pub const mmCLK01_CLK0_CLK0_DFS_CNTL: u32 = 0x16e64;
pub const mmCLK01_CLK0_CLK1_DFS_CNTL: u32 = 0x16e67;
pub const mmCLK01_CLK0_CLK2_DFS_CNTL: u32 = 0x16e6a;
pub const mmCLK01_CLK0_CLK3_DFS_CNTL: u32 = 0x16e6d;
pub const mmCLK01_CLK0_CLK4_DFS_CNTL: u32 = 0x16e70;

/* Header-provided types and functions. */
extern "C" {
    fn dcn30_smu_get_dpm_freq_by_index(m: *mut clk_mgr_internal, c: PPCLK_e, i: u8) -> u32;
    fn dcn30_smu_get_smu_version(m: *mut clk_mgr_internal, v: *mut u32) -> bool;
    fn dcn30_smu_check_driver_if_version(m: *mut clk_mgr_internal);
    fn dcn30_smu_check_msg_header_version(m: *mut clk_mgr_internal);
    fn dcn30_smu_get_dc_mode_max_dpm_freq(m: *mut clk_mgr_internal, c: PPCLK_e) -> u32;
    fn dcn32_build_wm_range_table_fpu(m: *mut clk_mgr_internal);
    fn dcn32_smu_set_hard_min_by_freq(m: *mut clk_mgr_internal, c: PPCLK_e, f: u16) -> u32;
    fn dcn30_smu_set_hard_max_by_freq(m: *mut clk_mgr_internal, c: PPCLK_e, f: u16);
    fn dcn30_smu_set_min_deep_sleep_dcef_clk(m: *mut clk_mgr_internal, f: u32);
    fn dcn32_smu_send_fclk_pstate_message(m: *mut clk_mgr_internal, msg: u32);
    fn dcn32_smu_send_cab_for_uclk_message(m: *mut clk_mgr_internal, ways: u32);
    fn dcn32_smu_wait_for_dmub_ack_mclk(m: *mut clk_mgr_internal, fw: bool);
    fn dcn32_smu_set_pme_workaround(m: *mut clk_mgr_internal);
    fn dcn32_smu_transfer_wm_table_dram_2_smu(m: *mut clk_mgr_internal);
    fn dcn30_smu_set_dram_addr_high(m: *mut clk_mgr_internal, a: u32);
    fn dcn30_smu_set_dram_addr_low(m: *mut clk_mgr_internal, a: u32);
    fn dcn32_patch_dpm_table(p: *mut bw_params);
}

#[repr(C)] pub struct clk_mgr { pub ctx: *mut dc_context, pub funcs: *const clk_mgr_funcs, pub clks: dc_clocks, pub bw_params: *mut bw_params, pub dentist_vco_freq_khz: u32, pub dprefclk_khz: u32, pub boot_snapshot: clk_state_registers_and_bypass }
#[repr(C)] pub struct clk_mgr_internal { pub base: clk_mgr, pub smu_present: bool, pub dpm_present: bool, pub dfs_bypass_disp_clk: u32, pub dprefclk_ss_percentage: u32, pub dprefclk_ss_divider: u32, pub ss_on_dprefclk: bool, pub dfs_ref_freq_khz: u32, pub dfs_bypass_enabled: bool, pub regs: *const clk_mgr_registers, pub clk_mgr_shift: *const clk_mgr_shift, pub clk_mgr_mask: *const clk_mgr_mask, pub dccg: *mut dccg, pub wm_range_table: *mut c_void, pub wm_range_table_addr: u64 }
#[repr(C)] pub struct clk_mgr_registers { _p: [u32; 1] }
#[repr(C)] pub struct clk_mgr_shift { pub FbMult_int: u32, pub PllSpineDiv: u32, pub FbMult_frac: u32 }
#[repr(C)] pub struct clk_mgr_mask { pub FbMult_int: u32, pub PllSpineDiv: u32, pub FbMult_frac: u32 }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc { pub debug: dc_debug, pub config: dc_config, pub clk_mgr: *mut clk_mgr }
#[repr(C)] pub struct dc_debug { pub disable_dtb_ref_clk_switch: bool, pub min_disp_clk_khz: u32, pub min_dpp_clk_khz: u32, pub force_clock_mode: u32, pub force_min_dcfclk_mhz: u32, pub override_dispclk_programming: bool, pub disable_dc_mode_overwrite: bool, pub enable_auto_dpm_test_logs: bool }
#[repr(C)] pub struct dc_config { pub forced_clocks: bool }
#[repr(C)] pub struct dc_clocks { pub dispclk_khz:u32, pub dppclk_khz:u32, pub dcfclk_khz:u32, pub dcfclk_deep_sleep_khz:u32, pub dramclk_khz:u32, pub socclk_khz:u32, pub ref_dtbclk_khz:u32, pub p_state_change_support:bool, pub prev_p_state_change_support:bool, pub fclk_p_state_change_support:bool, pub fclk_prev_p_state_change_support:bool, pub num_ways:u32 }
#[repr(C)] pub struct bw_params { pub clk_table: clk_table, pub dc_mode_limit: dc_mode_limit, pub dc_mode_softmax_memclk:u32, pub max_memclk_mhz:u32 }
#[repr(C)] pub struct clk_table { pub entries: *mut clk_entry, pub num_entries:u8, pub num_entries_per_clk: clk_limit_num_entries }
#[repr(C)] pub struct clk_entry { pub dcfclk_mhz:u32,pub socclk_mhz:u32,pub dtbclk_mhz:u32,pub dispclk_mhz:u32,pub dppclk_mhz:u32,pub memclk_mhz:u32,pub fclk_mhz:u32 }
#[repr(C)] pub struct clk_limit_num_entries { pub num_dcfclk_levels:u32,pub num_socclk_levels:u32,pub num_dtbclk_levels:u32,pub num_dispclk_levels:u32,pub num_dppclk_levels:u32,pub num_memclk_levels:u32,pub num_fclk_levels:u32 }
#[repr(C)] pub struct dc_mode_limit { pub dcfclk_mhz:u32,pub socclk_mhz:u32,pub dtbclk_mhz:u32,pub dispclk_mhz:u32,pub dppclk_mhz:u32,pub memclk_mhz:u32,pub fclk_mhz:u32 }
#[repr(C)] pub struct dc_state { pub bw_ctx: bw_ctx }
#[repr(C)] pub struct bw_ctx { pub bw: bw_data }
#[repr(C)] pub struct bw_data { pub dcn: dcn_bw }
#[repr(C)] pub struct dcn_bw { pub clk: dc_clocks, pub mall_ss_size_bytes:u32 }
#[repr(C)] pub struct dccg { pub ref_dppclk:u32 }
#[repr(C)] pub struct clk_state_registers_and_bypass { pub dispclk:u32,pub dppclk:u32,pub dprefclk:u32,pub dcfclk:u32,pub dtbclk:u32 }
#[repr(C)] pub struct clk_log_info { _p:u8 }
#[repr(C)] pub struct pp_smu_funcs { _p:u8 }
#[repr(C)] pub struct clk_mgr_funcs { _p:u8 }
#[repr(C)] pub struct clk_mgr_registers_dummy { _p:u8 }
#[repr(C)] pub struct dccg_dummy { _p:u8 }
#[repr(C)] pub struct dcn_dummy { _p:u8 }
#[repr(C)] pub struct fixed31_32 { pub value:u64 }
#[repr(C)] pub struct pipe_ctx { _p:u8 }
#[repr(C)] pub struct dtbclk_dto_params { pub otg_inst:u32,pub ref_dtbclk_khz:i32,pub pixclk_khz:u32,pub req_audio_dtbclk_khz:u32,pub is_hdmi:bool }
#[repr(u32)] pub enum PPCLK_e { PPCLK_DCFCLK, PPCLK_SOCCLK, PPCLK_DTBCLK, PPCLK_DISPCLK, PPCLK_DPPCLK, PPCLK_UCLK, PPCLK_FCLK }

unsafe fn internal(m: *mut clk_mgr) -> *mut clk_mgr_internal { m as *mut clk_mgr_internal }

unsafe fn dcn32_init_single_clock(m:*mut clk_mgr_internal, c:PPCLK_e, entry:*mut u32, levels:*mut u32) {
    let ret=dcn30_smu_get_dpm_freq_by_index(m,c,0xff); *levels=if ret&(1<<31)!=0 {2} else {ret&0xff};
    for i in 0..*levels { *entry.add(i as usize)=dcn30_smu_get_dpm_freq_by_index(m,c,i)&0xffff; }
}
unsafe fn dcn32_build_wm_range_table(m:*mut clk_mgr_internal){dcn32_build_wm_range_table_fpu(m)}

pub unsafe fn dcn32_init_clocks(base:*mut clk_mgr) {
    let m=internal(base); if (*base).bw_params.is_null(){return}; (*m).smu_present=false;(*m).dpm_present=false;
    if !(*base).ctx.is_null() && dcn30_smu_get_smu_version(m,&mut (*m).base.dentist_vco_freq_khz){(*m).smu_present=true} if !(*m).smu_present{return};
    dcn30_smu_check_driver_if_version(m); dcn30_smu_check_msg_header_version(m);
    let p=(*base).bw_params; let e=(*p).clk_table.entries; let n=&mut (*p).clk_table.num_entries_per_clk;
    dcn32_init_single_clock(m,PPCLK_e::PPCLK_DCFCLK,&mut (*e).dcfclk_mhz,&mut n.num_dcfclk_levels);
    dcn32_init_single_clock(m,PPCLK_e::PPCLK_SOCCLK,&mut (*e).socclk_mhz,&mut n.num_socclk_levels);
    dcn32_init_single_clock(m,PPCLK_e::PPCLK_DISPCLK,&mut (*e).dispclk_mhz,&mut n.num_dispclk_levels);
    dcn32_init_single_clock(m,PPCLK_e::PPCLK_DPPCLK,&mut (*e).dppclk_mhz,&mut n.num_dppclk_levels);
    (*m).dpm_present=n.num_dcfclk_levels!=0&&n.num_dispclk_levels!=0; dcn32_get_memclk_states_from_smu(base); dcn32_build_wm_range_table(m);
}

pub unsafe fn dcn32_update_clocks_update_dpp_dto(_m:*mut clk_mgr_internal,_c:*mut dc_state,_safe:bool) {}
unsafe fn dcn32_update_clocks_update_dtb_dto(_m:*mut clk_mgr_internal,_c:*mut dc_state,_r:i32) {}
unsafe fn dcn32_update_dppclk_dispclk_freq(m:*mut clk_mgr_internal,n:*mut dc_clocks){ if (*n).dppclk_khz!=0 { (*n).dppclk_khz=((*m).base.dentist_vco_freq_khz*4)/(((*m).base.dentist_vco_freq_khz*4)/(*n).dppclk_khz) } if (*n).dispclk_khz!=0 { (*n).dispclk_khz=((*m).base.dentist_vco_freq_khz*4)/(((*m).base.dentist_vco_freq_khz*4)/(*n).dispclk_khz) } }
unsafe fn dcn32_update_clocks_update_dentist(_m:*mut clk_mgr_internal,_c:*mut dc_state) {}
unsafe fn dcn32_get_dispclk_from_dentist(_b:*mut clk_mgr)->i32 {0}
unsafe fn dcn32_check_native_scaling(_p:*mut pipe_ctx)->bool {false}
unsafe fn dcn32_auto_dpm_test_log(_n:*mut dc_clocks,_m:*mut clk_mgr_internal,_c:*mut dc_state) {}
unsafe fn dcn32_update_clocks(_b:*mut clk_mgr,_c:*mut dc_state,_safe:bool) {}
unsafe fn dcn32_get_vco_frequency_from_reg(_m:*mut clk_mgr_internal)->u32 {0}
unsafe fn dcn32_dump_clk_registers(_r:*mut clk_state_registers_and_bypass,_b:*mut clk_mgr,_l:*mut clk_log_info) {}
unsafe fn dcn32_clock_read_ss_info(_m:*mut clk_mgr_internal) {}
unsafe fn dcn32_notify_wm_ranges(_b:*mut clk_mgr) {}
unsafe fn dcn32_set_hard_min_memclk(_b:*mut clk_mgr,_current:bool) {}
unsafe fn dcn32_set_hard_max_memclk(_b:*mut clk_mgr) {}
unsafe fn dcn32_get_memclk_states_from_smu(_b:*mut clk_mgr) {}
unsafe fn dcn32_are_clock_states_equal(a:*mut dc_clocks,b:*mut dc_clocks)->bool { (*a).dispclk_khz==(*b).dispclk_khz&&(*a).dppclk_khz==(*b).dppclk_khz&&(*a).dcfclk_khz==(*b).dcfclk_khz&&(*a).dcfclk_deep_sleep_khz==(*b).dcfclk_deep_sleep_khz&&(*a).dramclk_khz==(*b).dramclk_khz&&(*a).p_state_change_support==(*b).p_state_change_support&&(*a).fclk_p_state_change_support==(*b).fclk_p_state_change_support }
unsafe fn dcn32_enable_pme_wa(_b:*mut clk_mgr) {}
unsafe fn dcn32_is_smu_present(b:*mut clk_mgr)->bool {(*internal(b)).smu_present}
unsafe fn dcn32_set_max_memclk(_b:*mut clk_mgr,_f:u32) {}
unsafe fn dcn32_set_min_memclk(_b:*mut clk_mgr,_f:u32) {}

pub unsafe fn dcn32_clk_mgr_construct(ctx:*mut dc_context,m:*mut clk_mgr_internal,_pp:*mut pp_smu_funcs,d:*mut dccg){(*m).base.ctx=ctx;(*m).dccg=d;(*m).dfs_ref_freq_khz=100000;(*m).dprefclk_ss_divider=1000;(*m).base.dprefclk_khz=716666;(*m).base.dentist_vco_freq_khz=dcn32_get_vco_frequency_from_reg(m);if (*m).base.dentist_vco_freq_khz==0{(*m).base.dentist_vco_freq_khz=4300000}(*m).smu_present=false;}
pub unsafe fn dcn32_clk_mgr_destroy(_m:*mut clk_mgr_internal) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
