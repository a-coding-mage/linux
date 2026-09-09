// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024 Google LLC
 *
 * This driver provides the ability to control GPIOs on the Chrome OS EC.
 * There isn't any direction control, and setting values on GPIOs is only
 * possible when the system is unlocked.
 */

// Linux kernel dependencies and build-time declarations are supplied by the
// surrounding Rust kernel environment.

/* Prefix all names to avoid collisions with EC <-> AP nets */
static CROS_EC_GPIO_PREFIX: &[u8] = b"EC:\0";

/* Setting gpios is only supported when the system is unlocked */
unsafe fn cros_ec_gpio_set(
    gc: *mut gpio_chip,
    gpio: c_uint,
    val: c_int,
) -> c_int {
    let name = (*(*gc).names.add(gpio as usize)).add(CROS_EC_GPIO_PREFIX.len() - 1);
    let cros_ec = gpiochip_get_data(gc);
    let mut params: ec_params_gpio_set = core::mem::zeroed();
    params.val = val;
    let copied = strscpy(
        params.name.as_mut_ptr(),
        name,
        core::mem::size_of_val(&params.name),
    );
    if copied < 0 {
        return copied as c_int;
    }
    cros_ec_cmd(
        cros_ec,
        0,
        EC_CMD_GPIO_SET,
        &params as *const _ as *const c_void,
        core::mem::size_of_val(&params),
        core::ptr::null_mut(),
        0,
    )
}

unsafe fn cros_ec_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let name = (*(*gc).names.add(gpio as usize)).add(CROS_EC_GPIO_PREFIX.len() - 1);
    let cros_ec = gpiochip_get_data(gc);
    let mut params: ec_params_gpio_get = core::mem::zeroed();
    let mut response: ec_response_gpio_get = core::mem::zeroed();
    let copied = strscpy(
        params.name.as_mut_ptr(),
        name,
        core::mem::size_of_val(&params.name),
    );
    if copied < 0 {
        return -EINVAL;
    }
    let ret = cros_ec_cmd(
        cros_ec,
        0,
        EC_CMD_GPIO_GET,
        &params as *const _ as *const c_void,
        core::mem::size_of_val(&params),
        &mut response as *mut _ as *mut c_void,
        core::mem::size_of_val(&response),
    );
    if ret < 0 {
        dev_err((*gc).parent, "error getting gpio%d (%s) on EC: %d\n", gpio, name, ret);
        return ret;
    }
    response.val
}

const CROS_EC_GPIO_INPUT: u32 = 1u32 << 8;
const CROS_EC_GPIO_OUTPUT: u32 = 1u32 << 9;

unsafe fn cros_ec_gpio_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let name = (*(*gc).names.add(gpio as usize)).add(CROS_EC_GPIO_PREFIX.len() - 1);
    let cros_ec = gpiochip_get_data(gc);
    let mut params: ec_params_gpio_get_v1 = core::mem::zeroed();
    params.subcmd = EC_GPIO_GET_INFO;
    params.get_info.index = gpio;
    let mut response: ec_response_gpio_get_v1 = core::mem::zeroed();
    let ret = cros_ec_cmd(cros_ec, 1, EC_CMD_GPIO_GET, &params as *const _ as *const c_void,
                          core::mem::size_of_val(&params), &mut response as *mut _ as *mut c_void,
                          core::mem::size_of_val(&response));
    if ret < 0 {
        dev_err((*gc).parent, "error getting direction of gpio%d (%s) on EC: %d\n", gpio, name, ret);
        return ret;
    }
    if response.get_info.flags & CROS_EC_GPIO_INPUT != 0 { return GPIO_LINE_DIRECTION_IN; }
    if response.get_info.flags & CROS_EC_GPIO_OUTPUT != 0 { return GPIO_LINE_DIRECTION_OUT; }
    -EINVAL
}

