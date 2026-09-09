// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  CLPS711X GPIO driver
 *
 *  Copyright (C) 2012,2013 Alexander Shiyan <shc_work@mail.ru>
 */

// Dependencies supplied by the surrounding kernel bindings are intentionally
// left external to this translation unit.

unsafe fn clps711x_gpio_probe(pdev: *mut platform_device) -> c_int {
    let mut config: gpio_generic_chip_config = core::mem::zeroed();
    let np: *mut device_node = (*pdev).dev.of_node;
    let mut gen_gc: *mut gpio_generic_chip;
    let mut dat: *mut core::ffi::c_void;
    let mut dir: *mut core::ffi::c_void;
    let mut err: c_int;
    let mut id: c_int;

    if np.is_null() {
        return -ENODEV;
    }

    id = of_alias_get_id(np, c"gpio".as_ptr());
    if (id < 0) || (id > 4) {
        return -ENODEV;
    }

    gen_gc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<gpio_generic_chip>(), GFP_KERNEL)
        as *mut gpio_generic_chip;
    if gen_gc.is_null() {
        return -ENOMEM;
    }

    dat = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(dat) {
        return PTR_ERR(dat);
    }

    dir = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR(dir) {
        return PTR_ERR(dir);
    }

    config.dev = &mut (*pdev).dev;
    config.sz = 1;
    config.dat = dat;

    match id {
        3 => {
            /* PORTD is inverted logic for direction register */
            config.dirin = dir;
        }
        _ => {
            config.dirout = dir;
        }
    }

    err = gpio_generic_chip_init(gen_gc, &mut config);
    if err != 0 {
        return err;
    }

    match id {
        4 => {
            /* PORTE is 3 lines only */
            (*gen_gc).gc.ngpio = 3;
        }
        _ => {}
    }

    (*gen_gc).gc.base = -1;
    (*gen_gc).gc.owner = THIS_MODULE;

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gen_gc).gc, core::ptr::null_mut())
}

static clps711x_gpio_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"cirrus,ep7209-gpio".as_ptr() },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];

static mut clps711x_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"clps711x-gpio".as_ptr(),
        of_match_table: clps711x_gpio_ids.as_ptr(),
    },
    probe: Some(clps711x_gpio_probe),
};

// module_platform_driver(clps711x_gpio_driver);

// MODULE_DEVICE_TABLE(of, clps711x_gpio_ids);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
// MODULE_DESCRIPTION("CLPS711X GPIO driver");
// MODULE_ALIAS("platform:clps711x-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
