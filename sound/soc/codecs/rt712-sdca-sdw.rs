// SPDX-License-Identifier: GPL-2.0-only
//
// rt712-sdca-sdw.c -- rt712 SDCA ALSA SoC audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
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
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool_,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool_,
    pub source_ports: c_ulong,
    pub sink_ports: c_ulong,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    pub control_port: c_uint,
    pub sdca_cascade: c_uint,
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
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
pub struct rt712_sdca_priv {
    pub hw_init: bool_,
    pub first_hw_init: bool_,
    pub hs_jack: bool_,
    pub disable_irq: bool_,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
}

pub type sdw_slave_status = c_uint;

unsafe extern "C" {
    static rt712_sdca_reg_defaults: [reg_default; 0];
    static rt712_sdca_mbq_defaults: [reg_default; 0];
    static system_power_efficient_wq: *mut c_void;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn sdw_slave_read_prop(slave: *mut sdw_slave);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn hweight32(w: c_uint) -> c_int;
    fn sdw_read_no_pm(slave: *mut sdw_slave, addr: c_uint) -> c_int;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, value: c_uint) -> c_int;
    fn sdw_update_no_pm(slave: *mut sdw_slave, addr: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn mod_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_ulong) -> bool_;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
    fn devm_regmap_init_sdw_mbq(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn rt712_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn rt712_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool_);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn SDW_SDCA_CTL(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint;
}

extern "C" {
    static SDW_SCP_SDCA_INTMASK1: c_uint;
    static SDW_SCP_SDCA_INTMASK2: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint;
    static SDW_SCP_SDCA_INT1: c_uint;
    static SDW_SCP_SDCA_INT2: c_uint;
    static SDW_DP0_INT: c_uint;
    static SDW_DP0_SDCA_CASCADE: c_uint;
    static SDW_SCP_INT1_BUS_CLASH: c_uint;
    static SDW_SCP_INT1_PARITY: c_uint;
    static SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint;
    static SDW_DPN_FULL: c_uint;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static REGCACHE_MAPLE: c_uint;
    static SDW_SLAVE_UNATTACHED: sdw_slave_status;
    static SDW_SLAVE_ATTACHED: sdw_slave_status;
    static FUNC_NUM_JACK_CODEC: c_uint;
    static FUNC_NUM_HID: c_uint;
    static FUNC_NUM_MIC_ARRAY: c_uint;
    static FUNC_NUM_AMP: c_uint;
    static RT712_SDCA_ENT_GE49: c_uint;
    static RT712_SDCA_ENT_HID01: c_uint;
    static RT712_SDCA_ENT0: c_uint;
    static RT712_SDCA_ENT_USER_FU05: c_uint;
    static RT712_SDCA_ENT_USER_FU0F: c_uint;
    static RT712_SDCA_ENT_USER_FU06: c_uint;
    static RT712_SDCA_ENT_USER_FU1E: c_uint;
    static RT712_SDCA_ENT_PLATFORM_FU15: c_uint;
    static RT712_SDCA_CTL_SELECTED_MODE: c_uint;
    static RT712_SDCA_CTL_DETECTED_MODE: c_uint;
    static RT712_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint;
    static RT712_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint;
    static RT712_SDCA_CTL_FUNC_STATUS: c_uint;
    static RT712_SDCA_CTL_FU_VOLUME: c_uint;
    static RT712_SDCA_CTL_FU_CH_GAIN: c_uint;
    static RT712_BUF_ADDR_HID1: c_uint;
    static RT712_BUF_ADDR_HID2: c_uint;
    static CH_01: c_uint;
    static CH_02: c_uint;
    static CH_03: c_uint;
    static CH_04: c_uint;
}

const fn BIT(n: c_uint) -> c_ulong {
    1_c_ulong << n
}

unsafe fn in_sdw_sdca_ctl_range(
    reg: c_uint,
    func: c_uint,
    ent: c_uint,
    ctl_start: c_uint,
    ctl_end: c_uint,
    ch: c_uint,
) -> bool_ {
    reg >= SDW_SDCA_CTL(func, ent, ctl_start, ch) && reg <= SDW_SDCA_CTL(func, ent, ctl_end, ch)
}

unsafe fn rt712_sdca_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if (0x201a..=0x201f).contains(&reg)
        || (0x2029..=0x202a).contains(&reg)
        || (0x202d..=0x2034).contains(&reg)
        || (0x2230..=0x2232).contains(&reg)
        || (0x2f01..=0x2f0a).contains(&reg)
        || (0x2f35..=0x2f36).contains(&reg)
        || reg == 0x2f50
        || reg == 0x2f54
        || (0x2f58..=0x2f5d).contains(&reg)
        || reg == 0x3201
        || reg == 0x320c
        || (0x3301..=0x3303).contains(&reg)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_SELECTED_MODE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_DETECTED_MODE, 0)
        || in_sdw_sdca_ctl_range(reg, FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_CURRENT_OWNER, RT712_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_HID, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || (RT712_BUF_ADDR_HID1..=RT712_BUF_ADDR_HID2).contains(&reg)
    {
        true
    } else {
        false
    }
}

