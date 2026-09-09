/* SPDX-License-Identifier: GPL-2.0-only */
/* TI Clock driver internal definitions */

// C header dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct clk_omap_divider {
    pub hw: clk_hw,
    pub reg: clk_omap_reg,
    pub shift: u8,
    pub flags: u8,
    pub latch: i8,
    pub min: u16,
    pub max: u16,
    pub mask: u16,
    pub table: *const clk_div_table,
    pub context: u32,
}

#[macro_export]
macro_rules! to_clk_omap_divider {
    ($hw:expr) => { container_of!($hw, clk_omap_divider, hw) };
}

#[repr(C)]
pub struct clk_omap_mux {
    pub hw: clk_hw,
    pub reg: clk_omap_reg,
    pub table: *mut u32,
    pub mask: u32,
    pub shift: u8,
    pub latch: i8,
    pub flags: u8,
    pub saved_parent: u8,
}

#[macro_export]
macro_rules! to_clk_omap_mux {
    ($hw:expr) => { container_of!($hw, clk_omap_mux, hw) };
}

#[repr(C)]
pub enum TiClk {
    TI_CLK_FIXED,
    TI_CLK_MUX,
    TI_CLK_DIVIDER,
    TI_CLK_COMPOSITE,
    TI_CLK_FIXED_FACTOR,
    TI_CLK_GATE,
    TI_CLK_DPLL,
}

pub const CLKF_INDEX_POWER_OF_TWO: u32 = 1 << 0;
pub const CLKF_INDEX_STARTS_AT_ONE: u32 = 1 << 1;
pub const CLKF_SET_RATE_PARENT: u32 = 1 << 2;
pub const CLKF_OMAP3: u32 = 1 << 3;
pub const CLKF_AM35XX: u32 = 1 << 4;
pub const CLKF_SET_BIT_TO_DISABLE: u32 = 1 << 5;
pub const CLKF_INTERFACE: u32 = 1 << 6;
pub const CLKF_SSI: u32 = 1 << 7;
pub const CLKF_DSS: u32 = 1 << 8;
pub const CLKF_HSOTGUSB: u32 = 1 << 9;
pub const CLKF_WAIT: u32 = 1 << 10;
pub const CLKF_NO_WAIT: u32 = 1 << 11;
pub const CLKF_HSDIV: u32 = 1 << 12;
pub const CLKF_CLKDM: u32 = 1 << 13;
pub const CLKF_LOW_POWER_STOP: u32 = 1 << 5;
pub const CLKF_LOCK: u32 = 1 << 6;
pub const CLKF_LOW_POWER_BYPASS: u32 = 1 << 7;
pub const CLKF_PER: u32 = 1 << 8;
pub const CLKF_CORE: u32 = 1 << 9;
pub const CLKF_J_TYPE: u32 = 1 << 10;
pub const CLKF_SW_SUP: u32 = BIT(5);
pub const CLKF_HW_SUP: u32 = BIT(6);
pub const CLKF_NO_IDLEST: u32 = BIT(7);
pub const CLKF_SOC_MASK: u32 = GENMASK(11, 8);
pub const CLKF_SOC_NONSEC: u32 = BIT(8);
pub const CLKF_SOC_DRA72: u32 = BIT(9);
pub const CLKF_SOC_DRA74: u32 = BIT(10);
pub const CLKF_SOC_DRA76: u32 = BIT(11);

#[macro_export]
macro_rules! CLK {
    ($dev:expr, $con:expr, $ck:expr) => { { lk: { dev_id: $dev, con_id: $con }, clk: $ck } };
}

#[repr(C)]
pub struct ti_clk { pub name: *const i8, pub clkdm_name: *const i8, pub type_: i32, pub data: *mut core::ffi::c_void, pub patch: *mut ti_clk, pub clk: *mut clk }
#[repr(C)]
pub struct ti_clk_mux { pub bit_shift: u8, pub num_parents: i32, pub reg: u16, pub module: u8, pub parents: *const *const i8, pub flags: u16 }
#[repr(C)]
pub struct ti_clk_divider { pub parent: *const i8, pub bit_shift: u8, pub max_div: u16, pub reg: u16, pub module: u8, pub dividers: *mut i32, pub num_dividers: i32, pub flags: u16 }
#[repr(C)]
pub struct ti_clk_gate { pub parent: *const i8, pub bit_shift: u8, pub reg: u16, pub module: u8, pub flags: u16 }

pub const CLK_COMPONENT_TYPE_GATE: i32 = 0;
pub const CLK_COMPONENT_TYPE_DIVIDER: i32 = 1;
pub const CLK_COMPONENT_TYPE_MUX: i32 = 2;
pub const CLK_COMPONENT_TYPE_MAX: i32 = 3;

