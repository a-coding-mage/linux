// SPDX-License-Identifier: GPL-2.0
/*
 * rt715-sdw.c -- rt715 ALSA SoC audio driver
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 *
 * ALC715 ASoC Codec Driver based Intel Dummy SdW codec driver
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type Bool = bool;
type U32 = u32;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const REGCACHE_NONE: c_uint = 0;
const SDW_SCP_INT1_IMPL_DEF: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 0;
const RT715_PROBE_TIMEOUT: c_int = 5000;

extern "C" {
    static rt715_reg_defaults: [reg_default; 0];

    static RT715_PRIV_DATA_R_H: c_uint;
    static RT715_PRIV_DATA_W_H: c_uint;
    static RT715_READ_HDA_3: c_uint;
    static RT715_READ_HDA_2: c_uint;
    static RT715_READ_HDA_1: c_uint;
    static RT715_READ_HDA_0: c_uint;
    static RT715_FUNC_RESET: c_uint;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn hweight32(w: c_uint) -> c_int;
    fn rt715_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn rt715_clock_config(dev: *mut device) -> c_int;
    fn rt715_init(
        dev: *mut device,
        sdw_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

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

pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt715_priv {
    pub sdw_regmap: *mut regmap,
    pub regmap: *mut regmap,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub params: sdw_bus_params,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: U32,
    pub simple_ch_prep_sm: bool,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool,
    pub source_ports: c_uint,
    pub sink_ports: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
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
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: usize,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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

unsafe extern "C" fn rt715_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x00e0..=0x00e5
        | 0x00ee..=0x00ef
        | 0x00f0..=0x00f5
        | 0x00fe..=0x00ff
        | 0x02e0
        | 0x02f0
        | 0x04e0
        | 0x04f0
        | 0x06e0
        | 0x06f0
        | 0x2000..=0x2016
        | 0x201a..=0x2027
        | 0x2029..=0x202a
        | 0x202d..=0x2034
        | 0x2200..=0x2204
        | 0x2206..=0x2212
        | 0x2220..=0x2223
        | 0x2230..=0x2239
        | 0x22f0..=0x22f3
        | 0x3122
        | 0x3123
        | 0x3124
        | 0x3125
        | 0x3607
        | 0x3608
        | 0x3609
        | 0x3610
        | 0x3611
        | 0x3627
        | 0x3712
        | 0x3713
        | 0x3718
        | 0x3719
        | 0x371a
        | 0x371b
        | 0x371d
        | 0x3729
        | 0x385e
        | 0x3859
        | 0x4c12
        | 0x4c13
        | 0x4c1d
        | 0x4c29
        | 0x4d12
        | 0x4d13
        | 0x4d1d
        | 0x4d29
        | 0x4e12
        | 0x4e13
        | 0x4e1d
        | 0x4e29
        | 0x4f12
        | 0x4f13
        | 0x4f1d
        | 0x4f29
        | 0x7207
        | 0x7208
        | 0x7209
        | 0x7227
        | 0x7307
        | 0x7308
        | 0x7309
        | 0x7312
        | 0x7313
        | 0x7318
        | 0x7319
        | 0x731a
        | 0x731b
        | 0x731d
        | 0x7327
        | 0x7329
        | 0x8287
        | 0x8288
        | 0x8289
        | 0x82a7
        | 0x8387
        | 0x8388
        | 0x8389
        | 0x8392
        | 0x8393
        | 0x8398
        | 0x8399
        | 0x839a
        | 0x839b
        | 0x839d
        | 0x83a7
        | 0x83a9
        | 0x752001
        | 0x752039 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt715_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x00e5
        | 0x00f0
        | 0x00f3
        | 0x00f5
        | 0x2009
        | 0x2016
        | 0x201b
        | 0x201c
        | 0x201d
        | 0x201f
        | 0x2023
        | 0x2230
        | 0x200b..=0x200e /* i2c read */
        | 0x2012..=0x2015 /* HD-A read */
        | 0x202d..=0x202f /* BRA */
        | 0x2201..=0x2212 /* i2c debug */
        | 0x2220..=0x2223 /* decoded HD-A */ => true,
        _ => false,
    }
}