/* Query EC for all gpio line names */
unsafe fn cros_ec_gpio_init_names(cros_ec: *mut cros_ec_device, gc: *mut gpio_chip) -> c_int {
    let mut params: ec_params_gpio_get_v1 = core::mem::zeroed();
    params.subcmd = EC_GPIO_GET_INFO;
    let mut response: ec_response_gpio_get_v1 = core::mem::zeroed();
    let mut ret: c_int;
    let name_len = CROS_EC_GPIO_PREFIX.len() - 1 + core::mem::size_of_val(&response.get_info.name) + 1;
    let mut names = devm_kcalloc((*gc).parent, (*gc).ngpio as usize, core::mem::size_of::<*const c_char>(), GFP_KERNEL) as *mut *const c_char;
    if names.is_null() { return -ENOMEM; }
    (*gc).names = names;
    let mut str_ptr = devm_kcalloc((*gc).parent, (*gc).ngpio as usize, name_len, GFP_KERNEL) as *mut c_char;
    if str_ptr.is_null() { return -ENOMEM; }
    /* Get gpio line names one at a time */
    for i in 0..(*gc).ngpio {
        params.get_info.index = i;
        ret = cros_ec_cmd(cros_ec, 1, EC_CMD_GPIO_GET, &params as *const _ as *const c_void,
                          core::mem::size_of_val(&params), &mut response as *mut _ as *mut c_void,
                          core::mem::size_of_val(&response));
        if ret < 0 {
            dev_err_probe((*gc).parent, ret, "error getting gpio%d info\n", i);
            return ret;
        }
        *names.add(i as usize) = str_ptr;
        let copied = scnprintf(str_ptr, name_len, "%s%s", CROS_EC_GPIO_PREFIX.as_ptr(), response.get_info.name.as_ptr());
        if copied < 0 { return copied; }
        str_ptr = str_ptr.add(copied as usize + 1);
    }
    0
}

/* Query EC for number of gpios */
unsafe fn cros_ec_gpio_ngpios(cros_ec: *mut cros_ec_device) -> c_int {
    let mut params: ec_params_gpio_get_v1 = core::mem::zeroed();
    params.subcmd = EC_GPIO_GET_COUNT;
    let mut response: ec_response_gpio_get_v1 = core::mem::zeroed();
    let ret = cros_ec_cmd(cros_ec, 1, EC_CMD_GPIO_GET, &params as *const _ as *const c_void,
                          core::mem::size_of_val(&params), &mut response as *mut _ as *mut c_void,
                          core::mem::size_of_val(&response));
    if ret < 0 { return ret; }
    response.get_count.val
}

unsafe fn cros_ec_gpio_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let parent = dev.parent;
    let ec_dev = dev_get_drvdata(parent) as *mut cros_ec_dev;
    let cros_ec = (*ec_dev).ec_dev;
    device_set_node(dev, dev_fwnode((*cros_ec).dev));
    let ngpios = cros_ec_gpio_ngpios(cros_ec);
    if ngpios < 0 { dev_err_probe(dev, ngpios, "error getting gpio count\n"); return ngpios; }
    let gc = devm_kzalloc(dev, core::mem::size_of::<gpio_chip>(), GFP_KERNEL) as *mut gpio_chip;
    if gc.is_null() { return -ENOMEM; }
    (*gc).ngpio = ngpios as c_uint;
    (*gc).parent = dev;
    let ret = cros_ec_gpio_init_names(cros_ec, gc);
    if ret != 0 { return ret; }
    (*gc).can_sleep = true;
    (*gc).label = dev_name(dev);
    (*gc).base = -1;
    (*gc).set = Some(cros_ec_gpio_set);
    (*gc).get = Some(cros_ec_gpio_get);
    (*gc).get_direction = Some(cros_ec_gpio_get_direction);
    devm_gpiochip_add_data(dev, gc, cros_ec)
}

static CROS_EC_GPIO_ID: [platform_device_id; 2] = [
    platform_device_id { name: b"cros-ec-gpio\0".as_ptr() as *const c_char },
    platform_device_id { name: core::ptr::null() },
];

static mut CROS_EC_GPIO_DRIVER: platform_driver = platform_driver {
    probe: Some(cros_ec_gpio_probe),
    driver: device_driver { name: b"cros-ec-gpio\0".as_ptr() as *const c_char },
    id_table: CROS_EC_GPIO_ID.as_ptr(),
};

module_platform_driver!(CROS_EC_GPIO_DRIVER);

module_description!("ChromeOS EC GPIO Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
