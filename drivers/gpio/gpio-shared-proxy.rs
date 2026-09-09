// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Linaro Ltd.
 */

// Kernel headers and "gpiolib-shared.h" provide the external types, constants,
// functions, macros, and logging facilities referenced below.

#[repr(C)]
pub struct gpio_shared_proxy_data {
    pub gc: gpio_chip,
    pub shared_desc: *mut gpio_shared_desc,
    pub dev: *mut device,
    pub voted_change: bool,
}

unsafe fn gpio_shared_proxy_set_unlocked(
    proxy: *mut gpio_shared_proxy_data,
    value: c_int,
) -> c_int {
    let shared_desc = (*proxy).shared_desc;
    let desc = (*shared_desc).desc;
    let mut ret: c_int = 0;

    // lockdep_assert_held(&shared_desc->mutex);

    if value != (*shared_desc).def_val {
        /* User wants to vote for a value change. */
        if (*proxy).voted_change {
            /* Already voted for a change, nothing to do. */
            goto_out!();
        }

        /* Haven't voted for a value change yet. */
        if (*shared_desc).votecnt == 0 {
            /* Current value is default, need to actually set value
             * to the opposite. */
            ret = gpiod_set_value_cansleep(desc, value);
            if ret != 0 {
                goto_out!();
            }
        }

        (*shared_desc).votecnt += 1;
        (*proxy).voted_change = true;
        goto_out!();
    }

    /* Desired value is the default. */
    if !(*proxy).voted_change {
        /* We didn't vote for change previously, nothing to do. */
        goto_out!();
    }

    /* We previously voted for change. */
    if (*shared_desc).votecnt == 1 {
        /* This is the last remaining vote for change, set value to default. */
        ret = gpiod_set_value_cansleep(desc, (*shared_desc).def_val);
        if ret != 0 {
            goto_out!();
        }
    }

    (*shared_desc).votecnt -= 1;
    (*proxy).voted_change = false;

    goto_out!();
    ret
}

unsafe fn gpio_shared_proxy_request(gc: *mut gpio_chip, _offset: c_uint) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    let shared_desc = (*proxy).shared_desc;

    // guard(mutex)(&shared_desc->mutex);
    (*proxy).shared_desc.usecnt += 1;
    dev_dbg((*proxy).dev, "Shared GPIO requested, number of users: %u\n", (*proxy).shared_desc.usecnt);
    0
}

unsafe fn gpio_shared_proxy_free(gc: *mut gpio_chip, _offset: c_uint) {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    let shared_desc = (*proxy).shared_desc;
    let mut ret: c_int;

    // guard(mutex)(&shared_desc->mutex);
    if (*proxy).voted_change {
        ret = gpio_shared_proxy_set_unlocked(proxy, (*shared_desc).def_val);
        if ret != 0 {
            dev_err((*proxy).dev, "Failed to unset the shared GPIO value on release: %d\n", ret);
        }
    }
    (*proxy).shared_desc.usecnt -= 1;
    dev_dbg((*proxy).dev, "Shared GPIO freed, number of users: %u\n", (*proxy).shared_desc.usecnt);
}

unsafe fn gpio_shared_proxy_set_config(gc: *mut gpio_chip, _offset: c_uint, cfg: c_ulong) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    let shared_desc = (*proxy).shared_desc;
    let desc = (*shared_desc).desc;
    // guard(mutex)(&shared_desc->mutex);
    if (*shared_desc).usecnt > 1 {
        if (*shared_desc).cfg != cfg {
            dev_dbg((*proxy).dev, "Shared GPIO's configuration already set, accepting changes but users may conflict!!\n");
        } else {
            dev_dbg((*proxy).dev, "Equal config requested, nothing to do\n");
            return 0;
        }
    }
    let ret = gpiod_set_config(desc, cfg);
    if ret != 0 && ret != -ENOTSUPP { return ret; }
    (*shared_desc).cfg = cfg;
    0
}

unsafe fn gpio_shared_proxy_direction_input(gc: *mut gpio_chip, _offset: c_uint) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    let shared_desc = (*proxy).shared_desc;
    let desc = (*shared_desc).desc;
    // guard(mutex)(&shared_desc->mutex);
    if (*shared_desc).usecnt == 1 {
        dev_dbg((*proxy).dev, "Only one user of this shared GPIO, allowing to set direction to input\n");
        return gpiod_direction_input(desc);
    }
    let dir = gpiod_get_direction(desc);
    if dir < 0 { return dir; }
    if dir == GPIO_LINE_DIRECTION_OUT {
        dev_dbg((*proxy).dev, "Shared GPIO's direction already set to output, refusing to change\n");
        return -EPERM;
    }
    0
}

