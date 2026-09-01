// SPDX-License-Identifier: GPL-2.0
//
// rt711-sdw.c -- rt711 ALSA SoC audio driver
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
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

type bool_t = bool;
type u32 = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool_t,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_int,
    pub simple_ch_prep_sm: bool_t,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool_t,
    pub source_ports: u32,
    pub sink_ports: u32,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    pub control_port: c_uint,
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
    pub unique_id: c_uint,
    pub version: c_uint,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_int,
    pub use_single_read: bool_t,
    pub use_single_write: bool_t,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct rt711_priv {
    pub sdw_regmap: *mut regmap,
    pub regmap: *mut regmap,
    pub hw_init: bool_t,
    pub first_hw_init: bool_t,
    pub params: sdw_bus_params,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool_t,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibration_work: work_struct,
    pub calibrate_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_slave_status {
    SDW_SLAVE_UNATTACHED = 0,
    SDW_SLAVE_ATTACHED = 1,
}

const RT711_PRIV_DATA_R_H: c_uint = 0;
const RT711_PRIV_DATA_W_H: c_uint = 0;
const RT711_READ_HDA_3: c_uint = 0;
const RT711_READ_HDA_2: c_uint = 0;
const RT711_READ_HDA_1: c_uint = 0;
const RT711_READ_HDA_0: c_uint = 0;
const RT711_FUNC_RESET: c_uint = 0;
const REGCACHE_MAPLE: c_int = 0;
const REGCACHE_NONE: c_int = 0;
const SDW_SCP_INT1_IMPL_DEF: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_DPN_FULL: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SDW_SCP_INTMASK1: c_uint = 0;
const RT711_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" {
    static rt711_reg_defaults: [reg_default; 0];
    static mut system_power_efficient_wq: *mut workqueue_struct;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hweight32(w: u32) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn rt711_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn rt711_clock_config(dev: *mut device) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mod_delayed_work(
        wq: *mut workqueue_struct,
        dwork: *mut delayed_work,
        delay: c_ulong,
    ) -> bool_t;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn rt711_init(
        dev: *mut device,
        sdw_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool_t;
    fn cancel_work_sync(work: *mut work_struct) -> bool_t;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_update_no_pm(
        slave: *mut sdw_slave,
        addr: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, value: c_uint) -> c_int;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
}

unsafe extern "C" fn rt711_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        0x00e0
        | 0x00f0
        | 0x2012..=0x2016
        | 0x201a..=0x2027
        | 0x2029..=0x202a
        | 0x202d..=0x2034
        | 0x2201..=0x2204
        | 0x2206..=0x2212
        | 0x2220..=0x2223
        | 0x2230..=0x2239
        | 0x2f01..=0x2f0f
        | 0x3000..=0x3fff
        | 0x7000..=0x7fff
        | 0x8300..=0x83ff
        | 0x9c00..=0x9cff
        | 0xb900..=0xb9ff
        | 0x752008
        | 0x752009
        | 0x75200b
        | 0x752011
        | 0x75201a
        | 0x752045
        | 0x752046
        | 0x752048
        | 0x75204a
        | 0x75206b
        | 0x75206f
        | 0x752080
        | 0x752081
        | 0x752091
        | 0x755800 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt711_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        0x2016
        | 0x201b
        | 0x201c
        | 0x201d
        | 0x201f
        | 0x2021
        | 0x2023
        | 0x2230
        | 0x2012..=0x2015 /* HD-A read */
        | 0x202d..=0x202f /* BRA */
        | 0x2201..=0x2212 /* i2c debug */
        | 0x2220..=0x2223 /* decoded HD-A */
        | 0x9c00..=0x9cff
        | 0xb900..=0xb9ff
        | 0xff01
        | 0x75201a
        | 0x752046
        | 0x752080
        | 0x752081
        | 0x755800 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt711_sdw_read(
    context: *mut c_void,
    mut reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let dev = context as *mut device;
    let rt711 = dev_get_drvdata(dev) as *mut rt711_priv;
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
        ret = regmap_write((*rt711).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT711_PRIV_DATA_R_H | nid;
        ret = regmap_write((*rt711).sdw_regmap, reg3, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg4, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x3000 {
        reg += 0x8000;
        ret = regmap_write((*rt711).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x7000 {
        reg += 0x2000;
        reg |= 0x800;
        ret = regmap_write((*rt711).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt711).sdw_regmap, reg, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x9000 {
        ret = regmap_write((*rt711).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, *val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0xb000 {
        ret = regmap_write((*rt711).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = regmap_read((*rt711).sdw_regmap, reg, val);
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
        ret = regmap_read((*rt711).sdw_regmap, RT711_READ_HDA_3, &mut sdw_data_3);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt711).sdw_regmap, RT711_READ_HDA_2, &mut sdw_data_2);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt711).sdw_regmap, RT711_READ_HDA_1, &mut sdw_data_1);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt711).sdw_regmap, RT711_READ_HDA_0, &mut sdw_data_0);
        if ret < 0 {
            return ret;
        }
        *val = ((sdw_data_3 & 0xff) << 24)
            | ((sdw_data_2 & 0xff) << 16)
            | ((sdw_data_1 & 0xff) << 8)
            | (sdw_data_0 & 0xff);
    }

    if is_hda_reg == 0 {
        dev_dbg(dev, c"[%s] %04x => %08x\n".as_ptr(), c"rt711_sdw_read".as_ptr(), reg, *val);
    } else if is_index_reg != 0 {
        dev_dbg(
            dev,
            c"[%s] %04x %04x %04x %04x => %08x\n".as_ptr(),
            c"rt711_sdw_read".as_ptr(),
            reg,
            reg2,
            reg3,
            reg4,
            *val,
        );
    } else {
        dev_dbg(
            dev,
            c"[%s] %04x %04x => %08x\n".as_ptr(),
            c"rt711_sdw_read".as_ptr(),
            reg,
            reg2,
            *val,
        );
    }

    0
}

unsafe extern "C" fn rt711_sdw_write(
    context: *mut c_void,
    mut reg: c_uint,
    val: c_uint,
) -> c_int {
    let dev = context as *mut device;
    let rt711 = dev_get_drvdata(dev) as *mut rt711_priv;
    let mut reg2: c_uint = 0;
    let mut reg3: c_uint = 0;
    let mut reg4: c_uint = 0;
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
        ret = regmap_write((*rt711).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT711_PRIV_DATA_W_H | nid;
        ret = regmap_write((*rt711).sdw_regmap, reg3, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg4, val & 0xff);
        if ret < 0 {
            return ret;
        }
        is_index_reg = 1;
    } else if reg < 0x4fff {
        ret = regmap_write((*rt711).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
    } else if reg == RT711_FUNC_RESET {
        ret = regmap_write((*rt711).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x7000 {
        ret = regmap_write((*rt711).sdw_regmap, reg, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, val & 0xff);
        if ret < 0 {
            return ret;
        }
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt711).sdw_regmap, reg2, (val >> 8) & 0xff);
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt711).sdw_regmap, reg, val & 0xff);
        if ret < 0 {
            return ret;
        }
    }

    if reg2 == 0 {
        dev_dbg(dev, c"[%s] %04x <= %04x\n".as_ptr(), c"rt711_sdw_write".as_ptr(), reg, val);
    } else if is_index_reg != 0 {
        dev_dbg(
            dev,
            c"[%s] %04x %04x %04x %04x <= %04x %04x\n".as_ptr(),
            c"rt711_sdw_write".as_ptr(),
            reg,
            reg2,
            reg3,
            reg4,
            val2,
            val,
        );
    } else {
        dev_dbg(
            dev,
            c"[%s] %04x %04x <= %04x\n".as_ptr(),
            c"rt711_sdw_write".as_ptr(),
            reg,
            reg2,
            val,
        );
    }

    0
}

static rt711_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 24,
    val_bits: 32,
    readable_reg: Some(rt711_readable_register),
    volatile_reg: Some(rt711_volatile_register),
    max_register: 0x755800,
    reg_defaults: unsafe { rt711_reg_defaults.as_ptr() },
    num_reg_defaults: unsafe { rt711_reg_defaults.len() as c_uint },
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
    reg_read: Some(rt711_sdw_read),
    reg_write: Some(rt711_sdw_write),
};

static rt711_sdw_regmap: regmap_config = regmap_config {
    name: c"sdw".as_ptr(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt711_readable_register),
    volatile_reg: None,
    max_register: 0xff01,
    reg_defaults: ptr::null(),
    num_reg_defaults: 0,
    cache_type: REGCACHE_NONE,
    use_single_read: true,
    use_single_write: true,
    reg_read: None,
    reg_write: None,
};

unsafe extern "C" fn rt711_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_priv;

    if status == sdw_slave_status::SDW_SLAVE_UNATTACHED {
        (*rt711).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt711).hw_init || status != sdw_slave_status::SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt711_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt711_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = false;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x14; /* BITMAP: 00010100 */
    (*prop).sink_ports = 0x8; /* BITMAP:  00001000 */

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

    i = 0;
    dpn = (*prop).src_dpn_prop;
    addr = (*prop).source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1_c_ulong << bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32((*prop).sink_ports);
    (*prop).sink_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1_c_ulong << bit)) != 0 {
            (*dpn.offset(j as isize)).num = bit;
            (*dpn.offset(j as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(j as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(j as isize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    /* wake-up event */
    (*prop).wake_capable = 1;

    0
}

unsafe extern "C" fn rt711_bus_config(
    slave: *mut sdw_slave,
    params: *mut sdw_bus_params,
) -> c_int {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_priv;
    let ret: c_int;

    memcpy(
        &mut (*rt711).params as *mut sdw_bus_params as *mut c_void,
        params as *const c_void,
        size_of::<sdw_bus_params>(),
    );

    ret = rt711_clock_config(&mut (*slave).dev);
    if ret < 0 {
        dev_err(&mut (*slave).dev, c"%s: Invalid clk config".as_ptr(), c"rt711_bus_config".as_ptr());
    }

    ret
}

unsafe extern "C" fn rt711_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_priv;

    dev_dbg(
        &mut (*slave).dev,
        c"%s control_port_stat=%x".as_ptr(),
        c"rt711_interrupt_callback".as_ptr(),
        (*status).control_port,
    );

    mutex_lock(&mut (*rt711).disable_irq_lock);
    if ((*status).control_port & 0x4) != 0 && !(*rt711).disable_irq {
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt711).jack_detect_work,
            msecs_to_jiffies(250),
        );
    }
    mutex_unlock(&mut (*rt711).disable_irq_lock);

    0
}

static rt711_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt711_read_prop),
    interrupt_callback: Some(rt711_interrupt_callback),
    update_status: Some(rt711_update_status),
    bus_config: Some(rt711_bus_config),
};

