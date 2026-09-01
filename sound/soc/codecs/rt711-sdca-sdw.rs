// SPDX-License-Identifier: GPL-2.0-only
//
// rt711-sdw-sdca.c -- rt711 SDCA ALSA SoC audio driver
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//
//

// Dependencies from the original C file:
// linux/cleanup.h, linux/delay.h, linux/device.h,
// linux/soundwire/sdw_registers.h, linux/module.h, linux/pm_runtime.h,
// "rt711-sdca.h", and "rt711-sdca-sdw.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

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
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_int,
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_int,
}

#[repr(C)]
pub struct sdw_dp0_prop {
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_int,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool_,
    pub source_ports: c_uint,
    pub sink_ports: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub dp0_prop: *mut sdw_dp0_prop,
    pub clk_stop_timeout: c_int,
    pub wake_capable: c_int,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool_,
    pub bus: *mut sdw_bus,
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
    pub class_id: c_uint,
    pub sdw_version: c_uint,
    pub unique_id: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_int,
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_slave_status {
    SDW_SLAVE_UNATTACHED = 0,
    SDW_SLAVE_ATTACHED = 1,
}

#[repr(C)]
pub struct rt711_sdca_priv {
    pub hw_init: bool_,
    pub hs_jack: bool_,
    pub slave: *mut sdw_slave,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool_,
    pub calibrate_mutex: mutex,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub first_hw_init: bool_,
}

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_int = 0;
const SDW_DPN_FULL: c_int = 0;

const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_SCP_SDCA_INTMASK1: c_uint = 0;
const SDW_SCP_SDCA_INTMASK2: c_uint = 0;
const SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint = 0;
const SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint = 0;
const SDW_SCP_SDCA_INT1: c_uint = 0;
const SDW_SCP_SDCA_INT2: c_uint = 0;
const SDW_DP0_INT: c_uint = 0;
const SDW_DP0_SDCA_CASCADE: c_uint = 0;

const FUNC_NUM_JACK_CODEC: c_uint = 0;
const FUNC_NUM_HID: c_uint = 0;
const FUNC_NUM_MIC_ARRAY: c_uint = 0;
const RT711_SDCA_ENT_GE49: c_uint = 0;
const RT711_SDCA_ENT_HID01: c_uint = 0;
const RT711_SDCA_ENT_USER_FU05: c_uint = 0;
const RT711_SDCA_ENT_USER_FU1E: c_uint = 0;
const RT711_SDCA_ENT_USER_FU0F: c_uint = 0;
const RT711_SDCA_ENT_PLATFORM_FU44: c_uint = 0;
const RT711_SDCA_ENT_PLATFORM_FU15: c_uint = 0;
const RT711_SDCA_CTL_SELECTED_MODE: c_uint = 0;
const RT711_SDCA_CTL_DETECTED_MODE: c_uint = 0;
const RT711_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0;
const RT711_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0;
const RT711_SDCA_CTL_FU_VOLUME: c_uint = 0;
const RT711_SDCA_CTL_FU_CH_GAIN: c_uint = 0;
const CH_L: c_uint = 0;
const CH_R: c_uint = 0;
const RT711_BUF_ADDR_HID1: c_uint = 0;
const RT711_BUF_ADDR_HID2: c_uint = 0;
const RT711_RC_CAL_STATUS: c_uint = 0;

const fn SDW_SDCA_CTL(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint {
    ((func) << 24) | ((ent) << 16) | ((ctl) << 8) | ch
}

const RT711_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" {
    static rt711_sdca_reg_defaults: [reg_default; 0];
    static rt711_sdca_mbq_defaults: [reg_default; 0];
    static mut system_power_efficient_wq: *mut workqueue_struct;

    fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn hweight32(w: c_uint) -> c_int;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, value: c_uint) -> c_int;
    fn sdw_read_no_pm(slave: *mut sdw_slave, addr: c_uint) -> c_int;
    fn sdw_update_no_pm(slave: *mut sdw_slave, addr: c_uint, mask: c_uint, value: c_uint)
        -> c_int;
    fn rt711_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mod_delayed_work(
        wq: *mut workqueue_struct,
        dwork: *mut delayed_work,
        delay: c_ulong,
    ) -> bool_;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn devm_regmap_init_sdw_mbq(
        slave: *mut sdw_slave,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn rt711_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, show: bool_);
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
}

