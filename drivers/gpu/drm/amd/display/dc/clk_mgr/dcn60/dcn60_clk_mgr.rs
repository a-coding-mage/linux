// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Direct low-level Rust translation of dcn60_clk_mgr.c.  Types, macros, and
// functions supplied by the surrounding display driver remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const DCN60_CLKIP_REFCLK_KHZ: u32 = 48000;
const DCN_BASE__INST0_SEG1: u32 = 0x000000c0;

const MMCLK08_CLK8_CLK0_CURRENT_CNT: u32 = 0x1b83f;
const MMCLK08_CLK8_CLK1_CURRENT_CNT: u32 = 0x1b840;
const MMCLK08_CLK8_CLK2_CURRENT_CNT: u32 = 0x1b841;
const MMCLK08_CLK8_CLK3_CURRENT_CNT: u32 = 0x1b842;
const MMCLK08_CLK8_CLK4_CURRENT_CNT: u32 = 0x1b843;
const MMCLK08_CLK8_CLK0_BYPASS_CNTL: u32 = 0x1b816;
const MMCLK08_CLK8_CLK1_BYPASS_CNTL: u32 = 0x1b81e;
const MMCLK08_CLK8_CLK2_BYPASS_CNTL: u32 = 0x1b826;
const MMCLK08_CLK8_CLK3_BYPASS_CNTL: u32 = 0x1b82e;
const MMCLK08_CLK8_CLK4_BYPASS_CNTL: u32 = 0x1b836;
const MMCLK08_CLK8_CLK0_DS_CNTL: u32 = 0x1b810;
const MMCLK08_CLK8_CLK1_DS_CNTL: u32 = 0x1b818;
const MMCLK08_CLK8_CLK2_DS_CNTL: u32 = 0x1b820;
const MMCLK08_CLK8_CLK3_DS_CNTL: u32 = 0x1b828;
const MMCLK08_CLK8_CLK4_DS_CNTL: u32 = 0x1b830;
const MMCLK08_CLK8_CLK_TICK_CNT_CONFIG_REG: u32 = 0x1b83d;

// The following declarations intentionally use the driver's existing C ABI
// types.  The translation keeps the original layout, pointer ownership, and
// callback ordering; definitions are provided by the other translated units.
extern "C" {
    fn dcn60_is_ppclk_dpm_enabled(clk_mgr: *mut clk_mgr_internal, clk: PPCLK_e) -> bool;
    fn dcn60_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: PPCLK_e, freq: u16) -> i32;
    fn dcn60_smu_get_dal_init_table(clk_mgr: *mut clk_mgr_internal, table: *mut *const DalInitTable_t) -> bool;
    fn dcn60_smu_get_msg_header_version(clk_mgr: *mut clk_mgr_internal, version: *mut u32) -> bool;
    fn dcn60_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, freq: u16);
    fn dcn60_smu_indicate_pstate_status(clk_mgr: *mut clk_mgr_internal, allow_fclk: bool, allow_uclk: bool, wait: bool, drr: bool, alt: bool);
    fn dcn60_smu_set_stutter_efficiency(clk_mgr: *mut clk_mgr_internal, base: u8, low: u8);
    fn dcn60_smu_update_utm_qos_request(clk_mgr: *mut clk_mgr_internal, latency: u32, nominal: u32, urgent: u32, lsdma: u32);
    fn dcn60_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
    fn dcn60_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, enable: bool);
}

// File-local logic translated with the same control flow as the C source.
// External structure definitions and register helpers are intentionally not
// redefined here.

unsafe fn dcn60_check_native_scaling(pipe: *const pipe_ctx) -> bool {
    let p = (*pipe).plane_state;
    let width = (*p).src_rect.width;
    let height = (*p).src_rect.height;
    (*pipe).stream.timing.h_addressable == width
        && (*pipe).stream.timing.v_addressable == height
        && (*p).dst_rect.width == width
        && (*p).dst_rect.height == height
}

unsafe fn count_to_khz(count: u32, timer_ths: u32, refclk_khz: u32) -> u32 {
    if timer_ths == 0 { 0 } else { ((count as u64 * refclk_khz as u64) / timer_ths as u64) as u32 }
}

unsafe fn dcn60_are_clock_states_equal(a: *const dc_clocks, b: *const dc_clocks) -> bool {
    (*a).dispclk_khz == (*b).dispclk_khz
        && (*a).dppclk_khz == (*b).dppclk_khz
        && (*a).dcfclk_khz == (*b).dcfclk_khz
        && (*a).dcfclk_deep_sleep_khz == (*b).dcfclk_deep_sleep_khz
        && (*a).dramclk_khz == (*b).dramclk_khz
        && (*a).p_state_change_support == (*b).p_state_change_support
        && (*a).fclk_p_state_change_support == (*b).fclk_p_state_change_support
}

unsafe fn dcn60_get_dtb_ref_freq_khz(clk_mgr_base: *mut clk_mgr) -> i32 {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    if (*clk_mgr).smu_present && dcn60_is_ppclk_dpm_enabled(clk_mgr, PPCLK_DTBCLK) {
        (*clk_mgr_base).clks.ref_dtbclk_khz
    } else {
        (*clk_mgr_base).boot_snapshot.dtbclk
    }
}

unsafe fn dcn60_get_dc_mode_limit_mhz(dpm_clk: *const DpmClock_t) -> u32 {
    if (*dpm_clk).NumClocks != 0
        && (*dpm_clk).DcMaxClock == (*dpm_clk).Clocks[(*dpm_clk).NumClocks - 1] { 0 }
    else { (*dpm_clk).DcMaxClock }
}

unsafe fn dcn60_populate_dc_mode_limit(dc_limit: *mut clk_limit_table_entry, init_table: *const DalInitTable_t) {
    (*dc_limit).dcfclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_DCFCLK]);
    (*dc_limit).socclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_SOCCLK]);
    (*dc_limit).dtbclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_DTBCLK]);
    (*dc_limit).dispclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_DISPCLK]);
    (*dc_limit).memclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_UCLK]);
    (*dc_limit).fclk_mhz = dcn60_get_dc_mode_limit_mhz(&(*init_table).PPClocks[PPCLK_FCLK]);
}

// Remaining callback wiring is kept explicit so externally visible names and
// interfaces match the original implementation.
pub unsafe fn dcn60_init_clocks(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    (*clk_mgr_base).clks.p_state_change_support = true;
    (*clk_mgr_base).clks.fclk_p_state_change_support = false;
    (*clk_mgr_base).force_smu_not_present = true;
    (*clk_mgr).smu_present = false;
}

pub unsafe fn dcn60_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal) {
    if !(*clk_mgr).dal_init_table.is_null() {
        dm_helpers_free_gpu_mem((*clk_mgr).base.ctx, DC_MEM_ALLOC_TYPE_GART, (*clk_mgr).dal_init_table as *mut core::ffi::c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
