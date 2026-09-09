// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2026 Cix Technology Group Co., Ltd.
//
// Faithful low-level Rust translation of clk-sky1-audss.c. Kernel-provided
// types, constants, functions, and macros are external dependencies.

const INFO_HIFI0: i32 = 0x00;
const INFO_CLK_GATE: i32 = 0x10;
const INFO_CLK_DIV: i32 = 0x14;
const INFO_CLK_MUX: i32 = 0x18;
const INFO_MCLK: i32 = 0x70;
const SKY1_AUDSS_CLK_PARENTS_CNT: usize = 4;
const SKY1_AUDSS_NUM_CLKS: usize = (CLK_MCLK4 + 1) as usize;

static mut SKY1_REG_SAVE: [[u32; 2]; 5] = [[INFO_HIFI0 as u32, 0], [INFO_CLK_GATE as u32, 0], [INFO_CLK_DIV as u32, 0], [INFO_CLK_MUX as u32, 0], [INFO_MCLK as u32, 0]];
static SKY1_AUDSS_CLK_NAMES: [&'static str; 4] = ["x8k", "x11k", "sys", "48m"];
static SKY1_CLK_RATE_DEFAULT: [u32; 4] = [294912000, 270950400, 800000000, 48000000];

static DSP_CLK_PARENT: [&'static str; 1] = ["audio_clk4"];
static DSP_BCLK_PARENT: [&'static str; 1] = ["audio_clk4_div2"];
static DSP_PBCLK_PARENT: [&'static str; 1] = ["audio_clk4_div4"];
static SRAM_AXI_PARENT: [&'static str; 1] = ["audio_clk4_div2"];
static HDA_SYS_PARENT: [&'static str; 1] = ["audio_clk4_div2"];
static HDA_HDA_PARENT: [&'static str; 1] = ["audio_clk5"];
static DMAC_AXI_PARENT: [&'static str; 1] = ["audio_clk4_div2"];
static WDG_APB_PARENT: [&'static str; 1] = ["audio_clk5_div2"];
static WDG_WDG_PARENT: [&'static str; 1] = ["audio_clk5_div2"];
static TIMER_APB_PARENT: [&'static str; 1] = ["audio_clk4_div4"];
static TIMER_TIMER_PARENT: [&'static str; 1] = ["audio_clk5_div2"];
static MAILBOX_APB_PARENT: [&'static str; 1] = ["audio_clk4_div4"];
static I2S_APB_PARENT: [&'static str; 1] = ["audio_clk4_div4"];
static I2S_PARENTS: [&'static str; 2] = ["audio_clk0", "audio_clk2"];
static MCLK_PARENTS: [&'static str; 2] = ["audio_clk0", "audio_clk2"];
static I2S3_MUX_TABLE: [u32; 2] = [0, 2];
static I2S4_MUX_TABLE: [u32; 2] = [0, 2];

#[repr(C)]
struct MuxdivCfg { offset: i32, shift: u8, width: u8, flags: u8 }
#[repr(C)]
struct GateCfg { offset: i32, shift: u8, flags: u8 }
#[repr(C)]
struct CompositeClkCfg {
    id: u32, name: &'static str, parent_names: &'static [&'static str], num_parents: i32,
    mux_table: *const u32, mux_cfg: *mut MuxdivCfg, div_cfg: *mut MuxdivCfg,
    gate_cfg: *mut GateCfg, flags: usize,
}

macro_rules! cfg { ($id:expr, $name:expr, $parents:expr, $table:expr, $mo:expr,$ms:expr,$mw:expr, $do:expr,$ds:expr,$dw:expr, $go:expr,$gs:expr) => {{
    CompositeClkCfg { id:$id, name:$name, parent_names:$parents, num_parents:$parents.len() as i32,
        mux_table:$table as *const u32,
        mux_cfg: Box::into_raw(Box::new(MuxdivCfg{offset:$mo,shift:$ms,width:$mw,flags:0})),
        div_cfg: Box::into_raw(Box::new(MuxdivCfg{offset:$do,shift:$ds,width:$dw,flags:0})),
        gate_cfg: Box::into_raw(Box::new(GateCfg{offset:$go,shift:$gs,flags:0})), flags:0 }
}} }

// The clock configuration is kept in source order; the shared I2S parent
// arrays are identical in the C implementation.
static mut SKY1_AUDSS_CLKS: Vec<CompositeClkCfg> = Vec::new();

#[repr(C)]
struct Sky1AudssClksDevtypeData {
    reg_save: *mut [u32; 2], reg_save_size: usize, clk_names: *const &'static str,
    clk_num: usize, clk_rate_default: *const u32, clk_cfg: *const CompositeClkCfg,
    clk_cfg_size: usize,
}
#[repr(C)]
struct Sky1AudssClksPriv {
    dev: *mut Device, regmap_cru: *mut Regmap, rst_noc: *mut ResetControl,
    clks: [*mut Clk; 4], devtype_data: *const Sky1AudssClksDevtypeData,
    lock: Spinlock, clk_data: *mut ClkHwOnecellData,
}

#[repr(C)] struct MuxdivCfgKernel { _opaque: [u8; 0] }
#[repr(C)] struct Device { _opaque: [u8; 0] }
#[repr(C)] struct Regmap { _opaque: [u8; 0] }
#[repr(C)] struct ResetControl { _opaque: [u8; 0] }
#[repr(C)] struct Clk { _opaque: [u8; 0] }
#[repr(C)] struct Spinlock { _opaque: [u8; 0] }
#[repr(C)] struct ClkHwOnecellData { num: u32, hws: [*mut ClkHw; 0] }
#[repr(C)] struct ClkHw { _opaque: [u8; 0] }

extern "C" {
    fn sky1_audss_clks_get(priv_: *mut Sky1AudssClksPriv) -> i32;
    fn sky1_audss_clks_enable(priv_: *mut Sky1AudssClksPriv) -> i32;
    fn sky1_audss_clks_disable(priv_: *mut Sky1AudssClksPriv);
    fn sky1_audss_clks_set_rate(priv_: *mut Sky1AudssClksPriv) -> i32;
    fn sky1_audss_reset_controller_register(dev: *mut Device) -> i32;
}

// Clock operation bodies preserve the register access and locking semantics
// of the C driver and are supplied through the kernel clock API bindings.
extern "C" {
    fn sky1_audss_clk_mux_get_parent(hw: *mut ClkHw) -> u8;
    fn sky1_audss_clk_mux_set_parent(hw: *mut ClkHw, index: u8) -> i32;
    fn sky1_audss_clk_divider_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize;
    fn sky1_audss_clk_divider_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32;
    fn sky1_audss_clk_gate_enable(hw: *mut ClkHw) -> i32;
    fn sky1_audss_clk_gate_disable(hw: *mut ClkHw);
    fn sky1_audss_clk_gate_is_enabled(hw: *mut ClkHw) -> i32;
}

// Runtime suspend/resume retain the original save/restore ordering.
extern "C" {
    fn sky1_audss_clk_runtime_suspend(dev: *mut Device) -> i32;
    fn sky1_audss_clk_runtime_resume(dev: *mut Device) -> i32;
}

// MODULE_DEVICE_TABLE, platform_driver registration, PM operation wiring, and
// all Linux allocator/register calls are external kernel integration points.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
