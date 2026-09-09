// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Kirkwood SoC clocks
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies are supplied by the surrounding kernel translation.

const SAR_KIRKWOOD_DDR_RATIO: u32 = 5;
const SAR_KIRKWOOD_DDR_RATIO_MASK: u32 = 0xf;
const SAR_MV88F6180_CLK: u32 = 2;
const SAR_MV88F6180_CLK_MASK: u32 = 0x7;
const SAR_KIRKWOOD_TCLK_FREQ: u32 = 21;
const SAR_KIRKWOOD_TCLK_FREQ_MASK: u32 = 0x1;

const KIRKWOOD_CPU_TO_L2: i32 = 0;
const KIRKWOOD_CPU_TO_DDR: i32 = 1;

#[inline]
fn sar_kirkwood_cpu_freq(x: u32) -> u32 {
    ((x & (1 << 1)) >> 1) | ((x & (1 << 22)) >> 21) | ((x & (3 << 3)) >> 1)
}

#[inline]
fn sar_kirkwood_l2_ratio(x: u32) -> u32 {
    ((x & (3 << 9)) >> 9) | ((x & (1 << 19)) >> 17)
}

static KIRKWOOD_CORECLK_RATIOS: [coreclk_ratio; 2] = [
    coreclk_ratio { id: KIRKWOOD_CPU_TO_L2, name: b"l2clk\0" as *const u8 as *const i8 },
    coreclk_ratio { id: KIRKWOOD_CPU_TO_DDR, name: b"ddrclk\0" as *const u8 as *const i8 },
];

unsafe extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn mvebu_coreclk_setup(np: *mut device_node, desc: *const coreclk_soc_desc);
    fn mvebu_clk_gating_setup(np: *mut device_node, desc: *const clk_gating_soc_desc);
    fn of_find_compatible_node(from: *mut device_node, type_: *const i8, compat: *const i8) -> *mut device_node;
    fn of_device_is_compatible(np: *mut device_node, compat: *const i8) -> bool;
    fn of_node_put(np: *mut device_node);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn of_clk_add_provider(np: *mut device_node, get: unsafe extern "C" fn(*mut of_phandle_args, *mut core::ffi::c_void) -> *mut clk, data: *mut core::ffi::c_void) -> i32;
    fn kzalloc_flex(size: usize) -> *mut clk_muxing_ctrl;
    fn clk_register_mux(clock: *mut clk, name: *const i8, parents: *const *const i8, num_parents: i32, flags: usize, base: *mut core::ffi::c_void, shift: i32, width: i32, mux_flags: usize, lock: *mut spinlock_t) -> *mut clk;
    fn __clk_get_hw(clk: *mut clk) -> *mut clk_hw;
    fn to_clk_mux(hw: *mut clk_hw) -> *mut clk_mux;
    fn err_ptr(error: i32) -> *mut clk;
}

#[repr(C)]
struct coreclk_ratio { id: i32, name: *const i8 }

#[repr(C)]
struct coreclk_soc_desc {
    get_tclk_freq: unsafe extern "C" fn(*mut core::ffi::c_void) -> u32,
    get_cpu_freq: unsafe extern "C" fn(*mut core::ffi::c_void) -> u32,
    get_clk_ratio: unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32),
    ratios: *const coreclk_ratio,
    num_ratios: usize,
}

#[repr(C)]
struct clk_gating_soc_desc { name: *const i8, bit_idx: *const i8, bit: i32, flags: i32 }

#[repr(C)]
struct clk_muxing_soc_desc { name: *const i8, parents: *const *const i8, num_parents: i32, shift: i32, width: i32, flags: usize }

#[repr(C)]
struct clk_muxing_ctrl { lock: *mut spinlock_t, num_muxes: i32, muxes: [*mut clk; 0] }

unsafe extern "C" fn kirkwood_get_tclk_freq(sar: *mut core::ffi::c_void) -> u32 {
    let opt = (readl(sar) >> SAR_KIRKWOOD_TCLK_FREQ) & SAR_KIRKWOOD_TCLK_FREQ_MASK;
    if opt != 0 { 166666667 } else { 200000000 }
}

