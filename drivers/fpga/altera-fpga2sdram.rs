// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA to SDRAM Bridge Driver for Altera SoCFPGA Devices
 *
 *  Copyright (C) 2013-2016 Altera Corporation, All Rights Reserved.
 */

/*
 * This driver manages a bridge between an FPGA and the SDRAM used by the ARM
 * host processor system (HPS).
 *
 * The bridge contains 4 read ports, 4 write ports, and 6 command ports.
 * Reconfiguring these ports requires that no SDRAM transactions occur during
 * reconfiguration.  The code reconfiguring the ports cannot run out of SDRAM
 * nor can the FPGA access the SDRAM during reconfiguration.  This driver does
 * not support reconfiguring the ports.  The ports are configured by code
 * running out of on chip ram before Linux is started and the configuration
 * is passed in a handoff register in the system manager.
 *
 * This driver supports enabling and disabling of the configured ports, which
 * allows for safe reprogramming of the FPGA, assuming that the new FPGA image
 * uses the same port configuration.  Bridges must be disabled before
 * reprogramming the FPGA and re-enabled after the FPGA has been programmed.
 */

// External Linux kernel types, functions, constants, and registration macros
// supplied by the surrounding kernel environment are intentionally referenced
// but not implemented here.

const ALT_SDR_CTL_FPGAPORTRST_OFST: usize = 0x80;
const ALT_SDR_CTL_FPGAPORTRST_PORTRSTN_MSK: i32 = 0x00003fff;
const ALT_SDR_CTL_FPGAPORTRST_RD_SHIFT: u32 = 0;
const ALT_SDR_CTL_FPGAPORTRST_WR_SHIFT: u32 = 4;
const ALT_SDR_CTL_FPGAPORTRST_CTRL_SHIFT: u32 = 8;

/*
 * From the Cyclone V HPS Memory Map document:
 *   These registers are used to store handoff information between the
 *   preloader and the OS. These 8 registers can be used to store any
 *   information. The contents of these registers have no impact on
 *   the state of the HPS hardware.
 */
const SYSMGR_ISWGRP_HANDOFF3: usize = 0x8C;

const F2S_BRIDGE_NAME: &str = "fpga2sdram";

#[repr(C)]
struct AltFpga2sdramData {
    dev: *mut Device,
    sdrctl: *mut Regmap,
    mask: i32,
}

// Opaque external kernel types.
#[repr(C)]
struct Device;
#[repr(C)]
struct Regmap;
#[repr(C)]
struct FpgaBridge {
    priv_: *mut AltFpga2sdramData,
}
#[repr(C)]
struct PlatformDevice {
    dev: Device,
}
#[repr(C)]
struct FpgaBridgeOps {
    enable_set: Option<unsafe extern "C" fn(*mut FpgaBridge, bool) -> i32>,
    enable_show: Option<unsafe extern "C" fn(*mut FpgaBridge) -> i32>,
}
#[repr(C)]
struct OfDeviceId {
    compatible: *const u8,
}
#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
}

extern "C" {
    fn regmap_read(map: *mut Regmap, reg: usize, val: *mut i32) -> i32;
    fn regmap_update_bits(map: *mut Regmap, reg: usize, mask: i32, val: i32) -> i32;
    fn syscon_regmap_lookup_by_compatible(compatible: *const u8) -> *mut Regmap;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut AltFpga2sdramData;
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut Device, fmt: *const u8, ...);
    fn dev_info(dev: *mut Device, fmt: *const u8, ...);
    fn fpga_bridge_register(
        dev: *mut Device,
        name: *const u8,
        ops: *const FpgaBridgeOps,
        priv_: *mut AltFpga2sdramData,
    ) -> *mut FpgaBridge;
    fn fpga_bridge_unregister(bridge: *mut FpgaBridge);
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut FpgaBridge);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut FpgaBridge;
    fn of_property_read_u32(node: *mut Device, property: *const u8, value: *mut u32) -> i32;
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

