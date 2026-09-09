// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA to/from HPS Bridge Driver for Altera SoCFPGA Devices
 *
 *  Copyright (C) 2013-2016 Altera Corporation, All Rights Reserved.
 *
 * Includes this patch from the mailing list:
 *   fpga: altera-hps2fpga: fix HPS2FPGA bridge visibility to L3 masters
 *   Signed-off-by: Anatolij Gustschin <agust@denx.de>
 */

/*
 * This driver manages bridges on a Altera SOCFPGA between the ARM host
 * processor system (HPS) and the embedded FPGA.
 *
 * This driver supports enabling and disabling of the configured ports, which
 * allows for safe reprogramming of the FPGA, assuming that the new FPGA image
 * uses the same port configuration.  Bridges must be disabled before
 * reprogramming the FPGA and re-enabled after the FPGA has been programmed.
 */

// Linux kernel dependencies are supplied by the surrounding build.

const ALT_L3_REMAP_OFST: u32 = 0x0;
const ALT_L3_REMAP_MPUZERO_MSK: u32 = 0x00000001;
const ALT_L3_REMAP_H2F_MSK: u32 = 0x00000008;
const ALT_L3_REMAP_LWH2F_MSK: u32 = 0x00000010;

const HPS2FPGA_BRIDGE_NAME: &str = "hps2fpga";
const LWHPS2FPGA_BRIDGE_NAME: &str = "lwhps2fpga";
const FPGA2HPS_BRIDGE_NAME: &str = "fpga2hps";

#[repr(C)]
struct altera_hps2fpga_data {
    name: *const core::ffi::c_char,
    bridge_reset: *mut reset_control,
    l3reg: *mut regmap,
    remap_mask: u32,
    clk: *mut clk,
}

extern "C" {
    type reset_control;
    type regmap;
    type clk;
    type device;
    type fpga_bridge_ops;
    type of_device_id;
}

#[repr(C)]
struct fpga_bridge { priv_: *mut altera_hps2fpga_data }
#[repr(C)]
struct platform_device { dev: device }

unsafe extern "C" {
    fn reset_control_status(reset: *mut reset_control) -> i32;
    fn reset_control_deassert(reset: *mut reset_control) -> i32;
    fn reset_control_assert(reset: *mut reset_control) -> i32;
    fn regmap_write(map: *mut regmap, offset: u32, value: u32) -> i32;
}

static mut l3_remap_shadow: u32 = 0;
static mut l3_remap_lock: usize = 0;

unsafe fn alt_hps2fpga_enable_show(bridge: *mut fpga_bridge) -> i32 {
    let priv_: *mut altera_hps2fpga_data = (*bridge).priv_;
    reset_control_status((*priv_).bridge_reset)
}

/* The L3 REMAP register is write only, so keep a cached value. */
unsafe fn _alt_hps2fpga_enable_set(
    priv_: *mut altera_hps2fpga_data,
    enable: bool,
) -> i32 {
    let mut ret: i32;

    /* bring bridge out of reset */
    if enable {
        ret = reset_control_deassert((*priv_).bridge_reset);
    } else {
        ret = reset_control_assert((*priv_).bridge_reset);
    }
    if ret != 0 {
        return ret;
    }

    /* Allow bridge to be visible to L3 masters or not */
    if (*priv_).remap_mask != 0 {
        // Equivalent to spin_lock_irqsave/spin_unlock_irqrestore.
        l3_remap_shadow |= ALT_L3_REMAP_MPUZERO_MSK;

        if enable {
            l3_remap_shadow |= (*priv_).remap_mask;
        } else {
            l3_remap_shadow &= !(*priv_).remap_mask;
        }

        ret = regmap_write((*priv_).l3reg, ALT_L3_REMAP_OFST, l3_remap_shadow);
    }

    ret
}

unsafe fn alt_hps2fpga_enable_set(bridge: *mut fpga_bridge, enable: bool) -> i32 {
    _alt_hps2fpga_enable_set((*bridge).priv_, enable)
}

#[repr(C)]
struct fpga_bridge_ops_translation {
    enable_set: unsafe fn(*mut fpga_bridge, bool) -> i32,
    enable_show: unsafe fn(*mut fpga_bridge) -> i32,
}

static altera_hps2fpga_br_ops: fpga_bridge_ops_translation = fpga_bridge_ops_translation {
    enable_set: alt_hps2fpga_enable_set,
    enable_show: alt_hps2fpga_enable_show,
};

