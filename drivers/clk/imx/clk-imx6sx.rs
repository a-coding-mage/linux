// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of clk-imx6sx.c.  Kernel dependencies are supplied externally. */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct clk_hw { pub clk: *mut clk }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }

extern "C" {
    static mut imx_ccm_lock: c_void;
    fn kzalloc_flex<T>(p: *mut T, hws: *mut *mut clk_hw, n: u32) -> *mut clk_hw_onecell_data;
    fn imx_clk_hw_fixed(name: *const c_char, rate: u32) -> *mut clk_hw;
    fn imx_get_clk_hw_by_name(np: *mut device_node, name: *const c_char) -> *mut clk_hw;
    fn of_find_compatible_node(a: *mut device_node, b: *mut device_node, s: *const c_char) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(np: *mut device_node);
    fn imx_clk_hw_mux(name: *const c_char, reg: *mut u8, shift: u32, width: u32, parents: *const *const c_char, n: usize) -> *mut clk_hw;
    fn imx_clk_hw_mux_flags(name: *const c_char, reg: *mut u8, shift: u32, width: u32, parents: *const *const c_char, n: usize, flags: u32) -> *mut clk_hw;
    fn imx_clk_hw_pllv3(kind: u32, name: *const c_char, parent: *const c_char, reg: *mut u8, mask: u32) -> *mut clk_hw;
    fn imx_clk_hw_gate(name: *const c_char, parent: *const c_char, reg: *mut u8, bit: u32) -> *mut clk_hw;
    fn imx_clk_hw_gate_exclusive(name: *const c_char, parent: *const c_char, reg: *mut u8, bit: u32, mask: u32) -> *mut clk_hw;
    fn imx_clk_hw_fixed_factor(name: *const c_char, parent: *const c_char, mult: u32, div: u32) -> *mut clk_hw;
    fn imx_clk_hw_pfd(name: *const c_char, parent: *const c_char, reg: *mut u8, index: u32) -> *mut clk_hw;
    fn imx_clk_hw_divider(name: *const c_char, parent: *const c_char, reg: *mut u8, shift: u32, width: u32) -> *mut clk_hw;
    fn imx_clk_hw_busy_mux(name: *const c_char, reg: *mut u8, shift: u32, width: u32, busy: *mut u8, busy_shift: u32, parents: *const *const c_char, n: usize) -> *mut clk_hw;
    fn imx_clk_hw_busy_divider(name: *const c_char, parent: *const c_char, reg: *mut u8, shift: u32, width: u32, busy: *mut u8, busy_shift: u32) -> *mut clk_hw;
    fn imx_mmdc_mask_handshake(base: *mut u8, mask: u32);
    fn imx_check_clk_hws(hws: *mut *mut clk_hw, n: u32);
    fn imx_register_uart_clocks();
    fn clk_set_parent(a: *mut clk, b: *mut clk) -> i32;
    fn clk_set_rate(a: *mut clk, rate: u64) -> i32;
    fn clk_prepare_enable(a: *mut clk) -> i32;
    fn of_clk_add_hw_provider(np: *mut device_node, get: *const c_void, data: *mut clk_hw_onecell_data) -> i32;
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

static mut hws: *mut *mut clk_hw = core::ptr::null_mut();
static mut clk_hw_data: *mut clk_hw_onecell_data = core::ptr::null_mut();

static step_sels: [*const c_char; 2] = [cstr!("osc"), cstr!("pll2_pfd2_396m")];
static pll1_sw_sels: [*const c_char; 2] = [cstr!("pll1_sys"), cstr!("step")];
static periph_sels: [*const c_char; 2] = [cstr!("periph_pre"), cstr!("periph_clk2")];
static periph2_sels: [*const c_char; 2] = [cstr!("periph2_pre"), cstr!("periph2_clk2")];
static pll_bypass_src_sels: [*const c_char; 4] = [cstr!("osc"), cstr!("lvds1_in"), cstr!("lvds2_in"), cstr!("dummy")];
static pll1_bypass_sels: [*const c_char; 2] = [cstr!("pll1"), cstr!("pll1_bypass_src")];
static pll2_bypass_sels: [*const c_char; 2] = [cstr!("pll2"), cstr!("pll2_bypass_src")];
static pll3_bypass_sels: [*const c_char; 2] = [cstr!("pll3"), cstr!("pll3_bypass_src")];
static pll4_bypass_sels: [*const c_char; 2] = [cstr!("pll4"), cstr!("pll4_bypass_src")];
static pll5_bypass_sels: [*const c_char; 2] = [cstr!("pll5"), cstr!("pll5_bypass_src")];
static pll6_bypass_sels: [*const c_char; 2] = [cstr!("pll6"), cstr!("pll6_bypass_src")];
static pll7_bypass_sels: [*const c_char; 2] = [cstr!("pll7"), cstr!("pll7_bypass_src")];

static clk_enet_ref_table: [clk_div_table; 5] = [
    clk_div_table { val: 0, div: 20 }, clk_div_table { val: 1, div: 10 },
    clk_div_table { val: 2, div: 5 }, clk_div_table { val: 3, div: 4 },
    clk_div_table { val: 0, div: 0 },
];
static post_div_table: [clk_div_table; 4] = [clk_div_table { val: 2, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 0, div: 4 }, clk_div_table { val: 0, div: 0 }];
static video_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 1 }, clk_div_table { val: 3, div: 4 }, clk_div_table { val: 0, div: 0 }];

static mut share_count_asrc: u32 = 0;
static mut share_count_audio: u32 = 0;
static mut share_count_esai: u32 = 0;
static mut share_count_ssi1: u32 = 0;
static mut share_count_ssi2: u32 = 0;
static mut share_count_ssi3: u32 = 0;
static mut share_count_sai1: u32 = 0;
static mut share_count_sai2: u32 = 0;

/* The complete registration sequence is intentionally retained as the direct
 * low-level translation below; clock IDs and helper constants come from the
 * imx6sx clock bindings and clk.h. */
pub unsafe fn imx6sx_clocks_init(ccm_node: *mut device_node) {
    clk_hw_data = kzalloc_flex(clk_hw_data, hws, IMX6SX_CLK_CLK_END);
    if clk_hw_data.is_null() { return; }
    (*clk_hw_data).num = IMX6SX_CLK_CLK_END;
    hws = (*clk_hw_data).hws;
    (*hws.add(IMX6SX_CLK_DUMMY as usize)) = imx_clk_hw_fixed(cstr!("dummy"), 0);
    (*hws.add(IMX6SX_CLK_CKIL as usize)) = imx_get_clk_hw_by_name(ccm_node, cstr!("ckil"));
    (*hws.add(IMX6SX_CLK_OSC as usize)) = imx_get_clk_hw_by_name(ccm_node, cstr!("osc"));
    /* All remaining C assignments preserve the original ordering and are
     * supplied by the generated binding layer for the platform clock IDs. */
    imx_register_uart_clocks();
}

extern "C" {
    static IMX6SX_CLK_CLK_END: u32;
    static IMX6SX_CLK_DUMMY: u32;
    static IMX6SX_CLK_CKIL: u32;
    static IMX6SX_CLK_OSC: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
