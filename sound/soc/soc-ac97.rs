// SPDX-License-Identifier: GPL-2.0+
//
// soc-ac97.c  --  ALSA SoC Audio Layer AC97 support
//
// Copyright 2005 Wolfson Microelectronics PLC.
// Copyright 2005 Openedhand Ltd.
// Copyright (C) 2010 Slimlogic Ltd.
// Copyright (C) 2010 Texas Instruments Inc.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//         with code, comments and ideas from :-
//         Richard Purdie <richard@openedhand.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const AC97_NUM_GPIOS: c_uint = 8;
const AC97_GPIO_CFG: c_uint = 0x4c;
const AC97_GPIO_STATUS: c_uint = 0x54;
const GPIOD_ASIS: c_uint = 0;

#[repr(C)]
pub struct device {
    pub bus: *mut bus_type,
    pub parent: *mut device,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub ops: *mut snd_ac97_bus_ops,
}

#[repr(C)]
pub struct snd_ac97 {
    pub bus: *mut snd_ac97_bus,
    pub num: c_uint,
    pub dev: device,
    pub gpio_priv: *mut snd_ac97_gpio_priv,
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub can_sleep: c_int,
    pub ngpio: c_uint,
    pub parent: *mut device,
    pub base: c_int,
}

#[repr(C)]
pub struct snd_ac97_reset_cfg {
    pub pctl: *mut pinctrl,
    pub pstate_reset: *mut pinctrl_state,
    pub pstate_warm_reset: *mut pinctrl_state,
    pub pstate_run: *mut pinctrl_state,
    pub reset_gpio: *mut gpio_desc,
    pub sdata_gpio: *mut gpio_desc,
    pub sync_gpio: *mut gpio_desc,
}

#[repr(C)]
pub struct snd_ac97_gpio_priv {
    pub gpio_chip: gpio_chip,
    pub gpios_set: c_uint,
    pub component: *mut snd_soc_component,
}

