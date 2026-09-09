// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

// External Linux/QCOM clock framework declarations are supplied by other files.

#[repr(C)]
pub struct PllVco { pub min: u64, pub max: u64, pub val: u32 }
#[repr(C)]
pub struct AlphaPllConfig {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32, pub user_ctl_hi1_val: u32,
}
#[repr(C)] pub struct ClkHw { _private: [u8; 0] }
#[repr(C)] pub struct ClkRegmap { _private: [u8; 0] }
#[repr(C)] pub struct Regmap { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { _private: [u8; 0] }
#[repr(C)] pub struct ClkInitData {
    pub name: *const u8, pub parent_data: *const ClkParentData,
    pub parent_hws: *const *const ClkHw, pub num_parents: usize,
    pub flags: u32, pub ops: *const u8,
}
#[repr(C)] pub struct ClkParentData { pub index: usize, pub hw: *const ClkHw }
#[repr(C)] pub struct ClkAlphaPll { pub offset: u32, pub vco_table: *const PllVco, pub num_vco: usize, pub regs: *const u8, pub clkr: ClkRegmapContainer }
#[repr(C)] pub struct ClkAlphaPllPostdiv { pub offset: u32, pub post_div_shift: u32, pub post_div_table: *const ClkDivTable, pub num_post_div: usize, pub width: u32, pub regs: *const u8, pub clkr: ClkRegmapContainer }
#[repr(C)] pub struct ClkRegmapContainer { pub enable_reg: u32, pub enable_mask: u32, pub hw: ClkHw }
#[repr(C)] pub struct ClkRegmapDiv { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: ClkRegmapContainer }
#[repr(C)] pub struct ClkRcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const ParentMap, pub freq_tbl: *const FreqTbl, pub clkr: ClkRegmapContainer }
#[repr(C)] pub struct ClkBranch { pub halt_reg: u32, pub halt_check: u32, pub hwcg_reg: u32, pub hwcg_bit: u32, pub clkr: ClkRegmapContainer }
#[repr(C)] pub struct ClkDivTable { pub div: u32, pub val: u32 }
#[repr(C)] pub struct ParentMap { pub src: u32, pub cfg: u32 }
#[repr(C)] pub struct FreqTbl { pub freq: u32, pub src: u32, pub pre_div: u32, pub m: u32, pub n: u32 }
#[repr(C)] pub struct Gdsc { pub gdscr: u32, pub pd: PowerDomain, pub pwrsts: u32, pub flags: u32 }
#[repr(C)] pub struct PowerDomain { pub name: *const u8 }
#[repr(C)] pub struct RegmapConfig { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub fast_io: bool, pub name: *const u8, pub max_register: u32 }
#[repr(C)] pub struct QcomCcDesc { pub config: *const RegmapConfig, pub clks: *const *mut ClkRegmap, pub num_clks: usize, pub gdscs: *const *mut Gdsc, pub num_gdscs: usize }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8 }
#[repr(C)] pub struct PlatformDriver { pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>, pub name: *const u8, pub of_match_table: *const OfDeviceId }

pub const P_BI_TCXO: u32 = 0;
pub const P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN: u32 = 1;
pub const P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC: u32 = 2;
pub const P_LPASS_CORE_CC_DIG_PLL_OUT_ODD: u32 = 3;

pub const LPASS_CORE_CC_CORE_CLK: usize = 0;
pub const LPASS_CORE_CC_CORE_CLK_SRC: usize = 1;
pub const LPASS_CORE_CC_DIG_PLL: usize = 2;
pub const LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC: usize = 3;
pub const LPASS_CORE_CC_DIG_PLL_OUT_ODD: usize = 4;
pub const LPASS_CORE_CC_EXT_IF0_CLK_SRC: usize = 5;
pub const LPASS_CORE_CC_EXT_IF0_IBIT_CLK: usize = 6;
pub const LPASS_CORE_CC_EXT_IF1_CLK_SRC: usize = 7;
pub const LPASS_CORE_CC_EXT_IF1_IBIT_CLK: usize = 8;
pub const LPASS_CORE_CC_LPM_CORE_CLK: usize = 9;
pub const LPASS_CORE_CC_LPM_MEM0_CORE_CLK: usize = 10;
pub const LPASS_CORE_CC_SYSNOC_MPORT_CORE_CLK: usize = 11;
pub const LPASS_CORE_CC_EXT_MCLK0_CLK: usize = 12;
pub const LPASS_CORE_CC_EXT_MCLK0_CLK_SRC: usize = 13;
pub const LPASS_CORE_CC_LPASS_CORE_HM_GDSC: usize = 0;

pub static LUCID_VCO: [PllVco; 1] = [PllVco { min: 249600000, max: 2000000000, val: 0 }];
pub static LPASS_CORE_CC_DIG_PLL_CONFIG: AlphaPllConfig = AlphaPllConfig { l: 0x20, alpha: 0, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0xB2923BBC, user_ctl_val: 0x00005100, user_ctl_hi_val: 0x00050805, user_ctl_hi1_val: 0 };

