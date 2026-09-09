// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell MV98DX3236 SoC clocks
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

use core::ffi::c_void;

type U32 = u32;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CoreclkRatio {
    pub id: i32,
    pub name: *const u8,
}

#[repr(C)]
pub struct CoreclkSocDesc {
    pub get_tclk_freq: unsafe extern "C" fn(*mut c_void) -> U32,
    pub get_cpu_freq: unsafe extern "C" fn(*mut c_void) -> U32,
    pub get_clk_ratio: unsafe extern "C" fn(*mut c_void, i32, *mut i32, *mut i32),
    pub ratios: *const CoreclkRatio,
    pub num_ratios: usize,
}

#[repr(C)]
pub struct ClkGatingSocDesc {
    pub name: *const u8,
    pub bit_idx: *const u8,
    pub bit: i32,
    pub flags: i32,
}

extern "C" {
    fn readl(addr: *mut c_void) -> U32;
    fn of_machine_is_compatible(compat: *const u8) -> bool;
    fn pr_err(fmt: *const u8, ...);
    fn of_find_compatible_node(from: *mut DeviceNode, typ: *const u8, compat: *const u8) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn mvebu_coreclk_setup(np: *mut DeviceNode, desc: *const CoreclkSocDesc);
    fn mvebu_clk_gating_setup(np: *mut DeviceNode, desc: *const ClkGatingSocDesc);
}

const SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT: u32 = 18;
const SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK: u32 = 0x7;

unsafe extern "C" fn mv98dx3236_get_tclk_freq(_sar: *mut c_void) -> U32 {
    /* Tclk = 200MHz, no SaR dependency */
    200000000
}

static MV98DX3236_CPU_FREQUENCIES: [U32; 8] = [0, 667000000, 400000000, 800000000, 0, 800000000, 0, 0];
static MV98DX4251_CPU_FREQUENCIES: [U32; 8] = [400000000, 0, 667000000, 800000000, 0, 0, 0, 0];

unsafe extern "C" fn mv98dx3236_get_cpu_freq(sar: *mut c_void) -> U32 {
    let mut cpu_freq = 0;
    let cpu_freq_select = ((readl(sar) >> SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT)
        & SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK) as usize;

    if of_machine_is_compatible(b"marvell,armadaxp-98dx4251\0".as_ptr()) {
        cpu_freq = MV98DX4251_CPU_FREQUENCIES[cpu_freq_select];
    } else if of_machine_is_compatible(b"marvell,armadaxp-98dx3236\0".as_ptr()) {
        cpu_freq = MV98DX3236_CPU_FREQUENCIES[cpu_freq_select];
    }

    if cpu_freq == 0 {
        pr_err(b"CPU freq select unsupported %d\n\0".as_ptr(), cpu_freq_select as i32);
    }
    cpu_freq
}

const MV98DX3236_CPU_TO_DDR: i32 = 0;
const MV98DX3236_CPU_TO_MPLL: i32 = 1;

static MV98DX3236_CORE_RATIOS: [CoreclkRatio; 2] = [
    CoreclkRatio { id: MV98DX3236_CPU_TO_DDR, name: b"ddrclk\0".as_ptr() },
    CoreclkRatio { id: MV98DX3236_CPU_TO_MPLL, name: b"mpll\0".as_ptr() },
];

static MV98DX3236_CPU_MPLL_RATIOS: [[i32; 2]; 8] = [[0, 1], [3, 1], [1, 1], [1, 1], [0, 1], [1, 1], [0, 1], [0, 1]];
static MV98DX3236_CPU_DDR_RATIOS: [[i32; 2]; 8] = [[0, 1], [1, 1], [1, 1], [1, 1], [0, 1], [1, 2], [0, 1], [0, 1]];
static MV98DX4251_CPU_MPLL_RATIOS: [[i32; 2]; 8] = [[2, 1], [0, 1], [3, 1], [2, 1], [0, 1], [0, 1], [0, 1], [0, 1]];
static MV98DX4251_CPU_DDR_RATIOS: [[i32; 2]; 8] = [[1, 1], [0, 1], [1, 1], [1, 1], [0, 1], [0, 1], [0, 1], [0, 1]];

unsafe extern "C" fn mv98dx3236_get_clk_ratio(sar: *mut c_void, id: i32, mult: *mut i32, div: *mut i32) {
    let opt = ((readl(sar) >> SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT) & SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK) as usize;
    match id {
        MV98DX3236_CPU_TO_DDR => {
            let ratios = if of_machine_is_compatible(b"marvell,armadaxp-98dx4251\0".as_ptr()) { &MV98DX4251_CPU_DDR_RATIOS } else if of_machine_is_compatible(b"marvell,armadaxp-98dx3236\0".as_ptr()) { &MV98DX3236_CPU_DDR_RATIOS } else { return };
            *mult = ratios[opt][0]; *div = ratios[opt][1];
        }
        MV98DX3236_CPU_TO_MPLL => {
            let ratios = if of_machine_is_compatible(b"marvell,armadaxp-98dx4251\0".as_ptr()) { &MV98DX4251_CPU_MPLL_RATIOS } else if of_machine_is_compatible(b"marvell,armadaxp-98dx3236\0".as_ptr()) { &MV98DX3236_CPU_MPLL_RATIOS } else { return };
            *mult = ratios[opt][0]; *div = ratios[opt][1];
        }
        _ => {}
    }
}

static MV98DX3236_CORE_CLOCKS: CoreclkSocDesc = CoreclkSocDesc {
    get_tclk_freq: mv98dx3236_get_tclk_freq,
    get_cpu_freq: mv98dx3236_get_cpu_freq,
    get_clk_ratio: mv98dx3236_get_clk_ratio,
    ratios: MV98DX3236_CORE_RATIOS.as_ptr(), num_ratios: 2,
};

static MV98DX3236_GATING_DESC: [ClkGatingSocDesc; 7] = [
    ClkGatingSocDesc { name: b"ge1\0".as_ptr(), bit_idx: core::ptr::null(), bit: 3, flags: 0 },
    ClkGatingSocDesc { name: b"ge0\0".as_ptr(), bit_idx: core::ptr::null(), bit: 4, flags: 0 },
    ClkGatingSocDesc { name: b"pex00\0".as_ptr(), bit_idx: core::ptr::null(), bit: 5, flags: 0 },
    ClkGatingSocDesc { name: b"sdio\0".as_ptr(), bit_idx: core::ptr::null(), bit: 17, flags: 0 },
    ClkGatingSocDesc { name: b"usb0\0".as_ptr(), bit_idx: core::ptr::null(), bit: 18, flags: 0 },
    ClkGatingSocDesc { name: b"xor0\0".as_ptr(), bit_idx: core::ptr::null(), bit: 22, flags: 0 },
    ClkGatingSocDesc { name: core::ptr::null(), bit_idx: core::ptr::null(), bit: 0, flags: 0 },
];

unsafe extern "C" fn mv98dx3236_clk_init(np: *mut DeviceNode) {
    let cgnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"marvell,mv98dx3236-gating-clock\0".as_ptr());
    mvebu_coreclk_setup(np, &MV98DX3236_CORE_CLOCKS);
    if !cgnp.is_null() {
        mvebu_clk_gating_setup(cgnp, MV98DX3236_GATING_DESC.as_ptr());
        of_node_put(cgnp);
    }
}

// CLK_OF_DECLARE(mv98dx3236_clk, "marvell,armadaxp-98dx3236-core-clock", mv98dx3236_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