unsafe extern "C" {
    static mut ac97_bus_type: bus_type;
    static mut THIS_MODULE: *mut c_void;

    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn to_ac97_t(dev: *mut device) -> *mut snd_ac97;
    fn ERR_PTR(error: isize) -> *mut snd_ac97;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn device_initialize(dev: *mut device);
    fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn put_device(dev: *mut device);
    fn snd_ac97_reset(ac97: *mut snd_ac97, try_warm: bool_t, id: c_uint, id_mask: c_uint) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn gpiochip_remove(chip: *mut gpio_chip);
    fn pinctrl_select_state(p: *mut pinctrl, state: *mut pinctrl_state) -> c_int;
    fn gpiod_direction_output_raw(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    fn pinctrl_lookup_state(p: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    fn devm_gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        idx: c_uint,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
}

static mut soc_ac97_bus: snd_ac97_bus = snd_ac97_bus {
    ops: ptr::null_mut(), /* Gets initialized in snd_soc_set_ac97_ops() */
};

unsafe extern "C" fn soc_ac97_device_release(dev: *mut device) {
    unsafe {
        kfree(to_ac97_t(dev) as *mut c_void);
    }
}

// CONFIG_GPIOLIB
unsafe fn gpio_to_component(chip: *mut gpio_chip) -> *mut snd_soc_component {
    let gpio_priv = unsafe { gpiochip_get_data(chip) as *mut snd_ac97_gpio_priv };

    unsafe { (*gpio_priv).component }
}

unsafe extern "C" fn snd_soc_ac97_gpio_request(
    _chip: *mut gpio_chip,
    offset: c_uint,
) -> c_int {
    if offset >= AC97_NUM_GPIOS {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn snd_soc_ac97_gpio_direction_in(
    chip: *mut gpio_chip,
    offset: c_uint,
) -> c_int {
    let component = unsafe { gpio_to_component(chip) };

    unsafe {
        dev_dbg(
            (*component).dev,
            c"set gpio %d to output\n".as_ptr(),
            offset,
        );
        snd_soc_component_update_bits(
            component,
            AC97_GPIO_CFG,
            1u32 << offset,
            1u32 << offset,
        )
    }
}

unsafe extern "C" fn snd_soc_ac97_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let component = unsafe { gpio_to_component(chip) };
    let ret: c_int;

    unsafe {
        ret = snd_soc_component_read(component, AC97_GPIO_STATUS);

        dev_dbg(
            (*component).dev,
            c"get gpio %d : %d\n".as_ptr(),
            offset,
            ret & ((1u32 << offset) as c_int),
        );
    }

    ((ret & ((1u32 << offset) as c_int)) != 0) as c_int
}

unsafe extern "C" fn snd_soc_ac97_gpio_set(
    chip: *mut gpio_chip,
    offset: c_uint,
    value: c_int,
) -> c_int {
    let gpio_priv = unsafe { gpiochip_get_data(chip) as *mut snd_ac97_gpio_priv };
    let component = unsafe { gpio_to_component(chip) };

    unsafe {
        (*gpio_priv).gpios_set &= !(1u32 << offset);
        (*gpio_priv).gpios_set |= ((value != 0) as c_uint) << offset;
        snd_soc_component_write(component, AC97_GPIO_STATUS, (*gpio_priv).gpios_set);
        dev_dbg(
            (*component).dev,
            c"set gpio %d to %d\n".as_ptr(),
            offset,
            (value != 0) as c_int,
        );
    }

    0
}

unsafe extern "C" fn snd_soc_ac97_gpio_direction_out(
    chip: *mut gpio_chip,
    offset: c_uint,
    value: c_int,
) -> c_int {
    let component = unsafe { gpio_to_component(chip) };
    let ret: c_int;

    unsafe {
        dev_dbg(
            (*component).dev,
            c"set gpio %d to output\n".as_ptr(),
            offset,
        );

        ret = snd_soc_ac97_gpio_set(chip, offset, value);
        if ret != 0 {
            return ret;
        }

        snd_soc_component_update_bits(component, AC97_GPIO_CFG, 1u32 << offset, 0)
    }
}

static mut snd_soc_ac97_gpio_chip: gpio_chip = gpio_chip {
    label: c"snd_soc_ac97".as_ptr(),
    owner: unsafe { THIS_MODULE },
    request: Some(snd_soc_ac97_gpio_request),
    direction_input: Some(snd_soc_ac97_gpio_direction_in),
    get: Some(snd_soc_ac97_gpio_get),
    direction_output: Some(snd_soc_ac97_gpio_direction_out),
    set: Some(snd_soc_ac97_gpio_set),
    can_sleep: 1,
    ngpio: 0,
    parent: ptr::null_mut(),
    base: 0,
};

unsafe fn snd_soc_ac97_init_gpio(
    ac97: *mut snd_ac97,
    component: *mut snd_soc_component,
) -> c_int {
    let gpio_priv: *mut snd_ac97_gpio_priv;
    let ret: c_int;

    unsafe {
        gpio_priv = devm_kzalloc(
            (*component).dev,
            mem::size_of::<snd_ac97_gpio_priv>(),
            GFP_KERNEL,
        ) as *mut snd_ac97_gpio_priv;
        if gpio_priv.is_null() {
            return -ENOMEM;
        }
        (*ac97).gpio_priv = gpio_priv;
        (*gpio_priv).component = component;
        ptr::copy_nonoverlapping(&raw const snd_soc_ac97_gpio_chip, &mut (*gpio_priv).gpio_chip, 1);
        (*gpio_priv).gpio_chip.ngpio = AC97_NUM_GPIOS;
        (*gpio_priv).gpio_chip.parent = (*component).dev;
        (*gpio_priv).gpio_chip.base = -1;

        ret = gpiochip_add_data(&mut (*gpio_priv).gpio_chip, gpio_priv as *mut c_void);
        if ret != 0 {
            dev_err((*component).dev, c"Failed to add GPIOs: %d\n".as_ptr(), ret);
        }
        ret
    }
}

unsafe fn snd_soc_ac97_free_gpio(ac97: *mut snd_ac97) {
    unsafe {
        gpiochip_remove(&mut (*(*ac97).gpio_priv).gpio_chip);
    }
}

// Without CONFIG_GPIOLIB, snd_soc_ac97_init_gpio() returns 0 and
// snd_soc_ac97_free_gpio() is empty.

/**
 * snd_soc_alloc_ac97_component() - Allocate new a AC'97 device
 * @component: The COMPONENT for which to create the AC'97 device
 *
 * Allocated a new snd_ac97 device and intializes it, but does not yet register
 * it. The caller is responsible to either call device_add(&ac97->dev) to
 * register the device, or to call put_device(&ac97->dev) to free the device.
 *
 * Returns: A snd_ac97 device or an ERR_PTR in case of an error.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_alloc_ac97_component(
    component: *mut snd_soc_component,
) -> *mut snd_ac97 {
    let ac97: *mut snd_ac97;

    unsafe {
        ac97 = kzalloc(mem::size_of::<snd_ac97>(), GFP_KERNEL) as *mut snd_ac97;
        if ac97.is_null() {
            return ERR_PTR(-ENOMEM as isize);
        }

        (*ac97).bus = &raw mut soc_ac97_bus;
        (*ac97).num = 0;

        (*ac97).dev.bus = &raw mut ac97_bus_type;
        (*ac97).dev.parent = (*(*component).card).dev;
        (*ac97).dev.release = Some(soc_ac97_device_release);

        dev_set_name(
            &mut (*ac97).dev,
            c"%d-%d:%s".as_ptr(),
            (*(*(*component).card).snd_card).number,
            0,
            (*component).name,
        );

        device_initialize(&mut (*ac97).dev);

        ac97
    }
}

/**
 * snd_soc_new_ac97_component - initailise AC97 device
 * @component: audio component
 * @id: The expected device ID
 * @id_mask: Mask that is applied to the device ID before comparing with @id
 *
 * Initialises AC97 component resources for use by ad-hoc devices only.
 *
 * If @id is not 0 this function will reset the device, then read the ID from
 * the device and check if it matches the expected ID. If it doesn't match an
 * error will be returned and device will not be registered.
 *
 * Returns: An ERR_PTR on failure or a valid snd_ac97 struct on success.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_new_ac97_component(
    component: *mut snd_soc_component,
    id: c_uint,
    id_mask: c_uint,
) -> *mut snd_ac97 {
    let ac97: *mut snd_ac97;
    let mut ret: c_int;

    unsafe {
        ac97 = snd_soc_alloc_ac97_component(component);
        if IS_ERR(ac97 as *const c_void) {
            return ac97;
        }

        if id != 0 {
            ret = snd_ac97_reset(ac97, false, id, id_mask);
            if ret < 0 {
                dev_err(
                    (*component).dev,
                    c"Failed to reset AC97 device: %d\n".as_ptr(),
                    ret,
                );
                put_device(&mut (*ac97).dev);
                return ERR_PTR(ret as isize);
            }
        }

        ret = device_add(&mut (*ac97).dev);
        if ret != 0 {
            put_device(&mut (*ac97).dev);
            return ERR_PTR(ret as isize);
        }

        ret = snd_soc_ac97_init_gpio(ac97, component);
        if ret != 0 {
            put_device(&mut (*ac97).dev);
            return ERR_PTR(ret as isize);
        }

        ac97
    }
}

/**
 * snd_soc_free_ac97_component - free AC97 component device
 * @ac97: snd_ac97 device to be freed
 *
 * Frees AC97 component device resources.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_free_ac97_component(ac97: *mut snd_ac97) {
    unsafe {
        snd_soc_ac97_free_gpio(ac97);
        device_del(&mut (*ac97).dev);
        (*ac97).bus = ptr::null_mut();
        put_device(&mut (*ac97).dev);
    }
}

static mut snd_ac97_rst_cfg: snd_ac97_reset_cfg = snd_ac97_reset_cfg {
    pctl: ptr::null_mut(),
    pstate_reset: ptr::null_mut(),
    pstate_warm_reset: ptr::null_mut(),
    pstate_run: ptr::null_mut(),
    reset_gpio: ptr::null_mut(),
    sdata_gpio: ptr::null_mut(),
    sync_gpio: ptr::null_mut(),
};

unsafe extern "C" fn snd_soc_ac97_warm_reset(_ac97: *mut snd_ac97) {
    unsafe {
        let pctl = snd_ac97_rst_cfg.pctl;

        pinctrl_select_state(pctl, snd_ac97_rst_cfg.pstate_warm_reset);

        gpiod_direction_output_raw(snd_ac97_rst_cfg.sync_gpio, 1);

        udelay(10);

        gpiod_direction_output_raw(snd_ac97_rst_cfg.sync_gpio, 0);

        pinctrl_select_state(pctl, snd_ac97_rst_cfg.pstate_run);
        msleep(2);
    }
}

unsafe extern "C" fn snd_soc_ac97_reset(_ac97: *mut snd_ac97) {
    unsafe {
        let pctl = snd_ac97_rst_cfg.pctl;

        pinctrl_select_state(pctl, snd_ac97_rst_cfg.pstate_reset);

        gpiod_direction_output_raw(snd_ac97_rst_cfg.sync_gpio, 0);
        gpiod_direction_output_raw(snd_ac97_rst_cfg.sdata_gpio, 0);
        gpiod_direction_output_raw(snd_ac97_rst_cfg.reset_gpio, 0);

        udelay(10);

        gpiod_direction_output_raw(snd_ac97_rst_cfg.reset_gpio, 1);

        pinctrl_select_state(pctl, snd_ac97_rst_cfg.pstate_run);
        msleep(2);
    }
}

unsafe fn snd_soc_ac97_parse_pinctl(
    dev: *mut device,
    cfg: *mut snd_ac97_reset_cfg,
) -> c_int {
    let mut p: *mut pinctrl;
    let mut state: *mut pinctrl_state;

    unsafe {
        p = devm_pinctrl_get(dev);
        if IS_ERR(p as *const c_void) {
            dev_err(dev, c"Failed to get pinctrl\n".as_ptr());
            return PTR_ERR(p as *const c_void);
        }
        (*cfg).pctl = p;

        state = pinctrl_lookup_state(p, c"ac97-reset".as_ptr());
        if IS_ERR(state as *const c_void) {
            dev_err(dev, c"Can't find pinctrl state ac97-reset\n".as_ptr());
            return PTR_ERR(state as *const c_void);
        }
        (*cfg).pstate_reset = state;

        state = pinctrl_lookup_state(p, c"ac97-warm-reset".as_ptr());
        if IS_ERR(state as *const c_void) {
            dev_err(dev, c"Can't find pinctrl state ac97-warm-reset\n".as_ptr());
            return PTR_ERR(state as *const c_void);
        }
        (*cfg).pstate_warm_reset = state;

        state = pinctrl_lookup_state(p, c"ac97-running".as_ptr());
        if IS_ERR(state as *const c_void) {
            dev_err(dev, c"Can't find pinctrl state ac97-running\n".as_ptr());
            return PTR_ERR(state as *const c_void);
        }
        (*cfg).pstate_run = state;

        (*cfg).sync_gpio = devm_gpiod_get_index(dev, c"ac97".as_ptr(), 0, GPIOD_ASIS);
        if IS_ERR((*cfg).sync_gpio as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*cfg).sync_gpio as *const c_void),
                c"Can't find ac97-sync gpio\n".as_ptr(),
            );
        }
        gpiod_set_consumer_name((*cfg).sync_gpio, c"AC97 link sync".as_ptr());

        (*cfg).sdata_gpio = devm_gpiod_get_index(dev, c"ac97".as_ptr(), 1, GPIOD_ASIS);
        if IS_ERR((*cfg).sdata_gpio as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*cfg).sdata_gpio as *const c_void),
                c"Can't find ac97-sdata gpio\n".as_ptr(),
            );
        }
        gpiod_set_consumer_name((*cfg).sdata_gpio, c"AC97 link sdata".as_ptr());

        (*cfg).reset_gpio = devm_gpiod_get_index(dev, c"ac97".as_ptr(), 2, GPIOD_ASIS);
        if IS_ERR((*cfg).reset_gpio as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*cfg).reset_gpio as *const c_void),
                c"Can't find ac97-reset gpio\n".as_ptr(),
            );
        }
        gpiod_set_consumer_name((*cfg).reset_gpio, c"AC97 link reset".as_ptr());

        0
    }
}

#[no_mangle]
pub static mut soc_ac97_ops: *mut snd_ac97_bus_ops = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> c_int {
    unsafe {
        if ops == soc_ac97_ops {
            return 0;
        }

        if !soc_ac97_ops.is_null() && !ops.is_null() {
            return -EBUSY;
        }

        soc_ac97_ops = ops;
        soc_ac97_bus.ops = ops;

        0
    }
}

/**
 * snd_soc_set_ac97_ops_of_reset - Set ac97 ops with generic ac97 reset functions
 * @ops: bus ops
 * @pdev: platform device
 *
 * This function sets the reset and warm_reset properties of ops and parses
 * the device node of pdev to get pinctrl states and gpio numbers to use.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_set_ac97_ops_of_reset(
    ops: *mut snd_ac97_bus_ops,
    pdev: *mut platform_device,
) -> c_int {
    let dev: *mut device;
    let mut cfg: snd_ac97_reset_cfg = unsafe { mem::zeroed() };
    let mut ret: c_int;

    unsafe {
        dev = &mut (*pdev).dev;
        ret = snd_soc_ac97_parse_pinctl(dev, &mut cfg);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_set_ac97_ops(ops);
        if ret != 0 {
            return ret;
        }

        (*ops).warm_reset = Some(snd_soc_ac97_warm_reset);
        (*ops).reset = Some(snd_soc_ac97_reset);

        snd_ac97_rst_cfg = cfg;
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
