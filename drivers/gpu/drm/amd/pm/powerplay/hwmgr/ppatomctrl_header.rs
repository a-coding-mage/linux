/* Translated from ppatomctrl.h. */

// Dependency supplied by hwmgr.h.

pub const PP_ATOM_POWER_BUDGET_DISABLE_OVERDRIVE: u32 = 0x80;
pub const PP_ATOM_POWER_BUDGET_SHOW_WARNING: u32 = 0x40;
pub const PP_ATOM_POWER_BUDGET_SHOW_WAIVER: u32 = 0x20;
pub const PP_ATOM_POWER_POWER_BUDGET_BEHAVIOUR: u32 = 0x0F;
pub const PP_ATOMCTRL_MAX_VOLTAGE_ENTRIES: usize = 32;

#[repr(C)]
pub struct pp_atomctrl_clock_dividers { pub pll_post_divider: u32, pub pll_feedback_divider: u32, pub pll_ref_divider: u32, pub enable_post_divider: bool }
pub type pp_atomctrl_clock_dividers_t = pp_atomctrl_clock_dividers;

#[repr(C)]
pub union pp_atomctrl_tcipll_fb_divider { pub ul_fb_divider: u32, pub bits: u32 }
pub type pp_atomctrl_tcipll_fb_divider_t = pp_atomctrl_tcipll_fb_divider;

#[repr(C)]
pub struct pp_atomctrl_clock_dividers_rv730 { pub pll_post_divider: u32, pub mpll_feedback_divider: pp_atomctrl_tcipll_fb_divider, pub pll_ref_divider: u32, pub enable_post_divider: bool, pub enable_dithen: bool, pub vco_mode: u32 }
pub type pp_atomctrl_clock_dividers_rv730_t = pp_atomctrl_clock_dividers_rv730;

#[repr(C)]
pub struct pp_atomctrl_clock_dividers_kong { pub pll_post_divider: u32, pub real_clock: u32 }
pub type pp_atomctrl_clock_dividers_kong_t = pp_atomctrl_clock_dividers_kong;

#[repr(C)]
pub struct pp_atomctrl_clock_dividers_ci { pub pll_post_divider: u32, pub real_clock: u32, pub ul_fb_div: pp_atomctrl_tcipll_fb_divider, pub uc_pll_ref_div: u8, pub uc_pll_post_div: u8, pub uc_pll_cntl_flag: u8 }
pub type pp_atomctrl_clock_dividers_ci_t = pp_atomctrl_clock_dividers_ci;
#[repr(C)]
pub struct pp_atomctrl_clock_dividers_vi { pub pll_post_divider: u32, pub real_clock: u32, pub ul_fb_div: pp_atomctrl_tcipll_fb_divider, pub uc_pll_ref_div: u8, pub uc_pll_post_div: u8, pub uc_pll_cntl_flag: u8 }
pub type pp_atomctrl_clock_dividers_vi_t = pp_atomctrl_clock_dividers_vi;
#[repr(C)]
pub struct pp_atomctrl_clock_dividers_ai { pub usSclk_fcw_frac: u16, pub usSclk_fcw_int: u16, pub ucSclkPostDiv: u8, pub ucSclkVcoMode: u8, pub ucSclkPllRange: u8, pub ucSscEnable: u8, pub usSsc_fcw1_frac: u16, pub usSsc_fcw1_int: u16, pub usReserved: u16, pub usPcc_fcw_int: u16, pub usSsc_fcw_slew_frac: u16, pub usPcc_fcw_slew_frac: u16 }
pub type pp_atomctrl_clock_dividers_ai_t = pp_atomctrl_clock_dividers_ai;

