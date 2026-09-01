// SPDX-License-Identifier: GPL-2.0-only
//
// rt721-sdca-sdw.c -- rt721 SDCA ALSA SoC audio driver
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//
//

// Rust translation of the original C implementation source.
// C include dependencies are expected to be supplied by surrounding bindings:
// linux/cleanup.h, linux/delay.h, linux/device.h, linux/module.h,
// linux/pm_runtime.h, linux/soundwire/sdw_registers.h,
// rt721-sdca.h, rt721-sdca-sdw.h, rt-sdw-common.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static rt721_sdca_reg_defaults: [reg_default; 0];
    static rt721_sdca_mbq_defaults: [reg_default; 0];
    static system_power_efficient_wq: *mut workqueue_struct;
    static __this_module: module;

    fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    fn sdw_write_no_pm(slave: *mut sdw_slave, reg: c_uint, val: c_uint) -> c_int;
    fn sdw_update_no_pm(slave: *mut sdw_slave, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn sdw_read_no_pm(slave: *mut sdw_slave, reg: c_uint) -> c_int;
    fn rt721_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn sdw_slave_read_prop(slave: *mut sdw_slave);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn hweight32(w: c_uint) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
    fn mod_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn devm_regmap_init_sdw_mbq(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn rt721_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
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
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool,
    pub source_ports: c_ulong,
    pub sink_ports: c_ulong,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
    pub lane_control_support: bool,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    pub sdca_cascade: bool,
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
pub struct rt721_sdca_priv {
    pub hw_init: bool,
    pub hs_jack: bool,
    pub slave: *mut sdw_slave,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub disable_irq_lock: mutex,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub disable_irq: bool,
    pub first_hw_init: bool,
    pub calibrate_mutex: mutex,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
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
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
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
    pub system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub owner: *const module,
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_slave_status {
    SDW_SLAVE_UNATTACHED = 0,
    SDW_SLAVE_ATTACHED = 1,
}

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_SCP_SDCA_INTMASK1: c_uint = 0;
const SDW_SCP_SDCA_INTMASK2: c_uint = 0;
const SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint = 0;
const SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint = 0;
const SDW_SCP_SDCA_INT1: c_uint = 0;
const SDW_SCP_SDCA_INT2: c_uint = 0;
const SDW_SCP_SDCA_INT_SDCA_0: c_uint = 0;
const SDW_DP0_INT: c_uint = 0;
const SDW_DP0_SDCA_CASCADE: c_uint = 0;
const FUNC_NUM_JACK_CODEC: c_uint = 0;
const FUNC_NUM_HID: c_uint = 0;
const FUNC_NUM_MIC_ARRAY: c_uint = 0;
const FUNC_NUM_AMP: c_uint = 0;
const RT721_SDCA_ENT_XUV: c_uint = 0;
const RT721_SDCA_CTL_XUV: c_uint = 0;
const RT721_SDCA_ENT_GE49: c_uint = 0;
const RT721_SDCA_CTL_SELECTED_MODE: c_uint = 0;
const RT721_SDCA_CTL_DETECTED_MODE: c_uint = 0;
const RT721_SDCA_ENT_HID01: c_uint = 0;
const RT721_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0;
const RT721_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0;
const RT721_BUF_ADDR_HID1: c_uint = 0;
const RT721_BUF_ADDR_HID2: c_uint = 0;
const RT721_SDCA_ENT_USER_FU05: c_uint = 0;
const RT721_SDCA_ENT_USER_FU0F: c_uint = 0;
const RT721_SDCA_ENT_PLATFORM_FU44: c_uint = 0;
const RT721_SDCA_ENT_USER_FU1E: c_uint = 0;
const RT721_SDCA_ENT_USER_FU06: c_uint = 0;
const RT721_SDCA_CTL_FU_VOLUME: c_uint = 0;
const RT721_SDCA_CTL_FU_CH_GAIN: c_uint = 0;
const CH_L: c_uint = 0;
const CH_R: c_uint = 0;
const CH_01: c_uint = 0;
const CH_02: c_uint = 0;
const CH_03: c_uint = 0;
const CH_04: c_uint = 0;

const fn BIT(n: c_uint) -> c_ulong {
    1u64.wrapping_shl(n) as c_ulong
}

const fn SDW_SDCA_CTL(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint {
    ((func & 0xff) << 24) | ((ent & 0xff) << 16) | ((ctl & 0xff) << 8) | (ch & 0xff)
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe extern "C" fn rt721_sdca_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f01..=0x2f0a
        | 0x2f35
        | 0x2f50
        | 0x2f51
        | 0x2f58..=0x2f5d
        | x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_XUV,
            RT721_SDCA_CTL_XUV, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_GE49,
            RT721_SDCA_CTL_SELECTED_MODE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_GE49,
            RT721_SDCA_CTL_DETECTED_MODE, 0) => true,
        x if x >= SDW_SDCA_CTL(FUNC_NUM_HID, RT721_SDCA_ENT_HID01,
            RT721_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
            && x <= SDW_SDCA_CTL(FUNC_NUM_HID, RT721_SDCA_ENT_HID01,
            RT721_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0) => true,
        x if x >= RT721_BUF_ADDR_HID1 && x <= RT721_BUF_ADDR_HID2 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt721_sdca_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f01 | 0x2f51 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_GE49,
            RT721_SDCA_CTL_DETECTED_MODE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_XUV,
            RT721_SDCA_CTL_XUV, 0) => true,
        x if x >= SDW_SDCA_CTL(FUNC_NUM_HID, RT721_SDCA_ENT_HID01,
            RT721_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
            && x <= SDW_SDCA_CTL(FUNC_NUM_HID, RT721_SDCA_ENT_HID01,
            RT721_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0) => true,
        x if x >= RT721_BUF_ADDR_HID1 && x <= RT721_BUF_ADDR_HID2 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt721_sdca_mbq_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x0900004..=0x0900009
        | 0x0a00005
        | 0x0c00005
        | 0x0d00014
        | 0x0310100
        | 0x2000000..=0x2000003
        | 0x2000013
        | 0x200002c
        | 0x200003c
        | 0x2000046
        | 0x5810000
        | 0x5810036
        | 0x5810037
        | 0x5810038
        | 0x5810039
        | 0x5b10018
        | 0x5b10019
        | 0x5f00045
        | 0x5f00048
        | 0x6100000
        | 0x6100005
        | 0x6100006
        | 0x610000d
        | 0x6100010
        | 0x6100011
        | 0x6100013
        | 0x6100015
        | 0x6100017
        | 0x6100025
        | 0x6100029
        | 0x610002c..=0x610002f
        | 0x6100053..=0x6100055
        | 0x6100057
        | 0x610005a
        | 0x610005b
        | 0x610006a
        | 0x610006d
        | 0x6100092 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05,
            RT721_SDCA_CTL_FU_VOLUME, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05,
            RT721_SDCA_CTL_FU_VOLUME, CH_R) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F,
            RT721_SDCA_CTL_FU_VOLUME, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F,
            RT721_SDCA_CTL_FU_VOLUME, CH_R) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PLATFORM_FU44,
            RT721_SDCA_CTL_FU_CH_GAIN, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PLATFORM_FU44,
            RT721_SDCA_CTL_FU_CH_GAIN, CH_R) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E,
            RT721_SDCA_CTL_FU_VOLUME, CH_01) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E,
            RT721_SDCA_CTL_FU_VOLUME, CH_02) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E,
            RT721_SDCA_CTL_FU_VOLUME, CH_03) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E,
            RT721_SDCA_CTL_FU_VOLUME, CH_04) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06,
            RT721_SDCA_CTL_FU_VOLUME, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06,
            RT721_SDCA_CTL_FU_VOLUME, CH_R) => true,
        _ => false,
    }
}

