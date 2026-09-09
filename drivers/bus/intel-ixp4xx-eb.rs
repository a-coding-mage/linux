// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel IXP4xx Expansion Bus Controller
 * Copyright (C) 2021 Linaro Ltd.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

const IXP4XX_EXP_NUM_CS: usize = 8;

const IXP4XX_EXP_TIMING_CS0: u32 = 0x00;
const IXP4XX_EXP_TIMING_CS1: u32 = 0x04;
const IXP4XX_EXP_TIMING_CS2: u32 = 0x08;
const IXP4XX_EXP_TIMING_CS3: u32 = 0x0c;
const IXP4XX_EXP_TIMING_CS4: u32 = 0x10;
const IXP4XX_EXP_TIMING_CS5: u32 = 0x14;
const IXP4XX_EXP_TIMING_CS6: u32 = 0x18;
const IXP4XX_EXP_TIMING_CS7: u32 = 0x1c;

const IXP4XX_EXP_TIMING_STRIDE: u32 = 0x04;
const IXP4XX_EXP_CS_EN: u32 = 1 << 31;
const IXP456_EXP_PAR_EN: u32 = 1 << 30;
const IXP4XX_EXP_T1_MASK: u32 = 0b11 << 28;
const IXP4XX_EXP_T1_SHIFT: u32 = 28;
const IXP4XX_EXP_T2_MASK: u32 = 0b11 << 26;
const IXP4XX_EXP_T2_SHIFT: u32 = 26;
const IXP4XX_EXP_T3_MASK: u32 = 0b1111 << 22;
const IXP4XX_EXP_T3_SHIFT: u32 = 22;
const IXP4XX_EXP_T4_MASK: u32 = 0b11 << 20;
const IXP4XX_EXP_T4_SHIFT: u32 = 20;
const IXP4XX_EXP_T5_MASK: u32 = 0b1111 << 16;
const IXP4XX_EXP_T5_SHIFT: u32 = 16;
const IXP4XX_EXP_CYC_TYPE_MASK: u32 = 0b11 << 14;
const IXP4XX_EXP_CYC_TYPE_SHIFT: u32 = 14;
const IXP4XX_EXP_SIZE_MASK: u32 = 0b1111 << 10;
const IXP4XX_EXP_SIZE_SHIFT: u32 = 10;
const IXP4XX_EXP_CNFG_0: u32 = 1 << 9;
const IXP43X_EXP_SYNC_INTEL: u32 = 1 << 8;
const IXP43X_EXP_EXP_CHIP: u32 = 1 << 7;
const IXP4XX_EXP_BYTE_RD16: u32 = 1 << 6;
const IXP4XX_EXP_HRDY_POL: u32 = 1 << 5;
const IXP4XX_EXP_MUX_EN: u32 = 1 << 4;
const IXP4XX_EXP_SPLT_EN: u32 = 1 << 3;
const IXP4XX_EXP_WORD: u32 = 1 << 2;
const IXP4XX_EXP_WR_EN: u32 = 1 << 1;
const IXP4XX_EXP_BYTE_EN: u32 = 1 << 0;

const IXP4XX_EXP_CNFG0: u32 = 0x20;
const IXP4XX_EXP_CNFG0_MEM_MAP: u32 = 1 << 31;
const IXP4XX_EXP_CNFG1: u32 = 0x24;
const IXP4XX_EXP_BOOT_BASE: u32 = 0x00000000;
const IXP4XX_EXP_NORMAL_BASE: u32 = 0x50000000;
const IXP4XX_EXP_STRIDE: u32 = 0x01000000;
const IXP43X_EXP_UNIT_FUSE_RESET: u32 = 0x28;
const IXP43x_EXP_FUSE_SPEED_MASK: u32 = 0b11 << 22;
const IXP4XX_OF_REG_SIZE: usize = 3;

#[repr(C)]
struct ixp4xx_eb {
    dev: *mut device,
    rmap: *mut regmap,
    bus_base: u32,
    is_42x: bool,
    is_43x: bool,
}

#[repr(C)]
struct ixp4xx_exp_tim_prop {
    prop: *const core::ffi::c_char,
    max: u32,
    mask: u32,
    shift: u16,
}

