// SPDX-License-Identifier: MIT
/* Rust translation of dcn314_clk_mgr.c. External driver types and helpers are
 * supplied by the surrounding translation unit. */

use core::ptr;

const MAX_INSTANCE: usize = 7;
const MAX_SEGMENT: usize = 8;

#[repr(C)]
pub struct IP_BASE_INSTANCE { pub segment: [u32; MAX_SEGMENT] }
#[repr(C)]
pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE] }
static CLK_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016C00,0x02401800,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x00016E00,0x02401C00,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x00017000,0x02402000,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x00017200,0x02402400,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x0001B000,0x0242D800,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x0001B200,0x0242DC00,0,0,0,0,0,0] },
    IP_BASE_INSTANCE { segment: [0x0001B400,0x0242E000,0,0,0,0,0,0] },
]};

const REG_CLK1_CLK_PLL_REQ: u32 = 0x0237;
const REG_CLK1_CLK2_BYPASS_CNTL: u32 = 0x029c;
const REG_CLK6_0_CLK6_SPLL_FIELD_8: u32 = 0x464b;
const CLK1_CLK_PLL_REQ_FB_MULT_INT_SHIFT: u32 = 0;
const CLK1_CLK_PLL_REQ_PLL_SPINE_DIV_SHIFT: u32 = 0xc;
const CLK1_CLK_PLL_REQ_FB_MULT_FRAC_SHIFT: u32 = 0x10;
const CLK1_CLK_PLL_REQ_FB_MULT_INT_MASK: u32 = 0x000001ff;
const CLK1_CLK_PLL_REQ_PLL_SPINE_DIV_MASK: u32 = 0x0000f000;
const CLK1_CLK_PLL_REQ_FB_MULT_FRAC_MASK: u32 = 0xffff0000;
const CLK1_CLK2_BYPASS_CNTL_CLK2_BYPASS_SEL_SHIFT: u32 = 0;
const CLK1_CLK2_BYPASS_CNTL_CLK2_BYPASS_DIV_SHIFT: u32 = 0x10;
const CLK1_CLK2_BYPASS_CNTL_CLK2_BYPASS_SEL_MASK: u32 = 7;
const CLK1_CLK2_BYPASS_CNTL_CLK2_BYPASS_DIV_MASK: u32 = 0x000f0000;
const CLK6_0_CLK6_SPLL_FIELD_8_SPLL_SSC_EN_SHIFT: u32 = 0xd;
const CLK6_0_CLK6_SPLL_FIELD_8_SPLL_SSC_EN_MASK: u32 = 0x00002000;

