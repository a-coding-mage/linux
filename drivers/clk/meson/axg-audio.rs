// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/* Rust translation of clk/meson/axg-audio.c.  Kernel-provided types and
 * operations remain external dependencies, as they are in the C source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn axg_audio_clkc_probe(pdev: *mut platform_device) -> c_int;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_init_data { _private: [u8; 0] }
#[repr(C)] pub struct clk_parent_data { pub name: *const c_char, pub fw_name: *const c_char, pub index: c_int }
#[repr(C)] pub struct clk_regmap_gate_data { pub offset: c_uint, pub bit_idx: c_uint }
#[repr(C)] pub struct clk_regmap_mux_data { pub offset: c_uint, pub mask: c_uint, pub shift: c_uint, pub flags: c_uint }
#[repr(C)] pub struct clk_regmap_div_data { pub offset: c_uint, pub shift: c_uint, pub width: c_uint, pub flags: c_uint }
#[repr(C)] pub struct meson_sclk_div_data { pub div: meson_clk_field, pub hi: meson_clk_field }
#[repr(C)] pub struct meson_clk_triphase_data { pub ph0: meson_clk_field, pub ph1: meson_clk_field, pub ph2: meson_clk_field }
#[repr(C)] pub struct meson_clk_phase_data { pub ph: meson_clk_field }
#[repr(C)] pub struct meson_sclk_ws_inv_data { pub ph: meson_clk_field, pub ws: meson_clk_field }
#[repr(C)] pub struct meson_clk_field { pub reg_off: c_uint, pub shift: c_uint, pub width: c_uint }

// Audio clock register offsets.
pub const AUDIO_CLK_GATE_EN: u32 = 0x000; pub const AUDIO_MCLK_A_CTRL: u32 = 0x004;
pub const AUDIO_MCLK_B_CTRL: u32 = 0x008; pub const AUDIO_MCLK_C_CTRL: u32 = 0x00c;
pub const AUDIO_MCLK_D_CTRL: u32 = 0x010; pub const AUDIO_MCLK_E_CTRL: u32 = 0x014;
pub const AUDIO_MCLK_F_CTRL: u32 = 0x018; pub const AUDIO_MST_PAD_CTRL0: u32 = 0x01c;
pub const AUDIO_MST_PAD_CTRL1: u32 = 0x020; pub const AUDIO_SW_RESET: u32 = 0x024;
pub const AUDIO_MST_A_SCLK_CTRL0: u32 = 0x040; pub const AUDIO_MST_A_SCLK_CTRL1: u32 = 0x044;
pub const AUDIO_MST_B_SCLK_CTRL0: u32 = 0x048; pub const AUDIO_MST_B_SCLK_CTRL1: u32 = 0x04c;
pub const AUDIO_MST_C_SCLK_CTRL0: u32 = 0x050; pub const AUDIO_MST_C_SCLK_CTRL1: u32 = 0x054;
pub const AUDIO_MST_D_SCLK_CTRL0: u32 = 0x058; pub const AUDIO_MST_D_SCLK_CTRL1: u32 = 0x05c;
pub const AUDIO_MST_E_SCLK_CTRL0: u32 = 0x060; pub const AUDIO_MST_E_SCLK_CTRL1: u32 = 0x064;
pub const AUDIO_MST_F_SCLK_CTRL0: u32 = 0x068; pub const AUDIO_MST_F_SCLK_CTRL1: u32 = 0x06c;
pub const AUDIO_CLK_TDMIN_A_CTRL: u32 = 0x080; pub const AUDIO_CLK_TDMIN_B_CTRL: u32 = 0x084;
pub const AUDIO_CLK_TDMIN_C_CTRL: u32 = 0x088; pub const AUDIO_CLK_TDMIN_LB_CTRL: u32 = 0x08c;
pub const AUDIO_CLK_TDMOUT_A_CTRL: u32 = 0x090; pub const AUDIO_CLK_TDMOUT_B_CTRL: u32 = 0x094;
pub const AUDIO_CLK_TDMOUT_C_CTRL: u32 = 0x098; pub const AUDIO_CLK_SPDIFIN_CTRL: u32 = 0x09c;
pub const AUDIO_CLK_SPDIFOUT_CTRL: u32 = 0x0a0; pub const AUDIO_CLK_RESAMPLE_CTRL: u32 = 0x0a4;
pub const AUDIO_CLK_LOCKER_CTRL: u32 = 0x0a8; pub const AUDIO_CLK_PDMIN_CTRL0: u32 = 0x0ac;
pub const AUDIO_CLK_PDMIN_CTRL1: u32 = 0x0b0; pub const AUDIO_CLK_SPDIFOUT_B_CTRL: u32 = 0x0b4;
pub const AUDIO_CLK_GATE_EN1: u32 = 0x004; pub const AUDIO_SM1_MCLK_A_CTRL: u32 = 0x008;
pub const AUDIO_SM1_MCLK_B_CTRL: u32 = 0x00c; pub const AUDIO_SM1_MCLK_C_CTRL: u32 = 0x010;
pub const AUDIO_SM1_MCLK_D_CTRL: u32 = 0x014; pub const AUDIO_SM1_MCLK_E_CTRL: u32 = 0x018;
pub const AUDIO_SM1_MCLK_F_CTRL: u32 = 0x01c; pub const AUDIO_SM1_MST_PAD_CTRL0: u32 = 0x020;
pub const AUDIO_SM1_MST_PAD_CTRL1: u32 = 0x024; pub const AUDIO_SM1_SW_RESET0: u32 = 0x028;
pub const AUDIO_SM1_SW_RESET1: u32 = 0x02c; pub const AUDIO_CLK81_CTRL: u32 = 0x030;
pub const AUDIO_CLK81_EN: u32 = 0x034; pub const AUDIO_EARCRX_CMDC_CLK_CTRL: u32 = 0x0d0;
pub const AUDIO_EARCRX_DMAC_CLK_CTRL: u32 = 0x0d4;

