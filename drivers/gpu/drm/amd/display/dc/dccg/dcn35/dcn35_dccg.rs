/* SPDX-License-Identifier: MIT */
/* Direct low-level translation of dcn35_dccg.c.  Register helpers, types,
 * and externally supplied symbols are intentionally left to dependencies. */

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symclk_fe_source { SYMCLK_FE_SYMCLK_A = 0, SYMCLK_FE_SYMCLK_B, SYMCLK_FE_SYMCLK_C, SYMCLK_FE_SYMCLK_D, SYMCLK_FE_SYMCLK_E, SYMCLK_FE_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symclk_be_source { SYMCLK_BE_PHYCLK = 0, SYMCLK_BE_DPIACLK_810 = 4, SYMCLK_BE_DPIACLK_162, SYMCLK_BE_DPIACLK_540, SYMCLK_BE_DPIACLK_270, SYMCLK_BE_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum physymclk_source { PHYSYMCLK_PHYCLK = 0, PHYSYMCLK_PHYD18CLK, PHYSYMCLK_PHYD32CLK, PHYSYMCLK_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dtbclk_source { DTBCLK_DPREFCLK = 0, DTBCLK_DPREFCLK_0, DTBCLK_DTBCLK0, DTBCLK_DTBCLK1, DTBCLK_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dppclk_clock_source { DPP_REFCLK = 0, DPP_DCCG_DTO }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_stream_clk_source { DP_STREAM_DTBCLK_P0 = 0, DP_STREAM_DTBCLK_P1, DP_STREAM_DTBCLK_P2, DP_STREAM_DTBCLK_P3, DP_STREAM_DTBCLK_P4, DP_STREAM_DTBCLK_P5, DP_STREAM_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_char_clk { HDMI_CHAR_PHYAD18CLK = 0, HDMI_CHAR_PHYBD18CLK, HDMI_CHAR_PHYCD18CLK, HDMI_CHAR_PHYDD18CLK, HDMI_CHAR_PHYED18CLK, HDMI_CHAR_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_stream_clk_source { HDMI_STREAM_DTBCLK_P0 = 0, HDMI_STREAM_DTBCLK_P1, HDMI_STREAM_DTBCLK_P2, HDMI_STREAM_DTBCLK_P3, HDMI_STREAM_DTBCLK_P4, HDMI_STREAM_DTBCLK_P5, HDMI_STREAM_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symclk32_se_clk_source { SYMCLK32_SE_PHYAD32CLK = 0, SYMCLK32_SE_PHYBD32CLK, SYMCLK32_SE_PHYCD32CLK, SYMCLK32_SE_PHYDD32CLK, SYMCLK32_SE_PHYED32CLK, SYMCLK32_SE_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symclk32_le_clk_source { SYMCLK32_LE_PHYAD32CLK = 0, SYMCLK32_LE_PHYBD32CLK, SYMCLK32_LE_PHYCD32CLK, SYMCLK32_LE_PHYDD32CLK, SYMCLK32_LE_PHYED32CLK, SYMCLK32_LE_REFCLK = 0xff }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsc_clk_source { DSC_CLK_REF_CLK = 0, DSC_DTO_TUNED_CK_GPU_DISCLK_3 }

/* The implementation uses the kernel's register-access macros.  They are
 * preserved as Rust macro calls so the surrounding translated driver can
 * provide their target-specific definitions. */
extern "C" {
    pub fn dccg35_trigger_dio_fifo_resync(dccg: *mut dccg);
    pub fn dccg35_init(dccg: *mut dccg);
    pub fn dccg35_enable_global_fgcg_rep(dccg: *mut dccg, value: bool);
    pub fn dccg35_enable_dscclk(dccg: *mut dccg, inst: i32);
    pub fn dccg35_disable_dscclk(dccg: *mut dccg, inst: i32);
    pub fn dccg35_enable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg35_disable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg35_disable_symclk32_se(dccg: *mut dccg, hpo_se_inst: i32);
}

/* External C driver types supplied by the translated dependency set. */
#[repr(C)] pub struct dccg { _private: [u8; 0] }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