#[repr(C)] pub union pp_atomctrl_s_mpll_fb_divider { pub ul_fb_divider: u32, pub bits: u32 }
pub type pp_atomctrl_s_mpll_fb_divider_t = pp_atomctrl_s_mpll_fb_divider;
#[repr(i32)] pub enum pp_atomctrl_spread_spectrum_mode { pp_atomctrl_spread_spectrum_mode_down = 0, pp_atomctrl_spread_spectrum_mode_center = 1 }
pub type pp_atomctrl_spread_spectrum_mode_t = pp_atomctrl_spread_spectrum_mode;
#[repr(C)] pub struct pp_atomctrl_memory_clock_param { pub mpll_fb_divider: pp_atomctrl_s_mpll_fb_divider, pub mpll_post_divider: u32, pub bw_ctrl: u32, pub dll_speed: u32, pub vco_mode: u32, pub yclk_sel: u32, pub qdr: u32, pub half_rate: u32 }
pub type pp_atomctrl_memory_clock_param_t = pp_atomctrl_memory_clock_param;
#[repr(C)] pub struct pp_atomctrl_memory_clock_param_ai { pub ulClock: u32, pub ulPostDiv: u32, pub ulMclk_fcw_frac: u16, pub ulMclk_fcw_int: u16 }
pub type pp_atomctrl_memory_clock_param_ai_t = pp_atomctrl_memory_clock_param_ai;
#[repr(C)] pub struct pp_atomctrl_internal_ss_info { pub speed_spectrum_percentage: u32, pub speed_spectrum_rate: u32, pub speed_spectrum_mode: pp_atomctrl_spread_spectrum_mode }
pub type pp_atomctrl_internal_ss_info_t = pp_atomctrl_internal_ss_info;

pub const NUMBER_OF_M3ARB_PARAMS: usize = 3;
pub const NUMBER_OF_M3ARB_PARAM_SETS: usize = 10;
#[repr(C)] pub struct pp_atomctrl_kong_system_info { pub ul_bootup_uma_clock: u32, pub us_max_nb_voltage: u16, pub us_min_nb_voltage: u16, pub us_bootup_nb_voltage: u16, pub uc_htc_tmp_lmt: u8, pub uc_tj_offset: u8, pub ul_csr_m3_srb_cntl: [[u32; NUMBER_OF_M3ARB_PARAMS]; NUMBER_OF_M3ARB_PARAM_SETS] }
pub type pp_atomctrl_kong_system_info_t = pp_atomctrl_kong_system_info;
#[repr(C)] pub struct pp_atomctrl_memory_info { pub memory_vendor: u8, pub memory_type: u8 }
pub type pp_atomctrl_memory_info_t = pp_atomctrl_memory_info;
pub const MAX_AC_TIMING_ENTRIES: usize = 16;
#[repr(C)] pub struct pp_atomctrl_memory_clock_range_table { pub num_entries: u8, pub rsv: [u8; 3], pub mclk: [u32; MAX_AC_TIMING_ENTRIES] }
pub type pp_atomctrl_memory_clock_range_table_t = pp_atomctrl_memory_clock_range_table;
#[repr(C)] pub struct pp_atomctrl_voltage_table_entry { pub value: u16, pub smio_low: u32 }
pub type pp_atomctrl_voltage_table_entry_t = pp_atomctrl_voltage_table_entry;
#[repr(C)] pub struct pp_atomctrl_voltage_table { pub count: u32, pub mask_low: u32, pub phase_delay: u32, pub entries: [pp_atomctrl_voltage_table_entry; PP_ATOMCTRL_MAX_VOLTAGE_ENTRIES] }
pub type pp_atomctrl_voltage_table_t = pp_atomctrl_voltage_table;
pub const VBIOS_MC_REGISTER_ARRAY_SIZE: usize = 32;
pub const VBIOS_MAX_AC_TIMING_ENTRIES: usize = 20;
#[repr(C)] pub struct pp_atomctrl_mc_reg_entry { pub mclk_max: u32, pub mc_data: [u32; VBIOS_MC_REGISTER_ARRAY_SIZE] }
pub type pp_atomctrl_mc_reg_entry_t = pp_atomctrl_mc_reg_entry;
#[repr(C)] pub struct pp_atomctrl_mc_register_address { pub s1: u16, pub uc_pre_reg_data: u8 }
pub type pp_atomctrl_mc_register_address_t = pp_atomctrl_mc_register_address;
pub const MAX_SCLK_RANGE: usize = 8;
#[repr(C)] pub struct pp_atom_ctrl_sclk_range_table_entry { pub ucVco_setting: u8, pub ucPostdiv: u8, pub usFcw_pcc: u16, pub usFcw_trans_upper: u16, pub usRcw_trans_lower: u16 }
#[repr(C)] pub struct pp_atom_ctrl_sclk_range_table { pub entry: [pp_atom_ctrl_sclk_range_table_entry; MAX_SCLK_RANGE] }
#[repr(C)] pub struct pp_atomctrl_mc_reg_table { pub last: u8, pub num_entries: u8, pub mc_reg_table_entry: [pp_atomctrl_mc_reg_entry; VBIOS_MAX_AC_TIMING_ENTRIES], pub mc_reg_address: [pp_atomctrl_mc_register_address; VBIOS_MC_REGISTER_ARRAY_SIZE] }
pub type pp_atomctrl_mc_reg_table_t = pp_atomctrl_mc_reg_table;
#[repr(C)] pub struct pp_atomctrl_gpio_pin_assignment { pub us_gpio_pin_aindex: u16, pub uc_gpio_pin_bit_shift: u8 }
pub type pp_atomctrl_gpio_pin_assignment_t = pp_atomctrl_gpio_pin_assignment;