unsafe extern "C" fn rt711_sdca_readable_register(
    _dev: *mut device,
    reg: c_uint,
) -> bool_ {
    match reg {
        0x201a..=0x2027
        | 0x2029..=0x202a
        | 0x202d..=0x2034
        | 0x2200..=0x2204
        | 0x2206..=0x2212
        | 0x2220..=0x2223
        | 0x2230..=0x2239
        | 0x2f01..=0x2f0f
        | 0x2f30..=0x2f36
        | 0x2f50..=0x2f5a
        | 0x2f60
        | 0x3200..=0x3212 => true,
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_GE49,
                RT711_SDCA_CTL_SELECTED_MODE,
                0,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_GE49,
                RT711_SDCA_CTL_DETECTED_MODE,
                0,
            ) =>
        {
            true
        }
        _ if reg
            >= SDW_SDCA_CTL(
                FUNC_NUM_HID,
                RT711_SDCA_ENT_HID01,
                RT711_SDCA_CTL_HIDTX_CURRENT_OWNER,
                0,
            )
            && reg
                <= SDW_SDCA_CTL(
                    FUNC_NUM_HID,
                    RT711_SDCA_ENT_HID01,
                    RT711_SDCA_CTL_HIDTX_MESSAGE_LENGTH,
                    0,
                ) =>
        {
            true
        }
        _ if reg >= RT711_BUF_ADDR_HID1 && reg <= RT711_BUF_ADDR_HID2 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt711_sdca_volatile_register(
    _dev: *mut device,
    reg: c_uint,
) -> bool_ {
    match reg {
        0x201b | 0x201c | 0x201d | 0x201f | 0x2021 | 0x2023 | 0x2230 => true,
        0x202d..=0x202f => true, /* BRA */
        0x2200..=0x2212 => true, /* i2c debug */
        _ if reg == RT711_RC_CAL_STATUS => true,
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_GE49,
                RT711_SDCA_CTL_DETECTED_MODE,
                0,
            ) =>
        {
            true
        }
        _ if reg
            >= SDW_SDCA_CTL(
                FUNC_NUM_HID,
                RT711_SDCA_ENT_HID01,
                RT711_SDCA_CTL_HIDTX_CURRENT_OWNER,
                0,
            )
            && reg
                <= SDW_SDCA_CTL(
                    FUNC_NUM_HID,
                    RT711_SDCA_ENT_HID01,
                    RT711_SDCA_CTL_HIDTX_MESSAGE_LENGTH,
                    0,
                ) =>
        {
            true
        }
        _ if reg >= RT711_BUF_ADDR_HID1 && reg <= RT711_BUF_ADDR_HID2 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt711_sdca_mbq_readable_register(
    _dev: *mut device,
    reg: c_uint,
) -> bool_ {
    match reg {
        0x2000000..=0x20000ff
        | 0x5600000..=0x56000ff
        | 0x5700000..=0x57000ff
        | 0x5800000..=0x58000ff
        | 0x5900000..=0x59000ff
        | 0x5b00000..=0x5b000ff
        | 0x5f00000..=0x5f000ff
        | 0x6100000..=0x61000ff => true,
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_USER_FU05,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_L,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_USER_FU05,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_R,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_MIC_ARRAY,
                RT711_SDCA_ENT_USER_FU1E,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_L,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_MIC_ARRAY,
                RT711_SDCA_ENT_USER_FU1E,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_R,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_USER_FU0F,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_L,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_USER_FU0F,
                RT711_SDCA_CTL_FU_VOLUME,
                CH_R,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_PLATFORM_FU44,
                RT711_SDCA_CTL_FU_CH_GAIN,
                CH_L,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_JACK_CODEC,
                RT711_SDCA_ENT_PLATFORM_FU44,
                RT711_SDCA_CTL_FU_CH_GAIN,
                CH_R,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_MIC_ARRAY,
                RT711_SDCA_ENT_PLATFORM_FU15,
                RT711_SDCA_CTL_FU_CH_GAIN,
                CH_L,
            ) =>
        {
            true
        }
        _ if reg
            == SDW_SDCA_CTL(
                FUNC_NUM_MIC_ARRAY,
                RT711_SDCA_ENT_PLATFORM_FU15,
                RT711_SDCA_CTL_FU_CH_GAIN,
                CH_R,
            ) =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn rt711_sdca_mbq_volatile_register(
    _dev: *mut device,
    reg: c_uint,
) -> bool_ {
    match reg {
        0x2000000 | 0x200001a | 0x2000046 | 0x2000080 | 0x2000081 | 0x2000083
        | 0x5800000 | 0x5800001 | 0x5f00001 | 0x6100008 => true,
        _ => false,
    }
}

static rt711_sdca_regmap: regmap_config = regmap_config {
    name: core::ptr::null(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt711_sdca_readable_register),
    volatile_reg: Some(rt711_sdca_volatile_register),
    max_register: 0x44ffffff,
    reg_defaults: unsafe { rt711_sdca_reg_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

static rt711_sdca_mbq_regmap: regmap_config = regmap_config {
    name: b"sdw-mbq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt711_sdca_mbq_readable_register),
    volatile_reg: Some(rt711_sdca_mbq_volatile_register),
    max_register: 0x40800f12,
    reg_defaults: unsafe { rt711_sdca_mbq_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt711_sdca_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_sdca_priv;

    if status == sdw_slave_status::SDW_SLAVE_UNATTACHED {
        (*rt711).hw_init = false;
    }

    if status == sdw_slave_status::SDW_SLAVE_ATTACHED {
        if (*rt711).hs_jack {
            /*
             * Due to the SCP_SDCA_INTMASK will be cleared by any reset, and then
             * if the device attached again, we will need to set the setting back.
             * It could avoid losing the jack detection interrupt.
             * This also could sync with the cache value as the rt711_sdca_jack_init set.
             */
            sdw_write_no_pm(
                (*rt711).slave,
                SDW_SCP_SDCA_INTMASK1,
                SDW_SCP_SDCA_INTMASK_SDCA_0,
            );
            sdw_write_no_pm(
                (*rt711).slave,
                SDW_SCP_SDCA_INTMASK2,
                SDW_SCP_SDCA_INTMASK_SDCA_8,
            );
        }
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt711).hw_init || status != sdw_slave_status::SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt711_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt711_sdca_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x14; /* BITMAP: 00010100 */
    (*prop).sink_ports = 0x8; /* BITMAP:  00001000 */

    nval = hweight32((*prop).source_ports);
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
    addr = (*prop).source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
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
        core::mem::size_of::<sdw_dpn_prop>(),
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
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(j as isize)).num = bit;
            (*dpn.offset(j as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(j as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(j as isize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    (*prop).dp0_prop = devm_kzalloc(
        &mut (*slave).dev,
        core::mem::size_of::<sdw_dp0_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dp0_prop;
    if (*prop).dp0_prop.is_null() {
        return -ENOMEM;
    }

    (*(*prop).dp0_prop).simple_ch_prep_sm = true;
    (*(*prop).dp0_prop).ch_prep_timeout = 10;

    /* set the timeout values */
    (*prop).clk_stop_timeout = 700;

    /* wake-up event */
    (*prop).wake_capable = 1;

    0
}

unsafe extern "C" fn rt711_sdca_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_sdca_priv;
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
        b"rt711_sdca_interrupt_callback\0".as_ptr() as *const c_char,
        (*status).control_port,
        (*status).sdca_cascade,
    );

    if cancel_delayed_work_sync(&mut (*rt711).jack_detect_work) {
        dev_warn(
            &mut (*slave).dev,
            b"%s the pending delayed_work was cancelled\0".as_ptr() as *const c_char,
            b"rt711_sdca_interrupt_callback\0".as_ptr() as *const c_char,
        );
        /* avoid the HID owner doesn't change to device */
        if (*rt711).scp_sdca_stat2 != 0 {
            scp_sdca_stat2 = (*rt711).scp_sdca_stat2;
        }
    }

    /*
     * The critical section below intentionally protects a rather large piece of code.
     * We don't want to allow the system suspend to disable an interrupt while we are
     * processing it, which could be problematic given the quirky SoundWire interrupt
     * scheme. We do want however to prevent new workqueues from being scheduled if
     * the disable_irq flag was set during system suspend.
     */
    mutex_lock(&mut (*rt711).disable_irq_lock);

    ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT1);
    if ret < 0 {
        goto_io_error(slave, rt711, ret);
        return ret;
    }
    (*rt711).scp_sdca_stat1 = ret as c_uint;
    ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT2);
    if ret < 0 {
        goto_io_error(slave, rt711, ret);
        return ret;
    }
    (*rt711).scp_sdca_stat2 = ret as c_uint;
    if scp_sdca_stat2 != 0 {
        (*rt711).scp_sdca_stat2 |= scp_sdca_stat2;
    }

    loop {
        /* clear flag */
        ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(slave, rt711, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0) != 0 {
            ret = sdw_write_no_pm(
                (*rt711).slave,
                SDW_SCP_SDCA_INT1,
                SDW_SCP_SDCA_INTMASK_SDCA_0,
            );
            if ret < 0 {
                goto_io_error(slave, rt711, ret);
                return ret;
            }
        }
        ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(slave, rt711, ret);
            return ret;
        }
        if (ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8) != 0 {
            ret = sdw_write_no_pm(
                (*rt711).slave,
                SDW_SCP_SDCA_INT2,
                SDW_SCP_SDCA_INTMASK_SDCA_8,
            );
            if ret < 0 {
                goto_io_error(slave, rt711, ret);
                return ret;
            }
        }

        /* check if flag clear or not */
        ret = sdw_read_no_pm((*rt711).slave, SDW_DP0_INT);
        if ret < 0 {
            goto_io_error(slave, rt711, ret);
            return ret;
        }
        sdca_cascade = ret as c_uint & SDW_DP0_SDCA_CASCADE;

        ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(slave, rt711, ret);
            return ret;
        }
        scp_sdca_stat1 = ret as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0;

        ret = sdw_read_no_pm((*rt711).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(slave, rt711, ret);
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
            b"rt711_sdca_interrupt_callback\0".as_ptr() as *const c_char,
            (*rt711).scp_sdca_stat1,
            (*rt711).scp_sdca_stat2,
        );
    }

    if (*status).sdca_cascade != 0 && !(*rt711).disable_irq {
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt711).jack_detect_work,
            msecs_to_jiffies(30),
        );
    }

    mutex_unlock(&mut (*rt711).disable_irq_lock);

    0
}

unsafe fn goto_io_error(slave: *mut sdw_slave, rt711: *mut rt711_sdca_priv, ret: c_int) {
    mutex_unlock(&mut (*rt711).disable_irq_lock);
    pr_err_ratelimited(
        b"IO error in %s, ret %d\n\0".as_ptr() as *const c_char,
        b"rt711_sdca_interrupt_callback\0".as_ptr() as *const c_char,
        ret,
    );
    let _ = slave;
}

static rt711_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt711_sdca_read_prop),
    interrupt_callback: Some(rt711_sdca_interrupt_callback),
    update_status: Some(rt711_sdca_update_status),
};