unsafe extern "C" fn rt711_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let sdw_regmap: *mut regmap;
    let regmap: *mut regmap;

    /* Regmap Initialization */
    sdw_regmap = devm_regmap_init_sdw(slave, &rt711_sdw_regmap);
    if IS_ERR(sdw_regmap as *const c_void) {
        return PTR_ERR(sdw_regmap as *const c_void);
    }

    regmap = devm_regmap_init(
        &mut (*slave).dev,
        ptr::null(),
        &mut (*slave).dev as *mut device as *mut c_void,
        &rt711_regmap,
    );
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt711_init(&mut (*slave).dev, sdw_regmap, regmap, slave)
}

unsafe extern "C" fn rt711_sdw_remove(slave: *mut sdw_slave) {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_priv;

    if (*rt711).hw_init {
        cancel_delayed_work_sync(&mut (*rt711).jack_detect_work);
        cancel_delayed_work_sync(&mut (*rt711).jack_btn_check_work);
        cancel_work_sync(&mut (*rt711).calibration_work);
    }

    pm_runtime_disable(&mut (*slave).dev);

    mutex_destroy(&mut (*rt711).calibrate_mutex);
    mutex_destroy(&mut (*rt711).disable_irq_lock);
}

const rt711_id: [sdw_device_id; 2] = [
    sdw_device_id {
        mfg_id: 0x025d,
        part_id: 0x711,
        class_id: 0x2,
        unique_id: 0,
        version: 0,
    },
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
        unique_id: 0,
        version: 0,
    },
];
/* MODULE_DEVICE_TABLE(sdw, rt711_id); */

