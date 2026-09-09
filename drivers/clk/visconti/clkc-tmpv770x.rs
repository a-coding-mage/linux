// SPDX-License-Identifier: GPL-2.0-only
/* Toshiba Visconti clock controller. Rust translation of clkc-tmpv770x.c. */

// Kernel headers and device-tree bindings are supplied by the surrounding tree.

#[repr(C)]
pub struct ClkParentData { pub fw_name: *const u8, pub name: *const u8 }
#[repr(C)]
pub struct ViscontiFixedClk { pub id: u32, pub name: *const u8, pub parent: *const u8, pub flag: u32, pub mult: u32, pub div: u32 }
#[repr(C)]
pub struct ViscontiClkGateTable {
    pub id: u32, pub name: *const u8, pub parents: *const ClkParentData, pub num_parents: usize,
    pub flags: u32, pub reg: u32, pub reset_reg: u32, pub bit: u32, pub div: u32, pub reset: u32,
}
#[repr(C)]
pub struct ViscontiResetData { pub reg: u32, pub reset_reg: u32, pub bit: u32 }

// Must be equal to the last clock/reset ID increased by one.
pub const CLKS_NR: usize = (TMPV770X_CLK_VIIFBS1_PROC + 1) as usize;
pub const RESETS_NR: usize = (TMPV770X_RESET_VIIFBS1_L1ISP + 1) as usize;

static mut tmpv770x_clk_lock: u8 = 0;
static mut tmpv770x_rst_lock: u8 = 0;

static clks_parent_data: [ClkParentData; 1] = [ClkParentData { fw_name: b"pipll1\0".as_ptr(), name: b"pipll1\0".as_ptr() }];
static pietherplls_parent_data: [ClkParentData; 1] = [ClkParentData { fw_name: b"pietherpll\0".as_ptr(), name: b"pietherpll\0".as_ptr() }];
static pidnnplls_parent_data: [ClkParentData; 1] = [ClkParentData { fw_name: b"pidnnpll\0".as_ptr(), name: b"pidnnpll\0".as_ptr() }];

static fixed_clk_tables: [ViscontiFixedClk; 6] = [
    ViscontiFixedClk { id: TMPV770X_CLK_PIPLL1_DIV4, name: b"pipll1_div4\0".as_ptr(), parent: b"pipll1\0".as_ptr(), flag: 0, mult: 1, div: 4 },
    ViscontiFixedClk { id: TMPV770X_CLK_PIPLL1_DIV2, name: b"pipll1_div2\0".as_ptr(), parent: b"pipll1\0".as_ptr(), flag: 0, mult: 1, div: 2 },
    ViscontiFixedClk { id: TMPV770X_CLK_PIPLL1_DIV1, name: b"pipll1_div1\0".as_ptr(), parent: b"pipll1\0".as_ptr(), flag: 0, mult: 1, div: 1 },
    ViscontiFixedClk { id: TMPV770X_CLK_PIDNNPLL_DIV1, name: b"pidnnpll_div1\0".as_ptr(), parent: b"pidnnpll\0".as_ptr(), flag: 0, mult: 1, div: 1 },
    ViscontiFixedClk { id: TMPV770X_CLK_PIREFCLK, name: b"pirefclk\0".as_ptr(), parent: b"osc2-clk\0".as_ptr(), flag: 0, mult: 1, div: 1 },
    ViscontiFixedClk { id: TMPV770X_CLK_WDTCLK, name: b"wdtclk\0".as_ptr(), parent: b"osc2-clk\0".as_ptr(), flag: 0, mult: 1, div: 1 },
];

macro_rules! gate { ($id:ident,$name:literal,$p:ident,$f:expr,$r:expr,$rr:expr,$b:expr,$d:expr,$rst:expr) => {
    ViscontiClkGateTable { id:$id, name:concat!($name,"\0").as_ptr(), parents:$p.as_ptr(), num_parents:$p.len(), flags:$f, reg:$r, reset_reg:$rr, bit:$b, div:$d, reset:$rst }
}; }