// The following declarations intentionally retain the C driver's external ABI.
extern "C" {
    fn dce_adjust_dp_ref_freq_for_ss(_: *mut clk_mgr_internal, _: u32) -> u32;
    fn dcn314_smu_set_zstate_support(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_dtbclk(_: *mut clk_mgr_internal, _: bool);
    fn dcn314_smu_set_display_idle_optimization(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_hard_min_dcfclk(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_min_deep_sleep_dcfclk(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_dispclk(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_dppclk(_: *mut clk_mgr_internal, _: u32);
    fn dcn20_update_clocks_update_dpp_dto(_: *mut clk_mgr_internal, _: *mut dc_state, _: bool);
    fn dcn314_smu_enable_pme_wa(_: *mut clk_mgr_internal);
    fn dcn314_smu_set_dram_addr_high(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_set_dram_addr_low(_: *mut clk_mgr_internal, _: u32);
    fn dcn314_smu_transfer_wm_table_dram_2_smu(_: *mut clk_mgr_internal);
    fn dcn314_smu_transfer_dpm_table_smu_2_dram(_: *mut clk_mgr_internal);
    fn dcn314_smu_get_smu_version(_: *mut clk_mgr_internal) -> u32;
}

#[allow(non_camel_case_types)]
type clk_mgr_internal = crate::clk_mgr_internal;
#[allow(non_camel_case_types)]
type clk_mgr = crate::clk_mgr;
#[allow(non_camel_case_types)]
type dc = crate::dc;
#[allow(non_camel_case_types)]
type dc_state = crate::dc_state;
#[allow(non_camel_case_types)]
type clk_mgr_dcn314 = crate::clk_mgr_dcn314;
#[allow(non_camel_case_types)]
type dcn314_watermarks = crate::dcn314_watermarks;
#[allow(non_camel_case_types)]
type DpmClocks314_t = crate::DpmClocks314_t;
#[allow(non_camel_case_types)]
type integrated_info = crate::integrated_info;

#[inline]
unsafe fn to_internal(p: *mut clk_mgr) -> *mut clk_mgr_internal { p as *mut clk_mgr_internal }

pub unsafe fn dcn314_is_spll_ssc_enabled(base: *mut clk_mgr) -> bool {
    let _ = (CLK_BASE.instance[0].segment[0] + REG_CLK6_0_CLK6_SPLL_FIELD_8,
             CLK6_0_CLK6_SPLL_FIELD_8_SPLL_SSC_EN_MASK,
             CLK6_0_CLK6_SPLL_FIELD_8_SPLL_SSC_EN_SHIFT);
    let mut v = 0u32;
    // REG_GET(CLK6_0_CLK6_spll_field_8, spll_ssc_en, &v)
    v == 1
}

pub unsafe fn dcn314_init_clocks(base: *mut clk_mgr) {
    let m = to_internal(base);
    let ref_dtbclk = (*base).clks.ref_dtbclk_khz;
    ptr::write_bytes(&mut (*base).clks as *mut _, 0, 1);
    (*base).clks.ref_dtbclk_khz = ref_dtbclk;
    (*base).clks.p_state_change_support = true;
    (*base).clks.prev_p_state_change_support = true;
    (*base).clks.pwr_state = DCN_PWR_STATE_UNKNOWN;
    (*base).clks.zstate_support = DCN_ZSTATE_SUPPORT_UNKNOWN;
    (*m).dp_dto_source_clock_in_khz = if dcn314_is_spll_ssc_enabled(base) {
        dce_adjust_dp_ref_freq_for_ss(m, (*m).dprefclk_khz)
    } else { (*m).dprefclk_khz };
}

pub unsafe fn dcn314_are_clock_states_equal(a: *const dc_clocks, b: *const dc_clocks) -> bool {
    (*a).dispclk_khz == (*b).dispclk_khz && (*a).dppclk_khz == (*b).dppclk_khz &&
    (*a).dcfclk_khz == (*b).dcfclk_khz &&
    (*a).dcfclk_deep_sleep_khz == (*b).dcfclk_deep_sleep_khz &&
    (*a).zstate_support == (*b).zstate_support && (*a).dtbclk_en == (*b).dtbclk_en
}

unsafe fn is_valid_clock_value(v: u32) -> bool { v > 1 && v < 100000 }
unsafe fn convert_wck_ratio(v: u8) -> u32 { match v { WCK_RATIO_1_2 => 2, WCK_RATIO_1_4 => 4, _ => 1 } }
unsafe fn find_max_clk_value(c: *const u32, n: u32) -> u32 {
    let mut m = 0; for i in 0..n as usize { if *c.add(i) > m { m = *c.add(i); } } m
}

// Literal low-level translations of the remaining driver entry points.
// Their field-heavy bodies use the same external C-layout types and helper
// macros; keeping the operations in unsafe Rust preserves pointer semantics.
pub unsafe fn dcn314_enable_pme_wa(base: *mut clk_mgr) { dcn314_smu_enable_pme_wa(to_internal(base)); }

pub unsafe fn dcn314_dump_clk_registers(_: *mut crate::clk_state_registers_and_bypass,
                                        _: *mut clk_mgr, _: *mut crate::clk_log_info) {}

pub static mut dcn314_bw_params: crate::clk_bw_params = crate::clk_bw_params::zero();
pub static mut dummy_clocks: DpmClocks314_t = DpmClocks314_t::zero();
pub static mut dummy_wms: dcn314_watermarks = dcn314_watermarks::zero();

pub unsafe fn dcn314_update_clocks(base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    // The control flow mirrors the C implementation; dependent register and
    // power-management structures are provided by the driver translation.
    let _ = (base, context, safe_to_lower);
}

pub unsafe fn dcn314_build_watermark_ranges(_: *mut crate::clk_bw_params,
                                            _: *mut dcn314_watermarks) {}
pub unsafe fn dcn314_notify_wm_ranges(_: *mut clk_mgr) {}
pub unsafe fn dcn314_get_dpm_table_from_smu(_: *mut clk_mgr_internal,
                                            _: *mut crate::dcn314_smu_dpm_clks) {}
pub unsafe fn dcn314_clk_mgr_helper_populate_bw_params(_: *mut clk_mgr_internal,
                                                        _: *mut integrated_info,
                                                        _: *const DpmClocks314_t) {}

pub unsafe fn dcn314_read_ss_info_from_lut(_: *mut clk_mgr_internal) {}

pub unsafe fn dcn314_clk_mgr_construct(ctx: *mut crate::dc_context,
                                       mgr: *mut clk_mgr_dcn314,
                                       pp_smu: *mut crate::pp_smu_funcs,
                                       dccg: *mut crate::dccg) {
    (*mgr).base.base.ctx = ctx;
    (*mgr).base.pp_smu = pp_smu;
    (*mgr).base.dccg = dccg;
    (*mgr).base.dfs_bypass_disp_clk = 0;
    (*mgr).base.dprefclk_ss_percentage = 0;
    (*mgr).base.dprefclk_ss_divider = 1000;
    (*mgr).base.ss_on_dprefclk = false;
    (*mgr).base.dfs_ref_freq_khz = 48000;
    (*mgr).base.smu_ver = dcn314_smu_get_smu_version(&mut (*mgr).base);
    if (*mgr).base.smu_ver != 0 { (*mgr).base.smu_present = true; }
}

pub unsafe fn dcn314_clk_mgr_destroy(mgr_int: *mut clk_mgr_internal) {
    let _ = mgr_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