#[repr(C)]
pub struct ti_dt_clk { pub lk: clk_lookup, pub node_name: *mut i8 }
#[macro_export]
macro_rules! DT_CLK { ($dev:expr, $con:expr, $name:expr) => { { lk: { dev_id: $dev, con_id: $con }, node_name: $name } }; }

#[repr(C)]
pub struct omap_clkctrl_div_data { pub dividers: *const i32, pub max_div: i32, pub flags: u32 }
#[repr(C)]
pub struct omap_clkctrl_bit_data { pub bit: u8, pub type_: u8, pub parents: *const *const i8, pub data: *const core::ffi::c_void }
#[repr(C)]
pub struct omap_clkctrl_reg_data { pub offset: u16, pub bit_data: *const omap_clkctrl_bit_data, pub flags: u16, pub parent: *const i8, pub clkdm_name: *const i8 }
#[repr(C)]
pub struct omap_clkctrl_data { pub addr: u32, pub regs: *const omap_clkctrl_reg_data }

pub type ti_of_clk_init_cb_t = unsafe extern "C" fn(*mut core::ffi::c_void, *mut device_node);

extern "C" {
    pub static omap4_clkctrl_data: [omap_clkctrl_data; 0];
    pub static omap5_clkctrl_data: [omap_clkctrl_data; 0];
    pub static dra7_clkctrl_data: [omap_clkctrl_data; 0];
    pub static dra7_clkctrl_compat_data: [omap_clkctrl_data; 0];
    pub static mut dra7xx_compat_clks: [ti_dt_clk; 0];
    pub static am3_clkctrl_data: [omap_clkctrl_data; 0];
    pub static am3_clkctrl_compat_data: [omap_clkctrl_data; 0];
    pub static mut am33xx_compat_clks: [ti_dt_clk; 0];
    pub static am4_clkctrl_data: [omap_clkctrl_data; 0];
    pub static am438x_clkctrl_data: [omap_clkctrl_data; 0];
    pub static dm814_clkctrl_data: [omap_clkctrl_data; 0];
    pub static dm816_clkctrl_data: [omap_clkctrl_data; 0];
}

// External functions and clock-operation objects declared by the C header.
extern "C" {
    pub fn of_ti_clk_register(node: *mut device_node, hw: *mut clk_hw, con: *const i8) -> *mut clk;
    pub fn of_ti_clk_register_omap_hw(node: *mut device_node, hw: *mut clk_hw, con: *const i8) -> *mut clk;
    pub fn ti_dt_clk_name(np: *mut device_node) -> *const i8;
    pub fn ti_clk_add_alias(clk: *mut clk, con: *const i8) -> i32;
    pub fn ti_clk_add_aliases();
    pub fn ti_clk_latch(reg: *mut clk_omap_reg, shift: i8);
    pub fn ti_clk_build_component_mux(setup: *mut ti_clk_mux) -> *mut clk_hw;
    pub fn ti_clk_parse_divider_data(div_table: *mut i32, num_dividers: i32, max_div: i32, flags: u8, div: *mut clk_omap_divider) -> i32;
    pub fn ti_clk_get_reg_addr(node: *mut device_node, index: i32, reg: *mut clk_omap_reg) -> i32;
    pub fn ti_clk_get_legacy_bit_shift(node: *mut device_node) -> i32;
    pub fn ti_dt_clocks_register(oclks: *mut ti_dt_clk);
    pub fn ti_clk_retry_init(node: *mut device_node, user: *mut core::ffi::c_void, func: ti_of_clk_init_cb_t) -> i32;
    pub fn ti_clk_add_component(node: *mut device_node, hw: *mut clk_hw, type_: i32) -> i32;
    pub fn of_ti_clk_autoidle_setup(node: *mut device_node) -> i32;
    pub fn omap2_clk_enable_init_clocks(clk_names: *const *const i8, num_clocks: u8);
    pub static clkhwops_omap3_dpll: clk_hw_omap_ops;
    pub static clkhwops_omap4_dpllmx: clk_hw_omap_ops;
    pub static clkhwops_wait: clk_hw_omap_ops;
    pub static clkhwops_iclk: clk_hw_omap_ops;
    pub static clkhwops_iclk_wait: clk_hw_omap_ops;
    pub static clkhwops_omap2430_i2chs_wait: clk_hw_omap_ops;
    pub static clkhwops_omap3430es2_dss_usbhost_wait: clk_hw_omap_ops;
    pub static clkhwops_omap3430es2_iclk_hsotgusb_wait: clk_hw_omap_ops;
    pub static clkhwops_omap3430es2_iclk_dss_usbhost_wait: clk_hw_omap_ops;
    pub static clkhwops_omap3430es2_iclk_ssi_wait: clk_hw_omap_ops;
    pub static clkhwops_am35xx_ipss_module_wait: clk_hw_omap_ops;
    pub static clkhwops_am35xx_ipss_wait: clk_hw_omap_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