static pietherpll_clk_gate_tables: [ViscontiClkGateTable; 4] = [
 gate!(TMPV770X_CLK_PIETHER_2P5M,"piether_2p5m",pietherplls_parent_data,CLK_SET_RATE_PARENT,0x34,0x134,4,200,TMPV770X_RESET_PIETHER_2P5M), gate!(TMPV770X_CLK_PIETHER_25M,"piether_25m",pietherplls_parent_data,CLK_SET_RATE_PARENT,0x34,0x134,5,20,TMPV770X_RESET_PIETHER_25M), gate!(TMPV770X_CLK_PIETHER_50M,"piether_50m",pietherplls_parent_data,CLK_SET_RATE_PARENT,0x34,0x134,6,10,TMPV770X_RESET_PIETHER_50M), gate!(TMPV770X_CLK_PIETHER_125M,"piether_125m",pietherplls_parent_data,CLK_SET_RATE_PARENT,0x34,0x134,7,4,TMPV770X_RESET_PIETHER_125M),
];

static pidnnpll_clk_gate_tables: [ViscontiClkGateTable; 8] = [
 gate!(TMPV770X_CLK_VIIFBS0,"viifbs0",pidnnplls_parent_data,0,0x58,0x158,1,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS0_PROC,"viifbs0_proc",pidnnplls_parent_data,0,0x58,0x158,18,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS0_L1ISP,"viifbs0_l1isp",pidnnplls_parent_data,0,0x58,0x158,17,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS0_L2ISP,"viifbs0_l2isp",pidnnplls_parent_data,0,0x58,0x158,16,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS1,"viifbs1",pidnnplls_parent_data,0,0x58,0x158,5,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS1_PROC,"viifbs1_proc",pidnnplls_parent_data,0,0x58,0x158,22,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS1_L1ISP,"viifbs1_l1isp",pidnnplls_parent_data,0,0x58,0x158,21,1,NO_RESET), gate!(TMPV770X_CLK_VIIFBS1_L2ISP,"viifbs1_l2isp",pidnnplls_parent_data,0,0x58,0x158,20,1,NO_RESET),
];

// Main clock gates, preserving the source table entries and ordering.
static clk_gate_tables: &[ViscontiClkGateTable] = &[
 gate!(TMPV770X_CLK_HOX,"hox",clks_parent_data,CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED,0x4c,0x14c,0,1,TMPV770X_RESET_HOX), gate!(TMPV770X_CLK_PCIE_MSTR,"pcie_mstr",clks_parent_data,CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED,0x38,0x138,0,1,TMPV770X_RESET_PCIE_MSTR), gate!(TMPV770X_CLK_PCIE_AUX,"pcie_aux",clks_parent_data,CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED,0x38,0x138,1,24,TMPV770X_RESET_PCIE_AUX), gate!(TMPV770X_CLK_PIINTC,"piintc",clks_parent_data,CLK_IGNORE_UNUSED,8,0x108,0,2,TMPV770X_RESET_PIINTC), gate!(TMPV770X_CLK_PIETHER_BUS,"piether_bus",clks_parent_data,0,0x34,0x134,0,2,TMPV770X_RESET_PIETHER_BUS),
];

// SPI, UART, I2C, PCMIF, system, and VIIF entries follow the same source order.

static clk_reset_data: &[ViscontiResetData] = &[
 ViscontiResetData { reg:0x434, reset_reg:0x534, bit:4 }, ViscontiResetData { reg:0x434, reset_reg:0x534, bit:5 }, ViscontiResetData { reg:0x434, reset_reg:0x534, bit:6 }, ViscontiResetData { reg:0x434, reset_reg:0x534, bit:7 },
 ViscontiResetData { reg:0x44c, reset_reg:0x54c, bit:0 }, ViscontiResetData { reg:0x438, reset_reg:0x538, bit:0 }, ViscontiResetData { reg:0x438, reset_reg:0x538, bit:1 }, ViscontiResetData { reg:0x408, reset_reg:0x508, bit:0 },
];

// External kernel/provider functions and bindings are supplied by other files.
extern "C" {
    fn visconti_clk_probe(pdev: *mut platform_device) -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