unsafe extern "C" fn rt711_sdca_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let regmap: *mut regmap;
    let mbq_regmap: *mut regmap;

    /* Regmap Initialization */
    mbq_regmap = devm_regmap_init_sdw_mbq(slave, &rt711_sdca_mbq_regmap);
    if IS_ERR(mbq_regmap as *const c_void) {
        return PTR_ERR(mbq_regmap as *const c_void);
    }

    regmap = devm_regmap_init_sdw(slave, &rt711_sdca_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt711_sdca_init(&mut (*slave).dev, regmap, mbq_regmap, slave)
}

unsafe extern "C" fn rt711_sdca_sdw_remove(slave: *mut sdw_slave) {
    let rt711 = dev_get_drvdata(&mut (*slave).dev) as *mut rt711_sdca_priv;

    if (*rt711).hw_init {
        cancel_delayed_work_sync(&mut (*rt711).jack_detect_work);
        cancel_delayed_work_sync(&mut (*rt711).jack_btn_check_work);
    }

    pm_runtime_disable(&mut (*slave).dev);

    mutex_destroy(&mut (*rt711).calibrate_mutex);
    mutex_destroy(&mut (*rt711).disable_irq_lock);
}

const fn SDW_SLAVE_ENTRY_EXT(
    mfg_id: c_uint,
    part_id: c_uint,
    class_id: c_uint,
    sdw_version: c_uint,
    unique_id: c_uint,
) -> sdw_device_id {
    sdw_device_id {
        mfg_id,
        part_id,
        class_id,
        sdw_version,
        unique_id,
        driver_data: 0,
    }
}