unsafe fn rt712_sdca_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == 0x201b
        || reg == 0x201c
        || reg == 0x201d
        || reg == 0x201f
        || (0x202d..=0x202f).contains(&reg)
        || reg == 0x2230
        || reg == 0x2f01
        || reg == 0x2f35
        || reg == 0x320c
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_DETECTED_MODE, 0)
        || in_sdw_sdca_ctl_range(reg, FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_CURRENT_OWNER, RT712_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_HID, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0)
        || (RT712_BUF_ADDR_HID1..=RT712_BUF_ADDR_HID2).contains(&reg)
    {
        true
    } else {
        false
    }
}

unsafe fn rt712_sdca_mbq_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if (0x2000000..=0x200008e).contains(&reg)
        || (0x5300000..=0x530000e).contains(&reg)
        || (0x5400000..=0x540000e).contains(&reg)
        || (0x5600000..=0x5600008).contains(&reg)
        || (0x5700000..=0x570000d).contains(&reg)
        || (0x5800000..=0x5800021).contains(&reg)
        || (0x5900000..=0x5900028).contains(&reg)
        || (0x5a00000..=0x5a00009).contains(&reg)
        || (0x5b00000..=0x5b00051).contains(&reg)
        || (0x5c00000..=0x5c0009a).contains(&reg)
        || (0x5d00000..=0x5d00009).contains(&reg)
        || (0x5f00000..=0x5f00030).contains(&reg)
        || (0x6100000..=0x61000f1).contains(&reg)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_VOLUME, CH_01)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_VOLUME, CH_02)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_VOLUME, CH_01)
        || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_VOLUME, CH_02)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_VOLUME, CH_01)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_VOLUME, CH_02)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_01)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_02)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_03)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_04)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_01)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_02)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_03)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_04)
    {
        true
    } else {
        false
    }
}

unsafe fn rt712_sdca_mbq_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x2000000 | 0x200001a | 0x2000020 | 0x2000024 | 0x2000030 | 0x2000046
        | 0x200008a | 0x5800000 | 0x5800001 | 0x6100008 => true,
        _ => false,
    }
}

static rt712_sdca_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt712_sdca_readable_register),
    volatile_reg: Some(rt712_sdca_volatile_register),
    max_register: 0x44ffffff,
    reg_defaults: unsafe { rt712_sdca_reg_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: unsafe { REGCACHE_MAPLE },
    use_single_read: true,
    use_single_write: true,
};