static KIRKWOOD_CPU_FREQS: [u32; 16] = [0, 0, 0, 0, 600000000, 0, 800000000, 1000000000, 0, 1200000000, 0, 0, 1500000000, 1600000000, 1800000000, 2000000000];

unsafe extern "C" fn kirkwood_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 { KIRKWOOD_CPU_FREQS[sar_kirkwood_cpu_freq(readl(sar)) as usize] }

static KIRKWOOD_CPU_L2_RATIOS: [[i32; 2]; 8] = [[0,1],[1,2],[0,1],[1,3],[0,1],[1,4],[0,1],[0,1]];
static KIRKWOOD_CPU_DDR_RATIOS: [[i32; 2]; 16] = [[0,1],[0,1],[1,2],[0,1],[1,3],[0,1],[1,4],[2,9],[1,5],[1,6],[0,1],[0,1],[0,1],[0,1],[0,1],[0,1]];

unsafe extern "C" fn kirkwood_get_clk_ratio(sar: *mut core::ffi::c_void, id: i32, mult: *mut i32, div: *mut i32) {
    match id {
        KIRKWOOD_CPU_TO_L2 => { let opt = sar_kirkwood_l2_ratio(readl(sar)) as usize; (*mult, *div) = (KIRKWOOD_CPU_L2_RATIOS[opt][0], KIRKWOOD_CPU_L2_RATIOS[opt][1]); }
        KIRKWOOD_CPU_TO_DDR => { let opt = ((readl(sar) >> SAR_KIRKWOOD_DDR_RATIO) & SAR_KIRKWOOD_DDR_RATIO_MASK) as usize; (*mult, *div) = (KIRKWOOD_CPU_DDR_RATIOS[opt][0], KIRKWOOD_CPU_DDR_RATIOS[opt][1]); }
        _ => {}
    }
}

static MV88F6180_CPU_FREQS: [u32; 8] = [0,0,0,0,0,600000000,800000000,1000000000];
unsafe extern "C" fn mv88f6180_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 { MV88F6180_CPU_FREQS[((readl(sar) >> SAR_MV88F6180_CLK) & SAR_MV88F6180_CLK_MASK) as usize] }
static MV88F6180_CPU_DDR_RATIOS: [[i32; 2]; 8] = [[0,1],[0,1],[0,1],[0,1],[0,1],[1,3],[1,4],[1,5]];
unsafe extern "C" fn mv88f6180_get_clk_ratio(sar: *mut core::ffi::c_void, id: i32, mult: *mut i32, div: *mut i32) {
    match id { KIRKWOOD_CPU_TO_L2 => { *mult=1; *div=2; }, KIRKWOOD_CPU_TO_DDR => { let opt=((readl(sar)>>SAR_MV88F6180_CLK)&SAR_MV88F6180_CLK_MASK) as usize; (*mult,*div)=(MV88F6180_CPU_DDR_RATIOS[opt][0],MV88F6180_CPU_DDR_RATIOS[opt][1]); }, _=>{} }
}
unsafe extern "C" fn mv98dx1135_get_tclk_freq(_: *mut core::ffi::c_void) -> u32 { 166666667 }