static rt711_sdca_id: [sdw_device_id; 2] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x711, 0x3, 0x1, 0),
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
        sdw_version: 0,
        unique_id: 0,
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(sdw, rt711_sdca_id);

unsafe extern "C" fn rt711_sdca_dev_suspend(dev: *mut device) -> c_int {
    let rt711 = dev_get_drvdata(dev) as *mut rt711_sdca_priv;

    if !(*rt711).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt711).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt711).jack_btn_check_work);

    regcache_cache_only((*rt711).regmap, true);
    regcache_cache_only((*rt711).mbq_regmap, true);

    0
}

unsafe extern "C" fn rt711_sdca_dev_system_suspend(dev: *mut device) -> c_int {
    let rt711_sdca = dev_get_drvdata(dev) as *mut rt711_sdca_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret1: c_int;
    let ret2: c_int;

    if !(*rt711_sdca).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt711_sdca).disable_irq_lock);
    (*rt711_sdca).disable_irq = true;
    ret1 = sdw_update_no_pm(
        slave,
        SDW_SCP_SDCA_INTMASK1,
        SDW_SCP_SDCA_INTMASK_SDCA_0,
        0,
    );
    ret2 = sdw_update_no_pm(
        slave,
        SDW_SCP_SDCA_INTMASK2,
        SDW_SCP_SDCA_INTMASK_SDCA_8,
        0,
    );
    mutex_unlock(&mut (*rt711_sdca).disable_irq_lock);

    if ret1 < 0 || ret2 < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(
            &mut (*slave).dev,
            b"%s: could not disable SDCA interrupts\n:\0".as_ptr() as *const c_char,
            b"rt711_sdca_dev_system_suspend\0".as_ptr() as *const c_char,
        );
    }

    rt711_sdca_dev_suspend(dev)
}

