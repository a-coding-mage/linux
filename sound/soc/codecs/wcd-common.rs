// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025, Qualcomm Technologies, Inc. and/or its subsidiaries.

// Rust translation of soc/codecs/wcd-common.c.
// C includes removed; referenced kernel and driver symbols are declared below.

use core::ffi::{c_char, c_int, c_uint, c_void};

const WCD_MIN_MICBIAS_MV: u32 = 1000;
const WCD_DEF_MICBIAS_MV: u32 = 1800;
const WCD_MAX_MICBIAS_MV: u32 = 2850;

const EINVAL: c_int = 22;
const IRQ_HANDLED: c_int = 1;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;

#[inline]
const fn SWRS_SCP_HOST_CLK_DIV2_CTL_BANK(m: c_uint) -> c_uint {
    0xE0 + 0x10 * m
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
}

pub type sdw_slave_status = c_int;

#[repr(C)]
pub struct sdw_bus_params {
    pub next_bank: c_uint,
}

#[repr(C)]
pub struct wcd_common {
    pub dev: *mut device,
    pub max_bias: c_int,
    pub micb_mv: *mut u32,
    pub micb_vout: *mut c_int,
}

#[repr(C)]
pub struct component_ops {
    pub bind: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            master: *mut device,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub unbind: Option<
        unsafe extern "C" fn(dev: *mut device, master: *mut device, data: *mut c_void),
    >,
}

unsafe extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_int,
    ) -> c_int;

    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);

    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn sdw_write(slave: *mut sdw_slave, addr: c_uint, value: u8) -> c_int;
    fn handle_nested_irq(irq: c_uint);
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: c_uint) -> c_uint;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcd_get_micb_vout_ctl_val(
    dev: *mut device,
    micb_mv: u32,
) -> c_int {
    /* min micbias voltage is 1V and maximum is 2.85V */
    if micb_mv < WCD_MIN_MICBIAS_MV || micb_mv > WCD_MAX_MICBIAS_MV {
        unsafe {
            dev_err(
                dev,
                c"Unsupported micbias voltage (%u mV)\n".as_ptr(),
                micb_mv,
            );
        }
        return -EINVAL;
    }

    ((micb_mv - WCD_MIN_MICBIAS_MV) / 50) as c_int
}

unsafe extern "C" fn wcd_get_micbias_val(
    dev: *mut device,
    micb_num: c_int,
    micb_mv: *mut u32,
) -> c_int {
    let mut micbias = [0 as c_char; 64];
    let mut mv: c_int = 0;

    unsafe {
        sprintf(
            micbias.as_mut_ptr(),
            c"qcom,micbias%d-microvolt".as_ptr(),
            micb_num,
        );
    }

    if unsafe {
        of_property_read_u32(
            (*dev).of_node,
            micbias.as_ptr(),
            &mut mv as *mut c_int,
        )
    } != 0
    {
        unsafe {
            dev_err(
                dev,
                c"%s value not found, using default\n".as_ptr(),
                micbias.as_ptr(),
            );
        }
        mv = WCD_DEF_MICBIAS_MV as c_int;
    } else {
        /* convert it to milli volts */
        mv = mv / 1000;
    }
    if !micb_mv.is_null() {
        unsafe {
            *micb_mv = mv as u32;
        }
    }

    mv = unsafe { wcd_get_micb_vout_ctl_val(dev, mv as u32) };
    if mv < 0 {
        unsafe {
            dev_err(
                dev,
                c"Unsupported %s voltage (%d mV), falling back to default (%d mV)\n".as_ptr(),
                micbias.as_ptr(),
                mv,
                WCD_DEF_MICBIAS_MV as c_int,
            );
        }
        return unsafe { wcd_get_micb_vout_ctl_val(dev, WCD_DEF_MICBIAS_MV) };
    }

    mv
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcd_dt_parse_micbias_info(common: *mut wcd_common) -> c_int {
    let mut ret: c_int;
    let mut i: c_int = 0;

    while i < unsafe { (*common).max_bias } {
        ret = unsafe {
            wcd_get_micbias_val(
                (*common).dev,
                i + 1,
                (*common).micb_mv.offset(i as isize),
            )
        };
        if ret < 0 {
            return ret;
        }
        unsafe {
            *(*common).micb_vout.offset(i as isize) = ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn wcd_sdw_component_bind(
    dev: *mut device,
    _master: *mut device,
    _data: *mut c_void,
) -> c_int {
    unsafe {
        pm_runtime_set_autosuspend_delay(dev, 3000);
        pm_runtime_use_autosuspend(dev);
        pm_runtime_mark_last_busy(dev);
        pm_runtime_set_active(dev);
        pm_runtime_enable(dev);
    }

    0
}

unsafe extern "C" fn wcd_sdw_component_unbind(
    dev: *mut device,
    _master: *mut device,
    _data: *mut c_void,
) {
    unsafe {
        pm_runtime_disable(dev);
        pm_runtime_set_suspended(dev);
        pm_runtime_dont_use_autosuspend(dev);
    }
}

#[unsafe(no_mangle)]
pub static wcd_sdw_component_ops: component_ops = component_ops {
    bind: Some(wcd_sdw_component_bind),
    unbind: Some(wcd_sdw_component_unbind),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcd_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let regmap = unsafe { dev_get_regmap(&mut (*slave).dev as *mut device, core::ptr::null()) };

    if !regmap.is_null() && status == SDW_SLAVE_ATTACHED {
        /* Write out any cached changes that happened between probe and attach */
        unsafe {
            regcache_cache_only(regmap, false);
            return regcache_sync(regmap);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcd_bus_config(
    slave: *mut sdw_slave,
    params: *mut sdw_bus_params,
) -> c_int {
    unsafe {
        sdw_write(
            slave,
            SWRS_SCP_HOST_CLK_DIV2_CTL_BANK((*params).next_bank),
            0x01,
        );
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcd_interrupt_callback(
    slave: *mut sdw_slave,
    slave_irq: *mut irq_domain,
    wcd_intr_status0: c_uint,
    wcd_intr_status1: c_uint,
    wcd_intr_status2: c_uint,
) -> c_int {
    let regmap = unsafe { dev_get_regmap(&mut (*slave).dev as *mut device, core::ptr::null()) };
    let mut sts1: u32;
    let mut sts2: u32;
    let mut sts3: u32;

    loop {
        unsafe {
            handle_nested_irq(irq_find_mapping(slave_irq, 0));
            regmap_read(regmap, wcd_intr_status0, &mut sts1 as *mut u32);
            regmap_read(regmap, wcd_intr_status1, &mut sts2 as *mut u32);
            regmap_read(regmap, wcd_intr_status2, &mut sts3 as *mut u32);
        }

        if !(sts1 != 0 || sts2 != 0 || sts3 != 0) {
            break;
        }
    }

    IRQ_HANDLED
}

// MODULE_DESCRIPTION("Common Qualcomm WCD Codec helpers driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