unsafe extern "C" fn rt721_sdca_mbq_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x0310100
        | 0x0900005
        | 0x0900009
        | 0x0a00005
        | 0x0c00005
        | 0x0d00014
        | 0x2000000
        | 0x200000d
        | 0x2000019
        | 0x2000020
        | 0x200002c
        | 0x2000030
        | 0x2000046
        | 0x2000067
        | 0x2000084
        | 0x2000086
        | 0x5810000
        | 0x5810036
        | 0x5810037
        | 0x5810038
        | 0x5810039
        | 0x5b10018
        | 0x5b10019 => true,
        _ => false,
    }
}

static rt721_sdca_regmap: regmap_config = regmap_config {
    name: core::ptr::null(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt721_sdca_readable_register),
    volatile_reg: Some(rt721_sdca_volatile_register),
    max_register: 0x44ffffff,
    reg_defaults: unsafe { rt721_sdca_reg_defaults.as_ptr() },
    num_reg_defaults: unsafe { ARRAY_SIZE(&rt721_sdca_reg_defaults) },
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

static rt721_sdca_mbq_regmap: regmap_config = regmap_config {
    name: b"sdw-mbq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt721_sdca_mbq_readable_register),
    volatile_reg: Some(rt721_sdca_mbq_volatile_register),
    max_register: 0x41000312,
    reg_defaults: unsafe { rt721_sdca_mbq_defaults.as_ptr() },
    num_reg_defaults: unsafe { ARRAY_SIZE(&rt721_sdca_mbq_defaults) },
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt721_sdca_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt721 = dev_get_drvdata(&mut (*slave).dev) as *mut rt721_sdca_priv;

    if status == sdw_slave_status::SDW_SLAVE_UNATTACHED {
        (*rt721).hw_init = false;
    }

    if status == sdw_slave_status::SDW_SLAVE_ATTACHED {
        if (*rt721).hs_jack {
            /*
             * Due to the SCP_SDCA_INTMASK will be cleared by any reset, and then
             * if the device attached again, we will need to set the setting back.
             * It could avoid losing the jack detection interrupt.
             * This also could sync with the cache value as the rt721_sdca_jack_init set.
             */
            sdw_write_no_pm((*rt721).slave, SDW_SCP_SDCA_INTMASK1,
                SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm((*rt721).slave, SDW_SCP_SDCA_INTMASK2,
                SDW_SCP_SDCA_INTMASK_SDCA_8);
        }
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt721).hw_init || status != sdw_slave_status::SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt721_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt721_sdca_read_prop(slave: *mut sdw_slave) -> c_int {
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

    /*
     * port = 1 for headphone playback
     * port = 2 for headset-mic capture
     * port = 3 for speaker playback
     * port = 6 for digital-mic capture
     */
    (*prop).source_ports = BIT(6) | BIT(2); /* BITMAP: 01000100 */
    (*prop).sink_ports = BIT(3) | BIT(1); /* BITMAP:  00001010 */

    nval = hweight32((*prop).source_ports as c_uint);
    (*prop).src_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        core::mem::size_of::<sdw_dpn_prop>(),
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
        core::mem::size_of::<sdw_dpn_prop>(),
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

    /* Three data lanes are supported by rt721-sdca codec */
    (*prop).lane_control_support = true;

    0
}

unsafe extern "C" fn rt721_sdca_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    let rt721 = dev_get_drvdata(&mut (*slave).dev) as *mut rt721_sdca_priv;
    let mut ret: c_int;
    let mut stat: c_int;
    let mut count: c_int = 0;
    let retry: c_int = 3;
    let mut sdca_cascade: c_uint;
    let mut scp_sdca_stat1: c_uint;
    let mut scp_sdca_stat2: c_uint = 0;

    if cancel_delayed_work_sync(&mut (*rt721).jack_detect_work) {
        dev_warn(&mut (*slave).dev,
            b"%s the pending delayed_work was cancelled\0".as_ptr() as *const c_char,
            b"rt721_sdca_interrupt_callback\0".as_ptr() as *const c_char);
        /* avoid the HID owner doesn't change to device */
        if (*rt721).scp_sdca_stat2 != 0 {
            scp_sdca_stat2 = (*rt721).scp_sdca_stat2;
        }
    }

    /*
     * The critical section below intentionally protects a rather large piece of code.
     * We don't want to allow the system suspend to disable an interrupt while we are
     * processing it, which could be problematic given the quirky SoundWire interrupt
     * scheme. We do want however to prevent new workqueues from being scheduled if
     * the disable_irq flag was set during system suspend.
     */
    mutex_lock(&mut (*rt721).disable_irq_lock);

    ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT1);
    if ret < 0 {
        goto_io_error(rt721, ret);
        return ret;
    }

    (*rt721).scp_sdca_stat1 = ret as c_uint;
    ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT2);
    if ret < 0 {
        goto_io_error(rt721, ret);
        return ret;
    }

    (*rt721).scp_sdca_stat2 = ret as c_uint;
    if scp_sdca_stat2 != 0 {
        (*rt721).scp_sdca_stat2 |= scp_sdca_stat2;
    }
    loop {
        /* clear flag */
        ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt721, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0) != 0 {
            ret = sdw_update_no_pm((*rt721).slave, SDW_SCP_SDCA_INT1,
                SDW_SCP_SDCA_INT_SDCA_0, SDW_SCP_SDCA_INT_SDCA_0);
            if ret < 0 {
                goto_io_error(rt721, ret);
                return ret;
            }
        }
        ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt721, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8) != 0 {
            ret = sdw_write_no_pm((*rt721).slave, SDW_SCP_SDCA_INT2,
                SDW_SCP_SDCA_INTMASK_SDCA_8);
            if ret < 0 {
                goto_io_error(rt721, ret);
                return ret;
            }
        }

        /* check if flag clear or not */
        ret = sdw_read_no_pm((*rt721).slave, SDW_DP0_INT);
        if ret < 0 {
            goto_io_error(rt721, ret);
            return ret;
        }
        sdca_cascade = ret as c_uint & SDW_DP0_SDCA_CASCADE;

        ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt721, ret);
            return ret;
        }
        scp_sdca_stat1 = ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0;

        ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt721, ret);
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
        dev_warn(&mut (*slave).dev,
            b"%s scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n\0".as_ptr() as *const c_char,
            b"rt721_sdca_interrupt_callback\0".as_ptr() as *const c_char,
            (*rt721).scp_sdca_stat1, (*rt721).scp_sdca_stat2);
    }
    ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT1);
    ret = sdw_read_no_pm((*rt721).slave, SDW_SCP_SDCA_INT2);
    let _ = ret;

    if (*status).sdca_cascade && !(*rt721).disable_irq {
        mod_delayed_work(system_power_efficient_wq,
            &mut (*rt721).jack_detect_work, msecs_to_jiffies(280));
    }

    mutex_unlock(&mut (*rt721).disable_irq_lock);

    0
}