unsafe extern "C" fn rt711_dev_suspend(dev: *mut device) -> c_int {
    let rt711 = dev_get_drvdata(dev) as *mut rt711_priv;

    if !(*rt711).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt711).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt711).jack_btn_check_work);
    cancel_work_sync(&mut (*rt711).calibration_work);

    regcache_cache_only((*rt711).regmap, true);

    0
}

unsafe extern "C" fn rt711_dev_system_suspend(dev: *mut device) -> c_int {
    let rt711 = dev_get_drvdata(dev) as *mut rt711_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret: c_int;

    if !(*rt711).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt711).disable_irq_lock);
    (*rt711).disable_irq = true;
    ret = sdw_update_no_pm(slave, SDW_SCP_INTMASK1, SDW_SCP_INT1_IMPL_DEF, 0);
    mutex_unlock(&mut (*rt711).disable_irq_lock);

    if ret < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(
            &mut (*slave).dev,
            c"%s: could not disable imp-def interrupts\n:".as_ptr(),
            c"rt711_dev_system_suspend".as_ptr(),
        );
    }

    rt711_dev_suspend(dev)
}

unsafe extern "C" fn rt711_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt711 = dev_get_drvdata(dev) as *mut rt711_priv;
    let ret: c_int;

    if !(*rt711).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt711).disable_irq_lock);
        if (*rt711).disable_irq {
            sdw_write_no_pm(slave, SDW_SCP_INTMASK1, SDW_SCP_INT1_IMPL_DEF);
            (*rt711).disable_irq = false;
        }
        mutex_unlock(&mut (*rt711).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT711_PROBE_TIMEOUT);
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*rt711).regmap, false);
    regcache_sync_region((*rt711).regmap, 0x3000, 0x8fff);
    regcache_sync_region((*rt711).regmap, 0x752009, 0x752091);

    0
}

static rt711_pm: dev_pm_ops = dev_pm_ops {
    system_suspend: Some(rt711_dev_system_suspend),
    system_resume: Some(rt711_dev_resume),
    runtime_suspend: Some(rt711_dev_suspend),
    runtime_resume: Some(rt711_dev_resume),
    runtime_idle: None,
};

static mut rt711_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c"rt711".as_ptr(),
        pm: &rt711_pm,
    },
    probe: Some(rt711_sdw_probe),
    remove: Some(rt711_sdw_remove),
    ops: &rt711_slave_ops,
    id_table: rt711_id.as_ptr(),
};
/* module_sdw_driver(rt711_sdw_driver); */

const MODULE_DESCRIPTION: &str = "ASoC RT711 SDW driver";
const MODULE_AUTHOR: &str = "Shuming Fan <shumingf@realtek.com>";
const MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