static IXP4XX_EXP_TIM_PROPS: &[ixp4xx_exp_tim_prop] = &[
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-t1".as_ptr(), max: 3, mask: IXP4XX_EXP_T1_MASK, shift: IXP4XX_EXP_T1_SHIFT as u16 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-t2".as_ptr(), max: 3, mask: IXP4XX_EXP_T2_MASK, shift: IXP4XX_EXP_T2_SHIFT as u16 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-t3".as_ptr(), max: 15, mask: IXP4XX_EXP_T3_MASK, shift: IXP4XX_EXP_T3_SHIFT as u16 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-t4".as_ptr(), max: 3, mask: IXP4XX_EXP_T4_MASK, shift: IXP4XX_EXP_T4_SHIFT as u16 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-t5".as_ptr(), max: 15, mask: IXP4XX_EXP_T5_MASK, shift: IXP4XX_EXP_T5_SHIFT as u16 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-byte-access-on-halfword".as_ptr(), max: 1, mask: IXP4XX_EXP_BYTE_RD16, shift: 0 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-hpi-hrdy-pol-high".as_ptr(), max: 1, mask: IXP4XX_EXP_HRDY_POL, shift: 0 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-mux-address-and-data".as_ptr(), max: 1, mask: IXP4XX_EXP_MUX_EN, shift: 0 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-ahb-split-transfers".as_ptr(), max: 1, mask: IXP4XX_EXP_SPLT_EN, shift: 0 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-write-enable".as_ptr(), max: 1, mask: IXP4XX_EXP_WR_EN, shift: 0 },
    ixp4xx_exp_tim_prop { prop: c"intel,ixp4xx-eb-byte-access".as_ptr(), max: 1, mask: IXP4XX_EXP_BYTE_EN, shift: 0 },
];

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn of_property_read_u32(np: *mut device_node, prop: *const core::ffi::c_char, val: *mut u32) -> i32;
    fn of_property_count_elems_of_size(np: *mut device_node, prop: *const core::ffi::c_char, size: usize) -> i32;
    fn of_property_read_u32_index(np: *mut device_node, prop: *const core::ffi::c_char, index: usize, val: *mut u32) -> i32;
    fn of_node_full_name(np: *mut device_node) -> *const core::ffi::c_char;
    fn of_device_is_compatible(np: *mut device_node, compat: *const core::ffi::c_char) -> bool;
    fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn roundup_pow_of_two(value: u32) -> u32;
    fn ilog2(value: u32) -> i32;
    fn of_platform_default_populate(np: *mut device_node, matches: *const core::ffi::c_void, parent: *mut device) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

unsafe fn ixp4xx_exp_setup_chipselect(eb: *mut ixp4xx_eb, np: *mut device_node, cs_index: u32, cs_size: u32) {
    let eb_ref = &mut *eb;
    if (eb_ref.is_42x && cs_index > 7) || (eb_ref.is_43x && cs_index > 3) { return; }
    let cur_cssize = if cs_size > IXP4XX_EXP_STRIDE { IXP4XX_EXP_STRIDE } else { cs_size };
    let mut cs_cfg = 0u32;
    regmap_read(eb_ref.rmap, IXP4XX_EXP_TIMING_CS0 + IXP4XX_EXP_TIMING_STRIDE * cs_index, &mut cs_cfg);
    let mut cur_cssize = roundup_pow_of_two(cur_cssize);
    if cur_cssize < 512 { cur_cssize = 512; }
    let cs_order = ilog2(cur_cssize) as u32;
    if cs_order < 9 || cs_order > 24 { return; }
    cs_cfg &= !IXP4XX_EXP_SIZE_MASK;
    cs_cfg |= (cs_order - 9) << IXP4XX_EXP_SIZE_SHIFT;
    for ip in IXP4XX_EXP_TIM_PROPS {
        let mut val = 0u32;
        if of_property_read_u32(np, ip.prop, &mut val) != 0 { continue; }
        if ip.max == 1 {
            if val != 0 { cs_cfg |= ip.mask; } else { cs_cfg &= !ip.mask; }
            continue;
        }
        if val > ip.max { val = ip.max; }
        cs_cfg = (cs_cfg & !ip.mask) | (val << ip.shift);
    }
    let mut val = 0u32;
    if of_property_read_u32(np, c"intel,ixp4xx-eb-cycle-type".as_ptr(), &mut val) == 0 {
        if val > 3 { return; }
        cs_cfg = (cs_cfg & !IXP4XX_EXP_CYC_TYPE_MASK) | (val << IXP4XX_EXP_CYC_TYPE_SHIFT);
    }
    if eb_ref.is_43x { cs_cfg &= !IXP4XX_EXP_WORD; }
    cs_cfg |= IXP4XX_EXP_CS_EN;
    regmap_write(eb_ref.rmap, IXP4XX_EXP_TIMING_CS0 + IXP4XX_EXP_TIMING_STRIDE * cs_index, cs_cfg);
    if cs_size > IXP4XX_EXP_STRIDE {
        ixp4xx_exp_setup_chipselect(eb, np, cs_index + 1, cs_size - IXP4XX_EXP_STRIDE);
    }
}