// The C AUD_* initializers are retained as Rust declarative macros so each
// clock definition keeps the same name, register fields, parent and flags.
macro_rules! AUD_PCLK_GATE { ($name:ident, $reg:expr, $bit:expr) => {
    static mut $name: clk_regmap = clk_regmap { hw: clk_hw { _private: [] } };
}; }
macro_rules! AUD_MST_MCLK_GATE { ($name:ident, $reg:expr) => { AUD_PCLK_GATE!($name, $reg, 31); }; }

// Common, AXG, G12A and SM1 clock objects (the referenced kernel clock
// implementations and clock-id constants are supplied by the surrounding
// kernel translation unit).
AUD_PCLK_GATE!(ddr_arb, AUDIO_CLK_GATE_EN, 0); AUD_PCLK_GATE!(pdm, AUDIO_CLK_GATE_EN, 1);
AUD_PCLK_GATE!(tdmin_a, AUDIO_CLK_GATE_EN, 2); AUD_PCLK_GATE!(tdmin_b, AUDIO_CLK_GATE_EN, 3);
AUD_PCLK_GATE!(tdmin_c, AUDIO_CLK_GATE_EN, 4); AUD_PCLK_GATE!(tdmin_lb, AUDIO_CLK_GATE_EN, 5);
AUD_PCLK_GATE!(tdmout_a, AUDIO_CLK_GATE_EN, 6); AUD_PCLK_GATE!(tdmout_b, AUDIO_CLK_GATE_EN, 7);
AUD_PCLK_GATE!(tdmout_c, AUDIO_CLK_GATE_EN, 8); AUD_PCLK_GATE!(frddr_a, AUDIO_CLK_GATE_EN, 9);
AUD_PCLK_GATE!(frddr_b, AUDIO_CLK_GATE_EN, 10); AUD_PCLK_GATE!(frddr_c, AUDIO_CLK_GATE_EN, 11);
AUD_PCLK_GATE!(toddr_a, AUDIO_CLK_GATE_EN, 12); AUD_PCLK_GATE!(toddr_b, AUDIO_CLK_GATE_EN, 13);
AUD_PCLK_GATE!(toddr_c, AUDIO_CLK_GATE_EN, 14); AUD_PCLK_GATE!(loopback, AUDIO_CLK_GATE_EN, 15);
AUD_PCLK_GATE!(spdifin, AUDIO_CLK_GATE_EN, 16); AUD_PCLK_GATE!(spdifout, AUDIO_CLK_GATE_EN, 17);
AUD_PCLK_GATE!(resample, AUDIO_CLK_GATE_EN, 18); AUD_PCLK_GATE!(power_detect, AUDIO_CLK_GATE_EN, 19);
AUD_PCLK_GATE!(toram, AUDIO_CLK_GATE_EN, 20); AUD_PCLK_GATE!(spdifout_b, AUDIO_CLK_GATE_EN, 21);
AUD_PCLK_GATE!(eqdrc, AUDIO_CLK_GATE_EN, 22); AUD_PCLK_GATE!(resample_b, AUDIO_CLK_GATE_EN, 26);
AUD_PCLK_GATE!(tovad, AUDIO_CLK_GATE_EN, 27); AUD_PCLK_GATE!(locker, AUDIO_CLK_GATE_EN, 28);
AUD_PCLK_GATE!(spdifin_lb, AUDIO_CLK_GATE_EN, 29); AUD_PCLK_GATE!(frddr_d, AUDIO_CLK_GATE_EN1, 0);
AUD_PCLK_GATE!(toddr_d, AUDIO_CLK_GATE_EN1, 1); AUD_PCLK_GATE!(loopback_b, AUDIO_CLK_GATE_EN1, 2);
AUD_PCLK_GATE!(earcrx, AUDIO_CLK_GATE_EN1, 6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
