// SPDX-License-Identifier: GPL-2.0-only
//
// rt715-sdca-sdw.c -- rt715 ALSA SoC audio driver
//
// Copyright(c) 2020 Realtek Semiconductor Corp.
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub paging_support: bool_,
    pub source_ports: u32,
    pub sink_ports: u32,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub sdw_version: c_uint,
    pub class_id: c_uint,
    pub unique_id: c_uint,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct rt715_sdca_priv {
    pub hw_init: bool_,
    pub first_hw_init: bool_,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool_,
    pub use_single_write: bool_,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

pub type sdw_slave_status = c_uint;

const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const REGCACHE_MAPLE: c_uint = 0;
const FUN_MIC_ARRAY: c_uint = 0;
const FUN_JACK_CODEC: c_uint = 0;
const RT715_SDCA_SMPU_TRIG_ST_EN: c_uint = 0;
const RT715_SDCA_SMPU_TRIG_ST_CTRL: c_uint = 0;
const RT715_SDCA_ST_EN: c_uint = 0;
const RT715_SDCA_ST_CTRL: c_uint = 0;
const CH_00: c_uint = 0;

const fn SDW_SDCA_CTL(function: c_uint, entity: c_uint, control: c_uint, channel: c_uint) -> c_uint {
    ((function & 0xff) << 24) | ((entity & 0xff) << 16) | ((control & 0xff) << 8) | (channel & 0xff)
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn SDW_SLAVE_ENTRY_EXT(
    mfg_id: c_uint,
    part_id: c_uint,
    sdw_version: c_uint,
    class_id: c_uint,
    unique_id: c_uint,
) -> sdw_device_id {
    sdw_device_id {
        mfg_id,
        part_id,
        sdw_version,
        class_id,
        unique_id,
    }
}

unsafe extern "C" {
    static rt715_reg_defaults_sdca: [reg_default; 0];
    static rt715_mbq_reg_defaults_sdca: [reg_default; 0];

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn hweight32(w: u32) -> c_int;
    fn devm_regmap_init_sdw_mbq(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn rt715_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn rt715_sdca_init(
        dev: *mut device,
        mbq_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool_);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn rt715_sdca_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x201a..=0x2027
        | 0x2029..=0x202a
        | 0x202d..=0x2034
        | 0x2200..=0x2204
        | 0x2206..=0x2212
        | 0x2230..=0x2239
        | 0x2f5b => true,
        x if x
            == SDW_SDCA_CTL(
                FUN_MIC_ARRAY,
                RT715_SDCA_SMPU_TRIG_ST_EN,
                RT715_SDCA_SMPU_TRIG_ST_CTRL,
                CH_00,
            ) =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn rt715_sdca_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x201b | 0x201c | 0x201d | 0x201f | 0x2021 | 0x2023 | 0x2230 => true,
        0x202d..=0x202f => true, /* BRA */
        0x2200..=0x2212 => true, /* i2c debug */
        0x2f07
        | 0x2f1b..=0x2f1e
        | 0x2f30..=0x2f34
        | 0x2f50..=0x2f51
        | 0x2f53..=0x2f59
        | 0x2f5c..=0x2f5f => true,
        x if x
            == SDW_SDCA_CTL(
                FUN_MIC_ARRAY,
                RT715_SDCA_SMPU_TRIG_ST_EN,
                RT715_SDCA_SMPU_TRIG_ST_CTRL,
                CH_00,
            ) =>
        {
            true /* VAD Searching status */
        }
        _ => false,
    }
}

unsafe extern "C" fn rt715_sdca_mbq_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x2000000 | 0x200002b | 0x2000036 | 0x2000037 | 0x2000039 | 0x2000044 | 0x6100000 => {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn rt715_sdca_mbq_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x2000000 => true,
        _ => false,
    }
}

static rt715_sdca_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt715_sdca_readable_register),
    volatile_reg: Some(rt715_sdca_volatile_register),
    max_register: 0x43ffffff,
    reg_defaults: unsafe { rt715_reg_defaults_sdca.as_ptr() },
    num_reg_defaults: ARRAY_SIZE(unsafe { &rt715_reg_defaults_sdca }),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