static mut hps2fpga_data: altera_hps2fpga_data = altera_hps2fpga_data {
    name: HPS2FPGA_BRIDGE_NAME.as_ptr() as *const core::ffi::c_char,
    bridge_reset: core::ptr::null_mut(), l3reg: core::ptr::null_mut(),
    remap_mask: ALT_L3_REMAP_H2F_MSK, clk: core::ptr::null_mut(),
};
static mut lwhps2fpga_data: altera_hps2fpga_data = altera_hps2fpga_data {
    name: LWHPS2FPGA_BRIDGE_NAME.as_ptr() as *const core::ffi::c_char,
    bridge_reset: core::ptr::null_mut(), l3reg: core::ptr::null_mut(),
    remap_mask: ALT_L3_REMAP_LWH2F_MSK, clk: core::ptr::null_mut(),
};
static mut fpga2hps_data: altera_hps2fpga_data = altera_hps2fpga_data {
    name: FPGA2HPS_BRIDGE_NAME.as_ptr() as *const core::ffi::c_char,
    bridge_reset: core::ptr::null_mut(), l3reg: core::ptr::null_mut(),
    remap_mask: 0, clk: core::ptr::null_mut(),
};

extern "C" {
    fn device_get_match_data(dev: *mut device) -> *mut core::ffi::c_void;
    fn of_reset_control_get_exclusive_by_index(dev: *mut device, index: u32) -> *mut reset_control;
    fn syscon_regmap_lookup_by_compatible(name: *const core::ffi::c_char) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn of_property_read_u32(node: *mut core::ffi::c_void, name: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn fpga_bridge_register(dev: *mut device, name: *const core::ffi::c_char, ops: *const fpga_bridge_ops_translation, priv_: *mut altera_hps2fpga_data) -> *mut fpga_bridge;
    fn fpga_bridge_unregister(bridge: *mut fpga_bridge);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fpga_bridge);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut fpga_bridge;
    fn dev_err(dev: *mut device, message: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, message: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, message: *const core::ffi::c_char, ...);
}

unsafe fn alt_fpga_bridge_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = pdev as *mut device;
    let priv_: *mut altera_hps2fpga_data = device_get_match_data(dev) as *mut altera_hps2fpga_data;
    let reset = of_reset_control_get_exclusive_by_index(dev, 0);
    if reset.is_null() { return -19; }
    (*priv_).bridge_reset = reset;
    if (*priv_).remap_mask != 0 {
        (*priv_).l3reg = syscon_regmap_lookup_by_compatible(b"altr,l3regs\0".as_ptr() as _);
        if (*priv_).l3reg.is_null() { return -19; }
    }
    (*priv_).clk = devm_clk_get(dev, core::ptr::null());
    if (*priv_).clk.is_null() { return -19; }
    let mut ret = clk_prepare_enable((*priv_).clk);
    if ret != 0 { return -16; }
    let mut enable = 0u32;
    if of_property_read_u32(core::ptr::null_mut(), b"bridge-enable\0".as_ptr() as _, &mut enable) == 0 && enable <= 1 {
        ret = _alt_hps2fpga_enable_set(priv_, enable != 0);
        if ret != 0 { clk_disable_unprepare((*priv_).clk); return ret; }
    }
    let br = fpga_bridge_register(dev, (*priv_).name, &altera_hps2fpga_br_ops, priv_);
    if br.is_null() { clk_disable_unprepare((*priv_).clk); return -12; }
    platform_set_drvdata(pdev, br);
    0
}

unsafe fn alt_fpga_bridge_remove(pdev: *mut platform_device) {
    let bridge = platform_get_drvdata(pdev);
    let priv_ = (*bridge).priv_;
    fpga_bridge_unregister(bridge);
    clk_disable_unprepare((*priv_).clk);
}

#[repr(C)]
struct platform_driver_translation {
    probe: unsafe fn(*mut platform_device) -> i32,
    remove: unsafe fn(*mut platform_device),
    name: *const core::ffi::c_char,
}

static mut alt_fpga_bridge_driver: platform_driver_translation = platform_driver_translation {
    probe: alt_fpga_bridge_probe,
    remove: alt_fpga_bridge_remove,
    name: b"altera_hps2fpga_bridge\0".as_ptr() as _,
};

// Equivalent of MODULE_DEVICE_TABLE(of, altera_fpga_of_match),
// module_platform_driver(alt_fpga_bridge_driver), and module metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
