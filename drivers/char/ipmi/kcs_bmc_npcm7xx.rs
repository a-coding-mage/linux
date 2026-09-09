// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, Nuvoton Corporation.
 * Copyright (c) 2018, Intel Corporation.
 */

// pr_fmt(fmt) = "nuvoton-kcs-bmc: " fmt

// Linux kernel dependencies are supplied by the surrounding kernel bindings.

use core::ffi::c_void;

const DEVICE_NAME: &[u8] = b"npcm-kcs-bmc\0";
const KCS_CHANNEL_MAX: usize = 3;

const KCS1ST: u32 = 0x0C;
const KCS2ST: u32 = 0x1E;
const KCS3ST: u32 = 0x30;
const KCS1DO: u32 = 0x0E;
const KCS2DO: u32 = 0x20;
const KCS3DO: u32 = 0x32;
const KCS1DI: u32 = 0x10;
const KCS2DI: u32 = 0x22;
const KCS3DI: u32 = 0x34;
const KCS1CTL: u32 = 0x18;
const KCS2CTL: u32 = 0x2A;
const KCS3CTL: u32 = 0x3C;
const KCS_CTL_IBFIE: u8 = 1 << 0;
const KCS_CTL_OBEIE: u8 = 1 << 1;
const KCS1IE: u32 = 0x1C;
const KCS2IE: u32 = 0x2E;
const KCS3IE: u32 = 0x40;
const KCS_IE_IRQE: u8 = 1 << 0;
const KCS_IE_HIRQE: u8 = 1 << 3;

#[repr(C)]
pub struct kcs_bmc_device {
    pub dev: *mut device,
    pub channel: u32,
    pub ioreg: kcs_bmc_io,
    pub ops: *const kcs_bmc_device_ops,
}

#[repr(C)]
pub struct kcs_bmc_io { pub idr: u32, pub odr: u32, pub str: u32 }

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device }

pub type irqreturn_t = i32;
pub type irq_handler_t = unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t;
pub const IRQF_SHARED: u32 = 0x00000080;
pub const KCS_BMC_EVENT_TYPE_IBF: u8 = 1 << 0;
pub const KCS_BMC_EVENT_TYPE_OBE: u8 = 1 << 1;

#[repr(C)]
pub struct npcm7xx_kcs_reg { pub sts: u32, pub dob: u32, pub dib: u32, pub ctl: u32, pub ie: u32 }

#[repr(C)]
pub struct npcm7xx_kcs_bmc {
    pub kcs_bmc: kcs_bmc_device,
    pub map: *mut regmap,
    pub reg: *const npcm7xx_kcs_reg,
}

#[repr(C)]
pub struct kcs_bmc_device_ops {
    pub irq_mask_update: Option<unsafe extern "C" fn(*mut kcs_bmc_device, u8, u8)>,
    pub io_inputb: Option<unsafe extern "C" fn(*mut kcs_bmc_device, u32) -> u8>,
    pub io_outputb: Option<unsafe extern "C" fn(*mut kcs_bmc_device, u32, u8)>,
    pub io_updateb: Option<unsafe extern "C" fn(*mut kcs_bmc_device, u32, u8, u8)>,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, data: u8) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u8, data: u8) -> i32;
    fn kcs_bmc_handle_event(kcs_bmc: *mut kcs_bmc_device) -> irqreturn_t;
    fn platform_get_irq(pdev: *mut platform_device, index: u32) -> i32;
    fn devm_request_irq(dev: *mut device, irq: i32, handler: irq_handler_t, flags: u32, name: *const u8, arg: *mut kcs_bmc_device) -> i32;
    fn of_property_read_u32(node: *mut device_node, name: *const u8, val: *mut u32) -> i32;
    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn kcs_bmc_add_device(kcs_bmc: *mut kcs_bmc_device) -> i32;
    fn kcs_bmc_remove_device(kcs_bmc: *mut kcs_bmc_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const u8;
}

static NPCM7XX_KCS_REG_TBL: [npcm7xx_kcs_reg; KCS_CHANNEL_MAX] = [
    npcm7xx_kcs_reg { sts: KCS1ST, dob: KCS1DO, dib: KCS1DI, ctl: KCS1CTL, ie: KCS1IE },
    npcm7xx_kcs_reg { sts: KCS2ST, dob: KCS2DO, dib: KCS2DI, ctl: KCS2CTL, ie: KCS2IE },
    npcm7xx_kcs_reg { sts: KCS3ST, dob: KCS3DO, dib: KCS3DI, ctl: KCS3CTL, ie: KCS3IE },
];

#[inline]
unsafe fn to_npcm7xx_kcs_bmc(kcs_bmc: *mut kcs_bmc_device) -> *mut npcm7xx_kcs_bmc {
    kcs_bmc as *mut npcm7xx_kcs_bmc
}

unsafe extern "C" fn npcm7xx_kcs_inb(kcs_bmc: *mut kcs_bmc_device, reg: u32) -> u8 {
    let priv_ = &mut *to_npcm7xx_kcs_bmc(kcs_bmc);
    let mut val = 0u32;
    let rc = regmap_read(priv_.map, reg, &mut val);
    if rc != 0 { /* WARN(rc != 0, "regmap_read() failed: %d\\n", rc) */ }
    if rc == 0 { val as u8 } else { 0 }
}