#[repr(C)] pub struct pp_atom_ctrl__avfs_parameters {
    pub ulAVFS_meanNsigma_Acontant0: u32, pub ulAVFS_meanNsigma_Acontant1: u32, pub ulAVFS_meanNsigma_Acontant2: u32,
    pub usAVFS_meanNsigma_DC_tol_sigma: u16, pub usAVFS_meanNsigma_Platform_mean: u16, pub usAVFS_meanNsigma_Platform_sigma: u16,
    pub ulGB_VDROOP_TABLE_CKSOFF_a0: u32, pub ulGB_VDROOP_TABLE_CKSOFF_a1: u32, pub ulGB_VDROOP_TABLE_CKSOFF_a2: u32,
    pub ulGB_VDROOP_TABLE_CKSON_a0: u32, pub ulGB_VDROOP_TABLE_CKSON_a1: u32, pub ulGB_VDROOP_TABLE_CKSON_a2: u32,
    pub ulAVFSGB_FUSE_TABLE_CKSOFF_m1: u32, pub usAVFSGB_FUSE_TABLE_CKSOFF_m2: u16, pub ulAVFSGB_FUSE_TABLE_CKSOFF_b: u32,
    pub ulAVFSGB_FUSE_TABLE_CKSON_m1: u32, pub usAVFSGB_FUSE_TABLE_CKSON_m2: u16, pub ulAVFSGB_FUSE_TABLE_CKSON_b: u32,
    pub usMaxVoltage_0_25mv: u16, pub ucEnableGB_VDROOP_TABLE_CKSOFF: u8, pub ucEnableGB_VDROOP_TABLE_CKSON: u8,
    pub ucEnableGB_FUSE_TABLE_CKSOFF: u8, pub ucEnableGB_FUSE_TABLE_CKSON: u8, pub usPSM_Age_ComFactor: u16,
    pub ucEnableApplyAVFS_CKS_OFF_Voltage: u8, pub ucReserved: u8,
}
#[repr(C)] pub struct AtomCtrl_HiLoLeakageOffsetTable { pub usHiLoLeakageThreshold: USHORT, pub usEdcDidtLoDpm7TableOffset: USHORT, pub usEdcDidtHiDpm7TableOffset: USHORT }
#[repr(C)] pub struct AtomCtrl_EDCLeakgeTable { pub DIDT_REG: [ULONG; 24] }