unsafe fn alt_fpga2sdram_enable_show(bridge: *mut FpgaBridge) -> i32 {
    let priv_: *mut AltFpga2sdramData = (*bridge).priv_;
    let mut value: i32 = 0;

    regmap_read((*priv_).sdrctl, ALT_SDR_CTL_FPGAPORTRST_OFST, &mut value);

    if (value & (*priv_).mask) == (*priv_).mask { 1 } else { 0 }
}

unsafe fn _alt_fpga2sdram_enable_set(
    priv_: *mut AltFpga2sdramData,
    enable: bool,
) -> i32 {
    regmap_update_bits(
        (*priv_).sdrctl,
        ALT_SDR_CTL_FPGAPORTRST_OFST,
        (*priv_).mask,
        if enable { (*priv_).mask } else { 0 },
    )
}

unsafe fn alt_fpga2sdram_enable_set(bridge: *mut FpgaBridge, enable: bool) -> i32 {
    _alt_fpga2sdram_enable_set((*bridge).priv_, enable)
}

static ALTERA_FPGA2SDRAM_BR_OPS: FpgaBridgeOps = FpgaBridgeOps {
    enable_set: Some(alt_fpga2sdram_enable_set),
    enable_show: Some(alt_fpga2sdram_enable_show),
};

static ALTERA_FPGA_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"altr,socfpga-fpga2sdram-bridge\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe fn alt_fpga_bridge_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev: *mut Device = &mut (*pdev).dev;
    let priv_: *mut AltFpga2sdramData;
    let br: *mut FpgaBridge;
    let mut enable: u32 = 0;
    let sysmgr: *mut Regmap;
    let mut ret: i32 = 0;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<AltFpga2sdramData>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).dev = dev;

    (*priv_).sdrctl = syscon_regmap_lookup_by_compatible(b"altr,sdr-ctl\0".as_ptr());
    if (*priv_).sdrctl.is_null() {
        dev_err(dev, b"regmap for altr,sdr-ctl lookup failed.\n\0".as_ptr());
        return -1;
    }

    sysmgr = syscon_regmap_lookup_by_compatible(b"altr,sys-mgr\0".as_ptr());
    if sysmgr.is_null() {
        dev_err(dev, b"regmap for altr,sys-mgr lookup failed.\n\0".as_ptr());
        return -1;
    }

    /* Get f2s bridge configuration saved in handoff register */
    regmap_read(sysmgr, SYSMGR_ISWGRP_HANDOFF3, &mut (*priv_).mask);

    br = fpga_bridge_register(dev, F2S_BRIDGE_NAME.as_ptr(), &ALTERA_FPGA2SDRAM_BR_OPS, priv_);
    if br.is_null() {
        return -1;
    }

    platform_set_drvdata(pdev, br);

    dev_info(dev, b"driver initialized with handoff %08x\n\0".as_ptr(), (*priv_).mask);

    if of_property_read_u32(dev, b"bridge-enable\0".as_ptr(), &mut enable) == 0 {
        if enable > 1 {
            dev_warn(dev, b"invalid bridge-enable %u > 1\n\0".as_ptr(), enable);
        } else {
            dev_info(
                dev,
                b"%s bridge\n\0".as_ptr(),
                if enable != 0 { b"enabling\0".as_ptr() } else { b"disabling\0".as_ptr() },
            );
            ret = _alt_fpga2sdram_enable_set(priv_, enable != 0);
            if ret != 0 {
                fpga_bridge_unregister(br);
                return ret;
            }
        }
    }

    ret
}

unsafe fn alt_fpga_bridge_remove(pdev: *mut PlatformDevice) {
    let br: *mut FpgaBridge = platform_get_drvdata(pdev);

    fpga_bridge_unregister(br);
}

static ALTERA_FPGA2SDRAM_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(alt_fpga_bridge_probe),
    remove: Some(alt_fpga_bridge_remove),
};

// MODULE_DEVICE_TABLE(of, altera_fpga_of_match);
// module_platform_driver(altera_fpga_driver);
// MODULE_DESCRIPTION("Altera SoCFPGA FPGA to SDRAM Bridge");
// MODULE_AUTHOR("Alan Tull <atull@opensource.altera.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
