// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Microchip LAN966x SoC Clock driver.
 *
 * Copyright (C) 2021 Microchip Technology, Inc. and its subsidiaries
 *
 * Author: Kavyasree Kotagiri <kavyasree.kotagiri@microchip.com>
 */

// Linux kernel dependencies supplied externally.

const GCK_ENA: u32 = 1 << 0;
const GCK_SRC_SEL: u32 = 0x3 << 8;
const GCK_PRESCALER: u32 = 0xff << 16;
const DIV_MAX: u32 = 255;

static LAN966X_CLK_NAMES: [&[u8]; 14] = [
    b"qspi0\0", b"qspi1\0", b"qspi2\0", b"sdmmc0\0", b"pi\0", b"mcan0\0",
    b"mcan1\0", b"flexcom0\0", b"flexcom1\0", b"flexcom2\0", b"flexcom3\0",
    b"flexcom4\0", b"timer1\0", b"usb_refclk\0",
];

static LAN969X_CLK_NAMES: [&[u8]; 12] = [
    b"qspi0\0", b"qspi2\0", b"sdmmc0\0", b"sdmmc1\0", b"mcan0\0", b"mcan1\0",
    b"flexcom0\0", b"flexcom1\0", b"flexcom2\0", b"flexcom3\0", b"timer1\0",
    b"usb_refclk\0",
];

#[repr(C)]
struct Lan966xGck {
    hw: ClkHw,
    reg: *mut core::ffi::c_void,
}

#[repr(C)]
struct ClkHw {
    init: *const ClkInitData,
}

#[repr(C)]
struct ClkParentData {
    fw_name: *const core::ffi::c_char,
}

#[repr(C)]
struct ClkInitData {
    parent_data: *const ClkParentData,
    num_parents: usize,
    ops: *const ClkOps,
    name: *const u8,
}

#[repr(C)]
struct ClkOps {
    enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize, usize) -> i32>,
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    set_parent: Option<unsafe extern "C" fn(*mut ClkHw, u8) -> i32>,
    get_parent: Option<unsafe extern "C" fn(*mut ClkHw) -> u8>,
}

#[repr(C)]
struct ClkRateRequest {
    rate: usize,
    best_parent_hw: *mut ClkHw,
    best_parent_rate: usize,
}

#[repr(C)]
struct ClkGateSocDesc {
    name: *const core::ffi::c_char,
    bit_idx: i32,
}

#[repr(C)]
struct Lan966xMatchData {
    name: *mut core::ffi::c_char,
    clk_name: *const *const u8,
    clk_gate_desc: *const ClkGateSocDesc,
    num_generic_clks: u8,
    num_total_clks: u8,
}

static mut LAN966X_GCK_PDATA: [ClkParentData; 3] = [
    ClkParentData { fw_name: b"cpu\0".as_ptr() as *const _ },
    ClkParentData { fw_name: b"ddr\0".as_ptr() as *const _ },
    ClkParentData { fw_name: b"sys\0".as_ptr() as *const _ },
];

static mut INIT: ClkInitData = ClkInitData {
    parent_data: core::ptr::null(), num_parents: 3, ops: core::ptr::null(), name: core::ptr::null(),
};

static LAN966X_CLK_GATE_DESC: [ClkGateSocDesc; 4] = [
    ClkGateSocDesc { name: b"uhphs\0".as_ptr() as *const _, bit_idx: 11 },
    ClkGateSocDesc { name: b"udphs\0".as_ptr() as *const _, bit_idx: 10 },
    ClkGateSocDesc { name: b"mcramc\0".as_ptr() as *const _, bit_idx: 9 },
    ClkGateSocDesc { name: b"hmatrix\0".as_ptr() as *const _, bit_idx: 8 },
];

static LAN969X_CLK_GATE_DESC: [ClkGateSocDesc; 3] = [
    ClkGateSocDesc { name: b"usb_drd\0".as_ptr() as *const _, bit_idx: 10 },
    ClkGateSocDesc { name: b"mcramc\0".as_ptr() as *const _, bit_idx: 9 },
    ClkGateSocDesc { name: b"hmatrix\0".as_ptr() as *const _, bit_idx: 8 },
];

extern "C" {
    fn readl(reg: *mut core::ffi::c_void) -> u32;
    fn writel(val: u32, reg: *mut core::ffi::c_void);
    fn clk_hw_get_num_parents(hw: *mut ClkHw) -> i32;
    fn clk_hw_get_parent_by_index(hw: *mut ClkHw, index: i32) -> *mut ClkHw;
    fn clk_hw_get_rate(hw: *mut ClkHw) -> usize;
}

unsafe fn lan966x_gck_enable(hw: *mut ClkHw) -> i32 {
    let gck = hw as *mut Lan966xGck;
    let val = readl((*gck).reg) | GCK_ENA;
    writel(val, (*gck).reg);
    0
}

unsafe fn lan966x_gck_disable(hw: *mut ClkHw) {
    let gck = hw as *mut Lan966xGck;
    let val = readl((*gck).reg) & !GCK_ENA;
    writel(val, (*gck).reg);
}

unsafe fn lan966x_gck_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32 {
    let gck = hw as *mut Lan966xGck;
    if rate == 0 || parent_rate == 0 { return -22; }
    let div = parent_rate / rate;
    let mut val = readl((*gck).reg);
    val = (val & !GCK_PRESCALER) | (((div as u32 - 1) << 16) & GCK_PRESCALER);
    writel(val, (*gck).reg);
    0
}