extern "C" {
    pub fn atomctrl_get_pp_assign_pin(hwmgr: *mut pp_hwmgr, pinId: u32, gpio_pin_assignment: *mut pp_atomctrl_gpio_pin_assignment) -> bool;
    pub fn atomctrl_get_voltage_evv_on_sclk(hwmgr: *mut pp_hwmgr, voltage_type: u8, sclk: u32, virtual_voltage_Id: u16, voltage: *mut u16) -> i32;
    pub fn atomctrl_get_voltage_evv(hwmgr: *mut pp_hwmgr, virtual_voltage_id: u16, voltage: *mut u16) -> i32;
    pub fn atomctrl_get_mpll_reference_clock(hwmgr: *mut pp_hwmgr) -> u32;
    pub fn atomctrl_is_asic_internal_ss_supported(hwmgr: *mut pp_hwmgr) -> bool;
    pub fn atomctrl_get_memory_clock_spread_spectrum(hwmgr: *mut pp_hwmgr, memory_clock: u32, ssInfo: *mut pp_atomctrl_internal_ss_info) -> i32;
    pub fn atomctrl_get_engine_clock_spread_spectrum(hwmgr: *mut pp_hwmgr, engine_clock: u32, ssInfo: *mut pp_atomctrl_internal_ss_info) -> i32;
    pub fn atomctrl_initialize_mc_reg_table(hwmgr: *mut pp_hwmgr, module_index: u8, table: *mut pp_atomctrl_mc_reg_table) -> i32;
    pub fn atomctrl_initialize_mc_reg_table_v2_2(hwmgr: *mut pp_hwmgr, module_index: u8, table: *mut pp_atomctrl_mc_reg_table) -> i32;
    pub fn atomctrl_set_engine_dram_timings_rv770(hwmgr: *mut pp_hwmgr, engine_clock: u32, memory_clock: u32) -> i32;
    pub fn atomctrl_get_reference_clock(hwmgr: *mut pp_hwmgr) -> u32;
    pub fn atomctrl_get_memory_pll_dividers_si(hwmgr: *mut pp_hwmgr, clock_value: u32, mpll_param: *mut pp_atomctrl_memory_clock_param, strobe_mode: bool) -> i32;
    pub fn atomctrl_get_engine_pll_dividers_vi(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_vi) -> i32;
    pub fn atomctrl_get_dfs_pll_dividers_vi(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_vi) -> i32;
    pub fn atomctrl_is_voltage_controlled_by_gpio_v3(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8) -> bool;
    pub fn atomctrl_get_voltage_table_v3(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8, voltage_table: *mut pp_atomctrl_voltage_table) -> i32;
    pub fn atomctrl_get_memory_pll_dividers_vi(hwmgr: *mut pp_hwmgr, clock_value: u32, mpll_param: *mut pp_atomctrl_memory_clock_param) -> i32;
    pub fn atomctrl_get_memory_pll_dividers_ai(hwmgr: *mut pp_hwmgr, clock_value: u32, mpll_param: *mut pp_atomctrl_memory_clock_param_ai) -> i32;
    pub fn atomctrl_get_engine_pll_dividers_kong(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_kong) -> i32;
    pub fn atomctrl_read_efuse(hwmgr: *mut pp_hwmgr, start_index: u16, end_index: u16, efuse: *mut u32) -> i32;
    pub fn atomctrl_get_engine_pll_dividers_ai(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_ai) -> i32;
    pub fn atomctrl_set_ac_timing_ai(hwmgr: *mut pp_hwmgr, memory_clock: u32, level: u8) -> i32;
    pub fn atomctrl_get_voltage_evv_on_sclk_ai(hwmgr: *mut pp_hwmgr, voltage_type: u8, sclk: u32, virtual_voltage_Id: u16, voltage: *mut u32) -> i32;
    pub fn atomctrl_get_smc_sclk_range_table(hwmgr: *mut pp_hwmgr, table: *mut pp_atom_ctrl_sclk_range_table) -> i32;
    pub fn atomctrl_get_avfs_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atom_ctrl__avfs_parameters) -> i32;
    pub fn atomctrl_get_svi2_info(hwmgr: *mut pp_hwmgr, voltage_type: u8, svd_gpio_id: *mut u8, svc_gpio_id: *mut u8, load_line: *mut u16) -> i32;
    pub fn atomctrl_get_leakage_vddc_base_on_leakage(hwmgr: *mut pp_hwmgr, vddc: *mut u16, vddci: *mut u16, virtual_voltage_id: u16, efuse_voltage_id: u16) -> i32;
    pub fn atomctrl_get_leakage_id_from_efuse(hwmgr: *mut pp_hwmgr, virtual_voltage_id: *mut u16) -> i32;
    pub fn atomctrl_get_voltage_range(hwmgr: *mut pp_hwmgr, max_vddc: *mut u32, min_vddc: *mut u32);
    pub fn atomctrl_get_edc_hilo_leakage_offset_table(hwmgr: *mut pp_hwmgr, table: *mut AtomCtrl_HiLoLeakageOffsetTable) -> i32;
    pub fn atomctrl_get_edc_leakage_table(hwmgr: *mut pp_hwmgr, table: *mut AtomCtrl_EDCLeakgeTable, offset: u16) -> i32;
    pub fn atomctrl_get_vddc_shared_railinfo(hwmgr: *mut pp_hwmgr, shared_rail: *mut u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
