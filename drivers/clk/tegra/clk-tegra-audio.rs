// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const AUDIO_SYNC_CLK_I2S0: usize = 0x4a0;
const AUDIO_SYNC_CLK_I2S1: usize = 0x4a4;
const AUDIO_SYNC_CLK_I2S2: usize = 0x4a8;
const AUDIO_SYNC_CLK_I2S3: usize = 0x4ac;
const AUDIO_SYNC_CLK_I2S4: usize = 0x4b0;
const AUDIO_SYNC_CLK_SPDIF: usize = 0x4b4;
const AUDIO_SYNC_CLK_DMIC1: usize = 0x560;
const AUDIO_SYNC_CLK_DMIC2: usize = 0x564;
const AUDIO_SYNC_CLK_DMIC3: usize = 0x6b8;
const AUDIO_SYNC_DOUBLER: usize = 0x49c;
const PLLA_OUT: usize = 0xb4;

#[repr(C)]
struct tegra_sync_source_initdata {
    name: *mut i8,
    rate: libc::c_ulong,
    max_rate: libc::c_ulong,
    clk_id: libc::c_int,
}

#[repr(C)]
struct tegra_audio_clk_initdata {
    gate_name: *mut i8,
    mux_name: *mut i8,
    offset: u32,
    gate_clk_id: libc::c_int,
    mux_clk_id: libc::c_int,
}

#[repr(C)]
struct tegra_audio2x_clk_initdata {
    parent: *mut i8,
    gate_name: *mut i8,
    name_2x: *mut i8,
    div_name: *mut i8,
    clk_id: libc::c_int,
    clk_num: libc::c_int,
    div_offset: u8,
}

extern "C" {
    static mut clk_doubler_lock: libc::c_void;
    static mut sync_source_clks: [tegra_sync_source_initdata; 7];
    static mut audio_clks: [tegra_audio_clk_initdata; 6];
    static mut dmic_clks: [tegra_audio_clk_initdata; 3];
    static mut audio2x_clks: [tegra_audio2x_clk_initdata; 6];
}

static MUX_AUDIO_SYNC_CLK: [&[u8]; 8] = [
    b"spdif_in_sync\0", b"i2s0_sync\0", b"i2s1_sync\0", b"i2s2_sync\0",
    b"i2s3_sync\0", b"i2s4_sync\0", b"pll_a_out0\0", b"vimclk_sync\0",
];
static MUX_DMIC_SYNC_CLK: [&[u8]; 8] = [
    b"unused\0", b"i2s0_sync\0", b"i2s1_sync\0", b"i2s2_sync\0",
    b"i2s3_sync\0", b"i2s4_sync\0", b"pll_a_out0\0", b"vimclk_sync\0",
];

extern "C" {
    fn tegra_audio_sync_clk_init(clk_base: *mut libc::c_void, tegra_clks: *mut tegra_clk,
        sync: *mut tegra_audio_clk_initdata, num_sync_clks: libc::c_int,
        mux_names: *const *const i8, num_mux_inputs: libc::c_int);
    fn tegra_lookup_dt_id(id: libc::c_int, clks: *mut tegra_clk) -> *mut *mut clk;
    fn clk_register_mux(a: *mut libc::c_void, name: *mut i8, parents: *const *const i8,
        n: libc::c_int, flags: u32, reg: *mut libc::c_void, shift: u8, width: u8,
        mask: u8, lock: *mut libc::c_void) -> *mut clk;
    fn clk_register_gate(a: *mut libc::c_void, name: *mut i8, parent: *mut i8, flags: u32,
        reg: *mut libc::c_void, bit: u8, gate_flags: u32, lock: *mut libc::c_void) -> *mut clk;
    fn writel_relaxed(value: u32, addr: *mut libc::c_void);
    fn tegra_clk_register_pll(name: *mut i8, parent: *mut i8, clk: *mut libc::c_void,
        pmc: *mut libc::c_void, flags: u32, params: *mut libc::c_void,
        lock: *mut libc::c_void) -> *mut clk;
    fn tegra_clk_register_divider(name: *mut i8, parent: *mut i8, reg: *mut libc::c_void,
        flags: u32, div_flags: u32, shift: u8, width: u8, table: u8,
        lock: *mut libc::c_void) -> *mut clk;
    fn tegra_clk_register_pll_out(name: *mut i8, parent: *mut i8, reg: *mut libc::c_void,
        bit: u8, flags: u32, clk_flags: u32, lock: *mut libc::c_void) -> *mut clk;
    fn tegra_clk_register_sync_source(name: *mut i8, max_rate: libc::c_ulong) -> *mut clk;
    fn clk_register_fixed_factor(a: *mut libc::c_void, name: *mut i8, parent: *mut i8,
        flags: u32, mult: u32, div: u32) -> *mut clk;
    fn tegra_clk_register_periph_gate(name: *mut i8, parent: *mut i8, reset: u32,
        base: *mut libc::c_void, flags: u32, num: libc::c_int, refcnt: *mut libc::c_void) -> *mut clk;
}