unsafe fn goto_io_error(rt721: *mut rt721_sdca_priv, ret: c_int) {
    mutex_unlock(&mut (*rt721).disable_irq_lock);
    pr_err_ratelimited(b"IO error in %s, ret %d\n\0".as_ptr() as *const c_char,
        b"rt721_sdca_interrupt_callback\0".as_ptr() as *const c_char, ret);
}

static rt721_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt721_sdca_read_prop),
    interrupt_callback: Some(rt721_sdca_interrupt_callback),
    update_status: Some(rt721_sdca_update_status),
};

unsafe extern "C" fn rt721_sdca_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let regmap: *mut regmap;
    let mbq_regmap: *mut regmap;

    /* Regmap Initialization */
    mbq_regmap = devm_regmap_init_sdw_mbq(slave, &rt721_sdca_mbq_regmap);
    if IS_ERR(mbq_regmap as *const c_void) {
        return PTR_ERR(mbq_regmap as *const c_void);
    }

    regmap = devm_regmap_init_sdw(slave, &rt721_sdca_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt721_sdca_init(&mut (*slave).dev, regmap, mbq_regmap, slave)
}

unsafe extern "C" fn rt721_sdca_sdw_remove(slave: *mut sdw_slave) {
    let rt721 = dev_get_drvdata(&mut (*slave).dev) as *mut rt721_sdca_priv;

    if (*rt721).hw_init {
        cancel_delayed_work_sync(&mut (*rt721).jack_detect_work);
        cancel_delayed_work_sync(&mut (*rt721).jack_btn_check_work);
    }

    if (*rt721).first_hw_init {
        pm_runtime_disable(&mut (*slave).dev);
    }

    mutex_destroy(&mut (*rt721).calibrate_mutex);
    mutex_destroy(&mut (*rt721).disable_irq_lock);
}