static rt715_sdca_mbq_regmap: regmap_config = regmap_config {
    name: b"sdw-mbq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt715_sdca_mbq_readable_register),
    volatile_reg: Some(rt715_sdca_mbq_volatile_register),
    max_register: 0x43ffffff,
    reg_defaults: unsafe { rt715_mbq_reg_defaults_sdca.as_ptr() },
    num_reg_defaults: ARRAY_SIZE(unsafe { &rt715_mbq_reg_defaults_sdca }),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt715_sdca_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt715 = dev_get_drvdata(&mut (*slave).dev) as *mut rt715_sdca_priv;

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt715).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt715_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt715_sdca_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x50; /* BITMAP: 01010000 */
    (*prop).sink_ports = 0x0; /* BITMAP:  00000000 */

    nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    dpn = (*prop).src_dpn_prop;
    i = 0;
    addr = (*prop).source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 200;

    0
}

static rt715_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt715_sdca_read_prop),
    update_status: Some(rt715_sdca_update_status),
};

unsafe extern "C" fn rt715_sdca_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let mut mbq_regmap: *mut regmap;
    let mut regmap: *mut regmap;

    /* Regmap Initialization */
    mbq_regmap = devm_regmap_init_sdw_mbq(slave, &rt715_sdca_mbq_regmap);
    if IS_ERR(mbq_regmap as *const c_void) {
        return PTR_ERR(mbq_regmap as *const c_void);
    }

    regmap = devm_regmap_init_sdw(slave, &rt715_sdca_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt715_sdca_init(&mut (*slave).dev, mbq_regmap, regmap, slave)
}

unsafe extern "C" fn rt715_sdca_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

static rt715_sdca_id: [sdw_device_id; 3] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x715, 0x3, 0x1, 0),
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x714, 0x3, 0x1, 0),
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        sdw_version: 0,
        class_id: 0,
        unique_id: 0,
    },
];
/* MODULE_DEVICE_TABLE(sdw, rt715_sdca_id); */

unsafe extern "C" fn rt715_dev_suspend(dev: *mut device) -> c_int {
    let rt715 = dev_get_drvdata(dev) as *mut rt715_sdca_priv;

    if !(*rt715).hw_init {
        return 0;
    }

    regcache_cache_only((*rt715).regmap, true);
    regcache_mark_dirty((*rt715).regmap);
    regcache_cache_only((*rt715).mbq_regmap, true);
    regcache_mark_dirty((*rt715).mbq_regmap);

    0
}

const RT715_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn rt715_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt715 = dev_get_drvdata(dev) as *mut rt715_sdca_priv;
    let mut ret: c_int;

    if !(*rt715).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT715_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt715).regmap, false);
    regcache_sync_region(
        (*rt715).regmap,
        SDW_SDCA_CTL(FUN_JACK_CODEC, RT715_SDCA_ST_EN, RT715_SDCA_ST_CTRL, CH_00),
        SDW_SDCA_CTL(
            FUN_MIC_ARRAY,
            RT715_SDCA_SMPU_TRIG_ST_EN,
            RT715_SDCA_SMPU_TRIG_ST_CTRL,
            CH_00,
        ),
    );
    regcache_cache_only((*rt715).mbq_regmap, false);
    regcache_sync_region((*rt715).mbq_regmap, 0x2000000, 0x61020ff);
    regcache_sync_region(
        (*rt715).mbq_regmap,
        SDW_SDCA_CTL(FUN_JACK_CODEC, RT715_SDCA_ST_EN, RT715_SDCA_ST_CTRL, CH_00),
        SDW_SDCA_CTL(
            FUN_MIC_ARRAY,
            RT715_SDCA_SMPU_TRIG_ST_EN,
            RT715_SDCA_SMPU_TRIG_ST_CTRL,
            CH_00,
        ),
    );

    0
}

/* SYSTEM_SLEEP_PM_OPS(rt715_dev_suspend, rt715_dev_resume)
 * RUNTIME_PM_OPS(rt715_dev_suspend, rt715_dev_resume, NULL)
 */
static rt715_pm: dev_pm_ops = dev_pm_ops { _private: [] };

static rt715_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt715-sdca\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&rt715_pm) },
    },
    probe: Some(rt715_sdca_sdw_probe),
    remove: Some(rt715_sdca_sdw_remove),
    ops: &rt715_sdca_slave_ops,
    id_table: rt715_sdca_id.as_ptr(),
};
/* module_sdw_driver(rt715_sdw_driver); */

/* MODULE_DESCRIPTION("ASoC RT715 driver SDW SDCA"); */
/* MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