unsafe extern "C" fn rt715_sdw_read(
    context: *mut c_void,
    mut reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let dev = context as *mut device;
    let rt715 = dev_get_drvdata(dev) as *mut rt715_priv;
    let mut sdw_data_3: c_uint;
    let mut sdw_data_2: c_uint;
    let mut sdw_data_1: c_uint;
    let mut sdw_data_0: c_uint;
    let mut reg2: c_uint = 0;
    let mut reg3: c_uint = 0;
    let mut reg4: c_uint = 0;
    let mask: c_uint;
    let nid: c_uint;
    let val2: c_uint;
    let mut is_hda_reg: c_uint = 1;
    let mut is_index_reg: c_uint = 0;
    let mut ret: c_int;

    if reg > 0xffff {
        is_index_reg = 1;
    }

    mask = reg & 0xf000;

    if is_index_reg != 0 {
        /* index registers */
        val2 = reg & 0xff;
        reg >>= 8;
        nid = reg & 0xff;
        ret = regmap_write((*rt715).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT715_PRIV_DATA_R_H | nid;
        ret = regmap_write((*rt715).sdw_regmap, reg3, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg4, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x3000 {
        reg += 0x8000;
        ret = regmap_write((*rt715).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x7000 {
        reg += 0x2000;
        reg |= 0x800;
        ret = regmap_write((*rt715).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt715).sdw_regmap, reg, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x9000 {
        ret = regmap_write((*rt715).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0xb000 {
        ret = regmap_write((*rt715).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = regmap_read((*rt715).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
        is_hda_reg = 0;
    }

    if is_hda_reg != 0 || is_index_reg != 0 {
        sdw_data_3 = 0;
        sdw_data_2 = 0;
        sdw_data_1 = 0;
        sdw_data_0 = 0;
        ret = regmap_read((*rt715).sdw_regmap, RT715_READ_HDA_3, &mut sdw_data_3);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt715).sdw_regmap, RT715_READ_HDA_2, &mut sdw_data_2);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt715).sdw_regmap, RT715_READ_HDA_1, &mut sdw_data_1);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt715).sdw_regmap, RT715_READ_HDA_0, &mut sdw_data_0);
        if ret < 0 {
            return ret;
        }
        *val = ((sdw_data_3 & 0xff) << 24)
            | ((sdw_data_2 & 0xff) << 16)
            | ((sdw_data_1 & 0xff) << 8)
            | (sdw_data_0 & 0xff);
    }

    if is_hda_reg == 0 {
        dev_dbg(dev, b"[%s] %04x => %08x\n\0".as_ptr() as *const c_char, b"rt715_sdw_read\0".as_ptr(), reg, *val);
    } else if is_index_reg != 0 {
        dev_dbg(dev, b"[%s] %04x %04x %04x %04x => %08x\n\0".as_ptr() as *const c_char, b"rt715_sdw_read\0".as_ptr(), reg, reg2, reg3, reg4, *val);
    } else {
        dev_dbg(dev, b"[%s] %04x %04x => %08x\n\0".as_ptr() as *const c_char, b"rt715_sdw_read\0".as_ptr(), reg, reg2, *val);
    }

    0
}

unsafe extern "C" fn rt715_sdw_write(context: *mut c_void, mut reg: c_uint, val: c_uint) -> c_int {
    let dev = context as *mut device;
    let rt715 = dev_get_drvdata(dev) as *mut rt715_priv;
    let mut reg2: c_uint = 0;
    let reg3: c_uint;
    let reg4: c_uint;
    let nid: c_uint;
    let mask: c_uint;
    let val2: c_uint;
    let mut is_index_reg: c_uint = 0;
    let mut ret: c_int;

    if reg > 0xffff {
        is_index_reg = 1;
    }

    mask = reg & 0xf000;

    if is_index_reg != 0 {
        /* index registers */
        val2 = reg & 0xff;
        reg >>= 8;
        nid = reg & 0xff;
        ret = regmap_write((*rt715).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT715_PRIV_DATA_W_H | nid;
        ret = regmap_write((*rt715).sdw_regmap, reg3, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg4, val & 0xff);
        if ret < 0 {
            return ret;
        }
        is_index_reg = 1;
    } else if reg < 0x4fff {
        ret = regmap_write((*rt715).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
        reg3 = 0;
        reg4 = 0;
        val2 = 0;
    } else if reg == RT715_FUNC_RESET {
        ret = regmap_write((*rt715).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
        reg3 = 0;
        reg4 = 0;
        val2 = 0;
    } else if mask == 0x7000 {
        ret = regmap_write((*rt715).sdw_regmap, reg, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, val & 0xff);
        if ret < 0 {
            return ret;
        }
        reg3 = 0;
        reg4 = 0;
        val2 = 0;
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt715).sdw_regmap, reg2, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt715).sdw_regmap, reg, val & 0xff);
        if ret < 0 {
            return ret;
        }
        reg3 = 0;
        reg4 = 0;
        val2 = 0;
    } else {
        reg3 = 0;
        reg4 = 0;
        val2 = 0;
    }

    if reg2 == 0 {
        dev_dbg(dev, b"[%s] %04x <= %04x\n\0".as_ptr() as *const c_char, b"rt715_sdw_write\0".as_ptr(), reg, val);
    } else if is_index_reg != 0 {
        dev_dbg(dev, b"[%s] %04x %04x %04x %04x <= %04x %04x\n\0".as_ptr() as *const c_char, b"rt715_sdw_write\0".as_ptr(), reg, reg2, reg3, reg4, val2, val);
    } else {
        dev_dbg(dev, b"[%s] %04x %04x <= %04x\n\0".as_ptr() as *const c_char, b"rt715_sdw_write\0".as_ptr(), reg, reg2, val);
    }

    0
}

static rt715_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 24,
    val_bits: 32,
    readable_reg: Some(rt715_readable_register), /* Readable registers */
    volatile_reg: Some(rt715_volatile_register), /* volatile register */
    max_register: 0x752039,                      /* Maximum number of register */
    reg_defaults: unsafe { rt715_reg_defaults.as_ptr() }, /* Defaults */
    num_reg_defaults: 0,                         /* ARRAY_SIZE(rt715_reg_defaults) */
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
    reg_read: Some(rt715_sdw_read),
    reg_write: Some(rt715_sdw_write),
};

static rt715_sdw_regmap: regmap_config = regmap_config {
    name: b"sdw\0".as_ptr() as *const c_char,
    reg_bits: 32,       /* Total register space for SDW */
    val_bits: 8,        /* Total number of bits in register */
    readable_reg: None,
    volatile_reg: None,
    max_register: 0xff01, /* Maximum number of register */
    reg_defaults: ptr::null(),
    num_reg_defaults: 0,
    cache_type: REGCACHE_NONE,
    use_single_read: true,
    use_single_write: true,
    reg_read: None,
    reg_write: None,
};

unsafe extern "C" fn rt715_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt715 = dev_get_drvdata(&mut (*slave).dev) as *mut rt715_priv;

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt715).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt715_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt715_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let nval: c_int;
    let mut i: c_int;
    let mut bit: U32;
    let mut addr: c_ulong;
    let dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = false;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x50; /* BITMAP: 01010000 */
    (*prop).sink_ports = 0x0; /* BITMAP:  00000000 */

    nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval,
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
        if (addr & (1 as c_ulong) << bit) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    /* wake-up event */
    (*prop).wake_capable = 1;

    0
}

unsafe extern "C" fn rt715_bus_config(
    slave: *mut sdw_slave,
    params: *mut sdw_bus_params,
) -> c_int {
    let rt715 = dev_get_drvdata(&mut (*slave).dev) as *mut rt715_priv;
    let ret: c_int;

    memcpy(
        &mut (*rt715).params as *mut sdw_bus_params as *mut c_void,
        params as *const c_void,
        size_of::<sdw_bus_params>(),
    );

    ret = rt715_clock_config(&mut (*slave).dev);
    if ret < 0 {
        dev_err(&mut (*slave).dev, b"%s: Invalid clk config\0".as_ptr() as *const c_char, b"rt715_bus_config\0".as_ptr());
    }

    0
}

static rt715_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt715_read_prop),
    update_status: Some(rt715_update_status),
    bus_config: Some(rt715_bus_config),
};