static rt721_sdca_id: [sdw_device_id; 2] = [
    sdw_device_id {
        mfg_id: 0x025d,
        part_id: 0x721,
        sdw_version: 0x3,
        class_id: 0x1,
        unique_id: 0,
    },
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        sdw_version: 0,
        class_id: 0,
        unique_id: 0,
    },
];
// MODULE_DEVICE_TABLE(sdw, rt721_sdca_id);

unsafe extern "C" fn rt721_sdca_dev_suspend(dev: *mut device) -> c_int {
    let rt721 = dev_get_drvdata(dev) as *mut rt721_sdca_priv;

    if !(*rt721).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt721).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt721).jack_btn_check_work);

    regcache_cache_only((*rt721).regmap, true);
    regcache_cache_only((*rt721).mbq_regmap, true);

    0
}

unsafe extern "C" fn rt721_sdca_dev_system_suspend(dev: *mut device) -> c_int {
    let rt721_sdca = dev_get_drvdata(dev) as *mut rt721_sdca_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret1: c_int;
    let ret2: c_int;

    if !(*rt721_sdca).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt721_sdca).disable_irq_lock);
    (*rt721_sdca).disable_irq = true;
    ret1 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK1,
        SDW_SCP_SDCA_INTMASK_SDCA_0, 0);
    ret2 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK2,
        SDW_SCP_SDCA_INTMASK_SDCA_8, 0);
    mutex_unlock(&mut (*rt721_sdca).disable_irq_lock);

    if ret1 < 0 || ret2 < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(&mut (*slave).dev,
            b"%s: could not disable SDCA interrupts\n:\0".as_ptr() as *const c_char,
            b"rt721_sdca_dev_system_suspend\0".as_ptr() as *const c_char);
    }

    rt721_sdca_dev_suspend(dev)
}