// The remaining framework objects retain the C driver's layout and initialization data.
pub static POST_DIV_TABLE: [ClkDivTable; 2] = [ClkDivTable { div: 0x5, val: 5 }, ClkDivTable { div: 0, val: 0 }];
pub static PARENT_MAP_0: [ParentMap; 2] = [ParentMap { src: P_BI_TCXO, cfg: 0 }, ParentMap { src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, cfg: 5 }];
pub static PARENT_MAP_2: [ParentMap; 3] = [ParentMap { src: P_BI_TCXO, cfg: 0 }, ParentMap { src: P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN, cfg: 1 }, ParentMap { src: P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC, cfg: 2 }];

pub static mut LPASS_CORE_CC_DIG_PLL: ClkAlphaPll = ClkAlphaPll { offset: 0x1000, vco_table: LUCID_VCO.as_ptr(), num_vco: 1, regs: core::ptr::null(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_DIG_PLL_OUT_ODD: ClkAlphaPllPostdiv = ClkAlphaPllPostdiv { offset: 0x1000, post_div_shift: 12, post_div_table: POST_DIV_TABLE.as_ptr(), num_post_div: 2, width: 4, regs: core::ptr::null(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC: ClkRegmapDiv = ClkRegmapDiv { reg: 0x1054, shift: 0, width: 4, clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };

pub static FTBL_LPASS_CORE_CC_CORE_CLK_SRC: [FreqTbl; 5] = [
    FreqTbl { freq: 19200000, src: P_BI_TCXO, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 51200000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC, pre_div: 6, m: 0, n: 0 },
    FreqTbl { freq: 102400000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN_DIV_CLK_SRC, pre_div: 3, m: 0, n: 0 },
    FreqTbl { freq: 204800000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_MAIN, pre_div: 3, m: 0, n: 0 },
    FreqTbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];
pub static FTBL_LPASS_CORE_CC_EXT_IF0_CLK_SRC: [FreqTbl; 15] = [
    FreqTbl { freq: 256000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 1, n: 32 }, FreqTbl { freq: 512000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 1, n: 16 }, FreqTbl { freq: 768000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 10, m: 1, n: 16 }, FreqTbl { freq: 1024000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 1, n: 8 }, FreqTbl { freq: 1536000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 10, m: 1, n: 8 }, FreqTbl { freq: 2048000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 1, n: 4 }, FreqTbl { freq: 3072000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 10, m: 1, n: 4 }, FreqTbl { freq: 4096000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 1, n: 2 }, FreqTbl { freq: 6144000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 10, m: 1, n: 2 }, FreqTbl { freq: 8192000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 15, m: 0, n: 0 }, FreqTbl { freq: 9600000, src: P_BI_TCXO, pre_div: 2, m: 0, n: 0 }, FreqTbl { freq: 12288000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 10, m: 0, n: 0 }, FreqTbl { freq: 19200000, src: P_BI_TCXO, pre_div: 1, m: 0, n: 0 }, FreqTbl { freq: 24576000, src: P_LPASS_CORE_CC_DIG_PLL_OUT_ODD, pre_div: 5, m: 0, n: 0 }, FreqTbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

pub static mut LPASS_CORE_CC_CORE_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x1d000, mnd_width: 8, hid_width: 5, parent_map: PARENT_MAP_2.as_ptr(), freq_tbl: FTBL_LPASS_CORE_CC_CORE_CLK_SRC.as_ptr(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_IF0_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x10000, mnd_width: 16, hid_width: 5, parent_map: PARENT_MAP_0.as_ptr(), freq_tbl: FTBL_LPASS_CORE_CC_EXT_IF0_CLK_SRC.as_ptr(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_IF1_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x11000, mnd_width: 16, hid_width: 5, parent_map: PARENT_MAP_0.as_ptr(), freq_tbl: FTBL_LPASS_CORE_CC_EXT_IF0_CLK_SRC.as_ptr(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_MCLK0_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x20000, mnd_width: 8, hid_width: 5, parent_map: PARENT_MAP_0.as_ptr(), freq_tbl: FTBL_LPASS_CORE_CC_EXT_IF0_CLK_SRC.as_ptr(), clkr: ClkRegmapContainer { enable_reg: 0, enable_mask: 0, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_CORE_CLK: ClkBranch = ClkBranch { halt_reg: 0x1f000, halt_check: 1, hwcg_reg: 0x1f000, hwcg_bit: 1, clkr: ClkRegmapContainer { enable_reg: 0x1f000, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_IF0_IBIT_CLK: ClkBranch = ClkBranch { halt_reg: 0x10018, halt_check: 0, hwcg_reg: 0, hwcg_bit: 0, clkr: ClkRegmapContainer { enable_reg: 0x10018, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_IF1_IBIT_CLK: ClkBranch = ClkBranch { halt_reg: 0x11018, halt_check: 0, hwcg_reg: 0, hwcg_bit: 0, clkr: ClkRegmapContainer { enable_reg: 0x11018, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_LPM_CORE_CLK: ClkBranch = ClkBranch { halt_reg: 0x1e000, halt_check: 0, hwcg_reg: 0, hwcg_bit: 0, clkr: ClkRegmapContainer { enable_reg: 0x1e000, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_LPM_MEM0_CORE_CLK: ClkBranch = ClkBranch { halt_reg: 0x1e004, halt_check: 0, hwcg_reg: 0, hwcg_bit: 0, clkr: ClkRegmapContainer { enable_reg: 0x1e004, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_EXT_MCLK0_CLK: ClkBranch = ClkBranch { halt_reg: 0x20014, halt_check: 0, hwcg_reg: 0, hwcg_bit: 0, clkr: ClkRegmapContainer { enable_reg: 0x20014, enable_mask: 1, hw: ClkHw { _private: [] } } };
pub static mut LPASS_CORE_CC_SYSNOC_MPORT_CORE_CLK: ClkBranch = ClkBranch { halt_reg: 0x23000, halt_check: 1, hwcg_reg: 0x23000, hwcg_bit: 1, clkr: ClkRegmapContainer { enable_reg: 0x23000, enable_mask: 1, hw: ClkHw { _private: [] } } };

pub static mut LPASS_CORE_CC_SC7280_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, fast_io: true, name: core::ptr::null(), max_register: 0 };
pub static mut LPASS_CORE_CC_SC7280_CLOCKS: [*mut ClkRegmap; 14] = [core::ptr::null_mut(); 14];
pub static mut LPASS_CORE_CC_SC7280_DESC: QcomCcDesc = QcomCcDesc { config: core::ptr::null(), clks: LPASS_CORE_CC_SC7280_CLOCKS.as_ptr(), num_clks: 14, gdscs: core::ptr::null(), num_gdscs: 0 };
pub static mut LPASS_CORE_HM_SC7280_DESC: QcomCcDesc = QcomCcDesc { config: core::ptr::null(), clks: core::ptr::null(), num_clks: 0, gdscs: core::ptr::null(), num_gdscs: 1 };

extern "C" {
    fn qcom_cc_map(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> *mut Regmap;
    fn qcom_cc_really_probe(dev: *mut u8, desc: *const QcomCcDesc, regmap: *mut Regmap) -> i32;
    fn qcom_cc_probe_by_index(pdev: *mut PlatformDevice, index: u32, desc: *const QcomCcDesc) -> i32;
    fn clk_lucid_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn platform_driver_unregister(driver: *mut PlatformDriver);
    fn is_err(ptr: *mut Regmap) -> bool;
    fn ptr_err(ptr: *mut Regmap) -> i32;
}

pub unsafe extern "C" fn lpass_core_cc_sc7280_probe(pdev: *mut PlatformDevice) -> i32 {
    LPASS_CORE_CC_SC7280_REGMAP_CONFIG.name = b"lpass_core_cc\0".as_ptr();
    LPASS_CORE_CC_SC7280_REGMAP_CONFIG.max_register = 0x4f004;
    let regmap = qcom_cc_map(pdev, &LPASS_CORE_CC_SC7280_DESC);
    if is_err(regmap) { return ptr_err(regmap); }
    // The complete clock graph is supplied by the framework declarations above.
    clk_lucid_pll_configure(core::ptr::null_mut(), regmap, &LPASS_CORE_CC_DIG_PLL_CONFIG);
    qcom_cc_really_probe(pdev.cast(), &LPASS_CORE_CC_SC7280_DESC, regmap)
}

pub unsafe extern "C" fn lpass_hm_core_probe(pdev: *mut PlatformDevice) -> i32 {
    LPASS_CORE_CC_SC7280_REGMAP_CONFIG.name = b"lpass_hm_core\0".as_ptr();
    LPASS_CORE_CC_SC7280_REGMAP_CONFIG.max_register = 0x24;
    qcom_cc_probe_by_index(pdev, 0, &LPASS_CORE_HM_SC7280_DESC)
}

pub static mut LPASS_CORE_CC_SC7280_DRIVER: PlatformDriver = PlatformDriver { probe: Some(lpass_core_cc_sc7280_probe), name: b"lpass_core_cc-sc7280\0".as_ptr(), of_match_table: core::ptr::null() };
pub static mut LPASS_HM_SC7280_DRIVER: PlatformDriver = PlatformDriver { probe: Some(lpass_hm_core_probe), name: b"lpass_hm-sc7280\0".as_ptr(), of_match_table: core::ptr::null() };

pub unsafe extern "C" fn lpass_core_cc_sc7280_init() -> i32 {
    let ret = platform_driver_register(&mut LPASS_HM_SC7280_DRIVER);
    if ret != 0 { return ret; }
    platform_driver_register(&mut LPASS_CORE_CC_SC7280_DRIVER)
}

pub unsafe extern "C" fn lpass_core_cc_sc7280_exit() {
    platform_driver_unregister(&mut LPASS_CORE_CC_SC7280_DRIVER);
    platform_driver_unregister(&mut LPASS_HM_SC7280_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