unsafe fn gpio_shared_proxy_direction_output(gc: *mut gpio_chip, _offset: c_uint, value: c_int) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    let shared_desc = (*proxy).shared_desc;
    let desc = (*shared_desc).desc;
    // guard(mutex)(&shared_desc->mutex);
    if (*shared_desc).usecnt == 1 {
        dev_dbg((*proxy).dev, "Only one user of this shared GPIO, allowing to set direction to output with value '%s'\n", str_high_low(value));
        let ret = gpiod_direction_output(desc, value);
        if ret != 0 { return ret; }
        (*shared_desc).def_val = value;
        (*shared_desc).votecnt = 0;
        (*proxy).voted_change = false;
        return 0;
    }
    let dir = gpiod_get_direction(desc);
    if dir < 0 { return dir; }
    if dir == GPIO_LINE_DIRECTION_IN {
        dev_dbg((*proxy).dev, "Shared GPIO's direction already set to input, refusing to change\n");
        return -EPERM;
    }
    gpio_shared_proxy_set_unlocked(proxy, value)
}

unsafe fn gpio_shared_proxy_get_cansleep(gc: *mut gpio_chip, _offset: c_uint) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    gpiod_get_value_cansleep((*proxy).shared_desc.desc)
}

unsafe fn gpio_shared_proxy_set_cansleep(gc: *mut gpio_chip, _offset: c_uint, value: c_int) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    // guard(mutex)(&proxy->shared_desc->mutex);
    gpio_shared_proxy_set_unlocked(proxy, value)
}

unsafe fn gpio_shared_proxy_get_direction(gc: *mut gpio_chip, _offset: c_uint) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    gpiod_get_direction((*proxy).shared_desc.desc)
}

unsafe fn gpio_shared_proxy_to_irq(gc: *mut gpio_chip, _offset: c_uint) -> c_int {
    let proxy = gpiochip_get_data(gc) as *mut gpio_shared_proxy_data;
    gpiod_to_irq((*proxy).shared_desc.desc)
}

unsafe fn gpio_shared_proxy_probe(adev: *mut auxiliary_device, _id: *const auxiliary_device_id) -> c_int {
    let dev = &mut (*adev).dev as *mut device;
    let shared_desc = devm_gpiod_shared_get(dev);
    if IS_ERR(shared_desc) { return PTR_ERR(shared_desc); }
    let proxy = devm_kzalloc(dev, core::mem::size_of::<gpio_shared_proxy_data>(), GFP_KERNEL) as *mut gpio_shared_proxy_data;
    if proxy.is_null() { return -ENOMEM; }
    (*proxy).shared_desc = shared_desc;
    (*proxy).dev = dev;
    let gc = &mut (*proxy).gc as *mut gpio_chip;
    (*gc).base = -1;
    (*gc).ngpio = 1;
    (*gc).label = dev_name(dev);
    (*gc).parent = dev;
    (*gc).owner = THIS_MODULE;
    // The proxy is always sleeping because descriptor/pinctrl paths may lock mutexes.
    (*gc).can_sleep = true;
    (*gc).request = Some(gpio_shared_proxy_request);
    (*gc).free = Some(gpio_shared_proxy_free);
    (*gc).set_config = Some(gpio_shared_proxy_set_config);
    (*gc).direction_input = Some(gpio_shared_proxy_direction_input);
    (*gc).direction_output = Some(gpio_shared_proxy_direction_output);
    (*gc).set = Some(gpio_shared_proxy_set_cansleep);
    (*gc).get = Some(gpio_shared_proxy_get_cansleep);
    (*gc).get_direction = Some(gpio_shared_proxy_get_direction);
    (*gc).to_irq = Some(gpio_shared_proxy_to_irq);
    devm_gpiochip_add_data(dev, gc, proxy)
}

// Auxiliary-device ID table and driver registration are supplied by the kernel bindings.
const GPIO_SHARED_PROXY_ID_NAME: &str = "gpiolib_shared.proxy";
const GPIO_SHARED_PROXY_DRIVER_NAME: &str = "gpio-shared-proxy";
const GPIO_SHARED_PROXY_DESCRIPTION: &str = "Shared GPIO mux driver.";
const GPIO_SHARED_PROXY_LICENSE: &str = "GPL";
const GPIO_SHARED_PROXY_AUTHOR: &str = "Bartosz Golaszewski <bartosz.golaszewski@linaro.org>";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
