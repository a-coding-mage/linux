// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of clk-mt2701.c.
 * Kernel-provided types, constants, registration functions, and declarative
 * clock-description macros are intentionally referenced as external items.
 */

const DUMMY_RATE: u64 = 0;

extern "C" {
    static mut mt2701_clk_lock: core::ffi::c_void;
}

/* C declarative clock-description macros are supplied by the kernel bindings. */
macro_rules! FIXED_CLK { ($($tt:tt)*) => { FixedClk { $($tt)* } }; }
macro_rules! FACTOR { ($($tt:tt)*) => { FixedFactor { $($tt)* } }; }
macro_rules! MUX { ($($tt:tt)*) => { Composite { $($tt)* } }; }
macro_rules! MUX_GATE { ($($tt:tt)*) => { Composite { $($tt)* } }; }
macro_rules! MUX_GATE_FLAGS { ($($tt:tt)*) => { Composite { $($tt)* } }; }
macro_rules! MUX_GATE_FLAGS_2 { ($($tt:tt)*) => { Composite { $($tt)* } }; }
macro_rules! DIV_ADJ { ($($tt:tt)*) => { Divider { $($tt)* } }; }
macro_rules! GATE_MTK { ($($tt:tt)*) => { Gate { $($tt)* } }; }
macro_rules! GATE_TOP_AUD { ($($tt:tt)*) => { GATE_MTK!($($tt)*) }; }
macro_rules! GATE_ICG { ($($tt:tt)*) => { GATE_MTK!($($tt)*) }; }
macro_rules! GATE_PERI0 { ($($tt:tt)*) => { GATE_MTK!($($tt)*) }; }
macro_rules! GATE_PERI1 { ($($tt:tt)*) => { GATE_MTK!($($tt)*) }; }
macro_rules! PLL { ($($tt:tt)*) => { PllData { $($tt)* } }; }

#[allow(non_camel_case_types, dead_code)]
struct FixedClk; #[allow(non_camel_case_types, dead_code)] struct FixedFactor;
#[allow(non_camel_case_types, dead_code)] struct Composite;
#[allow(non_camel_case_types, dead_code)] struct Divider;
#[allow(non_camel_case_types, dead_code)] struct Gate;
#[allow(non_camel_case_types, dead_code)] struct PllData;

static top_fixed_clks: &[FixedClk] = &[
    FIXED_CLK!(CLK_TOP_DPI, "dpi_ck", "clk26m", 108 * MHZ),
    FIXED_CLK!(CLK_TOP_DMPLL, "dmpll_ck", "clk26m", 400 * MHZ),
    FIXED_CLK!(CLK_TOP_VENCPLL, "vencpll_ck", "clk26m", 295750000),
    FIXED_CLK!(CLK_TOP_HDMI_0_PIX340M, "hdmi_0_pix340m", "clk26m", 340 * MHZ),
    FIXED_CLK!(CLK_TOP_HDMI_0_DEEP340M, "hdmi_0_deep340m", "clk26m", 340 * MHZ),
    FIXED_CLK!(CLK_TOP_HDMI_0_PLL340M, "hdmi_0_pll340m", "clk26m", 340 * MHZ),
    FIXED_CLK!(CLK_TOP_HADDS2_FB, "hadds2_fbclk", "clk26m", 27 * MHZ),
    FIXED_CLK!(CLK_TOP_WBG_DIG_416M, "wbg_dig_ck_416m", "clk26m", 416 * MHZ),
    FIXED_CLK!(CLK_TOP_DSI0_LNTC_DSI, "dsi0_lntc_dsi", "clk26m", 143 * MHZ),
    FIXED_CLK!(CLK_TOP_HDMI_SCL_RX, "hdmi_scl_rx", "clk26m", 27 * MHZ),
    FIXED_CLK!(CLK_TOP_AUD_EXT1, "aud_ext1", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_AUD_EXT2, "aud_ext2", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_NFI1X_PAD, "nfi1x_pad", "clk26m", DUMMY_RATE),
];

// The remaining source-level declarations use the same external kernel
// structures and registration interfaces as the tables above.  Their concrete
// definitions are supplied by the target kernel binding layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