static rt712_sdca_mbq_regmap: regmap_config = regmap_config {
    name: b"sdw-mbq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt712_sdca_mbq_readable_register),
    volatile_reg: Some(rt712_sdca_mbq_volatile_register),
    max_register: 0x41000312,
    reg_defaults: unsafe { rt712_sdca_mbq_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: unsafe { REGCACHE_MAPLE },
    use_single_read: true,
    use_single_write: true,
};

unsafe fn rt712_sdca_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let rt712 = dev_get_drvdata(&mut (*slave).dev) as *mut rt712_sdca_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt712).hw_init = false;
    }

    if status == SDW_SLAVE_ATTACHED {
        if (*rt712).hs_jack {
            /*
             * Due to the SCP_SDCA_INTMASK will be cleared by any reset, and then
             * if the device attached again, we will need to set the setting back.
             * It could avoid losing the jack detection interrupt.
             * This also could sync with the cache value as the rt712_sdca_jack_init set.
             */
            sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        }
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt712).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt712_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe fn rt712_sdca_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    sdw_slave_read_prop(slave);

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = BIT(8) | BIT(4); /* BITMAP: 100010000 */
    (*prop).sink_ports = BIT(3) | BIT(1); /* BITMAP:  00001010 */

    nval = hweight32((*prop).source_ports as c_uint);
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
    addr = (*prop).source_ports;
    bit = 0;
    while bit < 32 {
        if (addr & BIT(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32((*prop).sink_ports as c_uint);
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
    addr = (*prop).sink_ports;
    bit = 0;
    while bit < 32 {
        if (addr & BIT(bit)) != 0 {
            (*dpn.offset(j as isize)).num = bit;
            (*dpn.offset(j as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(j as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(j as isize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 1380;

    /* wake-up event */
    (*prop).wake_capable = 1;

    0
}

unsafe fn rt712_sdca_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    let rt712 = dev_get_drvdata(&mut (*slave).dev) as *mut rt712_sdca_priv;
    let mut ret: c_int;
    let mut stat: c_int;
    let mut count: c_int = 0;
    let retry: c_int = 3;
    let mut sdca_cascade: c_uint;
    let mut scp_sdca_stat1: c_uint;
    let mut scp_sdca_stat2: c_uint = 0;

    dev_dbg(
        &mut (*slave).dev,
        b"%s control_port_stat=%x, sdca_cascade=%x\0".as_ptr() as *const c_char,
        b"rt712_sdca_interrupt_callback\0".as_ptr(),
        (*status).control_port,
        (*status).sdca_cascade,
    );

    if cancel_delayed_work_sync(&mut (*rt712).jack_detect_work) {
        dev_warn(
            &mut (*slave).dev,
            b"%s the pending delayed_work was cancelled\0".as_ptr() as *const c_char,
            b"rt712_sdca_interrupt_callback\0".as_ptr(),
        );
        /* avoid the HID owner doesn't change to device */
        if (*rt712).scp_sdca_stat2 != 0 {
            scp_sdca_stat2 = (*rt712).scp_sdca_stat2;
        }
    }

    /*
     * The critical section below intentionally protects a rather large piece of code.
     * We don't want to allow the system suspend to disable an interrupt while we are
     * processing it, which could be problematic given the quirky SoundWire interrupt
     * scheme. We do want however to prevent new workqueues from being scheduled if
     * the disable_irq flag was set during system suspend.
     */
    mutex_lock(&mut (*rt712).disable_irq_lock);

    ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT1);
    if ret < 0 {
        goto_io_error(rt712, ret);
        return ret;
    }
    (*rt712).scp_sdca_stat1 = ret as c_uint;
    ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT2);
    if ret < 0 {
        goto_io_error(rt712, ret);
        return ret;
    }
    (*rt712).scp_sdca_stat2 = ret as c_uint;
    if scp_sdca_stat2 != 0 {
        (*rt712).scp_sdca_stat2 |= scp_sdca_stat2;
    }

    loop {
        /* clear flag */
        ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt712, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0) != 0 {
            ret = sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INT1, SDW_SCP_SDCA_INTMASK_SDCA_0);
            if ret < 0 {
                goto_io_error(rt712, ret);
                return ret;
            }
        }
        ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt712, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8) != 0 {
            ret = sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INT2, SDW_SCP_SDCA_INTMASK_SDCA_8);
            if ret < 0 {
                goto_io_error(rt712, ret);
                return ret;
            }
        }

        /* check if flag clear or not */
        ret = sdw_read_no_pm((*rt712).slave, SDW_DP0_INT);
        if ret < 0 {
            goto_io_error(rt712, ret);
            return ret;
        }
        sdca_cascade = ret as c_uint & SDW_DP0_SDCA_CASCADE;

        ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt712, ret);
            return ret;
        }
        scp_sdca_stat1 = ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0;

        ret = sdw_read_no_pm((*rt712).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt712, ret);
            return ret;
        }
        scp_sdca_stat2 = ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8;

        stat = ((scp_sdca_stat1 != 0) || (scp_sdca_stat2 != 0) || (sdca_cascade != 0)) as c_int;

        count += 1;
        if !(stat != 0 && count < retry) {
            break;
        }
    }

    if stat != 0 {
        dev_warn(
            &mut (*slave).dev,
            b"%s scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n\0".as_ptr() as *const c_char,
            b"rt712_sdca_interrupt_callback\0".as_ptr(),
            (*rt712).scp_sdca_stat1,
            (*rt712).scp_sdca_stat2,
        );
    }

    if (*status).sdca_cascade != 0 && !(*rt712).disable_irq {
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt712).jack_detect_work,
            msecs_to_jiffies(30),
        );
    }

    mutex_unlock(&mut (*rt712).disable_irq_lock);

    0
}

unsafe fn goto_io_error(rt712: *mut rt712_sdca_priv, ret: c_int) {
    mutex_unlock(&mut (*rt712).disable_irq_lock);
    pr_err_ratelimited(
        b"IO error in %s, ret %d\n\0".as_ptr() as *const c_char,
        b"rt712_sdca_interrupt_callback\0".as_ptr(),
        ret,
    );
}

static rt712_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt712_sdca_read_prop),
    interrupt_callback: Some(rt712_sdca_interrupt_callback),
    update_status: Some(rt712_sdca_update_status),
};