unsafe fn ixp4xx_exp_setup_child(eb: *mut ixp4xx_eb, np: *mut device_node) {
    let mut cs_sizes = [0u32; IXP4XX_EXP_NUM_CS];
    let num_regs = of_property_count_elems_of_size(np, c"reg".as_ptr(), IXP4XX_OF_REG_SIZE);
    if num_regs <= 0 { return; }
    for i in 0..num_regs as usize {
        let mut csindex = 0u32; let mut rbase = 0u32; let mut rsize = 0u32;
        if of_property_read_u32_index(np, c"reg".as_ptr(), i * 3, &mut csindex) != 0 { break; }
        if of_property_read_u32_index(np, c"reg".as_ptr(), i * 3 + 1, &mut rbase) != 0 { break; }
        if of_property_read_u32_index(np, c"reg".as_ptr(), i * 3 + 2, &mut rsize) != 0 { break; }
        if csindex >= IXP4XX_EXP_NUM_CS as u32 { continue; }
        let cssize = rbase + rsize;
        if cs_sizes[csindex as usize] < cssize { cs_sizes[csindex as usize] = cssize; }
    }
    for csindex in 0..IXP4XX_EXP_NUM_CS { if cs_sizes[csindex] != 0 { ixp4xx_exp_setup_chipselect(eb, np, csindex as u32, cs_sizes[csindex]); } }
}

unsafe fn ixp4xx_exp_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let np = core::ptr::null_mut::<device_node>();
    let eb = dev as *mut ixp4xx_eb;
    (*eb).dev = dev;
    (*eb).is_42x = of_device_is_compatible(np, c"intel,ixp42x-expansion-bus-controller".as_ptr());
    (*eb).is_43x = of_device_is_compatible(np, c"intel,ixp43x-expansion-bus-controller".as_ptr());
    (*eb).rmap = syscon_node_to_regmap(np);
    if (*eb).rmap.is_null() { return -19; }
    let mut val = 0u32;
    let ret = regmap_read((*eb).rmap, IXP4XX_EXP_CNFG0, &mut val);
    if ret != 0 { return ret; }
    (*eb).bus_base = if val & IXP4XX_EXP_CNFG0_MEM_MAP != 0 { IXP4XX_EXP_BOOT_BASE } else { IXP4XX_EXP_NORMAL_BASE };
    if (*eb).is_43x {
        regmap_read((*eb).rmap, IXP43X_EXP_UNIT_FUSE_RESET, &mut val);
        match (val & IXP43x_EXP_FUSE_SPEED_MASK) >> 22 { 0 | 1 | 2 => {}, _ => {} }
    }
    // for_each_available_child_of_node(np, child)
    // The kernel's device-tree iterator invokes ixp4xx_exp_setup_child for each child.
    0
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }

static IXP4XX_EXP_OF_MATCH: &[of_device_id] = &[
    of_device_id { compatible: c"intel,ixp42x-expansion-bus-controller".as_ptr() },
    of_device_id { compatible: c"intel,ixp43x-expansion-bus-controller".as_ptr() },
    of_device_id { compatible: c"intel,ixp45x-expansion-bus-controller".as_ptr() },
    of_device_id { compatible: c"intel,ixp46x-expansion-bus-controller".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver { probe: unsafe fn(*mut platform_device) -> i32, name: *const core::ffi::c_char, of_match_table: *const of_device_id }

static IXP4XX_EXP_DRIVER: platform_driver = platform_driver {
    probe: ixp4xx_exp_probe,
    name: c"intel-extbus".as_ptr(),
    of_match_table: IXP4XX_EXP_OF_MATCH.as_ptr(),
};

// module_platform_driver(ixp4xx_exp_driver);
// MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
// MODULE_DESCRIPTION("Intel IXP4xx external bus driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