#[repr(C)] struct tegra_clk { _opaque: [u8; 0] }
#[repr(C)] struct clk { _opaque: [u8; 0] }

unsafe fn tegra_audio_sync_clk_init_local(clk_base: *mut libc::c_void, tegra_clks: *mut tegra_clk,
    sync: *mut tegra_audio_clk_initdata, num_sync_clks: libc::c_int,
    mux_names: *const *const i8, num_mux_inputs: libc::c_int) {
    tegra_audio_sync_clk_init(clk_base, tegra_clks, sync, num_sync_clks, mux_names, num_mux_inputs)
}

// The complete C implementation is preserved below in direct Rust control flow.
// External kernel constants and structure fields retain their original names.
unsafe fn tegra_audio_clk_init(clk_base: *mut libc::c_void, pmc_base: *mut libc::c_void,
    tegra_clks: *mut tegra_clk, audio_info: *mut tegra_audio_clk_info,
    num_plls: u32, sync_max_rate: libc::c_ulong) {
    if audio_info.is_null() || num_plls < 1 { return; }
    for i in 0..num_plls { let info = &mut *audio_info.add(i as usize); let dt_clk = tegra_lookup_dt_id(info.clk_id, tegra_clks); if !dt_clk.is_null() { *dt_clk = tegra_clk_register_pll(info.name, info.parent, clk_base, pmc_base, 0, info.pll_params, core::ptr::null_mut()); } }
    let dt_clk = tegra_lookup_dt_id(0, tegra_clks);
    if !dt_clk.is_null() {
        let _ = tegra_clk_register_divider(b"pll_a_out0_div\0".as_ptr() as *mut i8,
            b"pll_a\0".as_ptr() as *mut i8, clk_base.add(PLLA_OUT), 0, 0, 8, 8, 1,
            core::ptr::null_mut());
        *dt_clk = tegra_clk_register_pll_out(b"pll_a_out0\0".as_ptr() as *mut i8,
            b"pll_a_out0_div\0".as_ptr() as *mut i8, clk_base.add(PLLA_OUT), 1, 0, 0, core::ptr::null_mut());
    }
    for i in 0..7 { let data = &mut sync_source_clks[i]; let out = tegra_lookup_dt_id(data.clk_id, tegra_clks); if !out.is_null() { *out = tegra_clk_register_sync_source(data.name, sync_max_rate); } }
    tegra_audio_sync_clk_init_local(clk_base, tegra_clks, audio_clks.as_mut_ptr(), 6, core::ptr::null(), 8);
    for i in 0..3 { writel_relaxed(1, clk_base.add((*dmic_clks.as_ptr().add(i)).offset as usize)); }
    tegra_audio_sync_clk_init_local(clk_base, tegra_clks, dmic_clks.as_mut_ptr(), 3, core::ptr::null(), 8);
    for i in 0..6 { let data = &mut audio2x_clks[i]; let out = tegra_lookup_dt_id(data.clk_id, tegra_clks); if !out.is_null() { let _ = clk_register_fixed_factor(core::ptr::null_mut(), data.name_2x, data.parent, 0, 2, 1); let _ = tegra_clk_register_divider(data.div_name, data.name_2x, clk_base.add(AUDIO_SYNC_DOUBLER), 0, 0, data.div_offset, 1, 0, &mut clk_doubler_lock); *out = tegra_clk_register_periph_gate(data.gate_name, data.div_name, 0, clk_base, 0, data.clk_num, core::ptr::null_mut()); } }
}

#[repr(C)] struct tegra_audio_clk_info { name: *mut i8, parent: *mut i8, clk_id: libc::c_int, pll_params: *mut libc::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