unsafe fn lan966x_gck_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let gck = hw as *mut Lan966xGck;
    let div = (readl((*gck).reg) & GCK_PRESCALER) >> 16;
    parent_rate / (div as usize + 1)
}

unsafe fn lan966x_gck_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    for i in 0..clk_hw_get_num_parents(hw) {
        let parent = clk_hw_get_parent_by_index(hw, i);
        if parent.is_null() { continue; }
        let parent_rate = clk_hw_get_rate(parent);
        if parent_rate / (*req).rate <= DIV_MAX as usize {
            (*req).best_parent_hw = parent;
            (*req).best_parent_rate = parent_rate;
            return 0;
        }
    }
    -22
}

unsafe fn lan966x_gck_get_parent(hw: *mut ClkHw) -> u8 {
    let gck = hw as *mut Lan966xGck;
    ((readl((*gck).reg) & GCK_SRC_SEL) >> 8) as u8
}

unsafe fn lan966x_gck_set_parent(hw: *mut ClkHw, index: u8) -> i32 {
    let gck = hw as *mut Lan966xGck;
    let mut val = readl((*gck).reg) & !GCK_SRC_SEL;
    val |= ((index as u32) << 8) & GCK_SRC_SEL;
    writel(val, (*gck).reg);
    0
}

static LAN966X_GCK_OPS: ClkOps = ClkOps {
    enable: Some(lan966x_gck_enable), disable: Some(lan966x_gck_disable),
    set_rate: Some(lan966x_gck_set_rate), recalc_rate: Some(lan966x_gck_recalc_rate),
    determine_rate: Some(lan966x_gck_determine_rate), set_parent: Some(lan966x_gck_set_parent),
    get_parent: Some(lan966x_gck_get_parent),
};

static mut LAN966X_DESC: Lan966xMatchData = Lan966xMatchData {
    name: b"lan966x\0".as_ptr() as *mut _,
    clk_name: LAN966X_CLK_NAMES.as_ptr() as *const *const u8,
    clk_gate_desc: LAN966X_CLK_GATE_DESC.as_ptr(), num_generic_clks: 14, num_total_clks: 18,
};
static mut LAN969X_DESC: Lan966xMatchData = Lan966xMatchData {
    name: b"lan969x\0".as_ptr() as *mut _,
    clk_name: LAN969X_CLK_NAMES.as_ptr() as *const *const u8,
    clk_gate_desc: LAN969X_CLK_GATE_DESC.as_ptr(), num_generic_clks: 12, num_total_clks: 15,
};

// The following driver-registration and platform interfaces are supplied by the kernel.
extern "C" {
    fn lan966x_gck_clk_register(dev: *mut Device, i: i32) -> *mut ClkHw;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_hw_register_gate(dev: *mut Device, name: *const core::ffi::c_char,
        parent: *mut core::ffi::c_char, flags: u32, reg: *mut core::ffi::c_void,
        bit_idx: i32, flags2: u32, lock: *mut core::ffi::c_void) -> *mut ClkHw;
    fn dev_err_probe(dev: *mut Device, err: i32, fmt: *const core::ffi::c_char, ...) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn devm_of_clk_add_hw_provider(dev: *mut Device, get: *const core::ffi::c_void,
        data: *mut ClkHwOnecellData) -> i32;
    fn device_get_match_data(dev: *mut Device) -> *const Lan966xMatchData;
}

#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct ClkHwOnecellData { num: usize, hws: *mut *mut ClkHw }

unsafe fn lan966x_gate_clk_register(dev: *mut Device, data: *const Lan966xMatchData,
    hw_data: *mut ClkHwOnecellData, gate_base: *mut core::ffi::c_void) -> i32 {
    for i in ((*data).num_generic_clks as usize)..((*data).num_total_clks as usize) {
        let idx = i - (*data).num_generic_clks as usize;
        let desc = &*(*data).clk_gate_desc.add(idx);
        let hw = devm_clk_hw_register_gate(dev, desc.name, (*data).name as *mut _, 0,
            gate_base, desc.bit_idx, 0, core::ptr::null_mut());
        if hw.is_null() {
            return dev_err_probe(dev, -1, b"failed to register %s clock\n".as_ptr() as *const _, desc.name);
        }
        *(*hw_data).hws.add(i) = hw;
    }
    0
}

unsafe fn lan966x_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let data = device_get_match_data(dev);
    if data.is_null() { return -22; }
    let count = (*data).num_total_clks as usize;
    let hw_data = devm_kzalloc(dev, core::mem::size_of::<ClkHwOnecellData>() + count * core::mem::size_of::<*mut ClkHw>(), 0) as *mut ClkHwOnecellData;
    if hw_data.is_null() { return -12; }
    (*hw_data).num = (*data).num_generic_clks as usize;
    (*hw_data).hws = hw_data.add(1) as *mut *mut ClkHw;
    INIT.parent_data = LAN966X_GCK_PDATA.as_ptr();
    INIT.ops = &LAN966X_GCK_OPS;
    for i in 0..(*data).num_generic_clks as usize {
        INIT.name = *(*data).clk_name.add(i);
        let hw = lan966x_gck_clk_register(dev, i as i32);
        if hw.is_null() { dev_err(dev, b"failed to register %s clock\n".as_ptr() as *const _, INIT.name); return -1; }
        *(*hw_data).hws.add(i) = hw;
    }
    let ret = lan966x_gate_clk_register(dev, data, hw_data, core::ptr::null_mut());
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, core::ptr::null(), hw_data)
}

// Device tables, platform-driver registration, and module metadata are retained as
// externally supplied kernel integration declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
