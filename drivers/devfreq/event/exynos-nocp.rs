// SPDX-License-Identifier: GPL-2.0-only
/*
 * exynos-nocp.c - Exynos NoC (Network On Chip) Probe support
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 * Author : Chanwoo Choi <cw00.choi@samsung.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct ExynosNocp {
    edev: *mut DevfreqEventDev,
    desc: DevfreqEventDesc,
    dev: *mut Device,
    regmap: *mut Regmap,
    clk: *mut Clk,
}

// The following types, constants, and functions are supplied by other files.
#[repr(C)] struct DevfreqEventDev { dev: Device, desc: *const DevfreqEventDesc }
#[repr(C)] struct DevfreqEventDesc { ops: *const DevfreqEventOps, driver_data: *mut core::ffi::c_void, name: *const core::ffi::c_char }
#[repr(C)] struct DevfreqEventData { load_count: u64, total_count: u64 }
#[repr(C)] struct DevfreqEventOps {
    set_event: Option<unsafe extern "C" fn(*mut DevfreqEventDev) -> i32>,
    get_event: Option<unsafe extern "C" fn(*mut DevfreqEventDev, *mut DevfreqEventData) -> i32>,
}
#[repr(C)] struct Device { of_node: *mut DeviceNode }
#[repr(C)] struct DeviceNode { full_name: *const core::ffi::c_char }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Resource;
#[repr(C)] struct Regmap;
#[repr(C)] struct Clk;
#[repr(C)] struct RegmapConfig { reg_bits: u32, val_bits: u32, reg_stride: u32, max_register: u32 }
#[repr(C)] struct OfDeviceId { compatible: *const core::ffi::c_char }
#[repr(C)] struct PlatformDriver { probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>, remove: Option<unsafe extern "C" fn(*mut PlatformDevice)> }

extern "C" {
    fn devfreq_event_get_drvdata(edev: *mut DevfreqEventDev) -> *mut core::ffi::c_void;
    fn regmap_update_bits(map: *mut Regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_write(map: *mut Regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut Regmap, reg: u32, val: *mut u32) -> i32;
    fn devm_clk_get(dev: *mut Device, name: *const core::ffi::c_char) -> *mut Clk;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut PlatformDevice, index: u32, res: *mut *mut Resource) -> *mut core::ffi::c_void;
    fn resource_size(res: *mut Resource) -> u64;
    fn devm_regmap_init_mmio(dev: *mut Device, base: *mut core::ffi::c_void, config: *mut RegmapConfig) -> *mut Regmap;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_devfreq_event_add_edev(dev: *mut Device, desc: *mut DevfreqEventDesc) -> *mut DevfreqEventDev;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut core::ffi::c_void;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
}

unsafe fn exynos_nocp_set_event(edev: *mut DevfreqEventDev) -> i32 {
    let nocp = devfreq_event_get_drvdata(edev) as *mut ExynosNocp;
    let mut ret: i32;
    ret = regmap_update_bits((*nocp).regmap, NOCP_MAIN_CTL, NOCP_MAIN_CTL_STATEN_MASK, 0);
    if ret < 0 { return ret; }
    ret = regmap_write((*nocp).regmap, NOCP_STAT_PERIOD, 0x0); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_COUNTERS_0_SRC, NOCP_CNT_SRC_INTEVENT_MASK, NOCP_CNT_SRC_INTEVENT_BYTE_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_COUNTERS_1_SRC, NOCP_CNT_SRC_INTEVENT_MASK, NOCP_CNT_SRC_INTEVENT_CHAIN_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_COUNTERS_2_SRC, NOCP_CNT_SRC_INTEVENT_MASK, NOCP_CNT_SRC_INTEVENT_CYCLE_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_COUNTERS_3_SRC, NOCP_CNT_SRC_INTEVENT_MASK, NOCP_CNT_SRC_INTEVENT_CHAIN_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_write((*nocp).regmap, NOCP_STAT_ALARM_MIN, 0x0); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_write((*nocp).regmap, NOCP_STAT_ALARM_MAX, 0x0); if ret < 0 { return reset_nocp(nocp, ret); }
    for reg in [NOCP_COUNTERS_0_ALARM_MODE, NOCP_COUNTERS_1_ALARM_MODE, NOCP_COUNTERS_2_ALARM_MODE, NOCP_COUNTERS_3_ALARM_MODE] {
        ret = regmap_update_bits((*nocp).regmap, reg, NOCP_CNT_ALARM_MODE_MASK, NOCP_CNT_ALARM_MODE_MIN_MAX_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    }
    ret = regmap_update_bits((*nocp).regmap, NOCP_MAIN_CTL, NOCP_MAIN_CTL_STATEN_MASK | NOCP_MAIN_CTL_ALARMEN_MASK, NOCP_MAIN_CTL_STATEN_MASK | NOCP_MAIN_CTL_ALARMEN_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_CFG_CTL, NOCP_CFG_CTL_GLOBALEN_MASK, NOCP_CFG_CTL_GLOBALEN_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret = regmap_update_bits((*nocp).regmap, NOCP_MAIN_CTL, NOCP_MAIN_CTL_STATEN_MASK, NOCP_MAIN_CTL_STATEN_MASK); if ret < 0 { return reset_nocp(nocp, ret); }
    ret
}

unsafe fn reset_nocp(nocp: *mut ExynosNocp, ret: i32) -> i32 {
    let _ = regmap_update_bits((*nocp).regmap, NOCP_MAIN_CTL, NOCP_MAIN_CTL_STATEN_MASK, 0);
    ret
}

unsafe fn exynos_nocp_get_event(edev: *mut DevfreqEventDev, edata: *mut DevfreqEventData) -> i32 {
    let nocp = devfreq_event_get_drvdata(edev) as *mut ExynosNocp;
    let mut counter = [0u32; 4];
    for (i, reg) in [NOCP_COUNTERS_0_VAL, NOCP_COUNTERS_1_VAL, NOCP_COUNTERS_2_VAL, NOCP_COUNTERS_3_VAL].iter().enumerate() {
        let ret = regmap_read((*nocp).regmap, *reg, &mut counter[i]); if ret < 0 { return ret; }
    }
    (*edata).load_count = ((counter[1] << 16) | counter[0]) as u64;
    (*edata).total_count = ((counter[3] << 16) | counter[2]) as u64;
    0
}

static EXYNOS_NOCP_OPS: DevfreqEventOps = DevfreqEventOps { set_event: Some(exynos_nocp_set_event), get_event: Some(exynos_nocp_get_event) };
static EXYNOS_NOCP_ID_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"samsung,exynos5420-nocp\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];
static mut EXYNOS_NOCP_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, val_bits: 32, reg_stride: 4, max_register: NOCP_COUNTERS_3_VAL };

unsafe fn exynos_nocp_parse_dt(pdev: *mut PlatformDevice, nocp: *mut ExynosNocp) -> i32 {
    let dev = (*nocp).dev;
    if (*dev).of_node.is_null() { return -22; }
    (*nocp).clk = devm_clk_get(dev, b"nocp\0".as_ptr() as *const _);
    let mut res: *mut Resource = core::ptr::null_mut();
    let base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if base.is_null() { return -1; }
    EXYNOS_NOCP_REGMAP_CONFIG.max_register = (resource_size(res) - 4) as u32;
    (*nocp).regmap = devm_regmap_init_mmio(dev, base, &mut EXYNOS_NOCP_REGMAP_CONFIG);
    if (*nocp).regmap.is_null() { return -1; }
    0
}

unsafe fn exynos_nocp_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let nocp = devm_kzalloc(dev, core::mem::size_of::<ExynosNocp>(), 0) as *mut ExynosNocp;
    if nocp.is_null() { return -12; }
    (*nocp).dev = dev;
    let mut ret = exynos_nocp_parse_dt(pdev, nocp); if ret < 0 { return ret; }
    (*nocp).desc.ops = &EXYNOS_NOCP_OPS;
    (*nocp).desc.driver_data = nocp as *mut _;
    (*nocp).desc.name = (*dev).of_node.as_ref().unwrap().full_name;
    (*nocp).edev = devm_devfreq_event_add_edev(dev, &mut (*nocp).desc);
    if (*nocp).edev.is_null() { return -1; }
    platform_set_drvdata(pdev, nocp as *mut _);
    ret = clk_prepare_enable((*nocp).clk); if ret != 0 { return ret; }
    0
}
unsafe fn exynos_nocp_remove(pdev: *mut PlatformDevice) {
    let nocp = platform_get_drvdata(pdev) as *mut ExynosNocp;
    clk_disable_unprepare((*nocp).clk);
}

static mut EXYNOS_NOCP_DRIVER: PlatformDriver = PlatformDriver { probe: Some(exynos_nocp_probe), remove: Some(exynos_nocp_remove) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