const RT721_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn rt721_sdca_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt721 = dev_get_drvdata(dev) as *mut rt721_sdca_priv;
    let mut ret: c_int;

    if !(*rt721).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt721).disable_irq_lock);
        if (*rt721).disable_irq {
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK1,
                SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK2,
                SDW_SCP_SDCA_INTMASK_SDCA_8);
            (*rt721).disable_irq = false;
        }
        mutex_unlock(&mut (*rt721).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT721_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt721).regmap, false);
    ret = regcache_sync((*rt721).regmap);
    if ret != 0 {
        regcache_cache_only((*rt721).regmap, true);
        return ret;
    }

    regcache_cache_only((*rt721).mbq_regmap, false);
    ret = regcache_sync((*rt721).mbq_regmap);
    if ret != 0 {
        regcache_cache_only((*rt721).mbq_regmap, true);
        regcache_cache_only((*rt721).regmap, true);
        return ret;
    }

    0
}

static rt721_sdca_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt721_sdca_dev_system_suspend, rt721_sdca_dev_resume)
    system_suspend: Some(rt721_sdca_dev_system_suspend),
    system_resume: Some(rt721_sdca_dev_resume),
    // RUNTIME_PM_OPS(rt721_sdca_dev_suspend, rt721_sdca_dev_resume, NULL)
    runtime_suspend: Some(rt721_sdca_dev_suspend),
    runtime_resume: Some(rt721_sdca_dev_resume),
    runtime_idle: None,
};

static mut rt721_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt721-sdca\0".as_ptr() as *const c_char,
        owner: unsafe { &__this_module },
        pm: unsafe { pm_ptr(&rt721_sdca_pm) },
    },
    probe: Some(rt721_sdca_sdw_probe),
    remove: Some(rt721_sdca_sdw_remove),
    ops: &rt721_sdca_slave_ops,
    id_table: rt721_sdca_id.as_ptr(),
};
// module_sdw_driver(rt721_sdca_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT721 SDCA SDW driver");
// MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