unsafe extern "C" fn npcm7xx_kcs_outb(kcs_bmc: *mut kcs_bmc_device, reg: u32, data: u8) {
    let priv_ = &mut *to_npcm7xx_kcs_bmc(kcs_bmc);
    let rc = regmap_write(priv_.map, reg, data);
    if rc != 0 { /* WARN(rc != 0, "regmap_write() failed: %d\\n", rc) */ }
}

unsafe extern "C" fn npcm7xx_kcs_updateb(kcs_bmc: *mut kcs_bmc_device, reg: u32, mask: u8, data: u8) {
    let priv_ = &mut *to_npcm7xx_kcs_bmc(kcs_bmc);
    let rc = regmap_update_bits(priv_.map, reg, mask, data);
    if rc != 0 { /* WARN(rc != 0, "regmap_update_bits() failed: %d\\n", rc) */ }
}

unsafe extern "C" fn npcm7xx_kcs_enable_channel(kcs_bmc: *mut kcs_bmc_device, enable: bool) {
    let priv_ = &mut *to_npcm7xx_kcs_bmc(kcs_bmc);
    let data = if enable { KCS_IE_IRQE | KCS_IE_HIRQE } else { 0 };
    regmap_update_bits(priv_.map, (*priv_.reg).ie, KCS_IE_IRQE | KCS_IE_HIRQE, data);
}

unsafe extern "C" fn npcm7xx_kcs_irq_mask_update(kcs_bmc: *mut kcs_bmc_device, mask: u8, state: u8) {
    let priv_ = &mut *to_npcm7xx_kcs_bmc(kcs_bmc);
    if mask & KCS_BMC_EVENT_TYPE_OBE != 0 {
        regmap_update_bits(priv_.map, (*priv_.reg).ctl, KCS_CTL_OBEIE,
            if state & KCS_BMC_EVENT_TYPE_OBE != 0 { KCS_CTL_OBEIE } else { 0 });
    }
    if mask & KCS_BMC_EVENT_TYPE_IBF != 0 {
        regmap_update_bits(priv_.map, (*priv_.reg).ctl, KCS_CTL_IBFIE,
            if state & KCS_BMC_EVENT_TYPE_IBF != 0 { KCS_CTL_IBFIE } else { 0 });
    }
}

unsafe extern "C" fn npcm7xx_kcs_irq(irq: i32, arg: *mut c_void) -> irqreturn_t {
    let _ = irq;
    kcs_bmc_handle_event(arg as *mut kcs_bmc_device)
}

unsafe fn npcm7xx_kcs_config_irq(kcs_bmc: *mut kcs_bmc_device, pdev: *mut platform_device) -> i32 {
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    devm_request_irq(&mut (*pdev).dev, irq, npcm7xx_kcs_irq, IRQF_SHARED,
        dev_name(&mut (*pdev).dev), kcs_bmc)
}

static NPCM7XX_KCS_OPS: kcs_bmc_device_ops = kcs_bmc_device_ops {
    irq_mask_update: Some(npcm7xx_kcs_irq_mask_update),
    io_inputb: Some(npcm7xx_kcs_inb), io_outputb: Some(npcm7xx_kcs_outb),
    io_updateb: Some(npcm7xx_kcs_updateb),
};

unsafe extern "C" fn npcm7xx_kcs_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut chan = 0u32;
    let rc = of_property_read_u32(core::ptr::null_mut(), b"kcs_chan\0".as_ptr(), &mut chan);
    if rc != 0 || chan == 0 || chan > KCS_CHANNEL_MAX as u32 { return -19; }
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<npcm7xx_kcs_bmc>(), 0) as *mut npcm7xx_kcs_bmc;
    if priv_.is_null() { return -12; }
    (*priv_).map = syscon_node_to_regmap(core::ptr::null_mut());
    if (*priv_).map.is_null() { return -19; }
    (*priv_).reg = &NPCM7XX_KCS_REG_TBL[(chan - 1) as usize];
    let kcs_bmc = &mut (*priv_).kcs_bmc;
    kcs_bmc.dev = dev; kcs_bmc.channel = chan;
    kcs_bmc.ioreg.idr = (*(*priv_).reg).dib; kcs_bmc.ioreg.odr = (*(*priv_).reg).dob;
    kcs_bmc.ioreg.str = (*(*priv_).reg).sts; kcs_bmc.ops = &NPCM7XX_KCS_OPS;
    platform_set_drvdata(pdev, priv_ as *mut c_void);
    let rc = npcm7xx_kcs_config_irq(kcs_bmc, pdev); if rc != 0 { return rc; }
    npcm7xx_kcs_irq_mask_update(kcs_bmc, KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE, 0);
    npcm7xx_kcs_enable_channel(kcs_bmc, true);
    let rc = kcs_bmc_add_device(kcs_bmc); if rc != 0 { return rc; }
    0
}

unsafe extern "C" fn npcm7xx_kcs_remove(pdev: *mut platform_device) {
    let priv_ = platform_get_drvdata(pdev) as *mut npcm7xx_kcs_bmc;
    let kcs_bmc = &mut (*priv_).kcs_bmc;
    kcs_bmc_remove_device(kcs_bmc);
    npcm7xx_kcs_enable_channel(kcs_bmc, false);
    npcm7xx_kcs_irq_mask_update(kcs_bmc, KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE, 0);
}

// Device-table and platform-driver registration are supplied by the kernel integration layer.
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Avi Fishman <avifishman70@gmail.com>");
// MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>");
// MODULE_DESCRIPTION("NPCM7xx device interface to the KCS BMC device");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
