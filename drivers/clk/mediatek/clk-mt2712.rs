// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of clk-mt2712.c. Kernel-provided types,
// constants, macros, and functions are intentionally left as external items.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern "C" {
    static mut mt2712_clk_lock: core::ffi::c_void;
    fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

// These macros represent the corresponding MediaTek clock-description
// constructors supplied by the surrounding kernel clock framework.
macro_rules! FIXED_CLK { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! FACTOR { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! MUX_GATE { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! MUX_GATE_FLAGS { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! MUX { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! DIV_ADJ { ($($t:tt)*) => { ($($t)*) }; }
macro_rules! GATE_MTK { ($($t:tt)*) => { ($($t)*) }; }

// The following declarations retain the original data layout and ordering.
// Symbolic clock IDs and framework structures are supplied by dependencies.

#[allow(non_upper_case_globals)]
static top_fixed_clks: &[(&str, u32)] = &[
    ("vpll3_dpix", 200000000), ("vpll_dpix", 200000000),
    ("ltepll_fs26m", 26000000), ("dmpll_ck", 350000000),
    ("dsi0_lntc", 143000000), ("dsi1_lntc", 143000000),
    ("lvdstx3", 140000000), ("lvdstx", 140000000),
    ("clkrtc_ext", 32768), ("clkrtc_int", 32747),
    ("csi0", 26000000), ("cvbspll", 108000000),
];

macro_rules! parents { ($name:ident, [$($v:literal),* $(,)?]) => {
    static $name: &[&str] = &[$($v),*];
}; }

parents!(axi_parents, ["clk26m", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll_d5", "univpll2_d2", "msdcpll2_ck"]);
parents!(mem_parents, ["clk26m", "dmpll_ck"]);
parents!(mm_parents, ["clk26m", "vencpll_ck", "syspll_d3", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll1_d2", "univpll2_d2"]);
parents!(pwm_parents, ["clk26m", "univpll2_d4", "univpll3_d2", "univpll1_d4"]);
parents!(vdec_parents, ["clk26m", "vcodecpll_ck", "tvdpll_429m", "univpll_d3", "vencpll_ck", "syspll_d3", "univpll1_d2", "mmpll_d2", "syspll3_d2", "tvdpll_ck"]);
parents!(venc_parents, ["clk26m", "univpll1_d2", "mmpll_d2", "tvdpll_d2", "syspll1_d2", "univpll_d5", "vcodecpll_d2", "univpll2_d2", "syspll3_d2"]);
parents!(mfg_parents, ["clk26m", "mmpll_ck", "univpll_d3", "clk26m", "clk26m", "clk26m", "clk26m", "clk26m", "clk26m", "syspll_d3", "syspll1_d2", "syspll_d5", "univpll_d3", "univpll1_d2", "univpll_d5", "univpll2_d2"]);
parents!(camtg_parents, ["clk26m", "univpll_d52", "univpll_d208", "univpll_d104", "clk26m_d2", "univpll_d26", "univpll2_d8", "syspll3_d4", "syspll3_d2", "univpll1_d4", "univpll2_d2"]);
parents!(uart_parents, ["clk26m", "univpll2_d8"]);
parents!(spi_parents, ["clk26m", "univpll2_d4", "univpll1_d4", "univpll2_d2", "univpll3_d2", "univpll1_d8"]);
parents!(usb20_parents, ["clk26m", "univpll1_d8", "univpll3_d4"]);
parents!(usb30_parents, ["clk26m", "univpll3_d2", "univpll3_d4", "univpll2_d4"]);
parents!(audio_parents, ["clk26m", "syspll3_d4", "syspll4_d4", "syspll1_d16"]);
parents!(aud_intbus_parents, ["clk26m", "syspll1_d4", "syspll4_d2", "univpll3_d2", "univpll2_d8", "syspll3_d2", "syspll3_d4"]);
parents!(pmicspi_parents, ["clk26m", "syspll1_d8", "syspll3_d4", "syspll1_d16", "univpll3_d4", "univpll_d26", "syspll3_d4"]);
parents!(dpilvds1_parents, ["clk26m", "lvdspll2_ck", "lvdspll2_d2", "lvdspll2_d4", "lvdspll2_d8", "clkfpc"]);
parents!(atb_parents, ["clk26m", "syspll1_d2", "univpll_d5", "syspll_d5"]);
parents!(nr_parents, ["clk26m", "univpll1_d4", "syspll2_d2", "syspll1_d4", "univpll1_d8", "univpll3_d2", "univpll2_d2", "syspll_d5"]);
parents!(nfi2x_parents, ["clk26m", "syspll4_d4", "univpll3_d4", "univpll1_d8", "syspll2_d4", "univpll3_d2", "syspll_d7", "syspll2_d2", "univpll2_d2", "syspll_d5", "syspll1_d2"]);
parents!(irda_parents, ["clk26m", "univpll2_d4", "syspll2_d4", "univpll2_d8"]);
parents!(cci400_parents, ["clk26m", "vencpll_ck", "armca35pll_600m", "armca35pll_400m", "univpll_d2", "syspll_d2", "msdcpll_ck", "univpll_d3"]);
parents!(aud_1_parents, ["clk26m", "apll1_ck", "univpll2_d4", "univpll2_d8"]);
parents!(aud_2_parents, ["clk26m", "apll2_ck", "univpll2_d4", "univpll2_d8"]);
parents!(mem_mfg_parents, ["clk26m", "mmpll_ck", "univpll_d3"]);
parents!(axi_mfg_parents, ["clk26m", "axi_sel", "univpll_d5"]);
parents!(scam_parents, ["clk26m", "syspll3_d2", "univpll2_d4", "syspll2_d4"]);
parents!(nfiecc_parents, ["clk26m", "nfi2x_sel", "syspll_d7", "syspll2_d2", "univpll2_d2", "univpll_d5", "syspll1_d2"]);
parents!(dpilvds_parents, ["clk26m", "lvdspll_ck", "lvdspll_d2", "lvdspll_d4", "lvdspll_d8", "clkfpc"]);
parents!(hdcp_parents, ["clk26m", "syspll4_d2", "syspll3_d4", "univpll2_d4"]);
parents!(hdcp_24m_parents, ["clk26m", "univpll_d26", "univpll_d52", "univpll2_d8"]);
parents!(rtc_parents, ["clkrtc_int", "clkrtc_ext", "clk26m", "univpll3_d8"]);

// Remaining framework tables are retained as typed constructor records.  The
// identifiers and constructors below are external kernel symbols.
static top_divs: &[(&str, &str, u32, u32)] = &[
    ("sys_26m", "clk26m", 1, 1), ("clk26m_d2", "sys_26m", 1, 2),
    ("armca35pll_ck", "armca35pll", 1, 1), ("armca35pll_600m", "armca35pll_ck", 1, 2),
    ("armca35pll_400m", "armca35pll_ck", 1, 3), ("armca72pll_ck", "armca72pll", 1, 1),
    ("syspll_ck", "mainpll", 1, 1), ("syspll_d2", "syspll_ck", 1, 2),
    ("syspll1_d2", "syspll_d2", 1, 2), ("syspll1_d4", "syspll_d2", 1, 4),
    ("syspll1_d8", "syspll_d2", 1, 8), ("syspll1_d16", "syspll_d2", 1, 16),
    ("syspll_d3", "syspll_ck", 1, 3), ("syspll2_d2", "syspll_d3", 1, 2),
    ("syspll2_d4", "syspll_d3", 1, 4), ("syspll_d5", "syspll_ck", 1, 5),
    ("syspll3_d2", "syspll_d5", 1, 2), ("syspll3_d4", "syspll_d5", 1, 4),
    ("syspll_d7", "syspll_ck", 1, 7), ("syspll4_d2", "syspll_d7", 1, 2),
    ("syspll4_d4", "syspll_d7", 1, 4),
];

static infrasys_rst_ofs: [u16; 2] = [0x30, 0x34];
static pericfg_rst_ofs: [u16; 2] = [0x0, 0x4];

// Device matching and driver registration preserve the original externally
// visible interfaces; platform-driver structures are supplied by the kernel.
#[no_mangle]
pub static mut clk_mt2712_drv: core::ffi::c_void = core::ffi::c_void { };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