unsafe extern "C" fn rt711_sdca_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt711 = dev_get_drvdata(dev) as *mut rt711_sdca_priv;
    let mut ret: c_int;

    if !(*rt711).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt711).disable_irq_lock);
        if (*rt711).disable_irq {
            sdw_write_no_pm(
                slave,
                SDW_SCP_SDCA_INTMASK1,
                SDW_SCP_SDCA_INTMASK_SDCA_0,
            );
            sdw_write_no_pm(
                slave,
                SDW_SCP_SDCA_INTMASK2,
                SDW_SCP_SDCA_INTMASK_SDCA_8,
            );
            (*rt711).disable_irq = false;
        }
        mutex_unlock(&mut (*rt711).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT711_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt711).regmap, false);
    regcache_sync((*rt711).regmap);
    regcache_cache_only((*rt711).mbq_regmap, false);
    regcache_sync((*rt711).mbq_regmap);
    0
}

static rt711_sdca_pm: dev_pm_ops = dev_pm_ops {
    _private: [],
};
// SYSTEM_SLEEP_PM_OPS(rt711_sdca_dev_system_suspend, rt711_sdca_dev_resume)
// RUNTIME_PM_OPS(rt711_sdca_dev_suspend, rt711_sdca_dev_resume, NULL)

static mut rt711_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt711-sdca\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&rt711_sdca_pm) },
    },
    probe: Some(rt711_sdca_sdw_probe),
    remove: Some(rt711_sdca_sdw_remove),
    ops: &rt711_sdca_slave_ops,
    id_table: rt711_sdca_id.as_ptr(),
};
// module_sdw_driver(rt711_sdca_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT711 SDCA SDW driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