unsafe fn rt712_sdca_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;
    let mbq_regmap: *mut regmap;

    /* Regmap Initialization */
    mbq_regmap = devm_regmap_init_sdw_mbq(slave, &rt712_sdca_mbq_regmap);
    if IS_ERR(mbq_regmap as *const c_void) {
        return PTR_ERR(mbq_regmap as *const c_void);
    }

    regmap = devm_regmap_init_sdw(slave, &rt712_sdca_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt712_sdca_init(&mut (*slave).dev, regmap, mbq_regmap, slave)
}

unsafe fn rt712_sdca_sdw_remove(slave: *mut sdw_slave) {
    let rt712 = dev_get_drvdata(&mut (*slave).dev) as *mut rt712_sdca_priv;

    if (*rt712).hw_init {
        cancel_delayed_work_sync(&mut (*rt712).jack_detect_work);
        cancel_delayed_work_sync(&mut (*rt712).jack_btn_check_work);
    }

    pm_runtime_disable(&mut (*slave).dev);

    mutex_destroy(&mut (*rt712).calibrate_mutex);
    mutex_destroy(&mut (*rt712).disable_irq_lock);
}

static rt712_sdca_id: [sdw_device_id; 5] = [
    sdw_device_id { mfg_id: 0x025d, part_id: 0x712, sdw_version: 0x3, class_id: 0x1, unique_id: 0 },
    sdw_device_id { mfg_id: 0x025d, part_id: 0x713, sdw_version: 0x3, class_id: 0x1, unique_id: 0 },
    sdw_device_id { mfg_id: 0x025d, part_id: 0x716, sdw_version: 0x3, class_id: 0x1, unique_id: 0 },
    sdw_device_id { mfg_id: 0x025d, part_id: 0x717, sdw_version: 0x3, class_id: 0x1, unique_id: 0 },
    sdw_device_id { mfg_id: 0, part_id: 0, sdw_version: 0, class_id: 0, unique_id: 0 },
];
// MODULE_DEVICE_TABLE(sdw, rt712_sdca_id);

unsafe fn rt712_sdca_dev_suspend(dev: *mut device) -> c_int {
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_priv;

    if !(*rt712).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt712).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt712).jack_btn_check_work);

    regcache_cache_only((*rt712).regmap, true);
    regcache_cache_only((*rt712).mbq_regmap, true);

    0
}

unsafe fn rt712_sdca_dev_system_suspend(dev: *mut device) -> c_int {
    let rt712_sdca = dev_get_drvdata(dev) as *mut rt712_sdca_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret1: c_int;
    let ret2: c_int;

    if !(*rt712_sdca).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt712_sdca).disable_irq_lock);
    (*rt712_sdca).disable_irq = true;
    ret1 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0, 0);
    ret2 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8, 0);
    mutex_unlock(&mut (*rt712_sdca).disable_irq_lock);

    if ret1 < 0 || ret2 < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(
            &mut (*slave).dev,
            b"%s: could not disable SDCA interrupts\n:\0".as_ptr() as *const c_char,
            b"rt712_sdca_dev_system_suspend\0".as_ptr(),
        );
    }

    rt712_sdca_dev_suspend(dev)
}

const RT712_PROBE_TIMEOUT: c_int = 5000;

unsafe fn rt712_sdca_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_priv;
    let mut ret: c_int;

    if !(*rt712).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt712).disable_irq_lock);
        if (*rt712).disable_irq {
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
            (*rt712).disable_irq = false;
        }
        mutex_unlock(&mut (*rt712).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT712_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt712).regmap, false);
    ret = regcache_sync((*rt712).regmap);
    if ret != 0 {
        regcache_cache_only((*rt712).regmap, true);
        return ret;
    }

    regcache_cache_only((*rt712).mbq_regmap, false);
    ret = regcache_sync((*rt712).mbq_regmap);
    if ret != 0 {
        regcache_cache_only((*rt712).mbq_regmap, true);
        regcache_cache_only((*rt712).regmap, true);
        return ret;
    }

    0
}

static rt712_sdca_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt712_sdca_dev_system_suspend, rt712_sdca_dev_resume)
    system_suspend: Some(rt712_sdca_dev_system_suspend),
    resume: Some(rt712_sdca_dev_resume),
    // RUNTIME_PM_OPS(rt712_sdca_dev_suspend, rt712_sdca_dev_resume, NULL)
    runtime_suspend: Some(rt712_sdca_dev_suspend),
    runtime_resume: Some(rt712_sdca_dev_resume),
};

static mut rt712_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt712-sdca\0".as_ptr() as *const c_char,
        pm: &rt712_sdca_pm,
    },
    probe: Some(rt712_sdca_sdw_probe),
    remove: Some(rt712_sdca_sdw_remove),
    ops: &rt712_sdca_slave_ops,
    id_table: rt712_sdca_id.as_ptr(),
};
// module_sdw_driver(rt712_sdca_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT712 SDCA SDW driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