static KIRKWOOD_GATING_DESC: [clk_gating_soc_desc; 16] = [
    clk_gating_soc_desc { name:b"ge0\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:0, flags:0 },
    clk_gating_soc_desc { name:b"pex0\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:2, flags:0 },
    clk_gating_soc_desc { name:b"usb0\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:3, flags:0 },
    clk_gating_soc_desc { name:b"sdio\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:4, flags:0 },
    clk_gating_soc_desc { name:b"tsu\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:5, flags:0 },
    clk_gating_soc_desc { name:b"runit\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:7, flags:0 },
    clk_gating_soc_desc { name:b"xor0\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:8, flags:0 },
    clk_gating_soc_desc { name:b"audio\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:9, flags:0 },
    clk_gating_soc_desc { name:b"sata0\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:14, flags:0 },
    clk_gating_soc_desc { name:b"sata1\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:15, flags:0 },
    clk_gating_soc_desc { name:b"xor1\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:16, flags:0 },
    clk_gating_soc_desc { name:b"crypto\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:17, flags:0 },
    clk_gating_soc_desc { name:b"pex1\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:18, flags:0 },
    clk_gating_soc_desc { name:b"ge1\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:19, flags:0 },
    clk_gating_soc_desc { name:b"tdm\0" as *const u8 as *const i8, bit_idx:core::ptr::null(), bit:20, flags:0 },
    clk_gating_soc_desc { name:core::ptr::null(), bit_idx:core::ptr::null(), bit:0, flags:0 },
];

static POWERSAVE_PARENTS: [*const i8; 2] = [b"cpuclk\0" as *const u8 as *const i8, b"ddrclk\0" as *const u8 as *const i8];
static KIRKWOOD_MUX_DESC: [clk_muxing_soc_desc; 2] = [
    clk_muxing_soc_desc { name:b"powersave\0" as *const u8 as *const i8, parents:POWERSAVE_PARENTS.as_ptr(), num_parents:2, shift:11, width:1, flags:0 },
    clk_muxing_soc_desc { name:core::ptr::null(), parents:core::ptr::null(), num_parents:0, shift:0, width:0, flags:0 },
];

#[repr(C)] struct device_node;
#[repr(C)] struct of_phandle_args { args_count: i32, args: *const u32 }
#[repr(C)] struct clk; #[repr(C)] struct clk_hw; #[repr(C)] struct clk_mux; #[repr(C)] struct spinlock_t;

unsafe extern "C" fn clk_muxing_get_src(clkspec: *mut of_phandle_args, data: *mut core::ffi::c_void) -> *mut clk {
    let ctrl = &*(data as *mut clk_muxing_ctrl);
    if (*clkspec).args_count < 1 { return err_ptr(-22); }
    for n in 0..ctrl.num_muxes as usize {
        let mux = to_clk_mux(__clk_get_hw(*ctrl.muxes.as_ptr().add(n)));
        // The mux shift is read from the kernel's clk_mux object.
        let shift = *(mux as *mut i32);
        if *(*clkspec).args == shift as u32 { return *ctrl.muxes.as_ptr().add(n); }
    }
    err_ptr(-19)
}

unsafe extern "C" fn kirkwood_clk_muxing_setup(np: *mut device_node, desc: *const clk_muxing_soc_desc) {
    let base = of_iomap(np, 0);
    if base.is_null() { return; }
    let mut n = 0usize;
    while !(*desc.add(n)).name.is_null() { n += 1; }
    let ctrl = kzalloc_flex(core::mem::size_of::<clk_muxing_ctrl>() + n * core::mem::size_of::<*mut clk>()) as *mut clk_muxing_ctrl;
    if ctrl.is_null() { iounmap(base); return; }
    (*ctrl).num_muxes = n as i32;
    (*ctrl).lock = core::ptr::null_mut();
    for i in 0..n {
        let d = &*desc.add(i);
        let mux = clk_register_mux(core::ptr::null_mut(), d.name, d.parents, d.num_parents, d.flags, base, d.shift, d.width, d.flags, (*ctrl).lock);
        (*ctrl).muxes.as_mut_ptr().add(i).write(mux);
    }
    of_clk_add_provider(np, clk_muxing_get_src, ctrl as *mut core::ffi::c_void);
}

unsafe extern "C" fn kirkwood_clk_init(np: *mut device_node) {
    let cgnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"marvell,kirkwood-gating-clock\0" as *const u8 as *const i8);
    if of_device_is_compatible(np, b"marvell,mv88f6180-core-clock\0" as *const u8 as *const i8) {
        mvebu_coreclk_setup(np, core::ptr::null());
    } else if of_device_is_compatible(np, b"marvell,mv98dx1135-core-clock\0" as *const u8 as *const i8) {
        mvebu_coreclk_setup(np, core::ptr::null());
    } else { mvebu_coreclk_setup(np, core::ptr::null()); }
    if !cgnp.is_null() {
        mvebu_clk_gating_setup(cgnp, KIRKWOOD_GATING_DESC.as_ptr());
        kirkwood_clk_muxing_setup(cgnp, KIRKWOOD_MUX_DESC.as_ptr());
        of_node_put(cgnp);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