unsafe extern "C" fn rt715_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let sdw_regmap: *mut regmap;
    let regmap: *mut regmap;

    /* Regmap Initialization */
    sdw_regmap = devm_regmap_init_sdw(slave, &rt715_sdw_regmap);
    if IS_ERR(sdw_regmap as *const c_void) {
        return PTR_ERR(sdw_regmap as *const c_void);
    }

    regmap = devm_regmap_init(&mut (*slave).dev, ptr::null(), &mut (*slave).dev, &rt715_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt715_init(&mut (*slave).dev, sdw_regmap, regmap, slave)
}

unsafe extern "C" fn rt715_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
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

static rt715_id: [sdw_device_id; 3] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x714, 0x2, 0, 0),
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x715, 0x2, 0, 0),
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        sdw_version: 0,
        class_id: 0,
        unique_id: 0,
    },
];
/* MODULE_DEVICE_TABLE(sdw, rt715_id); */

unsafe extern "C" fn rt715_dev_suspend(dev: *mut device) -> c_int {
    let rt715 = dev_get_drvdata(dev) as *mut rt715_priv;

    if !(*rt715).hw_init {
        return 0;
    }

    regcache_cache_only((*rt715).regmap, true);

    0
}

unsafe extern "C" fn rt715_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt715 = dev_get_drvdata(dev) as *mut rt715_priv;
    let ret: c_int;

    if !(*rt715).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT715_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt715).regmap, false);
    regcache_sync_region((*rt715).regmap, 0x3000, 0x8fff);
    regcache_sync_region((*rt715).regmap, 0x752039, 0x752039);

    0
}

static rt715_pm: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(rt715_dev_suspend, rt715_dev_resume) */
    suspend: Some(rt715_dev_suspend),
    resume: Some(rt715_dev_resume),
    /* RUNTIME_PM_OPS(rt715_dev_suspend, rt715_dev_resume, NULL) */
    runtime_suspend: Some(rt715_dev_suspend),
    runtime_resume: Some(rt715_dev_resume),
    runtime_idle: None,
};

static mut rt715_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt715\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&rt715_pm) },
    },
    probe: Some(rt715_sdw_probe),
    remove: Some(rt715_sdw_remove),
    ops: &rt715_slave_ops,
    id_table: rt715_id.as_ptr(),
};
/* module_sdw_driver(rt715_sdw_driver); */

/* MODULE_DESCRIPTION("ASoC RT715 driver SDW"); */
/* MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
