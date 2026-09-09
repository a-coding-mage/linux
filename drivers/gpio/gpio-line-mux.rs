// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO line mux which acts as virtual gpiochip and provides a 1-to-many
 * mapping between virtual GPIOs and a real GPIO + multiplexer.
 *
 * Copyright (c) 2025 Jonas Jelonek <jelonek.jonas@gmail.com>
 */

// Dependencies supplied by the Linux kernel bindings.

const MUX_SELECT_DELAY_US: u32 = 100;

#[repr(C)]
struct gpio_lmux {
    gc: gpio_chip,
    mux: *mut mux_control,
    muxed_gpio: *mut gpio_desc,

    num_gpio_mux_states: u32,
    // Flexible array member: storage is allocated for num_gpio_mux_states entries.
    gpio_mux_states: [u32; 0],
}

unsafe fn gpio_lmux_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let glm: *mut gpio_lmux = gpiochip_get_data(gc);
    let mut ret: i32;

    ret = mux_control_select_delay(
        (*glm).mux,
        *(*glm).gpio_mux_states.as_ptr().add(offset as usize),
        MUX_SELECT_DELAY_US,
    );
    if ret < 0 {
        return ret;
    }

    ret = gpiod_get_raw_value_cansleep((*glm).muxed_gpio);
    mux_control_deselect((*glm).mux);
    ret
}

unsafe fn gpio_lmux_gpio_get_direction(
    _gc: *mut gpio_chip,
    _offset: u32,
) -> i32 {
    GPIO_LINE_DIRECTION_IN
}

unsafe fn gpio_lmux_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let glm: *mut gpio_lmux;
    let ngpio: u32;
    let size: usize;
    let mut ret: i32;

    ngpio = device_property_count_u32(dev, c"gpio-line-mux-states");
    if ngpio == 0 {
        return -EINVAL;
    }

    size = struct_size_gpio_lmux_gpio_mux_states(ngpio);
    glm = devm_kzalloc(dev, size, GFP_KERNEL);
    if glm.is_null() {
        return -ENOMEM;
    }

    (*glm).gc.base = -1;
    (*glm).gc.can_sleep = true;
    (*glm).gc.fwnode = dev_fwnode(dev);
    (*glm).gc.label = dev_name(dev);
    (*glm).gc.ngpio = ngpio;
    (*glm).gc.owner = THIS_MODULE;
    (*glm).gc.parent = dev;

    (*glm).gc.get = Some(gpio_lmux_gpio_get);
    (*glm).gc.get_direction = Some(gpio_lmux_gpio_get_direction);

    (*glm).mux = devm_mux_control_get(dev, core::ptr::null());
    if IS_ERR((*glm).mux) {
        return dev_err_probe(
            dev,
            PTR_ERR((*glm).mux),
            c"could not get mux controller\n",
        );
    }

    (*glm).muxed_gpio = devm_gpiod_get(dev, c"muxed", GPIOD_IN);
    if IS_ERR((*glm).muxed_gpio) {
        return dev_err_probe(
            dev,
            PTR_ERR((*glm).muxed_gpio),
            c"could not get muxed-gpio\n",
        );
    }

    (*glm).num_gpio_mux_states = ngpio;
    ret = device_property_read_u32_array(
        dev,
        c"gpio-line-mux-states",
        (*glm).gpio_mux_states.as_mut_ptr(),
        ngpio,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret, c"could not get mux states\n");
    }

    ret = devm_gpiochip_add_data(dev, &mut (*glm).gc, glm as *mut core::ffi::c_void);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"failed to add gpiochip\n");
    }

    0
}

static gpio_lmux_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"gpio-line-mux", ..Default::default() },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, gpio_lmux_of_match);

static mut gpio_lmux_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"gpio-line-mux",
        of_match_table: gpio_lmux_of_match.as_ptr(),
        ..Default::default()
    },
    probe: Some(gpio_lmux_probe),
    ..Default::default()
};
module_platform_driver!(gpio_lmux_driver);

MODULE_AUTHOR!(c"Jonas Jelonek <jelonek.jonas@gmail.com>");
MODULE_DESCRIPTION!(c"GPIO line mux driver");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
