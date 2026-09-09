// SPDX-License-Identifier: GPL-2.0-only
/*
 * Qualcomm External Bus Interface 2 (EBI2) driver
 * an older version of the Qualcomm Parallel Interface Controller (QPIC)
 *
 * Copyright (C) 2016 Linaro Ltd.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 *
 * See the device tree bindings for this block for more details on the
 * hardware.
 */

// C dependencies supplied by the kernel and other translation units.

const EBI2_CS0_ENABLE_MASK: u32 = (1 << 0) | (1 << 1);
const EBI2_CS1_ENABLE_MASK: u32 = (1 << 2) | (1 << 3);
const EBI2_CS2_ENABLE_MASK: u32 = 1 << 4;
const EBI2_CS3_ENABLE_MASK: u32 = 1 << 5;
const EBI2_CS4_ENABLE_MASK: u32 = (1 << 6) | (1 << 7);
const EBI2_CS5_ENABLE_MASK: u32 = (1 << 8) | (1 << 9);
const EBI2_CSN_MASK: u32 = (1 << 10) - 1;

const EBI2_XMEM_CFG: usize = 0x0000;
const EBI2_XMEM_CS0_SLOW_CFG: usize = 0x0008;
const EBI2_XMEM_CS1_SLOW_CFG: usize = 0x000c;
const EBI2_XMEM_CS2_SLOW_CFG: usize = 0x0010;
const EBI2_XMEM_CS3_SLOW_CFG: usize = 0x0014;
const EBI2_XMEM_CS4_SLOW_CFG: usize = 0x0018;
const EBI2_XMEM_CS5_SLOW_CFG: usize = 0x001c;

const EBI2_XMEM_RECOVERY_SHIFT: u32 = 28;
const EBI2_XMEM_WR_HOLD_SHIFT: u32 = 24;
const EBI2_XMEM_WR_DELTA_SHIFT: u32 = 16;
const EBI2_XMEM_RD_DELTA_SHIFT: u32 = 8;
const EBI2_XMEM_WR_WAIT_SHIFT: u32 = 4;
const EBI2_XMEM_RD_WAIT_SHIFT: u32 = 0;

const EBI2_XMEM_CS0_FAST_CFG: usize = 0x0028;
const EBI2_XMEM_CS1_FAST_CFG: usize = 0x002c;
const EBI2_XMEM_CS2_FAST_CFG: usize = 0x0030;
const EBI2_XMEM_CS3_FAST_CFG: usize = 0x0034;
const EBI2_XMEM_CS4_FAST_CFG: usize = 0x0038;
const EBI2_XMEM_CS5_FAST_CFG: usize = 0x003c;

const EBI2_XMEM_RD_HOLD_SHIFT: u32 = 24;
const EBI2_XMEM_ADV_OE_RECOVERY_SHIFT: u32 = 16;
const EBI2_XMEM_ADDR_HOLD_ENA_SHIFT: u32 = 5;

#[repr(C)]
struct cs_data {
    enable_mask: u32,
    slow_cfg: u16,
    fast_cfg: u16,
}

static cs_info: [cs_data; 6] = [
    cs_data { enable_mask: EBI2_CS0_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS0_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS0_FAST_CFG as u16 },
    cs_data { enable_mask: EBI2_CS1_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS1_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS1_FAST_CFG as u16 },
    cs_data { enable_mask: EBI2_CS2_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS2_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS2_FAST_CFG as u16 },
    cs_data { enable_mask: EBI2_CS3_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS3_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS3_FAST_CFG as u16 },
    cs_data { enable_mask: EBI2_CS4_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS4_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS4_FAST_CFG as u16 },
    cs_data { enable_mask: EBI2_CS5_ENABLE_MASK, slow_cfg: EBI2_XMEM_CS5_SLOW_CFG as u16, fast_cfg: EBI2_XMEM_CS5_FAST_CFG as u16 },
];

#[repr(C)]
struct ebi2_xmem_prop {
    prop: *const core::ffi::c_char,
    max: u32,
    slowreg: bool,
    shift: u16,
}

static xmem_props: [ebi2_xmem_prop; 9] = [
    ebi2_xmem_prop { prop: b"qcom,xmem-recovery-cycles\0".as_ptr() as *const _, max: 15, slowreg: true, shift: EBI2_XMEM_RECOVERY_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-write-hold-cycles\0".as_ptr() as *const _, max: 15, slowreg: true, shift: EBI2_XMEM_WR_HOLD_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-write-delta-cycles\0".as_ptr() as *const _, max: 255, slowreg: true, shift: EBI2_XMEM_WR_DELTA_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-read-delta-cycles\0".as_ptr() as *const _, max: 255, slowreg: true, shift: EBI2_XMEM_RD_DELTA_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-write-wait-cycles\0".as_ptr() as *const _, max: 15, slowreg: true, shift: EBI2_XMEM_WR_WAIT_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-read-wait-cycles\0".as_ptr() as *const _, max: 15, slowreg: true, shift: EBI2_XMEM_RD_WAIT_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-address-hold-enable\0".as_ptr() as *const _, max: 1, slowreg: false, shift: EBI2_XMEM_ADDR_HOLD_ENA_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-adv-to-oe-recovery-cycles\0".as_ptr() as *const _, max: 3, slowreg: false, shift: EBI2_XMEM_ADV_OE_RECOVERY_SHIFT as u16 },
    ebi2_xmem_prop { prop: b"qcom,xmem-read-hold-cycles\0".as_ptr() as *const _, max: 15, slowreg: false, shift: EBI2_XMEM_RD_HOLD_SHIFT as u16 },
];

// The following function bodies retain the C driver's logic; kernel symbols and types are external dependencies.
unsafe fn qcom_ebi2_setup_chipselect(np: *mut device_node, dev: *mut device, ebi2_base: *mut core::ffi::c_void, ebi2_xmem: *mut core::ffi::c_void, csindex: u32) {
    let csd = &cs_info[csindex as usize];
    let mut val = readl(ebi2_base);
    val |= csd.enable_mask;
    writel(val, ebi2_base);
    dev_dbg(dev, "enabled CS%u\n", csindex);
    let mut slowcfg: u32 = 0;
    let mut fastcfg: u32 = 0;
    for xp in xmem_props.iter() {
        let mut propval: u32 = 0;
        let ret = of_property_read_u32(np, xp.prop, &mut propval);
        if ret != 0 { dev_dbg(dev, "could not read %s for CS%d\n", xp.prop, csindex); continue; }
        if xp.max == 1 && propval != 0 {
            if xp.slowreg { slowcfg |= 1 << xp.shift; } else { fastcfg |= 1 << xp.shift; }
            dev_dbg(dev, "set %s flag\n", xp.prop);
            continue;
        }
        if propval > xp.max { dev_err(dev, "too high value for %s: %u, capped at %u\n", xp.prop, propval, xp.max); propval = xp.max; }
        if xp.slowreg { slowcfg |= propval << xp.shift; } else { fastcfg |= propval << xp.shift; }
        dev_dbg(dev, "set %s to %u\n", xp.prop, propval);
    }
    dev_info(dev, "CS%u: SLOW CFG 0x%08x, FAST CFG 0x%08x\n", csindex, slowcfg, fastcfg);
    if slowcfg != 0 { writel(slowcfg, ebi2_xmem.add(csd.slow_cfg as usize)); }
    if fastcfg != 0 { writel(fastcfg, ebi2_xmem.add(csd.fast_cfg as usize)); }
}

unsafe fn qcom_ebi2_probe(pdev: *mut platform_device) -> i32 {
    let np = (*(*pdev).dev.of_node);
    let dev = &mut (*pdev).dev as *mut device;
    let mut res: *mut resource;
    let mut ebi2_base: *mut core::ffi::c_void;
    let mut ebi2_xmem: *mut core::ffi::c_void;
    let ebi2xclk = devm_clk_get_enabled(dev, b"ebi2x\0".as_ptr() as *const _);
    if is_err(ebi2xclk) { return ptr_err(ebi2xclk); }
    let ebi2clk = devm_clk_get_enabled(dev, b"ebi2\0".as_ptr() as *const _);
    if is_err(ebi2clk) { return ptr_err(ebi2clk); }
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    ebi2_base = devm_ioremap_resource(dev, res);
    if is_err(ebi2_base) { return ptr_err(ebi2_base); }
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    ebi2_xmem = devm_ioremap_resource(dev, res);
    if is_err(ebi2_xmem) { return ptr_err(ebi2_xmem); }
    writel(0, ebi2_xmem.add(EBI2_XMEM_CFG));
    let mut val = readl(ebi2_base);
    val &= !EBI2_CSN_MASK;
    writel(val, ebi2_base);
    let mut have_children = false;
    for_each_available_child_of_node_scoped(np, child) {
        let mut csindex: u32 = 0;
        let ret = of_property_read_u32(child, b"reg\0".as_ptr() as *const _, &mut csindex);
        if ret != 0 { return ret; }
        if csindex > 5 {
            dev_err(dev, "invalid chipselect %u, we only support 0-5\n", csindex);
            continue;
        }
        qcom_ebi2_setup_chipselect(child, dev, ebi2_base, ebi2_xmem, csindex);
        have_children = true;
    }
    if have_children { return devm_of_platform_populate(dev); }
    0
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }
static qcom_ebi2_of_match: [of_device_id; 3] = [
    of_device_id { compatible: b"qcom,msm8660-ebi2\0".as_ptr() as *const _ },
    of_device_id { compatible: b"qcom,apq8060-ebi2\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

// Platform-driver registration and module metadata correspond to module_platform_driver,
// MODULE_AUTHOR, and MODULE_DESCRIPTION in the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
